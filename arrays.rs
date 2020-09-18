fn main() {
    let all_zero = [0u8; 32];  // type: [u8; 32]
    let mut init = [9, 2, 3];  // type: [{integer}; 3]
    let initial  = [1u8, 2, 3];// type: [u8; 3]

    init[0] = 1;
    //init[4] = 1;  // compile or runtime error
    assert_eq!(initial, init);
    assert_eq!(initial, initial.clone());

    let first_5: &[u8] = &all_zero[0..5];
    let f1rst_5: &[u8] = &all_zero[ ..5];
    let first_6: &[u8] = &all_zero[0..=5];

    println!("{:?}", first_5);
    println!("{:?}", f1rst_5);
    println!("{:?}", first_6);
}
