use crate::{
  cpu::{
    core::{gpr_get, gpr_set},
    exec::Decode,
  },
  crumble,
  engine::control::{invalid_inst, set_emu_state, ExecState},
  memory::pmem::{mem_read, mem_write},
  utils::config::{SWord, Word},
};

use super::decode::find_inst;

enum ImmType {
  R,
  I,
  S,
  B,
  U,
  J,
}

pub fn isa_exec(s: &mut Decode) {
  s.inst = mem_read(s.pc, 4); // fetch inst
  isa_decode(s);
}

#[rustfmt::skip]
#[allow(unused_assignments)]
fn isa_decode(s: &mut Decode) {
  let mut imm: Word = 0;
  let mut src1: Word = 0; let mut src2: Word = 0;
  let mut rd: usize = 0;

  macro_rules! instexec {
    ($itype:expr, $($stat:stmt),*) => {
      let rs1 = split_bits(s.inst, 19, 15) as usize;
      let rs2 = split_bits(s.inst, 24, 20) as usize;
      rd      = split_bits(s.inst, 11, 7)  as usize;
      match $itype {
        ImmType::R => {
          src1 = gpr_get(rs1);
          src2 = gpr_get(rs2);
        }
        ImmType::I => {
          src1 = gpr_get(rs1);
          imm = imm_i(s.inst);
        }
        ImmType::S => {
          src1 = gpr_get(rs1);
          src2 = gpr_get(rs2);
          imm = imm_s(s.inst);
        }
        ImmType::B => {
          src1 = gpr_get(rs1);
          src2 = gpr_get(rs2);
          imm = imm_b(s.inst);
        }
        ImmType::U => {
          imm = imm_u(s.inst);
        }
        ImmType::J => {
          imm = imm_j(s.inst);
        }
      }
      $(
        $stat
      )*
    };
  }

  s.npc = s.pc + 4;
  match find_inst(s.inst) {
    "add"   => { instexec!(ImmType::R, gpr_set(rd, src1 + src2)); }
    "sub"   => { instexec!(ImmType::R, gpr_set(rd, src1 - src2)); }
    "sll"   => { instexec!(ImmType::R, gpr_set(rd, src1 << src2)); }
    "slt"   => { instexec!(ImmType::R, gpr_set(rd, if (src1 as SWord) < (src2 as SWord) {1} else {0})); }
    "sltu"  => { instexec!(ImmType::R, gpr_set(rd, if src1 < src2 {1} else {0})); }
    "xor"   => { instexec!(ImmType::R, gpr_set(rd, src1 ^ src2)); }
    "srl"   => { instexec!(ImmType::R, gpr_set(rd, src1 >> src2)); }
    "sra"   => { instexec!(ImmType::R, gpr_set(rd, (src1 as SWord >> src2) as Word)); }
    "or"    => { instexec!(ImmType::R, gpr_set(rd, src1 | src2)); }
    "and"   => { instexec!(ImmType::R, gpr_set(rd, src1 & src2)); }

    "auipc" => { instexec!(ImmType::U, gpr_set(rd, s.pc + imm)); }
    "lui"   => { instexec!(ImmType::U, gpr_set(rd, imm)); }
    "jal"   => { instexec!(ImmType::J, gpr_set(rd, s.pc + 4), s.npc = s.pc+imm); }
    "jalr"  => { instexec!(ImmType::I, gpr_set(rd, s.pc + 4), s.npc = src1+imm); }

    "addi"  => { instexec!(ImmType::I, gpr_set(rd, src1 + imm)); }
    "slli"  => { instexec!(ImmType::I, gpr_set(rd, src1 << imm)); }
    "slti"  => { instexec!(ImmType::I, gpr_set(rd, if (src1 as SWord) < (imm as SWord) {1} else {0})); }
    "sltiu" => { instexec!(ImmType::I, gpr_set(rd, if src1 < imm {1} else {0})); }
    "xori"  => { instexec!(ImmType::I, gpr_set(rd, src1 ^ imm)); }
    "srli"  => { instexec!(ImmType::I, gpr_set(rd, src1 >> imm)); }
    "srai"  => { instexec!(ImmType::I, gpr_set(rd, (src1 as SWord >> imm) as Word)); }
    "ori"   => { instexec!(ImmType::I, gpr_set(rd, src1 | imm)); }
    "andi"  => { instexec!(ImmType::I, gpr_set(rd, src1 & imm)); }

    "lb"    => { instexec!(ImmType::I, gpr_set(rd, expand_signed(mem_read(src1 + imm, 1), 8))); }
    "lh"    => { instexec!(ImmType::I, gpr_set(rd, expand_signed(mem_read(src1 + imm, 2), 16))); }
    "lw"    => { instexec!(ImmType::I, gpr_set(rd, expand_signed(mem_read(src1 + imm, 4), 32))); }
    "lbu"   => { instexec!(ImmType::I, gpr_set(rd, mem_read(src1 + imm, 1))); }
    "lhu"   => { instexec!(ImmType::I, gpr_set(rd, mem_read(src1 + imm, 2))); }
    "lwu"   => { instexec!(ImmType::I, gpr_set(rd, mem_read(src1 + imm, 4))); }
    "sb"    => { instexec!(ImmType::S, mem_write(src1 + imm, src2, 1)); }
    "sh"    => { instexec!(ImmType::S, mem_write(src1 + imm, src2, 2)); }
    "sw"    => { instexec!(ImmType::S, mem_write(src1 + imm, src2, 4)); }

    "beq"   => { instexec!(ImmType::B, if src1 == src2 { s.npc = s.pc+imm }); }
    "bne"   => { instexec!(ImmType::B, if src1 != src2 { s.npc = s.pc+imm }); }
    "blt"   => { instexec!(ImmType::B, if (src1 as SWord) < (src2 as SWord) { s.npc = s.pc+imm }); }
    "bge"   => { instexec!(ImmType::B, if (src1 as SWord) >= (src2 as SWord) { s.npc = s.pc+imm }); }
    "bltu"  => { instexec!(ImmType::B, if src1 < src2 { s.npc = s.pc+imm }); }
    "bgeu"  => { instexec!(ImmType::B, if src1 >= src2 { s.npc = s.pc+imm }); }

    "ebreak" => { set_emu_state(ExecState::End, s.pc, gpr_get(10) as usize); } // state = end
    "inv"    => { invalid_inst(s.pc); } // state = abort

    _ => { crumble!("never reach here!"); }
  }

  gpr_set(0, 0); // x0 is always zero
}

