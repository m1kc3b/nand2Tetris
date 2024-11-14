use std::io;

use crate::{code_writer, parser};
use crate::parser::CommandType;

// This is the main program that drives the translation process, using the services of a Parser and a CodeWriter.
pub fn vmtranslator(arguments: Vec<String>) {
  let file = &arguments[1];
  // It constructs a Parser
  let mut parser = parser::Parser::new(file).unwrap();

  // And constructs an output file, Prog.asm
  let mut output_file = code_writer::CodeWriter::new(file).unwrap();

  // Loop: iteartes through the VM commands in the input file
  while parser.has_more_lines() == true {
      let command_type = parser.command_type();

      match command_type {
        Some(CommandType::C_PUSH | CommandType::C_POP) => {
          let segment = parser.arg1();
          let index = parser.arg2();
          output_file.write_push_pop(command_type, &segment, index);
        },
        Some(CommandType::C_ARITHMETIC) => {
          let command = parser.commands[parser.index].split(" ").collect()[0];
          output_file.write_arithmetic(&command);
        },
        _ => parser.advance(),
      }
  }
}
