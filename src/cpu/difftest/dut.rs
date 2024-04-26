use std::{ffi::c_void, path::PathBuf, sync::Arc};

struct HasDifftest {
  exist: bool,
  libstr: &'static str
}

static mut HAS_DIFFTEST: HasDifftest = HasDifftest {
  exist: false,
  libstr: ""
};

struct DifftestLib {
  exist: bool,
  lib: Arc<libloading::Library>,
  difftest_memcpy: libloading::Symbol<'static, unsafe extern fn(u32, c_void, isize, bool)>
}

pub fn init_difftest(diff: Option<PathBuf>) {
  match diff {
    Some(path) => {
      unsafe {
        HAS_DIFFTEST.exist = true;
        let path_str = path.as_path().to_str().expect("Failed to load difftest lib path");
        let boxed_str = path_str.to_owned().into_boxed_str();
        HAS_DIFFTEST.libstr = Box::leak(boxed_str);
      }
    }
    None => {}
  }
}

// let lib = libloading::Library::new(f).unwrap();
// let func: libloading::Symbol<unsafe extern fn()> = lib.get(b"difftest_memcpy").unwrap();
