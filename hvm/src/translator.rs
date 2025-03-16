use std::{fs::File, path::PathBuf};
use std::io::Write;
use std::path::Path;
use crate::{code::{write_arithmetic, write_push_pop}, parser::{parse_file, CommandType}};

pub fn translate(input: &str) -> std::io::Result<()> {
  let path = Path::new(input);
  let output_name = path.file_name();
  let filename = output_name.and_then(|name| name.to_str()).unwrap();
  let mut output = File::create(output_name.unwrap())?;
  let mut files: Vec<PathBuf> = Vec::new();

  // Check if path is a file
  if path.is_file() == true {
      let file = path.to_path_buf();
      files.push(file);
  }
  // Check if path is a folder
  if path.is_dir() == true {
      for entry in path.read_dir().unwrap() {
        let file = entry.unwrap().path();
        files.push(file);
      }
  }

  // Getting all instructions from the parser
  let instructions = parse_file(files)?;
  
  // Translate instructions into asm commands
  for instruction in instructions {
    let code: String = match instruction {
        CommandType::Arithmetic(command) => {
          let cmd = write_arithmetic(command)?;
          format!("{}", cmd)
        },
        CommandType::Push(segment, index) => {
          let cmd = write_push_pop(CommandType::Push(segment, index), filename)?;
          format!("{}", cmd)
        },
        CommandType::Pop(segment, index) => {
          let cmd = write_push_pop(CommandType::Pop(segment, index), filename)?;
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
    write!(output, "{}", code)?;
  }

  // Adding an infinite end loop at the end of the file
  let _ = write!(output, "{}", format!("// End\n(END)\n@END\n0;JMP\n"));
  
  Ok(())
}
