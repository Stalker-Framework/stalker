use anyhow::Result;
use log::warn;
use rayon::prelude::*;
use stalker_injector::{Change, Injectable, InjectionConfig};
use stalker_mutator::FaultModel;
use stalker_utils::{config::LibConfig, context::Context};

pub fn exec<M: FaultModel>(
    ctx: &mut Context,
    lib_config: &LibConfig,
    inj_config: &InjectionConfig,
    parallel: bool,
) -> Result<()> {
    let injections = inj_config.target_syms.iter().map(|i| ctx.inject::<M>(i));
    let iter = injections.flatten();
    let closure = |i: Change| {
        i.perform::<M>(ctx, lib_config, inj_config)
            .expect("Not normal.");
    };

    if parallel {
        warn!("Using rayon parallel iterator");
        iter.par_bridge().for_each(closure);
    } else {
        warn!("Using sequential iterator");
        iter.for_each(closure);
    }
    Ok(())
}
