use std::{convert::TryInto, slice};

use libraw_sys as sys;
use sys::{
    LibRaw_colorspace_LIBRAW_COLORSPACE_AdobeRGB, LibRaw_colorspace_LIBRAW_COLORSPACE_Unknown,
    LibRaw_colorspace_LIBRAW_COLORSPACE_sRGB,
};

#[allow(non_snake_case)]
pub struct Colordata {
    pub curve: [u16; 65536],
    pub black: u32,

    // These are all from cblack
    pub black_per_channel: [u32; 4],
    pub black_block_width: u32,
    pub black_block_height: u32,
    pub black_block_data: [u32; 4098],

    pub maximum: u32,

    //NOTE: The docs lnked below call this an unsigned, but it's a signed 64?
    // https://www.libraw.org/docs/API-datastruct.html#libraw_colordata_t
    pub linear_max: [i64; 4],
    pub fmaximum: f32,
    pub fnorm: f32,
    pub white: [[u16; 8]; 8],
    pub cam_xyz: [[f32; 3]; 4],
    // WB coefficients as shot
    pub cam_mul: [f32; 4],
    // WB coefficients for Daylight
    pub pre_mul: [f32; 4],
    pub cmatrix: [[f32; 4]; 3], // Not clear what this is
    pub rgb_cam: [[f32; 4]; 3],

    pub phase_one_data: PhaseOne,
    pub flash_used: f32, // Why the heck is this a float. Can it be a bool?
    pub canon_ev: f32,

    pub model2: [i8; 64], // NOTE: How to stringify this? Should I?
    pub UniqueCameraModel: [i8; 64],
    pub LocalizedCameraModel: [i8; 64],

    pub profile: Vec<u8>,
    pub black_stat: [u32; 8],

    pub dng_color: [DngColor; 2],
    pub dng_levels: DngLevels,

    pub WB_Coeffs: [[i32; 4]; 256],
    pub WBCT_Coeffs: [[f32; 5]; 64],
    pub as_shot_wb_applied: bool,

    pub P1_color: [PhaseOneColor; 2],
    pub raw_bps: u32,

    pub exif_color_space: ExifColorSpace,
}

impl Colordata {
    pub(crate) fn new(colordata: sys::libraw_colordata_t) -> Self {
        let profile = unsafe {
            slice::from_raw_parts(
                colordata.profile as *mut u8,
                colordata.profile_length as usize,
            )
        }
        .to_vec();

        Colordata {
            curve: colordata.curve,
            black: colordata.black,
            black_per_channel: colordata.cblack[0..4].try_into().unwrap(),
            black_block_width: colordata.cblack[4],
            black_block_height: colordata.cblack[5],
            black_block_data: colordata.cblack[6..].try_into().unwrap(),
            maximum: colordata.maximum,
            linear_max: colordata.linear_max,
            fmaximum: colordata.fmaximum,
            fnorm: colordata.fnorm,
            white: colordata.white,
            cam_xyz: colordata.cam_xyz,
            cam_mul: colordata.cam_mul,
            pre_mul: colordata.pre_mul,
            cmatrix: colordata.cmatrix,
            rgb_cam: colordata.rgb_cam,
            phase_one_data: colordata.phase_one_data.into(),
            flash_used: colordata.flash_used,
            canon_ev: colordata.canon_ev,
            model2: colordata.model2,
            UniqueCameraModel: colordata.UniqueCameraModel,
            LocalizedCameraModel: colordata.LocalizedCameraModel,
            profile,
            black_stat: colordata.black_stat,
            dng_color: [colordata.dng_color[0].into(), colordata.dng_color[1].into()],
            dng_levels: colordata.dng_levels.into(),
            as_shot_wb_applied: colordata.as_shot_wb_applied == 1,
            P1_color: [colordata.P1_color[0].into(), colordata.P1_color[1].into()],
            WB_Coeffs: colordata.WB_Coeffs,
            WBCT_Coeffs: colordata.WBCT_Coeffs,
            raw_bps: colordata.raw_bps,
            exif_color_space: ExifColorSpace::from_sys(colordata.ExifColorSpace),
        }
    }
}

pub struct PhaseOne {
    pub format: i32,
    pub key_off: i32,
    pub tag_21a: i32,
    pub t_black: i32,
    pub split_col: i32,
    pub black_col: i32,
    pub split_row: i32,
    pub black_row: i32,
    pub tag_210: f32,
}

impl From<sys::ph1_t> for PhaseOne {
    fn from(ph1: sys::ph1_t) -> Self {
        PhaseOne {
            format: ph1.format,
            key_off: ph1.key_off,
            tag_21a: ph1.tag_21a,
            t_black: ph1.t_black,
            split_col: ph1.split_col,
            black_col: ph1.black_col,
            split_row: ph1.split_row,
            black_row: ph1.black_row,
            tag_210: ph1.tag_210,
        }
    }
}

pub struct DngColor {
    pub parsedfields: u32,
    pub illuminant: u16,
    pub calibration: [[f32; 4]; 4],
    pub colormatrix: [[f32; 3]; 4],
    pub forwardmatrix: [[f32; 4]; 3],
}

impl From<sys::libraw_dng_color_t> for DngColor {
    fn from(dng: sys::libraw_dng_color_t) -> Self {
        Self {
            parsedfields: dng.parsedfields,
            illuminant: dng.illuminant,
            calibration: dng.calibration,
            colormatrix: dng.colormatrix,
            forwardmatrix: dng.forwardmatrix,
        }
    }
}

#[allow(non_snake_case)]
pub struct DngLevels {
    pub parsedfields: u32,
    //TODO: Separate out fields of cblack like in the main Colordata struct
    pub cblack: [u32; 4104],
    pub black: u32,
    pub fcblack: [f32; 4104],
    pub fblack: f32,
    pub whitelevel: [u32; 4],
    pub default_crop: [u32; 4],
    pub preview_colorspace: u32,
    pub analogbalance: [f32; 4],
    pub baseline_exposure: f32,
    pub LinearResponseLimit: f32,
}

impl From<sys::libraw_dng_levels_t> for DngLevels {
    fn from(dng: sys::libraw_dng_levels_t) -> Self {
        Self {
            parsedfields: dng.parsedfields,
            cblack: dng.dng_cblack,
            black: dng.dng_black,
            fcblack: dng.dng_fcblack,
            fblack: dng.dng_fblack,
            whitelevel: dng.dng_whitelevel,
            default_crop: dng.default_crop,
            preview_colorspace: dng.preview_colorspace,
            analogbalance: dng.analogbalance,
            baseline_exposure: dng.baseline_exposure,
            LinearResponseLimit: dng.LinearResponseLimit,
        }
    }
}

pub struct PhaseOneColor {
    pub romm_cam: [f32; 9],
}

impl From<sys::libraw_P1_color_t> for PhaseOneColor {
    fn from(p1: sys::libraw_P1_color_t) -> Self {
        Self {
            romm_cam: p1.romm_cam,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum ExifColorSpace {
    Unknown,
    sRGB,
    AdobeRgb,
}

impl ExifColorSpace {
    fn from_sys(sys_int: i32) -> Self {
        #[allow(non_upper_case_globals)]
        match sys_int as u32 {
            LibRaw_colorspace_LIBRAW_COLORSPACE_Unknown => ExifColorSpace::Unknown,
            LibRaw_colorspace_LIBRAW_COLORSPACE_AdobeRGB => ExifColorSpace::AdobeRgb,
            LibRaw_colorspace_LIBRAW_COLORSPACE_sRGB => ExifColorSpace::sRGB,
            _ => panic!(),
        }
    }
}
