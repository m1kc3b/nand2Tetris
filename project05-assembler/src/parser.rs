use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
};

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionType {
    AInstruction,
    CInstruction,
    LInstruction,
}

#[derive(Debug)]
pub struct Parser {
    lines: Lines<BufReader<File>>,
    line_count: usize,
}

impl Parser {
    pub fn new(filename: &str) -> Result<Self> {
        let path = format!("asm-files/{}", &filename);
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(Self {
            lines: reader.lines(),
            line_count: 0,
        })
    }

    pub fn get_line_count(&self) -> Option<usize> {
        Some(self.line_count)
    }

    // pub fn has_more_lines(&self) -> bool {
    //     let lines: Vec<&str> = self.input.split("\n").collect();
    //     lines.len() > self.index
    // }

    pub fn advance(&mut self) -> Option<Result<String>> {
        while let Some(line) = self.lines.next() {
            match line {
                Ok(content) => {
                    let trimmed = content.trim();
                    if !trimmed.is_empty() & !trimmed.starts_with("//") {
                        self.line_count += 1;
                        // if trimmed.starts_with("(") {
                        //     self.line_count += 1;
                        // }
                        return Some(Ok(content.trim().to_string()));
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
        None
    }

    pub fn reinitialize_lines(&mut self, filename: &str) -> Result<()> {
        let path = format!("asm-files/{}", &filename);
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        self.lines = reader.lines();
        self.line_count = 0;

        Ok(())
    }

    
    pub fn instruction_type(&self, line: &str) -> Option<InstructionType> {
        if line.starts_with("@") && is_not_uppercase(line) {
            return Some(InstructionType::AInstruction);
        } else if line.starts_with("(") {
            return Some(InstructionType::LInstruction);
        } else {
            return Some(InstructionType::CInstruction);
        }
    }
    
    pub fn symbol(&self, line: String) -> Option<String> {
        let instruction_type = self.instruction_type(&line);
        match instruction_type {
            Some(InstructionType::AInstruction) => Some(line[1..].to_string()),
            Some(InstructionType::LInstruction) => Some(line[1..line.len() - 1].to_string()),
            _ => None,
        }
    }
    
    pub fn dest(&self, line: &str) -> Option<&str> {
        let instruction_type = self.instruction_type(&line);
        if let Some(InstructionType::CInstruction) = instruction_type {
            // check if "="
            if line.contains("=") {
                let instruction: Vec<&str> = line.split("=").collect();
                match instruction[0] {
                    "M" => return Some("001"),
                    "D" => return Some("010"),
                    "DM" => return Some("011"),
                    "A" => return Some("100"),
                    "AM" => return Some("101"),
                    "AD" => return Some("110"),
                    "ADM" => return Some("111"),
                    _ => return Some("000"),
                }
            }
            return Some("000");
        }
        None
    }
    
    pub fn comp(&self, line: &str) -> Option<&str> {
        let instruction_type = self.instruction_type(&line);
        if let Some(InstructionType::CInstruction) = instruction_type {
            if line.contains("=") {
                let instruction: Vec<&str> = line.split("=").collect();
                match instruction[1] {
                    "0" => return Some("0101010"),
                    "1" => return Some("0111111"),
                    "-1" => return Some("0111010"),
                    "D" => return Some("0001100"),
                    "A" => return Some("0110000"),
                    "M" => return Some("1110000"),
                    "!D" => return Some("0001101"),
                    "!A" => return Some("0110001"),
                    "!M" => return Some("1110001"),
                    "-D" => return Some("0001111"),
                    "-A" => return Some("0110011"),
                    "-M" => return Some("1110011"),
                    "D+1" => return Some("0011111"),
                    "A+1" => return Some("0110111"),
                    "M+1" => return Some("1110111"),
                    "D-1" => return Some("0001110"),
                    "A-1" => return Some("0110010"),
                    "M-1" => return Some("1110010"),
                    "D+A" => return Some("0000010"),
                    "D+M" => return Some("1000010"),
                    "D-A" => return Some("0010011"),
                    "D-M" => return Some("1010011"),
                    "A-D" => return Some("0000111"),
                    "M-D" => return Some("1000111"),
                    "D&A" => return Some("0000000"),
                    "D&M" => return Some("1000000"),
                    "D|A" => return Some("0010101"),
                    "D|M" => return Some("1010101"),
                    _ => return None,
                }
            }
            if line.contains(";") {
                let instruction: Vec<&str> = line.split("=").collect();
                match instruction[0] {
                    "0" => return Some("0101010"),
                    "1" => return Some("0111111"),
                    "-1" => return Some("0111010"),
                    "D" => return Some("0001100"),
                    "A" => return Some("0110000"),
                    "M" => return Some("1110000"),
                    "!D" => return Some("0001101"),
                    "!A" => return Some("0110001"),
                    "!M" => return Some("1110001"),
                    "-D" => return Some("0001111"),
                    "-A" => return Some("0110011"),
                    "-M" => return Some("1110011"),
                    "D+1" => return Some("0011111"),
                    "A+1" => return Some("0110111"),
                    "M+1" => return Some("1110111"),
                    "D-1" => return Some("0001110"),
                    "A-1" => return Some("0110010"),
                    "M-1" => return Some("1110010"),
                    "D+A" => return Some("0000010"),
                    "D+M" => return Some("1000010"),
                    "D-A" => return Some("0010011"),
                    "D-M" => return Some("1010011"),
                    "A-D" => return Some("0000111"),
                    "M-D" => return Some("1000111"),
                    "D&A" => return Some("0000000"),
                    "D&M" => return Some("1000000"),
                    "D|A" => return Some("0010101"),
                    "D|M" => return Some("1010101"),
                    _ => return None,
                }
            }
        }
        None
    }
    
    pub fn jump(&self, line: &str) -> Option<&str> {
        let instruction_type = self.instruction_type(&line);
        if let Some(InstructionType::CInstruction) = instruction_type {
            // check if contains ";"
            if line.contains(";") {
                let instruction: Vec<&str> = line.split(";").collect();
                match instruction[1] {
                    "JGT" => return Some("001"),
                    "JEQ" => return Some("010"),
                    "JGE" => return Some("011"),
                    "JLT" => return Some("100"),
                    "JNE" => return Some("101"),
                    "JLE" => return Some("110"),
                    "JMP" => return Some("111"),
                    _ => return Some("000"),
                }
            }
            return Some("000")
        }
        None
    }
}

fn is_not_uppercase(s: &str) -> bool {
    s.chars().any(|c| c.is_lowercase())
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_if_whitespace_and_comments_are_skipped_and_line_count_is_incremented() {
        let mut parser = Parser::new("Add.asm").unwrap();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(text, "@2".to_string());
            assert_eq!(parser.line_count, 1);
        }
    }

    #[test]
    fn check_it_works_when_advance_is_called_twice() {
        let mut parser = Parser::new("Add.asm").unwrap();
        parser.advance();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(text, "D=A".to_string());
            assert_eq!(parser.line_count, 2);
        }
    }

    #[test]
    fn check_it_works_when_advance_is_called_eleventh() {
        let mut parser = Parser::new("Sum1ToN.asm").unwrap();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(text, "D;JGT".to_string());
        }
    }

    #[test]
    fn check_it_works_for_label() {
        let mut parser = Parser::new("Sum1ToN.asm").unwrap();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(text, "(LOOP)".to_string());
            assert_eq!(parser.line_count, 6);
        }
    }

