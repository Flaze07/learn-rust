fn main() {
    let mut name = String::from("Comprehensive Rust :D");
    while let Some(c) = name.pop() {
        dbg!(c);
    }
}
