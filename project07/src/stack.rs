
struct Stack {
  data: Vec<i16>,
  sp: u16
}

impl Stack {
  fn new() -> Self {
    Self { 
      data: Vec::new(),
      sp: 0,
     }
  }

  fn push(&mut self, value: i16) {
    self.data.push(value);
    self.sp += 1;
  }

  fn pop(&mut self) -> Option<i16> {
    self.sp -= 1;
    self.data.pop()
  }

  fn add(&mut self) {
    let y = self.data.pop().unwrap();
    let x = self.data.pop().unwrap();
    self.data.push(x + y);
  }

  fn sub(&mut self) {
    let y = self.data.pop().unwrap();
    let x = self.data.pop().unwrap();
    self.data.push(x - y);
  }

  fn neg(&mut self) {
    let y = self.data.pop().unwrap();
    self.data.push(-y);
  }
}