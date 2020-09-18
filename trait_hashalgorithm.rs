trait HashAlgorithm256 {
    const OUTPUT_SIZE: u32 = 256;
    fn hash(&self, content: &[u8]) -> [u8; 32];
}

struct Xor { init: [u8; 32] }

impl Xor {
    fn new() -> Xor {
        Xor { init: [0u8; 32] }
    }
    
    fn name(&self) -> &'static str { "xor" }
}

impl HashAlgorithm256 for Xor {
    fn hash(&self, input: &[u8]) -> [u8; 32] {
        let mut digest = self.init;
        for (i, byte) in input.iter().enumerate() {
            digest[i % 32] ^= byte;
        }
        digest
    }
}

fn main() {
    let op = Xor::new();
    let digest = op.hash(&[42, 11, 13]);
    println!("{} gives {:?}", op.name(), digest);
}
