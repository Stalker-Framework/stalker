use super::config::StalkerConfig;
use rzpipe::RzPipe;
use std::boxed::Box;

pub struct Context {
  pub config: StalkerConfig,
  pub rz: Box<RzPipe>,
}

impl Default for Context {
  fn default() -> Self {
    let mut rz = RzPipe::spawn("/bin/ls", None).expect("Spawn pipe");
    rz.cmd("aaa").expect("Perform analysis");
    Context {
      config: StalkerConfig::default(),
      rz: Box::new(rz),
    }
  }
}
