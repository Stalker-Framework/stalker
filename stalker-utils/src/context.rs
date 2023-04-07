use crate::tag::Tag;

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
    pub rz: Box<RzPipe>,
    pub db: Option<Db>,
    pub lib: LibInstance,
}

pub struct PreContext {
    pub lib_path: String,
    pub config: Option<Config>,
    pub rz: Option<Box<RzPipe>>,
}

impl Default for Context {
    fn default() -> Self {
        Context::pre("/bin/ls").init().unwrap()
    }
}

impl Default for PreContext {
    fn default() -> Self {
        Self {
            lib_path: "/bin/ls".to_string(),
            config: None,
            rz: None,
        }
    }
}

impl PreContext {
    pub fn init(self) -> Result<Context> {
        let mut rz = RzPipe::spawn(&self.lib_path, None)?;
        if let Some(config) = self.config {
            let config = config.update(&mut rz)?;
            let rzdb = format!("{}/{}.rzdb", config.rz_path, config.id());
            if Path::exists(Path::new(&rzdb)) {
                info!("Found rizin db, loading...");
                Context::init_from_saved_rzdb(rz, &rzdb, config)
            } else {
                warn!("Rizin db not found, creating...");
                create_dir_all(&config.rz_path)?;
                Context::init_from_scratch_with_config(rz, &rzdb, config)
            }
        } else {
            warn!("Config not present, initializing...");
            Context::init_from_scratch(rz)
        }
    }

    pub fn with_lib(self, lib_path: &str) -> Self {
        Self {
            lib_path: lib_path.to_string(),
            ..self
        }
    }

    pub fn with_config(self, config: Config) -> Self {
        Self {
            config: Some(config),
            ..self
        }
    }

    pub fn data_path(self, data_path: &str) -> Self {
        Self {
            config: Some(Config {
                rz_path: data_path.to_string() + "/rizin",
                db_path: data_path.to_string(),
                ..self.config.unwrap_or_default()
            }),
            ..self
        }
    }
}

impl Context {
    pub fn pre(lib_path: &str) -> PreContext {
        PreContext {
            config: None,
            lib_path: lib_path.to_string(),
            ..Default::default()
        }
    }

    pub fn new(lib_path: &str) -> Result<Context> {
        Context::pre(lib_path).init()
    }

    fn init_from_scratch_with_config(
        mut rz: RzPipe,
        rzdb: &str,
        config: Config,
    ) -> Result<Context> {
        rz.cmd(&format!("Ps {}", rzdb))?;
        info!("Saved rizin db at {}", rzdb);
        Ok(Context {
            config,
            rz: Box::new(rz),
            db: None,
            lib: LibInstance::default(),
        })
    }

    fn init_config(rz: &mut RzPipe) -> Result<Config> {
        Config::default().update(rz)
    }

    fn init_from_scratch(mut rz: RzPipe) -> Result<Context> {
        let config = Self::init_config(&mut rz)?;
        create_dir_all(&config.rz_path)?;
        let rzdb = format!("{}/{}.rzdb", config.rz_path, config.id());
        rz.cmd(&format!("Ps {}", rzdb))?;
        info!("Saved rizin db at {}", rzdb);
        Ok(Context {
            config,
            rz: Box::new(rz),
            db: None,
            lib: LibInstance::default(),
        })
    }

    fn init_from_saved_rzdb(mut rz: RzPipe, rzdb: &str, config: Config) -> Result<Context> {
        rz.cmd(&format!("Po {}", rzdb))?;
        let ctx = Context {
            config: config.update(&mut rz)?,
            rz: Box::new(rz),
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
}

impl Tag for Context {
    fn id(&self) -> String {
        self.config.id()
    }

    fn tag() -> String {
        "context".into()
    }
}
