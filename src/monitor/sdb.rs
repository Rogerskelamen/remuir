use rustyline::Editor;

use crate::{
  cpu::{core::isa_gpr_print, exec::cpu_exec},
  crumble,
  engine::control::{ExecState, EMUSTATE},
  log,
  memory::access::{check_bound, pmem_read},
  utils::config::Addr,
};

static mut BATCH_MODE: bool = false;

/* function pointer for cmd call */
type CmdFn = fn(&str) -> isize;

struct CmdTable {
  name: &'static str,
  desc: &'static str,
  func: CmdFn,
}

const NR_CMD: usize = 6;

const CMDTAB: [CmdTable; NR_CMD] = [
  CmdTable {
    name: "help",
    desc: "Display information about all supporte commands",
    func: cmd_help,
  },
  CmdTable { name: "c", desc: "Continue the execution of the program", func: cmd_c },
  CmdTable { name: "q", desc: "Exit remuir", func: cmd_q },
  CmdTable {
    name: "si",
    desc: "Step execute [N] instructions, N=1 if N is not specified",
    func: cmd_si,
  },
  CmdTable { name: "info", desc: "Print state of the program [r/w]", func: cmd_info },
  CmdTable {
    name: "x",
    desc: "Examine memory at address, return [N] 4 bytes value with hex format",
    func: cmd_x,
  },
];

pub fn init_sdb(is_batch: bool) {
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
          println!("{:<8} - {}", cmd.name, cmd.desc);
          return 0;
        }
      }
      println!("Unknow command '{arg}'");
      return 0;
    }
    None => {
      for cmd in CMDTAB.iter() {
        println!("{:<8} - {}", cmd.name, cmd.desc);
      }
      return 0;
    }
  }
}

#[allow(unused_variables)]
fn cmd_c(args: &str) -> isize {
  /* infinite execute */
  cpu_exec(usize::MAX);
  return 0;
}

#[allow(unused_variables)]
fn cmd_q(args: &str) -> isize {
  unsafe {
    if EMUSTATE.state == ExecState::Stop {
      EMUSTATE.state = ExecState::End;
    }
  }
  return -1;
}

fn cmd_si(args: &str) -> isize {
  match args.split_whitespace().next() {
    Some(arg) => {
      match arg.parse::<usize>() {
        /* step execute */
        Ok(n) => {
          if n == 0 {
            println!("Please input a positive number");
            return 0;
          }
          cpu_exec(n);
        }
        Err(_) => {
          println!("Please input a positive number");
        }
      }
    }
    None => {
      cpu_exec(1);
    }
  }
  return 0;
}

fn cmd_info(args: &str) -> isize {
  match args.split_whitespace().next() {
    Some(arg) => {
      if arg == "r" {
        isa_gpr_print();
      } else {
        println!("Subcommand not found");
      }
    }
    None => {
      println!("Please give a subcommand [r/w]");
    }
  }
  return 0;
}

fn cmd_x(args: &str) -> isize {
  let mut args = args.split_whitespace();
  match args.next() {
    Some(n) => match n.parse::<usize>() {
      Ok(n) => {
        if n == 0 {
          println!("Please input a positive number");
          return 0;
        }
        match args.next() {
          Some(addr) => match Addr::from_str_radix(addr, 16) {
            Ok(addr) => {
              for i in 0..n {
                if !check_bound(addr, 4) {
                  log!("Address [{:#x} - {:#x}] out of Memory", addr, addr + 4);
                  return 0;
                }
                let value = pmem_read(addr + (i * 4) as Addr, 4);
                if i % 4 == 0 {
                  print!("\n{:#x}: ", addr + (i * 4) as Addr);
                }
                print!("0x{:<10x}", value);
              }
              println!();
            }
            Err(_) => {
              println!("Accept a hex address");
            }
          },
          None => {
            println!("Please enter an address");
          }
        }
      }
      Err(_) => {
        println!("Please input a positive number");
      }
    },
    None => {
      println!("Please enter a range of memory you want to examine");
    }
  }
  return 0;
}

pub fn sdb_start() {
  unsafe {
    if BATCH_MODE {
      cmd_c("");
      return;
    }
  }

  /* Head tip for sdb */
  log!("Start sdb!");

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
