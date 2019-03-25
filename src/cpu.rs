use crate::bus::Bus;
use crate::instruction::{Instruction, JumpTest};
use crate::registers::{Register, RegisterPair, Registers};

#[derive(Debug)]
pub struct CPU {
  registers: Registers,
  pc: u16,
  sp: u16,
  bus: Bus,
  halted: bool,
}

impl CPU {
  pub fn new() -> Self {
    Self {
      registers: Registers::new(),
      pc: 0x0,
      sp: 0x0,
      bus: Bus::new(),
      halted: false,
    }
  }

  pub fn chomp_byte(&mut self) -> u8 {
    let value = self.bus.read_byte(self.pc);

    self.pc = self.pc.wrapping_add(1);

    value
  }

  pub fn chomp_word(&mut self) -> u16 {
    let a = self.bus.read_byte(self.pc);
    let b = self.bus.read_byte(self.pc + 1);

    self.pc = self.pc.wrapping_add(2);

    u16::from_le_bytes([a, b])
  }

  pub fn step(&mut self) {
    let mut instruction_byte = self.chomp_byte();

    let is_prefixed = instruction_byte == 0xCB;

    if is_prefixed {
      instruction_byte = self.chomp_byte();
    }

    let instruction =
      Instruction::of_byte(instruction_byte, is_prefixed).expect("unknown instruction");

    self.execute(instruction);
  }

  fn execute(&mut self, instruction: Instruction) {
    match instruction {
      Instruction::Noop => self.noop(),
      Instruction::Halt => self.halt(),
      Instruction::Add(target) => self.add(target),
      Instruction::Inc(target) => self.inc(target),
      Instruction::Jump(test) => self.jump(test),
      _ => panic!("instruction not implemented"),
    };
  }

  fn noop(&mut self) {}

  fn halt(&mut self) {
    self.halted = true;
  }

  fn add(&mut self, target: Register) {
    let value = self.registers.get(target);

    let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

    self.registers.f.zero = new_value == 0;
    self.registers.f.subtract = false;
    self.registers.f.carry = did_overflow;
    self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

    self.registers.a = new_value;
  }

  fn inc(&mut self, target: RegisterPair) {}

  fn jump(&mut self, test: JumpTest) {
    let flags = self.registers.f;

    let should_jump = match test {
      JumpTest::Always => true,
      JumpTest::Zero => flags.zero,
      JumpTest::NotZero => !flags.zero,
      JumpTest::Carry => flags.carry,
      JumpTest::NotCarry => !flags.carry,
    };

    if should_jump {
      let a = self.bus.read_byte(self.pc + 1);
      let b = self.bus.read_byte(self.pc + 2);

      self.pc = u16::from_le_bytes([a, b]);
    }
  }
}
