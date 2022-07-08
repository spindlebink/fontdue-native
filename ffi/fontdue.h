#ifndef FONTDUE_H
#define FONTDUE_H

/* Generated with cbindgen:0.24.3 */

/* Automatically generated. Don't edit. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * The direction that the Y coordinate increases in. Layout needs to be aware
 * of your coordinate system to place the glyphs correctly.
 */
typedef enum FTD_CoordinateSystem {
  PositiveYUp,
  PositiveYDown,
} FTD_CoordinateSystem;

/**
 * Horizontal alignment options for text when a `max_width` is provided.
 */
typedef enum FTD_HorizontalAlign {
  Left,
  Center,
  Right,
} FTD_HorizontalAlign;

/**
 * Vertical alignment options for text when a `max_height` is provided.
 */
typedef enum FTD_VerticalAlign {
  Top,
  Middle,
  Bottom,
} FTD_VerticalAlign;

/**
 * Wrap style is a hint for how strings of text should be wrapped to the next
 * line. Line wrapping can happen when the max width/height is reached.
 */
typedef enum FTD_WrapStyle {
  Word,
  Letter,
} FTD_WrapStyle;

/**
 * Opaque pointer to a font.
 */
typedef void *FTD_Font;

typedef struct FTD_FontSettings {
  uint32_t collection_index;
  float scale;
} FTD_FontSettings;

/**
 * 32-bit character type.
 */
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
  size_t width;
  size_t height;
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
  size_t data_length;
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
 * Metadata for character layout, backing type.
 */
typedef uint8_t FTD_OpaqueCharacterData[1];

/**
 * Metadata for layout system.
 */
typedef struct FTD_CharacterData {
  FTD_OpaqueCharacterData _cd;
} FTD_CharacterData;

/**
 * Opaque pointer to layout context. Reuse between layout calls to conserve
 * allocations and improve performance.
 */
typedef void *FTD_Layout;

/**
 * Settings to configure how text layout is constrainted.
 */
typedef struct FTD_LayoutSettings {
  float x;
  float y;
  bool constrain_width;
  float max_width;
  bool constrain_height;
  float max_height;
  enum FTD_HorizontalAlign horizontal_align;
  enum FTD_VerticalAlign vertical_align;
  enum FTD_WrapStyle wrap_style;
  bool wrap_hard_breaks;
} FTD_LayoutSettings;

/**
 * Metrics about a positioned line.
 */
typedef struct FTD_LinePosition {
  float baseline_y;
  float padding;
  float max_ascent;
  float min_descent;
  float max_line_gap;
  float max_new_line_size;
  size_t glyph_start;
  size_t glyph_end;
} FTD_LinePosition;

/**
 * Pointer to arbitrary userdata in a glyph.
 */
typedef void *FTD_GlyphUserData;

/**
 * A style description for a segment of text.
 */
typedef struct FTD_TextStyle {
  const char *text;
  float px;
  uintptr_t font_index;
  FTD_GlyphUserData user_data;
} FTD_TextStyle;

/**
 * A positioned, scaled glyph.
 */
typedef struct FTD_GlyphPosition {
  struct FTD_GlyphRasterConfig key;
  uintptr_t font_index;
  FTD_Char parent;
  float x;
  float y;
  size_t width;
  size_t height;
  size_t byte_offset;
  struct FTD_CharacterData char_data;
  FTD_GlyphUserData user_data;
} FTD_GlyphPosition;

/**
 * Allocates a font from an array of bytes.
 */
FTD_Font ftd_font_new_from_bytes(uint8_t *bytes, size_t size, struct FTD_FontSettings settings);

/**
 * Frees a font previously allocated with `ftd_font_new_from_bytes`.
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
size_t ftd_font_char_count(FTD_Font font);

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

/**
 * Retrieves character data given a character and its index in a font.
 */
void ftd_char_data_classify(FTD_Char character, uint16_t index, struct FTD_CharacterData *data);

/**
 * Heuristic for if the glyph a character data was classified from should be
 * rasterized. Missing glyphs, whitespace, and control characters will return
 * `false`.
 */
bool ftd_char_data_rasterize(struct FTD_CharacterData char_data);

/**
 * Marks if the character is an ASCII whitespace character.
 */
bool ftd_char_data_is_whitespace(struct FTD_CharacterData char_data);

/**
 * Marks if the character is an ASCII control character.
 */
bool ftd_char_data_is_control(struct FTD_CharacterData char_data);

/**
 * Marks if the character is missing from its associated font.
 */
bool ftd_char_data_is_missing(struct FTD_CharacterData char_data);

/**
 * Creates a new layout instance.
 */
FTD_Layout ftd_layout_new(enum FTD_CoordinateSystem coordinate_system);

/**
 * Frees a layout previous allocated with `ftd_layout_new`.
 */
void ftd_layout_free(FTD_Layout layout);

/**
 * Resets the current layout settings and clears all appended text.
 */
void ftd_layout_reset(FTD_Layout layout, struct FTD_LayoutSettings settings);

/**
 * Keeps current layout settings but clears all appended text.
 */
void ftd_layout_clear(FTD_Layout layout);

/**
 * Gets the current height of the appended text.
 */
float ftd_layout_height(FTD_Layout layout);

/**
 * Gets the current number of positioned lines.
 */
size_t ftd_layout_lines_count(FTD_Layout layout);

/**
 * Gets the currently positioned lines.
 *
 * Returns `false` if there are currently no lines. Returns `true` otherwise.
 */
bool ftd_layout_lines(FTD_Layout layout, struct FTD_LinePosition *lines);

/**
 * Performs layout for text horizontally and wrapping vertically.
 */
void ftd_layout_append(FTD_Layout layout,
                       FTD_Font *fonts,
                       size_t font_count,
                       struct FTD_TextStyle style);

/**
 * Gets the currently laid out glyphs.
 */
void ftd_layout_glyphs(FTD_Layout layout, struct FTD_GlyphPosition *glyphs);

/**
 * Gets the number of laid out glyphs.
 */
size_t ftd_layout_glyphs_count(FTD_Layout layout);

#endif /* FONTDUE_H */
