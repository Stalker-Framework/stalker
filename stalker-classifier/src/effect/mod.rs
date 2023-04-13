use crate::{
    analyze::{analyze, AnalyzeConfig},
    parse::{Fields, ParseError},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::AsRef;
use std::fmt::Display;
use std::fs::read_dir;
use std::path::Path;
use strum::{EnumProperty, VariantNames};

mod dc;
mod dsa;
pub use dc::*;
pub use dsa::*;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum AnalyzeModel {
    DeterministicCipher,
    ProbabilisticSignature,
    Nothing,
}

impl Default for AnalyzeModel {
    fn default() -> Self {
        Self::Nothing
    }
}

fn nop(_: &AnalyzeConfig) -> Result<()> {
    Ok(())
}

impl AnalyzeModel {
    pub fn analyze_fn(&self) -> impl Fn(&AnalyzeConfig) -> Result<()> {
        match self {
            Self::DeterministicCipher => analyze::<DeterministicCipherEffect>,
            Self::ProbabilisticSignature => analyze::<ProbabilisticSignatureEffect>,
            Self::Nothing => nop,
        }
    }
}

pub trait Effect: Sized {
    const HAS_EXPECT: bool;
    fn check(expect: &Option<Vec<Fields>>, res: &[Fields]) -> Self;
}

pub fn stats<T: Effect + VariantNames + AsRef<str>>(
    effects: &[(String, Result<T, ParseError>)],
) -> String {
    let t_enums = T::VARIANTS;
    let p_enums = ParseError::VARIANTS;
    let mut stat_ef = vec![0usize; t_enums.len()];
    let mut stat_er = vec![0usize; p_enums.len()];

    let cnt = effects.len();

    for (_, e) in effects.iter() {
        match e {
            Ok(ef) => {
                if let Some(n) = t_enums.iter().position(|s| s == &ef.as_ref()) {
                    stat_ef[n] += 1;
                }
            }
            Err(er) => {
                if let Some(n) = p_enums.iter().position(|s| s == &er.as_ref()) {
                    stat_er[n] += 1;
                }
            }
        }
    }

    let mut res = String::from("Class               , Count, All\n");
    for (i, k) in t_enums.iter().enumerate() {
        let v = stat_ef[i];
        if v > 0 {
            res += &format!("{:20}, {:5}, {:4}\n", k, v, cnt);
        }
    }
    for (i, k) in p_enums.iter().enumerate() {
        let v = stat_er[i];
        if v > 0 {
            res += &format!("{:20}, {:5}, {:4}\n", k, v, cnt);
        }
    }
    res
}

pub fn inspect<T: Display + Effect + VariantNames + AsRef<str> + EnumProperty>(
    effects: &[(String, Result<T, ParseError>)],
) -> String {
    let t_enums = T::VARIANTS;
    let mut patches: HashMap<String, Vec<String>> = HashMap::new();

    for &_enum in t_enums.iter() {
        patches.insert(String::from(_enum), vec![]);
    }

    effects.iter().for_each(|(p, e)| {
        if let Ok(ef) = e {
            if ef.get_str("Show").is_some() {
                let ps = patches.get_mut(ef.as_ref()).unwrap();
                ps.push(format!("{:48}:  {}", p.split('/').last().unwrap(), ef));
            }
        }
    });

    let mut res = String::default();
    for (k, ps) in patches.iter() {
        if ps.is_empty() {
            continue;
        } else {
            res += &format!("{}:\n", k);
        }
        for p in ps.iter() {
            res += &format!("{}\n", p)
        }
    }
    res
}

pub fn process_dir<T: Effect>(
    path: &Path,
    group_size: usize,
) -> Vec<(String, Result<T, ParseError>)> {
    let mut res_files = read_dir(path).unwrap();
    let mut res_vec: Vec<(String, Result<T, ParseError>)> = vec![];
    let expect: Option<Vec<Fields>> = if T::HAS_EXPECT {
        Fields::from_file(path.join("expected.txt"), group_size).ok()
    } else {
        None
    };

    while let Some(Ok(path)) = res_files.next() {
        let p_s = path.path().clone();
        let res = Fields::from_file(path.path(), group_size);
        let p = p_s.to_str().unwrap().to_string();
        match res {
            Ok(e) => res_vec.push((p, Ok(Effect::check(&expect, &e)))),
            Err(e) => res_vec.push((p, Err(e))),
        }
    }
    res_vec
}
