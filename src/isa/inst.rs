use crate::{cpu::exec::Decode, memory::access::mem_read};

pub fn isa_exec(s: &mut Decode) {
  s.inst = mem_read(s.pc, 4);
}
