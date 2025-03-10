use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
};

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionType {
    AInstruction,
    CInstruction,
    LInstruction,
}

#[derive(Debug)]
pub struct Parser {
    lines: Lines<BufReader<File>>,
    line_count: usize,
}

impl Parser {
    pub fn new(filename: &str) -> Result<Self> {
        let path = format!("asm-files/{}", &filename);
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(Self {
            lines: reader.lines(),
            line_count: 0,
        })
    }

    pub fn get_line_count(&self) -> Option<usize> {
        Some(self.line_count)
    }

    // pub fn has_more_lines(&self) -> bool {
    //     let lines: Vec<&str> = self.input.split("\n").collect();
    //     lines.len() > self.index
    // }

    pub fn advance(&mut self) -> Option<Result<String>> {
        while let Some(line) = self.lines.next() {
            match line {
                Ok(content) => {
                    let trimmed = content.trim();
                    if !trimmed.is_empty() & !trimmed.starts_with("//") {
                        self.line_count += 1;
                        // if trimmed.starts_with("(") {
                        //     self.line_count += 1;
                        // }
                        return Some(Ok(content.trim().to_string()));
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
        None
    }

    pub fn reinitialize_lines(&mut self, filename: &str) -> Result<()> {
        let path = format!("asm-files/{}", &filename);
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        self.lines = reader.lines();
        self.line_count = 0;

        Ok(())
    }

    
    pub fn instruction_type(&self, line: &str) -> Option<InstructionType> {
        if line.starts_with("@") && is_not_uppercase(line) {
            return Some(InstructionType::AInstruction);
        } else if line.starts_with("(") {
            return Some(InstructionType::LInstruction);
        } else {
            return Some(InstructionType::CInstruction);
        }
    }
    
    pub fn symbol(&self, line: String) -> Option<String> {
        let instruction_type = self.instruction_type(&line);
        match instruction_type {
            Some(InstructionType::AInstruction) => Some(line[1..].to_string()),
            Some(InstructionType::LInstruction) => Some(line[1..line.len() - 1].to_string()),
            _ => None,
        }
    }
    
    pub fn dest(&self, line: &str) -> Option<&str> {
        let instruction_type = self.instruction_type(&line);
        if let Some(InstructionType::CInstruction) = instruction_type {
            // check if "="
            if line.contains("=") {
                let instruction: Vec<&str> = line.split("=").collect();
                match instruction[0] {
                    "M" => return Some("001"),
                    "D" => return Some("010"),
                    "DM" => return Some("011"),
                    "A" => return Some("100"),
                    "AM" => return Some("101"),
                    "AD" => return Some("110"),
                    "ADM" => return Some("111"),
                    _ => return Some("000"),
                }
            }
            return Some("000");
        }
        None
    }
    
    pub fn comp(&self, line: &str) -> Option<&str> {
        let instruction_type = self.instruction_type(&line);
        if let Some(InstructionType::CInstruction) = instruction_type {
            if line.contains("=") {
                let instruction: Vec<&str> = line.split("=").collect();
                match instruction[1] {
                    "0" => return Some("0101010"),
                    "1" => return Some("0111111"),
                    "-1" => return Some("0111010"),
                    "D" => return Some("0001100"),
                    "A" => return Some("0110000"),
                    "M" => return Some("1110000"),
                    "!D" => return Some("0001101"),
                    "!A" => return Some("0110001"),
                    "!M" => return Some("1110001"),
                    "-D" => return Some("0001111"),
                    "-A" => return Some("0110011"),
                    "-M" => return Some("1110011"),
                    "D+1" => return Some("0011111"),
                    "A+1" => return Some("0110111"),
                    "M+1" => return Some("1110111"),
                    "D-1" => return Some("0001110"),
                    "A-1" => return Some("0110010"),
                    "M-1" => return Some("1110010"),
                    "D+A" => return Some("0000010"),
                    "D+M" => return Some("1000010"),
                    "D-A" => return Some("0010011"),
                    "D-M" => return Some("1010011"),
                    "A-D" => return Some("0000111"),
                    "M-D" => return Some("1000111"),
                    "D&A" => return Some("0000000"),
                    "D&M" => return Some("1000000"),
                    "D|A" => return Some("0010101"),
                    "D|M" => return Some("1010101"),
                    _ => return None,
                }
            }
            if line.contains(";") {
                let instruction: Vec<&str> = line.split("=").collect();
                match instruction[0] {
                    "0" => return Some("0101010"),
                    "1" => return Some("0111111"),
                    "-1" => return Some("0111010"),
                    "D" => return Some("0001100"),
                    "A" => return Some("0110000"),
                    "M" => return Some("1110000"),
                    "!D" => return Some("0001101"),
                    "!A" => return Some("0110001"),
                    "!M" => return Some("1110001"),
                    "-D" => return Some("0001111"),
                    "-A" => return Some("0110011"),
                    "-M" => return Some("1110011"),
                    "D+1" => return Some("0011111"),
                    "A+1" => return Some("0110111"),
                    "M+1" => return Some("1110111"),
                    "D-1" => return Some("0001110"),
                    "A-1" => return Some("0110010"),
                    "M-1" => return Some("1110010"),
                    "D+A" => return Some("0000010"),
                    "D+M" => return Some("1000010"),
                    "D-A" => return Some("0010011"),
                    "D-M" => return Some("1010011"),
                    "A-D" => return Some("0000111"),
                    "M-D" => return Some("1000111"),
                    "D&A" => return Some("0000000"),
                    "D&M" => return Some("1000000"),
                    "D|A" => return Some("0010101"),
                    "D|M" => return Some("1010101"),
                    _ => return None,
                }
            }
        }
        None
    }
    
    pub fn jump(&self, line: &str) -> Option<&str> {
        let instruction_type = self.instruction_type(&line);
        if let Some(InstructionType::CInstruction) = instruction_type {
            // check if contains ";"
            if line.contains(";") {
                let instruction: Vec<&str> = line.split(";").collect();
                match instruction[1] {
                    "JGT" => return Some("001"),
                    "JEQ" => return Some("010"),
                    "JGE" => return Some("011"),
                    "JLT" => return Some("100"),
                    "JNE" => return Some("101"),
                    "JLE" => return Some("110"),
                    "JMP" => return Some("111"),
                    _ => return Some("000"),
                }
            }
            return Some("000")
        }
        None
    }
}

fn is_not_uppercase(s: &str) -> bool {
    s.chars().any(|c| c.is_lowercase())
}


#[cfg(test)]
mod tests {

    use super::*;

    
}
