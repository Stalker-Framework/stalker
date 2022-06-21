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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Config {
    pub arch: Arch,
    pub db_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            arch: Arch::default(),
            db_path: "/tmp/stalker.db".into(),
        }
    }
}
