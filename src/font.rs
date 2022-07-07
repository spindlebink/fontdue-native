extern crate alloc;

use crate::*;
use core::{mem, ptr};

/// Allocates a font from an array of bytes.
#[no_mangle]
pub extern "C" fn ftd_font_create_from_bytes(bytes: *mut u8, size: usize) -> Font {
    unsafe {
        let buf = core::slice::from_raw_parts_mut(bytes, size);
        let loaded_font = fontdue::Font::from_bytes(buf, fontdue::FontSettings::default()).unwrap();
        return Box::<fontdue::Font>::into_raw(Box::new(loaded_font)) as Font;
    }
}

/// Frees a font previously allocated with `ftd_font_from_bytes`.
#[no_mangle]
pub extern "C" fn ftd_font_free(font: Font) {
    unsafe {
        let ptr = font as *mut fontdue::Font;
        drop(Box::from_raw(ptr));
    }
}

/// Retrieves char -> index mappings from the font, populating `chars` with the
/// glyph pairs.
///
/// To determine the necessary size for `chars`, you'll need to use
/// `ftd_font_char_count`. This function can't do any bounds checking.
#[no_mangle]
pub extern "C" fn ftd_font_chars(font: Font, chars: *mut GlyphMapping) {
    unsafe {
        let ptr = font as *const fontdue::Font;
        let map = ptr.as_ref().unwrap().chars();
        let mut i = 0;
        for (&char, &ind) in map {
            (*chars.offset(i)).character = char.into();
            (*chars.offset(i)).index = ind.into();
            i += 1;
        }
    }
}

/// Returns the number of available unicode characters in the font. The result
/// will be the minimum buffer size required for a call to `ftd_font_chars`.
#[no_mangle]
pub extern "C" fn ftd_font_char_count(font: Font) -> usize {
    unsafe {
        let ptr = font as *const fontdue::Font;
        ptr.as_ref().unwrap().chars().len()
    }
}

/// Returns a precomputed hash for the font file.
#[no_mangle]
pub extern "C" fn ftd_font_file_hash(font: Font) -> usize {
    unsafe {
        let ptr = font as *const fontdue::Font;
        (*ptr).file_hash()
    }
}

/// Finds the internal glyph index (usable for `*_indexed` calls) for the given
///character. If the character isn't in the font, returns 0.
#[no_mangle]
pub extern "C" fn ftd_font_lookup_glyph_index(font: Font, character: Char) -> u16 {
    unsafe {
        let ptr = font as *const fontdue::Font;
        return ptr
            .as_ref()
            .unwrap()
            .lookup_glyph_index(char::from_u32(character).unwrap());
    }
}

/// Returns the total number of glyphs in the font.
#[no_mangle]
pub extern "C" fn ftd_font_glyph_count(font: Font) -> u16 {
    unsafe {
        let ptr = font as *const fontdue::Font;
        return ptr.as_ref().unwrap().glyph_count();
    }
}

