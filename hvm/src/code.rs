use std::io::Result;
use crate::parser::{CommandType, MemorySegment};
// use crate::errors::HVMError;

pub fn write_arithmetic(command: String) -> Result<String> {
  // writes to the output file the assembly code that implements the given arithmetic-logical command.
  match command.as_str() {
      "add" => Ok("// add\n@SP\nAM=M-1\nD=M\nA=A-1\nM=M+D\n\n".to_string()),
      "sub" => Ok("// sub\n@SP\nAM=M-1\nD=M\nA=A-1\nM=M-D\n\n".to_string()),
      "neg" => Ok("// neg\n@SP\nA=M-1\nM=-M\n\n".to_string()),
      "eq" => Ok("// eq@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@EQUAL\nD;JEQ\n@SP\nA=M-1\nM=0\n@END\n0;JMP\n(EQUAL)\n@SP\nA=M-1\nM=-1\n(END)\n\n".to_string()),
      "gt" => Ok("// gt\n@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@GREATER\nD;JGT\n@SP\nA=M-1\nM=0\n@END\n0;JMP\n(GREATER)\n@SP\nA=M-1\nM=-1\n(END)\n\n".to_string()),
      "lt" => Ok("// lt\n@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@LESS\nD;JLT\n@SP\nA=M-1\nM=0\n@END\n0;JMP\n(LESS)\n@SP\nA=M-1\nM=-1\n(END)\n\n".to_string()),
      "and" => Ok("// and\n@SP\nAM=M-1\nD=M\nA=A-1\nM=M&D\n\n".to_string()),
      "or" => Ok("// or\n@SP\nAM=M-1\nD=M\nA=A-1\nM=M|D\n\n".to_string()),
      "not" => Ok("// not\n@SP\nA=M-1\nM=!M\n".to_string()),
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
  format!("// push argument {index}\n@ARG\nD=M\n@{index}\nA=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
}

fn write_pop_argument(index: u16) -> String {
  format!("// pop argument {index}\n@ARG\nD=M\n@{index}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n\n")
}

fn write_push_this(index: u16) -> String {
  format!("// push this {index}\n@THIS\nD=M\n@{index}\nA=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
}

fn write_pop_this(index: u16) -> String {
  format!("// pop this {index}\n@THIS\nD=M\n@{index}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n\n")
}

fn write_push_that(index: u16) -> String {
  format!("// push that {index}\n@THAT\nD=M\n@{index}\nA=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
}

fn write_pop_that(index: u16) -> String {
  format!("// pop that {index}\n@THAT\nD=M\n@{index}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n\n")
}

fn write_push_temp(index: u16) -> String {
  format!("// push temp {index}\n@5\nD=A\n@{index}\nA=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
}

fn write_pop_temp(index: u16) -> String {
  format!("// pop temp {index}\n@5\nD=A\n@{index}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n\n")
}

fn write_push_static(filename: &str, index: u16) -> String {
  format!("// push static {index}\n@{filename}.{index}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
}

fn write_pop_static(filename: &str, index: u16) -> String {
  format!("// pop static {index}\n@SP\nAM=M-1\nD=M\n@{filename}.{index}\nM=D\n\n")
}

fn write_push_pointer(index: u16) -> String {
  if index == 0 {
    return format!("// push pointer {index}\n@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n");
  } else {
    return format!("// push pointer {index}\n@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n");
  }
}

fn write_pop_pointer(index: u16) -> String {
  if index == 0 {
    return format!("// pop pointer {index}\n@SP\nAM=M-1\nD=M\n@THIS\nM=D\n\n");
  } else {
    return format!("// pop pointer {index}\n@SP\nAM=M-1\nD=M\n@THAT\nM=D\n\n");
  }
}

fn write_push_constant(index: u16) -> String {
  format!("// push constant {index}\n@{index}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
}

fn write_push_local(index: u16) -> String {
  format!("// push local {index}\n@LCL\nD=M\n@{index}\nA=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n")
}

fn write_pop_local(index: u16) -> String {
  format!("// pop local {index}\n@LCL\nD=M\n@{index}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n\n")
}