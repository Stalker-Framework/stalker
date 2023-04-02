use crate::metric::Metric;
use std::ops::Add;

pub struct EqualByByte;

/// ```
/// use crate::stalker_classifier::metric::{Metric, EqualByByte};
///
/// let a = [0u8; 8];
/// let b = [1u8; 8];
/// assert_eq!(0.0, EqualByByte::dist(&a, &b));
///
/// let a = [0u8; 4];
/// let b = [0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 0];
/// assert_eq!(0.5, EqualByByte::dist(&a, &b));
/// ```
impl Metric for EqualByByte {
    type Output = f64;
    fn dist(secret: &[u8], output: &[u8]) -> f64 {
        // a is the secret to EqualByByte
        // b is the exposed output
        let secret_len = secret.len();
        if let Some(r) = output
            .chunks(secret_len)
            .map(|lst| {
                lst.iter()
                    .zip(secret.iter())
                    .map(|(a, b)| if a == b { 1.0 } else { 0.0 })
                    .reduce(f64::add)
                    .unwrap()
                    / secret_len as f64
            })
            .reduce(f64::max)
        {
            r
        } else {
            0.0
        }
    }
}
