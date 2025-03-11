use crate::parser::{CommandType, MemorySegment};

pub fn write_arithmetic(command: String) -> String {
  // writes to the output file the assembly code that implements the given arithmetic-logical command.
  match command.as_str() {
      "add" => {
        // x + y
        return "// add\n@SP\nAM=M-1\nD=M\nA=A-1\nM=M+D\n\n".to_string();
      },
      "sub" => { 
        // SP--, SP--, M = M - D
        return "// sub\n@SP\nAM=M-1\nD=M\nA=A-1\nM=M-D\n\n".to_string();
      },
      "neg" => {
        // M = -M
        return "// neg\n@SP\nA=M-1\nM=-M\n\n".to_string();
      },
      "eq" => {
        // P--, SP--, M = (M == D) ? -1 : 0
        return "// eq@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@EQUAL\nD;JEQ\n@SP\nA=M-1\nM=0\n@END\n0;JMP\n(EQUAL)\n@SP\nA=M-1\nM=-1\n(END)\n\n".to_string();
      },
      "gt" => {
        // SP--, SP--, M = (M > D) ? -1 : 0
        return "// gt\n@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@GREATER\nD;JGT\n@SP\nA=M-1\nM=0\n@END\n0;JMP\n(GREATER)\n@SP\nA=M-1\nM=-1\n(END)\n\n".to_string();
      },
      "lt" => {
        // SP--, SP--, M = (M < D) ? -1 : 0
        return "// lt\n@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@LESS\nD;JLT\n@SP\nA=M-1\nM=0\n@END\n0;JMP\n(LESS)\n@SP\nA=M-1\nM=-1\n(END)\n\n".to_string();
      },
      "and" => {
        // SP--, SP--, M = M & D
        return "// and\n@SP\nAM=M-1\nD=M\nA=A-1\nM=M&D\n\n".to_string();
      },
      "or" => {
        // SP--, SP--, M = M | D
        return "// or\n@SP\nAM=M-1\nD=M\nA=A-1\nM=M|D\n\n".to_string();
      },
      "not" => {
        //  = !M
        return "// not\n@SP\nA=M-1\nM=!M\n".to_string();
      },
      _ => panic!("Command unknown!")
  }
}

pub fn write_push_pop(command: CommandType) -> String {
  // write to the output file the assembly code that implements the given push or pop command.
  match command {
      CommandType::Push(segment, index) => {
        let code = match segment {
            MemorySegment::Argument => format!("// push argument {index}\n@{}\nD=A\n@ARG\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n", index),
            MemorySegment::Constant => format!("// push constant {index}\n@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n", index),
            MemorySegment::Local => format!("// push local {index}\n@{}\nD=A\n@LCL\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n", index),
            MemorySegment::Pointer => {
              if index == 0 {
                  return format!("// push pointer {index}\n@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
              }
              return format!("// push pointer {index}\n@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
            },
            MemorySegment::Static => format!("// push static {index}\n@StaticVar_{}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n", index),
            MemorySegment::Temp => format!("// push temp {index}\n@5\nD=A\n@{}\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n", index),
            MemorySegment::That => format!("// push that {index}\n@{}\nD=A\n@THAT\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n", index),
            MemorySegment::This => format!("// push this {index}\n@{}\nD=A\n@THIS\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n", index),
        };
        return code;
      },
      CommandType::Pop(segment, index) => {
        let code = match segment {
          MemorySegment::Argument => format!("// pop argument {index}\n@{}\nD=A\n@ARG\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n\n", index),
          MemorySegment::Local => format!("// pop local {index}\n@{}\nD=A\n@LCL\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n\n", index),
          MemorySegment::Pointer => {
            if index == 0 {
              return format!("// pop pointer {index}\n@SP\nAM=M-1\nD=M\n@THIS\nM=D\n\n")
            }
            return format!("// pop pointer {index}\n@SP\nAM=M-1\nD=M\n@THAT\nM=D\n\n")
          },
          MemorySegment::Static => format!("// pop static {index}\n@SP\nAM=M-1\nD=M\n@StaticVar_{}\nM=D\n\n", index),
          MemorySegment::Temp => format!("// pop temp {index}\n@5\nD=A\n@{}\nD=A+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n\n", index),
          MemorySegment::That => format!("// pop that {index}\n@{}\nD=A\n@THAT\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n\n", index),
          MemorySegment::This => format!("// pop this {index}\n@{}\nD=A\n@THIS\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n\n", index),
          _ => panic!("Segment unknow!")
      };
      return code;
      },
      _ => panic!("Command unknown!"),
  }
}
