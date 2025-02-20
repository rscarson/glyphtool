pub fn preprocess_word(input: &str) -> String {
    let mut output = input.to_string();

    // Words ending in 's are posessive - remove the suffix and add the O' prefix
    if let Some(s) = output.strip_suffix("'s") {
        output = s.to_string();
    }

    // English compound words use -, replace with '
    output = output.replace('-', "'");

    output
}

pub fn preprocess_text(input: &str) -> String {
    let mut output = input.to_string();

    // Remove floating dashes and apostrophes
    output = output.replace(" -", " ");
    output = output.replace("- ", " ");
    output = output.replace(" '", " ");
    output = output.replace("' ", " ");

    output
}
