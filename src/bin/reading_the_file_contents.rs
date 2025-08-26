use std::fs::File;
use std::io::{stdin, Read};
use std::process;

fn main() {
    println!("Please enter the name of the file you'd like to read:");
    let mut input: String = String::new();

    let user_requested_file = stdin().read_line(&mut input);

    if let Err(error) = user_requested_file {
        eprintln!("Something went wrong collecting user input, the error was {error:?}");
        process::exit(1);
    }

    let mut file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong opening the file. The error was {error:?}");
            process::exit(1);
        }
    };
    
    let mut file_contents = String::new();
    let read_operations = file.read_to_string(&mut file_contents);

    if let Err(error) = read_operations {
        eprintln!("Something went wrong reading the file as a string. The error was {error:?}");
        process::exit(1);
    }

    println!("{file_contents}");
}