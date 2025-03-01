macro_rules! print {
    ($msg:expr) => {
        println!("{}", $msg);
    };
}

pub fn print_menue() {
    println!();
    println!();
    println!("**** PROGRAM MENU ****");
    println!("Enter command:");
    println!("'show' - show all notes");
    println!("'add' - add new note");
    println!("other - exit");
}

pub fn read_input() -> String {
    let mut commad: String = String::new();
    std::io::stdin().read_line(&mut commad).unwrap();
    commad
}

pub fn show_notes(notes: &Vec<String>) {
    println!();
    for note in notes {
        print!(note);
    }
}

pub fn add_notes(notes: &mut Vec<String>) {
    println!("Input note pls:");
    
    let inp: String = read_input();
    let note: String = inp.trim().to_string();

    notes.push(note);
}