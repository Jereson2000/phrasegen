use phrasegen::{actions::Actions, settings::Settings, ui};

fn main() {
    let settings = Settings::new();

    loop {
        let user_action = ui::select_action();

        match user_action {
            Ok(action) => {
                let result = match action {
                    Actions::Quit => break,
                    Actions::Generate => ui::generate_passphrase(&settings),
                    Actions::Settings => ui::change_settings(),
                };

                match result {
                    Err(error) => println!("{error}"),
                    Ok(_) => {}
                };
            }
            Err(error) => println!("{error}"),
        }
    }
}
