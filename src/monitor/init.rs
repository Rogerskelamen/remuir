use std::env;

pub fn init_monitor() {
  parse_args();
}

fn parse_args() {
  let args: Vec<String> = env::args().collect();

  // print all args
  for (idx, item) in args.iter().enumerate() {
    if idx == 0 {
      continue;
    }
    println!("{}", item)
  }
}
