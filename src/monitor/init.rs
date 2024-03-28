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
  for (idx, item) in args.iter().enumerate() {
    if idx == 0 {
      continue;
    }
    println!("get arg[{idx}]: {item}");
  }
}

fn welcome() {
  println!("Welcome to remuir!");
}
