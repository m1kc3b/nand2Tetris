use std::fs::File;
use std::io;

use crate::{parser::{self, InstructionType, Parser}, symbol_table::SymbolTable};

pub struct HackAssembler {
    parser: Parser,
    symbol_table: SymbolTable,
    pass: usize,
    current_line: usize,
    output_file: File,
}

impl HackAssembler {
    pub fn new(filename: &str) -> Result<Self, io::Error> {
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
        while let Some(Ok(line)) = self.parser.advance() {
          // get label instruction
          if let Some(InstructionType::LInstruction) = self.parser.instruction_type(&line) {
            // insert into symbol_table
            let symbol = self.parser.symbol(line).unwrap();
            self.symbol_table.add_entry(symbol, self.parser.get_line_count().unwrap() +1);
          }
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
    use crate::hack_assembler;

    use super::HackAssembler;


    #[test]
    fn symbol_table_should_contains_loop_and_stop() {
      let mut hack_assembler = HackAssembler::new("Sum1ToN.asm").unwrap();
      hack_assembler.execute();
      assert_eq!(hack_assembler.symbol_table.contains("LOOP"), true);
      assert_eq!(hack_assembler.symbol_table.contains("STOP"), true);
    }

    #[test]
    fn symbol_table_should_contains_loop_with_address_6() {
      let mut hack_assembler = HackAssembler::new("Sum1ToN.asm").unwrap();
      hack_assembler.execute();
      assert_eq!(hack_assembler.symbol_table.get_address("LOOP"), Some(6));
    }

    #[test]
    fn symbol_table_should_contains_stop_with_address_21() {
      let mut hack_assembler = HackAssembler::new("Sum1ToN.asm").unwrap();
      hack_assembler.execute();
      assert_eq!(hack_assembler.symbol_table.get_address("STOP"), Some(21));
    }
}
