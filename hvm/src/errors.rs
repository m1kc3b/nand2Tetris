use std::{fmt, error::Error};

#[derive(Debug)]
pub enum HVMError {
  UnknownCommand(&'static str),

}

impl Error for HVMError {}

impl fmt::Display for HVMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HVMError::UnknownCommand(msg) => write!(f, "Error: {}", msg),
        }
    }
}
