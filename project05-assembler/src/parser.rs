

enum InstructionType {
  AInstruction,
  CInstruction,
  LInstruction,
}

struct Parser {
  file: String,
}

impl Parser {
  fn new(&self, file: String) -> Self {
      Self { file }
  }

  fn has_more_lines(&self) -> bool {
      todo!()
  }

  fn advance(&self) {
      todo!()
  }

  fn instruction_type(&self) -> InstructionType {
      todo!()
  }

  fn symbol(&self) -> String {
      todo!()
  }

  fn dest(&self) -> String {
      todo!()
  }

  fn comp(&self) -> String {
      todo!()
  }

  fn jump(&self) -> String {
      todo!()
  }
}



#[cfg(test)]
mod tests {

    use super::*;

    fn get_file_from_cli_args() {
        todo!()
    }
}