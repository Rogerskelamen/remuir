use crate::{log, utils::config::Addr};

pub fn invalid_inst(pc: Addr) {
  log!("invalid opcode(PC = {:#x})", pc);
}
