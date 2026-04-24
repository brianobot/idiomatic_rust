

trait ReversedExt<T> {
    fn reversed(&self) -> Vec<T>;
}


impl<T: Clone> ReversedExt<T> for Vec<T> {
    fn reversed(&self) -> Vec<T> {
        self.iter().rev().cloned().collect()
    }
}

fn main() {
    let vector = vec![1, 2, 3, 4, 5];
    let rev = vector.reversed();
    println!("{rev:?}");
}