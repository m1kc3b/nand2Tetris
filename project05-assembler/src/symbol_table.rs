#[derive(Debug, PartialEq, Eq)]
struct Symbol {
  name: String,
  address: usize
}

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolTable {
  entries: Vec<Symbol>
}

impl SymbolTable {
  pub fn new() -> Self {
    Self { entries: Vec::new() }
  }

  pub fn add_entry(&mut self, name: String, address: usize) {
    self.entries.push(Symbol { name, address });
  }
}






#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_create_and_symbol_table() {
    let symbol_table = SymbolTable::new();
    assert_eq!(symbol_table, SymbolTable { entries: Vec::new()})
  }

  #[test]
  fn should_add_new_entry_to_the_symbol_table() {
    let mut symbol_table = SymbolTable::new();
    symbol_table.add_entry("test".to_string(), 1);
    assert_eq!(symbol_table, SymbolTable { entries: vec![Symbol { name: "test".to_string(), address: 1 }]})
  }
}