use crate::actions::Actions;
use dialoguer::{Select, theme::ColorfulTheme};
use rand::seq::IndexedRandom;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn select_action() -> Actions {
    let action_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an action")
        .items(&Actions::VALUES)
        .default(0)
        .interact()
        .expect("Failed to select action.");

    Actions::VALUES[action_index].clone()
}

pub fn select_settings() {}

pub fn generate_passphrase() {
    // yes yes... very bad to publish wordlist in git... yes yes...
    // TODO: get path from settings.
    let path = String::from("./kaikkisanat.txt");
    let wordlist = parse_wordlist(path);
    // TODO: get amount from settings.
    let words: Vec<&String> = wordlist.choose_multiple(&mut rand::rng(), 4).collect();

    for word in words {
        println!("{word}");
    }

    // TODO: get delimiter from settings.
}

// TODO: Replace String with the Path type.
fn parse_wordlist(path: String) -> Vec<String> {
    let file = File::open(path).expect("Couldn't read the wordlist.");
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

    wordlist
}
