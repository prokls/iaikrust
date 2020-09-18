#[derive(Debug)]
struct HashAlgo {
    security_margin: u32,
    names: Vec<String>,
    state: [u8; 9],
}

fn main() {
    let h = HashAlgo{ security_margin: 32, names: vec![], state: [0u8; 9] };
    println!("{:?}", h);
}