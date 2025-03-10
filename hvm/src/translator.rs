use std::fs::File;
use std::io::Write;
use crate::parser::parse_file;

pub fn translate(input: &str, output: &str) -> std::io::Result<()> {
  let instructions = parse_file(input)?;
  
  Ok(())
}