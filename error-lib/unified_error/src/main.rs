use std::{
    fs::File,
    io::{self, Read},
    num::ParseIntError,
};

// 1. Define a unified error type
#[derive(Debug)]
pub enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
    Msg(String),
}

// 2. Implement std::error::Error trait
impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::Io(e) => Some(e),
            MyError::Parse(e) => Some(e),
            MyError::Msg(_) => None,
        }
    }
}

// 3. Implement Display trait
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO error: {}", e),
            MyError::Parse(e) => write!(f, "Parse error: {}", e),
            MyError::Msg(msg) => write!(f, "Error: {}", msg),
        }
    }
}

// 4. Implement From trait to support automatic conversion
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::Io(err)
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> MyError {
        MyError::Parse(err)
    }
}

// 5. Example function: read a number from a file
fn read_number_from_file(path: &str) -> Result<i32, MyError> {
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?; // ? auto-converts to MyError
    let num: i32 = content.trim().parse()?; // ? auto-converts ParseIntError
    Ok(num)
}

// 6. Main function demonstration
fn main() {
    match read_number_from_file("number.txt") {
        Ok(n) => println!("Read number: {}", n),
        Err(e) => eprintln!("Failed: {}", e),
    }
}
