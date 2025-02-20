use super::espeakng::text_to_phonemes;

pub trait PhonemeExt {
    fn to_ipa(&self) -> Option<String>;
    fn to_glyphs(&self) -> Option<String>;
}

impl PhonemeExt for &str {
    fn to_ipa(&self) -> Option<String> {
        if ["E", "O", "A"].contains(self) {
            return Some(self.to_string());
        }

        text_to_phonemes(self).ok()
    }

    fn to_glyphs(&self) -> Option<String> {
        if ["E", "O", "A"].contains(self) {
            return Some(self.to_string());
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
                eprintln!("No replacement found for phoneme: {}", char);
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
    (&['i', 'ɪ'], "i"),
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
    (&['j'], "E'"),
    (&['w'], "A'"),
    (&['h'], "O'"),
    //
    // Combinatory sounds
    (&['ɚ', 'ɜ'], "uhr"),
    (&['ʔ'], ""),
];
