use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryInfo {
    pub core: CoreInfo,
    pub bin: BinInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreInfo {
    pub file: String,
    pub size: usize,
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinInfo {
    pub arch: String,
    pub bits: u8,
    pub os: String,
    pub endian: String,
}
