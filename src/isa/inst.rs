use core::panic;

use crate::{cpu::exec::Decode, memory::access::mem_read};

pub fn isa_exec(s: &mut Decode) {
  s.inst = mem_read(s.pc, 4);
  isa_decode(s);
}

fn isa_decode(s: &mut Decode) {
  println!("{:#x}", s.inst);
  if s.inst == 0x100073 {
    panic!("encounter ebreak");
  }
  s.npc = s.pc + 4;
}
