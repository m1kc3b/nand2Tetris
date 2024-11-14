use std::fs::read_to_string;
use std::io::Error;


#[derive(Debug, PartialEq, Eq)]
pub enum CommandType {
  C_ARITHMETIC,
  C_PUSH,
  C_POP,
  C_LABEL,
  C_GOTO,
  C_IF,
  C_FUNCTION,
  C_RETURN,
  C_CALL
}

pub struct Parser {
  pub commands: Vec<String>,
  pub index: usize,
}

impl Parser {
  // Opens the input file/stream, and gets ready to parse it
  pub fn new(path: &str) -> Result<Self, Error> {
    let file_content = read_to_string(path)?;
    let commands: Vec<String> = file_content.lines().map(String::from).collect();

    Ok(Self { 
      commands, 
      index: 0,
    })
  }

  // Are there more lines in the input ?
  pub fn has_more_lines(&self) -> bool {
    self.index < self.commands.len()
  }

  // Reads the next command from the input and makes it the current command
  pub fn advance(&mut self) {
    if self.index < self.commands.len() {
      self.index += 1;
    }
  }

  // Returns a constant representing the type of the current command.
  pub fn command_type(&self) -> Option<CommandType> {
    let command = &self.commands[self.index];
    let cmd: Vec<&str> = command.split(" ").collect();
      match cmd[0] {
        "push" => return Some(CommandType::C_PUSH),
        "pop" => return  Some(CommandType::C_POP),
        "add" => return  Some(CommandType::C_ARITHMETIC),
        "sub" => return  Some(CommandType::C_ARITHMETIC),
        "neg" => return  Some(CommandType::C_ARITHMETIC),
        "eq" => return  Some(CommandType::C_ARITHMETIC),
        "gt" => return  Some(CommandType::C_ARITHMETIC),
        "lt" => return  Some(CommandType::C_ARITHMETIC),
        "and" => return  Some(CommandType::C_ARITHMETIC),
        "or" => return  Some(CommandType::C_ARITHMETIC),
        "not" => return  Some(CommandType::C_ARITHMETIC),
        _ => return None
      }
  }

  // Returns the first argument of the current command
  pub fn arg1(&self) -> Option<&str> {
    let command = &self.commands[self.index];
    let args: Vec<&str> = command.split(" ").collect();
    Some(args[1])
  }

  // Returns the second argument of the current command
  pub fn arg2(&self) -> Option<&str>{
    let command = &self.commands[self.index];
    let args: Vec<&str> = command.split(" ").collect();
    Some(args[2])
  }
}

mod tests {
  use super::*;

  #[test]
  fn open_the_test_file_and_create_the_parser_struct() {
    let parser = Parser::new("test_file.txt");
    if let Ok(p) = parser {
      assert_eq!(p.commands, vec!["This is a test file...".to_string(), "Second line of this file.".to_string()]);
    }
  }

  #[test]
  fn parser_should_have_more_lines() {
    let parser = Parser::new("test_file.txt");
    if let Ok(p) = parser {
      assert_eq!(p.has_more_lines(), true);
    }
  }

  #[test]
  fn parser_should_not_have_more_lines() {
    let parser = Parser::new("test_file_with_one_line.txt");
    if let Ok(mut p) = parser {
      p.index = 1;
      assert_eq!(p.has_more_lines(), false);
    }
  }

  #[test]
  fn reading_current_command_should_return_this_is_a_test_file() {
    let parser = Parser::new("test_file.txt");
    if let Ok(mut p) = parser {
      assert_eq!(p.index, 0);
      p.advance();
      assert_eq!(p.index, 1);
    }
  }

  #[test]
  fn command_type_should_be_c_arithmetic() {
    let parser = Parser::new("ProgramTest.asm");
    if let Ok(p) = parser {
      assert_eq!(p.command_type(), Some(CommandType::C_PUSH));
    }
  }

  #[test]
  fn arg1_should_be_local() {
    let parser = Parser::new("ProgramTest.asm");
    if let Ok(p) = parser {
      assert_eq!(p.arg1(), Some("local"));
    }
  }

  #[test]
  fn arg1_should_not_be_argument() {
    let parser = Parser::new("ProgramTest.asm");
    if let Ok(p) = parser {
      assert_ne!(p.arg1(), Some("argument"));
    }
  }

  #[test]
  fn arg2_should_be_one() {
    let parser = Parser::new("ProgramTest.asm");
    if let Ok(mut p) = parser {
      assert_eq!(p.arg2(), Some("1"));
    }
  }

  #[test]
  fn arg2_should_not_be_three() {
    let parser = Parser::new("ProgramTest.asm");
    if let Ok(mut p) = parser {
      assert_ne!(p.arg2(), Some("3"));
    }
  }
}