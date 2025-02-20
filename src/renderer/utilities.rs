use super::glyphs::{special, GlyphRef, GLYPH_SOUND_MAP, STOP_SOUNDS};
use crate::{lexer::collections::WordKind, renderer::glyphs::numbers::numeric_phonoglyph};

pub fn insert_into_bitmap(dst: &mut [Vec<u8>], src: &[Vec<u8>], pos: (u16, u16)) {
    // Sanity check - ensure the source fits in the destination at the given position
    let (src_w, src_h) = (src[0].len() as u16, src.len() as u16);
    let (dst_w, dst_h) = (dst[0].len() as u16, dst.len() as u16);

    if pos.0 + src_w > dst_w || pos.1 + src_h > dst_h {
        panic!(
            "Image of size {:?} at position {:?} does not fit in destination of size {:?}",
            (src_w, src_h),
            pos,
            (dst_w, dst_h)
        );
    }

    for (y, row) in src.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            let x = pos.0 as usize + x;
            let y = pos.1 as usize + y;

            dst[y][x] = *pixel;
        }
    }
}

/// Returns the IPA string for the given word
pub fn ipa_string_for(token: &WordKind) -> String {
    let glyphs = glyphs_for(token);
    let syllables = glyphs
        .iter()
        .map(|s| s.iter().map(|g| g.as_ref().ipa_symbol).collect::<String>());
    syllables
        .filter(|g| !g.is_empty())
        .collect::<Vec<_>>()
        .join("")
}

/// Returns the set of glyphs rendering to the given words
///
/// Each word is composed of a set of phoneme groups
pub fn glyphs_for(token: &WordKind) -> Vec<Vec<GlyphRef>> {
    match token {
        WordKind::Number(n) => {
            let glyph = numeric_phonoglyph(*n);
            vec![vec![GlyphRef::Owned(glyph)]]
        }
        WordKind::PhonemeGroup(groups) => {
            let mut stacks = vec![];
            for group in groups {
                let mut phonemes = group.as_str();
                let mut glyphs = vec![];

                while !phonemes.is_empty() {
                    if phonemes.starts_with("'") {
                        phonemes = &phonemes[1..];

                        // End the current vowel stack
                        if !glyphs.is_empty() {
                            stacks.push(glyphs);
                            glyphs = vec![];
                        }
                        continue;
                    }

                    let mut has_match = false;
                    for (sound, glyph) in GLYPH_SOUND_MAP {
                        if phonemes.starts_with(sound) {
                            if STOP_SOUNDS.contains(sound) {
                                // End the current vowel stack
                                if !glyphs.is_empty() {
                                    stacks.push(glyphs);
                                    glyphs = vec![];
                                }
                                stacks.push(vec![GlyphRef::Borrowed(glyph)]);
                            } else {
                                glyphs.push(GlyphRef::Borrowed(glyph));
                            }

                            phonemes = &phonemes[sound.len()..];
                            has_match = true;
                            break;
                        }
                    }

                    if !has_match {
                        // If we reach this point, we have an unrecognized phoneme
                        glyphs.push(GlyphRef::Borrowed(&special::UNKNOWN));
                        phonemes = &phonemes[1..];
                    }
                }

                if !glyphs.is_empty() {
                    stacks.push(glyphs);
                }
            }

            stacks
        }
    }
}
