# Unreleased
<!--
    This file is maintained by the repository organization members. Please, do
    not put changes into changelog files directly, they lead to frequent merging
    conflicts. Instead put incoming changelog entries into the Pull Request
    description. They will be transferred to this file right after the
    Pull Request merge. 
-->

# Version 0.30.0 (2022-07-20)

- **Breaking** Removed the `try_gpu_lock`, `increase_gpu_lock` and `unlock` methods from the `BufferAccess` and `ImageAccess` traits. Locking is now implemented internally in `UnsafeBuffer` and `UnsafeImage`.
- **Breaking** All `check_buffer_access` and `check_image_access` functions now take an `UnsafeBuffer`/`UnsafeImage` and a `Range<DeviceSize>`.
- **Breaking** `UnsafeCommandBufferBuilder::pipeline_barrier` now takes a `DependencyInfo`. It will use `vkCmdPipelineBarrier2` if supported.
- **Breaking** The debug command buffer commands have been renamed to match Vulkan. They now take a `DebugUtilsLabel` value.
- **Breaking** `end_debug_utils_label` is now `unsafe`, as it requires validation checks that are not implemented yet.
- **Breaking** `DebugCallback` is renamed to `DebugUtilsMessenger` to match Vulkan, and now takes a `DebugUtilsMessengerCreateInfo` to construct. It is now `unsafe` to create, because the the callback cannot call any Vulkan API functions, which Vulkano is unable to check.
- **Breaking** The copy, blit, clear, fill and update commands are completely rewritten, and now use `Info` structs to specify the parameters. They now allow you to specify multiple regions at once to copy, give you fill control over the image subresources, and also let you select image layouts.
- **Breaking** The `transfer_source` and `transfer_destination` fields of `BufferUsage` and `ImageUsage` are renamed to `transfer_src` and `transfer_dst` to match Vulkan.
- **Breaking** `SubImage` has been removed.
- **Breaking** The `conflict_key` method on the `BufferAccess` and `ImageAccess` traits is removed.
- **Breaking** `ImageViewCreateInfo` now has a single `subresource_range` field instead of separate fields for aspect, mip levels and array layers.
- **Breaking** The `aspects`, `mip_levels` and `array_layers` methods of `ImageViewAbstract` are removed, and replaced with a single `subresource_range` method.
- **Breaking** The `current_mip_levels_access` and `current_array_layers_access` methods of `ImageAccess` are removed.
- **Breaking** `begin_render_pass` now takes a `RenderPassBeginInfo` struct.
- **Breaking** `ClearRect` now has a single `Range<u32>` for array layers.
- **Breaking** The fields of `ClearAttachment::Color` are now named.
- **Breaking** The `ImageClearValue` trait is removed.
- **Breaking** The various `AutoCommandBufferBuilder` constructors for secondary command buffers have been merged into one `secondary` function, which directly takes a `CommandBufferInheritanceInfo` value.
- **Breaking** The `render_pass` values of `GraphicsPipelineBuilder` and `CommandBufferInheritanceInfo` have both been changed into an enum that selects between rendering with `begin_render_pass` and rendering with `begin_rendering`. They implement `Into` for easy conversion.
- **Breaking** Added the missing `rasterization_samples` field to `MultisampleState`, which must be provided when doing multisampled rendering.
- **Breaking** Renamed the `ready` method of `Fence` to `is_signaled`.
- **Breaking** The `set_depth_bounds` parameter is now a single `RangeInclusive` instead of two separate `f32`, to match the type used on the `DepthBoundsState` struct.
- **Breaking** Removed `DeviceExtensions::required_extensions()`. Required extensions such as `khr_portability_subset` are now enabled automatically when available.
- **Breaking** Removed `PhysicalDevice::required_extensions()`
- `UnsafeCommandPoolCreateInfo` and `UnsafeCommandPoolCreationError` interfaces exposed.
- Fixed compile error in Vulkano-win on Android.
- Added `COVERAGE.md`, a document detailing how much of Vulkan is currently covered by Vulkano.
- Added debug utils commands to `Queue`.
- Added `Instance::with_debug_utils_messengers`, to provide creation info for messengers that should be used at instance creation and destruction time. This function is also `unsafe`.
- Added `subresource_layers` and `subresource_range` methods to `UnsafeImage` and `ImageAccess` to easily generate these types from an image.
- Added support for the `khr_copy_commands2` device extension.
- Added the `resolve_image` command buffer command.
- `BufferViewAbstract` now has a `range` method that returns the byte range of the underlying buffer that the view covers.
- Added new enum value `CheckDispatchError::ZeroLengthDimensions` to be returned when `dispatch()` is called with dimension(s) of length zero.
- Vulkano-shaders now supports `raygen`, `anyhit`, `closesthit`, `miss`, `intersection` and `callable` shaders.
- Fix PresentFuture flushing twice if `then_swapchain_present` fails.
- Added new enum value `SwapchainCreationError::ImageExtentZeroLengthDimensions` to be returned when at least one of the image extent's dimensions are zero.
- Added support for dynamic rendering, and a `triangle-v1_3` example demonstrating how it's used.
- Fixed a bug where `NonExhaustive` implemented the `Default` trait and was therefore still constructable by the user.
- Updated ash to 0.37.0+1.3.209.
- Fixed bug in various Vulkan calls where the returned data might be incomplete.
- Fixed bug that triggered an assert if a render pass had an attachment with `Undefined` initial layout.
- Added an `is_signaled` method to `FenceSignalFuture`.
- Add a simple `general_purpose_image_view` method to `StorageImage` for less verbose image view creation for e.g. intermediary render targets.
- Add a `vulkano_util` crate to help reduce boilerplate in many use cases. `VulkanoContext` to hold access to device & instances, `VulkanoWindows` to organize windows and their renderers. `VulkanoRenderer` to hold the window and methods to `acquire` (swapchain) and `present` between which you are intended to execute your pipelines.
- Add option to change `PresentMode` at runtime in `vulkano_util` with `set_present_mode`
- Made `ImageUsage` constructor functions `const`.
- Added `intersection` and `max_count` methods to `SampleCounts`, as well as a `BitAnd` implementation.
- `InstanceCreateInfo` now has `enumerate_portability` field.

# Version 0.29.0 (2022-03-11)

- **Breaking** `Instance` creation parameters are given using `InstanceCreateInfo`.
- **Breaking** `ApplicationInfo` and the `app_info_from_cargo_toml!` macro are removed, their functionality is now integrated into `InstanceCreateInfo`.
- **Breaking** `Device` creation parameters are given using `DeviceCreateInfo`.
- **Breaking** Changes to `UnsafeBuffer`:
  - Creation parameters are given using `UnsafeBufferCreateInfo`. It is no longer unsafe to construct, only to use.
  - Memory requirements are no longer returned on construction, but retrieved by calling `memory_requirements` on the `UnsafeBuffer`.
- **Breaking** Constructing a buffer of zero size now results in a panic (except for `CpuBufferPool` chunks).
- **Breaking** `BufferView` creation parameters are given using `BufferViewCreateInfo`.
- **Breaking** The following types are now considered non-exhaustive and must be constructed using a method call and struct update syntax: `BufferUsage`, `SparseLevel`.
- **Breaking** `RenderPass` creation parameters are given using `RenderPassCreateInfo`. `RenderPassDesc` is removed.
- **Breaking** `Framebuffer` creation parameters are given using `FramebufferCreateInfo`.
- **Breaking** The `dimensions`, `width` and `height` methods of `Framebuffer` are replaced with `extent` and `layers`.
- **Breaking** Changes to `Surface`:
    - The constructors are renamed to match their Vulkan equivalents:
        - `from_display_mode` > `from_display_plane`
        - `from_anativewindow` > `from_android`
        - `from_ios_moltenvk` > `from_ios`
        - `from_macos_moltenvk` > `from_mac_os`
        - `from_vi_surface` > `from_vi`
        - `from_hwnd` > `from_win32`
    - `from_raw` now requires a `SurfaceApi` value.
    - `Surface::capabilities` has been moved to `PhysicalDevice` and split into three functions: `surface_capabilities`, `surface_formats` and `surface_present_modes`.
    - `Capabilities` has been renamed to `SurfaceCapabilities`.
    - The `is_supported` method has been moved to `QueueFamily` and renamed to `supports_surface`.
- **Breaking** Changes to `Swapchain`:
  - Creation parameters are given using `SwapchainCreateInfo`. Methods of `Swapchain` are renamed to match.
  - All names with `Fullscreen` and `fullscreen` have been renamed to `FullScreen` and `full_screen`, to match Vulkan word breaking.
  - `FullScreenExclusive::AppControlled` has been renamed to `FullScreenExclusive::ApplicationControlled` to match Vulkan.
  - Using `FullScreenExclusive::ApplicationControlled` on Windows now requires a `Win32Monitor`.
