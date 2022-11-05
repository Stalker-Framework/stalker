mod executor;
mod generator;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use env_logger::Env;
use log::info;
use serfig::collectors::{from_file, from_self};
use serfig::parsers::Toml;
use serfig::Builder;
use stalker_injector::InjectionConfig;
use stalker_utils::config::{Config, LibConfig};
use stalker_utils::context::Context;

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Number of times to greet
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE")]
    lib_config: Option<PathBuf>,

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
    Inject,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let lib_builder = Builder::default()
        .collect(from_self(LibConfig::default()))
        .collect(from_file(Toml, "config/lib.toml"));
    let inj_builder = Builder::default()
        .collect(from_self(InjectionConfig::default()))
        .collect(from_file(Toml, "config/inject.toml"));
    let builder = Builder::default()
        .collect(from_self(Config::default()))
        .collect(from_file(Toml, "config/platform.toml"));

    let env = match cli.verbose {
        0 => Env::default().default_filter_or("warn,sled=error,serfig=error"),
        1 => Env::default().default_filter_or("info,sled=error,serfig=error"),
        _ => Env::default().default_filter_or("debug,sled=error,serfig=error"),
    };

    env_logger::Builder::from_env(env)
        .format_timestamp(None)
        .init();

    info!("Verbose level: {}", cli.verbose);

    let config: Config = builder.build()?;
    let lib_config: LibConfig = lib_builder.build()?;
    let inj_config: InjectionConfig = inj_builder.build()?;
    info!("Target lib: {}", &lib_config.path);

    // Initialization
    let mut ctx = Context::new(&lib_config.path, Some(config))?;
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
        Some(Commands::Inject) => {
            inj_config.init(&ctx, &lib_config)?;
            executor::inject::exec(&mut ctx, &lib_config, &inj_config)?;
        }
        None => {}
    }

    Ok(())
}
