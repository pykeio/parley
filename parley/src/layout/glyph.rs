// Copyright 2025 the Parley Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GlyphClass {
    #[default]
    Unclassified,
    Cjk,
    Emoji,
}

impl GlyphClass {
    pub(crate) fn from_char(char: char) -> Self {
        if matches!(char as u32, 0x1F600..=0x1F64F | 0x1F300..=0x1F5FF | 0x1F680..=0x1F6FF | 0x2600..=0x26FF | 0x2700..=0x27BF)
        {
            GlyphClass::Emoji
        } else if matches!(char as u32, 0x4E00..=0x9FFF | 0x3400..=0x4DBF | 0x2B740..=0x2B81F) {
            GlyphClass::Cjk
        } else {
            GlyphClass::Unclassified
        }
    }
}

/// Glyph with an offset and advance.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Glyph {
    pub id: u32,
    pub style_index: u16,
    pub class: GlyphClass,
    pub x: f32,
    pub y: f32,
    pub advance: f32,
}

impl Glyph {
    /// Returns the index into the layout style collection.
    pub fn style_index(&self) -> usize {
        self.style_index as usize
    }
}
