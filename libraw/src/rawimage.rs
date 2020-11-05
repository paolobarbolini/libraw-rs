use crate::{Processor, Sizes};
use std::marker::PhantomData;
use std::ops::Deref;
use std::slice;

use libraw_sys as sys;

pub struct RawImage<'a> {
    rawdata: *const sys::libraw_rawdata_t,
    pub sizes: Sizes,
    phantom: PhantomData<&'a Processor>,
}

impl<'a> RawImage<'a> {
    pub(crate) fn new(processor: &'a Processor) -> Self {
        Self {
            rawdata: unsafe { &(*processor.inner).rawdata },
            sizes: Sizes::new(unsafe { (*processor.inner).sizes }),
            phantom: PhantomData,
        }
    }
}

impl<'a> Deref for RawImage<'a> {
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
