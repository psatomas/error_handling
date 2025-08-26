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

    let user_requested_file = stdin().read_line(&mut input);

    if let Err(error) = user_requested_file {
        return Err(error);
    }

    let mut file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    
    let mut file_contents = String::new();
    let read_operations = file.read_to_string(&mut file_contents);

    if let Err(error) = read_operations {
        return Err(error);
    }

    Ok(file_contents)
}