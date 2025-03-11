use std::fs::File;
use std::io::Write;
use crate::{code::{write_arithmetic, write_push_pop}, parser::{parse_file, CommandType}};

pub fn translate(input: &str, output: &str) -> std::io::Result<()> {
  let instructions = parse_file(input)?;
  let mut file = File::create(output)?;

  for instruction in instructions {
    let code: String = match instruction {
        CommandType::Arithmetic(command) => {
          let cmd = write_arithmetic(command)?;
          format!("{}", cmd)
        },
        CommandType::Push(segment, index) => {
          let cmd = write_push_pop(CommandType::Push(segment, index))?;
          format!("{}", cmd)
        },
        CommandType::Pop(segment, index) => {
          let cmd = write_push_pop(CommandType::Pop(segment, index))?;
          format!("{}", cmd)
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
    write!(file, "{}", code)?;
  }
  
  Ok(())
}