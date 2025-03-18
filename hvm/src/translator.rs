use crate::code::{
    write_bootstrap, write_call, write_function, write_goto, write_if, write_label, write_return,
};
use crate::{
    code::{write_arithmetic, write_push_pop},
    parser::{CommandType, parse_file},
};
use std::io::Write;
use std::path::Path;
use std::fs;
use std::{fs::File, path::PathBuf};

pub fn translate(input: &str) -> std::io::Result<()> {

    let (files, mut output, output_name) = handle_path(input)?;

    println!("__FILES__: {:?}", files);

    // Getting all instructions from the parser
    let instructions = parse_file(files)?;

    // Adding bootstrap code
    let bootstrap_code = write_bootstrap()?;
    write!(output, "{}", bootstrap_code)?;

    // Translate instructions into asm commands
    for instruction in instructions {
        let code: String = match instruction {
            CommandType::NewFile(filename) => {
                format!("// {}\n", filename)
            }
            CommandType::Arithmetic(command) => {
                let cmd = write_arithmetic(&command)?;
                format!("{}", cmd)
            }
            CommandType::Push(_, _) => {
                let cmd = write_push_pop(&instruction, output_name.as_str())?;
                format!("{}", cmd)
            }
            CommandType::Pop(_, _) => {
                let cmd = write_push_pop(&instruction, output_name.as_str())?;
                format!("{}", cmd)
            }
            CommandType::Call(function_name, nargs) => {
                let cmd = write_call(&function_name, nargs)?;
                format!("{}", cmd)
            }
            CommandType::Function(function_name, nvars) => {
                let cmd = write_function(&function_name, nvars)?;
                format!("{}", cmd)
            }
            CommandType::Goto(label) => {
                let cmd = write_goto(&label)?;
                format!("{}", cmd)
            }
            CommandType::If(label) => {
                let cmd = write_if(&label)?;
                format!("{}", cmd)
            }
            CommandType::Label(label) => {
                let cmd = write_label(&label)?;
                format!("{}", cmd)
            }
            CommandType::Return => {
                let cmd = write_return()?;
                format!("{}", cmd)
            }
        };
        write!(output, "{}", code)?;
    }

    // Adding an infinite end loop at the end of the file
    // let _ = write!(output, "{}", format!("// End\n(END)\n@END\n0;JMP\n"));

    Ok(())
}

// Handles path
fn handle_path(input: &str) -> std::io::Result<(Vec<PathBuf>, File, String)> {
    let path = Path::new(input);

    let output_name = path
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("default")
        .to_string();

    let output_path = if path.is_dir() {
        let filename = path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("output");
        path.join(format!("{}.asm", filename))
    } else {
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        let file_stem = path
            .file_stem()
            .and_then(|f| f.to_str())
            .unwrap_or("output");
        parent.join(format!("{}.asm", file_stem))
    };

    let output = File::create(&output_path)?;

    let mut files: Vec<PathBuf> = Vec::new();

    // Check if path is a file
    if path.is_file() {
      if let Some(ext) = path.extension() {
          if ext == "vm" {
              files.push(path.to_path_buf());
          }
      }
  }

    // Check if path is a folder
    if path.is_dir() {
      for entry in fs::read_dir(path)? {
          let file_path = entry?.path();
          if file_path.is_file() {
              if let Some(ext) = file_path.extension() {
                  if ext == "vm" {
                      files.push(file_path);
                  }
              }
          }
      }
  }

    Ok((files, output, output_name))
}
