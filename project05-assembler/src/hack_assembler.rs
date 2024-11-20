use std::fs::File;

use crate::parser::Parser;
use crate::symbol_table::SymbolTable;


pub struct HackAssembler<'a> {
  parser: Parser,
  symbol_table: SymbolTable<'a>,
  pass: usize,
  current_line: usize,
  output_file: File
}

impl<'a> HackAssembler<'a> {
  pub fn new(filename: &str) -> Self {
    let parser = Parser::new(filename);
    let symbol_table = SymbolTable::new();
    let output_file = File::create(filename).unwrap();
    Self { parser, symbol_table, pass: 0, current_line: 0, output_file }
  }

  pub fn execute(&self) {

  }
}

#[cfg(test)]
mod tests {
  
}