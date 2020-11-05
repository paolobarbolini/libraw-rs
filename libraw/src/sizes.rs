use libraw_sys as sys;

#[derive(Debug, Copy, Clone)]
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
    pub(crate) fn new(sizes: sys::libraw_image_sizes_t) -> Self {
        Self {
            raw_height: sizes.raw_height,
            raw_width: sizes.raw_width,
            height: sizes.height,
            width: sizes.width,
            top_margin: sizes.top_margin,
            left_margin: sizes.left_margin,
            iheight: sizes.iheight,
            iwidth: sizes.iwidth,
            raw_pitch: sizes.raw_pitch,
            pixel_aspect: sizes.pixel_aspect,
            flip: sizes.flip,
        }
    }
}
