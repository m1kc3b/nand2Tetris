use std::fs::File;
use std::io::Write;
use crate::{code::{write_arithmetic, write_push_pop}, parser::{parse_file, CommandType}};

pub fn translate(input: &str, output: &str) -> std::io::Result<()> {
  let instructions = parse_file(input)?;
  let mut file = File::create(output)?;
  
  let name: Vec<&str> = input.split("/").collect();
  let filename: Vec<&str> = name[1].split('.').collect();

  for instruction in instructions {
    let code: String = match instruction {
        CommandType::Arithmetic(command) => {
          let cmd = write_arithmetic(command)?;
          format!("{}", cmd)
        },
        CommandType::Push(segment, index) => {
          let cmd = write_push_pop(CommandType::Push(segment, index), filename[0])?;
          format!("{}", cmd)
        },
        CommandType::Pop(segment, index) => {
          let cmd = write_push_pop(CommandType::Pop(segment, index), filename[0])?;
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

  write!(file, "{}", write_end_inifinite_loop());
  
  Ok(())
}

fn write_end_inifinite_loop() -> String {
  format!("// End\n(END)\n0;JMP\n")
}