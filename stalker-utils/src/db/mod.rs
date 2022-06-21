use crate::Result;
use std::collections::HashMap;

pub struct Db {
    pub mutants: sled::Tree,
    pub injection: HashMap<&'static str, sled::Tree>,
}

impl Db {
    pub fn new(db_path: &str) -> Result<Db> {
        let db = sled::open(db_path)?;
        let db = Db {
            injection: std::collections::HashMap::default(),
            mutants: db.open_tree("mutants")?,
        };
        Ok(db)
    }
}
