use anyhow::Result;
use std::fs;

use crate::{expansions::do_expansions, parser::parse};

mod expansions;
mod parser;

fn main() -> Result<()> {
    let file = fs::read_to_string("test_file.sh")?;
    let tokens = parse(file)?;
    println!("{:#?}", &tokens);

    do_expansions(tokens);

    Ok(())
}