fn split_bits(data: Word, hi: usize, lo: usize) -> Word {
  (data >> lo) & ((1usize << (hi - lo + 1)) - 1) as Word
}

fn expand_signed(data: Word, width: usize) -> Word {
  let expand_bits: Word =
    if data >> (width - 1) == 1 { !((1usize << width) - 1) as Word } else { 0 as Word };
  data | expand_bits
}

fn imm_i(inst: Word) -> Word {
  expand_signed(split_bits(inst, 31, 20), 12)
}

fn imm_u(inst: Word) -> Word {
  expand_signed(split_bits(inst, 31, 12), 20) << 12
}

fn imm_s(inst: Word) -> Word {
  expand_signed(split_bits(inst, 31, 25), 7) << 5 | split_bits(inst, 11, 7)
}

fn imm_b(inst: Word) -> Word {
  expand_signed(split_bits(inst, 31, 31), 1) << 12
    | split_bits(inst, 30, 25) << 5
    | split_bits(inst, 11, 8) << 1
    | split_bits(inst, 7, 7) << 11
}

fn imm_j(inst: Word) -> Word {
  expand_signed(split_bits(inst, 31, 31), 1) << 20
    | split_bits(inst, 30, 21) << 1
    | split_bits(inst, 20, 20) << 11
    | split_bits(inst, 19, 12) << 12
}
