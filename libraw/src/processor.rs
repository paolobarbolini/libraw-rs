use crate::{Error, ProcessedImage, Result};
use libraw_sys as sys;

pub struct Processor {
    inner: *mut sys::libraw_data_t,
}

impl Processor {
    pub fn new() -> Self {
        let inner = unsafe { sys::libraw_init(0) };
        Self { inner }
    }

    pub fn decode(self, mut buf: Vec<u8>) -> Result<ProcessedImage> {
        Error::check(unsafe {
            sys::libraw_open_buffer(self.inner, buf.as_mut_ptr() as *mut _, buf.len())
        })?;
        Error::check(unsafe { sys::libraw_unpack(self.inner) })?;
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
