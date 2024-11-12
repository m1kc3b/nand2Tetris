use std::fs::File;


struct code_writer {
  file: File
}

impl code_writer {
  fn new(file: File) -> Self {
    todo!()
  }

  // Writes to the output file the assembly code that implements the given arithmetic-logical command
  fn writeArithmetic(&mut self, command: String) {
    todo!()
  }

  // Writes to the output file the assembly code that implements the given push or pop command
  fn writePushPop(&mut self, command: String, segment: String, index: i16) {
    todo!()
  }

  // Closes the output file/stream
  fn closes(&self) {
    todo!()
  }
}