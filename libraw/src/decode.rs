use crate::Sizes;

use libraw_sys as sys;

pub struct DecodedImage {
    pub rawdata: Vec<u16>,
    pub sizes: Sizes,
}

impl DecodedImage {
    pub(crate) fn new(
        rawdata_ref: &sys::libraw_rawdata_t,
        sizes_ref: &sys::libraw_image_sizes_t,
    ) -> Self {
        let sizes = Sizes::new(sizes_ref);
        let mut rawdata: Vec<u16> =
            Vec::with_capacity(sizes.raw_width as usize * sizes.raw_height as usize);
        unsafe {
            let vec_ptr = rawdata.as_mut_ptr();
            let capacity = rawdata.capacity();
            vec_ptr.copy_from(rawdata_ref.raw_image, capacity);
            rawdata.set_len(capacity);
        }

        Self { rawdata, sizes }
    }
}
