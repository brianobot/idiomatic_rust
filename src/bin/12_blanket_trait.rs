

trait RecognizeBrian<T> {
    fn call_me(&self) {
        println!("Brian!!!");
    }
}


// notice how the trait is implemented for any type T which is a generic
impl<T> RecognizeBrian<T> for T {
    
}

fn main() {
    let name = String::from("Brian");
    let age = 12;
    name.call_me();
    age.call_me();
}