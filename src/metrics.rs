use crate::*;

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

/// Defines the bounds for a glyph's outline in subpixels.
#[repr(C)]
pub struct OutlineBounds {
    pub xmin: f32,
    pub ymin: f32,
    pub width: f32,
    pub height: f32,
}
