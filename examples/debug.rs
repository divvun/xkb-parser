use xkb_parser::parse;

fn main() {
    let file_name = std::env::args().nth(1).expect("USAGE: debug <FILE>");
    let file = std::fs::read_to_string(&file_name).expect("read input file");
    let x = parse(&file).expect("parse");
    println!("{:#?}", x);
}
