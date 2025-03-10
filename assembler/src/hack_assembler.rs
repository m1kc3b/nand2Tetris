use std::fs::OpenOptions;
use std::io::Error;
use std::io::Write;

use crate::{
    parser::{InstructionType, Parser},
    symbol_table::SymbolTable,
};

pub struct HackAssembler {
    parser: Parser,
    symbol_table: SymbolTable,
    output_file: String,
    filename: String,
}

impl HackAssembler {
    pub fn new(filename: &str) -> Result<Self, Error> {
        let parser = Parser::new(filename)?;
        let symbol_table = SymbolTable::new();
        let name: Vec<&str> = filename.split(".").collect();
        let file_name = name[0];
        let output_file = format!("hack-files/{}.hack", file_name);
        Ok(Self {
            parser,
            symbol_table,
            output_file,
            filename: file_name.to_string(),
        })
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        // First pass: TODO create a Lines
        while let Some(Ok(line)) = self.parser.advance() {
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
                }
                _ => continue,
            }
        }

        // Second pass:
        match self.parser.reinitialize_lines(format!("{}.asm", &self.filename).as_str()) {
            Ok(()) => {
                while let Some(Ok(line)) = self.parser.advance() {
                  let mut file = OpenOptions::new().append(true).create(true).open(&self.output_file)?;
                    match self.parser.instruction_type(&line) {
                        Some(InstructionType::AInstruction) => {
                            let symbol = self.parser.symbol(line).unwrap();

                            // symbol == label -> get_address -> binary
                            if let Some(add) = self.symbol_table.get_address(&symbol) {
                                let binary = format!("{:016b}\n", &add);
                                file.write_all(binary.as_bytes())?;
                            }

                            // symbol == num -> binary
                            if let Ok(num) =  symbol.parse::<i32>() {
                                file.write_all(format!("{:016b}", num).as_bytes())?;
                            }
                          }
                        Some(InstructionType::CInstruction) => {
                            // concatenate dest + comp + jump
                            // 111 a cccccc ddd jjj
                            let mut instruction = "111".to_string();

                            if let Some(value) = self.parser.comp(&line) {
                                instruction.push_str(value);
                            }

                            if let Some(value) = self.parser.dest(&line) {
                                instruction.push_str(value);
                            }

                            if let Some(value) = self.parser.jump(&line) {
                                instruction.push_str(value);
                            }
                            instruction.push_str("\n");
                            // insert in output_file
                            let _ = file.write_all(instruction.as_bytes());
                        }
                        _ => continue,
                    }
                }
            }
            Err(err) => eprintln!("Error with reinitialization: {}", err),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

   
}
