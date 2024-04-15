use std::process;

use crate::monitor::sdb::sdb_start;

use super::control::{EMUSTATE, ExecState};

pub fn engine_start() {
  sdb_start();
}

pub fn exit_from_engine() {
  unsafe {
    if matches!(EMUSTATE.state, ExecState::End) &&
      EMUSTATE.halt_ret == 0
    {
      println!("Exit safely");
      process::exit(0);
    }else {
      println!("Exit with specified code [{}]", EMUSTATE.halt_ret);
      process::exit(1);
    }
  }
}
