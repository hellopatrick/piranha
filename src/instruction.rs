use crate::registers::{Register, RegisterPair};

pub enum JumpTest {
  NotZero,
  Zero,
  NotCarry,
  Carry,
  Always,
}

pub enum Target {
  A,
  B,
  C,
  D,
  E,
  F,
  H,
  L,
  AF,
  BC,
  DE,
  HL,
  D8,
  D16,
  HLI,
}

pub enum Instruction {
  Noop,
  Halt,
  Add(Register),
  Inc(RegisterPair),
  Jump(JumpTest),
}

impl Instruction {
  pub fn of_byte(byte: u8, prefixed: bool) -> Option<Self> {
    if prefixed {
      Self::of_unprefixed_byte(byte)
    } else {
      Self::of_prefixed_byte(byte)
    }
  }

  pub fn of_unprefixed_byte(byte: u8) -> Option<Self> {
    match byte {
      0x00 => Some(Instruction::Noop),
      0x03 => Some(Instruction::Inc(RegisterPair::BC)),
      0x13 => Some(Instruction::Inc(RegisterPair::DE)),
      _ => None,
    }
  }

  pub fn of_prefixed_byte(byte: u8) -> Option<Self> {
    match byte {
      0x00 => Some(Instruction::Noop),
      _ => None,
    }
  }
}
