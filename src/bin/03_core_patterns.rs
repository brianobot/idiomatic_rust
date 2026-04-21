#![allow(dead_code)]
use std::{path::Path, sync::{Arc, Condvar, Mutex}, thread};


#[derive(Debug)]
struct Person {
    state: String
}


impl Person {
    fn from(state: &str) -> Self {
        Self { state: state.to_string() }
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("Dropping Person!: {}", self.state);
    }
}


static NAME: &str = "hello world";
const DIFFICULTY: usize = 10;


fn main() -> std::io::Result<()> {
    // Objects mgt in Rust follows the RAII rule except for unsafe code and Copy values
    // let status = Person::from("Active");
    
    // let statuses = vec![status]; // at this point value has been moved into the Vec and status is uninitialized
    // println!("{:?}", statuses);
    // println!("Reached the End of the Main function!");
    // 
    let outer = Arc::new((Mutex::new(0), Condvar::new()));
    let inner = outer.clone();
    
    thread::spawn(move || {
        let (mutex, condvar) = &*inner;
        let mut guard = mutex.lock().unwrap();
        *guard += 1;
        println!("Inner Guard: {guard}");
        condvar.notify_one();
    });
    
    let (mutex, condvar) = &*outer;
    let mut guard = mutex.lock().unwrap();
    println!("Outer Before wait: {guard}");
    
    while *guard == 0 {
        guard = condvar.wait(guard).unwrap();
    }
    
    println!("Outer Guard After wait: {guard}");
    
    let rev_brian = reverse(String::from("Brian"));
    println!("{rev_brian}");
    
    // Errors are values in Rust, Struct or Enums
    let path = Path::new("Cargo.toml");
    
    let second_line = read_nth_line(path, 1).expect("Failed becuase if this -> ");
    println!("The second line from {:?} is {}", path, second_line);
    
    
    println!("Name: {NAME}");
    println!("Difficulty: {DIFFICULTY}");
    Ok(())
    
}


fn reverse(s: String) -> String {
    let mut v = s.chars().collect::<Vec<char>>();
    v.reverse();
    String::from_iter(v.iter())
}


#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    BadLineArgument(usize),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}


fn read_nth_line(path: &Path, n: usize) -> Result<String, Error> {
    if n < 1 {
        return Err(Error::BadLineArgument(n))
    }
    
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    
    let file = File::open(path)?;
    let mut reader_lines = BufReader::new(file).lines();
    
    reader_lines
        .nth(n - 1)
        .map(|result| result.map_err(|err| err.into()))
        .unwrap_or_else(|| Err(Error::BadLineArgument(n)))
}