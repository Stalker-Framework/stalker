use std::fs::create_dir_all;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use stalker_utils::{config::LibConfig, context::Context};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(default)]
pub struct InjectionConfig {
    pub work_dir: String,
    pub name: String,
    pub target_sym: String,
    pub exec_command: String,
    pub dry_run: bool,
}

impl Default for InjectionConfig {
    fn default() -> Self {
        InjectionConfig {
            work_dir: "examples/libgcrypt/aes".into(),
            name: "aes".into(),
            target_sym: "sym._gcry_aes_cfb_enc_armv8_ce".into(),
            exec_command: "./bin".into(),
            dry_run: true,
        }
    }
}

impl InjectionConfig {
    pub fn init(&self, ctx: &Context, lib_config: &LibConfig) -> Result<()> {
        let dir = format!(
            "data/stalker/output/{}/{}/{}/",
            ctx.id(),
            lib_config.link_name,
            &self.name,
        );
        create_dir_all(dir)?;
        Ok(())
    }
}
