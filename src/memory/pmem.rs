use std::mem;

use crate::utils::constant::MSIZE;

pub fn init_mem() -> [u8;MSIZE] {
  unsafe { mem::zeroed() }
}
