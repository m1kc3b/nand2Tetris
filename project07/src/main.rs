use std::env;

use vm_translator::vmtranslator;

mod code_writer;
mod parser;
mod stack;
mod vm_translator;

fn main() {
    // The program gets the name of the input source file, say Prog.vm, from the command-line argument.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Aucun fichier fourni !");
        std::process::exit(1);
    }
    vmtranslator(args);
}
