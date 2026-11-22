//! Exact replay of nagao-e2e/native/analyze.rs `band()` (seed 0xE2E_1009,
//! 10000 draws, order statistics 249 and 9749) on the paired ratios given on
//! the command line. Validator reproduction tool, [pcc:unverified].
use rand::{rngs::StdRng, Rng, SeedableRng};
fn geometric(v: &[f64]) -> f64 {
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}
fn main() {
    let v: Vec<f64> = std::env::args().skip(1).map(|a| a.parse().unwrap()).collect();
    assert!(!v.is_empty(), "usage: nagao_bootstrap_replay RATIO...");
    let mut rng = StdRng::seed_from_u64(0xE2E_1009);
    let mut b = Vec::with_capacity(10000);
    for _ in 0..10000 {
        let sample: Vec<_> = (0..v.len()).map(|_| v[rng.gen_range(0..v.len())]).collect();
        b.push(geometric(&sample));
    }
    b.sort_by(f64::total_cmp);
    println!("{:?} {:?}", b[249], b[9749]);
}
