type Digest = [u8; 32];   // type alias: one type, 2 names

enum Result {             // enumerable
    Okay(Digest),
    Error(String),
}

fn generate_digest() -> Result {
    Result::Okay([42u8; 32])
}

fn main() {
    match generate_digest() {
        Result::Okay(d) => {
            for byte in d.iter() {
                print!("{:02X}", byte);
            }
            println!("");
        },
        Result::Error(msg) => eprintln!("error: {}", msg),
    }
}