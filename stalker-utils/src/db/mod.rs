use crate::context::Context;
use crate::tag::Tag;
use crate::Result;
use std::fs::create_dir_all;

pub struct Db {
    pub mutant: sled::Db,
    pub instruction: sled::Db,
}

impl Db {
    pub fn new(ctx: &Context, db_path: &str) -> Result<Db> {
        let identity = &ctx.id();
        create_dir_all(format!("{}/mutant", db_path))?;
        create_dir_all(format!("{}/target/{}", db_path, identity))?;
        let mutant_db = sled::open(format!("{}/mutant", db_path))?;
        let db = Db {
            instruction: sled::open(format!("{}/target/{}/instruction", db_path, identity))?,
            mutant: mutant_db,
        };
        Ok(db)
    }

    pub fn list_injections(&self) {}
}
