use std::io::Result;
use rand;
use crate::parser::{CommandType, MemorySegment};
use crate::asm;
// use crate::errors::HVMError;

static STATIC_COUNT: u16 = 16;


// Writes to the output file the assembly code that implements the given arithmetic-logical command.
pub fn write_arithmetic(command: String) -> Result<String> {
  match command.as_str() {
      "add" => Ok(format!("{}", asm::ADD)),
      "sub" => Ok(format!("{}", asm::SUB)),
      "neg" => Ok(format!("{}", asm::NEG)),
      "and" => Ok(format!("{}", asm::AND)),
      "or" => Ok(format!("{}", asm::OR)),
      "not" => Ok(format!("{}", asm::NOT)),
      "eq" => {
        let count1 = rand::random_range(1000..=99999);
        let count2 = rand::random_range(1000..=99999);
        Ok(format!("// eq\n@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@COUNT_{count1}\nD;JEQ\nD=0\n@COUNT_{count2}\n0;JMP\n(COUNT_{count1})\nD=-1\n(COUNT_{count2})\n@SP\nA=M-1\nM=D\n\n"))
      },
      "gt" => {
        let count1 = rand::random_range(1000..=99999);
        let count2 = rand::random_range(1000..=99999);
        Ok(format!("// eq\n@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@COUNT_{count1}\nD;JGT\nD=0\n@COUNT_{count2}\n0;JMP\n(COUNT_{count1})\nD=-1\n(COUNT_{count2})\n@SP\nA=M-1\nM=D\n\n"))
      },
      "lt" => {
        let count1 = rand::random_range(1000..=99999);
        let count2 = rand::random_range(1000..=99999);
        Ok(format!("// eq\n@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@COUNT_{count1}\nD;JLT\nD=0\n@COUNT_{count2}\n0;JMP\n(COUNT_{count1})\nD=-1\n(COUNT_{count2})\n@SP\nA=M-1\nM=D\n\n"))
      },
      _ => panic!("Unknow arithmetic command")
      }
}

// Write to the output file the assembly code that implements the given push or pop command.
pub fn write_push_pop(command: CommandType, filename: &str) -> Result<String> {
  match command {
      CommandType::Push(segment, index) => {
        let code = match segment {
            MemorySegment::Argument => write_push(index, "ARG", "argument"),
            MemorySegment::Local => write_push(index, "LCL", "local"),
            MemorySegment::Temp => write_push(index, "TEMP", "temp"),
            MemorySegment::That => write_push(index, "THAT", "that"),
            MemorySegment::This => write_push(index, "THIS", "this"),
            MemorySegment::Static => write_push_static(filename, index),
            MemorySegment::Constant => write_push_constant(index),
            MemorySegment::Pointer => write_push_pointer(index),
        };
        return Ok(code);
      },
      CommandType::Pop(segment, index) => {
        let code = match segment {
          MemorySegment::Argument => write_pop(index, "ARG", "argument"),
          MemorySegment::Local => write_pop(index, "LCL", "local"),
          MemorySegment::Temp => write_pop(index, "TEMP", "temp"),
          MemorySegment::That => write_pop(index, "THAT", "that"),
          MemorySegment::This => write_pop(index, "THIS", "this"),
          MemorySegment::Static => write_pop_static(filename, index),
          MemorySegment::Pointer => write_pop_pointer(index),
          _ => panic!("Segment unknow!")
      };
      return Ok(code);
      },
      // _ => Err(HVMError::UnknownCommand("Unknow push/pop command")),
      _ => panic!("Unknow push/pop command")
  }
}


// TODO:
// Writes assembly code that effects the label command
fn write_label(label: String) {}

// TODO:
// Writes assembly code that effects the goto command
fn write_goto(label: String) {}

// TODO:
// Writes assembly code that effects the if-goto command
fn write_if(label: String) {}

// TODO:
// Writes assembly code that effects the function command
fn write_function(function_name: String, n_vars: u8) {}

// TODO:
// Writes assembly code that effects the call command
fn write_call(function_name: String, n_args: u8) {}

// TODO:
// Writes assembly code that effects the return command
fn write_return() {}



/* 

HELPERS 

*/
// Generic function to write push ARG, LCL, TEMP, THIS, THAT command
fn write_push(index: u16, label: &str, segment: &str) -> String {
  if label == "TEMP" {
    let count = index + 5;
    format!("// push {segment} {index}\n@{index}\nD=A\n@{count}\nD=M\n{}\n\n", asm::PUSH_X)
  } else {
    format!("// push {segment} {index}\n@{index}\nD=A\n@{label}\nA=D+M\nD=M\n{}\n\n", asm::PUSH_X)
  }
}

// Writes push STATIC command
fn write_push_static(filename: &str, index: u16) -> String {
  let count =  STATIC_COUNT + index;
  format!("// push static {index}\n@{index}\nD=A\n@{filename}.{index}\n@{count}\nD=M{}\n\n", asm::PUSH_X)
}

// Writes push CONSTANT command
fn write_push_constant(index: u16) -> String {
  format!("// push constant {index}\n@{index}\nD=A{}\n\n", asm::PUSH_X)
}

// Writes push POINTER command
fn write_push_pointer(index: u16) -> String {
  if index == 0 {
    format!("// push pointer {index}\n@THIS\nD=M\n{}\n\n", asm::PUSH_X)
  } else {
    format!("// push pointer {index}\n@THAT\nD=M\n{}\n\n", asm::PUSH_X)
  }
}

// Generic function to write pop ARG, LCL, TEMP, THIS, THAT command
fn write_pop(index: u16, label: &str, segment: &str) -> String {
  let increment = incremente_m(index);
  if label == "TEMP" {
    let count = index + 5;
    return format!("// pop {segment} {index}\n{}\n@{count}\nM=D\n\n", asm::POP_Y)  
  } else {
    return format!("// pop {segment} {index}{}\n@{label}\n{}M=D\n\n", asm::POP_Y, increment)
  }
}

// Writes pop STATIC command
fn write_pop_static(filename: &str, index: u16) -> String {
  let count =  STATIC_COUNT + index;
  format!("// pop static {index}{}\n@{filename}.{index}\n@{count}\nM=D\n\n", asm::POP_Y)
}

// Writes pop POINTER command
fn write_pop_pointer(index: u16) -> String {
  if index == 0 {
    return format!("// pop pointer {index}\n{}\n@THIS\nM=D\n\n", asm::POP_Y);
  } else {
    return format!("// pop pointer {index}\n{}\n@THAT\nM=D\n\n", asm::POP_Y);
  }
}

// Get the memory segment 
fn incremente_m(index: u16) -> String {
  let mut increment = String::new();

  match index {
      0 => increment.push_str("A=M\n"),
      1 => increment.push_str("A=M+1\n"),
      2..=u16::MAX => {
        increment.push_str("A=M+1\n");
        for _ in 1..index {
          increment.push_str("A=A+1\n")
        }
      },
  }

  increment
}
