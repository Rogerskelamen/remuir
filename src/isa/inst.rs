use crate::{
  cpu::{core::gpr_set, exec::Decode}, crumble, isa::decode::find_inst, memory::access::mem_read, utils::config::Word,
};

enum ImmType {
  R,
  I,
  S,
  B,
  U,
  J
}

pub fn isa_exec(s: &mut Decode) {
  s.inst = mem_read(s.pc, 4);
  isa_decode(s);
}

#[rustfmt::skip]
fn isa_decode(s: &mut Decode) {
  let mut imm: Word = 0;
  let mut src1: Word = 0; let mut src2: Word = 0;
  macro_rules! instexe {
    ($itype:expr, $stat:stmt) => {
      decode_operand(s.inst, $itype, &mut imm, &mut src1, &mut src2)
    };
  }

  println!("{:#x}", s.inst);
  match find_inst(s.inst) {
    "auipc" => {
      // 1. rd = s.pc + imm
      instexe!(ImmType::U, );
      println!("here");
    }
    "lbu" => {

    }
    "sb" => {

    }
    "ebreak" => {
      crumble!("encounter ebreak");
    }
    _ => { crumble!("never reach here!"); }
  }
  gpr_set(0, 0); // x0 is always zero
  s.npc = s.pc + 4;
}

fn decode_operand(inst: Word, itype: ImmType, imm: &mut Word, src1: &mut Word, src2: &mut Word) {
  *src1 = split_bits(inst, 19, 15);
}

fn split_bits(data: Word, hi: usize, lo: usize) -> Word {
  (data >> lo) & (1usize << (hi - lo + 1) - 1) as Word
}
