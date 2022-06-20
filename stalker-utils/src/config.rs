use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Arch {
    pub arch: String,
    pub bits: u8,
}

impl Default for Arch {
    fn default() -> Self {
        Arch {
            arch: "arm".into(),
            bits: 64,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct StalkerConfig {
    pub arch: Arch,
}

impl Arch {
    pub fn to_cli_arg(&self) -> String {
        let mut args = String::new();
        args += &format!("-a {} ", self.arch);
        args += &format!("-b {}", self.bits);
        args
    }
}
