#![feature(test)]


#[cfg(test)]
mod test {
    extern crate test;
    use test::Bencher;
    
    #[bench]
    fn bench_mow(b: &mut Bencher) {
        b.iter(|| {
            println!("Hello World!");
        });
    }
}
