// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! Contains `SyncCommandBufferBuilder` and `SyncCommandBuffer`.
//!
//! # How pipeline stages work in Vulkan
//!
//! Imagine you create a command buffer that contains 10 dispatch commands, and submit that command
//! buffer. According to the Vulkan specs, the implementation is free to execute the 10 commands
//! simultaneously.
//!
//! Now imagine that the command buffer contains 10 draw commands instead. Contrary to the dispatch
//! commands, the draw pipeline contains multiple stages: draw indirect, vertex input, vertex shader,
//! ..., fragment shader, late fragment test, color output. When there are multiple stages, the
//! implementations must start and end the stages in order. In other words it can start the draw
//! indirect stage of all 10 commands, then start the vertex input stage of all 10 commands, and so
//! on. But it can't for example start the fragment shader stage of a command before starting the
//! vertex shader stage of another command. Same thing for ending the stages in the right order.
//!
//! Depending on the type of the command, the pipeline stages are different. Compute shaders use the
//! compute stage, while transfer commands use the transfer stage. The compute and transfer stages
//! aren't ordered.
//!
//! When you submit multiple command buffers to a queue, the implementation doesn't do anything in
//! particular and behaves as if the command buffers were appended to one another. Therefore if you
//! submit a command buffer with 10 dispatch commands, followed with another command buffer with 5
//! dispatch commands, then the implementation can perform the 15 commands simultaneously.
//!
//! ## Introducing barriers
//!
//! In some situations this is not the desired behaviour. If you add a command that writes to a
//! buffer followed with another command that reads that buffer, you don't want them to execute
//! simultaneously. Instead you want the second one to wait until the first one is finished. This
//! is done by adding a pipeline barrier between the two commands.
//!
//! A pipeline barriers has a source stage and a destination stage (plus various other things).
//! A barrier represents a split in the list of commands. When you add it, the stages of the commands
//! before the barrier corresponding to the source stage of the barrier, must finish before the
//! stages of the commands after the barrier corresponding to the destination stage of the barrier
//! can start.
//!
//! For example if you add a barrier that transitions from the compute stage to the compute stage,
//! then the compute stage of all the commands before the barrier must end before the compute stage
//! of all the commands after the barrier can start. This is appropriate for the example about
//! writing then reading the same buffer.
//!
//! ## Batching barriers
//!
//! Since barriers are "expensive" (as the queue must block), vulkano attempts to group as many
//! pipeline barriers as possible into one.
//!
//! Adding a command to a sync command buffer builder does not immediately add it to the underlying
//! command buffer builder. Instead the command is added to a queue, and the builder keeps a
//! prototype of a barrier that must be added before the commands in the queue are flushed.
//!
//! Whenever you add a command, the builder will find out whether a barrier is needed before the
//! command. If so, it will try to merge this barrier with the prototype and add the command to the
//! queue. If not possible, the queue will be entirely flushed and the command added to a fresh new
//! queue with a fresh new barrier prototype.

pub use self::builder::{
    CommandBufferState, SetOrPush, StencilOpStateDynamic, StencilStateDynamic,
    SyncCommandBufferBuilder, SyncCommandBufferBuilderBindDescriptorSets,
    SyncCommandBufferBuilderBindVertexBuffer, SyncCommandBufferBuilderError,
    SyncCommandBufferBuilderExecuteCommands,
};
use super::{
    sys::{UnsafeCommandBuffer, UnsafeCommandBufferBuilder},
    CommandBufferExecError,
};
use crate::range_map::RangeMap;
use crate::{
    buffer::{sys::UnsafeBuffer, BufferAccess},
    device::{Device, DeviceOwned, Queue},
    image::{sys::UnsafeImage, ImageAccess, ImageLayout, ImageSubresourceRange},
    sync::{
        AccessCheckError, AccessError, AccessFlags, GpuFuture, PipelineMemoryAccess, PipelineStages,
    },
    DeviceSize,
};
use std::{borrow::Cow, collections::HashMap, ops::Range, sync::Arc};

mod builder;

/// Command buffer built from a `SyncCommandBufferBuilder` that provides utilities to handle
/// synchronization.
pub struct SyncCommandBuffer {
    // The actual Vulkan command buffer.
    inner: UnsafeCommandBuffer,

