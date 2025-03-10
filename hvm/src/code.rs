use crate::parser::CommandType;

pub fn writeArithmetic(command: String) -> String {
  // writes to the output file the assembly code that implements the given arithmetic-logical command.
  match command.as_str() {
      "add" => {
        // x + y
        return "@SP\nAM=M-1\nD=M\nA=A-1\nM=M+D\n".to_string();
      },
      "sub" => {
        // x - y
        return "@SP\nAM=M-1\nD=M\nA=A-1\nM=M-D\n".to_string();
      },
      "neg" => {
        // -y
        return "@SP\nA=M-1\nM=-M\n".to_string();
      },
      "eq" => {
        // x == y
        return "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@EQUAL\nD;JEQ\n@SP\nA=M-1\nM=0\n@END\n0;JMP\n(EQUAL)\n@SP\nA=M-1\nM=-1\n(END)\n".to_string();
      },
      "gt" => {
        // x > y
        return "".to_string();
      },
      "lt" => {
        // x < y
        return "".to_string();
      },
      "and" => {
        // x and y
        return "".to_string();
      },
      "or" => {
        // x or y
        return "".to_string();
      },
      "not" => {
        // not y
        return "".to_string();
      },
      _ => panic!("Command unknown!")
  }
}

pub fn writePushPop(command: CommandType) -> String {
  // write to the output file the assembly code that implements the given push or pop command.
  match command {
      CommandType::Pop(segment, index) => {
        return "".to_string()
      },
      CommandType::Push(segment, index) => {
        return "".to_string()
      },
      _ => panic!("Command unknown!"),
  }
}
