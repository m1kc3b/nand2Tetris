use std::collections::HashMap;

struct MemorySegment {
    argument: HashMap<i16, i16>,
    local: HashMap<i16, i16>,
    statiq: HashMap<i16, i16>,
    this: HashMap<i16, i16>,
    that: HashMap<i16, i16>,
    pointer: HashMap<i16, i16>,
    temp: HashMap<i16, i16>,
}

impl MemorySegment {
    pub fn new() -> Self {
        Self {
            argument: HashMap::new(),
            local: HashMap::new(),
            statiq: HashMap::new(),
            this: HashMap::new(),
            that: HashMap::new(),
            pointer: HashMap::new(),
            temp: HashMap::new(),
        }
    }

    pub fn push(&mut self, segment: &str, index: i16, value: i16) {
      let seg = match segment {
        "argument" => &mut self.argument,
        "local" => &mut self.local,
        "static" => &mut self.statiq,
        "this" => &mut self.this,
        "that" => &mut self.that,
        "pointer" => &mut self.pointer,
        _ => &mut self.temp,
      };

      seg.insert(index, value);
    }
}
