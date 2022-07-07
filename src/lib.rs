use cty::*;

mod font;
mod layout;

pub use crate::font::*;
pub use crate::layout::*;

/// Opaque pointer to a font.
type Font = *mut cty::c_void;

/// 32-bit character type.
type Char = u32;

/// Metrics associated with line positioning.
#[repr(C)]
pub struct LineMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
    pub new_line_size: f32,
}

/// Encapsulates all layout information associated with a glyph for a fixed
/// scale.
///
/// You can use a Metrics to calculate the necessary buffer size for
/// rasterization, as a rasterized letter will be `width * height` pixels long,
/// or for subpixel rasterization will be `width * 3 * height` pixels long.
#[repr(C)]
pub struct Metrics {
    pub xmin: i32,
    pub ymin: i32,
    pub width: size_t,
    pub height: size_t,
    pub advance_width: f32,
    pub advance_height: f32,
    pub bounds: OutlineBounds,
}

/// Defines the bounds for a glyph's outline in subpixels.
#[repr(C)]
pub struct OutlineBounds {
    pub xmin: f32,
    pub ymin: f32,
    pub width: f32,
    pub height: f32,
}

/// Contains a rasterized glyph. Each byte in `data` is a 0-255 SDF.
#[repr(C)]
pub struct GlyphBitmap {
    pub metrics: Metrics,
    pub data: *mut u8,
    pub data_length: size_t,
}

/// A mapping between character and index, usable for the `*_indexed` functions.
#[repr(C)]
pub struct GlyphMapping {
    pub character: Char,
    pub index: u16,
}

/// Rasterization config usable in `rasterize_config*` functions.
#[repr(C)]
pub struct GlyphRasterConfig {
    pub glyph_index: u16,
    pub px: f32,
    pub font_hash: usize,
}

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
