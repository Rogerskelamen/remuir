use std::mem::size_of;

use crate::{alert, utils::config::Word};

#[rustfmt::skip]
pub fn find_inst(inst: Word) -> &'static str {
  macro_rules! instpat {
    ($p:literal, $r:literal) => {
      if inst_pat(inst, $p) { return $r; }
    };
  }

  /* INSTPAT START */
  instpat!("??????? ????? ????? ??? ????? 00101 11", "auipc");
  instpat!("??????? ????? ????? 100 ????? 00000 11", "lbu");
  instpat!("??????? ????? ????? 000 ????? 01000 11", "sb");
  instpat!("0000000 00001 00000 000 00000 11100 11", "ebreak");
  /* INSTPAT END */

  return "inv";
}


#[rustfmt::skip]
fn inst_pat(inst: Word, pattern: &str) -> bool {
  let p: String = pattern.split_whitespace().collect();
  // 0. check args
  alert!(
    p.len() == (size_of::<Word>() * 8),
    "Pattern string length [{}] is not paired with instruction [{}]\nAfter whitespace was removed",
    p.len(),
    size_of::<Word>() * 8
  );
  // 1. generate key and mask to pair with inst later
  let mut key: Word = 0;
  let mut mask: Word = 0;
  for c in p.chars() {
    alert!(
      c != '0' || c != '1' || c != '?',
      "Invalid character '{}' in pattern string", c
    );
    key  = (key  << 1) | (if c == '1' {1} else {0});
    mask = (mask << 1) | (if c == '?' {0} else {1});
  }
  // 2. and mask and pair with key
  inst & mask == key
}
