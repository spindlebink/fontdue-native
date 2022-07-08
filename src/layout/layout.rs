use crate::*;

/// Opaque pointer to layout context. Reuse between layout calls to conserve
/// allocations and improve performance.
pub type Layout = *mut cty::c_void;

/// Creates a new layout instance.
#[no_mangle]
pub extern "C" fn ftd_layout_new(coordinate_system: CoordinateSystem) -> Layout {
    let l = fontdue::layout::Layout::<GlyphUserData>::new(coordinate_system.into());
    return Box::<fontdue::layout::Layout<GlyphUserData>>::into_raw(Box::new(l)) as Layout;
}

/// Frees a layout previous allocated with `ftd_layout_new`.
#[no_mangle]
pub extern "C" fn ftd_layout_free(layout: Layout) {
    unsafe {
        drop(Box::from_raw(layout as *mut fontdue::layout::Layout));
    }
}

/// Resets the current layout settings and clears all appended text.
#[no_mangle]
pub extern "C" fn ftd_layout_reset(layout: Layout, settings: LayoutSettings) {
    unsafe {
        (layout as *mut fontdue::layout::Layout)
            .as_mut()
            .unwrap()
            .reset(&settings.into())
    }
}

/// Keeps current layout settings but clears all appended text.
#[no_mangle]
pub extern "C" fn ftd_layout_clear(layout: Layout) {
    unsafe {
        (layout as *mut fontdue::layout::Layout)
            .as_mut()
            .unwrap()
            .clear()
    }
}

/// Gets the current height of the appended text.
#[no_mangle]
pub extern "C" fn ftd_layout_height(layout: Layout) -> f32 {
    unsafe {
        (layout as *const fontdue::layout::Layout)
            .as_ref()
            .unwrap()
            .height()
    }
}

/// Gets the current number of positioned lines.
#[no_mangle]
pub extern "C" fn ftd_layout_lines_count(layout: Layout) -> size_t {
    match unsafe {
        (layout as *const fontdue::layout::Layout)
            .as_ref()
            .unwrap()
            .lines()
    } {
        Some(lines) => lines.len(),
        None => 0,
    }
}

/// Gets the currently positioned lines.
///
/// Returns `false` if there are currently no lines. Returns `true` otherwise.
#[no_mangle]
pub extern "C" fn ftd_layout_lines(layout: Layout, lines: *mut LinePosition) -> bool {
    match unsafe {
        (layout as *const fontdue::layout::Layout)
            .as_ref()
            .unwrap()
            .lines()
    } {
        Some(ftd_lines) => {
            unsafe {
                let mut cur_offset = 0;
                for &line_pos in ftd_lines {
                    *(lines.offset(cur_offset)) = line_pos.into();
                    cur_offset += 1;
                }
            }
            true
        }
        None => false,
    }
}

/// Performs layout for text horizontally and wrapping vertically.
#[no_mangle]
pub extern "C" fn ftd_layout_append(
    layout: Layout,
    fonts: *mut Font,
    font_count: size_t,
    style: TextStyle,
) {
    unsafe {
        let lt = (layout as *mut fontdue::layout::Layout<GlyphUserData>)
            .as_mut()
            .unwrap();
        let fonts = core::slice::from_raw_parts_mut::<*mut fontdue::Font>(
            fonts as *mut *mut fontdue::Font,
            font_count,
        )
        .into_iter()
        .map(|it| &**it)
        .collect::<Vec<&fontdue::Font>>();
        lt.append(&fonts.as_slice(), &style.into());
    };
}

/// Gets the currently laid out glyphs.
#[no_mangle]
pub extern "C" fn ftd_layout_glyphs(layout: Layout, glyphs: *mut GlyphPosition) {
    unsafe {
        let ftd_glyphs = (layout as *mut fontdue::layout::Layout<GlyphUserData>)
            .as_ref()
            .unwrap()
            .glyphs();
        let mut cur_offset = 0;
        for &glyph in ftd_glyphs {
            *(glyphs.offset(cur_offset)) = glyph.into();
            cur_offset += 1;
        }
    }
}

/// Gets the number of laid out glyphs.
#[no_mangle]
pub extern "C" fn ftd_layout_glyphs_count(layout: Layout) -> size_t {
    unsafe {
        (layout as *const fontdue::layout::Layout)
            .as_ref()
            .unwrap()
            .glyphs()
            .len()
    }
}
