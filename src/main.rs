use std::error::Error;
use std::fs;
use crate::json::*;

mod json;

fn main() -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string("./src/test.json")?;
    let tokens = lex(contents);
    let mut iterator = tokens.iter().peekable();
    let json = parse(&mut iterator);
    println!("{:?}", json);
    Ok(())
}