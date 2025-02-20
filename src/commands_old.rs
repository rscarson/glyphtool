use crate::{
    database::Database,
    error::Result,
    lexer::{self, collections::WordKind},
    postprocessor::{ImageExt, OutputImage},
    renderer::{glyphs::GLYPH_SOUND_MAP, utilities::ipa_string_for, GlyphBlockRenderer},
};

pub fn command_parser() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let mut args = args.into_iter();

    let cmd = args.next().map(|s| s.to_lowercase());
    let cmd = cmd.as_deref().unwrap_or("help");

    match cmd {
        "add" => {
            let Some(word) = args.next() else {
                println!("Usage: phonambulator add [word] [phonemes]");
                return Ok(());
            };

            let Some(phonemes) = args.next() else {
                println!("Usage: phonambulator add [word] [phonemes]");
                return Ok(());
            };

            let db = Database::new(None)?;
            db.insert(&word, &phonemes)?;
        }

        "list" => {
            let db = Database::new(None)?;

            let words = if let Some(phonemes) = args.next() {
                db.search(&phonemes)?
            } else {
                db.all()?
            };

            let max_len = words
                .iter()
                .fold(0, |max, (word, _)| std::cmp::max(max, word.len()));

            for (word, phonemes) in &words {
                let word = format!("{:>1$}", word, max_len);
                println!("  {word} - {phonemes}");
            }

            println!("Found {} words", words.len());
        }

        "delete" => {
            let Some(word) = args.next() else {
                println!("Usage: phonambulator delete [word]");
                return Ok(());
            };

            let db = Database::new(None)?;
            db.delete(&word)?;
        }

        "translate" => {
            let Some(text) = args.next() else {
                println!("Usage: phonambulator translate [text]");
                return Ok(());
            };

            let auto = args.any(|arg| arg == "--auto");
            let block = lexer::parse(&text, None, auto)?;
            println!("{block}");
        }

        "ipa" => {
            let Some(text) = args.next() else {
                println!("Usage: phonambulator ipa [word]");
                return Ok(());
            };

            let auto = args.any(|arg| arg == "--auto");
            let block = lexer::parse(&text, None, auto)?;
            for line in block.lines() {
                for token in line.sentences() {
                    for word in token.words() {
                        let ipa = ipa_string_for(word);
                        print!("{ipa} ");
                    }

                    print!(". ");
                }

                println!();
            }
        }

        "ipa_from" => {
            let Some(path) = args.next() else {
                println!("Usage: phonambulator ipa_from [path]");
                return Ok(());
            };
            let auto = args.any(|arg| arg == "--auto");

            let text = std::fs::read_to_string(&path)?;
            let block = lexer::parse(&text, None, auto)?;
            for line in block.lines() {
                for token in line.sentences() {
                    for word in token.words() {
                        let ipa = ipa_string_for(word);
                        print!("{ipa} ");
                    }

                    print!(". ");
                }

                println!();
            }
        }

        "reverse_ipa" => {
            let Some(text) = args.next() else {
                println!("Usage: phonambulator reverse_ipa [word]");
                return Ok(());
            };

            let syllables = text.split('-');
            let word = WordKind::PhonemeGroup(syllables.map(|s| s.to_string()).collect());
            let ipa = ipa_string_for(&word);

            println!("{ipa}");
        }

        "translate_from" => {
            let Some(path) = args.next() else {
                println!("Usage: phonambulator translate_from [path]");
                return Ok(());
            };
            let auto = args.any(|arg| arg == "--auto");

            let text = std::fs::read_to_string(&path)?;
            let block = lexer::parse(&text, None, auto)?;
            println!("{block}");
        }

        "phoneme" => {
            let Some(sound) = args.next() else {
                println!("Usage: phonambulator phoneme [sound]");
                return Ok(());
            };

            let (mut w, mut h) = (0, 0);
            for arg in args {
                if let Some(width) = arg.strip_prefix("w=") {
                    w = width.parse().unwrap_or_default();
                } else if let Some(height) = arg.strip_prefix("h=") {
                    h = height.parse().unwrap_or_default();
                }
            }

            for (pronounciation, glyph) in GLYPH_SOUND_MAP {
                if &sound == pronounciation {
                    println!("{}", glyph.render_ascii(w, h));
                    return Ok(());
                }
            }

            println!("No phoneme found for {sound}");
        }

        "ascii" => {
            let Some(text) = args.next() else {
                println!("Usage: phonambulator ascii [text] ?[margin=]");
                return Ok(());
            };

            let mut margin = 1;
            let mut auto = false;
            for arg in args {
                if let Some(m) = arg.strip_prefix("margin=") {
                    margin = m.parse().unwrap_or_default();
                }

                if arg == "--auto" {
                    auto = true;
                }
            }

            let block = lexer::parse(&text, None, auto)?;
            let renderer = GlyphBlockRenderer::new(&block, margin);
            let bitmap = renderer.render();

            for row in bitmap {
                for pixel in row {
                    print!("{}", if pixel == 0xFF { " " } else { "█" });
                }
                println!();
            }
        }

        "export" => {
            let Some(path) = args.next() else {
                println!("Usage: phonambulator export [path] [text] ?[scale=]");
                return Ok(());
            };

            let Some(text) = args.next() else {
                println!("Usage: phonambulator export [path] [text] ?[scale=]");
                return Ok(());
            };

            let (mut scale, mut margin) = (3, 1);
            let mut auto = false;
            let mut filter = None;
            for arg in args {
                if let Some(s) = arg.strip_prefix("scale=") {
                    scale = s.parse().unwrap_or_default();
                } else if let Some(m) = arg.strip_prefix("margin=") {
                    margin = m.parse().unwrap_or_default();
                } else if let Some(f) = arg.strip_prefix("filter=") {
                    filter = Some(f.to_string());
                }

                if arg == "--auto" {
                    auto = true;
                }
            }

            let block = lexer::parse(&text, None, auto)?;
            let renderer = GlyphBlockRenderer::new(&block, margin);

            let mut bitmap = renderer.render();
            bitmap.scale(scale);

            let mut image = bitmap.to_grayscale();
            if let Some(filter) = filter {
                match filter.as_str() {
                    "space" => image.filter_space(1.0),
                    "sketch" => image.filter_sketch(1.0),
                    _ => {
                        println!("Unknown filter {filter}");
                    }
                }
            }

            image.export(path)?;
        }

        "export_from" => {
            let Some(path_in) = args.next() else {
                println!(
                    "Usage: phonambulator export_from [path in] [path out] ?[margin=] ?[scale=]"
                );
                return Ok(());
            };

            let Some(path_out) = args.next() else {
                println!(
                    "Usage: phonambulator export_from [path in] [path out] ?[margin=] ?[scale=]"
                );
                return Ok(());
            };

            let (mut scale, mut margin) = (10, 1);
            let mut auto = false;
            let mut filter = Some("sketch".to_string());
            for arg in args {
                if let Some(s) = arg.strip_prefix("scale=") {
                    scale = s.parse().unwrap_or_default();
                } else if let Some(m) = arg.strip_prefix("margin=") {
                    margin = m.parse().unwrap_or_default();
                } else if let Some(f) = arg.strip_prefix("filter=") {
                    filter = Some(f.to_string());
                }

                if arg == "--auto" {
                    auto = true;
                }
            }

            let text = std::fs::read_to_string(&path_in)?;
            let block = lexer::parse(&text, None, auto)?;
            let renderer = GlyphBlockRenderer::new(&block, margin);

            let mut bitmap = renderer.render();
            bitmap.scale(scale);

            let mut image = bitmap.to_grayscale();
            if let Some(filter) = filter {
                match filter.as_str() {
                    "space" => image.filter_space(1.0),
                    "sketch" => image.filter_sketch(1.0),
                    _ => {
                        println!("Unknown filter {filter}");
                    }
                }
            }

            image.export(path_out)?;
        }

        "filter" => {
            let Some(filter) = args.next() else {
                println!("Usage: phonambulator filter [space | sketch] [path in] [path out] ?[strength=]");
                return Ok(());
            };

            let Some(path_in) = args.next() else {
                println!("Usage: phonambulator filter [space | sketch] [path in] [path out] ?[strength=]");
                return Ok(());
            };

            let Some(path_out) = args.next() else {
                println!("Usage: phonambulator filter [space | sketch] [path in] [path out] ?[strength=]");
                return Ok(());
            };

            let mut strength: f32 = 1.0;
            for arg in args {
                if let Some(s) = arg.strip_prefix("strength=") {
                    strength = s.parse().unwrap_or_default();
                }
            }
            strength = strength.clamp(0.1, 10.0);

            let mut image = OutputImage::load(&path_in)?;
            match filter.as_str() {
                "space" => image.filter_space(strength),
                "sketch" => image.filter_sketch(strength),
                _ => {
                    println!("Unknown filter {filter}");
                    return Ok(());
                }
            }

            image.export(path_out)?;
        }

        _ => {
            println!("Usage: etrois [command] [args]");
            println!("Commands:");
            println!("  help - Display this help message");

            println!();
            println!("Database Commands:");
            println!("  add [word] [phonemes] - Add a word to the database");
            println!("  search ?[phonemes] - Search for words with the given phonemes. List all words if no phonemes are given");
            println!("  delete [word] - Delete a word from the database");
            println!("  list - Display all words in the database");

            println!();
            println!("Translation Commands:");
            println!("  translate [text] ?[--auto] - Translate the given text into phonemes");
            println!(
                "  reverse_ipa [word] ?[--auto] - Display the IPA representation of the given phoneme string"
            );
            println!("  ipa_from [path] ?[--auto] - Display the IPA representation of the given text file");
            println!("  ipa [word] ?[--auto] - Display the IPA representation of the given word");
            println!(
                "  translate_from [path] ?[--auto] - Translate the given text in a given file into phonemes"
            );

            println!();
            println!("Rendering Commands:");
            println!("  phoneme [sound] ?[w=] ?[h=] - Render the glyph for a given phoneme");
            println!("  ascii [text] ?[margin=] - Render the given text in ASCII art");
            println!(
                "  export [path] [text] ?[scale=] ?[margin=] ?[filter=] - Export the given text as an image"
            );
            println!(
                "  export_from [path in] [path out]  ?[margin=] ?[scale=] ?[filter=] - Export the given text as an image"
            );

            println!();
            println!("Post-processing Commands:");
            println!("  filter [space | sketch] [path in] [path out] ?[strength=] - Apply a filter to an image");
        }
    }

    Ok(())
}
