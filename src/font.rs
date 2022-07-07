use std::{ffi, mem, ptr};
extern crate core;

type Font = *mut ffi::c_void;
type Char = u32;

#[repr(C)]
pub struct LineMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
    pub new_line_size: f32,
}

#[repr(C)]
pub struct Metrics {
    pub xmin: i32,
    pub ymin: i32,
    pub width: usize,
    pub height: usize,
    pub advance_width: f32,
    pub advance_height: f32,
    pub bounds: OutlineBounds,
}

#[repr(C)]
pub struct OutlineBounds {
    pub xmin: f32,
    pub ymin: f32,
    pub width: f32,
    pub height: f32,
}

#[repr(C)]
pub struct RasterizedBitmap {
    pub metrics: Metrics,
    pub data: *mut u8,
    pub data_length: usize,
}

#[repr(C)]
pub struct GlyphRasterConfig {
    pub glyph_index: u16,
    pub px: f32,
    pub font_hash: usize,
}

//

impl From<fontdue::Metrics> for Metrics {
    fn from(metrics: fontdue::Metrics) -> Self {
        Self {
            xmin: metrics.xmin,
            ymin: metrics.ymin,
            width: metrics.width,
            height: metrics.height,
            advance_width: metrics.advance_width,
            advance_height: metrics.advance_height,
            bounds: OutlineBounds {
                xmin: metrics.bounds.xmin,
                ymin: metrics.bounds.ymin,
                width: metrics.bounds.width,
                height: metrics.bounds.height,
            },
        }
    }
}

impl From<GlyphRasterConfig> for fontdue::layout::GlyphRasterConfig {
    fn from(config: GlyphRasterConfig) -> Self {
        Self {
            glyph_index: config.glyph_index,
            px: config.px,
            font_hash: config.font_hash,
        }
    }
}

//

#[no_mangle]
pub extern "C" fn ftd_font_from_bytes(bytes: *mut u8, size: usize) -> Font {
    let buf = unsafe { core::slice::from_raw_parts_mut(bytes, size) };
    let loaded_font = fontdue::Font::from_bytes(buf, fontdue::FontSettings::default()).unwrap();
    return Box::<fontdue::Font>::into_raw(Box::new(loaded_font)) as Font;
}

