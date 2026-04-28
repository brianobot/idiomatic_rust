use std::{ops::Deref, rc::Rc};


struct Person(String, String, u8);


impl Deref for Person {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let person = Person(String::from("Brian"), String::from("Obot"), 25);
    println!("person = {}", *person);
    println!("Length of Person: {}", person.len());
    
    let name = String::from("Brian");
    
    let rc = Rc::new(name);
    let rc2 = rc.clone();
    
    println!("{rc:?}");
    println!("{rc2:?}");
    
    println!("{:?}", Rc::strong_count(&rc));
    println!("{:?}", Rc::strong_count(&rc2));
}