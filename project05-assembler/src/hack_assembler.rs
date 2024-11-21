use std::fs::File;
use std::io;

use crate::parser::Parser;
use crate::symbol_table::SymbolTable;

pub struct HackAssembler<'a> {
    parser: Parser,
    symbol_table: SymbolTable<'a>,
    pass: usize,
    current_line: usize,
    output_file: File,
}

impl<'a> HackAssembler<'a> {
    pub fn new(filename: &'a str) -> Result<Self, io::Error> {
        let parser = Parser::new(filename)?;
        let symbol_table = SymbolTable::new();
        let output_file = File::create(filename)?;
        Ok(Self {
            parser,
            symbol_table,
            pass: 0,
            current_line: 0,
            output_file,
        })
    }

    pub fn execute(&mut self) {
        // First pass:
        // read each line and keeping track of line number for A_instruction and C_instruction only.
        // add label to symbol table (symbol name, line number)
        // LOOP
        // get the line
        // check instruction_type
        // add label to symbol_table
        while let Some(line) = self.parser.advance() {
          println!("{:?}", line.unwrap())
        }

        // Second pass:
        // read each line and parse each one.
        // LOOP
        // get the line
        // check instruction_type
        // translate it
        // insert into output_file
    }
}

#[cfg(test)]
mod tests {
    use super::HackAssembler;


    #[test]
    fn should_return_each_line() {
      let mut hack_assembler = HackAssembler::new("Add.asm").unwrap();
      hack_assembler.execute();
    }
}
