use std::ops::{Deref, DerefMut};


#[derive(Debug)]
struct WrappedVec<T>(Vec<T>);


impl<T> WrappedVec<T> {
    fn new() -> Self {
        Self(vec![])
    }
}

impl<T> Deref for WrappedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl<T> DerefMut for WrappedVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut wrapped_vec = WrappedVec::<i32>::new();
    
    wrapped_vec.push(120);
    
    println!("{wrapped_vec:?}");
}