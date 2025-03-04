use phrasegen::ui;

fn main() {
    loop {
        let user_action = ui::select_action();

        if user_action == Some(()) {
            break;
        }
    }
}
