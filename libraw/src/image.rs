use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::slice;

use crate::BitDepth;
use libraw_sys as sys;

pub struct ProcessedImage<T> {
    inner: *mut sys::libraw_processed_image_t,
    marker_: PhantomData<T>,
}

impl<T: BitDepth> ProcessedImage<T> {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::libraw_processed_image_t) -> Self {
        debug_assert!(!ptr.is_null());
        debug_assert_eq!((*ptr).bits as usize, mem::size_of::<T>() * 8);

        Self {
            inner: ptr,
            marker_: PhantomData,
        }
    }

    pub fn width(&self) -> u32 {
        unsafe { (*self.inner).width }.into()
    }

    pub fn height(&self) -> u32 {
        unsafe { (*self.inner).height }.into()
    }

    pub fn colors(&self) -> u16 {
        unsafe { (*self.inner).colors }
    }

    pub fn pixels(&self) -> Option<&[u8]> {
        let pixels = unsafe {
            slice::from_raw_parts(
                (*self.inner).data.as_ptr(),
                (*self.inner).data_size as usize,
            )
        };
        Some(pixels)
    }
}

impl Deref for ProcessedImage<u8> {
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

impl Deref for ProcessedImage<u16> {
    type Target = [u16];

    fn deref(&self) -> &Self::Target {
        unsafe {
            debug_assert_eq!((*self.inner).data.as_ptr() as usize % 2, 0);

            slice::from_raw_parts(
                (*self.inner).data.as_ptr() as *const u16,
                (*self.inner).data_size as usize / 2,
            )
        }
    }
}

impl<T> Drop for ProcessedImage<T> {
    fn drop(&mut self) {
        unsafe { sys::libraw_dcraw_clear_mem(self.inner) }
    }
}
