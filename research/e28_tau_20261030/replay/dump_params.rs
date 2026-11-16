//! E28 validation (independent validator, 2026-10-31): print the coefficients
//! of E28's 40 Stage 0 curves (which include the 10 Stage A curves) for
//! `curve_check.rs`.  It only calls the producer's instance generator, as
//! Stage 0 and Stage A do; every check is made by `curve_check.rs` with its own
//! arithmetic.  Built as an extra example in the export of 789dc2ff6, after the
//! replay binary had been built and hashed.
//!
//! Output, one line per curve:
//! `p seed n h lambda a0..a3 b0..b3 gx0..gx3 gy0..gy3 cx0..cx3 cy0..cy3`.

use crypto_lib::cryptanalysis::quartic_folds::generate_gls_tau_instance;

fn main() {
    for p in [53u64, 101, 149, 197, 269] {
        for seed in 1..=8u64 {
            let inst = generate_gls_tau_instance(p, seed, 16).expect("E28's instance");
            let (cx, cy) = inst.gls.expect("a GLS instance");
            let mut v: Vec<u64> = vec![p, seed, inst.n, inst.cofactor, inst.lambda];
            for e in [inst.a, inst.b, inst.g.x, inst.g.y, cx, cy] {
                v.extend_from_slice(&e.0);
            }
            println!(
                "{}",
                v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")
            );
        }
    }
}
