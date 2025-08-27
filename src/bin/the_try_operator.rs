use std::fs::File;
use std::io::{self, stdin, Read};



fn main() {
    let file_result = read_file();

    match file_result {
        Ok(contents) => println!("{contents}"),
        Err(error) => {
            eprintln!("There was an error: {error:?}");
        }
    }
}

fn read_file() -> Result<String, io::Error> {
    println!("Please enter the name of the file you'd like to read:");

    let mut input: String = String::new();
    stdin().read_line(&mut input)?;

    let mut file_contents = String::new();
    File::open(input.trim())?.read_to_string(&mut file_contents)?;
    
    Ok(file_contents)
}