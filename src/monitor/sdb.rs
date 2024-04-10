use crate::{cpu::exec::cpu_exec, log};

static mut BATCH_MODE: bool = false;

pub fn sdb_init(is_batch: bool) {
  unsafe {
    BATCH_MODE = is_batch;
  }
}

pub fn sdb_start() {
  log!("Start sdb!");
  cpu_exec(u32::MAX);
}
