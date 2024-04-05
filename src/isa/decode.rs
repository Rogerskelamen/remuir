use crate::utils::config::Word;

pub fn inst_pad(inst: Word) -> &'static str {
  if inst == 0x100073 {
    "ebreak"
  } else {
    "inv"
  }
}
