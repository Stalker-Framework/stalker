
use crate::{executor, generator};
use clap::{Args, Parser, Subcommand};
use env_logger::Env;
use log::info;
use serde::{Deserialize, Serialize};
use serfig::collectors::{from_file, from_self};
use serfig::parsers::Toml;
use serfig::Builder;
use stalker_classifier::analyze::AnalyzeConfig;
use stalker_injector::InjectionConfig;
use stalker_mutator::{FaultModel};
use stalker_utils::config::{Config, LibConfig};
use stalker_utils::context::Context;

/// Analyzing fragility of dynamic libs under hardware fault models.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Number of times to greet
    #[arg(short, long, value_name = "FILE", default_value_t = String::from("config/config.toml"))]
    config: String,

    #[arg(short, long, value_name = "FILE", default_value_t = String::from("config/lib.toml"))]
    lib_config: String,

    /// Turn logging on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    GenLocs,
    GenMuts,
    GenExps(GenExpsArgs),
    Inject(InjectArgs),
    Analyze(AnalyzeArgs),
}

#[derive(Args)]
struct InjectArgs {
    #[arg(short, long, value_name = "FILE", default_value_t = String::from("config/inject.toml"))]
    inj_config: String,

    /// Turn parallel on
    #[arg(short, default_value_t = false)]
    parallel: bool,
}

#[derive(Args)]
struct AnalyzeArgs {
    #[arg(short, long, value_name = "FILE", default_value_t = String::from("config/experiments/dsa.yaml"))]
    config: String,
}

#[derive(Args)]
struct GenExpsArgs {
    #[arg(short, long, value_name = "PATH", default_value_t = String::from("data/stalker/output"))]
    path: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct CliConfig {
    platform: Config,
    lib: LibConfig,
    injection: InjectionConfig,
}

pub fn run<M: FaultModel>() -> anyhow::Result<()> {
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

    // To analyze or generate the experiments;
    match cli.command {
        Some(Commands::Analyze(analyze_args)) => {
            let analyze_config = Builder::default()
                .collect(from_file(Toml, &analyze_args.config))
                .collect(from_self(AnalyzeConfig::default()))
                .build()?;
            return executor::analyze::exec(&analyze_config);
        }
        Some(Commands::GenExps(genexps_args)) => return generator::exps::gen(&genexps_args.path),
        _ => {}
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

    // type Model = Stuck<0xff>;

    match &cli.command {
        Some(Commands::GenLocs) => {
            generator::liblocs::gen(&mut ctx)?;
        }
        Some(Commands::GenMuts) => {
            generator::mutants::gen::<M>(&mut ctx, &lib_config)?;
        }
        Some(Commands::Inject(inj_args)) => {
            let inj_builder = Builder::default()
                .collect(from_file(Toml, &inj_args.inj_config))
                .collect(from_self(InjectionConfig::default()));
            let inj_config: InjectionConfig = inj_builder.build()?;
            inj_config.init::<M>(&ctx, &lib_config)?;
            executor::inject::exec::<M>(&mut ctx, &lib_config, &inj_config, inj_args.parallel)?;
        }
        _ => {}
    }

    Ok(())
}