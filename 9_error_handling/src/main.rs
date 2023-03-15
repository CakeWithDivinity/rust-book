use std::fs::File;
use std::io::{ErrorKind, self, Read};

fn main() {
    let v = vec![1, 2, 3];

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username) 
}
