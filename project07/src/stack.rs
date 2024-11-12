
struct Stack {
  data: Vec<i16>,
  sp: u8
}

struct Segment {
  name: String,
  data: Vec<i16>
}

impl Segment {
  fn new(segment_name: String) -> Self {
    Self {
      name: segment_name,
      data: Vec::new()
    }
  }
}

impl Stack {
  fn new() -> Self {
    Self { 
      data: Vec::new(),
      sp: 0,
     }
  }

  // push value from segment[index] in the Stack
  fn push(&mut self, segment: &Segment, index: usize) {
    let value = segment.data[index];
    self.data.push(value);
    self.sp += 1;
  }

  // pop Stack value to push it in segment[index]
  fn pop(&mut self, segment: &mut Segment, index: usize) {
    self.sp -= 1;
    let value = self.data.pop().unwrap();

    if index >= segment.data.len() {
      segment.data.resize_with(index + 1, || 0);
      segment.data[index] = value;
    }
    segment.data[index] = value;
  }

  // integer addition (x + y)
  fn add(&mut self) {
    let y = self.data.pop().unwrap();
    let x = self.data.pop().unwrap();
    self.data.push(x + y);
  }

  // integer substraction (x - y)
  fn sub(&mut self) {
    let y = self.data.pop().unwrap();
    let x = self.data.pop().unwrap();
    self.data.push(x - y);
  }

  // arithmetic negation (-y)
  fn neg(&mut self) {
    let y = self.data.pop().unwrap();
    self.data.push(-y);
  }

  // equality (x == y)
  fn eq(&mut self) {
    todo!()
  }

  // greater than (x > y)
  fn gt(&mut self) {
    todo!()
  }

  // less than (x < y)
  fn lt(&mut self) {
    todo!()
  }

  // bit-wise And (x And y)
  fn and(&mut self) {
    todo!()
  }

  // bit-wise Or (x Or y)
  fn or(&mut self) {
    todo!()
  }

  // bit-wise Not (Not y)
  fn not(&mut self) {
    todo!()
  }
}

mod tests {
  use super::*;

  #[test]
  fn push_value_from_segment_test_in_the_stack() {
    let mut stack = Stack::new();
    let mut test_seg = Segment::new("test".to_string());
    test_seg.data.push(10);
    stack.push(&test_seg, 0);

    assert_eq!(&stack.data, &test_seg.data);
  }

  #[test]
  fn pop_stack_to_push_it_in_the_test_segment() {
    let mut stack = Stack::new();
    let mut test_seg = Segment::new("test".to_string());
    test_seg.data.push(10);
    stack.push(&test_seg, 0);
    stack.pop(&mut test_seg, 1);

    let test_vec = vec![10,10];
    assert_eq!(test_vec, test_seg.data)
  }

  #[test]
  fn add_x2_and_y7_from_the_stack() {
    let mut stack = Stack::new();
    stack.data.push(2);
    stack.data.push(7);

    stack.add();

    let test_stack = vec![9];
    assert_eq!(stack.data, test_stack);
  }

  #[test]
  fn sub_x2_and_y7_from_the_stack() {
    let mut stack = Stack::new();
    stack.data.push(2);
    stack.data.push(7);

    stack.sub();

    let test_stack = vec![-5];
    assert_eq!(stack.data, test_stack);
  }

  #[test]
  fn neg_value_from_the_stack() {
    let mut stack = Stack::new();
    stack.data.push(2);
    stack.data.push(7);

    stack.neg();

    let test_stack = vec![2, -7];
    assert_eq!(stack.data, test_stack);
  }
}