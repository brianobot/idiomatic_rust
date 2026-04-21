#![allow(dead_code)]
use std::{sync::{Arc, Condvar, Mutex}, thread};


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

fn main() {
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
}


fn reverse(s: String) -> String {
    let mut v = s.chars().collect::<Vec<char>>();
    v.reverse();
    String::from_iter(v.iter())
}