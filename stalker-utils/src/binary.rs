use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BinaryInfo {
    pub core: CoreInfo,
    pub bin: BinInfo,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CoreInfo {
    pub file: String,
    pub size: usize,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinInfo {
    pub arch: String,
    pub bits: u8,
    pub os: String,
    pub endian: String,
}

impl Default for BinInfo {
    fn default() -> Self {
        BinInfo {
            arch: "arm".into(),
            bits: 64,
            os: "linux".into(),
            endian: "big".into(),
        }
    }
}
