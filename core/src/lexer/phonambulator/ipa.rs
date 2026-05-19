use crate::{glyphs::AsGlyphs, lexer::collections::WordKind};
use std::io::Write;

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

                // Append the character to a new line in bad_phonemes.txt
                if let Ok(mut file) = std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("bad_phonemes.txt")
                {
                    writeln!(file, "{char}").ok();
                }

                output.push('?');
            }
        }

        Some(output)
    }
}

/// Convert a string of IPA characters to a string of phonemes
#[must_use]
pub fn glyphs_to_ipa(input: &str) -> String {
    let mut input = input;
    let mut output = String::new();
    while !input.is_empty() {
        let mut found = false;
        for (chars, replacement) in PHONEME_REPLACEMENT_TABLE {
            if replacement.is_empty() {
                continue;
            }
            if let Some(rest) = input.strip_prefix(replacement) {
                output.push(chars[0]);
                input = rest;
                found = true;
                break;
            }
        }

        if input.starts_with('\'') {
            output.push('.');
            input = &input[1..];
            found = true;
        } else if input.starts_with(['?', '!', '.']) {
            output.push_str("||");
            input = &input[1..];
            found = true;
        } else if input.starts_with(' ') {
            output.push(' ');
            input = &input[1..];
            found = true;
        } else if input.starts_with(['\n', '\r']) {
            output.push('\n');
            input = &input[1..];
            found = true;
        } else if input.chars().next().unwrap_or_default().is_numeric() {
            output.push_str(&input[..1]);
            input = &input[1..];
            found = true;
        } else if let Some(rest) = input.strip_prefix("A'") {
            output.push('w');

            if !rest.starts_with(['a', 'e', 'i', 'o', 'u']) {
                output.push('e');
            }

            input = rest;
            found = true;
        } else if let Some(rest) = input.strip_prefix("E'") {
            output.push('j');

            if !rest.starts_with(['a', 'e', 'i', 'o', 'u']) {
                output.push('e');
            }

            input = rest;
            found = true;
        } else if let Some(rest) = input.strip_prefix("O'") {
            output.push('h');

            if !rest.starts_with(['a', 'e', 'i', 'o', 'u']) {
                output.push('e');
            }

            input = rest;
            found = true;
        }

        if !found {
            eprintln!("No replacement found for phoneme: {input}");
            input = &input[1..];
            output += "?";
        }
    }

    output
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
    (&['r', 'ɹ', 'R'], "r"),
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
    // Combinatory sounds
    (&['ɚ', 'ɜ'], "uhr"),
    (&['ʔ', 'ʲ', 'h'], ""), // ignore stray h's for now - those have special meaning
];
