
struct Stack<T> {
  data: Vec<T>,
  sp: u16
}

impl<T> Stack<T> {
  fn new() -> Self {
    Self { 
      data: Vec::new(),
      sp: 0,
     }
  }

  fn push(&mut self, value: T) {
    self.data.push(value);
    self.sp += 1;
  }

  fn pop(&mut self) -> Option<T> {
    self.sp -= 1;
    self.data.pop()
  }

  fn add(&mut self) {
    let y = self.data.pop().unwrap();
    let x = self.data.pop().unwrap();
    self.data.push(x + y);
  }

  fn neg(&mut self) {
    let y = self.data.pop().unwrap();
    self.data.push(-y);
  }
}