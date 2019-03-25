mod bus;
mod cpu;
mod instruction;
mod registers;

fn main() {
  println!("hello, world {:?}", cpu::CPU::new());
}
