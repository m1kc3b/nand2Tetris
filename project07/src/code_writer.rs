use std::{fs::File, io::Write};

pub struct CodeWriter {
  file: File
}

impl CodeWriter {
  pub fn new(file: &str) -> Result<Self, String> {
    let filename = file.split(".").next().unwrap_or(file);
    let file = File::create_new(format!("{}.asm", filename));

    if let Ok(f) = file {
      return Ok(Self { file: f })
    }
    Err("Impossible de créer le fichier".to_string())
  }

  // Writes to the output file the assembly code that implements the given arithmetic-logical command
  pub fn write_arithmetic(&mut self, command: String) -> std::io::Result<()>{
    self.file.write(command.as_bytes())?;
    Ok(())
  }

  // Writes to the output file the assembly code that implements the given push or pop command
  pub fn write_push_pop(&mut self, command: String, segment: String, index: i16) -> std::io::Result<()> {
    // Check the command_type (C_PUSH or C_POP)
    let command = format!("{command} {segment} {index}/n");
    self.file.write(command.as_bytes())?;
    Ok(())
  }

  // Closes the output file/stream
  // fn closes(&self) {
  //   todo!()
  // }
}


mod tests {
    use super::*;


}