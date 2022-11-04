use anyhow::Result;
use stalker_injector::Injectable;
use stalker_utils::context::Context;
use std::env;

/// It is required to run `gen-liblocs` and `gen-mutants` first.
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut ctx = Context::new(args.get(1).unwrap_or(&String::from("/bin/ls")))?;
    let default_function_name = String::from("main");
    let loc_name = args.get(2).unwrap_or(&default_function_name);

    ctx.init_db()?;
    println!("Injections for location `{}`:", loc_name);
    let injection = ctx.inject(loc_name);
    for i in injection.take(64) {
        println!("{}", i);
    }
    Ok(())
}
