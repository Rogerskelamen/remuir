use crate::cpu::exec::cpu_exec;

static mut BATCH_MODE: bool = false;

pub fn sdb_init(is_batch: bool) {
  unsafe {
    BATCH_MODE = is_batch;
  }
}

pub fn sdb_start() {
  cpu_exec(u32::MAX);
  println!("start sdb!");
}
