use std::fs::File;


struct CodeWriter {
  file: File
}

impl CodeWriter {
  fn new(file: File) -> Self {
    todo!()
  }

  // Writes to the output file the assembly code that implements the given arithmetic-logical command
  fn write_arithmetic(&mut self, command: String) {
    todo!()
  }

  // Writes to the output file the assembly code that implements the given push or pop command
  fn write_push_pop(&mut self, command: String, segment: String, index: i16) {
    todo!()
  }

  // Closes the output file/stream
  fn closes(&self) {
    todo!()
  }
}