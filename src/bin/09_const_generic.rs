
#[derive(Debug)]
struct Buffer<T: Default + Copy, const LENGTH: usize> {
    buf: [T; LENGTH]
}


impl<T: Default + Copy, const LENGTH: usize> Buffer<T, LENGTH> {
    fn new() -> Self {
        Self {
            buf: [T::default(); LENGTH]
        }
    }
}

impl<T: Default + Copy, const LENGTH: usize> From<[T; LENGTH]> for Buffer<T, LENGTH> {
    fn from(buf: [T; LENGTH]) -> Self {
        Buffer { buf }
    }
}

fn main() {
    let buff = Buffer::from([1, 2, 3, 4]);
    let buffer = Buffer{ buf: [0.0; 256]};
    let buf = Buffer::<i32, 10>::new();
    
    
    println!("{buff:?}");
    println!("{buf:?}");
    println!("{buffer:?}");
    
}