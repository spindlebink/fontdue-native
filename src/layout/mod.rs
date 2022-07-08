use crate::*;
use libc::strlen;
use std::{slice, str};

pub mod character_data;
pub mod layout;

pub use character_data::*;
pub use layout::*;

/// Pointer to arbitrary userdata in a glyph.
pub type GlyphUserData = *mut cty::c_void;

/// A positioned, scaled glyph.
#[repr(C)]
pub struct GlyphPosition {
    pub key: GlyphRasterConfig,
    pub font_index: usize,
    pub parent: Char,
    pub x: f32,
    pub y: f32,
    pub width: size_t,
    pub height: size_t,
    pub byte_offset: size_t,
    pub char_data: CharacterData,
    pub user_data: GlyphUserData,
}

impl From<fontdue::layout::GlyphPosition<GlyphUserData>> for GlyphPosition {
    fn from(pos: fontdue::layout::GlyphPosition<GlyphUserData>) -> Self {
        Self {
            key: pos.key.into(),
            font_index: pos.font_index,
            parent: pos.parent.into(),
            x: pos.x,
            y: pos.y,
            width: pos.width,
            height: pos.height,
            byte_offset: pos.byte_offset,
            char_data: pos.char_data.into(),
            user_data: 0 as GlyphUserData,
        }
    }
}

/// Rasterization config usable in `rasterize_config*` functions.
#[repr(C)]
pub struct GlyphRasterConfig {
    pub glyph_index: u16,
    pub px: f32,
    pub font_hash: usize,
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

impl From<fontdue::layout::GlyphRasterConfig> for GlyphRasterConfig {
    fn from(config: fontdue::layout::GlyphRasterConfig) -> Self {
        Self {
            glyph_index: config.glyph_index,
            px: config.px,
            font_hash: config.font_hash,
        }
    }
}

/// Settings to configure how text layout is constrainted.
#[repr(C)]
pub struct LayoutSettings {
    pub x: f32,
    pub y: f32,
    pub constrain_width: bool,
    pub max_width: f32,
    pub constrain_height: bool,
    pub max_height: f32,
    pub horizontal_align: HorizontalAlign,
    pub vertical_align: VerticalAlign,
    pub wrap_style: WrapStyle,
    pub wrap_hard_breaks: bool,
}

impl From<LayoutSettings> for fontdue::layout::LayoutSettings {
    fn from(settings: LayoutSettings) -> Self {
        Self {
            x: settings.x,
            y: settings.y,
            max_width: if settings.constrain_width {
                Some(settings.max_width)
            } else {
                None
            },
            max_height: if settings.constrain_height {
                Some(settings.max_height)
            } else {
                None
            },
            horizontal_align: settings.horizontal_align.into(),
            vertical_align: settings.vertical_align.into(),
            wrap_style: settings.wrap_style.into(),
            wrap_hard_breaks: settings.wrap_hard_breaks,
        }
    }
}

/// Metrics about a positioned line.
#[repr(C)]
pub struct LinePosition {
    pub baseline_y: f32,
    pub padding: f32,
    pub max_ascent: f32,
    pub min_descent: f32,
    pub max_line_gap: f32,
    pub max_new_line_size: f32,
    pub glyph_start: size_t,
    pub glyph_end: size_t,
}

impl From<fontdue::layout::LinePosition> for LinePosition {
    fn from(line_pos: fontdue::layout::LinePosition) -> Self {
        Self {
            baseline_y: line_pos.baseline_y,
            padding: line_pos.padding,
            max_ascent: line_pos.max_ascent,
            min_descent: line_pos.min_descent,
            max_line_gap: line_pos.max_line_gap,
            max_new_line_size: line_pos.max_new_line_size,
            glyph_start: line_pos.glyph_start,
            glyph_end: line_pos.glyph_end,
        }
    }
}

/// A style description for a segment of text.
#[repr(C)]
pub struct TextStyle {
    pub text: *const c_char,
    pub px: f32,
    pub font_index: usize,
    pub user_data: GlyphUserData,
}

impl From<TextStyle> for fontdue::layout::TextStyle<'_, GlyphUserData> {
    fn from(style: TextStyle) -> Self {
        Self {
            text: unsafe {
                str::from_utf8_unchecked(slice::from_raw_parts(
                    style.text as *const u8,
                    strlen(style.text) + 1,
                ))
            },
            px: style.px,
            font_index: style.font_index,
            user_data: style.user_data,
        }
    }
}

/// The direction that the Y coordinate increases in. Layout needs to be aware
/// of your coordinate system to place the glyphs correctly.
#[repr(C)]
pub enum CoordinateSystem {
    PositiveYUp,
    PositiveYDown,
}

impl From<CoordinateSystem> for fontdue::layout::CoordinateSystem {
    fn from(coord: CoordinateSystem) -> Self {
        match coord {
            CoordinateSystem::PositiveYUp => fontdue::layout::CoordinateSystem::PositiveYUp,
            CoordinateSystem::PositiveYDown => fontdue::layout::CoordinateSystem::PositiveYDown,
        }
    }
}

/// Horizontal alignment options for text when a `max_width` is provided.
#[repr(C)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

impl From<HorizontalAlign> for fontdue::layout::HorizontalAlign {
    fn from(align: HorizontalAlign) -> Self {
        match align {
            HorizontalAlign::Left => fontdue::layout::HorizontalAlign::Left,
            HorizontalAlign::Center => fontdue::layout::HorizontalAlign::Center,
            HorizontalAlign::Right => fontdue::layout::HorizontalAlign::Right,
        }
    }
}

/// Vertical alignment options for text when a `max_height` is provided.
#[repr(C)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

impl From<VerticalAlign> for fontdue::layout::VerticalAlign {
    fn from(align: VerticalAlign) -> Self {
        match align {
            VerticalAlign::Top => fontdue::layout::VerticalAlign::Top,
            VerticalAlign::Middle => fontdue::layout::VerticalAlign::Middle,
            VerticalAlign::Bottom => fontdue::layout::VerticalAlign::Bottom,
        }
    }
}

/// Wrap style is a hint for how strings of text should be wrapped to the next
/// line. Line wrapping can happen when the max width/height is reached.
#[repr(C)]
pub enum WrapStyle {
    Word,
    Letter,
}

impl From<WrapStyle> for fontdue::layout::WrapStyle {
    fn from(style: WrapStyle) -> Self {
        match style {
            WrapStyle::Word => fontdue::layout::WrapStyle::Word,
            WrapStyle::Letter => fontdue::layout::WrapStyle::Letter,
        }
    }
}
