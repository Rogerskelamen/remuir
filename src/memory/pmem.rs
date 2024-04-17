use crate::{alert, memory::access::PMEM, utils::config::*};

use super::access::{check_bound, pmem_read, pmem_write};

pub fn init_mem(buf: &Vec<Byte>) -> usize {
  println!("{{");
  for (idx, byte) in buf.iter().enumerate() {
    unsafe {
      PMEM[idx] = *byte;
    }
    println!("  {:#x}: {}", idx + MBASE, byte);
  }
  println!("}}");
  buf.len()
}

const DEF_IMG_NR: usize = 5;
pub fn load_default_img() -> Vec<u8> {
  let default_img: [u32; DEF_IMG_NR] = [
    0x00000297, // auipc t0, 0
    0x00028823, // sb zero, 16(t0)
    0x0102c503, // lbu a0, 16(t0)
    0x00100073, // ebreak (as a sign to exit)
    0xdeadbeef, // dummy data
  ];

  // IMG -> img : [u32;5] -> [u8;20]
  let img: [u8; DEF_IMG_NR * 4] = default_img
    .iter()
    .flat_map(|&x| {
      let mut bytes = [0; 4];
      bytes.copy_from_slice(&x.to_ne_bytes());
      bytes.to_vec()
    })
    .collect::<Vec<u8>>()
    .try_into()
    .unwrap();
  let buf = img.to_vec();
  buf
}

pub fn mem_read(addr: Addr, len: usize) -> Word {
  if !check_bound(addr, len) {
    alert!(false, "Address [{:#x} - {:#x}] out of Memory", addr, addr as usize + len);
  }
  pmem_read(addr, len)
}

pub fn mem_write(addr: Addr, data: Word, len: usize) {
  if !check_bound(addr, len) {
    alert!(false, "Address [{:#x} - {:#x}] out of Memory", addr, addr as usize + len);
  }
  pmem_write(addr, data, len);
}
