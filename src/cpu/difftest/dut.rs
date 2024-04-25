use std::path::PathBuf;

static mut HAS_DIFFTEST: bool = false;

pub fn init_difftest(diff: Option<PathBuf>) {
  match diff {
    Some(f) => {
      unsafe {
        HAS_DIFFTEST = true;
        let lib = libloading::Library::new(f).unwrap();
      }
    }
    None => {
    }
  }
}
