use std::fs::File;
use std::fs::read_to_string;
use std::io::Error;
use std::path::Path;



enum CommandType {
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

struct Parser {
  commands: Vec<String>
}

impl Parser {
  // Opens the input file/stream, and gets ready to parse it
  fn new(path: String) -> Result<Self, Error> {
    // Checking the extension (.asm)
    // if Path::new(&path).extension().map_or(false, |ext| ext == "asm") {
    //   return Err(String::from("Le fichier doit avoir l'extension .asm"));
    // }

    let file_content = read_to_string(path)?;
    let commands: Vec<String> = file_content.lines().map(String::from).collect();

    Ok(Self { commands })
  }

  // Are there more lines in the input ?
  fn has_more_lines(&self) {
    todo!()
  }

  // Reads the next command from the input and makes it the current command
  fn advance(&self) {
    todo!()
  }

  // Returns a constant representing the type of the current command.
  fn command_type(&self) -> CommandType {
    todo!()
  }

  // Returns the first argument of the current command
  fn arg1(&self) -> String {
    todo!()
  }

  // Returns the second argument of the current command
  fn arg2(&self) {
    todo!()
  }
}

mod tests {
  use super::*;

  #[test]
  fn open_the_test_file_and_create_the_parser_struct() {
    let parser = Parser::new("test_file.txt".to_string());
    if let Ok(p) = parser {
      assert_eq!(p.commands, vec!["This is a test file...".to_string(), "Second line of this file.".to_string()]);
    }
  }
}