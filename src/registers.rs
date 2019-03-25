use std::convert::From;

#[derive(Debug, Copy, Clone)]
pub struct FlagsRegister {
  pub zero: bool,
  pub subtract: bool,
  pub half_carry: bool,
  pub carry: bool,
}

const ZERO_FLAG: u8 = 0x80;
const SUBTRACT_FLAG: u8 = 0x40;
const HALF_CARRY_FLAG: u8 = 0x20;
const CARRY_FLAG: u8 = 0x10;

impl Default for FlagsRegister {
  fn default() -> Self {
    Self {
      zero: false,
      subtract: false,
      half_carry: false,
      carry: false,
    }
  }
}

impl From<FlagsRegister> for u8 {
  fn from(flag: FlagsRegister) -> u8 {
    (if flag.zero { ZERO_FLAG } else { 0 })
      | (if flag.subtract { SUBTRACT_FLAG } else { 0 })
      | (if flag.half_carry { HALF_CARRY_FLAG } else { 0 })
      | (if flag.carry { CARRY_FLAG } else { 0 })
  }
}

impl From<u8> for FlagsRegister {
  fn from(byte: u8) -> Self {
    let zero = (byte & ZERO_FLAG) != 0;
    let subtract = (byte & SUBTRACT_FLAG) != 0;
    let half_carry = (byte & HALF_CARRY_FLAG) != 0;
    let carry = (byte & CARRY_FLAG) != 0;

    FlagsRegister {
      zero,
      subtract,
      half_carry,
      carry,
    }
  }
}

#[derive(Debug)]
pub enum Register {
  A,
  B,
  C,
  D,
  E,
  F,
  H,
  L,
}

#[derive(Debug)]
pub enum RegisterPair {
  AF,
  BC,
  DE,
  HL,
  HLI,
  HLD,
}

#[derive(Debug)]
pub struct Registers {
  pub a: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub f: FlagsRegister,
  pub h: u8,
  pub l: u8,
}

impl Registers {
  pub fn new() -> Self {
    Self {
      a: 0x0,
      b: 0x0,
      c: 0x0,
      d: 0x0,
      e: 0x0,
      f: 0x0u8.into(),
      h: 0x0,
      l: 0x0,
    }
  }

  pub fn get(&self, register: Register) -> u8 {
    match register {
      Register::A => self.a,
      Register::B => self.b,
      Register::C => self.c,
      Register::D => self.d,
      Register::E => self.e,
      Register::F => self.f.into(),
      Register::H => self.h,
      Register::L => self.l,
    }
  }

  pub fn get_pair(&mut self, pair: RegisterPair) -> u16 {
    match pair {
      RegisterPair::AF => self.af(),
      RegisterPair::BC => self.bc(),
      RegisterPair::DE => self.de(),
      RegisterPair::HL => self.hl(),
      RegisterPair::HLI => self.hli(),
      RegisterPair::HLD => self.hld(),
    }
  }

  pub fn af(&self) -> u16 {
    let f: u8 = self.f.into();

    u16::from_be_bytes([self.a, f & 0xF0])
  }

  pub fn set_af(&mut self, value: u16) {
    let [a, f] = value.to_be_bytes();

    self.a = a;
    self.f = (f & 0xF0).into();
  }

  pub fn bc(&self) -> u16 {
    u16::from_be_bytes([self.b, self.c])
  }

  pub fn set_bc(&mut self, value: u16) {
    let [b, c] = value.to_be_bytes();

    self.b = b;
    self.c = c;
  }

  pub fn de(&self) -> u16 {
    u16::from_be_bytes([self.d, self.e])
  }

  pub fn set_de(&mut self, value: u16) {
    let [d, e] = value.to_be_bytes();

    self.d = d;
    self.e = e;
  }

  pub fn hl(&self) -> u16 {
    u16::from_be_bytes([self.h, self.l])
  }

  pub fn set_hl(&mut self, value: u16) {
    let [h, l] = value.to_be_bytes();

    self.h = h;
    self.l = l;
  }

  pub fn hld(&mut self) -> u16 {
    let res = self.hl();

    self.set_hl(res - 1);

    res
  }
  pub fn hli(&mut self) -> u16 {
    let res = self.hl();

    self.set_hl(res + 1);

    res
  }
}

#[cfg(test)]
mod test {
  use super::Registers;

  #[test]
  fn read_combined_registers() {
    let mut reg = Registers::new();

    reg.a = 0x12;
    reg.f = 0x23.into();
    reg.b = 0x34;
    reg.c = 0x45;
    reg.d = 0x56;
    reg.e = 0x67;
    reg.h = 0x78;
    reg.l = 0x89;

    assert_eq!(reg.af(), 0x1220);
    assert_eq!(reg.bc(), 0x3445);
    assert_eq!(reg.de(), 0x5667);
    assert_eq!(reg.hl(), 0x7889);
  }

  #[test]
  fn write_combined_registers() {
    let mut reg = Registers::new();

    reg.set_af(0x1234);
    reg.set_bc(0x2345);
    reg.set_de(0x3456);
    reg.set_hl(0x4567);

    assert_eq!(reg.a, 0x12);
    assert_eq!(u8::from(reg.f), 0x30);
    assert_eq!(reg.b, 0x23);
    assert_eq!(reg.c, 0x45);
    assert_eq!(reg.d, 0x34);
    assert_eq!(reg.e, 0x56);
    assert_eq!(reg.h, 0x45);
    assert_eq!(reg.l, 0x67);
  }
}
