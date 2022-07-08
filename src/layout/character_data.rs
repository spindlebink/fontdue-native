use crate::*;

/// Metadata for character layout, backing type.
pub type OpaqueCharacterData = [u8; 1]; // Have to use hardcoded 1 because
                                        // cbindgen can't evaluate functions

// Version that ensures correct size (TODO: is there a way we can use this
// instead?):
// type OpaqueCharacterData = [u8; mem::size_of::<fontdue::layout::CharacterData>()];

/// Metadata for layout system.
#[repr(C)]
pub struct CharacterData {
    pub _cd: OpaqueCharacterData,
}

// Conversion between fontdue character data and interface character data has to
// be done through direct byte transmutation, since the only field currently in
// fontdue's character data is private.

impl From<fontdue::layout::CharacterData> for CharacterData {
    fn from(cd: fontdue::layout::CharacterData) -> Self {
        Self {
            _cd: unsafe {
                mem::transmute::<fontdue::layout::CharacterData, OpaqueCharacterData>(cd)
            },
        }
    }
}

impl From<CharacterData> for fontdue::layout::CharacterData {
    fn from(cd: CharacterData) -> Self {
        unsafe { mem::transmute::<OpaqueCharacterData, fontdue::layout::CharacterData>(cd._cd) }
    }
}

/// Retrieves character data given a character and its index in a font.
#[no_mangle]
pub extern "C" fn ftd_char_data_classify(character: Char, index: u16, data: *mut CharacterData) {
    let cd = fontdue::layout::CharacterData::classify(char::from_u32(character).unwrap(), index);
    unsafe {
        *data = cd.into();
    }
}

/// Heuristic for if the glyph a character data was classified from should be
/// rasterized. Missing glyphs, whitespace, and control characters will return
/// `false`.
#[no_mangle]
pub extern "C" fn ftd_char_data_rasterize(char_data: CharacterData) -> bool {
    fontdue::layout::CharacterData::from(char_data).rasterize()
}

/// Marks if the character is an ASCII whitespace character.
#[no_mangle]
pub extern "C" fn ftd_char_data_is_whitespace(char_data: CharacterData) -> bool {
    fontdue::layout::CharacterData::from(char_data).is_whitespace()
}

/// Marks if the character is an ASCII control character.
#[no_mangle]
pub extern "C" fn ftd_char_data_is_control(char_data: CharacterData) -> bool {
    fontdue::layout::CharacterData::from(char_data).is_control()
}

/// Marks if the character is missing from its associated font.
#[no_mangle]
pub extern "C" fn ftd_char_data_is_missing(char_data: CharacterData) -> bool {
    fontdue::layout::CharacterData::from(char_data).is_missing()
}
