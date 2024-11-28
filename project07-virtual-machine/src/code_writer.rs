use crate::parser::CommandType;

fn write_push_pop(
    command: &Option<CommandType>,
    segment: Option<&str>,
    index: Option<i16>,
) -> String {
    let mut output_string: String;
    match command {
        &Some(CommandType::C_push) => {
            let pust_to_stack = format!("@SP\nA=M\nM=D\n@SP\nM=M+1");
            match segment {
                Some("constant") => {
                    output_string = format!("@{}\nD=A\n{}", index.unwrap(), pust_to_stack);
                }
                Some("local") => {
                    output_string = format!(
                        "@{}\nD=A\n@LCL\nA=M+D\nM=A\nD=M\n{}",
                        index.unwrap(),
                        pust_to_stack
                    );
                }
                Some("argument") => {
                    output_string = format!(
                        "@{}\nD=A\n@ARG\nA=M+D\nM=A\nD=M\n{}",
                        index.unwrap(),
                        pust_to_stack
                    );
                }
                Some("this") => {
                    output_string = format!(
                        "@{}\nD=A\n@THIS\nA=M+D\nM=A\nD=M\n{}",
                        index.unwrap(),
                        pust_to_stack
                    );
                }
                Some("that") => {
                    output_string = format!(
                        "@{}\nD=A\n@THAT\nA=M+D\nM=A\nD=M\n{}",
                        index.unwrap(),
                        pust_to_stack
                    );
                }
                Some("pointer") => {
                    if index.unwrap() == 0 {
                        output_string = format!("@THIS\nA=M+D\nM=A\nD=M\n{}", pust_to_stack);
                    } else {
                        output_string = format!("@THAT\nA=M+D\nM=A\nD=M\n{}", pust_to_stack);
                    }
                }
                Some("temp") => {
                    output_string = format!(
                        "@{}\nD=A\n@TEMP\nA=M+D\nM=A\nD=M\n{}",
                        index.unwrap(),
                        pust_to_stack
                    );
                }
                Some("static") => {
                    output_string = format!("Foo.{}\n{}", index.unwrap(), pust_to_stack);
                }
                _ => output_string = format!("ERROR"),
            }
        }
        _ => {
          
        }
    }
    output_string
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_translate_push_command_into_asm() {
        let mut output_string: String;
        let (command, segment, index) = (&Some(CommandType::C_push), Some("static"), Some(7));

        if let &Some(CommandType::C_push) = command {
            let pust_to_stack = format!("@SP\nA=M\nM=D\n@SP\nM=M+1");
            match segment {
                Some("constant") => {
                    output_string = format!("@{}\nD=A\n{}", index.unwrap(), pust_to_stack);
                }
                Some("local") => {
                    output_string = format!(
                        "@{}\nD=A\n@LCL\nA=M+D\nM=A\nD=M\n{}",
                        index.unwrap(),
                        pust_to_stack
                    );
                }
                Some("argument") => {
                    output_string = format!(
                        "@{}\nD=A\n@ARG\nA=M+D\nM=A\nD=M\n{}",
                        index.unwrap(),
                        pust_to_stack
                    );
                }
                Some("this") => {
                    output_string = format!(
                        "@{}\nD=A\n@THIS\nA=M+D\nM=A\nD=M\n{}",
                        index.unwrap(),
                        pust_to_stack
                    );
                }
                Some("that") => {
                    output_string = format!(
                        "@{}\nD=A\n@THAT\nA=M+D\nM=A\nD=M\n{}",
                        index.unwrap(),
                        pust_to_stack
                    );
                }
                Some("pointer") => {
                    if index.unwrap() == 0 {
                        output_string = format!("@THIS\nA=M+D\nM=A\nD=M\n{}", pust_to_stack);
                    } else {
                        output_string = format!("@THAT\nA=M+D\nM=A\nD=M\n{}", pust_to_stack);
                    }
                }
                Some("temp") => {
                    output_string = format!(
                        "@{}\nD=A\n@TEMP\nA=M+D\nM=A\nD=M\n{}",
                        index.unwrap(),
                        pust_to_stack
                    );
                }
                Some("static") => {
                    output_string = format!("Foo.{}\n{}", index.unwrap(), pust_to_stack);
                }
                _ => output_string = format!("ERROR"),
            }
        } else {
            output_string = format!("None");
        }

        // assert_eq!(output_string, "@7\nD=A\n@TEMP\nA=M+D\nM=A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1".to_string());
        assert_eq!(
            output_string,
            "Foo.7\n@SP\nA=M\nM=D\n@SP\nM=M+1".to_string()
        );
    }
}
