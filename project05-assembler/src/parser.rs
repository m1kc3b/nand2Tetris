#[derive(Debug, PartialEq, Eq)]
enum InstructionType {
    AInstruction,
    CInstruction,
    LInstruction,
}

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
            // skip comments and whitespaces
            if lines[self.index].trim().starts_with("//") {
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

    //   fn symbol(&self) -> String {
    //       todo!()
    //   }

    //   fn dest(&self) -> String {
    //       todo!()
    //   }

    //   fn comp(&self) -> String {
    //       todo!()
    //   }

    //   fn jump(&self) -> String {
    //       todo!()
    //   }
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
}
