use crate::{code_writer, parser};

// This is the main program that drives the translation process, using the services of a Parser and a CodeWriter.
pub fn vmtranslator(arguments: Vec<String>) {
  let file = &arguments[1];
  // It constructs a Parser
  let new_parser = parser::Parser::new(file).unwrap();

  // And constructs an output file, Prog.asm
  let output_file = code_writer::CodeWriter::new(file).unwrap();

  // Loop: iteartes through the VM commands in the input file
  while new_parser.has_more_lines() == true {
      // parser.commands[index]
      // get args1
      // get args2
      // command_type(args1)
      // if C_arithmetic -> code_writer.write_arithmetic
      // if C_PUSH or C_POP -> code_writer.write_push_pop
      // parser.advance
  }
}