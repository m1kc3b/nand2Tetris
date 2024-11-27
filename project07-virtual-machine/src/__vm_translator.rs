use crate::__code_writer::CodeWriter;
use crate::__parser::Parser;
use crate::__parser::CommandType;

// This is the main program that drives the translation process, using the services of a Parser and a CodeWriter.
pub fn vmtranslator(arguments: Vec<String>) {
  let file = &arguments[1];
  // It constructs a Parser
  let mut parser = Parser::new(file).unwrap();

  // And constructs an output file, Prog.asm
  let mut code_writer = CodeWriter::new(file).unwrap();

  // Loop: iteartes through the VM commands in the input file
  while parser.has_more_lines() == true {
      let command_type = parser.command_type();

      match command_type {
        Some(CommandType::C_PUSH | CommandType::C_POP) => {
          let segment = parser.arg1().unwrap();                     // TODO: Handling Result
          let index = parser.arg2().unwrap();                        // TODO: Handling Result
          code_writer.write_push_pop(command_type, &segment, index);      // Handle command line
          parser.advance();                                               // Increment index to read next line
          continue;                                                       // Move to the next loop
        },
        Some(CommandType::C_ARITHMETIC) => {
          let command: Vec<&str> = parser.commands[parser.index].split_whitespace().collect();     // TODO: Handling Result
          code_writer.write_arithmetic(&command[0]);                                       // Handle command line
          parser.advance();                                                                         // Next line
          continue;                                                                                 // Next loop
        },
        _ => {
          parser.advance(); // Next line
          continue;         // Next loop
        }
      }
  }
}
