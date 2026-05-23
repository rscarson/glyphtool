use libglyphtool::{error::EtroisResult, lexer::phonambulator::PhonambulationSource};

pub(crate) struct StdinSource(bool);
impl StdinSource {
    pub fn new(auto: bool) -> Self {
        Self(auto)
    }
}
impl PhonambulationSource for StdinSource {
    fn get_next(
        &mut self,
        target: &str,
        auto_suggested: &str,
        err: Option<&str>,
    ) -> EtroisResult<String> {
        if self.0 {
            return Ok(String::new());
        }

        println!(
            "No mapping found for `{target}` - please enter a set of phoneme groups seperated by '-'"
        );
        println!("For example, 'mad-shik' for magic");
        println!("Vowel Sounds:       [ah/a   ]  [  e/i  ] [   u/uh   ] [ o ]");
        println!("                    [on/apple] [egg/ice] [oops/dunce] [oat]");
        println!("Closed Consonants:  [m] [p] [b] [f]");
        println!("Open Consonants:    [t] [r] [l] [s] [sh] [th] [n] [ng] [k] [d] [z]");
        println!(
            "Special characters: [E' | Deific Mark   ] [O' | Posessive Mark] [A' | Honourific Mark]"
        );
        println!(
            "                    [y- | yellow, yonder] [h- | heather, hoot ] [w- | water, weather ]"
        );
        println!();
        println!("[ j = d-sh ] [ ch = t-sh ] [ v = f ] [ g = k ]");

        println!("Suggested: {auto_suggested}");
        if let Some(err) = err {
            println!("ERROR: Unrecognized phoneme: {err}");
        }
        print!("{target} > ");
        std::io::Write::flush(&mut std::io::stdout()).ok();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
}
