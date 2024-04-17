use capstone::prelude::*;

use super::config::Addr;

pub fn disasm(code: u32, addr: Addr) -> String {
  /* init riscv disassembler */
  let cs = Capstone::new()
    .riscv()
    .mode(arch::riscv::ArchMode::RiscV32) // this can be turned into RV64
    .build()
    .expect("Capstone engine build failed");

  let disasm = cs.disasm_all(&code.to_ne_bytes(), addr as u64).expect("disassemble failed");
  let inst = disasm.iter().next().expect("disassemble failed");
  format!("{}", inst)
}
