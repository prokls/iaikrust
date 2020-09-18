fn create_tuple() -> (u32, u64) {
    (4, 2)
}

fn main() {
    let (a, _b) = (4, 2);

    // comparison by equality
    assert_eq!((4, 2), create_tuple());

    let pair = create_tuple();
    // access by tuple.{zero-based index}
    assert_eq!(a, pair.0);
}
