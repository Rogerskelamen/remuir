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
  unsafe {
    CORE.pc = npc;
  }
}

pub fn gpr_get(idx: usize) -> Word {
  unsafe { CORE.gpr[idx] }
}

pub fn gpr_set(idx: usize, reg_wb: Word) {
  unsafe {
    CORE.gpr[idx] = reg_wb;
  }
}

#[rustfmt::skip]
pub fn _isa_gpr_print() {
  let reg_name: [&str;32] = [
    "$0", "ra", "sp", "gp", "tp", "t0", "t1", "t2",
    "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5",
    "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7",
    "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6"
  ];
  unsafe {
    for (id, reg) in CORE.gpr.iter().enumerate() {
      println!(
        "{:<12}{:<16}{}",
        reg_name[id],
        format!("{:#x}", reg),
        reg
      )
    }
  }
}
