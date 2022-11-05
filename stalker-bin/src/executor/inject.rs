use anyhow::Result;
use stalker_injector::{Injectable, InjectionConfig};
use stalker_utils::{config::LibConfig, context::Context};

pub fn exec(ctx: &mut Context, lib_config: &LibConfig, inj_config: &InjectionConfig) -> Result<()> {
    let injection = ctx.inject(&inj_config.target_sym);
    for i in injection {
        i.perform(ctx, lib_config, &inj_config)?;
    }
    Ok(())
}
