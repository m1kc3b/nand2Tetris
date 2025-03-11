use crate::parser::{CommandType, MemorySegment};

pub fn write_arithmetic(command: String) -> String {
  // writes to the output file the assembly code that implements the given arithmetic-logical command.
  match command.as_str() {
      "add" => {
        // x + y
        return "@SP\nAM=M-1\nD=M\nA=A-1\nM=M+D\n".to_string();
      },
      "sub" => { 
        // SP--, SP--, M = M - D
        return "@SP\nAM=M-1\nD=M\nA=A-1\nM=M-D\n".to_string();
      },
      "neg" => {
        // M = -M
        return "@SP\nA=M-1\nM=-M\n".to_string();
      },
      "eq" => {
        // P--, SP--, M = (M == D) ? -1 : 0
        return "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@EQUAL\nD;JEQ\n@SP\nA=M-1\nM=0\n@END\n0;JMP\n(EQUAL)\n@SP\nA=M-1\nM=-1\n(END)\n".to_string();
      },
      "gt" => {
        // SP--, SP--, M = (M > D) ? -1 : 0
        return "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@GREATER\nD;JGT\n@SP\nA=M-1\nM=0\n@END\n0;JMP\n(GREATER)\n@SP\nA=M-1\nM=-1\n(END)\n".to_string();
      },
      "lt" => {
        // SP--, SP--, M = (M < D) ? -1 : 0
        return "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@LESS\nD;JLT\n@SP\nA=M-1\nM=0\n@END\n0;JMP\n(LESS)\n@SP\nA=M-1\nM=-1\n(END)\n".to_string();
      },
      "and" => {
        // SP--, SP--, M = M & D
        return "@SP\nAM=M-1\nD=M\nA=A-1\nM=M&D\n".to_string();
      },
      "or" => {
        // SP--, SP--, M = M | D
        return "@SP\nAM=M-1\nD=M\nA=A-1\nM=M|D\n".to_string();
      },
      "not" => {
        //  = !M
        return "@SP\nA=M-1\nM=!M\n".to_string();
      },
      _ => panic!("Command unknown!")
  }
}

pub fn write_push_pop(command: CommandType) -> String {
  // write to the output file the assembly code that implements the given push or pop command.
  match command {
      CommandType::Push(segment, index) => {
        let code = match segment {
            MemorySegment::Argument => format!("@{}\nD=A\n@ARG\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", index),
            MemorySegment::Constant => format!("@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", index),
            MemorySegment::Local => format!("@{}\nD=A\n@LCL\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", index),
            MemorySegment::Pointer => {
              if index == 0 {
                  return format!("@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n")
              }
              return format!("@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n")
            },
            MemorySegment::Static => format!("@StaticVar_{}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", index),
            MemorySegment::Temp => format!("@5\nD=A\n@{}\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", index),
            MemorySegment::That => format!("@{}\nD=A\n@THAT\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", index),
            MemorySegment::This => format!("@{}\nD=A\n@THIS\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", index),
        };
        return code;
      },
      CommandType::Pop(segment, index) => {
        let code = match segment {
          MemorySegment::Argument => format!("@{}\nD=A\n@ARG\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n", index),
          MemorySegment::Local => format!("@{}\nD=A\n@LCL\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n", index),
          MemorySegment::Pointer => {
            if index == 0 {
              return format!("@SP\nAM=M-1\nD=M\n@THIS\nM=D\n")
            }
            return format!("@SP\nAM=M-1\nD=M\n@THAT\nM=D\n")
          },
          MemorySegment::Static => format!("@SP\nAM=M-1\nD=M\n@StaticVar_{}\nM=D\n", index),
          MemorySegment::Temp => format!("@5\nD=A\n@{}\nD=A+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n", index),
          MemorySegment::That => format!("@{}\nD=A\n@THAT\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n", index),
          MemorySegment::This => format!("@{}\nD=A\n@THIS\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n", index),
          _ => panic!("Segment unknow!")
      };
      return code;
      },
      _ => panic!("Command unknown!"),
  }
}
