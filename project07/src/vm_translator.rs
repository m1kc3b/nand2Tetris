use crate::{code_writer, parser};
use crate::parser::CommandType;

// This is the main program that drives the translation process, using the services of a Parser and a CodeWriter.
pub fn vmtranslator(arguments: Vec<String>) {
  let file = &arguments[1];
  // It constructs a Parser
  let mut new_parser = parser::Parser::new(file).unwrap();

  // And constructs an output file, Prog.asm
  let mut output_file = code_writer::CodeWriter::new(file).unwrap();

  // Loop: iteartes through the VM commands in the input file
  while new_parser.has_more_lines() == true {
      // commands[index]
      let command = &new_parser.commands[new_parser.index];
      // command_type
      let cmd_type = new_parser.command_type();
      // get args1
      let segment = new_parser.arg1();
      // get args2
      let index = new_parser.arg2();
      // if C_PUSH or C_POP -> code_writer.write_push_pop
      if let Some(CommandType::C_PUSH) | Some(CommandType::C_POP) = &cmd_type {
        output_file.write_push_pop(&command, &segment, index)
      }
      // if C_arithmetic -> code_writer.write_arithmetic
      // parser.advance
      output_file.write_arithmetic(command);
      new_parser.advance();
  }
}