    #[test]
    fn call_symbol_should_return_some_2_if_line_is_an_a_instruction() {
        let mut parser = Parser::new("Add.asm").unwrap();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(parser.symbol(text), Some("2".to_string()))
        }
    }

    #[test]
    fn call_symbol_should_return_none_if_line_is_an_c_instruction() {
        let mut parser = Parser::new("Add.asm").unwrap();
        parser.advance();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(parser.symbol(text), None)
        }
    }

    #[test]
    fn call_symbol_should_return_loop_if_line_is_an_l_instruction() {
        let mut parser = Parser::new("Sum1ToN.asm").unwrap();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(parser.symbol(text), Some("LOOP".to_string()))
        }
    }

    #[test]
    fn call_dest_should_return_none_if_line_is_an_a_instruction() {
        let mut parser = Parser::new("Add.asm").unwrap();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(parser.dest(&text), None)
        }
    }

    #[test]
    fn call_dest_should_return_010_if_line_is_an_c_instruction() {
        let mut parser = Parser::new("Add.asm").unwrap();
        parser.advance();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(parser.dest(&text), Some("010")) // "D"
        }
    }

    #[test]
    fn call_comp_should_return_none_if_line_is_an_a_instruction() {
        let mut parser = Parser::new("Add.asm").unwrap();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(parser.comp(&text), None)
        }
    }

    #[test]
    fn call_comp_should_return_110000_if_line_is_an_c_instruction() {
        let mut parser = Parser::new("Add.asm").unwrap();
        parser.advance();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(parser.comp(&text), Some("110000")) // "A"
        }
    }

    #[test]
    fn call_jump_should_return_none_if_line_is_an_a_instruction() {
        let mut parser = Parser::new("Add.asm").unwrap();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(parser.jump(&text), None)
        }
    }

    #[test]
    fn call_jump_should_return_none_if_line_is_an_c_instruction() {
        let mut parser = Parser::new("Sum1ToN.asm").unwrap();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(parser.jump(&text), Some("001")) // "JGT"
        }
    }
}
