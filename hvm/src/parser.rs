use std::io::{self, BufRead};
use std::{fs::File, path::PathBuf};

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
    Arithmetic(String),       // command
    Push(MemorySegment, u16), // arg1, arg2
    Pop(MemorySegment, u16),  // arg1, arg2
    Label(String, String),    // FunctionName, label
    Goto(String, String),     // FunctionName, label
    If(String, String),       // FunctionName, label
    Function(String, u8),     // functionName, nVars
    Call(String, u8),         // functionName, nArgs
    Return,
    NewFile(String),          // Filename
}

// Parses a given file into a Vec<CommandType>
pub fn parse_file(files: Vec<PathBuf>) -> io::Result<Vec<CommandType>> {
    let mut commands = Vec::new();

    for file in files {
        let f = File::open(&file)?;
        let reader = io::BufReader::new(f);

        let mut func_name = String::new();

        commands.push(CommandType::NewFile(get_file_name(&file)));

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
            } else if line.starts_with("function") {
                let (function_name, nvars) = get_function_name_and_nvars(&line);
                func_name = function_name.clone();
                commands.push(CommandType::Function(function_name, nvars));
            } else if line.starts_with("call") {
                let (function_name, nargs) = get_function_name_and_nvars(&line);
                commands.push(CommandType::Call(function_name, nargs));
            } else if line.starts_with("return") {
                commands.push(CommandType::Return);
            } else if line.starts_with("goto") {
                let label = handle_label(&line);
                commands.push(CommandType::Goto(func_name.clone(), label));
            } else if line.starts_with("if-goto") {
                let label = handle_label(&line);
                commands.push(CommandType::If(func_name.clone(), label));
            } else if line.starts_with("label") {
                let label = handle_label(&line);
                commands.push(CommandType::Label(func_name.clone(), label));
            }
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
        "argument" => MemorySegment::Argument,
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

// Get label
// example: label N_LT_2 
// Returns: N_LT_2 
fn handle_label(line: &str) -> String {
  let args: Vec<&str> = line.split_whitespace().collect();
  args[1].to_string()
}

// Get functionName and nVars
// Example: function Main.fibonacci 0
// returns (Main.fibonacci, O)
fn get_function_name_and_nvars(line: &str) -> (String, u8) {
  let args: Vec<&str> = line.split_whitespace().collect();
  let function_name = args[1].to_string();
  let nvars = args[2].parse::<u8>().unwrap();
  (function_name, nvars)
}

// Get Filename
fn get_file_name(file: &PathBuf) -> String {
    file.file_stem()
        .unwrap_or_else(|| file.as_os_str())
        .to_string_lossy()
        .into_owned()
}