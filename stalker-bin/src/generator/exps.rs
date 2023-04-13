use std::path::Path;

use anyhow::Result;
use log::info;
use stalker_classifier::analyze::AnalyzeConfig;

pub fn gen(path: &str) -> Result<()> {
    let root = Path::new(path);
    let mut res = AnalyzeConfig {
        path : path.to_string(),
        ..Default::default()
    };
    if root.is_dir() {
        for entry in std::fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let id = path
                    .file_name()
                    .and_then(std::ffi::OsStr::to_str)
                    .map(ToString::to_string)
                    .unwrap();
                res.experiments.insert(id.clone(), vec![]);
                for exp in std::fs::read_dir(&path)? {
                    let exp = exp?;
                    let exp_path = exp.path();
                    if exp_path.is_dir() {
                        res.experiments.get_mut(&id).unwrap().push(
                            exp_path
                                .file_name()
                                .and_then(std::ffi::OsStr::to_str)
                                .map(ToString::to_string)
                                .unwrap(),
                        );
                    }
                }
            }
        }
    }
    info!(
        "Sample analyze configuration for path `{}` generated. ",
        path
    );
    println!(
        "Sample configuration:\n\n```\n{}\n```",
        toml::to_string(&res)?
    );
    Ok(())
}
