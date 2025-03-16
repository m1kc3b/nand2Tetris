use std::{fs::File, path::PathBuf};
use std::io::{self, BufRead};

const ARITHMETIC_COMMANDS: [&str; 9] = ["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"];

pub enum MemorySegment {
  Local,
  Argument,
  This,
  That,
  Temp,
  Pointer,
  Static,
  Constant,
}

pub enum CommandType {
    Arithmetic(String), // command
    Push(MemorySegment, u16), // arg1, arg2
    Pop(MemorySegment, u16), // arg1, arg2
    // Label(String), // label
    // Goto(String), // label
    // If(String), // label
    // Function(String, u8), // functionName, nVars
    // Call(String, u8), // functionName, nArgs
    // Return,
}

// Parses a given file into a Vec<CommandType>
pub fn parse_file(files: Vec<PathBuf>) -> io::Result<Vec<CommandType>> {
  let mut commands = Vec::new();

  
  for file in files {
    let f = File::open(file)?;
    let reader = io::BufReader::new(f);

  for line in reader.lines() {
      let line = line?;
      let line = line.split("//").next().unwrap().trim().to_string(); // Remove comments

      
      if line.is_empty() {
          continue;
      } else if ARITHMETIC_COMMANDS.contains(&line.as_str()) {
        commands.push(CommandType::Arithmetic(line));
      } else if line.starts_with("push") {
        let (segment, index) = get_args(&line);
        commands.push(CommandType::Push(segment, index));
      } else if line.starts_with("pop") {
        let (segment, index) = get_args(&line);
        commands.push(CommandType::Pop(segment, index));
      }
      // TODO: impl remaining CommandType:
      // Label
      // Goto
      // If
      // Function
      // Return
      // Call 
  }
  }

  Ok(commands)
}


/// Get arguments
/// example: push constant 7
/// returns (MemorySegment::Constant, 7)
fn get_args(line: &str) -> (MemorySegment, u16) {
  let args: Vec<&str> = line.split_whitespace().collect();
  let segment = match args[1] {
      "local" => MemorySegment::Local,
      "argument" =>MemorySegment::Argument,
      "this" => MemorySegment::This,
      "that" => MemorySegment::That,
      "temp" => MemorySegment::Temp,
      "pointer" => MemorySegment::Pointer,
      "static" => MemorySegment::Static,
      "constant" => MemorySegment::Constant,
      _ => panic!("Segment unknow!"),
  };

  (segment, args[2].parse::<u16>().unwrap())
}