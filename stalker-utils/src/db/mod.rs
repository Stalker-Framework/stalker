use crate::Result;
use std::collections::HashMap;

pub struct Db {
    pub mutant: sled::Tree,
    pub injection: HashMap<&'static str, sled::Tree>,
    pub instruction: HashMap<&'static str, sled::Tree>,
}

impl Db {
    pub fn new(db_path: &str) -> Result<Db> {
        let db = sled::open(db_path)?;
        let db = Db {
            injection: std::collections::HashMap::default(),
            mutant: db.open_tree("mutant")?,
            instruction: std::collections::HashMap::default(),
        };
        Ok(db)
    }
}
