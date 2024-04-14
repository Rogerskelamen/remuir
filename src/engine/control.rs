use crate::{log, utils::config::Addr};

pub enum ExecState {
  Running,
  Stop,
  End,
  Abort,
}

pub struct EmuState {
  pub state: ExecState,
  pub halt_pc: Addr,
  pub halt_ret: usize,
}

pub static mut EMUSTATE: EmuState = EmuState { state: ExecState::Stop, halt_pc: 0, halt_ret: 0 };

pub fn set_emu_state(state: ExecState, pc: Addr, ret: usize) {
  unsafe {
    EMUSTATE.state = state;
    EMUSTATE.halt_pc = pc;
    EMUSTATE.halt_ret = ret;
  }
}

pub fn invalid_inst(pc: Addr) {
  log!("invalid opcode(PC = {:#x})", pc);
  print!(
    r#"
There are two cases which will trigger this unexpected exception:
1. The instruction at PC = [{:#x}] is not implemented.
2. Something is implemented incorrectly.
"#,
    pc
  );
  // print!("Find this PC({:#x}) in the disassembling result to distinguish which case it is.\n\n", pc);
  println!(
    r#"
If it is the first case, see riscv-manual for more details.
If it is the second case, remember:
* The machine is always right!
* Every line of untested code is always wrong!
"#
  );
  set_emu_state(ExecState::Abort, pc, usize::MAX);
}
