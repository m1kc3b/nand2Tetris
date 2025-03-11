pub struct Ram {
  memory: [u16; 32768],
}

impl Ram {
  pub fn new() -> Self {
      Self { memory: [0; 32768] }
  }

  pub fn read(&self, address: usize) -> u16 {
      self.memory[address]
  }

  pub fn write(&mut self, address: usize, value: u16) {
      self.memory[address] = value;
  }
}
