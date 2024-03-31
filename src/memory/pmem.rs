// use std::mem;

static IMG: [u32;5] = [
  0x00000297, // auipc t0, 0
  0x00028823, // sb zero, 16(t0)
  0x0102c503, // lbu a0, 16(t0)
  0x00100073, // ebreak (as a sign to exit)
  0xdeadbeef, // dummy data
];

pub fn _init_mem() {
  // unsafe { mem::zeroed() }
}

pub fn load_default_img() -> Vec<u8> {
  // IMG -> img : [u32;5] -> [u8;20]
  let img: [u8;20] = IMG
    .iter()
    .flat_map(|&x| {
      let mut bytes = [0;4];
      bytes.copy_from_slice(&x.to_le_bytes());
      bytes.to_vec()
    })
    .collect::<Vec<u8>>()
    .try_into()
    .unwrap();
  let buf = img.to_vec();
  buf
}
