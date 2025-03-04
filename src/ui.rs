use crate::actions::Actions;
use dialoguer::{Input, Select, theme::ColorfulTheme};

pub fn select_action() -> Option<()> {
    let action_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an action")
        .items(&Actions::VALUES)
        .default(0)
        .interact()
        .expect("Failed to select action.");

    match Actions::VALUES[action_index] {
        Actions::Quit => return Some(()),
        Actions::Settings => select_settings(),
        Actions::Generate => generate_passphrase(),
    };

    None
}

pub fn generate_passphrase() {
    let delimiter: char = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Select delimiter")
        .default('-')
        .interact_text()
        .expect("Failed to select delimiter.");

    let word_length: u8 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Select word length")
        .default(4)
        .interact_text()
        .expect("Failed to select the word length.");

    println!("delimiter: {delimiter}, word_length: {word_length}");
}

pub fn select_settings() {}
