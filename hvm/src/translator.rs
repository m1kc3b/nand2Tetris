use std::{fs::File, path::PathBuf};
use std::io::Write;
use std::path::Path;
use crate::code::{write_call, write_function, write_goto, write_if, write_label, write_return};
use crate::{code::{write_arithmetic, write_push_pop}, parser::{parse_file, CommandType}};

pub fn translate(input: &str) -> std::io::Result<()> {
  let path = Path::new(input);
  let output_name = path.file_name().and_then(|name| name.to_str()).unwrap_or("default");
  let output_path = PathBuf::from(output_name).with_extension("asm");
  let mut output = File::create(output_path)?;
  
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
          let cmd = write_push_pop(CommandType::Push(segment, index), output_name)?;
          format!("{}", cmd)
        },
        CommandType::Pop(segment, index) => {
          let cmd = write_push_pop(CommandType::Pop(segment, index), output_name)?;
          format!("{}", cmd)
        },
        CommandType::Call(function_name, nargs) => {
          let cmd = write_call(&function_name, nargs)?;
          format!("{}", cmd)
        },
        CommandType::Function(function_name, nvars) => {
          let cmd = write_function(&function_name, nvars)?;
          format!("{}", cmd)
        },
        CommandType::Goto(label) => {
          let cmd = write_goto(&label)?;
          format!("{}", cmd)
        },
        CommandType::If(label) => {
          let cmd = write_if(&label)?;
          format!("{}", cmd)
        },
        CommandType::Label(label) => {
          let cmd = write_label(&label)?;
          format!("{}", cmd)
        },
        CommandType::Return => {
          let cmd = write_return()?;
          format!("{}", cmd)
        },
    };
    write!(output, "{}", code)?;
  }

  // Adding an infinite end loop at the end of the file
  // let _ = write!(output, "{}", format!("// End\n(END)\n@END\n0;JMP\n"));
  
  Ok(())
}
