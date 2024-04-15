use crate::{crumble, utils::config::*};

// 128KiB available RAM
pub static mut PMEM: [Byte; MSIZE] = [0; MSIZE];

fn pmem_get(addr: Addr) -> Byte {
  unsafe { PMEM[addr as usize - MBASE] }
}

fn pmem_set(addr: Addr, byte: u8) {
  unsafe {
    PMEM[addr as usize - MBASE] = byte;
  }
}

pub fn check_bound(addr: Addr, len: usize) -> bool {
  addr as usize >= MBASE && addr as usize + len <= MBASE + MSIZE
}

pub fn pmem_read(addr: Addr, len: usize) -> Word {
  match len {
    1 => pmem_get(addr) as Word,
    2 => u16::from_ne_bytes([pmem_get(addr), pmem_get(addr + 1)]) as Word,
    4 => u32::from_ne_bytes([
      pmem_get(addr),
      pmem_get(addr + 1),
      pmem_get(addr + 2),
      pmem_get(addr + 3),
    ]),
    _ => {
      crumble!("Address align length [{len}] is invalid, expect [1/2/4]");
    }
  }
}

pub fn pmem_write(addr: Addr, data: Word, len: usize) {
  match len {
    1 => {
      pmem_set(addr, data as Byte);
    }
    2 => {
      let bytes = data.to_ne_bytes();
      pmem_set(addr, bytes[0]);
      pmem_set(addr + 1, bytes[1]);
    }
    4 => {
      let bytes = data.to_ne_bytes();
      for (i, byte) in bytes.iter().enumerate() {
        pmem_set(addr + i as Addr, *byte);
      }
    }
    _ => {
      crumble!("Address align length [{len}] is invalid, expect [1/2/4]");
    }
  }
}
