mod cli;
mod executor;
mod generator;

use clap::Parser;
use cli::{run, Cli};
use log::info;
use stalker_mutator::model::*;
use env_logger::Env;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let env = match cli.verbose {
        0 => Env::default().default_filter_or("warn,sled=error,serfig=error"),
        1 => Env::default().default_filter_or("info,sled=error,serfig=error"),
        _ => Env::default().default_filter_or("debug,sled=error,serfig=error"),
    };

    env_logger::Builder::from_env(env)
        .format_timestamp(None)
        .init();

    info!("Verbose level: {}", cli.verbose);

    // run::<Bitflip>()?;
    run::<Stuck<0xff>>(&cli)?;
    run::<Stuck<0x00>>(&cli)?;
    Ok(())
}
