// remuir -- a simple emulator to run riscv binary code

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // print all args
    for item in &args {
        println!("{}", item)
    }
    println!("{}", args[1]);
}
