// user config
pub const CONFIG_ITRACE: bool = true;
pub const CONFIG_ATRACE: bool = false; // trace for args

// global constant config
pub const MBASE: usize = 0x80000000;
pub const MSIZE: usize = 0x8000000;
pub const GPR_NR: usize = 32;

// io device address
pub const SERIAL_PORT: Addr = 0xa00003f8;

// type alias
pub type Word = u32;
pub type SWord = i32;
pub type SDoubl = i64;
pub type Doubl = u64;
pub type Addr = u32;
pub type Byte = u8;
