mod cli;
mod executor;
mod generator;

use cli::run;
use stalker_mutator::model::*;

fn main() -> anyhow::Result<()> {
    // run::<Bitflip>()?;
    // run::<Stuck<0xff>>()?;
    run::<Stuck<0x00>>()?;
    Ok(())
}
