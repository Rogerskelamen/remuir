// remuir -- a simple emulator to run riscv binary code

mod cpu;
mod isa;
mod memory;
mod monitor;
mod utils;

use monitor::{init::init_monitor, sdb::sdb_start};

fn main() {
  /* Initialize the monitor */
  init_monitor();

  /* Start SDB */
  sdb_start();
}
