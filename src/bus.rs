use std::fmt;

pub struct Bus {
  memory: [u8; 0xFFFF],
}

impl fmt::Debug for Bus {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Bus {{ memory: {:X}, ... }}", self.memory[0])
  }
}

impl Bus {
  pub fn new() -> Self {
    Self {
      memory: [0; 0xFFFF],
    }
  }

  pub fn read_byte(&self, loc: u16) -> u8 {
    self.memory[loc as usize]
  }
}
