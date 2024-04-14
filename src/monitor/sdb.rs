use rustyline::Editor;

use crate::{cpu::exec::cpu_exec, crumble, log};

static mut BATCH_MODE: bool = false;

// /* function pointer for cmd call */
// type CmdFn = fn(String);
//
// struct CmdTable {
//
// }

pub fn sdb_init(is_batch: bool) {
  unsafe {
    BATCH_MODE = is_batch;
  }
}

fn rl_get(rl: &mut Editor<()>) -> String {
  let line_read = rl.readline("(remuir) ");
  match line_read {
    Ok(line) => {
      rl.add_history_entry(line.as_str());
      return line;
    }
    Err(e) => {
      crumble!("Error: {}", e);
    }
  }
}

fn cmd_c(args: &str) {
  println!("{}", args);
  cpu_exec(usize::MAX);
}

pub fn sdb_start() {
  log!("Start sdb!");
  unsafe {
    if BATCH_MODE {
      cpu_exec(usize::MAX);
      return;
    }
  }

  // cpu_exec(usize::MAX);
  // return;

  /* create shell-prompt */
  let mut rl = Editor::<()>::new();
  loop {
    let l = rl_get(&mut rl);
    let cmd: &str;
    match l.split_whitespace().next() {
      Some(s) => {
        cmd = s;
      }
      None => {
        continue;
      }
    }
    let id = l.find(' ').unwrap_or(l.len());
    println!("{}", id);
    let args = if id >= l.len() { "" } else { &l[id + 1..] };
    match cmd {
      "c" => {
        cmd_c(args);
      }
      _ => {
        println!("Unknow command '{}'", cmd)
      }
    }
    println!("{}", l);
  }
}
