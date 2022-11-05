fn main() {
    let a = [1, 2, 3, 4];

    let s = &a[0..2];

    assert_eq!(s, &[1, 3]);
}
