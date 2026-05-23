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

#[derive(Debug, PartialEq)]
enum IpaItem {
    Word(Vec<String>),
    Space,
    Newline,
    SentenceBoundary,
}

/// Convert a string of IPA characters to a string of phonemes
#[allow(clippy::too_many_lines, reason = "ok yeah but shuddup tho")]
#[must_use]
pub fn glyphs_to_ipa(input: &str) -> String {
    let mut input = input;
    let mut items = vec![];
    let mut current_syllable = String::new();
    let mut current_word = vec![];

    input = input.trim();
    if input.is_empty() {
        return String::new();
    }

    while !input.is_empty() {
        let mut found = false;
        for (chars, replacement) in PHONEME_REPLACEMENT_TABLE {
            if replacement.is_empty() {
                continue;
            }

            if let Some(rest) = input.strip_prefix(replacement) {
                current_syllable.push(chars[0]);
                input = rest;
                found = true;
                break;
            }
        }

        let next = input.chars().next().unwrap_or_default();
        match next {
            '|' => {
                input = &input[1..];
                found = true;
            }

            ' ' => {
                if !current_syllable.is_empty() {
                    current_word.push(current_syllable);
                    current_syllable = String::new();
                }

                if !current_word.is_empty() {
                    items.push(IpaItem::Word(current_word));
                    current_word = vec![];
                }

                input = &input[1..];
                items.push(IpaItem::Space);
                found = true;
            }

            '\'' => {
                current_word.push(current_syllable);
                current_syllable = String::new();
                input = &input[1..];
                found = true;
            }

            '\n' | '\r' => {
                if !current_syllable.is_empty() {
                    current_word.push(current_syllable);
                    current_syllable = String::new();
                }

                if !current_word.is_empty() {
                    items.push(IpaItem::Word(current_word));
                    current_word = vec![];
                }

                items.push(IpaItem::Newline);
                input = &input[1..];
                found = true;
            }

            '.' | '!' | '?' => {
                if !current_syllable.is_empty() {
                    current_word.push(current_syllable);
                    current_syllable = String::new();
                }

                if !current_word.is_empty() {
                    items.push(IpaItem::Word(current_word));
                    current_word = vec![];
                }

                items.push(IpaItem::SentenceBoundary);
                input = &input[1..];
                found = true;
            }

            x if x.is_numeric() => {
                current_syllable.push(x);
                input = &input[1..];
                found = true;
            }

            _ => (),
        }

        if let Some(rest) = input.strip_prefix("A'") {
            current_syllable.push('w');

            if !rest.starts_with(['a', 'e', 'i', 'o', 'u']) {
                current_syllable.push('e');
                current_word.push(current_syllable);
                current_syllable = String::new();
            }

            input = rest;
            found = true;
        } else if let Some(rest) = input.strip_prefix("E'") {
            current_syllable.push('j');

            if !rest.starts_with(['a', 'e', 'i', 'o', 'u']) {
                current_syllable.push('e');
                current_word.push(current_syllable);
                current_syllable = String::new();
            }

            input = rest;
            found = true;
        } else if let Some(rest) = input.strip_prefix("O'") {
            current_syllable.push('h');

            if !rest.starts_with(['a', 'e', 'i', 'o', 'u']) {
                current_syllable.push('e');
                current_word.push(current_syllable);
                current_syllable = String::new();
            }

            input = rest;
            found = true;
        }

        if input.is_empty() {
            current_word.push(current_syllable);
            items.push(IpaItem::Word(current_word));
            items.push(IpaItem::SentenceBoundary);
            break;
        }

        if !found {
            eprintln!(
                "No replacement found for phoneme: {}",
                input.chars().next().unwrap_or_default()
            );
            input = &input[1..];
            current_syllable += "?";
        }
    }

    let mut output = String::new();
    for item in items {
        match item {
            IpaItem::Word(mut syllables) => {
                let last = syllables.pop().unwrap_or_default();
                let mut word = syllables.join(".");
                if !syllables.is_empty() {
                    word.push_str(".ˈ");
                }
                word.push_str(&last);
                output.push('/');
                output.push_str(&word);
                output.push('/');
            }

            IpaItem::Space => output.push(' '),
            IpaItem::Newline => output.push('\n'),
            IpaItem::SentenceBoundary => output.push('.'),
        }
    }

    output
}

const PHONEME_REPLACEMENT_TABLE: &[(&[char], &str)] = &[
    //
    // Vowels
    (&['ɑ', 'ɔ', 'ə', 'ɐ'], "ah"),
    (&['a', 'æ'], "a"),
    (&['e', 'ɛ', 'ᵻ'], "e"),
    (&['i', 'ɪ', 'j'], "i"),
    (&['o'], "o"),
    (&['ʌ', 'ʊ'], "uh"),
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
