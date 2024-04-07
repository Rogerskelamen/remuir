use crate::{
  cpu::exec::Decode, crumble, isa::decode::find_inst, memory::access::mem_read,
};

pub fn isa_exec(s: &mut Decode) {
  s.inst = mem_read(s.pc, 4);
  isa_decode(s);
}

#[rustfmt::skip]
fn isa_decode(s: &mut Decode) {
  println!("{:#x}", s.inst);
  match find_inst(s.inst) {
    "ebreak" => {
      crumble!("encounter ebreak");
    }
    _ => { crumble!("never reach here!"); }
  }
  s.npc = s.pc + 4;
}
