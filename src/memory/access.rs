use crate::{alert, crumble, utils::config::*};

use super::pmem::PMEM;

fn check_bound(addr: Addr, len: usize) -> bool {
  addr as usize >= MBASE && addr as usize + len <= MBASE + MSIZE
}

pub fn mem_read(addr: Addr, len: usize) -> Word {
  if !check_bound(addr, len) {
    alert!(
      false,
      "Address [{:#x} - {:#x}] out of Memory",
      addr,
      addr as usize + len
    );
  }
  pmem_read(addr, len)
}

fn pmem_read(addr: Addr, len: usize) -> Word {
  let addr = addr as usize - MBASE;
  let data: Word;
  match len {
    1 => {
      unsafe {
        data = *PMEM.get(addr).unwrap() as Word;
      }
      data
    }
    2 => {
      unsafe {
        let tmp = &PMEM[addr..addr + 2];
        data = u16::from_le_bytes([tmp[0], tmp[1]]) as Word
      }
      data
    }
    4 => {
      unsafe {
        let tmp = &PMEM[addr..addr + 4];
        data = Word::from_le_bytes([tmp[0], tmp[1], tmp[2], tmp[3]])
      }
      data
    }
    _ => {
      crumble!("Address align length [{}] is invalid, expect [1/2/4]", len);
    }
  }
}
