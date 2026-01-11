use std::{collections::HashMap, hash::Hasher};
fn main() {
    let mut map = HashMap::with_hasher(FastHasherBuilder);
    map.insert(1, 2);
    map.insert(2, 2);
    println!("{:?}", map);
}
struct FastHasherBuilder;
struct FastHasher(u64);

impl std::hash::BuildHasher for FastHasherBuilder {
    type Hasher = FastHasher;

    fn build_hasher(&self) -> Self::Hasher {
        FastHasher(0)
    }
}
impl Hasher for FastHasher {
    fn finish(&self) -> u64 {
        u64::from(self.0)
    }
    /// 并不是一个好hash，只是为了演示
    fn write(&mut self, bytes: &[u8]) {
        let (chunks, remainder) = bytes.as_chunks::<8>();
        let mut last = [1u8; 8];
        last[..remainder.len()].copy_from_slice(remainder);
        for chunk in chunks {
            let mixed = self.0 as u128 * (u64::from_ne_bytes(*chunk) as u128);
            self.0 = (mixed >> 64) as u64 ^ mixed as u64;
        }
    }
}
