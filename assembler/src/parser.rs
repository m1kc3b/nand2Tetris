use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum Instruction {
    A(String), // @value
    C { dest: Option<String>, comp: String, jump: Option<String> }, // dest=comp;jump
    Label(String), // (LABEL)
}

pub fn parse_file(filename: &str) -> io::Result<Vec<Instruction>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut instructions = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.split("//").next().unwrap().trim(); // Remove comments

        if line.is_empty() {
            continue;
        } else if line.starts_with('@') {
            instructions.push(Instruction::A(line[1..].to_string()));
        } else if line.starts_with('(') && line.ends_with(')') {
            instructions.push(Instruction::Label(line[1..line.len() - 1].to_string()));
        } else {
            let mut parts = line.split('=');
            let (dest, comp_jump) = if let Some(d) = parts.next() {
                if let Some(cj) = parts.next() {
                    (Some(d.to_string()), cj)
                } else {
                    (None, d)
                }
            } else {
                (None, line)
            };

            let mut parts = comp_jump.split(';');
            let comp = parts.next().unwrap().to_string();
            let jump = parts.next().map(|j| j.to_string());

            instructions.push(Instruction::C { dest, comp, jump });
        }
    }
    Ok(instructions)
}
