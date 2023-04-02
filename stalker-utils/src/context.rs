use super::binary::BinaryInfo;
use super::config::Config;
use super::db::Db;
use super::loc::LibInstance;
use super::Result;
use log::{info, warn};
use rzpipe::RzPipe;
use std::boxed::Box;
use std::fs::create_dir_all;
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
        Context::new("/bin/ls", Some(Config::default())).unwrap()
    }
}

impl Context {
    pub fn new(lib_path: &str, optional_config: Option<Config>) -> Result<Context> {
        let rz = RzPipe::spawn(lib_path, None)?;
        if let Some(config) = optional_config {
            let rzdb = format!(
                "{}/{}.rzdb",
                config.rz_path,
                Self::identity(lib_path, &config)
            );
            if Path::exists(Path::new(&rzdb)) {
                info!("Found rizin db, loading...");
                Self::new_from_saved_rzdb(rz, &rzdb, config)
            } else {
                warn!("Rizin db not found, creating...");
                create_dir_all(&config.rz_path)?;
                Self::new_from_scratch_with_config(rz, &rzdb, lib_path, config)
            }
        } else {
            warn!("Config not present, initializing...");
            Self::new_from_scratch(rz, lib_path)
        }
    }

    fn new_from_scratch_with_config(
        mut rz: RzPipe,
        rzdb: &str,
        lib_path: &str,
        config: Config,
    ) -> Result<Context> {
        rz.cmd(&format!("o {}", lib_path))?;
        rz.cmd("aa")?;
        let binary_info_s = rz.cmd("ij")?;
        let binary_info: BinaryInfo = serde_json::from_str(&binary_info_s)?;
        rz.cmd(&format!("Ps {}", rzdb))?;
        info!("Saved rizin db at {}", rzdb);
        Ok(Context {
            config,
            rz: Box::new(rz),
            binary_info,
            db: None,
            lib: LibInstance::default(),
        })
    }

    fn new_from_scratch(mut rz: RzPipe, lib_path: &str) -> Result<Context> {
        rz.cmd("aa")?;
        let mut config = Config::default();
        let binary_info_s = rz.cmd("ij")?;
        let binary_info: BinaryInfo = serde_json::from_str(&binary_info_s)?;
        config.arch.arch = binary_info.bin.arch.clone();
        config.arch.bits = binary_info.bin.bits;
        create_dir_all(&config.rz_path)?;
        let rzdb = format!(
            "{}/{}.rzdb",
            config.rz_path,
            Context::identity(lib_path, &config)
        );
        rz.cmd(&format!("Ps {}", rzdb))?;
        info!("Saved rizin db at {}", rzdb);
        Ok(Context {
            config,
            rz: Box::new(rz),
            binary_info,
            db: None,
            lib: LibInstance::default(),
        })
    }

    fn new_from_saved_rzdb(mut rz: RzPipe, rzdb: &str, config: Config) -> Result<Context> {
        rz.cmd(&format!("Po {}", rzdb))?;
        let binary_info_s = rz.cmd("ij")?;
        let binary_info: BinaryInfo = serde_json::from_str(&binary_info_s)?;
        let ctx = Context {
            config,
            rz: Box::new(rz),
            binary_info,
            db: None,
            lib: LibInstance::default(),
        };
        Ok(ctx)
    }

    pub fn init_db(&mut self) -> Result<()> {
        if self.db.is_none() {
            let db = Db::new(self, &self.config.db_path)?;
            self.db = Some(db);
        }
        Ok(())
    }

    pub fn identity(path: &str, config: &Config) -> String {
        let file = Path::new(path);
        let filename = file.file_name().unwrap().to_str().unwrap();
        format!("{}-{}-{}", &config.arch, &config.os, &filename)
    }

    pub fn id(&self) -> String {
        Self::identity(&self.binary_info.core.file, &self.config)
    }
}
