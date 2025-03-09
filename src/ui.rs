use crate::{actions::Actions, settings::Settings, utils::parse_wordlist};
use dialoguer::{Select, theme::ColorfulTheme};
use rand::seq::IndexedRandom;

pub fn select_action() -> Result<Actions, dialoguer::Error> {
    let action_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an action")
        .items(&Actions::VALUES)
        .default(0)
        .interact()?;

    Ok(Actions::VALUES[action_index].clone())
}

pub fn change_settings() -> Result<(), std::io::Error> {
    Ok(())
}

// TODO: refactor this pile of garbage.
pub fn generate_passphrase(settings: &Settings) -> Result<(), std::io::Error> {
    // Parse the wordlist provided by settings.
    let wordlist = parse_wordlist(settings.wordlist())?;

    // Collect 4 random Strings from wordlist
    let words: Vec<&String> = wordlist
        .choose_multiple(&mut rand::rng(), settings.wordcount())
        .collect();

    // map the words to correct types and join to get the phrase.
    let phrase = words
        .iter()
        .map(|w| w.as_str())
        .collect::<Vec<&str>>()
        .join(&settings.delimiter().to_string());

    // print the phrase.
    println!("{phrase}");

    Ok(())
}
