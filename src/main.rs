use parser::parse_deadline;

mod data;
mod parser;

fn main() {
    println!("Hello, world!");
    let file = std::fs::read_to_string("test.td").unwrap();
    let mut s = file.as_str();
    let deadline = match parse_deadline(&mut s) {
        Ok(x) => x,
        Err(e) => panic!("{}", e),
    };
    println!("{deadline:?}")
}
