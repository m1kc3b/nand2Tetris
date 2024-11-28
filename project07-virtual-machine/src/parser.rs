#[derive(Debug, PartialEq, Eq)]
pub enum CommandType {
    C_arithmetic,
    C_push,
    C_pop,
    C_label,
    C_goto,
    C_if,
    C_function,
    C_return,
    C_call,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parser<'a> {
    command: &'a str,
    command_type: Option<CommandType>,
    arg1: Option<&'a str>,
    arg2: Option<i16>,
}

impl<'a> Parser<'a> {
    pub fn new(command: &'a str) -> Self {
        Self {
            command,
            command_type: None,
            arg1: None,
            arg2: None,
        }
    }

    pub fn parse(&mut self) -> (&Option<CommandType>, Option<&str>, Option<i16>) {
      let _ = self.get_command_type();
      let _ = self.get_arg1();
      let _ = self.get_arg2();

      
      (&self.command_type, self.arg1, self.arg2)
    }

    fn get_command_type(&mut self) {
        let splitted_cmd: Vec<&str> = self.command.split_whitespace().collect();
        match splitted_cmd[0] {
            "push" => self.command_type = Some(CommandType::C_push),
            "pop" => self.command_type = Some(CommandType::C_pop),
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
                self.command_type = Some(CommandType::C_arithmetic)
            }
            _ => self.command_type = None,
        }
    }

    fn get_arg1(&mut self) {
        let splitted_cmd: Vec<&str> = self.command.split_whitespace().collect();
        if splitted_cmd.len() > 1 {
            self.arg1 = Some(splitted_cmd[1]);
        }
    }

    fn get_arg2(&mut self) {
      let splitted_cmd: Vec<&str> = self.command.split_whitespace().collect();
      if splitted_cmd.len() > 1 {
        if let Ok(i) = splitted_cmd[2].parse::<i16>() {
          self.arg2 = Some(i);
        }
      }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_correct_command_type() {
        let command_1 = "push constant 10";
        let command_2 = "pop constant 10";
        let command_3 = "add";

        let mut parser_1 = Parser::new(command_1);
        parser_1.get_command_type();
        let mut parser_2 = Parser::new(command_2);
        parser_2.get_command_type();
        let mut parser_3 = Parser::new(command_3);
        parser_3.get_command_type();

        assert_eq!(parser_1.command_type, Some(CommandType::C_push));
        assert_eq!(parser_2.command_type, Some(CommandType::C_pop));
        assert_eq!(parser_3.command_type, Some(CommandType::C_arithmetic));
    }

    #[test]
    fn should_return_arg1() {
        let command_1 = "push constant 10";
        let command_2 = "pop constant 10";
        let command_3 = "add";

        let mut parser_1 = Parser::new(command_1);
        parser_1.get_arg1();
        let mut parser_2 = Parser::new(command_2);
        parser_2.get_arg1();
        let mut parser_3 = Parser::new(command_3);
        parser_3.get_arg1();

        assert_eq!(parser_1.arg1, Some("constant"));
        assert_eq!(parser_2.arg1, Some("constant"));
        assert_eq!(parser_3.arg1, None);
    }

    #[test]
    fn should_return_arg2() {
        let command_1 = "push constant 10";
        let command_2 = "pop constant 10";
        let command_3 = "add";

        let mut parser_1 = Parser::new(command_1);
        parser_1.get_arg2();
        let mut parser_2 = Parser::new(command_2);
        parser_2.get_arg2();
        let mut parser_3 = Parser::new(command_3);
        parser_3.get_arg2();

        assert_eq!(parser_1.arg2, Some(10));
        assert_eq!(parser_2.arg2, Some(10));
        assert_eq!(parser_3.arg2, None);
    }

    #[test]
    fn call_parse_should_return_tuple_command_type_arg1_arg2() {
        let command_1 = "push constant 10";
        let command_2 = "pop constant 10";
        let command_3 = "add";

        let mut parser_1 = Parser::new(command_1);
        let mut parser_2 = Parser::new(command_2);
        let mut parser_3 = Parser::new(command_3);

        assert_eq!(parser_1.parse(), (&Some(CommandType::C_push), Some("constant"), Some(10)));
        assert_eq!(parser_2.parse(), (&Some(CommandType::C_pop), Some("constant"), Some(10)));
        assert_eq!(parser_3.parse(), (&Some(CommandType::C_arithmetic), None, None));

    }
}
