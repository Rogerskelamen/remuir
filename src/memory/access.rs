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

fn pmem_read(addr: Addr, len: usize) -> Word{
  let addr = addr as usize - MBASE;
  let data: Word;
  match len {
    1 => {
      unsafe {
        data = *PMEM.get(addr).unwrap() as Word
      }
      data
    }
    2 => {
      unsafe {
        let tmp = &PMEM[addr..addr+2];
        data = u16::from_le_bytes([tmp[0], tmp[1]]) as Word
        // PMEM.get(addr) + PMEM.get(addr + 1)
      }
      data
    }
  }
}