    // List of commands used by the command buffer. Used to hold the various resources that are
    // being used.
    commands: Vec<Box<dyn Command>>,

    // Locations within commands that pipeline barriers were inserted. For debugging purposes.
    // TODO: present only in cfg(debug_assertions)?
    barriers: Vec<usize>,

    // State of all the resources used by this command buffer.
    buffers2: HashMap<Arc<UnsafeBuffer>, RangeMap<DeviceSize, BufferFinalState>>,
    images2: HashMap<Arc<UnsafeImage>, RangeMap<DeviceSize, ImageFinalState>>,

    // Resources and their accesses. Used for executing secondary command buffers in a primary.
    buffers: Vec<(
        Arc<dyn BufferAccess>,
        Range<DeviceSize>,
        PipelineMemoryAccess,
    )>,
    images: Vec<(
        Arc<dyn ImageAccess>,
        ImageSubresourceRange,
        PipelineMemoryAccess,
        ImageLayout,
        ImageLayout,
    )>,
}

impl SyncCommandBuffer {
    /// Tries to lock the resources used by the command buffer.
    ///
    /// > **Note**: You should call this in the implementation of the `CommandBuffer` trait.
    pub fn lock_submit(
        &self,
        future: &dyn GpuFuture,
        queue: &Queue,
    ) -> Result<(), CommandBufferExecError> {
        /*
            Acquire the state mutexes and check if the resources can be locked.
        */

        let buffer_state_mutexes = self
            .buffers2
            .iter()
            .map(|(buffer, range_map)| {
                let mut buffer_state = buffer.state();

                for (range, state) in range_map.iter() {
                    match future.check_buffer_access(buffer, range.clone(), state.exclusive, queue)
                    {
                        Err(AccessCheckError::Denied(err)) => {
                            let resource_use = &state.resource_uses[0];

                            return Err(CommandBufferExecError::AccessError {
                                error: err,
                                command_name: self.commands[resource_use.command_index]
                                    .name()
                                    .into(),
                                command_param: resource_use.name.clone(),
                                command_offset: resource_use.command_index,
                            });
                        }
                        Err(AccessCheckError::Unknown) => {
                            let result = if state.exclusive {
                                buffer_state.check_gpu_write(range.clone())
                            } else {
                                buffer_state.check_gpu_read(range.clone())
                            };

                            if let Err(err) = result {
                                let resource_use = &state.resource_uses[0];

                                return Err(CommandBufferExecError::AccessError {
                                    error: err,
                                    command_name: self.commands[resource_use.command_index]
                                        .name()
                                        .into(),
                                    command_param: resource_use.name.clone(),
                                    command_offset: resource_use.command_index,
                                });
                            }
                        }
                        _ => (),
                    }
                }

                Ok((buffer.as_ref(), buffer_state))
            })
            .collect::<Result<Vec<(_, _)>, _>>()?;

        let image_state_mutexes = self
            .images2
            .iter()
            .map(|(image, range_map)| {
                let mut image_state = image.state();

                for (range, state) in range_map.iter() {
                    match future.check_image_access(
                        image,
                        range.clone(),
                        state.exclusive,
                        state.initial_layout,
                        queue,
                    ) {
                        Err(AccessCheckError::Denied(err)) => {
                            let resource_use = &state.resource_uses[0];

                            return Err(CommandBufferExecError::AccessError {
                                error: err,
                                command_name: self.commands[resource_use.command_index]
                                    .name()
                                    .into(),
                                command_param: resource_use.name.clone(),
                                command_offset: resource_use.command_index,
                            });
                        }
                        Err(AccessCheckError::Unknown) => {
                            let result = if state.exclusive {
                                image_state.check_gpu_write(range.clone(), state.initial_layout)
                            } else {
                                image_state.check_gpu_read(range.clone(), state.initial_layout)
                            };

                            if let Err(err) = result {
                                let resource_use = &state.resource_uses[0];

                                return Err(CommandBufferExecError::AccessError {
                                    error: err,
                                    command_name: self.commands[resource_use.command_index]
                                        .name()
                                        .into(),
                                    command_param: resource_use.name.clone(),
                                    command_offset: resource_use.command_index,
                                });
                            }
                        }
                        _ => (),
                    };
                }

                Ok((image.as_ref(), image_state))
            })
            .collect::<Result<Vec<(_, _)>, _>>()?;

        /*
            We verified that the resources can be locked, so while still holding the mutexes,
            lock them now.
        */
        unsafe {
            for (buffer, mut buffer_state) in buffer_state_mutexes {
                for (range, state) in self.buffers2[buffer].iter() {
                    if state.exclusive {
                        buffer_state.gpu_write_lock(range.clone());
                    } else {
                        buffer_state.gpu_read_lock(range.clone());
                    }
                }
            }

            for (image, mut image_state) in image_state_mutexes {
                for (range, state) in self.images2[image].iter() {
                    if state.exclusive {
                        image_state.gpu_write_lock(range.clone(), state.final_layout);
                    } else {
                        image_state.gpu_read_lock(range.clone());
                    }
                }
            }
        }

        // TODO: pipeline barriers if necessary?

        Ok(())
    }

