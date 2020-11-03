use libraw_sys as sys;

pub struct Sizes {
    pub raw_height: u16,
    pub raw_width: u16,
    pub height: u16,
    pub width: u16,
    pub top_margin: u16,
    pub left_margin: u16,
    pub iheight: u16,
    pub iwidth: u16,
    pub raw_pitch: u32,
    pub pixel_aspect: f64,
    pub flip: i32,
}

impl Sizes {
    pub(crate) fn new(sizes: &sys::libraw_image_sizes_t) -> Self {
        let deref = *sizes;

        Self {
            raw_height: deref.raw_height,
            raw_width: deref.raw_width,
            height: deref.height,
            width: deref.width,
            top_margin: deref.top_margin,
            left_margin: deref.left_margin,
            iheight: deref.iheight,
            iwidth: deref.iwidth,
            raw_pitch: deref.raw_pitch,
            pixel_aspect: deref.pixel_aspect,
            flip: deref.flip,
        }
    }
}
