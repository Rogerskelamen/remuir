use std::env;

use super::sdb::sdb_init;

pub fn init_monitor() {
  /* Parse arguments */
  parse_args();
  sdb_init();
  welcome();
}

fn parse_args() {
  let args: Vec<String> = env::args().collect();
  // println!("{:?}", args);
  // let args: Vec<String> = env::args().collect();

  // print all args
  for arg in args.iter().skip(1) {
    println!("{arg}");
  }
}

fn welcome() {
  println!("Welcome to remuir!");
}
