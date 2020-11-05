use crate::{Processor, Sizes};
use std::ops::Deref;
use std::slice;

use libraw_sys as sys;

pub struct RawImage {
    _processor: Processor,
    rawdata: *const sys::libraw_rawdata_t,
    pub sizes: Sizes,
}

impl RawImage {
    pub(crate) fn new(processor: Processor) -> Self {
        let rawdata: *const sys::libraw_rawdata_t = unsafe { &(*processor.inner).rawdata };
        let sizes = Sizes::new(unsafe { (*processor.inner).sizes });

        Self {
            _processor: processor,
            rawdata,
            sizes,
        }
    }
}

impl Deref for RawImage {
    type Target = [u16];

    fn deref(&self) -> &Self::Target {
        unsafe {
            slice::from_raw_parts(
                (*self.rawdata).raw_image,
                self.sizes.raw_width as usize * self.sizes.raw_height as usize,
            )
        }
    }
}
