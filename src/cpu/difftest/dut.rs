use std::{
  ffi::{c_int, c_void},
  path::PathBuf,
  ptr::addr_of,
  sync::Mutex,
};

use lazy_static::lazy_static;

use crate::{
  cpu::core::{gpr_get, isa_gpr_print, pc_get, Cpu, CORE, REGNAME},
  engine::control::{ExecState, EMUSTATE},
  log,
  memory::access::PMEM,
  utils::config::*,
};

static mut HAS_DIFFTEST: bool = false;
static mut DIFFTEST_SKIP: bool = false;

const DIFFTEST_TO_DUT: c_int = 0;
const DIFFTEST_TO_REF: c_int = 1;

#[rustfmt::skip]
lazy_static! {
  static ref DIFF_REGCPY: Mutex<Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_void>> =
    Mutex::new(None);
  static ref DIFF_EXEC: Mutex<Option<unsafe extern "C" fn(u64) -> c_void>> =
    Mutex::new(None);
}

#[rustfmt::skip]
pub fn init_difftest(diff: Option<PathBuf>, img_size: usize) {
  match diff {
    Some(path) => {
      let lib = libloading::Library::new(path).unwrap();
      let ref_difftest_init: libloading::Symbol<unsafe extern "C" fn(c_int) -> c_void> =
        unsafe { lib.get(b"difftest_init").expect("failed to load symbol [difftest_init]") };

      let difftest_memcpy: libloading::Symbol<
        unsafe extern "C" fn(u32, *mut c_void, usize, c_int) -> c_void,
      > = unsafe { lib.get(b"difftest_memcpy").expect("failed to load symbol [difftest_memcpy]") };

      let difftest_regcpy: libloading::Symbol<unsafe extern "C" fn(*mut c_void, c_int) -> c_void> =
        unsafe { lib.get(b"difftest_regcpy").expect("failed to load symbol [difftest_regcpy]") };
      *DIFF_REGCPY.lock().unwrap() = Some(*difftest_regcpy);

      let difftest_exec: libloading::Symbol<unsafe extern "C" fn(u64) -> c_void> =
        unsafe { lib.get(b"difftest_exec").expect("failed to load symbol [difftest_exec]") };
      *DIFF_EXEC.lock().unwrap() = Some(*difftest_exec);

      unsafe {
        /* Initiailize simulator reference */
        ref_difftest_init(1234);

        let pmem_ptr = addr_of!(PMEM) as *const [Byte; MSIZE];
        difftest_memcpy(MBASE as Addr, pmem_ptr as *mut c_void, img_size, DIFFTEST_TO_REF);

        let core_ptr = addr_of!(CORE) as *const Cpu;
        difftest_regcpy(core_ptr as *mut c_void, DIFFTEST_TO_REF);
      }

      unsafe { HAS_DIFFTEST = true; }
    }
    None => {}
  }
}

pub fn difftest_step(pc: Addr) {
  let ref_r: Cpu = Cpu::default();
  let ref_r_ptr = addr_of!(ref_r) as *const Cpu;

  /* Check if skip */
  unsafe {
    if DIFFTEST_SKIP {
      ref_difftest_regcpy(ref_r_ptr as *mut c_void, DIFFTEST_TO_DUT);
      DIFFTEST_SKIP = false;
      return;
    }
  }

  /* Step execution */
  ref_difftest_exec(1);
  ref_difftest_regcpy(ref_r_ptr as *mut c_void, DIFFTEST_TO_DUT);

  checkregs(ref_r, pc);
}

#[rustfmt::skip]
pub fn difftest_skip_ref() {
  unsafe { DIFFTEST_SKIP = true; }
}

pub fn has_difftest() -> bool {
  unsafe { HAS_DIFFTEST }
}

fn checkregs(ref_r: Cpu, pc: Addr) {
  if !difftest_checkregs(ref_r, pc) {
    unsafe {
      EMUSTATE.state = ExecState::Abort;
      EMUSTATE.halt_pc = pc;
      isa_gpr_print();
    }
  }
}

fn difftest_checkregs(ref_r: Cpu, pc: Addr) -> bool {
  /* Check for pc */
  if !log_difftest_checkregs("pc", pc, ref_r.pc, pc_get()) {
    return false;
  }
  /* Check for gpr */
  for (i, reg) in ref_r.gpr.into_iter().enumerate() {
    if !log_difftest_checkregs(REGNAME[i], pc, reg, gpr_get(i)) {
      return false;
    }
  }
  return true;
}

fn log_difftest_checkregs(name: &str, pc: Addr, ref_r: Word, dut_r: Word) -> bool {
  if ref_r != dut_r {
    log!("{} is different after executing instruction at pc = {:#x}, right = {:#x}, wrong = {:#x}, diff = {:#x}", name, pc, ref_r, dut_r, ref_r ^ dut_r);
    return false;
  }
  return true;
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
