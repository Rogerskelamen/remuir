use rustyline::Editor;

use crate::{cpu::exec::cpu_exec, log, crumble};

static mut BATCH_MODE: bool = false;

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
      return line
    }
    Err(e) => {
      crumble!("Error: {}", e);
    }
  }
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

  let mut rl = Editor::<()>::new();
  loop {
    let l = rl_get(&mut rl);
    if let None = l.split_whitespace().next() {
      continue;
    }
    println!("{}", l);
  }
}
