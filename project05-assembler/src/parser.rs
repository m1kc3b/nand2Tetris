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
    // pub input: String,
    // pub index: usize,
    lines: Lines<BufReader<File>>,
    line_count: usize,
}

impl Parser {
    pub fn new(filename: &str) -> Result<Self> {
        // TODO: return Result<Self, Error>
        // let path = format!("asm-files/{}", &filename);
        // if let Ok(text) = read_to_string(path) {
        // return Self { input: text, index: 0 };
        // }
        // Self { input: String::new(), index: 0 }

        // INIT WITH LINES
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
        // let lines: Vec<&str> = self.input.split("\n").collect();
        // if self.has_more_lines() {
        //     self.index += 1;
        //     // skip comments, whitespaces and empty lines
        //     if lines[self.index].trim().starts_with("//") | lines[self.index].is_empty() {
        //         self.index += 1;
        //     }
        // }

        while let Some(line) = self.lines.next() {
            match line {
                Ok(content) => {
                    let trimmed = content.trim();
                    if !trimmed.is_empty() & !trimmed.starts_with("//") {
                        self.line_count += 1;
                        return Some(Ok(content));
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
        None
    }

    fn symbol(line: &str) -> Option<&str> {
        // let lines: Vec<&str> = self.input.split("\n").collect();
        // let current_instruction = lines[self.index].trim();

        // match self.instruction_type() {
        //     InstructionType::AInstruction => {
        //         Some(&current_instruction[1..])
        //     }
        //     InstructionType::LInstruction => {
        //         Some(&current_instruction[1..current_instruction.len() -1])
        //     }
        //     _ => None,
        // }

        let instruction_type = instruction_type(line);
        match instruction_type {
            Some(InstructionType::AInstruction) => Some(&line[1..]),
            Some(InstructionType::LInstruction) => Some(&line[1..line.len() - 1]),
            _ => None,
        }
    }

    fn dest(line: &str) -> Option<&str> {
        // let lines: Vec<&str> = self.input.split("\n").collect();
        // let current_instruction = lines[self.index].trim();

        // if let InstructionType::CInstruction = self.instruction_type() {
        //     let instruction = &current_instruction[..1];

        //     match instruction {
        //         "M" => return Some("001"),
        //         "D" => return Some("010"),
        //         "DM" => return Some("011"),
        //         "A" => return Some("100"),
        //         "AM" => return Some("101"),
        //         "AD" => return Some("110"),
        //         "ADM" => return Some("111"),
        //         _ => return Some("000"),
        //     }
        // }
        // None

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
        // let lines: Vec<&str> = self.input.split("\n").collect();
        // let current_instruction = lines[self.index].trim();

        // if let InstructionType::CInstruction = self.instruction_type() {
        //     let instruction = &current_instruction[2..];

        //     match instruction {
        //         "0" => return Some("101010"),
        //         "1" => return Some("111111"),
        //         "-1" => return Some("111010"),
        //         "D" => return Some("001100"),
        //         "A"|"M" => return Some("110000"),
        //         "!D" => return Some("001101"),
        //         "!A"|"!M" => return Some("110001"),
        //         "-D" => return Some("001111"),
        //         "-A"|"-M" => return Some("110011"),
        //         "D+1" => return Some("011111"),
        //         "A+1"|"M+1" => return Some("110111"),
        //         "D-1" => return Some("001110"),
        //         "A-1"|"M-1" => return Some("110010"),
        //         "D+A"|"D+M" => return Some("000010"),
        //         "D-A"|"D-M" => return Some("010011"),
        //         "A-D"|"M-D" => return Some("000111"),
        //         "D&A"|"D&M" => return Some("000000"),
        //         "D|A"|"D|M" => return Some("010101"),
        //         _ => return None
        //     }
        // }
        // None
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
        //     // let lines: Vec<&str> = self.input.split("\n").collect();
        //     // let current_instruction = lines[self.index].trim();

        //     if let InstructionType::CInstruction = self.instruction_type() {
        //         let instruction = &current_instruction[2..];

        //         match instruction {
        //             "JGT" => return Some("001"),
        //             "JEQ" => return Some("010"),
        //             "JGE" => return Some("011"),
        //             "JLT" => return Some("100"),
        //             "JNE" => return Some("101"),
        //             "JLE" => return Some("110"),
        //             "JMP" => return Some("111"),
        //             _ => return Some("000"),
        //         }
        //     }
        //     None
        let instruction_type = instruction_type(line);
        if let Some(InstructionType::CInstruction) = instruction_type {
            let instruction: Vec<&str> = line[2..].split(";").collect();
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
}

fn instruction_type(line: &str) -> Option<InstructionType> {
    // let lines: Vec<&str> = self.input.split("\n").collect();
    // let current_instruction = lines[self.index].trim();
    // if current_instruction.starts_with("@") {
    //     return InstructionType::AInstruction;
    // } else if current_instruction.starts_with("(") {
    //     return InstructionType::LInstruction;
    // } else {
    //     return InstructionType::CInstruction;
    // }

    if line.starts_with("@") {
        return Some(InstructionType::AInstruction);
    } else if line.starts_with("(") {
        return Some(InstructionType::LInstruction);
    } else {
        return Some(InstructionType::CInstruction);
    }
    None
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::*;

    // #[test]
    // fn should_return_line_content() {
    //     let mut parser = Parser::new("Add.asm").unwrap();
    //     let line = parser.advance().unwrap();

    //     if let Ok(text) = line {
    //         assert_eq!(text, "// This file is part of www.nand2tetris.org".to_string())
    //     }

    // }

    // #[test]
    // fn init_new_parser_with_test_file() {
    //     let parser = Parser::new("test.asm");
    //     assert_eq!(
    //         parser.input,
    //         "// Computes R1=1+...+R0\n// i = 1\n@i".to_string()
    //     );
    // }

    // #[test]
    // fn check_if_there_is_more_lines() {
    //     let parser = Parser::new("test.asm");
    //     assert_eq!(parser.has_more_lines(), true)
    // }

    #[test]
    fn check_if_whitespace_and_comments_are_skipped_and_line_count_is_incremented() {
        let mut parser = Parser::new("Add.asm").unwrap();
        let line = parser.advance().unwrap();

        if let Ok(text) = line {
            assert_eq!(text, "@2".to_string());
            assert_eq!(parser.line_count, 1);
        }
    }

    // #[test]
    // fn check_if_whitespace_and_comments_are_skipped_when_advance_called_more_times() {
    //     let mut parser = Parser::new("Sum1ToN.asm");
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     assert_eq!(parser.index, 9)
    // }

    // #[test]
    // fn the_instruction_type_should_be_a_instruction() {
    //     let mut parser = Parser::new("test.asm");
    //     parser.advance();
    //     assert_eq!(parser.instruction_type(), InstructionType::AInstruction)
    // }

    // #[test]
    // fn the_instruction_type_should_be_l_instruction() {
    //     let mut parser = Parser::new("Sum1ToN.asm");
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     assert_eq!(parser.instruction_type(), InstructionType::LInstruction)
    // }

    // #[test]
    // fn the_instruction_type_should_be_c_instruction() {
    //     let mut parser = Parser::new("Sum1ToN.asm");
    //     parser.advance();
    //     parser.advance();
    //     assert_eq!(parser.instruction_type(), InstructionType::CInstruction)
    // }

    // #[test]
    // fn should_return_the_label_without_parenthesis_if_the_current_is_a_l_instruction() {
    //     let mut parser = Parser::new("Sum1ToN.asm");
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     assert_eq!(parser.symbol(), Some("LOOP"))
    // }

    // #[test]
    // fn should_return_the_label_without_arobase_if_the_current_is_an_a_instruction() {
    //     let mut parser = Parser::new("Sum1ToN.asm");
    //     parser.advance();
    //     assert_eq!(parser.symbol(), Some("i"))
    // }

    // #[test]
    // fn should_return_none_if_the_current_is_a_c_instruction() {
    //     let mut parser = Parser::new("Sum1ToN.asm");
    //     parser.advance();
    //     parser.advance();
    //     assert_eq!(parser.symbol(), None)
    // }

    // #[test]
    // fn should_return_001_the_dest_part_if_the_current_is_a_c_instruction() {
    //     let mut parser = Parser::new("Sum1ToN.asm");
    //     parser.advance();
    //     parser.advance();
    //     assert_eq!(parser.dest(), Some("001"))
    // }

    // #[test]
    // fn should_return_010_the_dest_part_if_the_current_is_a_c_instruction() {
    //     let mut parser = Parser::new("Sum1ToN.asm");
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     assert_eq!(parser.dest(), Some("010"))
    // }

    // #[test]
    // fn should_return_111111_the_comp_part_if_the_current_is_a_c_instruction() {
    //     let mut parser = Parser::new("Sum1ToN.asm");
    //     parser.advance();
    //     parser.advance();
    //     assert_eq!(parser.comp(), Some("111111"))
    // }

    // #[test]
    // fn should_return_111111_the_dest_part_if_the_current_is_a_c_instruction() {
    //     let mut parser = Parser::new("Sum1ToN.asm");
    //     parser.advance();
    //     parser.advance();
    //     assert_eq!(parser.comp(), Some("111111"))
    // }

    // #[test]
    // fn should_return_001_the_jump_if_the_current_is_a_c_instruction() {
    //     let mut parser = Parser::new("Sum1ToN.asm");
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     assert_eq!(parser.jump(), Some("001"))
    // }

    // #[test]
    // fn should_return_111_the_jump_if_the_current_is_a_c_instruction() {
    //     let mut parser = Parser::new("Sum1ToN.asm");
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     parser.advance();
    //     assert_eq!(parser.jump(), Some("111"))
    // }
}
