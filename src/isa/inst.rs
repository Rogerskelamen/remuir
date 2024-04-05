use crate::{
  cpu::exec::Decode, crumble, isa::decode::inst_pad, memory::access::mem_read,
};

pub fn isa_exec(s: &mut Decode) {
  s.inst = mem_read(s.pc, 4);
  isa_decode(s);
}

fn isa_decode(s: &mut Decode) {
  println!("{:#x}", s.inst);
  match inst_pad(s.inst) {
    "ebreak" => {
      crumble!("encounter ebreak");
    }
    _ => {}
  }
  s.npc = s.pc + 4;
}
