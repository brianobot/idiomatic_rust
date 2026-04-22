
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


#[derive(Default, Debug)]
struct Bicycle {
    make: String,
    model: String,
    size: i32,
    color: String,
}

struct BicycleBuilder {
    bicycle: Bicycle 
}


macro_rules! with {
    ($name:ident, $func:ident, $type:ty) => {
        fn $func(mut self, $name: $type) -> Self {
            self.bicycle.$name = $name.into();
            self
        }
    };
}

impl BicycleBuilder {
    fn new() -> Self {
        Self {
            bicycle: Default::default()
        }
    }
    
    with!(make, with_make, &str);
    with!(model, with_model, &str);
    with!(color, with_color, &str);
    with!(size, with_size, i32);
    
    fn build(self) -> Bicycle {
        self.bicycle
    }
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
    
    let bicycle = BicycleBuilder::new()
        .with_color("Black")
        .with_model("Aero flame")
        .with_make("2025Q1")
        .with_size(10)
        .build();
    
    dbg!(bicycle);
    
}