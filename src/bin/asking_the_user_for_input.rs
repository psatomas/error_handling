use std::fs::File;
use std::io::stdin;
use std::process;

fn main() {
    println!("Please enter the name of the file you'd like to read:");
    let mut input: String = String::new();

    let user_requested_file = stdin().read_line(&mut input);

    if let Err(error) = user_requested_file {
        eprintln!("Something went wrong collecting user input, the error was {error:?}");
        process::exit(1);
    }

    let file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong reading the file. The error was {error:?}");
            process::exit(1);
        }
    };

    println!("{file:?}");
}