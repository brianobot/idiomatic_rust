
macro_rules! noop_macro {
    () => { println!("No Argument" )};
    ($e:expr) => { println!("Matched a single expression") };
    ($s:stmt) => { println!("Matched a single statement") };
    ($e:expr, $s:stmt) => { println!("Matched a single expression followed by a statement") };

}

macro_rules! var_print {
    ($($v:ident),*) => {
        println!(
            concat!(
                $(stringify!($v), "={:?} "), *
            ),
            $($v), *
        );
    };
}


macro_rules! generate {
    ($name:ident) => {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct $name {
            name: String,
            race: String,
            level: u8,
        }
        
        impl $name {
            fn new(name: String, race: String) -> Self {
                Self {
                    name,
                    race,
                    level: 0
                }
            }
        }
    };
}

fn main() {
    noop_macro!();
    noop_macro!(1 + 1);
    noop_macro!(let name = "Brian");
    noop_macro!({});
    noop_macro!(;);
    noop_macro!(1+1,;);
    
    let name = String::from("Brian");
    let age = 10;
    var_print!(name, age);
    
    generate!(Person);
    
    let person = Person::new(String::from("Brian"), String::from("Black"));
    println!("{person:?}");
    
}