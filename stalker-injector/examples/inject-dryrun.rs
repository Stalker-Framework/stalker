use anyhow::Result;
use stalker_injector::Injectable;
use stalker_utils::context::Context;

/// It is required to run `gen-liblocs` and `gen-mutants` first.
fn main() -> Result<()> {
    let loc_name = "sym.func.100004b88";
    let mut ctx = Context::default();
    ctx.init_db()?;
    println!("Injections for location `{}`:", loc_name);
    let injection = ctx.inject(loc_name);
    for i in injection {
        println!("{:?}", i);
    }
    Ok(())
}
