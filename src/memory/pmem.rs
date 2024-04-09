use std::{collections::BTreeMap, sync::Mutex};

use lazy_static::lazy_static;

use crate::utils::config::{Addr, Byte, MBASE};

lazy_static! {
  pub static ref PMEM: Mutex<BTreeMap<Addr, Byte>> =
    Mutex::new(BTreeMap::new());
}

pub fn init_mem(buf: &Vec<Byte>) -> usize {
  let mut pmem = PMEM.lock().unwrap();
  for (idx, &byte) in buf.iter().enumerate() {
    pmem.insert((MBASE + idx) as Addr, byte);
  }
  println!("{{");
  for (key, value) in &mut *pmem {
    println!("  {:#x}: {}", key, value);
  }
  println!("}}");
  pmem.len()
}

pub fn load_default_img() -> Vec<u8> {
  let default_img: [u32; 5] = [
    0x00000297, // auipc t0, 0
    0x00028823, // sb zero, 16(t0)
    0x0102c503, // lbu a0, 16(t0)
    0x00100073, // ebreak (as a sign to exit)
    0xdeadbeef, // dummy data
  ];

  // IMG -> img : [u32;5] -> [u8;20]
  let img: [u8; 20] = default_img
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
