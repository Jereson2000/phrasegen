use crate::{actions::Actions, utils::parse_wordlist};
use dialoguer::{theme::ColorfulTheme, Select};
use rand::seq::IndexedRandom;

pub fn select_action() -> Result<Actions, dialoguer::Error> {
    let action_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an action")
        .items(&Actions::VALUES)
        .default(0)
        .interact()?;

    Ok(Actions::VALUES[action_index].clone())
}

pub fn select_settings() -> Result<(), std::io::Error> {
    Ok(())
}

pub fn generate_passphrase() -> Result<(), std::io::Error> {
    // yes yes... very bad to publish wordlist in git... yes yes...
    // TODO: get path from settings.
    let path = String::from("./kaikkisanat.txt");
    let wordlist = parse_wordlist(path)?;
    // TODO: get amount from settings.
    let words: Vec<&String> = wordlist.choose_multiple(&mut rand::rng(), 4).collect();

    for word in words {
        println!("{word}");
    }

    // TODO: get delimiter from settings.
    Ok(())
}

