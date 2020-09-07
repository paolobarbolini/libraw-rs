use std::ops::Deref;
use std::slice;

use libraw_rs_sys as sys;

pub struct ProcessedImage {
    inner: *mut sys::libraw_processed_image_t,
}

impl ProcessedImage {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::libraw_processed_image_t) -> Self {
        debug_assert!(!ptr.is_null());

        Self { inner: ptr }
    }

    pub fn width(&self) -> u32 {
        unsafe { (*self.inner).width }.into()
    }

    pub fn height(&self) -> u32 {
        unsafe { (*self.inner).height }.into()
    }
}

impl Deref for ProcessedImage {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            slice::from_raw_parts(
                (*self.inner).data.as_ptr(),
                (*self.inner).data_size as usize,
            )
        }
    }
}

impl Drop for ProcessedImage {
    fn drop(&mut self) {
        unsafe { sys::libraw_dcraw_clear_mem(self.inner) }
    }
}
