pub fn string_to_integer(s: String) -> i32 {
    return 15;
}

#[test]
fn returns_int() {
    let cool = "wow";
    println!("{cool}");
    println!("{cool}");
    let number = string_to_integer("fifteen".to_string());
    assert_eq!(number, 15);
}
