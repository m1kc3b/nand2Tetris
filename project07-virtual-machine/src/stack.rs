#[derive(Debug, PartialEq, Eq)]
pub struct Stack {
  memory: Vec<i16>,
  sp: usize
}

impl Stack {
    pub fn new() -> Self {
      Self { memory: Vec::new(), sp: 0 }
    }

    pub fn push(&mut self, value: i16) {
      self.memory.insert(self.sp, value);
      self.sp = self.memory.len();
    }

    pub fn pop(&mut self) -> Option<i16> {
      let v = self.memory.pop();
      self.sp = self.memory.len();
      v
    }

    pub fn add(&mut self) {
      let y = self.pop().unwrap();
      let x = self.pop().unwrap();
      let _ = self.push(x + y);
    }

    pub fn sub(&mut self) {
      let y = self.pop().unwrap();
      let x = self.pop().unwrap();
      let _ = self.push(x - y);
    }

    pub fn neg(&mut self) {
      let x = self.pop().unwrap();
      let _ = self.push(-x);
    }

    // pub fn eq(&mut self) {
    //   let y = self.pop().unwrap();
    //   let x = self.pop().unwrap();
    //   if x == y {
    //     let _ = self.push(1);
    //   } else {
    //     let _ = self.push(0);
    //   }
    // }
}

#[cfg(test)]
mod tests {
    use super::Stack;


  #[test]
  fn should_push_x_in_the_stack() {
    let mut stack = Stack::new();
    let _ = stack.push(16);
    let _ = stack.push(2);

    assert_eq!(stack, Stack { memory: vec![16, 2], sp: 2})
  }

  #[test]
  fn should_pop_last_value_in_the_stack() {
    let mut stack = Stack::new();
    let _ = stack.push(16);
    let _ = stack.push(2);
    let _ = stack.push(7);
    let _ = stack.pop();

    assert_eq!(stack, Stack { memory: vec![16, 2], sp: 2})
  }

  #[test]
  fn should_add_x_and_y_in_the_stack() {
    let mut stack = Stack::new();
    let _ = stack.push(16);
    let _ = stack.push(2);
    let _ = stack.add();

    assert_eq!(stack, Stack { memory: vec![18], sp: 1})
  }

  #[test]
  fn should_sub_x_and_y_in_the_stack() {
    let mut stack = Stack::new();
    let _ = stack.push(16);
    let _ = stack.push(16);
    let _ = stack.push(2);
    let _ = stack.sub();

    assert_eq!(stack, Stack { memory: vec![16, 14], sp: 2})
  }

  #[test]
  fn should_neg_last_value_in_the_stack() {
    let mut stack = Stack::new();
    let _ = stack.push(16);
    let _ = stack.push(16);
    let _ = stack.push(2);
    let _ = stack.neg();

    assert_eq!(stack, Stack { memory: vec![16, 16, -2], sp: 3})
  }

  #[test]
  fn should_verify_eq_x_and_y_in_the_stack() {
    let mut stack = Stack::new();
    let _ = stack.push(16);
    let _ = stack.push(16);
    let _ = stack;

    assert_eq!(stack, Stack { memory: vec![16, 16, 1], sp: 3})
  }
}