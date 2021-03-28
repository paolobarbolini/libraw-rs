use std::mem;

use crate::{BitDepth, Error, ProcessedImage, RawImage, Result};
use libraw_sys as sys;

pub struct Processor {
    pub(crate) inner: *mut sys::libraw_data_t,
}

impl Processor {
    pub fn new() -> Self {
        let inner = unsafe { sys::libraw_init(0) };
        Self { inner }
    }

    fn open(&self, buf: &[u8]) -> Result<()> {
        Error::check(unsafe {
            sys::libraw_open_buffer(self.inner, buf.as_ptr() as *const _, buf.len())
        })?;
        Error::check(unsafe { sys::libraw_unpack(self.inner) })?;

        Ok(())
    }

    pub fn decode(self, buf: &[u8]) -> Result<RawImage> {
        self.open(buf)?;

        let decoded = RawImage::new(self);
        Ok(decoded)
    }

    #[inline]
    pub fn process_8bit(self, buf: &[u8]) -> Result<ProcessedImage<u8>> {
        self.process(buf)
    }

    #[inline]
    pub fn process_16bit(self, buf: &[u8]) -> Result<ProcessedImage<u16>> {
        self.process(buf)
    }

    fn process<T: BitDepth>(self, buf: &[u8]) -> Result<ProcessedImage<T>> {
        let bps = mem::size_of::<T>() * 8;
        debug_assert!(bps == 8 || bps == 16);
        unsafe { (*self.inner).params.output_bps = bps as i32 };

        self.open(buf)?;
        Error::check(unsafe { sys::libraw_dcraw_process(self.inner) })?;

        let mut result = 0i32;
        let processed = unsafe { sys::libraw_dcraw_make_mem_image(self.inner, &mut result) };
        Error::check(result)?;

        let processed = unsafe { ProcessedImage::from_raw(processed) };
        Ok(processed)
    }
}

impl Drop for Processor {
    fn drop(&mut self) {
        unsafe { sys::libraw_close(self.inner) }
    }
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}