    /// Unlocks the resources used by the command buffer.
    ///
    /// > **Note**: You should call this in the implementation of the `CommandBuffer` trait.
    ///
    /// # Safety
    ///
    /// The command buffer must have been successfully locked with `lock_submit()`.
    ///
    pub unsafe fn unlock(&self) {
        for (buffer, range_map) in &self.buffers2 {
            let mut buffer_state = buffer.state();

            for (range, state) in range_map.iter() {
                if state.exclusive {
                    buffer_state.gpu_write_unlock(range.clone());
                } else {
                    buffer_state.gpu_read_unlock(range.clone());
                }
            }
        }

        for (image, range_map) in &self.images2 {
            let mut image_state = image.state();

            for (range, state) in range_map.iter() {
                if state.exclusive {
                    image_state.gpu_write_unlock(range.clone());
                } else {
                    image_state.gpu_read_unlock(range.clone());
                }
            }
        }
    }

    /// Checks whether this command buffer has access to a buffer.
    ///
    /// > **Note**: Suitable when implementing the `CommandBuffer` trait.
    #[inline]
    pub fn check_buffer_access(
        &self,
        buffer: &UnsafeBuffer,
        range: Range<DeviceSize>,
        exclusive: bool,
        queue: &Queue,
    ) -> Result<Option<(PipelineStages, AccessFlags)>, AccessCheckError> {
        let range_map = match self.buffers2.get(buffer) {
            Some(x) => x,
            None => return Err(AccessCheckError::Unknown),
        };

        // TODO: check the queue family

        range_map
            .range(&range)
            .try_fold(
                (PipelineStages::none(), AccessFlags::none()),
                |(stages, access), (_range, state)| {
                    if !state.exclusive && exclusive {
                        Err(AccessCheckError::Unknown)
                    } else {
                        Ok((stages | state.final_stages, access | state.final_access))
                    }
                },
            )
            .map(Some)
    }

    /// Checks whether this command buffer has access to an image.
    ///
    /// > **Note**: Suitable when implementing the `CommandBuffer` trait.
    #[inline]
    pub fn check_image_access(
        &self,
        image: &UnsafeImage,
        range: Range<DeviceSize>,
        exclusive: bool,
        expected_layout: ImageLayout,
        queue: &Queue,
    ) -> Result<Option<(PipelineStages, AccessFlags)>, AccessCheckError> {
        let range_map = match self.images2.get(image) {
            Some(x) => x,
            None => return Err(AccessCheckError::Unknown),
        };

        // TODO: check the queue family

        range_map
            .range(&range)
            .try_fold(
                (PipelineStages::none(), AccessFlags::none()),
                |(stages, access), (_range, state)| {
                    if expected_layout != ImageLayout::Undefined
                        && state.final_layout != expected_layout
                    {
                        return Err(AccessCheckError::Denied(
                            AccessError::UnexpectedImageLayout {
                                allowed: state.final_layout,
                                requested: expected_layout,
                            },
                        ));
                    }

                    if !state.exclusive && exclusive {
                        Err(AccessCheckError::Unknown)
                    } else {
                        Ok((stages | state.final_stages, access | state.final_access))
                    }
                },
            )
            .map(Some)
    }

