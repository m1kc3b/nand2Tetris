use crate::parser::CommandType;

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn should_create_a_new_code_writer() {
    let output_string: String;
    let (command, segment, index) = (&Some(CommandType::C_push), Some("constant"), Some(10));

    match command {
      Some(cmd) => {
        match cmd {
          CommandType::C_arithmetic => {
            output_string = format!("arithmetic ops", )
          },
          _ => {
            output_string = format!("push/pop ops")
          }
        }
      },
      None => output_string = format!("Error")
    }

    assert_eq!(output_string, "push/pop ops".to_string());

  }
}