use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("parse error, nom output: {0}")]
    Parse(String),
    #[error("disassembled error.")]
    Disasm,
    #[error("rizin pipe error")]
    RzPipe(#[from] rzpipe::errors::RzPipeError),
    #[error("hex error")]
    HexDecode(#[from] hex::FromHexError),
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("db error")]
    DbError(#[from] sled::Error),
}
