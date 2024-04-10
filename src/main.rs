// remuir -- a simple emulator to run riscv binary code

mod cpu;
mod engine;
mod isa;
mod memory;
mod monitor;
mod utils;

use engine::init::engine_start;
use monitor::init::init_monitor;

fn main() {
  /* Initialize the monitor */
  init_monitor();

  /* Start Engine */
  engine_start();
}
