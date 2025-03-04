use crate::actions::Actions;
use dialoguer::{Select, theme::ColorfulTheme};

pub fn run() {
    loop {
        // User selects which action to execute.
        // Valid actions are documented in the actions.rs.
        let user_action = select_action();

        match user_action {
            Actions::Quit => break,
            Actions::Settings => todo!(),
            Actions::Generate =>  select_generator_options(),
        }
    }
}

pub fn select_action() -> Actions {
    let action_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you choose?")
        .items(&Actions::VALUES)
        .default(0)
        .interact()
        .expect("Failed to select action!");

    Actions::VALUES[action_index].clone()
}

pub fn select_generator_options() {

}