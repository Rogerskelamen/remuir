use std::time::Instant;

use crate::{isa::inst::isa_exec, utils::config::*, engine::control::{EMUSTATE, ExecState}};

use super::core::{pc_get, pc_set};

#[derive(Default, Debug)]
pub struct Decode {
  pub pc: Addr,
  pub npc: Addr,
  pub inst: Word,
}

static mut TIMER: usize = 0;

/*
 * Execute for n times
 * Statistic the process,
 * Control Cpu status
 */
pub fn cpu_exec(n: u32) {
  unsafe {
    match EMUSTATE.state {
      ExecState::End | ExecState::Abort => {
        println!("Program execution has ended. To restart the program, exit remuir and run again.");
        return;
      }
      _ => {
        EMUSTATE.state = ExecState::Running;
      }
    }
  }

  let timer_start = Instant::now();
  // execute instructions
  execute(n);
  let timer_end = Instant::now();
  unsafe {TIMER += timer_end - timer_start;}

  /* Some control task */
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
  s.pc = pc_get();
  isa_exec(s);
  pc_set(s.npc); // update pc
}
