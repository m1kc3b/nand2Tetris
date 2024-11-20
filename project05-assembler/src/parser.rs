#[derive(Debug, PartialEq, Eq)]
enum InstructionType {
    AInstruction,
    CInstruction,
    LInstruction,
}

#[derive(Debug)]
pub struct Parser {
    pub input: String,
    pub index: usize,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Self { input, index: 0 }
    }

    pub fn has_more_lines(&self) -> bool {
        let lines: Vec<&str> = self.input.split("\n").collect();
        lines.len() > self.index
    }

    fn advance(&mut self) {
        let lines: Vec<&str> = self.input.split("\n").collect();
        if self.has_more_lines() {
            self.index += 1;
            // skip comments, whitespaces and empty lines
            if lines[self.index].trim().starts_with("//") | lines[self.index].is_empty() {
                self.index += 1;
            }
        }
    }

    fn instruction_type(&self) -> InstructionType {
        let lines: Vec<&str> = self.input.split("\n").collect();
        let current_instruction = lines[self.index].trim();
        if current_instruction.starts_with("@") {
            return InstructionType::AInstruction;
        } else if current_instruction.starts_with("(") {
            return InstructionType::LInstruction;
        } else {
            return InstructionType::CInstruction;
        }
    }

    fn symbol(&self) -> Option<&str> {
        let lines: Vec<&str> = self.input.split("\n").collect();
        let current_instruction = lines[self.index].trim();

        match self.instruction_type() {
            InstructionType::AInstruction => {
                Some(&current_instruction[1..])
            }
            InstructionType::LInstruction => {
                Some(&current_instruction[1..current_instruction.len() -1])
            }
            _ => None,
        }
    }

      fn dest(&self) -> Option<&str> {
        let lines: Vec<&str> = self.input.split("\n").collect();
        let current_instruction = lines[self.index].trim();

        if let InstructionType::CInstruction = self.instruction_type() {
            let instruction = &current_instruction[..1];

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

      fn comp(&self) -> Option<&str> {
        let lines: Vec<&str> = self.input.split("\n").collect();
        let current_instruction = lines[self.index].trim();

        if let InstructionType::CInstruction = self.instruction_type() {
            let instruction = &current_instruction[2..];

            match instruction {
                "0" => return Some("101010"),
                "1" => return Some("111111"),
                "-1" => return Some("111010"),
                "D" => return Some("001100"),
                "A"|"M" => return Some("110000"),
                "!D" => return Some("001101"),
                "!A"|"!M" => return Some("110001"),
                "-D" => return Some("001111"),
                "-A"|"-M" => return Some("110011"),
                "D+1" => return Some("011111"),
                "A+1"|"M+1" => return Some("110111"),
                "D-1" => return Some("001110"),
                "A-1"|"M-1" => return Some("110010"),
                "D+A"|"D+M" => return Some("000010"),
                "D-A"|"D-M" => return Some("010011"),
                "A-D"|"M-D" => return Some("000111"),
                "D&A"|"D&M" => return Some("000000"),
                "D|A"|"D|M" => return Some("010101"),
                _ => return None
            }
        }
        None
      }

      fn jump(&self) -> Option<&str> {
        let lines: Vec<&str> = self.input.split("\n").collect();
        let current_instruction = lines[self.index].trim();

        if let InstructionType::CInstruction = self.instruction_type() {
            let instruction = &current_instruction[2..];

            match instruction {
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

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn init_new_parser_with_test_file() {
        let input = read_to_string("asm-files/test.asm").unwrap();
        let parser = Parser::new(input);
        assert_eq!(
            parser.input,
            "// Computes R1=1+...+R0\n// i = 1\n@i".to_string()
        );
    }

    #[test]
    fn check_if_there_is_more_lines() {
        let input = read_to_string("asm-files/test.asm").unwrap();
        let parser = Parser::new(input);
        assert_eq!(parser.has_more_lines(), true)
    }

    #[test]
    fn check_if_whitespace_and_comments_are_skipped() {
        let input = read_to_string("asm-files/test.asm").unwrap();
        let mut parser = Parser::new(input);
        parser.advance();
        assert_eq!(parser.index, 2)
    }

    #[test]
    fn check_if_whitespace_and_comments_are_skipped_when_advance_called_more_times() {
        let input = read_to_string("asm-files/Sum1ToN.asm").unwrap();
        let mut parser = Parser::new(input);
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        assert_eq!(parser.index, 9)
    }

    #[test]
    fn the_instruction_type_should_be_a_instruction() {
        let input = read_to_string("asm-files/test.asm").unwrap();
        let mut parser = Parser::new(input);
        parser.advance();
        assert_eq!(parser.instruction_type(), InstructionType::AInstruction)
    }

    #[test]
    fn the_instruction_type_should_be_l_instruction() {
        let input = read_to_string("asm-files/Sum1ToN.asm").unwrap();
        let mut parser = Parser::new(input);
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        assert_eq!(parser.instruction_type(), InstructionType::LInstruction)
    }

    #[test]
    fn the_instruction_type_should_be_c_instruction() {
        let input = read_to_string("asm-files/Sum1ToN.asm").unwrap();
        let mut parser = Parser::new(input);
        parser.advance();
        parser.advance();
        assert_eq!(parser.instruction_type(), InstructionType::CInstruction)
    }

    #[test]
    fn should_return_the_label_without_parenthesis_if_the_current_is_a_l_instruction() {
        let input = read_to_string("asm-files/Sum1ToN.asm").unwrap();
        let mut parser = Parser::new(input);
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        assert_eq!(parser.symbol(), Some("LOOP"))
    }

    #[test]
    fn should_return_the_label_without_arobase_if_the_current_is_an_a_instruction() {
        let input = read_to_string("asm-files/Sum1ToN.asm").unwrap();
        let mut parser = Parser::new(input);
        parser.advance();
        assert_eq!(parser.symbol(), Some("i"))
    }

    #[test]
    fn should_return_none_if_the_current_is_a_c_instruction() {
        let input = read_to_string("asm-files/Sum1ToN.asm").unwrap();
        let mut parser = Parser::new(input);
        parser.advance();
        parser.advance();
        assert_eq!(parser.symbol(), None)
    }

    #[test]
    fn should_return_001_the_dest_part_if_the_current_is_a_c_instruction() {
        let input = read_to_string("asm-files/Sum1ToN.asm").unwrap();
        let mut parser = Parser::new(input);
        parser.advance();
        parser.advance();
        assert_eq!(parser.dest(), Some("001"))
    }

    #[test]
    fn should_return_010_the_dest_part_if_the_current_is_a_c_instruction() {
        let input = read_to_string("asm-files/Sum1ToN.asm").unwrap();
        let mut parser = Parser::new(input);
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        assert_eq!(parser.dest(), Some("010"))
    }

    #[test]
    fn should_return_111111_the_comp_part_if_the_current_is_a_c_instruction() {
        let input = read_to_string("asm-files/Sum1ToN.asm").unwrap();
        let mut parser = Parser::new(input);
        parser.advance();
        parser.advance();
        assert_eq!(parser.comp(), Some("111111"))
    }

    #[test]
    fn should_return_111111_the_dest_part_if_the_current_is_a_c_instruction() {
        let input = read_to_string("asm-files/Sum1ToN.asm").unwrap();
        let mut parser = Parser::new(input);
        parser.advance();
        parser.advance();
        assert_eq!(parser.comp(), Some("111111"))
    }

    #[test]
    fn should_return_001_the_jump_if_the_current_is_a_c_instruction() {
        let input = read_to_string("asm-files/Sum1ToN.asm").unwrap();
        let mut parser = Parser::new(input);
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
        parser.advance();
        assert_eq!(parser.jump(), Some("001"))
    }

    #[test]
    fn should_return_111_the_jump_if_the_current_is_a_c_instruction() {
        let input = read_to_string("asm-files/Sum1ToN.asm").unwrap();
        let mut parser = Parser::new(input);
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
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        parser.advance();
        assert_eq!(parser.jump(), Some("111"))
    }
}
