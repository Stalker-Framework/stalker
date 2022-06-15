use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Arch {
  pub arch: String,
  pub bits: u8,
}

impl Default for Arch {
  fn default() -> Self {
    Arch {
      arch: "x86".into(),
      bits: 64,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct StalkerConfig {
  pub arch: Arch,
}

impl StalkerConfig {
  pub fn to_cli_args(&self) -> String {
    self.arch.to_cli_args()
  }
}

impl Arch {
  pub fn to_cli_args(&self) -> String {
    let mut args = String::new();
    args += &format!("-a {} ", self.arch);
    args += &format!("-b {}", self.bits);
    args
  }
}
