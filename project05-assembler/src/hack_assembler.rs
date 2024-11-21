use std::{fs::File, io::Write};
use std::io::{self, Error};

use crate::{
    parser::{InstructionType, Parser},
    symbol_table::SymbolTable,
};

pub struct HackAssembler {
    parser: Parser,
    symbol_table: SymbolTable,
    output_file: File,
}

impl HackAssembler {
    pub fn new(filename: &str) -> Result<Self, io::Error> {
        let parser = Parser::new(filename)?;
        let symbol_table = SymbolTable::new();
        let name: Vec<&str> = filename.split(".").collect();
        let output_file = File::create(format!("hack-files/{}.hack", name[0]))?;
        Ok(Self {
            parser,
            symbol_table,
            output_file,
        })
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        // First pass:
        while let Some(Ok(line)) = self.parser.advance() {
          println!("ENTER INTO 1ST PASS");
            match self.parser.instruction_type(&line) {
                Some(InstructionType::LInstruction) => {
                    let symbol = self.parser.symbol(line).unwrap();
                    self.symbol_table
                        .add_entry(symbol, self.parser.get_line_count().unwrap() + 1);
                }
                Some(InstructionType::AInstruction) => {
                  let symbol = self.parser.symbol(line).unwrap();
                  self.symbol_table
                        .update_entry(symbol, self.parser.get_line_count().unwrap());
                },
                _ => continue,
            }
        }

        // Second pass: TODO -> re-init Lines !!!
        while let Some(Ok(line)) = self.parser.advance() {
          println!("ENTER INTO 2ND PASS");
            match self.parser.instruction_type(&line) {
                Some(InstructionType::AInstruction) => {
                  let symbol = self.parser.symbol(line).unwrap();
                  // symbol == label -> get_address -> binary
                  if let Some(add) = self.symbol_table.get_address(&symbol) {
                    let binary = format!("{:016b}", &add);
                    write!(self.output_file, "{}", binary)?;
                  }
                  // symbol == num -> binary
                  let parsed: usize = symbol.parse().unwrap();
                  let binary = format!("{:016b}", parsed);
                  write!(self.output_file, "{}", binary)?;
                  // insert in output_file
                },
                Some(InstructionType::CInstruction) => {
                  // concatenate dest + comp + jump
                  // insert in output_file
                },
                _ => continue,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn symbol_table_should_contains_loop_stop_i_sum() {
        let mut hack_assembler = HackAssembler::new("Sum1ToN.asm").unwrap();
        let _ = hack_assembler.execute();
        assert_eq!(hack_assembler.symbol_table.contains("LOOP"), true);
        assert_eq!(hack_assembler.symbol_table.contains("STOP"), true);
        assert_eq!(hack_assembler.symbol_table.contains("i"), true);
        assert_eq!(hack_assembler.symbol_table.contains("sum"), true);
    }

    #[test]
    fn symbol_table_should_contains_loop_with_address_6() {
        let mut hack_assembler = HackAssembler::new("Sum1ToN.asm").unwrap();
        let _ = hack_assembler.execute();
        assert_eq!(hack_assembler.symbol_table.get_address("LOOP"), Some(6));
    }

    #[test]
    fn symbol_table_should_contains_stop_with_address_21() {
        let mut hack_assembler = HackAssembler::new("Sum1ToN.asm").unwrap();
        let _ = hack_assembler.execute();
        assert_eq!(hack_assembler.symbol_table.get_address("STOP"), Some(21));
    }

    #[test]
    fn symbol_table_should_contains_i_with_address_16() {
        let mut hack_assembler = HackAssembler::new("Sum1ToN.asm").unwrap();
        let _ = hack_assembler.execute();
        assert_eq!(hack_assembler.symbol_table.get_address("i"), Some(16));
    }

    #[test]
    fn symbol_table_should_contains_sum_with_address_17() {
        let mut hack_assembler = HackAssembler::new("Sum1ToN.asm").unwrap();
        let _ = hack_assembler.execute();
        assert_eq!(hack_assembler.symbol_table.get_address("sum"), Some(21));
    }

    // #[test]
    // fn show_symbol_table() {
    //   let mut hack_assembler = HackAssembler::new("Sum1ToN.asm").unwrap();
    //     let _ = hack_assembler.execute();
    //     println!("{:#?}", hack_assembler.symbol_table)
    // }

    #[test]
    fn output_file_should_exists() {
      let mut hack_assembler = HackAssembler::new("Sum1ToN.asm").unwrap();
      let _ = hack_assembler.execute();
      assert_eq!(Path::new("hack-files/Sum1ToN.hack").exists(), true);
    }

    // #[test]
    // fn output_file_should_contains_a_instructions_only() {
    //   let mut hack_assembler = HackAssembler::new("Sum1ToN.asm").unwrap();
    //   let _ = hack_assembler.execute();
    //   assert_eq!(
    //   read_to_string("Sum1ToN.hack").unwrap(),
    //   "0000000000000001\n0000000000000110\n0000000000001100\n0000000000001110\n0000000000010000\n0000000000010101");
    // }
}
