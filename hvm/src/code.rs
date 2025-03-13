use std::{io::Result, sync::Mutex};

use crate::parser::{CommandType, MemorySegment};
// use crate::errors::HVMError;

static STATIC_COUNT: u16 = 16;

pub fn write_arithmetic(command: String) -> Result<String> {
  // writes to the output file the assembly code that implements the given arithmetic-logical command.
  match command.as_str() {
      "add" => Ok("// add\n@SP\nA=M-1\nD=M\nM=0\n@SP\nM=M-1\nA=M-1\nM=D+M\n\n".to_string()),
      "sub" => Ok("// sub\n@SP\nA=M-1\nD=-M\nM=0\n@SP\nM=M-1\nA=M-1\nM=D+M\n\n".to_string()),
      "neg" => Ok("// neg\n@SP\nA=M-1\nM=-M\n\n".to_string()),
      // "eq" => Ok("// eq\n@SP\nA=M-1\nD=M\nM=0\n@SP\nM=M-1\nA=M-1\nD=M-D\n@FALSE\nD;JEQ\nM=1\n(FALSE)\nM=0\n\n".to_string()),
      "eq" => Ok("// eq\n@SP\nA=M-1\nD=M\nA=A-1\nD=M-D\n@EQ_TRUE\nD;JEQ\n@SP\nA=M-1\nM=0\n@EQ_END\n0;JMP\n(EQ_TRUE)\n  @SP\n  A=M-1\n  M=-1\n(EQ_END)\n\n".to_string()),
      // "gt" => Ok("// gt\n@SP\nA=M-1\nD=M\nM=0\n@SP\nM=M-1\nA=M-1\nD=M-D\n@FALSE\nD;JLT\nM=1\n(FALSE)\nM=0\n\n".to_string()),
      "gt" => Ok("// gt\n@SP\nA=M-1\nD=M\nA=A-1\nD=M-D\n@GT_TRUE\nD;JGT\n@SP\nA=M-1\nM=0\n@GT_END\n0;JMP\n(GT_TRUE)\n  @SP\n  A=M-1\n  M=-1\n(GT_END)\n\n".to_string()),
      // "lt" => Ok("// lt\n@SP\nA=M-1\nD=M\nM=0\n@SP\nM=M-1\nA=M-1\nD=M-D\n@FALSE\nD;JGT\nM=1\n(FALSE)\nM=0\n\n".to_string()),
      "lt" => Ok("// lt\n@SP\nM=M-1\nD=M\nA=A-1\nD=M-D\n@LT_TRUE\nD;JLT\n@SP\nA=M-1\nM=0\n@LT_END\n0;JMP\n(LT_TRUE)\n  @SP\n  A=M-1\n  M=-1\n(LT_END)\n\n".to_string()),
      // "and" => Ok("// and\n@SP\nA=M-1\nD=M\nM=0\n@SP\nM=M-1\nA=M-1\nD=D&M\n@FALSE\nD;JNE\nM=1\n(FALSE)\nM=0\n\n".to_string()),
      "and" => Ok("// and\n@SP\nA=M-1\nD=M\nA=A-1\nM=D&M\n\n".to_string()),
      // "or" => Ok("// or\n@SP\nA=M-1\nD=M\nM=0\n@SP\nM=M-1\nA=M-1\nD=D|M\n@FALSE\nD;JNE\nM=1\n(FALSE)\nM=0\n\n".to_string()),
      "or" => Ok("// or\n@SP\nA=M-1\nD=M\nA=A-1\nM=D|M\n\n".to_string()),
      // "not" => Ok("// not\n@SP\nA=M-1\nD=M\nM=0\nD=!D\n@FALSE\nD;JNE\nM=1\n(FALSE)\nM=D\n".to_string()),
      "not" => Ok("// not\n@SP\nA=M-1\nM=!M\n\n".to_string()),
      // _ => Err(HVMError::UnknownCommand("Unknow arithmetic command")),
      _ => panic!("Unknow arithmetic command")
      }
}

