use xkb_parser::parse;

#[test]
fn garbage() {
    let file = "lorem ipsum dolor sit amet";
    assert!(parse(&file).is_err());
}
