
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
    let bicycle = BicycleBuilder::new()
        .with_color("Black")
        .with_model("Aero flame")
        .with_make("2025Q1")
        .with_size(10)
        .build();
    
    dbg!(bicycle);
}