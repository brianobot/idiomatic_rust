use std::{borrow::Cow, cell::RefCell};




fn main() {
    // mostly bugs in programming are linked to one of these causes
    // - logic error
    // - race conditions
    // - unexpected side effects
    // - memory saftey problems
    // 
    // leaning to immutability can and would help reduce the bugs causes by all these reasons
    let x = 1;
    println!("x = {x}");
    
    let y = x + 1;
    println!("y = {y}");
    
    let mut x = x;
    x += 1;
    println!("x = {x}");
    
    
    fn mutability(a: u8, mut b: u8) {
        b += 1;
        println!("a = {a}, b = {b}");
    }
    
    
    let a = 10;
    let b = 20;
    mutability(a, b);
    
    println!("a = {a}, b = {b}");
    
    let immutable_string = String::from("Immutable String");
    let not_so_immutable_string = RefCell::from(immutable_string);
    
    not_so_immutable_string.borrow_mut().push_str(".");
    println!("{not_so_immutable_string:?}");
    
    let cow_say_what = Cow::from("Hello World");
    println!("{cow_say_what:?}");
    
    let cow_dont_say_what = cow_say_what.clone().to_mut().replace("World", "Mars");
    println!("{cow_dont_say_what:?}");
    
    println!("{cow_say_what:?}");
    println!("{cow_dont_say_what:?}");
}