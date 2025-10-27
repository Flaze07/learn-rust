use std::cmp::Ordering;

fn min<T: Ord>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

#[test]
fn integers() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);
}

#[test]
fn strings() {
    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}

fn main() {
    // println!("Hello new main")
}
