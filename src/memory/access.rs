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

pub fn mem_write(addr: Addr, data: Word, len: usize) {
  if !check_bound(addr, len) {
    alert!(
      false,
      "Address [{:#x} - {:#x}] out of Memory",
      addr,
      addr as usize + len
    );
  }
  pmem_write(addr, data, len);
}

fn pmem_write(addr: Addr, data: Word, len: usize) {
  
}

fn pmem_set(addr: Addr, byte: u8) {
  let mut pmem = PMEM.lock().unwrap();
  if let Some(value) = pmem.get_mut(&addr) {
    *value = byte;
  }else {
    pmem.insert(addr, byte);
  }
}

fn pmem_get(addr: Addr) -> Byte {
  let pmem = PMEM.lock().unwrap();
  match pmem.get(&addr) {
    Some(byte) => {
      *byte
    }
    None => { 0 }
  }
}

fn pmem_read(addr: Addr, len: usize) -> Word {
  match len {
    1 => {
      pmem_get(addr) as Word
    }
    2 => {
      u16::from_le_bytes([pmem_get(addr), pmem_get(addr + 1)]) as Word
    }
    4 => {
      u32::from_le_bytes([pmem_get(addr), pmem_get(addr + 1), pmem_get(addr + 2), pmem_get(addr + 3)])
    }
    _ => {
      crumble!("Address align length [{}] is invalid, expect [1/2/4]", len);
    }
  }
}
