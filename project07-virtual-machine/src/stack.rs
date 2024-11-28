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

    pub fn equal(&mut self) {
      let y = self.pop().unwrap();
      let x = self.pop().unwrap();
      if x == y {
        let _ = self.push(1);
      } else {
        let _ = self.push(0);
      }
    }

    pub fn gt(&mut self) {
      let y = self.pop().unwrap();
      let x = self.pop().unwrap();
      if x > y {
        let _ = self.push(1);
      } else {
        let _ = self.push(0);
      }
    }

    pub fn lt(&mut self) {
      let y = self.pop().unwrap();
      let x = self.pop().unwrap();
      if x < y {
        let _ = self.push(1);
      } else {
        let _ = self.push(0);
      }
    }

    pub fn and(&mut self) {
      let y = self.pop().unwrap();
      let x = self.pop().unwrap();

      if x & y != 0 {
        let _ = self.push(1);
      } else {
        let _ = self.push(0);
      }
    }

    pub fn or(&mut self) {
      let y = self.pop().unwrap();
      let x = self.pop().unwrap();

      if x | y != 0 {
        let _ = self.push(1);
      } else {
        let _ = self.push(0);
      }
    }

    pub fn not(&mut self) {
      let x = self.pop().unwrap();
      self.push(!x);
    }
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
  fn should_verify_x_equal_y_in_the_stack() {
    let mut stack = Stack::new();
    let _ = stack.push(16);
    let _ = stack.push(16);
    let _ = stack.push(15);
    let _ = stack.push(15);
    let _ = stack.equal();

    assert_eq!(stack, Stack { memory: vec![16, 16, 1], sp: 3})
  }

  #[test]
  fn should_verify_x_greater_than_y_in_the_stack() {
    let mut stack = Stack::new();
    let _ = stack.push(16);
    let _ = stack.push(15);
    let _ = stack.push(15);
    let _ = stack.gt();

    assert_eq!(stack, Stack { memory: vec![16, 0], sp: 2})
  }

  #[test]
  fn should_verify_x_less_than_y_in_the_stack() {
    let mut stack = Stack::new();
    let _ = stack.push(16);
    let _ = stack.push(15);
    let _ = stack.push(17);
    let _ = stack.lt();

    assert_eq!(stack, Stack { memory: vec![16, 1], sp: 2})
  }

  #[test]
  fn should_verify_x_bitwise_and_than_y_in_the_stack() {
    let mut stack = Stack::new();
    let _ = stack.push(16);
    let _ = stack.push(15);
    let _ = stack.push(17);
    let _ = stack.and();

    assert_eq!(stack, Stack { memory: vec![16, 1], sp: 2})
  }

  #[test]
  fn should_not_x_in_the_stack() {
    let mut stack = Stack::new();
    let _ = stack.push(5);
    let _ = stack.not();

    assert_eq!(stack, Stack { memory: vec![-6], sp: 1})
  }
}