#ifndef FONTDUE_H
#define FONTDUE_H

/* Generated with cbindgen:0.24.3 */

/* Automatically generated. Don't edit. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void *FTD_Font;

typedef struct FTD_LineMetrics {
  float ascent;
  float descent;
  float line_gap;
  float new_line_size;
} FTD_LineMetrics;

typedef uint32_t FTD_Char;

typedef struct FTD_OutlineBounds {
  float xmin;
  float ymin;
  float width;
  float height;
} FTD_OutlineBounds;

typedef struct FTD_Metrics {
  int32_t xmin;
  int32_t ymin;
  uintptr_t width;
  uintptr_t height;
  float advance_width;
  float advance_height;
  struct FTD_OutlineBounds bounds;
} FTD_Metrics;

typedef struct FTD_GlyphRasterConfig {
  uint16_t glyph_index;
  float px;
  uintptr_t font_hash;
} FTD_GlyphRasterConfig;

typedef struct FTD_RasterizedBitmap {
  struct FTD_Metrics metrics;
  uint8_t *data;
  uintptr_t data_length;
} FTD_RasterizedBitmap;

FTD_Font ftd_font_from_bytes(uint8_t *bytes, uintptr_t size);

void ftd_font_drop(FTD_Font font);

uintptr_t ftd_font_file_hash(FTD_Font font);

bool ftd_font_horizontal_line_metrics(FTD_Font font,
                                      float px,
                                      struct FTD_LineMetrics *line_metrics);

bool ftd_font_vertical_line_metrics(FTD_Font font, float px, struct FTD_LineMetrics *line_metrics);

float ftd_font_units_per_em(FTD_Font font);

float ftd_font_scale_factor(FTD_Font font, float px);

bool ftd_font_horizontal_kern(FTD_Font font,
                              FTD_Char left,
                              FTD_Char right,
                              float px,
                              float *kerning);

bool ftd_font_horizontal_kern_indexed(FTD_Font font,
                                      uint16_t left,
                                      uint16_t right,
                                      float px,
                                      float *kerning);

void ftd_font_metrics(FTD_Font font, FTD_Char character, float px, struct FTD_Metrics *metrics);

void ftd_font_metrics_indexed(FTD_Font font, uint16_t index, float px, struct FTD_Metrics *metrics);

void ftd_font_rasterize_config(FTD_Font font,
                               struct FTD_GlyphRasterConfig config,
                               struct FTD_RasterizedBitmap *bitmap);

void ftd_font_rasterize(FTD_Font font,
                        FTD_Char character,
                        float px,
                        struct FTD_RasterizedBitmap *bitmap);

void ftd_font_rasterize_config_subpixel(FTD_Font font,
                                        struct FTD_GlyphRasterConfig config,
                                        struct FTD_RasterizedBitmap *bitmap);

void ftd_font_rasterize_subpixel(FTD_Font font,
                                 FTD_Char character,
                                 float px,
                                 struct FTD_RasterizedBitmap *bitmap);

void ftd_font_rasterize_indexed(FTD_Font font,
                                uint16_t index,
                                float px,
                                struct FTD_RasterizedBitmap *bitmap);

void ftd_font_rasterize_indexed_subpixel(FTD_Font font,
                                         uint16_t index,
                                         float px,
                                         struct FTD_RasterizedBitmap *bitmap);

uint16_t ftd_font_lookup_glyph_index(FTD_Font font, FTD_Char character);

uint16_t ftd_font_glyph_count(FTD_Font font);

#endif /* FONTDUE_H */
