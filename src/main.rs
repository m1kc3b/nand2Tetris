use std::env;
use std::process;

// use hack_assembler::assemble;

fn main() {
    // $ HackAssembler Add.asm
    let args: Vec<String> = env::args().collect();
    // Check filename is provided
    if args.len() < 2 {
        eprintln!("No files provided!");
        process::exit(1);
    }
    // Filename
    let filename = &args[1];
    // Create a HackAssembler
    // let assembler = HackAssembler::new(filename);
    // assembler.execute();
}
