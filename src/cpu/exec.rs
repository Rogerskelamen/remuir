use crate::utils::config::*;

#[derive(Debug)]
struct Cpu {
  pc: Addr,
  gpr: [Word; GPR_NR],
}

#[derive(Default, Debug)]
pub struct Decode {
  pub pc: Addr,
  pub npc: Addr,
  pub inst: Word,
}

static mut CORE: Cpu = Cpu { pc: 0, gpr: [0; GPR_NR] };

/*
 * Execute for n times
 * Statistic the process,
 * Control Cpu status
 */
pub fn cpu_exec(n: u32) {
  // execute a inst
  execute(n);
}

/*
 * Execute for n times
 * But do some extra work during the time
 */
fn execute(mut n: u32) {
  let mut s = Decode::default();
  while n > 0 {
    /* some work before exec */
    exec_once(&mut s);
    /* some work after exec */
    n -= 1;
  }
}

fn exec_once(s: &mut Decode) {
  s.pc = unsafe { CORE.pc };
  isa_exec(s);
}