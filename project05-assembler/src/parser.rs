use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader, Lines, Result},
};

#[derive(Debug, PartialEq, Eq)]
enum InstructionType {
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
                        if trimmed.starts_with("(") {
                            self.line_count += 1;
                        }
                        return Some(Ok(trimmed.to_string()));
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
        None
    }

    
}

fn instruction_type(line: &str) -> Option<InstructionType> {
    if line.starts_with("@") {
        return Some(InstructionType::AInstruction);
    } else if line.starts_with("(") {
        return Some(InstructionType::LInstruction);
    } else {
        return Some(InstructionType::CInstruction);
    }
    None
}

fn symbol(line: &str) -> Option<&str> {
    let instruction_type = instruction_type(line);
    match instruction_type {
        Some(InstructionType::AInstruction) => Some(&line[1..]),
        Some(InstructionType::LInstruction) => Some(&line[1..line.len() - 1]),
        _ => None,
    }
}

fn dest(line: &str) -> Option<&str> {
    let instruction_type = instruction_type(line);
    if let Some(InstructionType::CInstruction) = instruction_type {
        let instruction = &line[..1];
        match instruction {
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
    None
}

fn comp(line: &str) -> Option<&str> {
    let instruction_type = instruction_type(line);
    if let Some(InstructionType::CInstruction) = instruction_type {
        let instruction = &line[2..];
        match instruction {
            "0" => return Some("101010"),
            "1" => return Some("111111"),
            "-1" => return Some("111010"),
            "D" => return Some("001100"),
            "A" | "M" => return Some("110000"),
            "!D" => return Some("001101"),
            "!A" | "!M" => return Some("110001"),
            "-D" => return Some("001111"),
            "-A" | "-M" => return Some("110011"),
            "D+1" => return Some("011111"),
            "A+1" | "M+1" => return Some("110111"),
            "D-1" => return Some("001110"),
            "A-1" | "M-1" => return Some("110010"),
            "D+A" | "D+M" => return Some("000010"),
            "D-A" | "D-M" => return Some("010011"),
            "A-D" | "M-D" => return Some("000111"),
            "D&A" | "D&M" => return Some("000000"),
            "D|A" | "D|M" => return Some("010101"),
            _ => return None,
        }
    }
    None
}

fn jump(line: &str) -> Option<&str> {
    let instruction_type = instruction_type(line);
    if let Some(InstructionType::CInstruction) = instruction_type {
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
    None
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
            assert_eq!(symbol(&text), Some("2"))
        }
    }

    #[test]
    fn call_symbol_should_return_none_if_line_is_an_c_instruction() {
        let mut parser = Parser::new("Add.asm").unwrap();
        parser.advance();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(symbol(&text), None)
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
            assert_eq!(symbol(&text), Some("LOOP"))
        }
    }

    #[test]
    fn call_dest_should_return_none_if_line_is_an_a_instruction() {
        let mut parser = Parser::new("Add.asm").unwrap();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(dest(&text), None)
        }
    }

    #[test]
    fn call_dest_should_return_010_if_line_is_an_c_instruction() {
        let mut parser = Parser::new("Add.asm").unwrap();
        parser.advance();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(dest(&text), Some("010")) // "D"
        }
    }

    #[test]
    fn call_comp_should_return_none_if_line_is_an_a_instruction() {
        let mut parser = Parser::new("Add.asm").unwrap();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(comp(&text), None)
        }
    }

    #[test]
    fn call_comp_should_return_110000_if_line_is_an_c_instruction() {
        let mut parser = Parser::new("Add.asm").unwrap();
        parser.advance();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(comp(&text), Some("110000")) // "A"
        }
    }

    #[test]
    fn call_jump_should_return_none_if_line_is_an_a_instruction() {
        let mut parser = Parser::new("Add.asm").unwrap();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(jump(&text), None)
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
            assert_eq!(jump(&text), Some("001")) // "JGT"
        }
    }
}
