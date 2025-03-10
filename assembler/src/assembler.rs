use std::fs::File;
use std::io::Write;
use crate::{parser::*, code::*, symbol_table::SymbolTable};

pub fn assemble(input: &str, output: &str) -> std::io::Result<()> {
  let instructions = parse_file(input)?;
  
  // First pass, records labels
  let mut sym_table = SymbolTable::new();
  for instruction in &instructions {
    match instruction {
        Instruction::Label(label) => {
          sym_table.add_label(label, sym_table.get_last_ram_address());
        },
        _ => continue
    };
  }
  
  // Second pass, translates into binary
    let mut file = File::create(output)?;
    for instruction in &instructions {
        let binary = match instruction {
            Instruction::A(value) => format!("0{:015b}", value.parse::<u16>().unwrap()),
            Instruction::C { dest, comp, jump } => {
                format!("111{}{}{}", comp_to_bin(&comp), dest_to_bin(&dest), jump_to_bin(&jump))
            },
            Instruction::Label(symbol) => {
              // get_address
              let value = sym_table.get_address(symbol);
              // translates
              format!("0{:015b}", value)
            },
        };
        writeln!(file, "{}", binary)?;
    }
    Ok(())
}
