use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const ARITHMETIC_COMMANDS: [&str; 9] = ["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"];

pub enum CommandType {
    Arithmetic(String), // command
    Push(String, String), // arg1, arg2
    Pop(String, String), // arg1, arg2
    Label(String), // arg1
    Goto(String), // arg1
    If(String), // arg1
    Function(String, String), // arg1, arg2
    Return,
    Call(String, String), // arg1, arg2
}

pub fn parse_file(filename: &str) -> io::Result<Vec<CommandType>> {
  let path = Path::new(filename);
  let file = File::open(&path)?;
  let reader = io::BufReader::new(file);
  let mut instructions = Vec::new();

  for line in reader.lines() {
      let line = line?;
      let line = line.split("//").next().unwrap().trim().to_string(); // Remove comments

      
      if line.is_empty() {
          continue;
      } else if ARITHMETIC_COMMANDS.contains(&line.as_str()) {
        instructions.push(CommandType::Arithmetic(line));
      } else if line.starts_with("push") {
        let (arg1, arg2) = get_args(&line);
        instructions.push(CommandType::Push(arg1, arg2));
      } else if line.starts_with("pop") {
        let (arg1, arg2) = get_args(&line);
        instructions.push(CommandType::Pop(arg1, arg2));
      }
      // TODO: impl remaining CommandType:
      // Label
      // Goto
      // If
      // Function
      // Return
      // Call 
  }

  Ok(instructions)
}


/// Get arguments
/// example: push constant 7
/// returns ("push".to_string(), "7".to_string())
fn get_args(line: &str) -> (String, String) {
  let args: Vec<&str> = line.split_whitespace().collect();
  (args[1].to_string(), args[2].to_string())
}