    #[inline]
    pub fn num_buffers(&self) -> usize {
        self.buffers.len()
    }

    #[inline]
    pub fn buffer(
        &self,
        index: usize,
    ) -> Option<(
        &Arc<dyn BufferAccess>,
        Range<DeviceSize>,
        PipelineMemoryAccess,
    )> {
        self.buffers
            .get(index)
            .map(|(buffer, range, memory)| (buffer, range.clone(), *memory))
    }

    #[inline]
    pub fn num_images(&self) -> usize {
        self.images.len()
    }

    #[inline]
    pub fn image(
        &self,
        index: usize,
    ) -> Option<(
        &Arc<dyn ImageAccess>,
        &ImageSubresourceRange,
        PipelineMemoryAccess,
        ImageLayout,
        ImageLayout,
    )> {
        self.images
            .get(index)
            .map(|(image, range, memory, start_layout, end_layout)| {
                (image, range, *memory, *start_layout, *end_layout)
            })
    }
}

impl AsRef<UnsafeCommandBuffer> for SyncCommandBuffer {
    #[inline]
    fn as_ref(&self) -> &UnsafeCommandBuffer {
        &self.inner
    }
}

unsafe impl DeviceOwned for SyncCommandBuffer {
    #[inline]
    fn device(&self) -> &Arc<Device> {
        self.inner.device()
    }
}

// Usage of a resource in a finished command buffer.
#[derive(Clone, PartialEq, Eq)]
struct BufferFinalState {
    // Lists every use of the resource.
    resource_uses: Vec<BufferUse>,

    // Stages of the last command that uses the resource.
    final_stages: PipelineStages,
    // Access for the last command that uses the resource.
    final_access: AccessFlags,

    // True if the resource is used in exclusive mode.
    exclusive: bool,
}

// Usage of a resource in a finished command buffer.
#[derive(Clone, PartialEq, Eq)]
struct ImageFinalState {
    // Lists every use of the resource.
    resource_uses: Vec<ImageUse>,

    // Stages of the last command that uses the resource.
    final_stages: PipelineStages,
    // Access for the last command that uses the resource.
    final_access: AccessFlags,

    // True if the resource is used in exclusive mode.
    exclusive: bool,

    // Layout that an image must be in at the start of the command buffer. Can be `Undefined` if we
    // don't care.
    initial_layout: ImageLayout,

    // Layout the image will be in at the end of the command buffer.
    final_layout: ImageLayout, // TODO: maybe wrap in an Option to mean that the layout doesn't change? because of buffers?
}

#[derive(Clone, PartialEq, Eq)]
struct BufferUse {
    command_index: usize,
    name: Cow<'static, str>,
}

#[derive(Clone, PartialEq, Eq)]
struct ImageUse {
    command_index: usize,
    name: Cow<'static, str>,
}

/// Type of resource whose state is to be tracked.
#[derive(Clone)]
pub(super) enum Resource {
    Buffer {
        buffer: Arc<dyn BufferAccess>,
        range: Range<DeviceSize>,
        memory: PipelineMemoryAccess,
    },
    Image {
        image: Arc<dyn ImageAccess>,
        subresource_range: ImageSubresourceRange,
        memory: PipelineMemoryAccess,
        start_layout: ImageLayout,
        end_layout: ImageLayout,
    },
}

// Trait for single commands within the list of commands.
pub(super) trait Command: Send + Sync {
    // Returns a user-friendly name for the command, for error reporting purposes.
    fn name(&self) -> &'static str;

    // Sends the command to the `UnsafeCommandBufferBuilder`. Calling this method twice on the same
    // object will likely lead to a panic.
    unsafe fn send(&self, out: &mut UnsafeCommandBufferBuilder);
}