- **Breaking** Vulkano-win: `create_vk_surface` and `create_vk_surface_from_handle` have been renamed to `create_surface_from_winit` and `create_surface_from_handle` respectively.
- **Breaking** `DescriptorSetLayout` creation parameters are given using `DescriptorSetLayoutCreateInfo`. The getter methods on `DescriptorSetLayout` have been changed to match.
- **Breaking** `DescriptorDesc` has been renamed to `DescriptorSetLayoutBinding` and must now be constructed with struct update syntax. The `ty` member has been renamed to `descriptor_type`, and `variable_count` has been renamed to `variable_descriptor_count`.
- **Breaking** `PipelineLayout` creation parameters are given using `PipelineLayoutCreateInfo`. The getter methods on `PipelineLayout` have been changed to match.
- **Breaking** `PipelineLayoutPcRange` has been renamed to `PushConstantRange`.
- **Breaking** `DeviceMemory` creation parameters are given using `MemoryAllocateInfo` and `MemoryImportInfo`. The number of constructors has been reduced.
- **Breaking** Importing memory is now `unsafe`, because the safety of the file descriptor can't be guaranteed.
- **Breaking** `DedicatedAlloc` is renamed to `DedicatedAllocation`, and its `None` variant is now represented by being wrapped in an `Option`.
- **Breaking** `DeviceMemoryAllocError` is renamed to `DeviceMemoryAllocationError`.
- **Breaking** `UnsafeCommandPool` creation parameters are given using `UnsafeCommandPoolCreateInfo`.
- **Breaking** `UnsafeCommandPool::allocate_command_buffers` now takes `CommandBufferAllocateInfo`.
- **Breaking** The `alloc` method on the `CommandPool` trait has been renamed to `allocate`, and it takes `CommandBufferLevel` instead of a boolean.
- **Breaking** The `new` constructors of  `UnsafeCommandBufferBuilder` and `SyncCommandBufferBuilder` now take `CommandBufferBeginInfo`.
- **Breaking** The `begin_render_pass` methods of `UnsafeCommandBufferBuilder` and `SyncCommandBufferBuilder` now take `RenderPassBeginInfo`.
- **Breaking** `CommandBufferLevel` is now a plain enum, `CommandBufferLevel::Secondary` has no associated value anymore. Its constructors have been removed.
- **Breaking** `CommandBufferInheritance` and `CommandBufferRenderPassInheritance` have been renamed to `CommandBufferInheritanceInfo` and `CommandBufferRenderPassInheritanceInfo` respectively.
- **Breaking** The `inheritance` method on the `SecondaryCommandBuffer` trait has been renamed to `inheritance_info`.
- **Breaking** `QueryPool` creation parameters are given using `QueryPoolCreateInfo`.
- **Breaking** The `ty` and `num_slots` methods of `QueryPool` have been renamed to `query_type` and `query_count`.
- **Breaking** `Fence` and `Semaphore` no longer have a type parameter.
- **Breaking** `Event` creation parameters are given using `EventCreateInfo`.
- **Breaking** `Fence` creation parameters are given using `FenceCreateInfo`.
- **Breaking** `Semaphore` creation parameters are given using `SemaphoreCreateInfo`.
- **Breaking** `Semaphore::export_opaque_fd` is now `unsafe`.
- **Breaking** `Sampler` creation parameters are given using `SamplerCreateInfo`.
- **Breaking** `SamplerYcbcrConversion` creation parameters are given using `SamplerYcbcrConversionCreateInfo`.
- **Breaking** `UnsafeDescriptorPool` creation parameters are given using `UnsafeDescriptorPoolCreateInfo`.
- **Breaking** The `alloc` and `free` methods of `UnsafeDescriptorPool` are renamed to `allocate_descriptor_sets` and `free_descriptor_sets`.
- **Breaking** The `alloc` method of the `DescriptorPool` trait is renamed to `allocate`.
- **Breaking** `DescriptorSetLayout::descriptors_count` is renamed to `descriptor_counts`, and returns a hashmap instead of a separate type.
- **Breaking** `UnsafeImage` creation parameters are given using `UnsafeImageCreateInfo`.
- **Breaking** `ImageView` creation parameters are given using `ImageViewCreateInfo`.
- **Breaking** `ImageView::new` has been renamed to `new_default`.
- **Breaking** `ImageView::ty` has been renamed to `view_type`.
- **Breaking** The `format` methods of `UnsafeImage` and `ImageViewAbstract` now return `Option<Format>`.
- **Breaking** `UnsafeImage::flags` has been replaced with methods to return individual flags.
- **Breaking** `MappedDeviceMemory` is now constructed separately with `new`, the `_and_map` variants of `DeviceMemory` are removed.
- **Breaking** Changed how `MappedDeviceMemory` handles CPU access; added `invalidate_range` and `flush_range` methods and `read` and `write` methods to get (im)mutable byte slices to the memory.
- **Breaking** Bytemuck dependencies:
  - All buffer types now require their data type parameter to implement `BufferContents`, which has `bytemuck::Pod` as a supertrait.
  - `Vertex` also has `Pod` as a supertrait.
  - Consequently, you must derive the `Zeroable` and `Pod` traits from bytemuck on any custom structures that are used as buffer contents.
  - To do this for structures generated by Vulkano-shaders, use the `types_meta` parameter as seen in the teapot example.
- Fixed sync bug in `copy_image` and `blit_image` where the `src` and `dest` images are the same but with different mip level and/or array layer.
- Fixed bug in `begin_render_pass` causing a panic when clearing a depth-only attachment.
- Fixed bug in the `QueueFamily::supports_` methods causing a panic when querying support for a stage that needs no queue flags.
- Fixed buffer overflow bug in `AutoCommandBufferBuilder::push_constants`.
- Fixed `AutoCommandBufferBuilder::push_constants` to push multiple times in case of range overlap (to accommodate VUIDs 01795 and 01796) 
- Fixed `shader!` macro failing to compile with geometry shaders.
- Refactored `VertexBuffersCollection` to allow `Arc<dyn BufferAccess>`.
- Added a `Format::texels_per_block` method.\
- Fixed bug on certain drivers where samplers would behave oddly when minmax samplers are enabled.
- Added support for the `khr_create_renderpass2` extension.
- Added an `api` method to `Surface` to return the windowing API that it was created from.
- Added a `create_info` method to `Swapchain` to return a `SwapchainCreateInfo` that contains all the parameters copied from the existing swapchain. This can be used for easy recreation of outdated swapchains.
- Added support for the `khr_get_surface_capabilities2` extension.
- Vulkano-win: Added `create_win32_monitor_from_winit` function.
- Added support for Vulkan 1.3.
- Added `Version::HEADER_VERSION`, a constant that contains the Vulkan spec version that Vulkano currently uses.
- Added `PhysicalDevice::external_buffer_properties`.
- `PhysicalDevice::image_format_properties` now includes external memory properties in the returned structure.
- Added `Device::memory_fd_properties`.
- Added `PhysicalDevice::external_semaphore_properties`.
- Replaced `fnv` crate's `FnvHashMap`, `FnvHashSet` & `FnvHasher` with their `std` counterparts.
- Validate SPIR-V code containing `OpUndef` correctly.
- `VertexBuffersCollection` now supports arrays.
- Fixed overflow in multiview vertex buffer validation.
- Fixed missing initialization in `PhysicalDevice::surface_formats`.

# Version 0.28.0 (2022-02-02)

- **Breaking** Updated to ash 0.35, half 1.8, smallvec 1.7, winit 0.26, raw-window-handle 0.4.
- **Breaking** `ColorSpace::DciP3Linear` is renamed to `DisplayP3Linear` to match an equivalent change in the Vulkan naming.
- Updated vk.xml to 1.2.203.
- Upgraded to Rust 2021.
- **Breaking** `DescriptorWrite` is renamed to `WriteDescriptorSet` to match Vulkan, and moved into the main `descriptor_set` module.
- **Breaking** The constructors of `WriteDescriptorSet` are no longer unsafe. Added convenience constructors for writing non-arrayed bindings.
- **Breaking** Descriptor sets are no longer constructed using a builder, but with an iterator of `WriteDescriptorSet` items. See in the examples how this is used.
  - `PersistentDescriptorSet::start` has been replaced with three constructors, `new`, `new_variable` and `new_with_pool`. Each of these takes an iterator of `WriteDescriptorSet`.
  - `SingleLayoutDescSetPool::next` takes an iterator of `WriteDescriptorSet` as well.
- **Breaking** Shader reflection now always generates only the descriptor requirements needed for each given entry point, instead of for all of them. The `exact_entrypoint_interface` argument to the `shader!` macro is removed.
- Added ClearRect struct, which contains the region information to be cleared
- Added ClearAttachment enum, which will contain the ClearValue and type of attachment
- Implemented AutoCommandBufferBuilder::clear_attachments, which implements all possible checks from the vulkan standards to make it safe.
- Implemented SyncCommandBufferBuilder::clear_attachments and UnsafeCommandBufferBuilder::clear_attachments.
- Allowed `copy_image`, `blit_image` and `copy_buffer` to work with `src` and `dest` images/beffers being the same.
  But, regions from src to dest must not overlap.
