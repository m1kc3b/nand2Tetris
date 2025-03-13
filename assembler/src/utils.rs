

pub fn contains_alphabetic(word: &str) -> bool {
  let mut has_letter = false;
  for c in word.chars() {
    if c.is_alphabetic() {
      has_letter = true;
    }
  }
  has_letter
}