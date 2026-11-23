//! Prime31 signed four-leaf representation-density prescreen.
//! Pure combinatorics; NO group-sum dictionary, target oracle, or ECDLP solve.
//! Reuses the original Prime31 Rust curve and published B677 factor-base fixture.
//! cargo test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m4_representation_density
//! cargo run  --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m4_representation_density

use prime31_algebraic_ic::{check_curve, Arithmetic, Base, PRIME31};
use std::collections::BTreeSet;

const Y7_EXTRA: &str =
    include_str!("../../../prime31_f4_admission_20261121/RUN66_Y7_EXTRA168.csv");
const N: u128 = 2_147_446_831;

fn choose(n: u128, k: u128) -> u128 {
    if k > n { return 0; }
    let k = k.min(n-k);
    let mut out = 1u128;
    for i in 1..=k { out = out * (n - k + i) / i; }
    out
}
fn distinct_four(b: u128) -> u128 { 16 * choose(b, 4) }
fn even_up_to_four(b: u128) -> u128 {
    1 + 4*b + 16*choose(b, 2) + 24*choose(b, 3) + distinct_four(b)
}
fn verify_real_base() -> Result<(), String> {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar)?;
    if (PRIME31.p, PRIME31.a, PRIME31.b, PRIME31.n) !=
        (2_147_483_647, 847_514_846, 2_087_444_724, 2_147_446_831) {
        return Err("Prime31 identity changed".into());
    }
    // Same public Dickson B509 and frozen 168-point extension as Run66.
    let canonical = |(x,y): (u64,u64)| (x, y.min((PRIME31.p-y) % PRIME31.p));
    let mut base = Base::build(PRIME31, 10, &mut ar)?.points
        .into_iter().map(canonical).collect::<Vec<_>>();
    if base.len() != 509 { return Err("Dickson count changed".into()); }
    let extra = Y7_EXTRA.lines().filter(|s| !s.is_empty())
        .map(|line| {
            let (x,y) = line.split_once(',').ok_or("missing factor-base comma")?;
            let x = x.parse::<u64>().map_err(|e| e.to_string())?;
            let y = y.parse::<u64>().map_err(|e| e.to_string())?;
            Ok::<_,String>(canonical((x,y)))
        }).collect::<Result<Vec<_>,_>>()?;
    if extra.len() != 168 { return Err("Y7 fixture count changed".into()); }
    base.extend(extra);
    base.sort_unstable();
    base.dedup();
    if base.len() != 677 { return Err("hybrid B677 duplicates".into()); }
    let mut xs = BTreeSet::new();
    for &point in &base {
        if !ar.on_curve(PRIME31, Some(point)) { return Err("off-curve factor-base point".into()); }
        if !xs.insert(point.0) { return Err("duplicate x: sign choices collapse".into()); }
        if point.1 == 0 { return Err("2-torsion point in signed base".into()); }
    }
    Ok(())
}
fn run() -> Result<(), String> {
    verify_real_base()?;
    println!("classification=EXACT_UNIFORM_TARGET_REPRESENTATION_EXPECTATION_NOT_MEASURED_YIELD");
    println!("curve_p={} subgroup_n={} signed_base=B677", PRIME31.p, PRIME31.n);
    println!("B,distinct_four_signed_coefficient_vectors,all_even_up_to_four_vectors,expected_distinct_four_per_uniform_Q,expected_even_upto_four_per_uniform_Q");
    for b in [4u128,8,16,31,64,96,128,160,192,224,256,288,320,384,509,512,677] {
        let distinct = distinct_four(b);
        let all = even_up_to_four(b);
        println!("{b},{distinct},{all},{:.10},{:.10}", distinct as f64/N as f64, all as f64/N as f64);
    }
    Ok(())
}
fn main() {
    if let Err(e) = run() { eprintln!("{e}"); std::process::exit(1); }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn exact_vectors_and_prime31_denominator() {
        assert_eq!(PRIME31.n as u128, N);
        assert_eq!(even_up_to_four(4), 225);
        assert_eq!(even_up_to_four(8), 2_945);
        assert_eq!(even_up_to_four(31), 618_885);
        assert_eq!(distinct_four(677), 138_805_851_600);
        assert_eq!(even_up_to_four(677), 140_045_175_925);
        assert_eq!(distinct_four(256), 2_796_682_240);
        assert_eq!(even_up_to_four(256), 2_863_529_985);
    }
    #[test] fn original_factor_base_identity() { verify_real_base().unwrap(); }
}
