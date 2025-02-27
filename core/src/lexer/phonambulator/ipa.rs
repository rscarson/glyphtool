use crate::{glyphs::AsGlyphs, lexer::collections::WordKind};

/// Trait for converting english strings to IPA and phonemes
pub trait PhonemeExt {
    /// Convert the string to IPA representation
    fn to_ipa(&self) -> Option<String>;

    /// Convert the string to phoneme representation
    ///
    /// Goes through the IPA representation and converts it to a phoneme representation
    fn to_glyphs(&self) -> Option<String>;
}

impl PhonemeExt for WordKind {
    fn to_ipa(&self) -> Option<String> {
        let glyphs = self.as_glyphs();
        let phonemes: String = glyphs
            .into_iter()
            .flatten()
            .map(|g| g.pronounciation())
            .collect();
        phonemes.as_str().to_ipa()
    }

    fn to_glyphs(&self) -> Option<String> {
        let glyphs = self.as_glyphs();
        let phonemes: String = glyphs
            .into_iter()
            .flatten()
            .map(|g| g.pronounciation())
            .collect();
        phonemes.as_str().to_glyphs()
    }
}

impl PhonemeExt for &str {
    fn to_ipa(&self) -> Option<String> {
        if ["E", "O", "A"].contains(self) {
            return Some((*self).to_string());
        }
        espeak::text_to_phonemes(self).ok()
    }

    fn to_glyphs(&self) -> Option<String> {
        if ["E", "O", "A"].contains(self) {
            return Some((*self).to_string());
        }

        let phonemes = self.to_ipa()?;
        let mut phonemes = phonemes.as_str();
        let mut output = String::new();

        //
        // Remove leading h sound for now
        if let Some(s) = phonemes.strip_prefix("h") {
            output += "O'";
            phonemes = s;
        }

        for char in phonemes.chars() {
            let mut found = false;

            //
            // syllabic marker diacritic - add a t before the previous character
            if char == '̩' {
                let last = output.pop()?;
                output.push('t');
                output.push(last);
                continue;
            }

            for (chars, replacement) in PHONEME_REPLACEMENT_TABLE {
                if chars.contains(&char) {
                    output += replacement;
                    found = true;
                    break;
                }
            }

            if !found {
                eprintln!("No replacement found for phoneme: {char}");
                output.push('?');
            }
        }

        Some(output)
    }
}

const PHONEME_REPLACEMENT_TABLE: &[(&[char], &str)] = &[
    //
    // Vowels
    (&['ɔ', 'ɑ', 'ə', 'ɐ'], "ah"),
    (&['a', 'æ'], "a"),
    (&['e', 'ɛ', 'ᵻ'], "e"),
    (&['i', 'ɪ', 'j'], "i"),
    (&['o'], "o"),
    (&['ʊ', 'ʌ'], "uh"),
    (&['u', 'w'], "u"),
    //
    // Open consonants
    (&['ʃ', 'ʒ'], "sh"),
    (&['ð', 'θ'], "th"),
    (&['r', 'ɹ'], "r"),
    (&['l'], "l"),
    (&['ŋ'], "ng"),
    (&['n'], "n"),
    (&['s'], "s"),
    (&['t', 'ɾ'], "t"),
    (&['z'], "z"),
    (&['k', 'g', 'ɡ'], "k"),
    (&['d'], "d"),
    //
    // Closed consonants
    (&['m'], "m"),
    (&['f', 'v'], "f"),
    (&['b'], "b"),
    (&['p'], "p"),
    //
    // Special chars
    (&['h'], "O'"),
    //
    // Combinatory sounds
    (&['ɚ', 'ɜ'], "uhr"),
    (&['ʔ'], ""),
];
