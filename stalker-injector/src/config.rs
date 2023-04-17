use std::fs::create_dir_all;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use stalker_mutator::FaultModel;
use stalker_utils::{config::LibConfig, context::Context, tag::Tag};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(default)]
pub struct InjectionConfig {
    pub work_dir: String,
    pub group: String,
    pub name: String,
    pub target_syms: Vec<String>,
    pub exec_command: String,
    pub exec_args: Vec<String>,
    pub dry_run: bool,
}

impl Default for InjectionConfig {
    fn default() -> Self {
        InjectionConfig {
            work_dir: "examples/libgcrypt/aes".into(),
            group: "symmetric".into(),
            name: "aes".into(),
            target_syms: vec![],
            exec_command: "./bin".into(),
            exec_args: vec![],
            dry_run: true,
        }
    }
}

impl InjectionConfig {
    pub fn init<M: FaultModel>(&self, ctx: &Context, _lib_config: &LibConfig) -> Result<()> {
        let dir = format!(
            "data/stalker/output/{}/{}/{}",
            M::tag(),
            ctx.id(),
            &self.name,
        );
        create_dir_all(dir)?;
        Ok(())
    }
}
