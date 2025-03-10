use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolTable {
  entries: HashMap<String, usize>
}

impl SymbolTable {
  pub fn new() -> Self {
    let mut entries= HashMap::new();
    entries.insert("R0".to_string(), 0);
    entries.insert("R1".to_string(), 1);
    entries.insert("R2".to_string(), 2);
    entries.insert("R3".to_string(), 3);
    entries.insert("R4".to_string(), 4);
    entries.insert("R5".to_string(), 5);
    entries.insert("R6".to_string(), 6);
    entries.insert("R7".to_string(), 7);
    entries.insert("R8".to_string(), 8);
    entries.insert("R9".to_string(), 9);
    entries.insert("R10".to_string(), 10);
    entries.insert("R11".to_string(), 11);
    entries.insert("R12".to_string(), 12);
    entries.insert("R13".to_string(), 13);
    entries.insert("R14".to_string(), 14);
    entries.insert("R15".to_string(), 15);
    entries.insert("SP".to_string(), 0);
    entries.insert("LCL".to_string(), 1);
    entries.insert("ARG".to_string(), 2);
    entries.insert("THIS".to_string(), 3);
    entries.insert("THAT".to_string(), 4);
    entries.insert("SCREEN".to_string(), 16384);
    entries.insert("KBD".to_string(), 24576);

    Self { entries }
  }

  pub fn add_entry(&mut self, symbol: String, address: usize) {
      self.entries.entry(symbol).or_insert(address);
  }

  pub fn update_entry(&mut self, symbol: String, address: usize) {
    self.entries.entry(symbol).and_modify(|v| *v = address).or_insert(address);
}

  pub fn contains(&self, given_symbol: &str) -> bool {
    self.entries.contains_key(given_symbol)
  }

  pub fn get_address(&self, given_symbol: &str) -> Option<usize> {
    if let Some((_, &v)) = self.entries.get_key_value(given_symbol) {
      return Some(v)
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

}