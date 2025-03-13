use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    table: HashMap<String, u16>,
    next_var_address: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = HashMap::new();
        // Adds registers R0-R15 and their Hack RAM address
        for i in 0..16 {
            table.insert(format!("R{}", i), i);
        }
        table.insert("SCREEN".to_string(), 16384);
        table.insert("KBD".to_string(), 24576);

        SymbolTable { table, next_var_address: 16 } // 16 is the RAM starting address.
    }

    pub fn add_label(&mut self, label: &str, address: u16) {
        self.table.insert(label.to_string(), address);
    }

    pub fn get_address(&mut self, symbol: &str) -> u16 {
        if let Some(&addr) = self.table.get(symbol) {
            return addr;
        }
        let addr = self.next_var_address;
        self.table.insert(symbol.to_string(), addr);
        self.next_var_address += 1;
        addr
    }

    pub fn get_last_ram_address(&self) -> u16 {
      self.next_var_address
    }
}
