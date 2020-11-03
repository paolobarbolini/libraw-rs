use crate::{DecodedImage, Error, Result};
use libraw_sys as sys;

pub struct Decoder {
    inner: *mut sys::libraw_data_t,
}

impl Decoder {
    pub fn new() -> Self {
        let inner = unsafe { sys::libraw_init(0) };
        Self { inner }
    }

    pub fn decode(self, buf: &[u8]) -> Result<DecodedImage> {
        Error::check(unsafe {
            sys::libraw_open_buffer(self.inner, buf.as_ptr() as *const _, buf.len())
        })?;
        Error::check(unsafe { sys::libraw_unpack(self.inner) })?;
        debug_assert!(!unsafe { (*self.inner).rawdata.raw_alloc }.is_null());

        let decoded = DecodedImage::new(&unsafe { (*self.inner).rawdata }, &unsafe {
            (*self.inner).sizes
        });
        Ok(decoded)
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe { sys::libraw_close(self.inner) }
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}
