use std::{env, fs::{self, read_to_string}, io, path::{self, Path}};

use parser::Parser;
mod parser;
mod code_writer;
mod stack;
mod memory_segment;

fn main() -> Result<(), io::Error> {
    // The program gets the name of the input source file, say Prog (.vm is mandatory), from the command-line argument.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Aucun fichier fourni !");
        std::process::exit(1);
    }

    /*
    TODO:
    // Read the file from data directory or from the path provided
    // Create output file with name of the file
    // Loop over each line of the file
    // parse each line
    // translate each parsed line
    // write command into the output file (with comments)
     */



    let commands = fs::read_to_string(&args[1])?;

    for command in commands.lines() {
        let parser = Parser::new(command).parse();
        // let code_writer = CodeWriter::new(parser);

    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn should_open_file_based_on_the_path_provided() {
        let file = "data/BasicTest";
        let path;

        if file.contains("/") {
            path = format!("{}.vm", file);
        } else {
            path = format!("data/{}.vm", file);
        }


        let content = match read_to_string(path) {
            Ok(c) => c,
            Err(err) => format!("{}", err),
        };

        assert_eq!(
            content,
            read_to_string("data/BasicTest.vm").unwrap()
        )
    }

    #[test]
    fn should_create_a_vm_translator() {
        #[derive(Debug, PartialEq, Eq)]
        struct VMTranslator {
            input_file: String,
            output_file: String,
        }

        impl VMTranslator {
            fn new(input_file: &str) -> Self {
                let input: String;
                let output: String;

                if input_file.contains("/") {
                    let path: Vec<&str> = input_file.split("/").collect();
                    input = format!("{}.vm", &input_file);
                    output = format!("data/{}.asm", path[1]);
                } else {
                    input = format!("data/{}.vm", &input_file);
                    output = format!("data/{}.asm", &input_file);
                }

                Self { input_file: input, output_file: output }
            }
        }

        let input_file = "data/BasicTest";
        let translator = VMTranslator::new(input_file);

        let translator_test = VMTranslator {
            input_file: "data/BasicTest.vm".to_string(),
            output_file: "data/BasicTest.asm".to_string(),
        };

        assert_eq!(translator, translator_test);   
    }
}
