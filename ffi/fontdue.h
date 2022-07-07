#ifndef FONTDUE_H
#define FONTDUE_H

/* Generated with cbindgen:0.24.3 */

/* Automatically generated. Don't edit. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void *FTD_Font;

typedef uint32_t FTD_Char;

/**
 * A mapping between character and index, usable for the `*_indexed` functions.
 */
typedef struct FTD_GlyphMapping {
  FTD_Char character;
  uint16_t index;
} FTD_GlyphMapping;

/**
 * Metrics associated with line positioning.
 */
typedef struct FTD_LineMetrics {
  float ascent;
  float descent;
  float line_gap;
  float new_line_size;
} FTD_LineMetrics;

/**
 * Defines the bounds for a glyph's outline in subpixels.
 */
typedef struct FTD_OutlineBounds {
  float xmin;
  float ymin;
  float width;
  float height;
} FTD_OutlineBounds;

/**
 * Encapsulates all layout information associated with a glyph for a fixed
 * scale.
 *
 * You can use a Metrics to calculate the necessary buffer size for
 * rasterization, as a rasterized letter will be `width * height` pixels long,
 * or for subpixel rasterization will be `width * 3 * height` pixels long.
 */
typedef struct FTD_Metrics {
  int32_t xmin;
  int32_t ymin;
  uintptr_t width;
  uintptr_t height;
  float advance_width;
  float advance_height;
  struct FTD_OutlineBounds bounds;
} FTD_Metrics;

/**
 * Contains a rasterized glyph. Each byte in `data` is a 0-255 SDF.
 */
typedef struct FTD_GlyphBitmap {
  struct FTD_Metrics metrics;
  uint8_t *data;
  uintptr_t data_length;
} FTD_GlyphBitmap;

/**
 * Rasterization config usable in `rasterize_config*` functions.
 */
typedef struct FTD_GlyphRasterConfig {
  uint16_t glyph_index;
  float px;
  uintptr_t font_hash;
} FTD_GlyphRasterConfig;

/**
 * Allocates a font from an array of bytes.
 */
FTD_Font ftd_font_create_from_bytes(uint8_t *bytes, uintptr_t size);

/**
 * Frees a font previously allocated with `ftd_font_from_bytes`.
 */
void ftd_font_free(FTD_Font font);

/**
 * Retrieves char -> index mappings from the font, populating `chars` with the
 * glyph pairs.
 *
 * To determine the necessary size for `chars`, you'll need to use
 * `ftd_font_char_count`. This function can't do any bounds checking.
 */
void ftd_font_chars(FTD_Font font, struct FTD_GlyphMapping *chars);

/**
 * Returns the number of available unicode characters in the font. The result
 * will be the minimum buffer size required for a call to `ftd_font_chars`.
 */
uintptr_t ftd_font_char_count(FTD_Font font);

/**
 * Returns a precomputed hash for the font file.
 */
uintptr_t ftd_font_file_hash(FTD_Font font);

/**
 * Finds the internal glyph index (usable for `*_indexed` calls) for the given
 *character. If the character isn't in the font, returns 0.
 */
uint16_t ftd_font_lookup_glyph_index(FTD_Font font, FTD_Char character);

/**
 * Returns the total number of glyphs in the font.
 */
uint16_t ftd_font_glyph_count(FTD_Font font);

/**
 * Populates `line_metrics` with data for fonts that append characters to
 * lines horizontally and new lines vertically.
 *
 * Returns `false` if the metrics are missing from the given font, otherwise
 * returns `true`.
 */
bool ftd_font_horizontal_line_metrics(FTD_Font font,
                                      float px,
                                      struct FTD_LineMetrics *line_metrics);

/**
 * Populates `line_metrics` with data for fonts that append characters to
 * lines vertically and new lines horizontally.
 *
 * Returns `false` if the metrics are missing from the given font, otherwise
 * returns `true`.
 */
bool ftd_font_vertical_line_metrics(FTD_Font font, float px, struct FTD_LineMetrics *line_metrics);

/**
 * Returns the font's units per em.
 */
float ftd_font_units_per_em(FTD_Font font);

/**
 * Returns the outline scale factor for a font size in pixels per em.
 */
float ftd_font_scale_factor(FTD_Font font, float px);

/**
 * Retrieves the horizontal scaled kerning value for two adjacent characters.
 *
 * Returns `false` if there isn't a kerning value for the pair, otherwise
 * returns `true`.
 */
bool ftd_font_horizontal_kern(FTD_Font font,
                              FTD_Char left,
                              FTD_Char right,
                              float px,
                              float *kerning);

