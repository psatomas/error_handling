use std::fs::File;
use std::process;

fn main() {
    let file = match File::open("story.txt") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong reading the file. The error was {error:?}");
            process::exit(1);
        }
    };

    println!("{file:?}");
}