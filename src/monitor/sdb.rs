use rustyline::Editor;

use crate::{
  cpu::exec::cpu_exec,
  crumble,
  engine::control::{ExecState, EMUSTATE},
  log,
};

static mut BATCH_MODE: bool = false;

/* function pointer for cmd call */
type CmdFn = fn(&str) -> isize;

struct CmdTable {
  name: &'static str,
  desc: &'static str,
  func: CmdFn,
}

const NR_CMD: usize = 3;

const CMDTAB: [CmdTable; NR_CMD] = [
  CmdTable {
    name: "help",
    desc: "Display information about all supporte commands",
    func: cmd_help,
  },
  CmdTable { name: "c", desc: "Continue the execution of the program", func: cmd_c },
  CmdTable { name: "q", desc: "Exit remuir", func: cmd_q },
];

pub fn sdb_init(is_batch: bool) {
  unsafe {
    BATCH_MODE = is_batch;
  }
}

/* Provide a shell prompt, wait user input */
fn rl_get(rl: &mut Editor<()>) -> String {
  let line_read = rl.readline("(remuir) ");
  match line_read {
    Ok(line) => {
      rl.add_history_entry(line.as_str());
      return line;
    }
    Err(e) => {
      crumble!("Error: {e}");
    }
  }
}

fn cmd_help(args: &str) -> isize {
  match args.split_whitespace().next() {
    Some(arg) => {
      for cmd in CMDTAB.iter() {
        if cmd.name == arg {
          println!("{} - {}", cmd.name, cmd.desc);
          return 0;
        }
      }
      println!("Unknow command '{arg}'");
      return 0;
    }
    None => {
      for cmd in CMDTAB.iter() {
        println!("{} - {}", cmd.name, cmd.desc);
      }
      return 0;
    }
  }
}

#[allow(unused_variables)]
fn cmd_c(args: &str) -> isize {
  cpu_exec(usize::MAX);
  return 0;
}

#[allow(unused_variables)]
fn cmd_q(args: &str) -> isize {
  unsafe {
    EMUSTATE.state = ExecState::End;
  }
  return -1;
}

pub fn sdb_start() {
  log!("Start sdb!");
  unsafe {
    if BATCH_MODE {
      cpu_exec(usize::MAX);
      return;
    }
  }

  /* create shell-prompt */
  let mut rl = Editor::<()>::new();
  loop {
    let input = rl_get(&mut rl);
    let cmd: &str;
    match input.split_whitespace().next() {
      Some(s) => {
        cmd = s;
      }
      None => {
        continue;
      }
    }
    let id = input.find(' ').unwrap_or(input.len());
    let args = if id >= input.len() { "" } else { &input[id + 1..] };
    for (idx, command) in CMDTAB.iter().enumerate() {
      if cmd == command.name {
        if (command.func)(args) < 0 {
          return;
        }
        break;
      }
      if idx == NR_CMD - 1 {
        println!("Unknow command '{cmd}'");
      }
    }
  }
}
