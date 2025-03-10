use std::fs::File;
use std::io::Write;
use crate::{code::{writeArithmetic, writePushPop}, parser::{parse_file, CommandType}};

pub fn translate(input: &str, output: &str) -> std::io::Result<()> {
  let instructions = parse_file(input)?;
  let mut file = File::create(output)?;

  for instruction in instructions {
    let binary: String = match instruction {
        CommandType::Arithmetic(command) => {
          let cmd = writeArithmetic(command);
          format!("")
        },
        CommandType::Push(arg1, arg2) => {
          let _ = writePushPop(CommandType::Push(arg1, arg2));
          format!("")
        },
        CommandType::Pop(arg1, arg2) => {
          let _ = writePushPop(CommandType::Pop(arg1, arg2));
          format!("")
        },
        // CommandType::Call(arg1, arg2) => {
        //   format!("")
        // },
        // CommandType::Function(arg1, arg2) => {
        //   format!("")
        // },
        // CommandType::Goto(arg1) => {
        //   format!("")
        // },
        // CommandType::If(arg1) => {
        //   format!("")
        // },
        // CommandType::Label(arg1) => {
        //   format!("")
        // },
        // CommandType::Return => {
        //   format!("")
        // },
    };
    write!(file, "{}", binary)?;
  }
  
  Ok(())
}