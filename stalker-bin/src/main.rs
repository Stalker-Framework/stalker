mod executor;
mod generator;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use env_logger::Env;
use log::info;
use serfig::collectors::from_self;
use serfig::Builder;
use stalker_utils::config::LibConfig;
use stalker_utils::context::Context;

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Number of times to greet
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    /// Turn logging on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    GenLocs,
    GenMuts,
    Inject { func_name: String },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let builder = Builder::default().collect(from_self(LibConfig::default()));

    let env = match cli.verbose {
        0 => Env::default().default_filter_or("warn,sled=error,serfig=error"),
        1 => Env::default().default_filter_or("info,sled=error,serfig=error"),
        _ => Env::default().default_filter_or("debug,sled=error,serfig=error"),
    };

    env_logger::Builder::from_env(env)
        .format_timestamp(None)
        .init();

    info!("Verbose level: {}", cli.verbose);

    let lib_config: LibConfig = builder.build()?;
    info!("Target lib: {}", &lib_config.path);

    // Initialization
    let mut ctx = Context::new(&lib_config.path)?;
    ctx.lib.init_locs(&mut ctx.rz)?;
    ctx.init_db()?;

    match &cli.command {
        Some(Commands::Init) => (),
        Some(Commands::GenLocs) => {
            generator::liblocs::gen(&mut ctx)?;
        }
        Some(Commands::GenMuts) => {
            generator::mutants::gen(&mut ctx, &lib_config)?;
        }
        Some(Commands::Inject { func_name }) => {
            executor::inject::exec(&mut ctx, &func_name)?;
        }
        None => {}
    }

    Ok(())
}