- Fix undefined behaviour on external semaphore creation.
- Fix undefined behaviour on external image creation.
- Add uuid checking to gl-interop example.
- Added additional fields to `DescriptorRequirements`, and changed `mutable` to be on a per-descriptor basis.
- **Breaking** `Sampler` is now constructed using a builder, using `start` instead of `new`. The shortcut functions `simple_repeat_linear` and `simple_repeat_linear_no_mipmap` are retained, but now return `Result`.
- **Breaking** `MipmapMode` is renamed to `SamplerMipmapMode` to match Vulkan.
- **Breaking** `SamplerAddressMode::ClampToBorder` no longer includes a `BorderColor` value. This is now provided with the new builder.
- **Breaking** The `ImageViewAbstract::can_be_sampled` method is removed, and replaced with `Sampler::can_sample`.
- Added `Filter::Cubic`.
- Added support for filter reduction modes on samplers.
- Added `buffer_self_copy_overlapping` and `buffer_self_copy_not_overlapping` as unit tests for self_copy feature (#1782).
- Removed test `basic_conflict` (failed after #1782).
- Added VertexMember implementations for nalgebra Vectors and Points as an optional crate feature.
- **Breaking** The spelling `mip_level` is used everywhere as a function and variable name for consistency with Vulkan naming.
- **Breaking** Changes to `UnsafeImage`:
  - It is now constructed with a builder.
  - It is no longer unsafe to construct, only to use.
  - Memory requirements are no longer returned on construction, but retrieved by calling `memory_requirements` on the `UnsafeImage`.
  - `format_features` now returns a reference.
  - `preinitialized_layout` is replaced by `initial_layout`, which returns an `ImageLayout` value.
- **Breaking** Changes to the `ImageAccess` trait:
  - Added a `format_features` method, and removed `supports_blit_source` and `supports_blit_destination`.
  - `preinitialized_layout` is replaced by `initial_layout`, which returns an `ImageLayout` value.
  - `current_layer_levels_access` was misnamed, and is now named `current_array_layers_access`.
  - `current_miplevels_access` is now named `current_mip_levels_access`.
- **Breaking** Changes to `ImageView`:
  - `UnsafeImageView` has been merged into `ImageView`.
  - Removed `with_` from all the builder functions.
  - The `ImageViewAbstract` trait now has `VulkanObject` as a supertrait, and no longer has an `inner` method.
- **Breaking** Changes to `Format`:
  - `properties` has been moved to `PhysicalDevice::format_properties`. It now supports the `khr_format_feature_flags2` extension.
  - `requires_sampler_ycbcr_conversion` has been renamed to `ycbcr_chroma_sampling` and returns `Option<ChromaSampling>`, where `Some` replaces `true`.
  - `size` has been renamed to `block_size`.
  - `block_dimensions` has been renamed to `block_extent`.
  - The members of the `CompressionType` enum have changed.
- **Breaking** The `BufferViewAbstract` trait now has `VulkanObject` as a supertrait.
- **Breaking** `ExternalMemoryHandleType` has been renamed to `ExternalMemoryHandleTypes`, and the singular name `ExternalMemoryHandleType` has been introduced as an enum.
- **Breaking** `ComponentMapping` and `ComponentSwizzle` have been moved to the `sampler` module.
- Added `PhysicalDevice::image_format_properties`.
- Added `UnsafeImage::tiling`.
- The `ImageViewAbstract` trait now includes a `format_features` method.
- The `BufferViewAbstract` trait now includes `format` and `format_features` methods.
- Add `boxed_send()`, `boxed_sync()`, `boxed_send_sync()` for `GpuFuture`.
- Fixed mipmap generation for 2D array images.
- Added `aspects` to the builder of `ImageView`, to choose which aspect(s) of an image to use in the view.
- Added `SamplerYcbcrConversion`, and the ability to attach it to a sampler or image view.
- Fixed of not being able to create multi-layer framebuffers.

# Version 0.27.1 (2021-12-07)

- Reimplement generic impl's for `BufferAcces`, `TypedBufferAccess` & `ImageAccess`.

# Version 0.27.0 (2021-12-06)

- **Breaking** The modules `compute_pipeline` and `graphics_pipeline` are shortened to `compute` and `graphics`, and are now public.
- **Breaking** `ComputePipeline`, `GraphicsPipeline` and `PipelineLayout` are re-exported from the base `pipeline` module, but other items are not.
- **Breaking** Major changes have been made to most parts of the graphics pipeline state.
  - Each state category is now provided by a separate type ending in `State` that lives in its own submodule of `graphics`. Some of these were existing types that were renamed.
  - Most of the state-setting methods on `GraphicsPipelineBuilder` are deprecated. State is now set using a `_state` method, which is given one of the above state types.
- **Breaking** `ShaderInterfaceEntry` now contains the start `location` and `component`, and specifies its type using a new `ty` member instead of as a format.
- **Breaking** `DynamicStateMode` is replaced with a simple `bool`.
- **Breaking** The presence of dynamic state in the pipeline state is now expressed more explicitly with two new types, `StateMode` and `PartialStateMode`.
- **Breaking** The `PersistentDescriptorSet::start` and `SingleLayoutDescSetPool::new` functions now return `Result`.
- **Breaking** `DescriptorWrite` now takes an iterator instead of a single item, allowing arrayed descriptors to be written in one operation.
- **Breaking** Removed the `EntryPointAbstract` trait, which was unused.
- **Breaking** `ComputeEntryPoint` and `GraphicsEntryPoint` now hold a list of `DescriptorRequirements` instead of `DescriptorSetDesc`.
- **Breaking** Removed `union`, `union_multiple` and `ensure_compatible_with_shader` from `DescriptorSetDesc`, and `union` from `DescriptorDesc`. This is now handled by `DescriptorRequirements`.
- **Breaking** `DescriptorDesc::ensure_compatibility_with_shader` now takes `DescriptorRequirements` instead of another `DescriptorDesc`.
- **Breaking** When the shader's layout specifies a descriptor with runtime array, its upper bound from now on interprets as zero by default(previously it was `1`).
- **Breaking** The `DescriptorSet` trait now has a `resources` method, which returns a reference to a `DescriptorSetResources` object. It no longer has the `num_buffers`, `buffer`, `num_images` and `image` methods.
- **Breaking** `BufferViewRef` is replaced with `BufferViewAbstract`, similar to the existing `ImageViewAbstract`.
- **Breaking** `UnsafeDescriptorSet::write` takes a `DescriptorSetLayout` instead of `Device`.
- **Breaking** `DescriptorWrite` is now constructed based on the resources stored instead of the descriptor type. The descriptor type is inferred from the descriptor set layout.
- **Breaking** Added an `Arc` to many parameters and return types:
  - Objects implementing `BufferAccess`, `BufferViewAbstract`, `ImageAccess`, `ImageViewAbstract` or `DescriptorSet` are now always constructed in an `Arc`, and parameters that take one of these types require the `Arc` wrapper.
  - The type parameters of `BufferView`, `ImageView` and `BufferSlice` do not contain this implicit `Arc`.
  - The types `DescriptorSetLayout`, `PipelineLayout`, `ComputePipeline`, `GraphicsPipeline`, `QueryPool`, `RenderPass` and `Framebuffer` are also always constructed in an `Arc`.
- **Breaking** `Framebuffer` no longer has a type parameter.
  - This made the `FramebufferAbstract` trait redundant, and it has been removed.
  - `CommandBufferLevel` and its nested types no longer have a type parameter either.
  - `AttachmentsList` is no longer needed and has been removed.
- **Breaking** The `dimensions` method has been removed as an inherent method from types that already implement `ImageAccess`, to avoid confusion between the inherent method and the method of the trait when they have different semantics.
- **Breaking** Replaced `DescriptorDescTy` with `DescriptorType` and made further changes to the members of `DescriptorDesc`.
- **Breaking** Added a `Pipeline` trait to hold methods that are common to all pipeline types.
- **Breaking** Changes to allow allocating variable descriptor counts:
  - The `alloc` method of the `DescriptorPool` trait now has a second parameter to specify the number of descriptors to allocate for the variable count binding. If there is no variable count binding in the layout, this should be 0.
  - The `alloc` method of `UnsafeDescriptorPool` now takes an iterator of `DescriptorSetAllocateInfo`.
- For `PersistentDescriptorSet` when using a layout with a variable count binding, allocate only the number of descriptors needed instead of always the maximum.
- Descriptor resources are now checked against the shader requirements at the time of a draw/dispatch call, rather than at the time the descriptor set is created. Only the resources that are actually needed in the shader are checked, the other resources in a descriptor set are ignored and don't need to be valid.
- Added a new `DescriptorRequirements` type, which contains requirements imposed by a shader onto a descriptor and the resources bound to it.
  - `DescriptorDesc` can be created from `DescriptorRequirements` with the `From` trait.
  - `DescriptorSetDesc`s can be created from the requirements with the `from_requirement` constructor.
  - The descriptor requirements of a pipeline can be queried using a new `descriptor_requirements` method.
- Added basic support for the `khr_push_descriptor` extension, with an example of how it's used. The implementation is somewhat limited and some of the details may change in a future update.
- Added support for lots more dynamic state, most of which requires features or extensions to use.
- Added support for discard rectangles, in the `pipeline::discard_rectangle` module.
- Added support for line rasterization state, in the `pipeline::rasterization` module.
- Added support for color write enable state, in the `pipeline::color_blend` module.
- `GraphicsPipeline` now has methods to return each state object it was created with.
- `ash` has been updated to 0.33.3, which provides new features and extensions.
- Added `CommandBufferState::push_constants` to retrieve the set of all push constant bytes that have been set.
- Added check to `AutoCommandBufferBuilder` to ensure that the push constants that are needed by the pipeline have been set.
- Added android platform to external memory cfgs.
- Fixed two bugs related to the requirements for enabled extensions:
- For required extensions that have been promoted, the promoted version now also fulfills the requirement.
- For features that must be enabled in tandem with extensions (e.g. `descriptor_indexing`), the requirement only applies to Vulkan 1.2 and above, since these features do not exist on earlier versions and thus cannot be enabled.
- Fix device memory builder not including p_next structures.
- Fix exportable image not created with VkExternalMemoryImageCreateInfo.
- Fix mutable bit not set on exportable image.
- `ShaderModule` can how handle multiple entry points with the same name but differing execution models. `ShaderModule::entry_point` will now return `None` if the entry point with the specified name does not exist or is ambiguous. To disambiguate, use `ShaderModule::entry_point_with_execution`.
- Added a `shaderc-debug` cargo feature to Vulkano-shaders, to emit debug information in the generated SPIR-V code.
- Added a `type_for_format` macro, which returns a Rust type that is suitable for representing a given `Format`.
- Fixed a bug when using vulkano-shaders macro with multiple shaders in release mode.
- Fixed bug where the wrong image layout was used for images used as a storage image.
- Fix typo in `clear_depth_stencil`.

# Version 0.26.0 (2021-10-2)

- **Breaking** The `draw` and `draw_indexed` commands on `AutoCommandBufferBuilder` now take parameters to explicitly specify the range of vertices and instances to draw.
- **Breaking** `BufferlessDefinition` is removed.
- **Breaking** The `VertexSource` trait is removed, and has been replaced with the new `VertexBuffersCollection` trait, which works analogously to `DescriptorSetsCollection`. Vertex buffers can now be passed in as a tuple, just like descriptor sets.
- **Breaking** Removed the vertex definition type parameter from `GraphicsPipeline`, which is no longer needed with the change above.
- **Breaking** The `ComputePipelineAbstract` and `GraphicsPipelineAbstract` traits are no longer needed and have been removed, with their methods made available on the base `ComputePipeline` and `GraphicsPipeline` types.
- **Breaking** Changes to `DescriptorDesc` and related changes:
  - The variants of `DescriptorDescTy` now match those of `DescriptorType` and Vulkan.
  - The `array_count` and `readonly` members are renamed to `descriptor_count` (to match Vulkan) and `mutable` (with the opposite sense, to match familiar Rust usage).
  - `DescriptorImageDesc` is renamed to `DescriptorDescImage`. The `dimensions` and `arrayed` members have been combined into a single `ImageViewType` value.
  - Removed the `arrayed` member of the `InputAttachment` variant as well, as the standard now explicitly disallows arrayed input attachments.
  - The `ensure_superset_of` method has been split into two, one for shader compatibility and one for binding descriptor sets. The error return types of these methods have been revised.
- **Breaking** The `Cubemap` and `CubemapArray` variants of `ImageViewType` are renamed to `Cube` and `CubeArray` to match Vulkan.
- **Breaking** Setting state in `AutoCommandBufferBuilder` is now done via separate commands (e.g. `bind_descriptor_sets`, `bind_vertex_buffers`) instead of being provided with dispatch/draw commands. This matches how these commands work in Vulkan.
  - Validity of the state is checked by the dispatch/draw commands.
  - `DynamicState` is removed; each individual state now has its own command to set it. This includes some states that were not previously included in `DynamicState`.
- **Breaking** `StateCacher` is removed; its task is now partially handled by `SyncCommandBufferBuilder`.
- **Breaking** Many traits now require `Send + Sync`: `BufferAccess`, `ImageAccess`, `ImageViewAbstract`, `FramebufferAbstract`, `AttachmentsList`, `MemoryPoolAlloc`, `DescriptorSet`, `DescriptorPoolAlloc`, `PrimaryCommandBuffer`, `SecondaryCommandBuffer`, `CommandPoolAlloc`. This further means that any type parameters of types implementing these traits also require `Send + Sync`.
- **BREAKING** `DescriptorSetLayout::new()` now returns `Result<Self, DescriptorSetLayoutError>` instead of Result<Self, OomError>`
- **BREAKING** `DescriptorCompatibilityError` additional variant `VariableCount`.
- **BREAKING** `GraphicsPipelineCreationError` additional variant `PipelineLayoutCreationError`.
- **BREAKING** `PipelineLayoutCreationError` additional variant `SetLayoutError`.
- **BREAKING** `FixedSizeDescriptorSetsPool` has been replaced by `SingleLayoutDescSetPool`.
- **BREAKING** Set builders now return `&mut Self` instead of `Self` & methods take values wrapped in an `Arc`.
- **Breaking** Changes to `Format`:
  - `Format` variants are now all uppercase with underscores. This avoids interpretation problems where the underscore is significant for the meaning.
  - The `ty` method and the `FormatTy` enum are removed. They are replaced with the `NumericType` enum, which concerns itself only with the numeric representation and not with other properties. There are now three `type_*` methods to retrieve it, for colour, depth and stencil respectively.
  - The `planes` method now returns a slice containing the equivalent single-plane formats of each plane.
- **Breaking** The `ImageAccess` trait no longer has the `has_color`, `has_depth` and `has_stencil` methods. This information can be queried using the `aspects` or `type_*` methods of `Format`.
- **Breaking** Changes made to `DescriptorDescTy` to support immutable samplers.
- **Breaking** `DescriptorWrite::combined_image_sampler` now takes the sampler in an `Option`. Use `None` when the descriptor has immutable samplers.
- **Breaking** Changes to pipeline layout tweaks:
  - `DescriptorSetDesc::tweak` is renamed to `set_buffer_dynamic`, and now takes only a single binding index.
  - `DescriptorSetDesc::tweak_multiple` is removed.
  - The dynamic buffers parameter of `GraphicsPipelineBuilder::with_auto_layout` has been replaced with a closure that can be used to make tweaks to the descriptor set layouts as needed.
  - `ComputePipeline::new` has an additional closure parameter identical to the one described above.
- **Breaking** `AttachmentImage::dimensions()` now returns `[u32; 3]` which includes the layer count.
- **Breaking** Buffers and Images that have `with_exportable_fd` use dedicated allocation, thus requiring khr_get_memory_requirements2 and khr_dedicated_allocation on top of 
already needed khr_external_memory and khr_external_memory_fd.
- **Breaking** `Compare` is renamed to `CompareOp` to match Vulkan.
- **Breaking** Vulkano-shaders no longer generates a `Shader` struct, but instead provides `load` as a standalone function that returns `Arc<ShaderModule>` directly.
- **Breaking** Vulkano-shaders no longer generates a function for each entry point. Entry points are now retrieved using the `entry_point` method of `ShaderModule`.
- **Breaking** The `shader` module is moved to the crate root, and `spirv` is now a submodule of it.
- **Breaking** `ShaderModule::new` is renamed to `from_bytes` to match the existing `from_words`.
- **Breaking** `ShaderModule` now parses and analyses the shader provided to it on construction, checks whether the shader is compatible with the device, and gathers data such as descriptor sets and input/output interfaces directly from its code. To provide this data yourself as before, and avoid parsing, you can use the `_with_data` constructors.
- **Breaking** `ComputeEntryPoint` and `GraphicsEntryPoint` are removed, as are the functions on `ShaderModule` that generate them. Instead, you can look up an entry point using the `entry_point` method.
- Added support for additional shader types in `ShaderStage`, `ShaderStages` and `PipelineStages`.
- Added `export_fd` and `with_exportable_fd` to `AttachmentImage` and `StorageImage` as well as `mem_size` which is needed when using those images with Cuda.
- Vulkano-shaders: added extension/feature checks for more SPIR-V capabilities.
- Added support for surface creation from a CAMetalLayer using VK_EXT_metal_surface.
- Bug fixed. Image layout passed to SubImage is now being respected
- The full Rust code is now generated from vk.xml by autogen directly, instead of using intermediate macros.
- Added `storage_buffer()` method to `BufferUsage`.
- Fixed mismatched types on Android.
- Fixed bug in descriptor set validity checking that allowed drawing/dispatching with descriptor sets that didn't match the pipeline.
- Fixed bug where the wrong functions were used for retrieving physical device info.
- Fixed minor bug in retrieving features from the physical device.
- Add BSD platforms to external memory (dma-buf fd) cfgs
- Added support for `u8` index buffers with the `ext_index_buffer_uint8` extension.
- Descriptor sets now support variable count descriptors.
    - e.g. `layout(set = 0, binding = 0) uniform sampler2D textures[];`
- Non-breaking `Format` additions:
  - Formats and their metadata are now auto-generated from vk.xml. This significantly expands the support for YCbCr formats in particular.
  - The `compatibility` method returns an object that can be used to check the compatibility of two formats, explained [here](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/chap43.html#formats-compatibility-classes).
  - The `components` method returns the number of bits in each colour component.
  - The `compression` method returns the compression scheme as a new enum, `CompressionType`.
  - The `requires_sampler_ycbcr_conversion` method returns whether the "sampler YCbCr conversion" must be enabled on an image view and sampler in order to use this format.
  - `GraphicsPipelineBuilder` now uses `Vec` instead of `SmallVec` internally to reduced stack usage.
- Added support for descriptors with immutable samplers.
  - They are included as separate members on the appropriate variants of `DescriptorDescTy`.
  - Added a `set_immutable_samplers` method to `DescriptorSetDesc`, which lets you set the immutable samplers for a descriptor. This can be used together with the closure provided to pipeline constructors.
  - `add_image` can be used when building a descriptor set, to provide an image to a combined image sampler descriptor that has immutable samplers.
- Updated dependencies:
  - png 0.16 > 0.17
  - time 0.2 > 0.3
- `AttachmentImage::current_layer_levels_access()` now returns the correct range which solves pipeline barriers only affecting the first layers of of a multi-layer `AttachmentImage`.
- A new Vulkano-shaders macro option `shaders` to compile several shaders in a single macro invocation producing generated Rust structs common for all specified shaders without duplications. This feature improves type-safe interoperability between shaders.
- Fixed CommandBufferExecFuture adding the command buffer to queue submission after being flushed.
- Added a `spirv` module to the main crate, which contains an auto-generated parser for SPIR-V files, and various other utilities that can be used to analyze shaders at runtime.
- `DescriptorSetLayout` now has `variable_descriptor_count` which returns the descriptor count in a variable count descriptor if present.
- Additional copy buffer to image checks.
- Add `clear_depth_stencil_image` to `AutoCommandBufferBuilder` to clear depth / stencil images.
- `AutoCommandBufferBuilder` and `SyncCommandBufferBuilder` now have a `state` method, which can be used to query the state set by previous "bind" and "set" commands.
- Changed parameters requiring an iterator so that they now take `IntoIterator`, for more flexibility.
- Removed various custom iterator types in favour of returning `impl Iterator`.
- Replaced the various dynamic state querying methods of `GraphicsPipeline` with two: `dynamic_state` to query a single dynamic state, and `dynamic_states` to iterate over all of them.


# Version 0.25.0 (2021-08-10)

- **Breaking** Refactoring of `PhysicalDevice` and related types.
  - The module `instance::physical_device` is moved to `device::physical`. Items are exported directly from there.
  - Functions on `PhysicalDevice` that previously returned a custom iterator type now return `impl ExactSizeIterator`.
  - `PhysicalDevice` now has new methods `supported_extensions` and `required_extensions`. The equivalent constructors on `DeviceExtensions` are deprecated, but still present.
- **Breaking** Renamed `Device::loaded_extensions` to `enabled_extensions`, to match the terminology used for `enabled_features` as well as the Vulkan standard.
- **Breaking** Renamed `Instance::loaded_extensions` and `loaded_layers` to `enabled_extensions` and `enabled_layers` respectively.
- **Breaking** Major reorganisation of the `descriptor` module. Most importantly, its `descriptor_set` child is moved to the crate root and everything is placed under there. Full list of module changes:
  - `descriptor::descriptor_set::*` > `descriptor_set`.
  - `descriptor::descriptor::*` > `descriptor_set::layout`.
    - But `ShaderStages*` are moved to `pipeline::shader`.
  - `descriptor_set::UnsafeDescriptorSetLayout` > `descriptor_set::layout::DescriptorSetLayout` (renamed).
  - `descriptor_set::DescriptorSetDesc` > `descriptor_set::layout`.
  - `descriptor_set::{FixedSize*}` > `descriptor_set::fixed_size_pool`.
    - Re-exported `descriptor_set::FixedSizeDescriptorSetsPool`.
  - `descriptor_set::{Persistent*}` > `descriptor_set::persistent`.
    - Re-exported `descriptor_set::PersistentDescriptorSet`, `descriptor_set::PersistentDescriptorSetBuildError`, `descriptor_set::PersistentDescriptorSetError`.
  - `descriptor_set::{DescriptorPool*, DescriptorsCount, UnsafeDescriptorSetPool*}` > `descriptor_set::pool`.
  - `descriptor_set::{StdDescriptorPool*}` > `descriptor_set::pool::standard`.
    - Re-exported `descriptor_set::pool::StdDescriptorPool`.
  - `descriptor_set::{DescriptorWrite, UnsafeDescriptorSet}` > `descriptor_set::sys`.
  - `descriptor_set::collection` is now private.
- **Breaking** `Features::superset_of` is renamed to `is_superset_of`.
- **Breaking** The `VertexDefinition` trait no longer has `AttribsIter` and `BuffersIter` as associated types. It instead returns a `VertexInput`.
- **Breaking** The various types for vertex definitions (except `BufferlessDefinition`) have been replaced with the new `BuffersDefinition` type, which can handle any number of vertex and instance buffers. Examples of how it's used can be seen in the "instancing" and "teapot" examples.
- **Breaking** `InputRate` has been renamed to `VertexInputRate` to match the Vulkan name. The `Instance` variant now has a `divisor` member.
- **Breaking** `DescriptorSetDesc` is converted from a trait to a struct. `DescriptorSetLayout` can take this struct in its constructor, as alternative to the iterator which was already possible previously.
- **Breaking** `PipelineLayout` is now built from a list of descriptor set layouts and push constant ranges, like in plain Vulkan. It no longer constructs its own descriptor set layouts internally.
- **Breaking** `PipelineLayoutDescPcRange` is renamed to `PipelineLayoutPcRange`.
- **Breaking** `PipelineLayoutDesc` is no longer needed and is removed. Its functionality has been moved to `DescriptorSetDesc` and `PipelineLayoutPcRange`. This also applies to `ComputeEntryPoint`, `GraphicsEntryPoint` and `EntryPointAbstract`.
- **Breaking** `PipelineLayout` now has a `descriptor_set_layouts` method which returns a slice of all the layouts, replacing the previous `descriptor_set_layout` which only returned one at a time.
- **Breaking** Vulkano is now relaying on features from **Rustc >= 1.53.0**.
- **Breaking** The boolean `graphics` parameter on the `bind_descriptor_sets` method of `SyncCommandBufferBuilder`, `UnsafeCommandBufferBuilder` and `StateCacher` has been replaced with a new enum `PipelineBindPoint`.
- **Breaking** Known physical device properties (VkPhysicalDeviceProperties, VkPhysicalDeviceLimits, VkPhysicalDeviceSparseProperties) are now no longer `Option<T>`.
  - These properties are available with every `vkGetPhysicalDeviceProperties2` call.
  - Add some `Default` instances to make the auto trait happy.
- **Breaking** Values that represent a size, offset or index on the device are now represented as `DeviceSize` (a type alias of `u64`).
- **Breaking** Fix in push constant ranges with intersecting stage flags.
- Added support for the `ext_vertex_attribute_divisor` extension, via the new `BuffersDefinition` type and the additions to `VertexInputRate`.
- Added `is_superset_of` method to `DeviceExtensions` and `InstanceExtensions`.
- Examples now enable only the features they need instead of all of them.
- Examples have much smarter device selection logic. In the triangle example this is supplied with comments.
- Errors checking(by unwrapping) in `MappedDeviceMemory::read_write`.
- Add creation of Semaphores with exportable Linux file descriptor on.
- Add method to export file descriptor corresponding to Semaphore.
- `SemaphoreBuilder` introduced. 
- Add DisplayNative enum variant to ColorSpaceEnum (AMD-specific feature).
- Vulkano-shaders now provides the image format for descriptors, if the shader requires a specific format.
- Vulkano-shaders now uses the `spirv_headers` crate for some of its types.
- Large allocations now use dedicated allocations which improves memory efficiency.
- Add `Clone` interface for `LayerProperties`.
- Fixed detection of storage buffers in SPIR-V so that it works for all SPIR-V versions.
- Added `VertexInput` type, which contains the binding and attribute descriptions.
- Added a `vertex_input` method to `GraphicsPipelineAbstract`, which returns a reference to the vertex input.
- Add support for 8bit uniform storage.
- Added limits check for vertex buffer binding numbers.
- Fixed synchronization bug where pipeline barriers were placed based on the binding of resources instead of the draw/dispatch calls that use them.
- `SyncCommandBufferBuilder` and `UnsafeCommandBufferBuilder` accept any `IntoIterator` where they previously only accepted `Iterator`.
- Updated ash to 0.33.0, vk.xml to 1.2.185, and ran `cargo update` to update other dependencies' patch versions.
- Add support for EXTENDED_SRGB_NONLINEAR_EXT color space.
- Better error message for wrong color space value
- Added `impl BitOr` for `debug::MessageType` and `debug::MessageSeverity` and some bit field generators, also turning them into `const fn`.

# Version 0.24.0 (2021-06-20)

- **Breaking** Vulkano-shaders now checks if the device supports the shader's SPIR-V version, when loading the shader.
- **Breaking** (but unlikely) Vulkano-shaders now compiles to SPIR-V 1.0 by default. If your shader needs features only available in a higher version, you can specify the target version on the `shader!` macro with the new `vulkan_version: "major.minor"` and `spirv_version: "major.minor"` arguments.
- **Breaking** Changes to how image sample counts are represented.
  - Instead of an integer, functions with a parameter for number of image samples now take a value of `SampleCount`, an enum with variants named `SampleN`, where `N` is a power-of-two integer. It can be converted to a Vulkan `SampleCountFlags`, and from an integer with `try_from`.
  - `sample_counts` field is originaly represented as u32 type, which is now represented by `SampleCounts` struct-type which is a boolean collection of supported `sample_counts`. It can be converted to and from a Vulkan `SampleCountFlags`.
- **Breaking** Changes to shader interfaces and pipeline layouts.
  - The module `descriptor::pipeline_layout` has been renamed to `pipeline::layout`.
  - The trait `ShaderInterfaceDef` has been replaced by a simple struct `ShaderInterface`, and its `elements` method returns a slice instead of an iterator. This means you no longer need to define a new type for a shader interface. The accompanying type `ShaderInterfaceDefEntry` has been renamed to `ShaderInterfaceEntry` to match. The `ShaderInterfaceDefMatch` trait and `EmptyShaderInterfaceDef` struct have been removed.
  - The trait `PipelineLayoutDesc` has also been converted into a struct, with methods `descriptor_sets` and `push_constants` which return slices. It is functionally equivalent to the old `RuntimePipelineDesc` type, which is now merged into it. The traits `PipelineLayoutSuperset`, `PipelineLayoutSetsCompatible` and `PipelineLayoutPushConstantsCompatible` have been integrated into this struct as well. The `EmptyPipelineDesc` trait has been replaced with an `::empty()` constructor.
  - Consequently, functions that took a value of these traits now take a plain `ShaderInterface` or `PipelineLayoutDesc`. Types that had a type parameter for it no longer have it, e.g. `VertexDefinition`, `GraphicsEntryPoint`, `GraphicsEntryPointAbstract`, `PipelineLayout`.
  - Now that `PipelineLayout` has no more type parameter, the trait `PipelineLayoutAbstract` is removed. The layout type parameter is also removed from `ComputePipeline` and `GraphicsPipeline`.
  - `ComputeEntryPoint` and `GraphicsEntryPoint` now take a value specifying the push constants descriptor, instead of having a type parameter. The corresponding associated type on `EntryPointAbstract` has been removed.
  - The `GraphicsEntryPointAbstract` trait has been removed. `GraphicsPipelineBuilder` now takes a `GraphicsEntryPoint` object directly, and has lifetime parameters for the 5 shader types instead. `EntryPointDummy` is no longer needed and has been removed.
- **Breaking** The constructors of `Instance` now take an additional argument to specify the maximum API version.
- **Breaking** Vulkano is now built on top of [Ash](https://github.com/MaikKlein/ash) instead of vk-sys.
  - The `EntryPoints`, `InstancePointers` and `DevicePointers` types from vk-sys have been replaced with a new module `fns` containing `EntryFunctions`, `InstanceFunctions` and `DeviceFunctions`. Rather than containing the functions directly, there is a member for each Vulkan version and extension, which is loaded from Ash.
  - The functions to retrieve the function pointers have been renamed to `fns`.
  - The `TYPE` associated constant has been removed from the `VulkanObject` trait. This is now provided by the Ash `Handle` trait, which the object returned by `internal_object` must implement.
- **Breaking** `RawInstanceExtensions` and `RawDeviceExtensions` have been removed. The `Instance` and `Device` constructors now take `&InstanceExtensions` and `&DeviceExtensions` respectively. The `loaded_extensions` function returns these reference types as well.
- **Breaking** The restrictions of each enabled extension and feature are checked when creating an instance or device.
- **Breaking** Physical device properties are now retrieved through a new `properties` method, returning a `&Properties` reference.
  - To allow for the possibility that not every property is known by the physical device, fields in `Properties` are wrapped by an `Option`.
  - The previous methods for retrieving properties, `name`, `ty`, `limits`, `driver_version`, `pci_device_id`, `pci_vendor_id`, `uuid` and `extended_properties`, have been removed.
  - The `api_version` method remains, but there is now a semantic difference between it and the version reported by `properties`: The raw property gives the maximum supported version that the driver itself reports, while the method on `PhysicalDevice` returns the version a logical device would support if it were created from this physical device (that is, restricted by the instance's `max_api_version`).
- **Breaking** `ImageAccess` trait method `try_gpu_lock()` now has an additional argument to allow locking the image in an uninitialized state.`
- **Breaking** The `conflicts_buffer` and `conflicts_image` methods on the `BufferAccess` and `ImageAccess` traits are removed.
- **Breaking** Draw and dispatch calls on `AutoCommandBufferBuilder` no longer have a parameter for dynamic offsets. Instead, they are provided as part of the descriptor sets parameter. They are added to each descriptor set individually using the new `offsets` method on the `DescriptorSet` trait. `SyncCommandBufferBuilder` and `StateCacher` likewise take dynamic offsets as part of the descriptor set.
- **Breaking** `StencilFaceFlags` is renamed to `StencilFaces`, and its variants are also shortened.
- **Breaking** `DynamicStencilValue` now contains a `front` and a `back` value.
- **Breaking** The stencil state commands of `SyncCommandBufferBuilder` now take a `StencilFaces` value and a `u32` instead of `DynamicStencilValue`.
- Improve `ImageLayout` checks to prevent `AccessError::ImageNotInitialized` from occurring where the image is safe to use uninitialized.
- Added `DeviceExtensions::khr_spirv_1_4`, which allows SPIR-V 1.4 shaders in Vulkan 1.1.
- Added `FunctionPointers::api_version` to query the highest supported instance version.
- Added `Instance::api_version` and `Device::api_version` to return the actual supported Vulkan version. These may differ between instance and device, and be lower than what `FunctionPointers::api_version` and `PhysicalDevice::api_version` return.
- Added `Instance::max_api_version`, which returns the maximum version that was specified when creating the instance.
- Fixed the issue when creating a buffer with exportable fd on Linux(see to #1545).
- The `draw_indirect` and `draw_indexed_indirect` commands on `AutoCommandBufferBuilder` now check the draw count against the `max_draw_indirect_count` limit.
- Fixed a few documentation errors.
- It is now possible to construct a graphics pipeline without a fragment shader.
- Added support for all core Vulkan 1.1 and 1.2 device features.
- Updated winit to 0.25.
- Fixed the teapot example on ArchLinux (GTX 1650).
- Added support for the SPIR-V draw parameters capability.
- Added support for the VK_KHR_multiview extension. 
- Vulkano-shaders: Added support for MultiView SPIR-V capability.
- Multiview example added showing how to utilize the VK_KHR_multiview extension to render to multiple layers of a framebuffer at once.
- All Vulkan extensions supported by Ash are now provided in `InstanceExtensions` and `DeviceExtensions`. This includes all but the very newest extensions; new extensions should be added whenever Ash is updated to a new version.
- Every extension is now documented with a link to its Vulkan page and information about requirements, conflicts, promotion and deprecation.
- `InstanceExtensions` and `DeviceExtensions` now have a `From` implementation that takes an iterator of `&CStr`. There is also a `From` implementation for `Vec<CString>` that performs the reverse conversion.
- All Vulkan features supported by Ash are now provided.
- Implement fmt::Display for DisplayMode.
- Clarify return value is in millihertz for DisplayMode.refresh_rate()
- Reworked some of the internals of `SyncCommandBufferBuilder`, so that a mutex is no longer needed and resource conflict detection is simplified.
- `Features::all` and `Features::none` are now `const fn`.

# Version 0.23.0 (2021-04-10)

- **Breaking** Changes to command buffers:
  - `AutoCommandBuffer` and the `CommandBuffer` trait have been split in two, one for primary and the other for secondary command buffers. `AutoCommandBufferBuilder` remains one type, but has a type parameter for the level of command buffer it will be creating, and some of its methods are only implemented for builders that create `PrimaryAutoCommandBuffer`.
  - The `Flags` enum is renamed to `CommandBufferUsage`, and is exported from the main `command_buffer` module. The `None` variant is renamed to `MultipleSubmit`.
  - Simplified the number of constructors on `AutoCommandBufferBuilder` by adding a `CommandBufferUsage` parameter.
  - `Kind` has been renamed to `CommandBufferLevel`, and for secondary command buffers it now contains a single `CommandBufferInheritance` value.
  - `CommandBufferInheritance::occlusion_query` and `UnsafeCommandBufferBuilder::begin_query` now take `QueryControlFlags` instead of a boolean.
- **Breaking** The non-default constructors of `ImageView` have been replaced with a builder, created with `ImageView::start(image)`.
- **Breaking** Added support for component mapping/swizzling on image views.
  - `image::Swizzle` is moved and renamed to `image::view::ComponentMapping`. It now has an `is_identity` method.
  - A non-default component mapping can now be specified for image views, via the new builder. A `ComponentMapping` parameter has been added to `UnsafeImageView` as well.
  - The `identity_swizzle` method on the `ImageViewAbstract` trait has been replaced with `component_mapping`, which returns a `ComponentMapping` directly.
  - Storage image and input attachment descriptors now check for identity swizzling when being built.
- **Breaking** Major rearranging of framebuffer and render pass-related types:
  - The `framebuffer` module is renamed to `render_pass`.
  - `RenderPassDesc` is now a struct, not a trait. The methods have been simplified, returning a slice reference to the `attachments`, `subpasses` and `dependencies`.
    - Renamed: `AttachmentDescription` > `AttachmentDesc`, `PassDescription` > `SubpassDesc`, `PassDependencyDescription` > `SubpassDependencyDesc`.
    - `EmptySinglePassRenderPassDesc` is replaced with the `RenderPassDesc::empty` constructor, or its `Default` implementation.
    - The `RenderPassCompatible`, `RenderPassDescClearValues` and `RenderPassSubpassInterface` traits are removed, their functionality is moved to `RenderPassDesc`.
  - `RenderPass` takes a concrete `RenderPassDesc` value for construction, and no longer has a type parameter.
    - The `RenderPassAbstract` trait is removed.
    - `GraphicsPipeline` and `Framebuffer` no longer have a render pass type parameter.
    - `GraphicsPipelineAbstract` and `FramebufferAbstract` have trait methods to retrieve the render pass instead.
  - The `ordered_passes_renderpass!` and `single_pass_renderpass!` macros are unchanged externally.
- Support for queries:
  - **Breaking** `UnsafeQueryPool`, `UnsafeQuery` and `UnsafeQueriesRange` have `Unsafe` removed from their names.
  - **Breaking** `QueriesRange` is now represented with a standard Rust `Range` in its API.
  - **Breaking** The secondary command buffer constructors that have parameters for queries will check if the corresponding features are enabled, and return a different error type.
  - Removed `OcclusionQueriesPool`, which was incomplete and never did anything useful.
  - `get_results` has been added to `QueriesRange`, to copy query results to the CPU.
  - The following functions have been added to both `SyncCommandBufferBuilder` and `AutoCommandBufferBuilder`: `begin_query` (still unsafe), `end_query` (safe), `write_timestamp` (still unsafe), `copy_query_pool_results` (safe), `reset_command_pool` (still unsafe).
  - Better documentation of everything in the `query` module.
  - An example demonstrating occlusion queries.
- **Breaking** Improved the handling of image aspects a little, with the enum `ImageAspect` and the struct `ImageAspects`. `UnsafeCommandBufferBuilderImageAspect` has been removed.
- **Breaking** Removed the separate structs for each image format. Now, only the `Format` enum exists.
  - Traits that no longer make sense in this context have been removed: `FormatDesc`, the `Possible*FormatDesc` traits, `StrongStorage`.
  - In types that had a type parameter for the format type, it has been removed.
  - `AcceptsPixels` has been converted to `Pixel`, which is implemented on the pixel type rather than on the format type.
- **Breaking** `shader!` will generate descriptor information for all variables declared in the shader module, even if they are not used. *This reverts the default behavior from the last release.*
  - **Breaking** Added the `exact_entrypoint_interface` option to `shader!` to force vulkano to only generate descriptor information for variables that are used. (the default behavior from the last release)
- **Breaking** `AccessFlagBits` is renamed to `AccessFlags`.
- **Breaking** Minor refactor of `UnsafeBuffer`:
  - Replaced the various usage functions with a single `usage` function, mirroring the change made earlier to `UnsafeImage`.
  - The `SparseLevel::sparse` member is removed, and `UnsafeBuffer::new` takes `Option<SparseLevel>` instead.
- **Breaking** `Swapchain` is now constructed using a builder. `Swapchain::start` will start building a new blank swapchain. Calling `recreate` on an existing swapchain will create a builder filled with all the properties of the old swapchain.
- Added two methods to `Format`: `planes` to query the number of planes in the format, and `aspects` to query what aspects an image of this type has.
- The deprecated `cause` trait function on Vulkano error types is replaced with `source`.
- Fixed bug in descriptor array layers check when the image is a cubemap.
- Vulkano-shaders: Fixed and refined the generation of the `readonly` descriptor attribute. It should now correctly mark uniforms and sampled images as read-only, but storage buffers and images only if explicitly marked as `readonly` in the shader.
- Vulkano-shaders: Added support for StoragePushConstant8 SPIR-V capability.
- Fixed a bug which caused a segfault when extending memory allocation info in DeviceMemoryBuilder
- `BufferlessDefinition` and `BufferlessVertices` now derive `Copy` and `Clone`. This allows `GraphicsPipelineBuilder`s that have not yet defined a vertex buffer type to be cloned.
- Various functions for converting to/from Vulkan flags have been consolidated into implementations of the standard `From` trait.
- Export root-level `entry_point` method on `loader::FunctionPointers` type.
- Add few more `InstanceExtensions` from KHR and EXT.

# Version 0.22.0 (2021-03-31)

- **Breaking** Updated all code to Rust 2018 edition.
- **Breaking** DeviceMemoryBuilder::new() takes in `memory_index` rather than `MemoryType`.
- Fixed `shader!` generated descriptor set layouts for shader modules with multiple entrypoints.
  - **Breaking** Prefixed `shader!` generated descriptor set `Layout` structs with the name of the entrypoint the layout belongs to. For shaders generated from GLSL source, this means `Layout` has been renamed to `MainLayout`.
  - **Breaking** `shader!` will no longer generate descriptor information for variables that are declared but not used in a shader.
- **Breaking** `shader!` now accepts structs in shader interfaces decorated with `BufferBlock` rather than `Block`.
- Fixed missing barriers in dispatch calls
  - **Breaking** `shader!` no longer marks descriptor sets as readonly as a fallback when it doesn't know
    - **Breaking** The keyword `readonly` might need to be added in front of the `buffer` keyword in GLSL files to get them working again
- **Breaking** Changes to image types:
  - Image types no longer implement `ImageViewAccess`.
  - `Dimensions` is removed. Image constructors now take `ImageDimensions`.
  - `ImageDimensions` no longer has the `cubemap_compatible` member. Instead, several image constructors take `ImageCreateFlags` which specifies this aspect.
  - Replaced the various functions of `UnsafeImage` to query format features and usage with two that simply return `Formatfeatures` and `ImageUsage`.
- **Breaking** Changes to image view handling:
  - Created a new `image::view` module for all image view related things.
  - Introduced a new `ImageView` type, a safe wrapper around `UnsafeImageView`.
  - The `ImageViewAccess` trait is renamed to `ImageViewAbstract`, some methods added, removed or renamed. `ImageView` implements this trait.
  - `UnsafeImageView` no longer holds image usage information, nor does it check for valid usage.
- **Breaking** `UnsafeCommandBuffer` and `SyncCommandBuffer` and their corresponding builders and other related types no longer have a type parameter for the command pool allocation, and no longer keep the command pool alive. Their constructors now take an `&UnsafeCommandPoolAlloc`. Users must now ensure that the pool allocation outlives the command buffers and their builders (`AutoCommandBuffer` does this itself).
- **Breaking** The `CommandBuffer` trait no longer has the `PoolAlloc` associated type, and has four new methods: `num_buffers`, `buffer`, `num_images` and `image`.
- **Breaking** structures passed to `ImmutableBuffer::from_data` and `CpuAccessibleBuffer::from_data` must implement [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) to ensure soundness of these functions
- Replaced deprecated `compare_and_swap` with `compare_exchange`.
- `UnsafeCommandPoolAlloc` now implements `DeviceOwned`.
- Allow `const` usage of features and `BufferUsage`.
- Opaque fd and dma-buf import support on `Linux`.
- `DeviceMemoryMapping` to separate device memory and mappings.
- Added external memory support for `DeviceLocalBuffer` for Linux.
- Implemented synchronization for `SyncCommandBufferBuilder::execute_commands`.
- `AutoCommandBufferBuilder::execute_commands` is now fully safe to use.
- `SyncCommandBufferBuilder` now becomes poisoned when it returns an error, to prevent using the builder in an inconsistent state.
- Added a `dispatch_indirect` command to `AutoCommandBufferBuilder`.

# Version 0.21.0 (2021-03-05)

- **Breaking** `Message::layer_prefix` turned to Option to prevent segfaults when Vulkan message didn't provide `pMessageIdName` value
- **Breaking** On `AutoCommandBufferBuilder`, methods that bind a descriptor set now take a `dynamic_offsets` parameter
- **Breaking** On `AutoCommandBufferBuilder` and `SyncCommandBufferBuilder`, the `update_buffer` method now takes `data` by reference
- **Breaking** Made `PipelineLayoutDescTweaks` public, for use with compute pipelines
- Added support for `ImageAspect` and YV12/NV12 formats,  for use with the UnsafeImage API.
- Added basic VK_KHR_external_memory, VK_KHR_external_memory_fd, and VK_EXT_external_memory_dma_buf support.
- Fixed potential segmentation fault in `ComputePipeline` when referencing `PipelineCache` objects.
- Fixed race condition in `StandardCommandPool` when allocating buffers.
- Fixed potential stack overflow error in loading large shaders by storing the bytecode as static.
- Fixed descriptor set layouts with arrays containing more than one element triggering unreachable code.
- Fixed panic on some machines when calling `Surface::capabilities`
- Added basic support and safety checks for dynamic uniform/storage buffers
- Updated dependencies:
  - `crossbeam` 0.7 -> 0.8
  - `half` 1.6 -> 1.7
  - `shaderc` 0.6 -> 0.7
  - `smallvec` 1.4 -> 1.6
  - `cgmath` 0.17 -> 0.18 (for examples)
  - `png` 0.15 -> 0.16 (for examples)
  - `time` 0.1 -> 0.2 (for examples)
- Added `VK_KHR_portability_subset` device extension.
- Added `DeviceExtensions::required_extensions` function that returns a set of available extensions required to create `Device` on this platform.
- `FormatFeatures` now implements `Copy`.
- Removed the `AttachmentImageView` trait, which didn't appear to be used for anything anyway.
- Added support and validation for the `mutable_format` and `block_texel_view_compatible` image creation flags.
- Added `format` to the `ImageViewBuilder`, to choose a different format from the underlying image.

# Version 0.20.0 (2020-12-26)

- **Breaking** The `ImmutableImage::from_iter` and `ImmutableImage::from_buffer` can build Mipmaps
- **Breaking** `CpuAccessibleBuffer` now uses `RwLock` from `parking_lot`.
- **Breaking** The `Kind` and `SubpassContents` types have been moved to the root of the `command_buffer` module.
- **Breaking** On `AutoCommandBufferBuilder`, the methods `begin_render_pass` and `next_subpass` now take `SubpassContents` instead of a boolean value.
- **Breaking** The `CommandBuffer` trait now has an additional required method, `kind`.
- **Breaking** Update dependency `winit` 0.22 -> 0.24
- **Breaking** Added an Option to the ComputePipeline constructor to enable pipeline caching.
- Add support for 8bit storage.
- Some safety checks have been added to `execute_commands`/`execute_commands_from_vec` on `AutoCommandBufferBuilder`, but not everything is checked yet so it's still unsafe to call.
- Added two vulkano-win features `raw-window-handle_` and `winit_` to be able to use vulkano without winit.
- Added function to create surface from `RawWindowHandle`.
- Added a `properties` method to `Format`.
- Added additional device feature flags for enabling SPIR-V related capabilities.
- Added method `copy_buffer_dimensions` that allows copying parts of buffers containing arrays.
- Added `debug_marker_begin`, `debug_marker_end` and `debug_marker_insert` to `AutoCommandBufferBuilder`.
- Fixed surface creation function on Windows(PR #1410).
- Travis CI Linux Nightly job temporary disabled until #1423 resolved.
- Renamed feature from `shader_f3264` to `shader_float64`.
- Added method `build_with_cache` to the `GraphicsPipelineBuilder` that enables pipeline caching.
- Check usage bits on image when creating image view.
- Fixing an assertion panic in the SyncCommandBuffer. If the buffer encountered an error while locking the necessary resources, it would unlock all previously locked resources. Images were unlocked incorrectly and an assert in the image unlock function would panic.
- Added support for including precompiled shaders in vulkano-shaders using the `bytes` option.
- Added an option for Vulkano-Shaders macro to automatically generate standard
  traits(Default, PartialEq, etc) implementations for Rust types generated from
  the Shader types, and to derive these structs with external traits.
- Fixed clear `Cubemap` and `CubemapArray` framebuffer attachment dimensions; previously only 1/6th of the layers in each of these image layouts were actually considered by the framebuffer (for clearing and rendering). This seems to have been a result of the difference between `Dimensions` and `ImageDimensions`.

# Version 0.19.0 (2020-06-01)

- **Breaking** AutoCommandBufferBuilder methods now take a mutable reference to `self` instead of taking ownership
- Update `smallvec` 1.2 -> 1.4
- Added additional `ImageUsage` constructor functions, `color_attachment`, `depth_stencil_attachment`, `transient_color_attachment` & `transient_depth_stencil_attachment`.
- Update dependency `half` 1.5 -> 1.6
- Update MacOS dependency `metal` 0.17 -> 0.18
- Added additional checks around image/device compatibility.
- Fixed all warnings as of Rust 1.43.1
- Provides new API to fetch additional information of the Physical Device from the `PhysicalDeviceProperties2KHR` structure whenever it possible. In the current implementation only `subgroupSize` property is fetching. This interface can be extended in the future to obtain more metadata depending on community needs.
- `dynamic-local-size` compute shader example added showing how to utilize `subgroupSize` to compute and set shader's local size in run time.
- Fixed Vulkano Shaders bug when compute shader local group layout values bound to specialization constants. Now it is possible to define the layout in form of `layout(local_size_x_id = 12, local_size_y_id = 13, local_size_z = 1) in;` and then set the values as `SpecializationConstants {constant_12: 8, constant_13: 4, ...}`.
- Allow applications to access the instance and device pointers
- Add a helper function '.boxed()' on the `GpuFuture` that is short for `Box::new(yourFuture) as Box<dyn GpuFuture>`

# Version 0.18.0 (2020-03-11)

- **Breaking** Update dependency `winit` 0.21 -> 0.22
- Update dependency `half` 1.4 -> 1.5
- Update dependency `smallvec` 0.6 -> 1.2

# Version 0.17.0 (2020-02-09)

- **Breaking** Swapchain::recreate_with_dimension() is now Swapchain::recreate_with_dimensions()
- **Breaking** Sync::SharingMode::Exclusive(queue_family: u32) is now Sync::SharingMode::Exclusive.
- **Breaking** Added Swapchain::with_old_swapchain() - same as previous Swapchain::new(), if an oldswapchain needs to be used
- **Breaking** Swapchain::new() now doesnt need to have the old_swapchain parameter anymore but requires the ColorSpace
- **Breaking** Decouple descriptor sets from pipeline
- **Breaking** Update Winit to 0.21.0
- **Breaking** Add `host_cached` field to all `CpuAccessibleBuffer` initializers to allow the user to perfer host cached memory.
- **Breaking** Added `fullscreen_exclusive` field to `Swapchain` initializers to allow the user to specify how fullscreen exclusivity should be handled.
    + Swapchain methods added: `Swapchain::acquire_fullscreen_exclusive()`, `Swapchain::release_fullscreen_exclusive()`, and `Swapchain::is_fullscreen_exclusive()`
- Add function `execute_commands_from_vec` to handle submission of multiple secondary command buffers.
- Allow `DebugCallback` to be sent between threads
- Pipeline barriers are now correctly inserted when a resource is used more than two times.
- Update MacOS dependency cocoa to 0.20
- Fixed code generated by `shader!` macro so that SSBO's are supported again (broken in 0.16.0).
- Added Swapchain::surface() - which returns the saved surface
- Propogate new lines correctly in shader compile errors.
- `Queue` and `QueueFamily` now implement `PartialEq` and `Eq`
- `Swapchain::acquire_next_image()`` now returns ``(image_id, suboptimal, aquire_future)``
    + *suboptimal indicates that the swapchain is usable, but should be recreated*
- Fixed Join Future implementation to not submit joined command buffers twice.
- The traits `GraphicsPipelineAbstract` and `DescriptorSet` now require `DeviceOwned`.
- Added `PartialEq`, `Eq` and `Hash` implementations to all types involved in a draw call, including:
  - `Instance`, `Device`, `GraphicsPipeline` and `dyn GraphicsPipelineAbstract`
  - `UnsafeBuffer` and all types implementing `BufferAccess`
  - `UnsafeImage`, `UnsafeImageView` and all types implementing `ImageAccess` or `ImageViewAccess`
  - All types implementing `DescriptorSet`

# Version 0.16.0 (2019-11-01)

- Fixed bug in examples causing OutOfHostMemory errors
- Replaced `VK_EXT_debug_report` `VK_EXT_debug_marker` with `VK_EXT_debug_utils`.
- Update MacOS dependencies metal to 0.17 and cocoa to 0.19
- Added dynamic stencil elements to `DynamicState`
- Fixed `ImageDimensions::mipmap_dimensions` and `max_mipmaps` in cases where the original size is not a power of two.
- Shader includes now work on Windows.
- **Breaking Change** Shader include directories passed to the `shader!` macro are now relative to the crates `Cargo.toml`
- Add support for `VK_KHR_16bit_storage` and `VK_KHR_storage_buffer_storage_class` device extensions.
- Update dependencies: lazy_static, half, syn, quote & proc-macro2
- Swapchain can now be recreated with dimensions of corresponding surface using `recreate()`.
- Added `raw_loaded_extensions()` to `Instance` to allow querying of all extensions, not just known ones.
- **Breaking Change** `loaded_extensions()` on `Instance` no longer returns a reference.
- Add support for GLSL macro defines to the `shader!` macro.
- Switch to Vulkan 1.1 and inherently SpirV 1.3 (shaderc default version for vulkan 1.1)

# Version 0.15.0 (2019-10-18)

*Yanked*

# Version 0.14.0 (2019-08-17)

- Update shaderc to 0.6. This again allows to use locally installed libraries which reduces the build-time significantly on Arch/Voidlinux (see https://github.com/google/shaderc-rs/issues/58)
- Removed faulty debug_assert in `SwapchainAcquireFuture::drop`.
- Compressed texture formats can now be uploaded using `ImmutableImage::from_iter`, `ImmutableImage::from_buffer`, `AutoCommandBuilder::copy_buffer_to_image_dimensions`,
  and `AutoCommandBuilder::copy_buffer_to_image`.

# Version 0.13.0 (2019-07-02)

This is an emergency breaking breaking change. It fixes Undefined Behaviour that was preventing the compilation of Vulkano on the latest rust nightlies.

- Structs that have the `impl_vertex` macro applied to them, now also need to `#[derive(Default)]`.

# Version 0.12.0 (2019-05-24)

- Update shaderc to 0.5.  New shaderc has improved pre-built options for libshaderc that significantly reduce package build time and are appropriate for use in CI
- `QueueFamily::explicitly_supports_tranfers` only returns true if `vk::QUEUE_TRANSFER_BIT` is set instead of also always returning true.  Removed `supports_transfers`.
- Update to winit 0.19
- Add support for `#include "..."` and `#include <...>` directives within source
  files.
- Add a `union` method for the extensions types.
- Make `BufferUsage` derive `PartialEq` and `Eq`
- Fix a compiler error in struct code generated by the `shader!` macro.
- Improve inference of image layout transitions and in turn image memory barrier
  behaviour.

# Version 0.11.1 (2018-11-16)

- Expose `CopyImageError` and `DrawIndexedIndirectError`.

# Version 0.11.0 (2018-11-08) (**yanked** because vk-sys 0.3.4 was accidentally breaking vulkano 0.10)

- Update to winit 0.18
- Export features and device extensions from the device module instead of the instance module
    +   `instance::Features` -> `device::Features`
    +   `instance::DeviceExtensions` -> `device::DeviceExtensions`
    +   `instance::RawDeviceExtensions` -> `device::RawDeviceExtensions`
- Added `vulkano_shaders::shader!` proc macro, use this instead of `vulkano_shader_deriver::VulkanoShaders`.
- The entire `vulkano_shader_derive` crate is deprecated.
- `vulkano_shaders::{reflect, compile, Error}` are no longer public.
- Remove vulkano_shaders::build_glsl_shaders
- Removed mir support, as it is being removed from the vulkan spec.
- Split `PersistentDescriptorSetError::MissingUsage` into `MissingImageUsage` and `MissingBufferUsage`
  each with a matching enum indicating the usage that was missing.
- Fix instance_count when using draw_index with instance buffers
- Added a `reinterpret` function to `BufferSlice`
- Made `AttributeInfo` derive `Copy`, `Clone` and `Debug`
- Use [google/shaderc](https://github.com/google/shaderc-rs) for shader compilation
- Reject generation of rust types for SPIR-V arrays that would have incorrect array stride.
- Removed the `Layout` prefix of the descriptions used for a render pass.
- Implemented DescriptorSetCollection for `Vec<T>` which allows easier use of construction them for usage when drawing.

# Version 0.10.0 (2018-08-10)

- Use dynamically loaded `libvulkan` like on other platforms instead of linking to MoltenVK on macOS
- Updated winit to version 0.17.
- Allow custom implementations of `RenderPassDesc` to specify `VK_SUBPASS_EXTERNAL` as a dependency source or destination
- Added `vulkano_win::create_vk_surface` which allows creating a surface safely without taking ownership of
  the window.
- `AutoCommandBufferBuilder::draw` and friends no longer consume the `DynamicState` argument, allowing reuse between calls.
- `Instance::new` and `Instance::with_loader` now take in the layers as an iterator of borrowed `str`s, not of references to
borrowed `str`s.

# Version 0.9.0 (2018-03-13)

- Updated winit to version 0.11.

# Version 0.8.0 (2018-03-11)

- Changed `ShaderInterfaceMismatchError` to be more verbose.
- Allow depth/stencil images to be used with `AutoCommandBufferBuilder::copy_image_to_buffer()`
- Allow `Surface` to own the window it references.
- Clear value validation for `AutoCommandBufferBuilder::begin_render_pass()`
- Fix occasional truncation of glslang_validator when glsl-to-spirv is rebuilt
- Fix linking against MoltenVK >= 0.19.0
- Fix panic on DeviceLost error after resizing swapchain on nvidia/amd hardware
- Added `AutoCommandBufferBuilder::copy_image`
- Added `VulkanObject::TYPE` to look up the `DebugReportObjectTypeEXT` of an object
- Added `Device::set_object_name` and `Device::set_object_name_raw`
- Added `GraphicsPipelineBuilder::with_auto_layout` to simplify use of dynamic buffers.

# Version 0.7.3 (2018-02-10)

*Yanked*

# Version 0.7.2 (2017-10-09)

- Allow `impl_vertex!` to support generic structs.
- Added `GraphicsPipelineBuilder::with_pipeline_layout`.
- Fixed creating a buffer view not checking the `min_texel_buffer_offset_alignment` limit.
- Added support for loading the `VK_EXT_debug_marker` extension and adding debug markers to
  `UnsafeCommandBufferBuilder`
- Changed `GraphicsPipelineBuilder` to default to a buffer-less vertex input.
- Deprecated `pipeline::input_assembly::InputAssembly`.

# Version 0.7.1 (2017-09-28)

- Fixed an HiDPI scaling issue on MacOS in vulkano-win.
- Fixed `le()` and `ge()` for `DescriptorsCount`, which also fixes a potential memory leak when
  allocating descriptor sets.
- Fixed the `ordered_passes_renderpass!` macro not working properly when using `resolve`.
- Added an optional `resolve` entry in `pass` for `single_pass_renderpass!`, for resolving
  multisampled attachments into non-multisampled attachments.
- Fixed the value of `rasterizationSamples` not corresponding to the render pass.
- Added `GraphicsPipelineBuilder::sample_shading_disabled`, `sample_shading_enabled`,
  `alpha_to_coverage_disabled`, `alpha_to_coverage_enabled`, `alpha_to_one_disabled` and
  `alpha_to_one_enabled`. Added `GraphicsPipelineCreationError::SampleRateShadingFeatureNotEnabled`
  and `AlphaToOneFeatureNotEnabled`.
- Deprecated `pipeline::multisample::Multisample`.

# Version 0.7.0 (2017-09-21)

- Added `RuntimePipelineDesc`, an implementation of `PipelineLayoutDesc` that makes creating custom
  layouts easier.
- Changed `CpuBufferPool::next()` and `chunk()` to return a `Result` in case of an error when
  allocating or mapping memory.
- Changed `CommandBufferExecError::AccessError` to provide a hint of where the error occurs.
- Added `vulkano::pipeline::vertex::BufferlessDefinition` and `BufferlessVertices` to enable
  bufferless drawing.
- Changed `ImageAccess::try_gpu_lock` and `unlock()` to verify whether the image layout is correct,
  especially at the first usage of an image.
- Changed `BufferAccess::conflict_*` and `ImageAccess::conflict_*` to forbid querying a specific
  range of the resource.
- Fixed `layers` argument validation in `Swapchain::new_inner`.
- Added a 32-bit word constructor for `ShaderModule` (`ShaderModule::from_words`).
- Changed the various `is_superset_of` functions to return a `Result` instead of a `bool`.

# Version 0.6.2 (2017-09-06)

- Added checking compatibility between the descriptor sets and the pipeline object when adding a
  draw or compute command.
- Fixed several bugs in the validation checks of `blit_image`.
- Fixed `blit_image`, `copy_buffer_to_image` and `copy_image_to_buffer` not taking the mipmap level
  into account when checking for the correct image dimensions.
- Added `ImageDimensions::mipmap_dimensions()` and `ImageDimensions::num_mipmaps()`.
- Added an implementation of `DeviceOwned` for `QueuesIter` and `Queue`.
- Fixed the standard command pool and descriptor pool being destroyed too often.
- Added `#[allow(non_snake_case)]` to structs generated by vulkano-shaders.
