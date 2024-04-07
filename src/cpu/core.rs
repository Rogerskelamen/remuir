use crate::utils::config::*;

#[derive(Debug)]
struct Cpu {
  pc: Addr,
  gpr: [Word; GPR_NR],
}

/*
 * CORE must be absolutely private
 * It can only be access through fns here
 * In case other program change status in CORE
 */
static mut CORE: Cpu = Cpu { pc: MBASE as u32, gpr: [0; GPR_NR] };

pub fn pc_get() -> Addr {
  unsafe { CORE.pc }
}

pub fn pc_set(npc: Addr) {
  unsafe { CORE.pc = npc; }
}

pub fn gpr_get(idx: usize) -> Word {
  unsafe { CORE.gpr[idx] }
}

pub fn gpr_set(idx: usize, reg_wb: Word) {
  unsafe { CORE.gpr[idx] = reg_wb; }
}
