static mut BATCH_MODE: bool = false;

pub fn sdb_init(is_batch:bool) {
  unsafe { BATCH_MODE = is_batch; }
}
