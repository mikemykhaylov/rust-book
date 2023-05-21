use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    println!("Hello, world!");
    // panic!("crash and burn");

    let v = vec![1, 2, 3];

    // v[99];
    v.get(99); // doesn't panic

    // recoverable errors
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating file: {:?}", error),
            },
            other_error => panic!("Problem opening file: {:?}", error),
        },
    };

    // custom handling
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });

    // calls generic panic!
    let greeting_file = File::open("hello.txt").unwrap();

    // calls panic! with custom message
    let greeting_file =
        File::open("hello.txt").expect("\"hello.txt\" should be included in the project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_clean() -> Result<String, io::Error> {
    // we can also use the built-in fs method
    // fs::read_to_string("hello.txt");

    // ? calls `from` when receiving an error, effectively casting it
    // to the error return type of the fn. useful for defining custom errors
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    // we can also use ? on fns returning Option
    text.lines().next()?.chars().last()
}
