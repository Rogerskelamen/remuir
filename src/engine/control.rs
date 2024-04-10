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
  set_emu_state(ExecState::Abort, pc, usize::MAX);
}
