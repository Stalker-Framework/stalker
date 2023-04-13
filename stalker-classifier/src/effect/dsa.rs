use crate::effect::Effect;
use crate::metric::entropy_of_cnts;
use crate::parse::Fields;
use derive_more::Display;
use hex::decode as from_hex;
use std::collections::HashSet;
use strum_macros::{AsRefStr, EnumProperty, EnumVariantNames};
use ProbabilisticSignatureEffect::*;

#[derive(Display, AsRefStr, EnumVariantNames, EnumProperty)]
pub enum ProbabilisticSignatureEffect {
    #[strum(props(Show = ""))]
    NonceReuse(f64),
    VerificationFail,
    Functional,
    #[strum(props(Show = ""))]
    OddBytes(f64),
    #[strum(props(Show = ""))]
    PartiallyFunctional(f64),
    Unknown,
}

impl Effect for ProbabilisticSignatureEffect {
    const HAS_EXPECT: bool = false;
    fn check(expect: &Option<Vec<Fields>>, res: &[Fields]) -> ProbabilisticSignatureEffect {
        // `ProbabilisticSignatureEffect` requires no expected results.
        assert!(expect.is_none());

        let cnt = res.len();
        let mut nonce_reuse = 0;
        let mut functional = 0;

        let mut nonces: HashSet<String> = HashSet::new();
        let mut byte_cnt = [[0usize; 256]; 48];

        for item in res.iter() {
            if item.is_empty() {
                continue;
            }
            // Insert `r`, which is equivalent to the nonce `k`.
            if nonces.contains(&item[0]) {
                nonce_reuse += 1;
                continue;
            } else {
                nonces.insert(item[0].clone());
            }

            let nt = from_hex(&item[0]);
            let st = from_hex(&item[1]);

            if nt.is_err() || st.is_err() {
                continue;
            }

            let nt = nt.unwrap();
            let st = st.unwrap();

            // log the byte conuts; we consider both the signature and nouce
            for i in 0..24 {
                byte_cnt[i * 2][*nt.get(i).unwrap_or(&0) as usize] += 1;
                byte_cnt[i * 2 + 1][*st.get(i).unwrap_or(&0) as usize] += 1;
            }

            if item[2].contains('1') {
                // encryption result is the same, but decryption fails
                functional += 1;
                continue;
            }
        }
        // Precomputate
        let freq_max = byte_cnt
            .iter()
            .map(|cnts| entropy_of_cnts(cnts, cnt))
            .reduce(f64::min)
            .unwrap();
        if nonce_reuse != 0 {
            NonceReuse(nonce_reuse as f64 / cnt as f64)
        } else if freq_max <= 0.2 {
            OddBytes(freq_max / cnt as f64)
        } else if functional == cnt {
            Functional
        } else if functional != 0 {
            PartiallyFunctional(functional as f64 / cnt as f64)
        } else if functional == 0 {
            VerificationFail
        } else {
            Unknown
        }
    }
}
