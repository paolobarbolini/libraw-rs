use crate::{Processor, Sizes};
use std::ops::Deref;
use std::slice;

pub struct RawImage {
    processor: Processor
}

impl RawImage {
    pub(crate) fn new(processor: Processor) -> Self {
        Self {
            processor: processor
        }
    }

    pub fn sizes(&self) -> Sizes {
        Sizes::new(unsafe { (*self.processor.inner).sizes })
    }
}

impl Deref for RawImage {
    type Target = [u16];

    fn deref(&self) -> &Self::Target {
        let sizes = self.sizes();

        unsafe {
            slice::from_raw_parts(
                (*self.processor.inner).rawdata.raw_image,
                sizes.raw_width as usize * sizes.raw_height as usize,
            )
        }
    }
}
