use super::binary::BinaryInfo;
use super::config::Config;
use super::db::Db;
use super::loc::LibInstance;
use super::Result;
use rzpipe::RzPipe;
use std::boxed::Box;
use std::path::Path;

pub struct Context {
    pub config: Config,
    pub binary_info: BinaryInfo,
    pub rz: Box<RzPipe>,
    pub db: Option<Db>,
    pub lib: LibInstance,
}

impl Default for Context {
    fn default() -> Self {
        Context::new("/bin/ls").unwrap()
    }
}

impl Context {
    pub fn new(lib_path: &str) -> Result<Context> {
        let mut rz = RzPipe::spawn(lib_path, None).expect("Spawn pipe");
        rz.cmd("aa")?;
        let mut config = Config::default();
        let binary_info_s = rz.cmd("ij")?;
        let binary_info: BinaryInfo = serde_json::from_str(&binary_info_s)?;
        config.arch.arch = binary_info.bin.arch.clone();
        config.arch.bits = binary_info.bin.bits;
        Ok(Context {
            config,
            rz: Box::new(rz),
            binary_info,
            db: None,
            lib: LibInstance::default(),
        })
    }

    pub fn init_db(&mut self) -> Result<()> {
        if self.db.is_none() {
            let db = Db::new(self, &self.config.db_path)?;
            self.db = Some(db);
        }
        Ok(())
    }

    pub fn identity(&self) -> String {
        let file = Path::new(&self.binary_info.core.file);
        let filename = file.file_name().unwrap().to_str().unwrap();
        format!(
            "{}-{}-{}",
            &self.config.arch, &self.binary_info.bin.os, &filename
        )
    }
}
