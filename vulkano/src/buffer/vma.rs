use super::{
    sys::UnsafeBuffer, BufferAccess, BufferInner, BufferUsage,
};
use crate::{
    device::{Device, DeviceOwned},
    DeviceSize,
};
use ash::vk::BufferCreateInfo;
use lazy_static::__Deref;
use std::sync::Arc;
use vk_mem;
use parking_lot::Mutex;

pub struct VmaBuffer {
    allocation_info: vk_mem::AllocationInfo,
    allocation: Mutex<Arc<vk_mem::Allocation>>,
    inner: Arc<UnsafeBuffer>,
}

unsafe impl Sync for VmaBuffer {}
unsafe impl Send for VmaBuffer {}

impl VmaBuffer {
    pub fn allocate(
        device: Arc<Device>,
        allocator: Arc<vk_mem::Allocator>,
        usage: BufferUsage,
        size: usize,
    ) -> Arc<VmaBuffer> {
        let allocation_create_info =
            vk_mem::AllocationCreateInfo::new().usage(vk_mem::MemoryUsage::CpuToGpu);
        let buffer_create_info = BufferCreateInfo::builder()
            .size(size as u64)
            .usage(usage.into())
            .build();
        // TODO error handling
        let (buffer, allocation, allocation_info) =
            unsafe { allocator.create_buffer(&buffer_create_info, &allocation_create_info) }
                .unwrap();

        let unsafe_buffer =
            UnsafeBuffer::from_raw_parts(buffer, device.clone(), size as DeviceSize, usage);

        return Arc::new(VmaBuffer {
            allocation_info,
            allocation: Mutex::new(Arc::new(allocation)),
            inner: unsafe_buffer,
        });
    }

    pub unsafe fn unmap(&self, allocator: Arc<vk_mem::Allocator>) {
        // TODO proper error handling
        let lock = self.allocation.lock();
        allocator.unmap_memory(*lock.deref().as_ref());
    }

    pub unsafe fn map(&self, allocator: Arc<vk_mem::Allocator>) -> *mut u8 {
        // TODO proper error handling
        let lock = self.allocation.lock();
        let result = allocator.map_memory(*lock.deref().as_ref());
        return result.unwrap();
    }
}

unsafe impl DeviceOwned for VmaBuffer {
    #[inline]
    fn device(&self) -> &Arc<Device> {
        self.inner.device()
    }
}

unsafe impl BufferAccess for VmaBuffer {
    #[inline]
    fn inner(&self) -> BufferInner {
        BufferInner {
            buffer: &self.inner,
            offset: 0,
        }
    }

    #[inline]
    fn size(&self) -> DeviceSize {
        self.inner.size()
    }
}
