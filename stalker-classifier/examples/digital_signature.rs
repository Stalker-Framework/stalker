use stalker_classifier::analyze::*;
use stalker_classifier::effect::*;
use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    type T = ProbabilisticSignatureEffect;

    let args: Vec<String> = env::args().collect();
    let show_patches = args.iter().any(|s| s == "--patches" || s == "-p");

    let arch = args.get(1).expect("No arch specified.");

    let config = load_config("examples/config/dsa.yaml");
    analyze::<T>(&config, arch, show_patches, "results");

    Ok(())
}
