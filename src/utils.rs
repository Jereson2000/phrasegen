use std::{fs::File, io::{BufRead, BufReader, Error}};

// TODO: Replace String with the Path type.
pub fn parse_wordlist(path: String) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut wordlist = Vec::<String>::new();
    let removable_chars = ["\n", "-", " "];
    let replacable_chars = ["å", "ä", "ö"];

    // Read the file by lines.
    // TODO: make it so that the lines are read by words.
    for line in reader.lines() {
        // Unwrap the result and print error if it fails.
        let mut trim = line.expect("Couldn't read the line");

        // Delete all characters that might affect the output.
        for char in removable_chars {
            trim = trim.replace(char, "");
        }

        // Replace some characters with understandable counterparts.
        for char in replacable_chars {
            match char {
                "ö" => trim = trim.replace(char, "o"),
                _ => trim = trim.replace(char, "a"),
            };
        }

        // Drop words with than 4 bytes.
        // TODO: implement this better. maybe with graphemes or characters!.
        if trim.len() < 4 {
            continue;
        }

        // Push the procesessed string into the wordlist.
        wordlist.push(trim);
    }

    Ok(wordlist)
}