/// Populates `line_metrics` with data for fonts that append characters to
/// lines horizontally and new lines vertically.
///
/// Returns `false` if the metrics are missing from the given font, otherwise
/// returns `true`.
#[no_mangle]
pub extern "C" fn ftd_font_horizontal_line_metrics(
    font: Font,
    px: f32,
    line_metrics: *mut LineMetrics,
) -> bool {
    unsafe {
        let ptr = font as *const fontdue::Font;
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

/// Populates `line_metrics` with data for fonts that append characters to
/// lines vertically and new lines horizontally.
///
/// Returns `false` if the metrics are missing from the given font, otherwise
/// returns `true`.
#[no_mangle]
pub extern "C" fn ftd_font_vertical_line_metrics(
    font: Font,
    px: f32,
    line_metrics: *mut LineMetrics,
) -> bool {
    unsafe {
        let ptr = font as *const fontdue::Font;
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

/// Returns the font's units per em.
#[no_mangle]
pub extern "C" fn ftd_font_units_per_em(font: Font) -> f32 {
    unsafe {
        (font as *const fontdue::Font)
            .as_ref()
            .unwrap()
            .units_per_em()
    }
}

/// Returns the outline scale factor for a font size in pixels per em.
#[no_mangle]
pub extern "C" fn ftd_font_scale_factor(font: Font, px: f32) -> f32 {
    unsafe {
        (font as *const fontdue::Font)
            .as_ref()
            .unwrap()
            .scale_factor(px)
    }
}

/// Retrieves the horizontal scaled kerning value for two adjacent characters.
///
/// Returns `false` if there isn't a kerning value for the pair, otherwise
/// returns `true`.
#[no_mangle]
pub extern "C" fn ftd_font_horizontal_kern(
    font: Font,
    left: Char,
    right: Char,
    px: f32,
    kerning: *mut f32,
) -> bool {
    unsafe {
        let ptr = font as *const fontdue::Font;
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

/// Retrieves the horizontal scaled kerning value for two adjacent characters
/// specified by index.
///
/// Returns `false` if there isn't a kerning value for the pair, otherwise
/// returns `true`.
#[no_mangle]
pub extern "C" fn ftd_font_horizontal_kern_indexed(
    font: Font,
    left: u16,
    right: u16,
    px: f32,
    kerning: *mut f32,
) -> bool {
    unsafe {
        let ptr = font as *const fontdue::Font;
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

/// Populates `metrics` with the layout metrics for the given character. If the
/// character isn't present in the font, it populates `metrics` using the font's
/// default character.
#[no_mangle]
pub extern "C" fn ftd_font_metrics(font: Font, character: Char, px: f32, metrics: *mut Metrics) {
    unsafe {
        let ptr = font as *const fontdue::Font;
        let met = ptr
            .as_ref()
            .unwrap()
            .metrics(char::from_u32(character as u32).unwrap(), px);
        *metrics = met.into();
    }
}

/// Populates `metrics` with the layout metrics for the given character
/// specified by index.
#[no_mangle]
pub extern "C" fn ftd_font_metrics_indexed(font: Font, index: u16, px: f32, metrics: *mut Metrics) {
    unsafe {
        let ptr = font as *const fontdue::Font;
        let met = ptr.as_ref().unwrap().metrics_indexed(index, px);
        *metrics = met.into();
    }
}

/// Populates `bitmap` with the rasterized bitmap for the given character. If
/// the config's character isn't in the font, it populates `bitmap` using the
/// font's default character.
///
/// WARNING: this function can't check the bounds of `bitmap.data`. You
/// can calculate necessary capacity using one of the metrics functions:
/// `bitmap.data` will need to be at least `width * height` capacity.
#[no_mangle]
pub extern "C" fn ftd_font_rasterize(
    font: Font,
    character: Char,
    px: f32,
    bitmap: *mut GlyphBitmap,
) {
    unsafe {
        let ptr = font as *const fontdue::Font;
        let (met, bits) = ptr
            .as_ref()
            .unwrap()
            .rasterize(char::from_u32(character).unwrap(), px);
        (*bitmap).metrics = met.into();
        ptr::copy(
            bits.as_ptr(),
            (*bitmap).data,
            bits.len() * mem::size_of::<u8>(),
        );
        (*bitmap).data_length = bits.len();
    }
}

/// Populates `bitmap` with the rasterized bitmap for the given character. If
/// the config's character isn't in the font, it populates `bitmap` using the
/// font's default character.
///
/// This variant performs rasterization with the width multiplied by 3 to
/// simulate subpixels. Taking these as RGB values will perform subpixel
/// antialiasing.
///
/// WARNING: this function can't check the bounds of `bitmap.data`. You
/// can calculate necessary capacity using one of the metrics functions:
/// `bitmap.data` will need to be at least `width * height` capacity.
#[no_mangle]
pub extern "C" fn ftd_font_rasterize_subpixel(
    font: Font,
    character: Char,
    px: f32,
    bitmap: *mut GlyphBitmap,
) {
    unsafe {
        let ptr = font as *const fontdue::Font;
        let (met, bits) = ptr
            .as_ref()
            .unwrap()
            .rasterize_subpixel(char::from_u32(character).unwrap(), px);
        (*bitmap).metrics = met.into();
        ptr::copy(
            bits.as_ptr(),
            (*bitmap).data,
            bits.len() * mem::size_of::<u8>(),
        );
        (*bitmap).data_length = bits.len();
    }
}

/// Populates `bitmap` with the rasterized bitmap for the given character
/// specified by index.
///
/// WARNING: this function can't check the bounds of `bitmap.data`. You
/// can calculate necessary capacity using one of the metrics functions:
/// `bitmap.data` will need to be at least `width * height` capacity.
#[no_mangle]
pub extern "C" fn ftd_font_rasterize_indexed(
    font: Font,
    index: u16,
    px: f32,
    bitmap: *mut GlyphBitmap,
) {
    unsafe {
        let ptr = font as *const fontdue::Font;
        let (met, bits) = ptr.as_ref().unwrap().rasterize_indexed(index, px);
        (*bitmap).metrics = met.into();
        ptr::copy(
            bits.as_ptr(),
            (*bitmap).data,
            bits.len() * mem::size_of::<u8>(),
        );
        (*bitmap).data_length = bits.len();
    }
}

/// Populates `bitmap` with the rasterized bitmap for the given character
/// specified by index.
///
/// This variant performs rasterization with the width multiplied by 3 to
/// simulate subpixels. Taking these as RGB values will perform subpixel
/// antialiasing.
///
/// WARNING: this function can't check the bounds of `bitmap.data`. You
/// can calculate necessary capacity using one of the metrics functions:
/// `bitmap.data` will need to be at least `width * height` capacity.
#[no_mangle]
pub extern "C" fn ftd_font_rasterize_indexed_subpixel(
    font: Font,
    index: u16,
    px: f32,
    bitmap: *mut GlyphBitmap,
) {
    unsafe {
        let ptr = font as *const fontdue::Font;
        let (met, bits) = ptr.as_ref().unwrap().rasterize_indexed_subpixel(index, px);
        (*bitmap).metrics = met.into();
        ptr::copy(
            bits.as_ptr(),
            (*bitmap).data,
            bits.len() * mem::size_of::<u8>(),
        );
        (*bitmap).data_length = bits.len();
    }
}

/// Populates `bitmap` with the rasterized bitmap for the given raster config.
/// If the config's character isn't in the font, it populates `bitmap` using the
/// font's default character.
///
/// WARNING: this function can't check the bounds of `bitmap.data`. You
/// can calculate necessary capacity using one of the metrics functions:
/// `bitmap.data` will need to be at least `width * height` capacity.
#[no_mangle]
pub extern "C" fn ftd_font_rasterize_config(
    font: Font,
    config: GlyphRasterConfig,
    bitmap: *mut GlyphBitmap,
) {
    unsafe {
        let ptr = font as *const fontdue::Font;
        let (met, bits) = ptr.as_ref().unwrap().rasterize_config(config.into());
        (*bitmap).metrics = met.into();
        ptr::copy(
            bits.as_ptr(),
            (*bitmap).data,
            bits.len() * mem::size_of::<u8>(),
        );
        (*bitmap).data_length = bits.len();
    }
}

/// Populates `bitmap` with the rasterized bitmap for the given raster config.
/// If the config's character isn't in the font, it populates `bitmap` using
/// the font's default character.
///
/// This variant performs rasterization with the width multiplied by 3 to
/// simulate subpixels. Taking these as RGB values will perform subpixel
/// antialiasing.
///
/// WARNING: this function can't check the bounds of `bitmap.data`. You
/// can calculate necessary capacity using one of the metrics functions:
/// `bitmap.data` will need to be at least `width * height` capacity.
#[no_mangle]
pub extern "C" fn ftd_font_rasterize_config_subpixel(
    font: Font,
    config: GlyphRasterConfig,
    bitmap: *mut GlyphBitmap,
) {
    unsafe {
        let ptr = font as *const fontdue::Font;
        let (met, bits) = ptr
            .as_ref()
            .unwrap()
            .rasterize_config_subpixel(config.into());
        (*bitmap).metrics = met.into();
        ptr::copy(
            bits.as_ptr(),
            (*bitmap).data,
            bits.len() * mem::size_of::<u8>(),
        );
        (*bitmap).data_length = bits.len();
    }
}