/**
 * Retrieves the horizontal scaled kerning value for two adjacent characters
 * specified by index.
 *
 * Returns `false` if there isn't a kerning value for the pair, otherwise
 * returns `true`.
 */
bool ftd_font_horizontal_kern_indexed(FTD_Font font,
                                      uint16_t left,
                                      uint16_t right,
                                      float px,
                                      float *kerning);

/**
 * Populates `metrics` with the layout metrics for the given character. If the
 * character isn't present in the font, it populates `metrics` using the font's
 * default character.
 */
void ftd_font_metrics(FTD_Font font, FTD_Char character, float px, struct FTD_Metrics *metrics);

/**
 * Populates `metrics` with the layout metrics for the given character
 * specified by index.
 */
void ftd_font_metrics_indexed(FTD_Font font, uint16_t index, float px, struct FTD_Metrics *metrics);

/**
 * Populates `bitmap` with the rasterized bitmap for the given character. If
 * the config's character isn't in the font, it populates `bitmap` using the
 * font's default character.
 *
 * WARNING: this function can't check the bounds of `bitmap.data`. You
 * can calculate necessary capacity using one of the metrics functions:
 * `bitmap.data` will need to be at least `width * height` capacity.
 */
void ftd_font_rasterize(FTD_Font font,
                        FTD_Char character,
                        float px,
                        struct FTD_GlyphBitmap *bitmap);

/**
 * Populates `bitmap` with the rasterized bitmap for the given character. If
 * the config's character isn't in the font, it populates `bitmap` using the
 * font's default character.
 *
 * This variant performs rasterization with the width multiplied by 3 to
 * simulate subpixels. Taking these as RGB values will perform subpixel
 * antialiasing.
 *
 * WARNING: this function can't check the bounds of `bitmap.data`. You
 * can calculate necessary capacity using one of the metrics functions:
 * `bitmap.data` will need to be at least `width * height` capacity.
 */
void ftd_font_rasterize_subpixel(FTD_Font font,
                                 FTD_Char character,
                                 float px,
                                 struct FTD_GlyphBitmap *bitmap);

/**
 * Populates `bitmap` with the rasterized bitmap for the given character
 * specified by index.
 *
 * WARNING: this function can't check the bounds of `bitmap.data`. You
 * can calculate necessary capacity using one of the metrics functions:
 * `bitmap.data` will need to be at least `width * height` capacity.
 */
void ftd_font_rasterize_indexed(FTD_Font font,
                                uint16_t index,
                                float px,
                                struct FTD_GlyphBitmap *bitmap);

/**
 * Populates `bitmap` with the rasterized bitmap for the given character
 * specified by index.
 *
 * This variant performs rasterization with the width multiplied by 3 to
 * simulate subpixels. Taking these as RGB values will perform subpixel
 * antialiasing.
 *
 * WARNING: this function can't check the bounds of `bitmap.data`. You
 * can calculate necessary capacity using one of the metrics functions:
 * `bitmap.data` will need to be at least `width * height` capacity.
 */
void ftd_font_rasterize_indexed_subpixel(FTD_Font font,
                                         uint16_t index,
                                         float px,
                                         struct FTD_GlyphBitmap *bitmap);

/**
 * Populates `bitmap` with the rasterized bitmap for the given raster config.
 * If the config's character isn't in the font, it populates `bitmap` using the
 * font's default character.
 *
 * WARNING: this function can't check the bounds of `bitmap.data`. You
 * can calculate necessary capacity using one of the metrics functions:
 * `bitmap.data` will need to be at least `width * height` capacity.
 */
void ftd_font_rasterize_config(FTD_Font font,
                               struct FTD_GlyphRasterConfig config,
                               struct FTD_GlyphBitmap *bitmap);

/**
 * Populates `bitmap` with the rasterized bitmap for the given raster config.
 * If the config's character isn't in the font, it populates `bitmap` using
 * the font's default character.
 *
 * This variant performs rasterization with the width multiplied by 3 to
 * simulate subpixels. Taking these as RGB values will perform subpixel
 * antialiasing.
 *
 * WARNING: this function can't check the bounds of `bitmap.data`. You
 * can calculate necessary capacity using one of the metrics functions:
 * `bitmap.data` will need to be at least `width * height` capacity.
 */
void ftd_font_rasterize_config_subpixel(FTD_Font font,
                                        struct FTD_GlyphRasterConfig config,
                                        struct FTD_GlyphBitmap *bitmap);

#endif /* FONTDUE_H */
