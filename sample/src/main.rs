mod core;

fn main() {
    let mut notes = Vec::new();
    loop {
        core::print_menue();
        let command: String = core::read_input();
        match command.trim() {
            "show" => core::show_notes(&notes),
            "add" => core::add_notes(&mut notes),
            _ => break,
        }
    }
}