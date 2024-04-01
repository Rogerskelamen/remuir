// use std::env;

use std::{fs::File, io::Read, path::PathBuf};

use structopt::StructOpt;

use crate::{log, memory::pmem::load_default_img};

use super::sdb::sdb_init;

#[derive(StructOpt, Debug)]
struct Opt {
  #[structopt(name = "image", help = "the image file path to load")]
  image: Option<PathBuf>,

  #[structopt(short = "-b", long = "--batch", help = "run in batch mode")]
  _batch: bool,
}

fn load_img(image: Option<PathBuf>) -> Vec<u8> {
  match image {
    Some(path) => {
      let mut f = File::open(path).expect("Please give a valid binary file");
      let mut buf = Vec::new();
      f.read_to_end(&mut buf).expect("Can't read the binary file");
      buf
    }
    None => {
      log!("Default image loaded");
      load_default_img()
    }
  }
}

pub fn init_monitor() {
  /* Parse arguments */
  let args = Opt::from_args();
  println!("{:#?}", args);

  // let pmem = init_mem();
  // println!("{:?}", pmem);

  let buf = load_img(args.image);
  println!("{:?}", buf);

  sdb_init();
  welcome();
}

fn welcome() {
  println!("Welcome to remuir!");
}
