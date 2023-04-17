use crate::effect::*;
use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use strum::{EnumProperty, VariantNames};

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AnalyzeConfig {
    pub path: String,
    pub output: AnalyzeOutput,
    pub fields: usize,
    pub experiments: BTreeMap<String, Vec<String>>,
    pub model: AnalyzeModel,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AnalyzeOutput {
    pub path: String,
    pub mode: AnalyzeOutputMode,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum AnalyzeOutputMode {
    Detail,
    Summary,
    All,
}

impl Default for AnalyzeOutputMode {
    fn default() -> Self {
        Self::All
    }
}

pub fn analyze<T: Display + Effect + VariantNames + AsRef<str> + EnumProperty>(
    config: &AnalyzeConfig,
) -> Result<()> {
    let root = Path::new(&config.path);

    for exp_id in config.experiments.keys() {
        let exps_dir = root.join(exp_id);
        let exps = config.experiments[exp_id].iter();
        for exp in exps {
            let exp_dir = exps_dir.join(exp);
            if exp_dir.is_dir() {
                info!("Processing exp: {} of {}", exp, exp_id);
                let es = process_dir::<T>(&exp_dir, config.fields);
                let mut work_vec: Vec<(String, String)> = vec![];
                match config.output.mode {
                    AnalyzeOutputMode::Detail => {
                        let res = inspect::<T>(&es);
                        let output_file =
                            format!("{}/{}/{}-detail.txt", config.output.path, exp_id, exp);
                        work_vec.push((res, output_file));
                    }
                    AnalyzeOutputMode::Summary => {
                        let res = stats::<T>(&es);
                        let output_file = format!("{}/{}/{}.csv", config.output.path, exp_id, exp);
                        work_vec.push((res, output_file));
                    }
                    AnalyzeOutputMode::All => {
                        let res = inspect::<T>(&es);
                        let output_file =
                            format!("{}/{}/{}-detail.txt", config.output.path, exp_id, exp);
                        work_vec.push((res, output_file));
                        let res = stats::<T>(&es);
                        let output_file = format!("{}/{}/{}.csv", config.output.path, exp_id, exp);
                        work_vec.push((res, output_file));
                    }
                }
                for (res, output_file) in work_vec.iter() {
                    let mut file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(output_file)
                        .expect("Result file created failed.");
                    file.write_all(res.as_bytes())?;
                }
            }
        }
    }
    Ok(())
}
