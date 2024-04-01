// remuir -- a simple emulator to run riscv binary code

mod memory;
mod monitor;
mod utils;

use monitor::init::init_monitor;

fn main() {
  init_monitor();

  // sdb_start();
}
