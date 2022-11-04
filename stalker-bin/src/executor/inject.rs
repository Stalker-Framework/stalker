use anyhow::Result;
use log::debug;
use stalker_injector::Injectable;
use stalker_utils::context::Context;

pub fn exec(ctx: &mut Context, func_name: &str) -> Result<()> {
    let injection = ctx.inject(func_name);
    for i in injection.take(64) {
        debug!("{}", i);
    }
    Ok(())
}
