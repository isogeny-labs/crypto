//! Secondary post-run arithmetic against the user's unchanged original count.
//! Does not replace or alter the separately frozen primary audit.
use std::env;
fn main() {
    let a: Vec<_> = env::args().skip(1).collect();
    assert_eq!(a.len(), 2);
    let baseline: u64 = a[0].parse().unwrap();
    let candidate: u64 = a[1].parse().unwrap();
    assert!(baseline > 0 && candidate <= baseline);
    let removed = (baseline - candidate) as f64 / baseline as f64;
    println!("{{\"schema\":\"fixed-reference-arithmetic/v1\",\"fixed_original_ir\":{baseline},\"candidate_ir\":{candidate},\"removed_fraction\":{removed},\"ceiling_ir\":{},\"meets_fixed_ceiling\":{},\"standing\":\"secondary post-run Rust arithmetic; not primary analysis or signed admission\"}}", baseline / 2, candidate <= baseline / 2);
}
