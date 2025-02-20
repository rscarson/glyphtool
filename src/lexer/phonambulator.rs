//! The part of the translation process that converts a word into a set of phonemes.
//!
//! The word will be split into groups by ', and each group is looked up in the database
//!
//! If nothing is found, the user will be prompted to enter a new phonambulation.
//!
use crate::autophononimbus::Autophononimbus;
use crate::database::Database;
use crate::error::Result;
use crate::lexer::collections::WordKind;
use crate::renderer::utilities::glyphs_for;

pub struct Phonambulator {
    db: Database,
    auto: Option<Autophononimbus>,
}
impl Phonambulator {
    pub fn new(path: Option<&str>, is_auto: bool) -> Result<Self> {
        let db = Database::new(path)?;
        let auto = if is_auto {
            Some(Autophononimbus::new()?)
        } else {
            None
        };
        Ok(Self { db, auto })
    }

    pub fn get_next(&mut self, group: &str) -> Result<String> {
        if let Some(auto) = &mut self.auto {
            if let Some(result) = auto.syllabify(group) {
                return Ok(result);
            }
        }

        let result = prompt_user(group);
        self.db.insert(group, &result)?;
        Ok(result)
    }

    pub fn phonambulate(&mut self, word: &str) -> Result<String> {
        let mut phonemes = vec![];

        for group in word.split('\'') {
            let phoneme = match self.db.get_encoded(group)? {
                Some(phoneme) => phoneme,
                None => self.get_next(group)?,
            };

            phonemes.push(phoneme);
        }

        let phonemes = phonemes.join("'").replace("''", "'");
        Ok(phonemes)
    }
}

#[rustfmt::skip]
fn prompt_user(target: &str) -> String {
    println!("No mapping found for `{target}` - please enter a set of phoneme groups seperated by '-'");
    println!("For example, 'mad-shik' for magic");
    println!("Vowel Sounds:       [ah/a   ]  [  e/i  ] [   u/uh   ] [ o ]");
    println!("                    [on/apple] [egg/ice] [oops/dunce] [oat]");
    println!("Closed Consonants:  [m] [p] [b] [f]");
    println!("Open Consonants:    [t] [r] [rr | rolled r] [l] [s] [sh] [th] [n] [ng] [k] [d] [z]");
    println!("Special characters: [E' | Deific Mark   ] [O' | Posessive Mark] [A' | Honourific Mark]");
    println!("                    [y- | yellow, yonder] [h- | heather, hoot ] [w- | water, weather ]");
    println!();
    println!("[ j = d-sh ] [ ch = t-sh ] [ v = f ] [ g = k ]");    

    loop {
        print!("{target} > ");
        std::io::Write::flush(&mut std::io::stdout()).ok();

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        println!();
        match validate_phonemes(&input) {
            Ok(output) => return output.to_string(),
            Err(output) => println!("ERROR: Unrecognized phoneme: {output}"),
        }
    }
}

pub fn validate_phonemes(phonemes: &str) -> std::result::Result<&str, String> {
    let phonemes = phonemes.trim();
    let groups = phonemes.split('-').map(ToString::to_string).collect();
    let glyphs = WordKind::PhonemeGroup(groups);
    let glyphs = glyphs_for(&glyphs);
    let glyphs = glyphs.into_iter().flatten();
    let glyphs: String = glyphs.map(|g| g.as_ref().pronounciation).collect();
    if glyphs.contains('?') {
        Err(glyphs)
    } else {
        Ok(phonemes)
    }
}
