#![allow(dead_code, unused_variables)]

use std::marker::PhantomData;

struct Container<T> {
    value: T
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}


fn main() {
    // often time the compiler can and will figure out the concrete type used in placed of the generic as shown below
    let str_container = Container{ value: "This is a value" };
    
    // other time the compiler needs hints to fully know the concrete type being used
    let ambigious_container: Container<Option<String>> = Container{ value: None };
    
    // creating the structure with the new associated function
    let another_container = Container::new(12);
    let short_ambigious_container = Container::<Option<String>>::new(None);
    
    let list_item = ListItem{ data: Box::new(12), next: None };
    let my_ekuke = Dog::<Ekuke>{name: String::from("James"), breed: PhantomData};
    
    println!("my ekuke: {:?} {}", my_ekuke, my_ekuke.breed_name());
    
}


#[derive(Clone)]
struct ListItem<T: Clone> {
    data: Box<T>,
    next: Option<Box<ListItem<T>>>
}

#[derive(Debug)]
struct Poodle{}

#[derive(Debug)]
struct Retriever{}

#[derive(Debug)]
struct Ekuke{}


#[derive(Debug)]
struct Dog<Breed> {
    name: String,
    breed: PhantomData<Breed>
}

impl Dog<Ekuke> {
    fn breed_name(&self) -> &str {
        "ekuke"
    }
}