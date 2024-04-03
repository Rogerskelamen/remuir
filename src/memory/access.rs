use crate::{alert, utils::config::*};

use super::pmem::PMEM;

fn check_bound(addr: Addr, len: usize) -> bool {
  addr as usize > MBASE && addr as usize + len < MBASE + MSIZE
}

pub fn mem_read(addr: Addr, len: usize) {
  if check_bound(addr, len) {
    alert!(false, "Address [{:#x} - {:#x}] out of Range", addr, addr as usize + len);
  }
  pmem_read(addr, len);
}

fn pmem_read(addr: Addr, len: usize) {
  match len {
    1 => {
      unsafe {
        // PMEM.get(addr as usize)?
      }
    }
  }
}

