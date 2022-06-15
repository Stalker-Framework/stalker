use crate::effect::*;
use serde::Deserialize;
use serde_yaml;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::fs::{read_dir, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use strum::{EnumProperty, VariantNames};

#[derive(Debug, PartialEq, Deserialize)]
pub struct AnalyzeConfig {
    path: String,
    fields: usize,
    experiments: BTreeMap<String, Vec<String>>,
    has_expected: bool,
}

pub fn load_config<P: AsRef<Path>>(p: P) -> AnalyzeConfig {
    let f = std::fs::File::open(p).expect("No config file.");
    serde_yaml::from_reader(f).expect("Not valid config.")
}

pub fn analyze<T: Display + Effect + VariantNames + AsRef<str> + EnumProperty>(
    config: &AnalyzeConfig,
    arch: &str,
    show_patches: bool,
    output: &str,
) {
    let root = Path::new(&config.path);
    let exps_dir = root.join(arch);
    let exps = config.experiments[arch].iter();
    let expect = if config.has_expected {
        Some("expected.txt")
    } else {
        None
    };
    for exp in exps {
        let exp_dir = exps_dir.join(exp);
        for entry in read_dir(exp_dir.clone()).expect("Example not found") {
            let symbol_dir = entry.expect("").path();
            if symbol_dir.is_dir() {
                let symbol = symbol_dir.file_name().unwrap().to_str().unwrap();
                println!("Processing {}...", symbol);
                let es = process_dir::<T>(&exp_dir, expect, symbol, config.fields);
                let res;
                let output_file;
                if show_patches {
                    res = inspect::<T>(&es);
                    output_file = format!("{}/{}/{}-detail.txt", output, arch, symbol);
                } else {
                    res = stats::<T>(&es);
                    output_file = format!("{}/{}/{}.csv", output, arch, symbol);
                }
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(output_file)
                    .expect("Result file created failed.");
                file.write_all(res.as_bytes()).unwrap();
            }
        }
    }
}
