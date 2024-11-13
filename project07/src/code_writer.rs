use std::{fs::File, io::Write};

struct CodeWriter {
  file: File
}

impl CodeWriter {
  fn new(filename: String) -> Result<Self, String> {
    let file = File::create_new(format!("{}.asm", filename));

    if let Ok(f) = file {
      return Ok(Self { file: f })
    }
    Err("Impossible de créer le fichier".to_string())
  }

  // Writes to the output file the assembly code that implements the given arithmetic-logical command
  fn write_arithmetic(&mut self, command: String) {
    let file = self.file;
    file.write(command);
  }

  // Writes to the output file the assembly code that implements the given push or pop command
  fn write_push_pop(&mut self, command: String, segment: String, index: i16) {
    let file = self.file;
    // Check the command_type (C_PUSH or C_POP)
    let command = format!("{command} {segment} {index}");
    file.write(command);
  }

  // Closes the output file/stream
  fn closes(&self) {
    todo!()
  }
}


mod tests {
    use super::*;


}