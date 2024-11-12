use std::fs::File;

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
  file: File
}

impl Parser {
  // Opens the input file/stream, and gets ready to parse it
  fn new(file: File) -> Self {
    todo!()
  }

  // Are there more lines in the input ?
  fn hasMoreLines(&self) {
    todo!()
  }

  // Reads the next command from the input and makes it the current command
  fn advance(&self) {
    todo!()
  }

  // Returns a constant representing the type of the current command.
  fn commandType(&self) -> CommandType {
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