use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        show_help();
    } else {
        let option: String = String::from(&args[1]);

        match option.as_str() {
            "-h" => show_help(),
            "-a" => add_notes(args),
            _ => show_help()
        }
    }
}

fn add_notes(notes: Vec<String>) {
    if notes.len() < 3 {
        println!("not enough arguments");
    } else {
        let note: String = String::from(&notes[2]);
        let val: Vec<&str> = note.split(",").collect();

        for c in val.iter() {
            println!("{}", c);
        }
    }
}

fn show_help() {
    println!("below are the options,");
    println!("-a add note");
    println!("-s search note");
    println!("-d delete note");
    println!("-h show help");
}