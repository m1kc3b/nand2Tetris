const MEMORY_SIZE: usize = 32 * 1024;

pub struct VirtualRAM {
  memory: Vec<i16>,
  sp: usize,              // stack pointer
  lcl: usize,             // LOCAL: function's local variables
  arg: usize,             // ARGUMENT: function's arguments
  this: usize,            // THIS: pointer
  that: usize,            // THAT: pointer
  temp_start: usize,      // TEMP: temporary variables
  temp_end: usize,
  r13: usize,             // R13: register
  r14: usize,             // R14:
  r15: usize,             // R15:
  static_start: usize,    // STATIC: static variables seen by the function
  static_end: usize,
  stack_start: usize,     // STACK
  stack_end: usize,
}

impl VirtualRAM {
  pub fn new() -> Self {
    Self { 
      memory: vec![0; MEMORY_SIZE],
      sp: 256,
      lcl: 1,
      arg: 2,
      this: 3,
      that: 4,
      temp_start: 5,
      temp_end: 12,
      r13: 13,
      r14: 14,
      r15: 15,
      static_start: 16,
      static_end: 255,
      stack_start: 256,
      stack_end: 2047,
     }
  }

  // push value from segment[index] in the Stack
  pub fn push(&mut self, segment: &str, index: usize) {
    let address = self.resolve_segment(segment, index);
    let value = self.memory[address];
    self.memory[self.sp] = value;
    self.sp += 1;
  }

  // pop Stack value to push it in segment[index]
  pub fn pop(&mut self, segment: &mut &str, index: usize) {
    let value = self.memory[self.sp];
    let address = self.resolve_segment(segment, index);
    self.memory[address] = value
  }

  fn resolve_segment(&self, segment: &str, index: usize) -> usize {
    match segment {
      "local" => self.lcl,
      "argument" => self.arg,
      "pointer" => {
        match index {
          0 => self.this,
          1 => self.that,
          _ => panic!("Index out of bounds for POINTER segment"),
        }
      },
      "temp" => {
        if index > self.temp_end - self.temp_start {
          panic!("Index out of bounds for TEMP segment");
        }
        self.temp_start + index
      },
      "constant" => self.sp,
      "static" => {
        if index > self.static_end - self.static_start {
          panic!("Index out of bounds for STATIC segment");
        }
        self.static_start + index
      },
      _ => panic!("Invalid segment name"),
    }
  }

  // integer addition (x + y)
  pub fn add(&mut self) {
    let y = self.memory[self.sp];
    self.memory[self.sp] = 0;
    self.sp -= 1;
    let x = self.memory[self.sp];
    let result = x + y;
    self.memory[self.sp] = result;
  }

  // integer substraction (x - y)
  pub fn sub(&mut self) {
    let y = self.memory[self.sp];
    self.memory[self.sp] = 0;
    self.sp -= 1;
    let x = self.memory[self.sp];
    let result = x - y;
    self.memory[self.sp] = result;
  }

  // arithmetic negation (-y)
  pub fn neg(&mut self) {
    let y = self.memory[self.sp];
    self.memory[self.sp] = -y;
  }

  // equality (x == y)
  pub fn eq(&mut self) {
    let y = self.memory[self.sp];
    self.memory[self.sp] = 0;
    self.sp -= 1;
    let x = self.memory[self.sp];
    if x == y {
      self.memory[self.sp] = 1;
    }
    self.memory[self.sp] = 0;
  
  }

  // greater than (x > y)
  pub fn gt(&mut self) {
    let y = self.memory[self.sp];
    self.memory[self.sp] = 0;
    self.sp -= 1;
    let x = self.memory[self.sp];
    if x > y {

      self.memory[self.sp] = 1;
    }
    self.memory[self.sp] = 0;
  }

  // less than (x < y)
  pub fn lt(&mut self) {
    let y = self.memory[self.sp];
    self.memory[self.sp] = 0;
    self.sp -= 1;
    let x = self.memory[self.sp];
    if x < y {
      self.memory[self.sp] = 1;
    }
    self.memory[self.sp] = 0;
  }

  // bit-wise And (x And y)
  pub fn and(&mut self) {
    let y = self.memory[self.sp];
    self.memory[self.sp] = 0;
    self.sp -= 1;
    let x = self.memory[self.sp];
    let result = x & y;
    self.memory[self.sp] = result
  }

  // bit-wise Or (x Or y)
  pub fn or(&mut self) {
    let y = self.memory[self.sp];
    self.memory[self.sp] = 0;
    self.sp -= 1;
    let x = self.memory[self.sp];
    let result = x | y;
    self.memory[self.sp] = result
  }

  // bit-wise Not (Not y)
  pub fn not(&mut self) {
    let y = self.memory[self.sp];
    self.memory[self.sp] = !y;
  }
}

mod tests {
  use super::*;

  #[test]
  fn push_value_from_segment_test_in_the_stack() {
    todo!()
  }

  #[test]
  fn pop_stack_to_push_it_in_the_test_segment() {
    todo!()
  }

  #[test]
  fn add_x2_and_y7_from_the_stack() {
    todo!()
  }

  #[test]
  fn sub_x2_and_y7_from_the_stack() {
    todo!()
  }

  #[test]
  fn neg_value_from_the_stack() {
    todo!()
  }
}