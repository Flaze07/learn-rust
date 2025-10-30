fn main() {
    let name = "我们不一样";
    let mut position: Option<usize> = name.find("我");
    dbg!(position);
    assert_eq!(position.unwrap(), 0);
    position = name.find('Z');
    dbg!(position);
    assert_eq!(position.is_none(), true);
}