#[no_mangle]
pub extern "C" fn ftd_font_drop(font: Font) {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        drop(Box::from_raw(ptr));
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_file_hash(font: Font) -> usize {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        (*ptr).file_hash()
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_horizontal_line_metrics(
    font: Font,
    px: f32,
    line_metrics: *mut LineMetrics,
) -> bool {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        if let Some(metrics) = ptr.as_ref().unwrap().horizontal_line_metrics(px) {
            *line_metrics = LineMetrics {
                ascent: metrics.ascent,
                descent: metrics.descent,
                line_gap: metrics.line_gap,
                new_line_size: metrics.new_line_size,
            };
            return true;
        } else {
            return false;
        }
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_vertical_line_metrics(
    font: Font,
    px: f32,
    line_metrics: *mut LineMetrics,
) -> bool {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        if let Some(metrics) = ptr.as_ref().unwrap().vertical_line_metrics(px) {
            *line_metrics = LineMetrics {
                ascent: metrics.ascent,
                descent: metrics.descent,
                line_gap: metrics.line_gap,
                new_line_size: metrics.new_line_size,
            };
            return true;
        } else {
            return false;
        }
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_units_per_em(font: Font) -> f32 {
    unsafe {
        (font as *mut fontdue::Font)
            .as_ref()
            .unwrap()
            .units_per_em()
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_scale_factor(font: Font, px: f32) -> f32 {
    unsafe {
        (font as *mut fontdue::Font)
            .as_ref()
            .unwrap()
            .scale_factor(px)
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_horizontal_kern(
    font: Font,
    left: Char,
    right: Char,
    px: f32,
    kerning: *mut f32,
) -> bool {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        if let Some(kern) = ptr.as_ref().unwrap().horizontal_kern(
            char::from_u32(left as u32).unwrap(),
            char::from_u32(right as u32).unwrap(),
            px,
        ) {
            *kerning = kern;
            return true;
        } else {
            return false;
        }
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_horizontal_kern_indexed(
    font: Font,
    left: u16,
    right: u16,
    px: f32,
    kerning: *mut f32,
) -> bool {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        if let Some(kern) = ptr
            .as_ref()
            .unwrap()
            .horizontal_kern_indexed(left, right, px)
        {
            *kerning = kern;
            return true;
        } else {
            return false;
        }
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_metrics(font: Font, character: Char, px: f32, metrics: *mut Metrics) {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        let met = ptr
            .as_ref()
            .unwrap()
            .metrics(char::from_u32(character as u32).unwrap(), px);
        *metrics = met.into();
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_metrics_indexed(font: Font, index: u16, px: f32, metrics: *mut Metrics) {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        let met = ptr.as_ref().unwrap().metrics_indexed(index, px);
        *metrics = met.into();
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_rasterize_config(
    font: Font,
    config: GlyphRasterConfig,
    bitmap: *mut RasterizedBitmap,
) {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        let (met, bits) = ptr.as_ref().unwrap().rasterize_config(config.into());
        (*bitmap).metrics = met.into();
        std::ptr::copy(
            bits.as_ptr(),
            (*bitmap).data,
            bits.len() * std::mem::size_of::<u8>(),
        );
        (*bitmap).data_length = bits.len();
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_rasterize(
    font: Font,
    character: Char,
    px: f32,
    bitmap: *mut RasterizedBitmap,
) {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        let (met, bits) = ptr
            .as_ref()
            .unwrap()
            .rasterize(char::from_u32(character).unwrap(), px);
        (*bitmap).metrics = met.into();
        std::ptr::copy(
            bits.as_ptr(),
            (*bitmap).data,
            bits.len() * std::mem::size_of::<u8>(),
        );
        (*bitmap).data_length = bits.len();
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_rasterize_config_subpixel(
    font: Font,
    config: GlyphRasterConfig,
    bitmap: *mut RasterizedBitmap,
) {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        let (met, bits) = ptr
            .as_ref()
            .unwrap()
            .rasterize_config_subpixel(config.into());
        (*bitmap).metrics = met.into();
        std::ptr::copy(
            bits.as_ptr(),
            (*bitmap).data,
            bits.len() * std::mem::size_of::<u8>(),
        );
        (*bitmap).data_length = bits.len();
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_rasterize_subpixel(
    font: Font,
    character: Char,
    px: f32,
    bitmap: *mut RasterizedBitmap,
) {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        let (met, bits) = ptr
            .as_ref()
            .unwrap()
            .rasterize_subpixel(char::from_u32(character).unwrap(), px);
        (*bitmap).metrics = met.into();
        std::ptr::copy(
            bits.as_ptr(),
            (*bitmap).data,
            bits.len() * std::mem::size_of::<u8>(),
        );
        (*bitmap).data_length = bits.len();
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_rasterize_indexed(
    font: Font,
    index: u16,
    px: f32,
    bitmap: *mut RasterizedBitmap,
) {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        let (met, bits) = ptr.as_ref().unwrap().rasterize_indexed(index, px);
        (*bitmap).metrics = met.into();
        std::ptr::copy(
            bits.as_ptr(),
            (*bitmap).data,
            bits.len() * std::mem::size_of::<u8>(),
        );
        (*bitmap).data_length = bits.len();
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_rasterize_indexed_subpixel(
    font: Font,
    index: u16,
    px: f32,
    bitmap: *mut RasterizedBitmap,
) {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        let (met, bits) = ptr.as_ref().unwrap().rasterize_indexed_subpixel(index, px);
        (*bitmap).metrics = met.into();
        std::ptr::copy(
            bits.as_ptr(),
            (*bitmap).data,
            bits.len() * std::mem::size_of::<u8>(),
        );
        (*bitmap).data_length = bits.len();
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_lookup_glyph_index(font: Font, character: Char) -> u16 {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        return ptr
            .as_ref()
            .unwrap()
            .lookup_glyph_index(char::from_u32(character).unwrap());
    }
}

#[no_mangle]
pub extern "C" fn ftd_font_glyph_count(font: Font) -> u16 {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        return ptr.as_ref().unwrap().glyph_count();
    }
}
