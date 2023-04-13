mod executor;
mod generator;

use clap::{Parser, Subcommand};
use env_logger::Env;
use log::info;
use serde::{Deserialize, Serialize};
use serfig::collectors::{from_file, from_self};
use serfig::parsers::Toml;
use serfig::Builder;
use stalker_classifier::analyze::AnalyzeConfig;
use stalker_injector::InjectionConfig;
use stalker_mutator::model::Bitflip;
use stalker_utils::config::{Config, LibConfig};
use stalker_utils::context::Context;

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Number of times to greet
    #[arg(short, long, value_name = "FILE", default_value_t = String::from("config/config.toml"))]
    config: String,

    #[arg(short, long, value_name = "FILE", default_value_t = String::from("config/lib.toml"))]
    lib_config: String,

    #[arg(short, long, value_name = "FILE", default_value_t = String::from("config/inject.toml"))]
    inj_config: String,

    #[arg(short, long, value_name = "FILE", default_value_t = String::from("config/experiments/dsa.yaml"))]
    analyze_config: String,

    /// Turn logging on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Turn logging on
    #[arg(short, default_value_t = true)]
    parallel: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, PartialEq)]
enum Commands {
    Init,
    GenLocs,
    GenMuts,
    Inject,
    Analyze,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct CliConfig {
    platform: Config,
    lib: LibConfig,
    injection: InjectionConfig,
}

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

    // To analyze;
    if cli.command == Some(Commands::Analyze) {
        let analyze_config = Builder::default()
            .collect(from_file(Toml, &cli.analyze_config))
            .collect(from_self(AnalyzeConfig::default()))
            .build()?;
        return executor::analyze::exec(&analyze_config);
    }

    // Build lib config.
    let lib_builder = Builder::default()
        .collect(from_file(Toml, &cli.lib_config))
        .collect(from_self(LibConfig::default()));
    let builder = Builder::default()
        .collect(from_file(Toml, &cli.config))
        .collect(from_self(Config::default()));

    let config: Config = builder.build()?;
    let lib_config: LibConfig = lib_builder.build()?;
    info!("Target lib: {}", &lib_config.path);

    // Initialization
    let mut ctx = Context::builder(&lib_config.path).config(config).build()?;
    ctx.lib.init_locs(&mut ctx.rz)?;
    ctx.init_db()?;

    type Model = Bitflip;

    match &cli.command {
        Some(Commands::Init) => (),
        Some(Commands::GenLocs) => {
            generator::liblocs::gen(&mut ctx)?;
        }
        Some(Commands::GenMuts) => {
            generator::mutants::gen::<Model>(&mut ctx, &lib_config)?;
        }
        Some(Commands::Inject) => {
            let inj_builder = Builder::default()
                .collect(from_file(Toml, &cli.inj_config))
                .collect(from_self(InjectionConfig::default()));
            let inj_config: InjectionConfig = inj_builder.build()?;
            inj_config.init(&ctx, &lib_config)?;
            executor::inject::exec(&mut ctx, &lib_config, &inj_config, cli.parallel)?;
        }
        _ => {}
    }

    Ok(())
}
