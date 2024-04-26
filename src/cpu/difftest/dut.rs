use std::{ffi::{c_int, c_void}, path::PathBuf, sync::Mutex};

use lazy_static::lazy_static;

lazy_static! {
  static ref DIFF_MEMCPY: Mutex<Option<unsafe extern fn(u32, &c_void, isize, bool) -> c_void>> = Mutex::new(None);
}

pub fn init_difftest(diff: Option<PathBuf>) {
  match diff {
    Some(path) => {
      let lib = libloading::Library::new(path).unwrap();
      let ref_difftest_init: libloading::Symbol<unsafe extern fn(c_int) -> c_void> = unsafe {
        lib.get(b"difftest_init").expect("failed to load symbol [difftest_init]")
      };
      unsafe { ref_difftest_init(1234) };
      let difftest_memcpy: libloading::Symbol<unsafe extern fn(u32, &c_void, isize, bool) -> c_void> = unsafe {
          lib.get(b"difftest_memcpy").expect("failed to load symbol [difftest_memcpy]")
      };
      *DIFF_MEMCPY.lock().unwrap() = Some(*difftest_memcpy);
    }
    None => {}
  }
}

pub fn ref_difftest_memcpy(addr: u32, buf: &c_void, n: isize, direction: bool) -> c_void {
  let func = DIFF_MEMCPY.lock().unwrap();
  let func = func.as_ref().expect("Function not initialized");
  unsafe { func(addr, buf, n, direction) }
}
