use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolTable {
  entries: HashMap<String, usize>
}

impl SymbolTable {
  pub fn new() -> Self {
    Self { entries: HashMap::new() }
  }

  pub fn add_entry(&mut self, symbol: &str, address: usize) {
    self.entries.insert(symbol.to_string(), address);
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

  #[test]
  fn should_create_and_symbol_table() {
    let symbol_table = SymbolTable::new();
    assert_eq!(symbol_table, SymbolTable { entries: HashMap::new()})
  }

  #[test]
  fn should_return_true_if_the_symboltable_contains_the_given_symbol() {
    let mut symbol_table = SymbolTable::new();
    symbol_table.add_entry("test", 1);
    assert_eq!(symbol_table.contains("test"), true)
  }

  #[test] 
  fn should_return_the_address_1_of_the_given_symbol() {
    let mut symbol_table = SymbolTable::new();
    symbol_table.add_entry("test", 1);
    assert_eq!(symbol_table.get_address("test"), Some(1))
  }
}