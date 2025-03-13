use crate::{code::*, parser::*, symbol_table::SymbolTable, utils::contains_alphabetic};
use std::fs::File;
use std::io::Write;

pub fn assemble(input: &str, output: &str) -> std::io::Result<()> {
    let instructions = parse_file(input)?;
    let mut symbol_table = SymbolTable::new();

    // First pass, records labels
    first_pass(&instructions, &mut symbol_table);

    // Second pass, translates into binary
    let mut file = File::create(output)?;
    for instruction in &instructions {
        let binary = match instruction {
            Instruction::A(value) => {
                let binary_string: String;
                // Check if value == @R0 or ADDRESS
                let is_alphabetic = contains_alphabetic(&value);
                if is_alphabetic == true {
                    let value = symbol_table.get_address(&value);
                    binary_string = format!("0{:015b}", value);
                } else {
                    binary_string = format!("0{:015b}", value.parse::<u16>().unwrap());
                }
                binary_string
            }
            Instruction::C { dest, comp, jump } => {
                format!(
                    "111{}{}{}",
                    comp_to_bin(&comp),
                    dest_to_bin(&dest),
                    jump_to_bin(&jump)
                )
            }
            _ => continue,
        };
        writeln!(file, "{}", binary)?;
    }

    Ok(())
}

fn first_pass(instructions: &Vec<Instruction>, symbol_table: &mut SymbolTable) {
  let mut count: u16 = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Label(label) => {
                symbol_table.add_label(label, count);
            },
            _ => count += 1,
        };
    }
}
