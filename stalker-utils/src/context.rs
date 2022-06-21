use super::binary::BinaryInfo;
use super::config::Config;
use super::db::Db;
use super::loc::LibInstance;
use super::Result;
use rzpipe::RzPipe;
use std::boxed::Box;

pub struct Context {
    pub config: Config,
    pub binary_info: BinaryInfo,
    pub rz: Box<RzPipe>,
    pub db: Option<Db>,
    pub lib: LibInstance,
}

impl Default for Context {
    fn default() -> Self {
        let mut rz = RzPipe::spawn("/bin/ls", None).expect("Spawn pipe");
        rz.cmd("aa").expect("Perform analysis");
        let mut config = Config::default();
        let binary_info_s = rz.cmd("ij").expect("");
        let binary_info: BinaryInfo = serde_json::from_str(&binary_info_s).unwrap();
        config.arch.arch = binary_info.bin.arch.clone();
        config.arch.bits = binary_info.bin.bits;
        Context {
            config,
            rz: Box::new(rz),
            binary_info,
            db: None,
            lib: LibInstance::default()
        }
    }
}

impl Context {
    pub fn init_db(&mut self) -> Result<()> {
        if self.db.is_none() {
            let db = Db::new(&self.config.db_path)?;
            self.db = Some(db);
        }
        Ok(())
    }
}
