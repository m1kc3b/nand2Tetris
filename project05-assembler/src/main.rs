use std::fs::File;

enum InstructionType {
    A_INSTRUCTION,
    C_INSTRUCTION,
    L_INSTRUCTION,
}

struct Parser {
    file: File,
}

impl Parser {
    fn new(&self, file: File) -> Self {
        Self { file }
    }

    fn has_more_lines(&self) -> bool {
        todo!()
    }

    fn advance(&self) {
        todo!()
    }

    fn instruction_type(&self) -> InstructionType {
        todo!()
    }

    fn symbol(&self) -> String {
        todo!()
    }

    fn dest(&self) -> String {
        todo!()
    }

    fn comp(&self) -> String {
        todo!()
    }

    fn jump(&self) -> String {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

}