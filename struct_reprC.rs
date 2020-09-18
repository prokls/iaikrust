use std::mem::{size_of, align_of};

#[repr(C)]
struct HashAlgo {
    security_margin: u32, //  4 bytes
    names: Vec<String>,   // 24 bytes
    state: [u8; 9],       //  9 bytes
}

fn main() {
    assert_eq!(size_of::<HashAlgo>(), 48);
    assert_eq!(align_of::<HashAlgo>(), 8);
}