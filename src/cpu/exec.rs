use std::time::{Duration, Instant};

use crate::{
  crumble,
  engine::control::{ExecState, EMUSTATE},
  isa::inst::isa_exec,
  log,
  utils::config::*,
};

use super::core::{pc_get, pc_set};

#[derive(Default, Debug)]
pub struct Decode {
  pub pc: Addr,
  pub npc: Addr,
  pub inst: Word,
}

static mut TIMER: Duration = Duration::new(0, 0);
static mut INST_CNT: u128 = 0;

fn statistic() {
  unsafe {
    log!("host time spent = {} us", TIMER.as_micros());
    log!("total guest instructions = {}", INST_CNT);
    if TIMER.as_micros() > 0 {
      log!("simulation frequency = {} inst/s", INST_CNT * 1_000_000 / TIMER.as_micros());
    } else {
      log!("Finish running in less than 1 us and can not calculate the simulation frequency");
    }
  }
}

/*
 * Execute for n times
 * Statistic the process,
 * Control Engine status
 */
pub fn cpu_exec(n: usize) {
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
  unsafe {
    TIMER += timer_end - timer_start;
  }

  /* Some control task */
  unsafe {
    match EMUSTATE.state {
      ExecState::Running => {
        EMUSTATE.state = ExecState::Stop;
      }
      ExecState::End => {
        if EMUSTATE.halt_ret == 0 {
          log!("remuir: \u{001b}[32mHIT GOOD TRAP\u{001b}[0m at pc = {:#x}", EMUSTATE.halt_pc);
        } else {
          log!("remuir: \u{001b}[31mHIT BAD TRAP\u{001b}[0m at pc = {:#x}", EMUSTATE.halt_pc);
        }
        statistic();
      }
      ExecState::Abort => {
        log!("remuir: \u{001b}[31mABORT\u{001b}[0m at pc = {:#x}", EMUSTATE.halt_pc);
        statistic();
      }
      _ => {
        crumble!("never reach here!");
      }
    }
  }
}

/*
 * Execute for n times
 * But do some extra work during the time
 */
fn execute(mut n: usize) {
  let mut s = Decode::default();
  while n > 0 {
    /* some work before exec */
    exec_once(&mut s);
    /* some work after exec */
    unsafe {
      INST_CNT += 1;
      if EMUSTATE.state != ExecState::Running {
        break;
      }
    }
    n -= 1;
  }
}

fn exec_once(s: &mut Decode) {
  s.pc = pc_get();
  isa_exec(s);
  pc_set(s.npc); // update pc
}
