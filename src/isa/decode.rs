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
  instpat!("0000000 ????? ????? 000 ????? 01100 11", "add");
  instpat!("0100000 ????? ????? 000 ????? 01100 11", "sub");
  instpat!("0000000 ????? ????? 001 ????? 01100 11", "sll");
  instpat!("0000000 ????? ????? 010 ????? 01100 11", "slt");
  instpat!("0000000 ????? ????? 011 ????? 01100 11", "sltu");
  instpat!("0000000 ????? ????? 100 ????? 01100 11", "xor");
  instpat!("0000000 ????? ????? 101 ????? 01100 11", "srl");
  instpat!("0100000 ????? ????? 101 ????? 01100 11", "sra");
  instpat!("0000000 ????? ????? 110 ????? 01100 11", "or");
  instpat!("0000000 ????? ????? 111 ????? 01100 11", "and");

  instpat!("??????? ????? ????? 000 ????? 00100 11", "addi");
  instpat!("0000000 ????? ????? 001 ????? 00100 11", "slli");
  instpat!("??????? ????? ????? 010 ????? 00100 11", "slti");
  instpat!("??????? ????? ????? 011 ????? 00100 11", "sltiu");
  instpat!("??????? ????? ????? 100 ????? 00100 11", "xori");
  instpat!("0000000 ????? ????? 101 ????? 00100 11", "srli");
  instpat!("0100000 ????? ????? 101 ????? 00100 11", "srai");
  instpat!("??????? ????? ????? 110 ????? 00100 11", "ori");
  instpat!("??????? ????? ????? 111 ????? 00100 11", "andi");

  instpat!("??????? ????? ????? 000 ????? 00000 11", "lb");
  instpat!("??????? ????? ????? 001 ????? 00000 11", "lh");
  instpat!("??????? ????? ????? 010 ????? 00000 11", "lw");
  instpat!("??????? ????? ????? 100 ????? 00000 11", "lbu");
  instpat!("??????? ????? ????? 101 ????? 00000 11", "lhu");
  instpat!("??????? ????? ????? 110 ????? 00000 11", "lwu");
  instpat!("??????? ????? ????? 000 ????? 01000 11", "sb");
  instpat!("??????? ????? ????? 001 ????? 01000 11", "sh");
  instpat!("??????? ????? ????? 010 ????? 01000 11", "sw");

  instpat!("??????? ????? ????? ??? ????? 00101 11", "auipc");
  instpat!("0000000 00001 00000 000 00000 11100 11", "ebreak");
  /* INSTPAT END */

  return "inv";
}

// TODO: try to improve the code to put down the time spending
///
/// == almost up to 70% time spend of execution ==
///        == is taken place here ==
/// This is the real boss of time consuming
/// time spending will keep increasing for
/// every time when adding a new instruction
/// which produces another pattern match
///
#[rustfmt::skip]
fn inst_pat(inst: Word, pattern: &str) -> bool {
  // 1. generate key and mask to pair with inst later
  let mut key: Word = 0;
  let mut mask: Word = 0;
  let mut len = 0;
  for c in pattern.chars() {
    // 1.1. check args
    if c != ' ' {
      len += 1;
      alert!(
        len <= (size_of::<Word>() * 8),
        "Pattern string length is not paired with instruction [{}]\nAfter whitespace was removed",
        size_of::<Word>() * 8
      );
      alert!(
        c != '0' || c != '1' || c != '?',
        "Invalid character '{}' in pattern string", c
      );
      key  = (key <<1) | (if c == '1' {1} else {0});
      mask = (mask<<1) | (if c == '?' {0} else {1});
    }
  }
  // 2. and mask and pair with key
  inst & mask == key
}
