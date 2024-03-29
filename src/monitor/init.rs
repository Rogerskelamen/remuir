// use std::env;

use std::{fs::File, io, path::PathBuf};

use structopt::StructOpt;

use crate::memory::pmem::init_mem;

use super::sdb::sdb_init;

#[derive(StructOpt, Debug)]
struct Opt {
  #[structopt(name = "image", help = "the image file path to load")]
  image: Option<PathBuf>,

  #[structopt(short = "-b", long = "--batch", help = "run in batch mode")]
  _batch: bool
}

fn load_img(image: Option<PathBuf>) {
  let mut flag: bool = false;
  let file: io::Result<File>;
  match image {
    Some(path) => {
      file = File::open(path);

    },
    None => {
      println!("Default image loaded");
      flag = true;
    }
  }
}

pub fn init_monitor() {
  /* Parse arguments */
  let opts = Opt::from_args();
  println!("{:#?}", opts);

  let pmem = init_mem();
  println!("{:?}", pmem);

  load_img(opts.image);

  sdb_init();
  welcome();
}

fn welcome() {
  println!("Welcome to remuir!");
}
