use phrasegen::{actions::Actions, ui};

fn main() {
    loop {
        let user_action = ui::select_action();

        match user_action {
            Ok(action) => {
                let result = match action {
                    Actions::Quit => break,
                    Actions::Generate => ui::generate_passphrase(),
                    Actions::Settings => ui::select_settings(),
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
