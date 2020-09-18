fn main() {
    let mut vec = vec![];
    vec.push(5);
    vec.extend(vec![3, 4]);
    vec[0] = 7;

    assert_eq!(vec[0], 7);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.pop(), Some(4));

    vec.sort();
    vec.sort_unstable();

    let elements: &[u8] = &vec[0..2];
    println!("{:?}", elements);
}