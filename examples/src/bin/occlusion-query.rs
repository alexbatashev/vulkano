// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

// This is a modification of the triangle example, that demonstrates the basics of occlusion queries.
// Occlusion queries allow you to query whether, and sometimes how many, pixels pass the depth test
// in a range of draw calls.

use bytemuck::{Pod, Zeroable};
use std::sync::Arc;
use vulkano::{
    buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassContents,
    },
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceExtensions, DeviceOwned, QueueCreateInfo,
    },
    format::Format,
    image::{view::ImageView, AttachmentImage, ImageAccess, ImageUsage, SwapchainImage},
    impl_vertex,
    instance::{Instance, InstanceCreateInfo},
    pipeline::{
        graphics::{
            depth_stencil::DepthStencilState,
            input_assembly::InputAssemblyState,
            vertex_input::BuffersDefinition,
            viewport::{Viewport, ViewportState},
        },
        GraphicsPipeline,
    },
    query::{QueryControlFlags, QueryPool, QueryPoolCreateInfo, QueryResultFlags, QueryType},
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    swapchain::{
        acquire_next_image, AcquireError, Swapchain, SwapchainCreateInfo, SwapchainCreationError,
    },
    sync::{self, FlushError, GpuFuture},
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn main() {
    let required_extensions = vulkano_win::required_extensions();
    let instance = Instance::new(InstanceCreateInfo {
        enabled_extensions: required_extensions,
        // Enable enumerating devices that use non-conformant vulkan implementations. (ex. MoltenVK)
        enumerate_portability: true,
        ..Default::default()
    })
    .unwrap();

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();

    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };
    let (physical_device, queue_family) = PhysicalDevice::enumerate(&instance)
        .filter(|&p| p.supported_extensions().is_superset_of(&device_extensions))
        .filter_map(|p| {
            p.queue_families()
                .find(|&q| q.supports_graphics() && q.supports_surface(&surface).unwrap_or(false))
                .map(|q| (p, q))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
        })
        .unwrap();

    println!(
        "Using device: {} (type: {:?})",
        physical_device.properties().device_name,
        physical_device.properties().device_type,
    );

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
            ..Default::default()
        },
    )
    .unwrap();
    let queue = queues.next().unwrap();

    let (mut swapchain, images) = {
        let surface_capabilities = physical_device
            .surface_capabilities(&surface, Default::default())
            .unwrap();
        let image_format = Some(
            physical_device
                .surface_formats(&surface, Default::default())
                .unwrap()[0]
                .0,
        );

        Swapchain::new(
            device.clone(),
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: surface_capabilities.min_image_count,
                image_format,
                image_extent: surface.window().inner_size().into(),
                image_usage: ImageUsage::color_attachment(),
                composite_alpha: surface_capabilities
                    .supported_composite_alpha
                    .iter()
                    .next()
                    .unwrap(),
                ..Default::default()
            },
        )
        .unwrap()
    };

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
    struct Vertex {
        position: [f32; 3],
        color: [f32; 3],
    }
    impl_vertex!(Vertex, position, color);

    let vertices = [
        // The first triangle (red) is the same one as in the triangle example.
        Vertex {
            position: [-0.5, -0.25, 0.5],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.0, 0.5, 0.5],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.25, -0.1, 0.5],
            color: [1.0, 0.0, 0.0],
        },
        // The second triangle (cyan) is the same shape and position as the first,
        // but smaller, and moved behind a bit.
        // It should be completely occluded by the first triangle.
        // (You can lower its z value to put it in front)
        Vertex {
            position: [-0.25, -0.125, 0.6],
            color: [0.0, 1.0, 1.0],
        },
        Vertex {
            position: [0.0, 0.25, 0.6],
            color: [0.0, 1.0, 1.0],
        },
        Vertex {
            position: [0.125, -0.05, 0.6],
            color: [0.0, 1.0, 1.0],
        },
        // The third triangle (green) is the same shape and size as the first,
        // but moved to the left and behind the second.
        // It is partially occluded by the first two.
        Vertex {
            position: [-0.25, -0.25, 0.7],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.25, 0.5, 0.7],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.1, 0.7],
            color: [0.0, 1.0, 0.0],
        },
    ];
    let vertex_buffer =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, vertices)
            .unwrap();

    // Create three buffer slices, one for each triangle.
    let triangle1 = vertex_buffer.slice::<Vertex>(0..3).unwrap();
    let triangle2 = vertex_buffer.slice::<Vertex>(3..6).unwrap();
    let triangle3 = vertex_buffer.slice::<Vertex>(6..9).unwrap();

    // Create a query pool for occlusion queries, with 3 slots.
    let query_pool = QueryPool::new(
        device.clone(),
        QueryPoolCreateInfo {
            query_count: 3,
            ..QueryPoolCreateInfo::query_type(QueryType::Occlusion)
        },
    )
    .unwrap();

    // Create a buffer on the CPU to hold the results of the three queries.
    // Query results are always represented as either `u32` or `u64`.
    // For occlusion queries, you always need one element per query. You can ask for the number of
    // elements needed at runtime by calling `QueryType::result_len`.
    // If you retrieve query results with `with_availability` enabled, then this array needs to
    // be 6 elements long instead of 3.
    let mut query_results = [0u32; 3];

    mod vs {
        vulkano_shaders::shader! {
            ty: "vertex",
            src: "
				#version 450

				layout(location = 0) in vec3 position;
                layout(location = 1) in vec3 color;

                layout(location = 0) out vec3 v_color;

				void main() {
                    v_color = color;
					gl_Position = vec4(position, 1.0);
				}
			"
        }
    }

    mod fs {
        vulkano_shaders::shader! {
            ty: "fragment",
            src: "
				#version 450

                layout(location = 0) in vec3 v_color;
				layout(location = 0) out vec4 f_color;

				void main() {
					f_color = vec4(v_color, 1.0);
				}
			"
        }
    }

    let vs = vs::load(device.clone()).unwrap();
    let fs = fs::load(device.clone()).unwrap();

    let render_pass = vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.image_format(),
                samples: 1,
            },
            depth: {
                load: Clear,
                store: DontCare,
                format: Format::D16_UNORM,
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {depth}
        }
    )
    .unwrap();

    let pipeline = GraphicsPipeline::start()
        .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
        .vertex_shader(vs.entry_point("main").unwrap(), ())
        .input_assembly_state(InputAssemblyState::new())
        .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
        .fragment_shader(fs.entry_point("main").unwrap(), ())
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        // Enable depth testing, which is needed for occlusion queries to make sense at all.
        // If you disable depth testing, every pixel is considered to pass the depth test, so
        // every query will return a nonzero result.
        .depth_stencil_state(DepthStencilState::simple_depth_test())
        .build(device.clone())
        .unwrap();

    let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [0.0, 0.0],
        depth_range: 0.0..1.0,
    };

    let mut framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);

    let mut recreate_swapchain = false;
    let mut previous_frame_end = Some(sync::now(device.clone()).boxed());

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            recreate_swapchain = true;
        }
        Event::RedrawEventsCleared => {
            let dimensions = surface.window().inner_size();
            if dimensions.width == 0 || dimensions.height == 0 {
                return;
            }

            previous_frame_end.as_mut().unwrap().cleanup_finished();

            if recreate_swapchain {
                let (new_swapchain, new_images) = match swapchain.recreate(SwapchainCreateInfo {
                    image_extent: dimensions.into(),
                    ..swapchain.create_info()
                }) {
                    Ok(r) => r,
                    Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
                    Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                };

                swapchain = new_swapchain;
                framebuffers =
                    window_size_dependent_setup(&new_images, render_pass.clone(), &mut viewport);
                recreate_swapchain = false;
            }

            let (image_num, suboptimal, acquire_future) =
                match acquire_next_image(swapchain.clone(), None) {
                    Ok(r) => r,
                    Err(AcquireError::OutOfDate) => {
                        recreate_swapchain = true;
                        return;
                    }
                    Err(e) => panic!("Failed to acquire next image: {:?}", e),
                };

            if suboptimal {
                recreate_swapchain = true;
            }

            let mut builder = AutoCommandBufferBuilder::primary(
                device.clone(),
                queue.family(),
                CommandBufferUsage::OneTimeSubmit,
            )
            .unwrap();

            // Beginning or resetting a query is unsafe for now.
            unsafe {
                builder
                    // A query must be reset before each use, including the first use.
                    // This must be done outside a render pass.
                    .reset_query_pool(query_pool.clone(), 0..3)
                    .unwrap()
                    .set_viewport(0, [viewport.clone()])
                    .bind_pipeline_graphics(pipeline.clone())
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into()), Some(1.0.into())],
                            ..RenderPassBeginInfo::framebuffer(framebuffers[image_num].clone())
                        },
                        SubpassContents::Inline,
                    )
                    .unwrap()
                    // Begin query 0, then draw the red triangle.
                    // Enabling the `precise` bit would give exact numeric results. This needs
                    // the `occlusion_query_precise` feature to be enabled on the device.
                    .begin_query(query_pool.clone(), 0, QueryControlFlags { precise: false })
                    .unwrap()
                    .bind_vertex_buffers(0, triangle1.clone())
                    .draw(triangle1.len() as u32, 1, 0, 0)
                    .unwrap()
                    // End query 0.
                    .end_query(query_pool.clone(), 0)
                    .unwrap()
                    // Begin query 1 for the cyan triangle.
                    .begin_query(query_pool.clone(), 1, QueryControlFlags { precise: false })
                    .unwrap()
                    .bind_vertex_buffers(0, triangle2.clone())
                    .draw(triangle2.len() as u32, 1, 0, 0)
                    .unwrap()
                    .end_query(query_pool.clone(), 1)
                    .unwrap()
                    // Finally, query 2 for the green triangle.
                    .begin_query(query_pool.clone(), 2, QueryControlFlags { precise: false })
                    .unwrap()
                    .bind_vertex_buffers(0, triangle3.clone())
                    .draw(triangle3.len() as u32, 1, 0, 0)
                    .unwrap()
                    .end_query(query_pool.clone(), 2)
                    .unwrap()
                    .end_render_pass()
                    .unwrap();
            }

            let command_buffer = builder.build().unwrap();

            let future = previous_frame_end
                .take()
                .unwrap()
                .join(acquire_future)
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
                .then_signal_fence_and_flush();

            match future {
                Ok(future) => {
                    previous_frame_end = Some(future.boxed());
                }
                Err(FlushError::OutOfDate) => {
                    recreate_swapchain = true;
                    previous_frame_end = Some(sync::now(device.clone()).boxed());
                }
                Err(e) => {
                    println!("Failed to flush future: {:?}", e);
                    previous_frame_end = Some(sync::now(device.clone()).boxed());
                }
            }

            // Retrieve the query results.
            // This copies the results to a variable on the CPU. You can also use the
            // `copy_query_pool_results` function on a command buffer to write results to a
            // Vulkano buffer. This could then be used to influence draw operations further down
            // the line, either in the same frame or a future frame.
            query_pool
                .queries_range(0..3)
                .unwrap()
                .get_results(
                    &mut query_results,
                    QueryResultFlags {
                        // Block the function call until the results are available.
                        // Note: if not all the queries have actually been executed, then this
                        // will wait forever for something that never happens!
                        wait: true,

                        // Blocking and waiting will never give partial results.
                        partial: false,

                        // Blocking and waiting will ensure the results are always available after
                        // the function returns.
                        //
                        // If you disable waiting, then this can be used to include the
                        // availability of each query's results. You need one extra element per
                        // query in your `query_results` buffer for this. This element will
                        // be filled with a zero/nonzero value indicating availability.
                        with_availability: false,
                    },
                )
                .unwrap();

            // If the `precise` bit was not enabled, then you're only guaranteed to get a boolean
            // result here: zero if all pixels were occluded, nonzero if only some were occluded.
            // Enabling `precise` will give the exact number of pixels.

            // Query 0 (red triangle) will always succeed, because the depth buffer starts empty
            // and will never occlude anything.
            assert_ne!(query_results[0], 0);

            // Query 1 (cyan triangle) will fail, because it's drawn completely behind the first.
            assert_eq!(query_results[1], 0);

            // Query 2 (green triangle) will succeed, because it's only partially occluded.
            assert_ne!(query_results[2], 0);
        }
        _ => (),
    });
}

fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<RenderPass>,
    viewport: &mut Viewport,
) -> Vec<Arc<Framebuffer>> {
    let dimensions = images[0].dimensions().width_height();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

    let depth_attachment = ImageView::new_default(
        AttachmentImage::with_usage(
            render_pass.device().clone(),
            dimensions,
            Format::D16_UNORM,
            ImageUsage {
                depth_stencil_attachment: true,
                transient_attachment: true,
                ..ImageUsage::none()
            },
        )
        .unwrap(),
    )
    .unwrap();

    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view, depth_attachment.clone()],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}
