use crate::{
  cpu::{
    core::{gpr_get, gpr_set},
    exec::Decode,
  },
  crumble,
  isa::decode::find_inst,
  memory::access::{mem_read, mem_write, show_pmem},
  utils::config::Word,
};

enum ImmType {
  R,
  I,
  S,
  B,
  U,
  J,
  N,
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
        ImmType::N => {}
      }
      $(
        $stat
      )*
    };
  }

  println!("{:#x}", s.inst);
  match find_inst(s.inst) {
    "auipc" => {
      // rd = s.pc + imm
      instexec!(ImmType::U, gpr_set(rd, s.pc + imm));
    }
    "sb" => {
      // mem(rs1 + imm, rs2, 1)
      instexec!(ImmType::S, mem_write(src1 + imm, src2, 1));
    }
    "lbu" => {
      // rd = mem(rs1 + imm, 1)
      instexec!(ImmType::I, gpr_set(rd, mem_read(src1 + imm, 1)));
    }
    "ebreak" => {
      show_pmem();
      crumble!("encounter ebreak");
    }
    _ => { crumble!("never reach here!"); }
  }

  gpr_set(0, 0); // x0 is always zero

  s.npc = s.pc + 4;
}

fn split_bits(data: Word, hi: usize, lo: usize) -> Word {
  (data >> lo) & ((1usize << (hi - lo + 1)) - 1) as Word
}

fn expand_signed(data: Word, width: usize) -> Word {
  let expand_bits: Word = if data >> (width - 1) == 1 {
    !((1usize << width) - 1) as Word
  } else {
    0 as Word
  };
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