impl std::fmt::Debug for dyn Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        buffer::{BufferUsage, CpuAccessibleBuffer, ImmutableBuffer},
        command_buffer::{
            pool::{CommandPool, CommandPoolBuilderAlloc},
            sys::CommandBufferBeginInfo,
            AutoCommandBufferBuilder, CommandBufferLevel, CommandBufferUsage, FillBufferInfo,
        },
        descriptor_set::{
            layout::{
                DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorSetLayoutCreateInfo,
                DescriptorType,
            },
            PersistentDescriptorSet, WriteDescriptorSet,
        },
        pipeline::{layout::PipelineLayoutCreateInfo, PipelineBindPoint, PipelineLayout},
        sampler::{Sampler, SamplerCreateInfo},
        shader::ShaderStages,
    };

    #[test]
    fn basic_creation() {
        unsafe {
            let (device, queue) = gfx_dev_and_queue!();

            let pool = Device::standard_command_pool(&device, queue.family());
            let pool_builder_alloc = pool
                .allocate(CommandBufferLevel::Primary, 1)
                .unwrap()
                .next()
                .unwrap();

            SyncCommandBufferBuilder::new(
                &pool_builder_alloc.inner(),
                CommandBufferBeginInfo {
                    usage: CommandBufferUsage::MultipleSubmit,
                    ..Default::default()
                },
            )
            .unwrap();
        }
    }

    #[test]
    fn secondary_conflicting_writes() {
        unsafe {
            let (device, queue) = gfx_dev_and_queue!();

            // Create a tiny test buffer
            let (buf, future) =
                ImmutableBuffer::from_data(0u32, BufferUsage::transfer_dst(), queue.clone())
                    .unwrap();
            future
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();

            // Two secondary command buffers that both write to the buffer
            let secondary = (0..2)
                .map(|_| {
                    let mut builder = AutoCommandBufferBuilder::secondary(
                        device.clone(),
                        queue.family(),
                        CommandBufferUsage::SimultaneousUse,
                        Default::default(),
                    )
                    .unwrap();
                    builder
                        .fill_buffer(FillBufferInfo {
                            data: 42u32,
                            ..FillBufferInfo::dst_buffer(buf.clone())
                        })
                        .unwrap();
                    Arc::new(builder.build().unwrap())
                })
                .collect::<Vec<_>>();

            let pool = Device::standard_command_pool(&device, queue.family());
            let allocs = pool
                .allocate(CommandBufferLevel::Primary, 2)
                .unwrap()
                .collect::<Vec<_>>();

            {
                let mut builder = SyncCommandBufferBuilder::new(
                    allocs[0].inner(),
                    CommandBufferBeginInfo {
                        usage: CommandBufferUsage::SimultaneousUse,
                        ..Default::default()
                    },
                )
                .unwrap();

                // Add both secondary command buffers using separate execute_commands calls.
                secondary.iter().cloned().for_each(|secondary| {
                    let mut ec = builder.execute_commands();
                    ec.add(secondary);
                    ec.submit().unwrap();
                });

                let primary = builder.build().unwrap();
                let names = primary
                    .commands
                    .iter()
                    .map(|c| c.name())
                    .collect::<Vec<_>>();

                // Ensure that the builder added a barrier between the two writes
                assert_eq!(&names, &["execute_commands", "execute_commands"]);
                assert_eq!(&primary.barriers, &[0, 1]);
            }

            {
                let mut builder = SyncCommandBufferBuilder::new(
                    allocs[1].inner(),
                    CommandBufferBeginInfo {
                        usage: CommandBufferUsage::SimultaneousUse,
                        ..Default::default()
                    },
                )
                .unwrap();

                // Add a single execute_commands for all secondary command buffers at once
                let mut ec = builder.execute_commands();
                secondary.into_iter().for_each(|secondary| {
                    ec.add(secondary);
                });
                ec.submit().unwrap();
            }
        }
    }

    #[test]
    fn vertex_buffer_binding() {
        unsafe {
            let (device, queue) = gfx_dev_and_queue!();

            let pool = Device::standard_command_pool(&device, queue.family());
            let pool_builder_alloc = pool
                .allocate(CommandBufferLevel::Primary, 1)
                .unwrap()
                .next()
                .unwrap();
            let mut sync = SyncCommandBufferBuilder::new(
                &pool_builder_alloc.inner(),
                CommandBufferBeginInfo {
                    usage: CommandBufferUsage::MultipleSubmit,
                    ..Default::default()
                },
            )
            .unwrap();
            let buf =
                CpuAccessibleBuffer::from_data(device, BufferUsage::all(), false, 0u32).unwrap();
            let mut buf_builder = sync.bind_vertex_buffers();
            buf_builder.add(buf);
            buf_builder.submit(1);

            assert!(sync.state().vertex_buffer(0).is_none());
            assert!(sync.state().vertex_buffer(1).is_some());
            assert!(sync.state().vertex_buffer(2).is_none());
        }
    }

    #[test]
    fn descriptor_set_binding() {
        unsafe {
            let (device, queue) = gfx_dev_and_queue!();

            let pool = Device::standard_command_pool(&device, queue.family());
            let pool_builder_alloc = pool
                .allocate(CommandBufferLevel::Primary, 1)
                .unwrap()
                .next()
                .unwrap();
            let mut sync = SyncCommandBufferBuilder::new(
                &pool_builder_alloc.inner(),
                CommandBufferBeginInfo {
                    usage: CommandBufferUsage::MultipleSubmit,
                    ..Default::default()
                },
            )
            .unwrap();
            let set_layout = DescriptorSetLayout::new(
                device.clone(),
                DescriptorSetLayoutCreateInfo {
                    bindings: [(
                        0,
                        DescriptorSetLayoutBinding {
                            stages: ShaderStages::all(),
                            ..DescriptorSetLayoutBinding::descriptor_type(DescriptorType::Sampler)
                        },
                    )]
                    .into(),
                    ..Default::default()
                },
            )
            .unwrap();
            let pipeline_layout = PipelineLayout::new(
                device.clone(),
                PipelineLayoutCreateInfo {
                    set_layouts: [set_layout.clone(), set_layout.clone()].into(),
                    ..Default::default()
                },
            )
            .unwrap();

            let set = PersistentDescriptorSet::new(
                set_layout.clone(),
                [WriteDescriptorSet::sampler(
                    0,
                    Sampler::new(device.clone(), SamplerCreateInfo::simple_repeat_linear())
                        .unwrap(),
                )],
            )
            .unwrap();

            let mut set_builder = sync.bind_descriptor_sets();
            set_builder.add(set.clone());
            set_builder.submit(PipelineBindPoint::Graphics, pipeline_layout.clone(), 1);

            assert!(sync
                .state()
                .descriptor_set(PipelineBindPoint::Compute, 0)
                .is_none());
            assert!(sync
                .state()
                .descriptor_set(PipelineBindPoint::Graphics, 0)
                .is_none());
            assert!(sync
                .state()
                .descriptor_set(PipelineBindPoint::Graphics, 1)
                .is_some());
            assert!(sync
                .state()
                .descriptor_set(PipelineBindPoint::Graphics, 2)
                .is_none());

            let mut set_builder = sync.bind_descriptor_sets();
            set_builder.add(set);
            set_builder.submit(PipelineBindPoint::Graphics, pipeline_layout, 0);

            assert!(sync
                .state()
                .descriptor_set(PipelineBindPoint::Graphics, 0)
                .is_some());
            assert!(sync
                .state()
                .descriptor_set(PipelineBindPoint::Graphics, 1)
                .is_some());

            let pipeline_layout = PipelineLayout::new(
                device.clone(),
                PipelineLayoutCreateInfo {
                    set_layouts: [
                        DescriptorSetLayout::new(device.clone(), Default::default()).unwrap(),
                        set_layout.clone(),
                    ]
                    .into(),
                    ..Default::default()
                },
            )
            .unwrap();

            let set = PersistentDescriptorSet::new(
                set_layout.clone(),
                [WriteDescriptorSet::sampler(
                    0,
                    Sampler::new(device, SamplerCreateInfo::simple_repeat_linear()).unwrap(),
                )],
            )
            .unwrap();

            let mut set_builder = sync.bind_descriptor_sets();
            set_builder.add(set);
            set_builder.submit(PipelineBindPoint::Graphics, pipeline_layout, 1);

            assert!(sync
                .state()
                .descriptor_set(PipelineBindPoint::Graphics, 0)
                .is_none());
            assert!(sync
                .state()
                .descriptor_set(PipelineBindPoint::Graphics, 1)
                .is_some());
        }
    }
}
