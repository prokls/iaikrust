struct HashAlgorithm {
  state: [u8; 32],
  security_margin: u32,
  names: Vec<String>,
}

fn main() {
  let h = HashAlgorithm{
    state: [0u8; 32],
    security_margin: 128,
    names: vec!["SHA-2".to_string(),
                "SHA-256".to_string()],
  };
  println!("aliases → {}", h.names.join(", "))
}