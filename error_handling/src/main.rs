use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        // No problem
        Ok(file) => file,
        // Some problem
        Err(error) => match error.kind() {
            // File not found => create file
            ErrorKind::NotFound => match File::create("hello.txt") {
                // Created
                Ok(fc) => fc,
                // Problem creating
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            // Any other error
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // Another way to write this
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // To panic on any error, can use
    let f = File::open("hello.txt").unwrap();

    // To panic with an error message
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// Propagate errors upward from a function
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    // main possible return values
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Propagate errors with the ? shortcut
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Even shorter
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// One line
fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
