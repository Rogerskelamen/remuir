use std::{
  ffi::{c_int, c_void},
  path::PathBuf,
  sync::Mutex,
};

use lazy_static::lazy_static;

use crate::utils::config::Addr;

pub static mut HAS_DIFFTEST: bool = false;

const DIFFTEST_TO_DUT: c_int = 0;
const DIFFTEST_TO_REF: c_int = 1;

#[rustfmt::skip]
lazy_static! {
  static ref DIFF_MEMCPY: Mutex<Option<unsafe extern "C" fn(u32, *mut c_void, usize, c_int) -> c_void>> =
    Mutex::new(None);
  static ref DIFF_REGCPY: Mutex<Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_void>> =
    Mutex::new(None);
  static ref DIFF_EXEC: Mutex<Option<unsafe extern "C" fn(u64) -> c_void>> =
    Mutex::new(None);
}

pub fn init_difftest(diff: Option<PathBuf>) {
  match diff {
    Some(path) => {
      let lib = libloading::Library::new(path).unwrap();
      let ref_difftest_init: libloading::Symbol<unsafe extern "C" fn(c_int) -> c_void> =
        unsafe { lib.get(b"difftest_init").expect("failed to load symbol [difftest_init]") };

      /* Initiailize simulator reference */
      unsafe { ref_difftest_init(1234) };

      let difftest_memcpy: libloading::Symbol<
        unsafe extern "C" fn(u32, *mut c_void, usize, c_int) -> c_void,
      > = unsafe { lib.get(b"difftest_memcpy").expect("failed to load symbol [difftest_memcpy]") };
      *DIFF_MEMCPY.lock().unwrap() = Some(*difftest_memcpy);

      let difftest_regcpy: libloading::Symbol<unsafe extern "C" fn(*mut c_void, c_int) -> c_void> =
        unsafe { lib.get(b"difftest_regcpy").expect("failed to load symbol [difftest_regcpy]") };
      *DIFF_REGCPY.lock().unwrap() = Some(*difftest_regcpy);

      let difftest_exec: libloading::Symbol<unsafe extern "C" fn(u64) -> c_void> =
        unsafe { lib.get(b"difftest_exec").expect("failed to load symbol [difftest_exec]") };
      *DIFF_EXEC.lock().unwrap() = Some(*difftest_exec);

      // unsafe {
        // difftest_regcpy(CORE as c_void, DIFFTEST_TO_REF);
      // }

      unsafe { HAS_DIFFTEST = true; }
    }
    None => {}
  }
}

pub fn difftest_step(pc: Addr, npc: Addr) {
  
}

fn ref_difftest_memcpy(addr: u32, buf: *mut c_void, n: usize, direction: c_int) -> c_void {
  let func = DIFF_MEMCPY.lock().unwrap();
  let func = func.as_ref().expect("Function not initialized");
  unsafe { func(addr, buf, n, direction) }
}

fn ref_difftest_regcpy(dut: *mut c_void, direction: c_int) -> c_void {
  let func = DIFF_REGCPY.lock().unwrap();
  let func = func.as_ref().expect("Function not initialized");
  unsafe { func(dut, direction) }
}

fn ref_difftest_exec(n: u64) -> c_void {
  let func = DIFF_EXEC.lock().unwrap();
  let func = func.as_ref().expect("Function not initialized");
  unsafe { func(n) }
}
