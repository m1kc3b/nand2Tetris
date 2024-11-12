mod parser;
mod code_writer;
mod stack;

fn main() {
    println!("Hello, world!");

}

// This is the main program that drives the translation process, using the services of a Parser and a CodeWriter.
fn vmtranslator() {
    // The program gets the name of the input source file, say Prog.vm, from the command-line argument.
    // It constructs a Parser
    // And constructs an output file, Prog.asm
    // Loop: iteartes through the VM commands in the input file

}