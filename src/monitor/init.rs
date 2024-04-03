// use std::env;

use std::{fs::File, io::Read, path::PathBuf};

use structopt::StructOpt;

use crate::{
  log,
  memory::pmem::{init_mem, load_default_img},
};

use super::sdb::sdb_init;

#[derive(StructOpt, Debug)]
struct Opt {
  #[structopt(name = "image", help = "the image file path to load")]
  image: Option<PathBuf>,

  #[structopt(short = "-b", long = "--batch", help = "run in batch mode")]
  batch: bool,
}

pub fn init_monitor() {
  /* Parse arguments */
  let args = Opt::from_args();
  println!("{:#?}", args);

  let imgsize = load_img(args.image);
  log!("Image loaded {} bytes", imgsize);

  sdb_init(args.batch);

  welcome(args.batch);
}

fn load_img(image: Option<PathBuf>) -> usize {
  let mut buf = Vec::new();

  match image {
    Some(path) => {
      let mut f = File::open(path).expect("Please give a valid binary file");
      f.read_to_end(&mut buf).expect("Can't read the binary file");
    }
    None => {
      log!("Loading default image...");
      buf = load_default_img();
    }
  }
  init_mem(&buf)
}

fn welcome(isbatch: bool) {
  let name = "remuir";
  println!("Welcome to \u{001b}[44;30;1m{}\u{001b}[0m!", name);
  if isbatch {
    println!("For help, type \"help\"");
  }
}
