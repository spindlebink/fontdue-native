use core::mem;
use cty::*;

mod font;
mod layout;
mod metrics;

pub use crate::font::*;
pub use crate::layout::*;
pub use crate::metrics::*;

/// 32-bit character type.
type Char = u32;

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
