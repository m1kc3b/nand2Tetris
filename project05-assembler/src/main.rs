use project05_assembler::parser::Parser;
use std::env;
use std::fs;
use std::process;

fn main() {
    // $ HackAssembler Add.asm
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No files provided!");
        process::exit(1);
    }
    // Open file
    let input = fs::read_to_string(&args[1])
        .unwrap_or_else(|err| format!("An error occured with the file: {}", err));
    // Create a new Parser
    let parser = Parser::new(input);
    
}
