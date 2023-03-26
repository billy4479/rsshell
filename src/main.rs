use anyhow::Result;
use std::fs;

use crate::parser::parse;

mod lexer;
mod parser;

fn main() -> Result<()> {
    let file = fs::read_to_string("test_file.sh")?;
    let tokens = parse(file)?;

    println!("{tokens:#?}");
    Ok(())
}