pub fn write_push_pop(command: CommandType, filename: &str) -> Result<String> {
  // write to the output file the assembly code that implements the given push or pop command.
  match command {
      CommandType::Push(segment, index) => {
        let code = match segment {
            MemorySegment::Argument => write_push_argument(index),
            MemorySegment::Constant => write_push_constant(index),
            MemorySegment::Local => write_push_local(index),
            MemorySegment::Pointer => write_push_pointer(index),
            MemorySegment::Static => write_push_static(filename, index),
            MemorySegment::Temp => write_push_temp(index),
            MemorySegment::That => write_push_that(index),
            MemorySegment::This => write_push_this(index),
        };
        return Ok(code);
      },
      CommandType::Pop(segment, index) => {
        let code = match segment {
          MemorySegment::Argument => write_pop_argument(index),
          MemorySegment::Local => write_pop_local(index),
          MemorySegment::Pointer => write_pop_pointer(index),
          MemorySegment::Static => write_pop_static(filename, index),
          MemorySegment::Temp => write_pop_temp(index),
          MemorySegment::That => write_pop_that(index),
          MemorySegment::This => write_pop_this(index),
          _ => panic!("Segment unknow!")
      };
      return Ok(code);
      },
      // _ => Err(HVMError::UnknownCommand("Unknow push/pop command")),
      _ => panic!("Unknow push/pop command")
  }
}

fn write_push_argument(index: u16) -> String {
  format!("// push argument {index}\n@{index}\nD=A\n@ARG\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
}

fn write_pop_argument(index: u16) -> String {
  let increment = incremente_m(index);

  format!("// pop argument {index}\n@SP\nA=M-1\nD=M\nM=0\n@SP\nM=M-1\n@ARG\n{}M=D\n\n", increment)
}

fn write_push_this(index: u16) -> String {
  format!("// push this {index}\n@{index}\nD=A\n@THIS\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
}

fn write_pop_this(index: u16) -> String {
  let increment = incremente_m(index);

  format!("// pop this {index}\n@SP\nA=M-1\nD=M\nM=0\n@SP\nM=M-1\n@THIS\n{}M=D\n\n", increment)
}

fn write_push_that(index: u16) -> String {
  format!("// push that {index}\n@{index}\nD=A\n@THAT\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
}

fn write_pop_that(index: u16) -> String {
  let increment = incremente_m(index);

  format!("// pop that {index}\n@SP\nA=M-1\nD=M\nM=0\n@SP\nM=M-1\n@THAT\n{}M=D\n\n", increment)
}

fn write_push_temp(index: u16) -> String {
  format!("// push temp {index}\n@{index}\nD=A\n@TEMP\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
}

fn write_pop_temp(index: u16) -> String {
  let increment = incremente_m(index);

  format!("// pop temp {index}\n@SP\nA=M-1\nD=M\nM=0\n@SP\nM=M-1\n@TEMP\n{}M=D\n\n", increment)
}

fn write_push_static(filename: &str, index: u16) -> String {
  let count =  STATIC_COUNT + index;
  format!("// push static {index}\n@{index}\nD=A\n@{filename}.{index}\n@{count}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
}

fn write_pop_static(filename: &str, index: u16) -> String {
  let count =  STATIC_COUNT + index;
  format!("// pop static {index}\n@SP\nA=M-1\nD=M\nM=0\n@SP\nM=M-1\n@{filename}.{index}\n@{count}\nM=D\n\n")
}

fn write_push_pointer(index: u16) -> String {
  if index == 0 {
    return write_push_this(index);
  } else {
    return write_pop_that(index);
  }
}

fn write_pop_pointer(index: u16) -> String {
  if index == 0 {
    return write_pop_this(index);
  } else {
    return write_pop_that(index);
  }
}

fn write_push_constant(index: u16) -> String {
  format!("// push constant {index}\n@{index}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
}

fn write_push_local(index: u16) -> String {
  format!("// push local {index}\n@{index}\nD=A\n@LCL\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
}

fn write_pop_local(index: u16) -> String {
  let increment = incremente_m(index);

  format!("// pop local {index}\n@SP\nA=M-1\nD=M\nM=0\n@SP\nM=M-1\n@LCL\n{}M=D\n\n", increment)
}


fn incremente_m(index: u16) -> String {
  let mut increment = String::new();

  match index {
      0 => increment.push_str("A=M\n"),
      1 => increment.push_str("A=M+1\n"),
      _ => {
        increment.push_str("A=M+1\n");
        for _ in 1..index {
          increment.push_str("A=A+1\n")
        }
      } 
  }

  increment
}