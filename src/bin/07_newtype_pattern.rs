

#[derive(Debug)]
struct BitCount(u32);


impl BitCount {
    fn to_bytes(&self) -> ByteCount {
        ByteCount( self.0 / 8)
    }
}

#[derive(Debug)]
struct ByteCount(u32);

impl ByteCount {
    fn to_bit(&self) -> BitCount {
        BitCount( self.0 * 8)
    }
}

fn main() {
    let bit_count = BitCount(8);
    let byte_count = ByteCount(1);
    
    println!("8 Bit = {:?}", bit_count.to_bytes().to_bit());
    println!("1 Byte = {:?}", byte_count.to_bit().to_bytes());

}