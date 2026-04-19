
#![allow(dead_code)]
use std::fmt::{Debug, Display};


fn some_or_none<T: Display + Debug>(option: &Option<T>) {
    match option {
        Some(__) => println!("Is Some: inner value = {__:?}"),
        None => println!("Is None"),
    }
}

fn what_type_of_number_is_this(value: u8) {
    match value {
        1 => println!("This is the number one"),
        value @ (2 | 3) => println!("Thi is number {value}"),
         _ => println!("Higher than 3"),
    }
}

#[derive(Debug)]
enum ErrorTypes {
    IoError(std::io::Error),
    FormatError(std::fmt::Error)
}

#[derive(Debug)]
struct ErrorWrapper {
    source: ErrorTypes,
    message: String
}

impl From<std::io::Error> for ErrorWrapper {
    fn from(source: std::io::Error) -> Self {
        Self {
            source: ErrorTypes::IoError(source),
            message: "This was an IO errro".to_string(),
        }
    }
}

fn write_to_file() -> Result<(), ErrorWrapper> {
    use std::fs::File;
    use std::io::prelude::*;
    
    let mut file = File::create("filename")?;
    file.write_all(b"File content")?;
    
    Ok(())
}


fn try_to_write_to_file() {
    match write_to_file() {
        Ok(_) => println!("Wrote to file Successfully"),
        Err(msg) => println!("Failed to write to file: {msg:?}"),
    }
}

fn main() {
    some_or_none::<bool>(&Some(false));
    what_type_of_number_is_this(20);
    
    try_to_write_to_file();
}