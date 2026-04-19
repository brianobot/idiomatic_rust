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
    // when working with generic objects in rust, the turbo fish way of hinting the concrete type must be applied
    // directly on the object that the generic is defined on, 
    // 
    // take for example i have a generic struct as follow
    struct Car<Brand> {
       brand: Brand,
       year: u16,
    }
    
    impl<Brand> Car<Brand> {
       fn new(brand: Brand, year: u16) -> Self {
         Self {
             brand, year
         }
       }
    }
    
    let benz = Car::new("Masda", 2023); // this is valid
    let omo = Car::<String>::new(String::from("Omo"), 2000); // notice how the turbo fish synxtax is on the struct nd not the method
    
    // if a trait method does not take reference to the instance it being implented on, then the trait acts on the type itself
    trait Talkable {
        fn talk() -> String {
            String::from("Talking")
        }
    }
    
    impl Talkable for String {
        
    }
    
    let simple_string = String::talk();
    println!("{simple_string}");
    
    struct Struct1 {}
    struct Struct2 {}
    
    trait MyTrait {
        fn hello(&self) {
            println!("Hello!");
        }
    }
    
    impl MyTrait for Struct1{}
    impl MyTrait for Struct2{}
    
    let mut v = Vec::<Box<dyn MyTrait>>::new();
    v.push(Box::new(Struct1{}));
    v.push(Box::new(Struct2{}));
    
    v.iter().for_each(|i| i.hello());
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