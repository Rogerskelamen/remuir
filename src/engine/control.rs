use crate::{log, utils::config::Addr};

pub enum ExecState {
  Running,
  Stop,
  End,
  Abort
}

pub struct EmuState {
  pub state: ExecState,
  pub halt_pc: Addr
}

pub static mut EMUSTATE: EmuState = EmuState {
  state: ExecState::Stop,
  halt_pc: 0
};

pub fn invalid_inst(pc: Addr) {
  log!("invalid opcode(PC = {:#x})", pc);
}
