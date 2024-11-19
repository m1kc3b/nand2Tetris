use std::env;
use std::fs;
use std::process;

fn main() {
    // $ HackAssembler Prog.asm
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No files provided!");
        process::exit(1);
    }
    // Open file
    let file = fs::read_to_string(&args[1]).unwrap_or_else(|err| {
        format!("An error occured with the file: {}", err)
});
    println!("{}", file);
    // parser(file)
}
