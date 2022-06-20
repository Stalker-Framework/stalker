use crate::contants::KEY_CANDIDATES;
use crate::effect::Effect;
use crate::metric::{entropy_of_cnts, EqualByByte, Metric};
use crate::parse::Fields;
use derive_more::Display;
use hex::decode as from_hex;
use strum_macros::{AsRefStr, EnumProperty, EnumVariantNames};
use DeterministicCipherEffect::*;

#[derive(Display, AsRefStr, EnumVariantNames, EnumProperty)]
pub enum DeterministicCipherEffect {
    // No information leakage
    Correct,
    // Maybe
    Functional,
    #[strum(props(Show = ""))]
    OddPlaintext(f64),
    #[strum(props(Show = ""))]
    DupeCiphertext(f64),
    #[strum(props(Show = ""))]
    DupeDecrypted(f64),
    #[strum(props(Show = ""))]
    PartiallyCorrect(f64),
    #[strum(props(Show = ""))]
    PartiallyFunctional(f64),
    #[strum(props(Show = ""))]
    OddBytes(f64),
    DecryptFail,
    // Must
    #[strum(props(Show = ""))]
    EqualByByteKey(f64),
    #[strum(props(Show = ""))]
    EqualByBytePlaintext(f64),
    // Unknown
    Unknown,
}

impl Effect for DeterministicCipherEffect {
    fn check(expect: &Option<Vec<Fields>>, res: &[Fields]) -> DeterministicCipherEffect {
        // `DeterministicCipherEffect` requires the expected result non-empty.
        assert!(expect.is_some());

        let expect = expect.clone().unwrap();
        let cnt = res.len().min(expect.len());

        let mut correct = 0;
        let mut odd_plaintext = 0;
        let mut decrypt_fail = 0;
        let mut functional = 0;

        let mut reveal_key = 0.0f64;
        let mut reveal_plaintext = 0.0f64;

        let mut dupe_etext = 0; // Duplicate encrypted plaintext
        let mut dupe_dtext = 0; // Duplicate decrypted ciphertext

        let mut cache = vec![vec![]; 3]; // cache the last <Fields> element.

        let mut byte_cnt = [[0usize; 256]; 16];
        let mut byte_cnt_flag = false;

        for i in 0..cnt {
            // exact the same, no need to check other properties
            if res[i] == expect[i] {
                correct += 1;
                continue;
            }

            if res[i].is_empty() {
                continue;
            }

            // the input plaintext is manipulated
            if res[i][0] != expect[i][0] {
                odd_plaintext += 1;
                continue;
            }

            // Get the raw data from the hex strings
            let pt = from_hex(&res[i][0]).unwrap();
            let ct = from_hex(&res[i][1]).unwrap();
            let rt = from_hex(&res[i][2]).unwrap();

            // log the byte conuts; we only consider first 16 bytes
            byte_cnt_flag = true;
            for i in 0..16 {
                byte_cnt[i][ct[i] as usize] += 1;
            }

            // check for odd etext and dtext
            if ct == cache[1] {
                dupe_etext += 1;
            }
            if rt == cache[2] {
                dupe_dtext += 1;
            }

            // are the outputs reveal something secret?
            reveal_key += KEY_CANDIDATES
                .iter()
                .map(|key| EqualByByte::dist(key, &ct))
                .reduce(f64::max)
                .unwrap();
            reveal_plaintext += EqualByByte::dist(&pt[0..16], &ct);

            cache = vec![pt, ct, rt];

            if res[i][1] == expect[i][1] && res[i][2] != expect[i][2] {
                // encryption result is the same, but decryption fails
                decrypt_fail += 1;
                continue;
            }
            if res[i][1] != expect[i][1] && res[i][0] == res[i][2] {
                // encryption result is different, but decryption works
                functional += 1;
                continue;
            }
        }
        // Precompute
        reveal_plaintext /= cnt as f64;
        reveal_key /= cnt as f64;
        let min_entropy = if byte_cnt_flag {
            byte_cnt
                .iter()
                .map(|cnts| entropy_of_cnts(cnts, cnt))
                .reduce(f64::min)
                .unwrap()
        } else {
            0.0
        };
        // Somthing interesting
        if dupe_etext > 0 {
            DupeCiphertext(dupe_etext as f64 / cnt as f64)
        } else if dupe_dtext > 0 {
            DupeDecrypted(dupe_dtext as f64 / cnt as f64)
        } else if reveal_key >= 0.125 {
            EqualByByteKey(reveal_key)
        } else if reveal_plaintext >= 0.125 {
            EqualByBytePlaintext(reveal_plaintext)
        } else if byte_cnt_flag && min_entropy <= 0.2 {
            OddBytes(min_entropy)
        } else if odd_plaintext != 0 {
            OddPlaintext(odd_plaintext as f64 / cnt as f64)
        }
        // Almost normal
        else if correct == cnt {
            Correct
        } else if correct != 0 {
            PartiallyCorrect(correct as f64 / cnt as f64)
        } else if functional == cnt {
            Functional
        } else if functional != 0 {
            PartiallyFunctional(functional as f64 / cnt as f64)
        } else if decrypt_fail != 0 {
            DecryptFail
        } else {
            Unknown
        }
    }
}
