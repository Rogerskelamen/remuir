// remuir -- a simple emulator to run riscv binary code

mod monitor;
use monitor::init::init_monitor;

fn main() {
  init_monitor();
}
