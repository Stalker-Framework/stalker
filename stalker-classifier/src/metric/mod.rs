mod equal_by_byte;
pub use equal_by_byte::*;

pub trait Metric {
    type Output;
    fn dist(a: &[u8], b: &[u8]) -> Self::Output;
}

pub fn entropy_of_cnts(cnts: &[usize; 256], cnt: usize) -> f64 {
    let mut h = 0.0;
    let cnt = cnt as f64;
    for &c in cnts.iter() {
        if c == 0 {
            continue;
        } else {
            let p = c as f64 / cnt;
            h -= p * p.log2()
        }
    }
    h
}
