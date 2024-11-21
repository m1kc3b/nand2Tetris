use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolTable<'a> {
  entries: HashMap<&'a str, usize>
}

impl<'a> SymbolTable<'a> {
  pub fn new() -> Self {
    let mut entries = HashMap::new();
    entries.insert("R0", 0);
    entries.insert("R1", 1);
    entries.insert("R2", 2);
    entries.insert("R3", 3);
    entries.insert("R4", 4);
    entries.insert("R5", 5);
    entries.insert("R6", 6);
    entries.insert("R7", 7);
    entries.insert("R8", 8);
    entries.insert("R9", 9);
    entries.insert("R10", 10);
    entries.insert("R11", 11);
    entries.insert("R12", 12);
    entries.insert("R13", 13);
    entries.insert("R14", 14);
    entries.insert("R15", 15);
    entries.insert("SP", 0);
    entries.insert("LCL", 1);
    entries.insert("ARG", 2);
    entries.insert("THIS", 3);
    entries.insert("THAT", 4);
    entries.insert("SCREEN", 16384);
    entries.insert("KBD", 24576);

    Self { entries }
  }

  pub fn add_entry(&mut self, symbol: &'a str, address: usize) {
    if !self.entries.contains_key(symbol) {
      self.entries.insert(&symbol, address);
    }
  }

  fn contains(&self, given_symbol: &str) -> bool {
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

  #[test]
  fn should_create_and_symbol_table() {
    let symbol_table = SymbolTable::new();
    // assert_eq!(symbol_table, SymbolTable { entries: HashMap::new()})
    println!("{:#?}", symbol_table)
  }

  #[test]
  fn should_return_true_if_the_symboltable_contains_the_given_symbol() {
    let symbol_table = SymbolTable::new();
    assert_eq!(symbol_table.contains("R0"), true);
    assert_eq!(symbol_table.contains("LCL"), true);
    assert_eq!(symbol_table.contains("KBD"), true);
  }

  #[test] 
  fn should_return_the_address_1_of_the_given_symbol() {
    let symbol_table = SymbolTable::new();
    assert_eq!(symbol_table.get_address("KBD"), Some(24576));
    assert_eq!(symbol_table.get_address("R0"), Some(0));
    assert_eq!(symbol_table.get_address("LCL"), Some(1));
  }
}