use std::env;

use crate::monitor::sdb::sdb_init;

pub fn init_monitor() {
  parse_args();
  sdb_init();
}

fn parse_args() {
  let args: Vec<String> = env::args().collect();
  println!("Welcome to remuir!");

  // print all args
  for (idx, item) in args.iter().enumerate() {
    if idx == 0 {
      continue;
    }
    println!("get arg[{idx}]: {item}");
  }
}
