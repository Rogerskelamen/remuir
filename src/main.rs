// remuir -- a simple emulator to run riscv binary code

mod monitor;
mod memory;
mod cpu;
mod utils;

use monitor::{init::init_monitor, sdb::sdb_start};

fn main() {
  /* Initialize the monitor */
  init_monitor();

  /* Start SDB */
  sdb_start();
}
