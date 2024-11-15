const MEMORY_SIZE: usize = 32 * 1024;

struct VirtualRAM {
  memory: Vec<i16>,
  sp: usize,
  lcl: usize,
  arg: usize,
  this: usize,
  that: usize,
  temp_start: usize,
  temp_end: usize,
  r13: usize,
  r14: usize,
  r15: usize,
  static_start: usize,
  static_end: usize,
  stack_start: usize,
  stack_end: usize,
}

impl VirtualRAM {
  fn new() -> Self {
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
  fn push(&mut self, segment: &str, index: usize) {
    let address = self.resolve_segment(segment, index);
    let value = self.memory[address];
    self.memory[self.sp] = value;
    self.sp += 1;
  }

  // pop Stack value to push it in segment[index]
  fn pop(&mut self, segment: &mut &str, index: usize) {
    let value = self.memory[self.sp];
    let address = self.resolve_segment(segment, index);
    self.memory[address] = value
  }

  fn resolve_segment(&self, segment: &str, index: usize) -> usize {
    match segment {
      "argument" => self.arg,
      "local" => self.lcl,
      "static" => {
        if index > self.static_end - self.static_start {
          panic!("Index out of bounds for STATIC segment");
        }
        self.static_start + index
      },
      "constant" => todo!(),
      "this" => self.this,
      "that" => self.that,
      "pointer" => todo!(),
      "temp" => {
        if index > self.temp_end - self.temp_start {
          panic!("Index out of bounds for TEMP segment");
        }
        self.temp_start + index
      },
      _ => panic!("Invalid segment name"),
    }
  }

  // integer addition (x + y)
  fn add(&mut self) {
    let y = self.memory[self.sp];
    self.memory[self.sp] = 0;
    self.sp -= 1;
    let x = self.memory[self.sp];
    let result = x + y;
    self.memory[self.sp] = result;
  }

  // integer substraction (x - y)
  fn sub(&mut self) {
    let y = self.memory[self.sp];
    self.memory[self.sp] = 0;
    self.sp -= 1;
    let x = self.memory[self.sp];
    let result = x - y;
    self.memory[self.sp] = result;
  }

  // arithmetic negation (-y)
  fn neg(&mut self) {
    let y = self.memory[self.sp];
    self.memory[self.sp] = -y;
  }

  // equality (x == y)
  fn eq(&mut self) {
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
  fn gt(&mut self) {
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
  fn lt(&mut self) {
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
  fn and(&mut self) {
    let y = self.memory[self.sp];
    self.memory[self.sp] = 0;
    self.sp -= 1;
    let x = self.memory[self.sp];
    let result = x & y;
    self.memory[self.sp] = result
  }

  // bit-wise Or (x Or y)
  fn or(&mut self) {
    let y = self.memory[self.sp];
    self.memory[self.sp] = 0;
    self.sp -= 1;
    let x = self.memory[self.sp];
    let result = x | y;
    self.memory[self.sp] = result
  }

  // bit-wise Not (Not y)
  fn not(&mut self) {
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