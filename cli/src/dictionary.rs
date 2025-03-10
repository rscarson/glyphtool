pub struct Dictionary(Vec<String>);
impl Dictionary {
    pub fn load() -> Self {
        let words = include_str!("dictionary");
        let mut words: Vec<_> = words.lines().map(|s| s.to_string()).collect();

        // Sort by string len descending
        words.sort_by_key(|b| std::cmp::Reverse(b.len()));

        Self(words)
    }

    pub fn process(&self, input: &str) -> Vec<&str> {
        // Remove lines that begin with '#'
        let input = input
            .lines()
            .filter(|l| !l.starts_with('#'))
            .collect::<String>();

        let mut input = input.as_str();
        let mut result = vec![];
        while !input.is_empty() {
            let next = input.chars().next().unwrap();
            if next.is_numeric() || !next.is_alphabetic() {
                input = &input[1..];
            } else {
                let mut found = false;
                for word in &self.0 {
                    if let Some(rest) = input.strip_prefix(word) {
                        result.push(word.as_str());
                        input = rest;
                        found = true;
                        break;
                    }
                }

                if !found {
                    let error = format!("Unknown word: {}...", &input[..10]);
                    eprintln!("{}", error);
                    break;
                }
            }
        }

        result
    }
}
