//! Paired correctness check for the prime-field S₄ oracles: on one frozen
//! 16-bit `a = −3` fixture and three deterministic targets, the S₃-anchored
//! sweep, the pair-sum meet-in-the-middle and the quartic/CZ oracle must all
//! recover the same scalar.
//!
//! ```bash
//! cargo run --release --example prime_mitm_check
//! ```
//!
//! Each driver re-adds every relation in the group before using it
//! (`try_finalise_s4`), so a passing run also checks that the MITM probe
//! never returns a triple that does not sum to the sampled `R`.
//! Public synthetic known-answer only.

use crypto_lib::cryptanalysis::ec_index_calculus::{
    ec_index_calculus_dlp_s4_mitm_staged, ec_index_calculus_dlp_s4_s3_staged,
    ec_index_calculus_dlp_s4_staged,
};
use crypto_lib::cryptanalysis::research_bench::bench_curves_a_minus_3;
use crypto_lib::ecc::point::Point;
use num_bigint::BigUint;
use num_traits::Zero;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn target_for(curve: &crypto_lib::ecc::curve::CurveParams, seed: u64) -> BigUint {
    let mut rng = StdRng::seed_from_u64(seed);
    let limbs = (curve.n.bits() / 64 + 1) as usize;
    let mut x = BigUint::zero();
    for i in 0..limbs {
        let word: u64 = rng.gen();
        x |= BigUint::from(word) << (64 * i);
    }
    (x % (&curve.n - 1u32)) + 1u32
}

fn main() {
    let (_, curve) = bench_curves_a_minus_3()
        .into_iter()
        .find(|(bits, _)| *bits == 16)
        .expect("16-bit a=-3 fixture");
    let a_fe = curve.a_fe();
    let g = curve.generator();

    let mut all_ok = true;
    for seed in [20261005u64, 20261006, 20261007] {
        let x_truth = target_for(&curve, seed);
        let q = g.scalar_mul(&x_truth, &a_fe);
        let (qx, qy) = match &q {
            Point::Affine { x, y } => (x.value.to_string(), y.value.to_string()),
            _ => {
                eprintln!("target is the identity");
                std::process::exit(2);
            }
        };
        let s3 = ec_index_calculus_dlp_s4_s3_staged(&curve, &g, &q, 120, 8, 1_000_000, 64);
        let mitm = ec_index_calculus_dlp_s4_mitm_staged(&curve, &g, &q, 120, 8, 1_000_000, 64);
        let quartic = ec_index_calculus_dlp_s4_staged(&curve, &g, &q, 120, 8, 1_000_000, 64);

        let ok = |r: &Option<(BigUint, _)>| {
            r.as_ref()
                .map(|(x, _)| x == &x_truth && g.scalar_mul(x, &a_fe) == q)
                .unwrap_or(false)
        };
        let (ok_s3, ok_mitm, ok_quartic) = (ok(&s3), ok(&mitm), ok(&quartic));
        all_ok &= ok_s3 && ok_mitm && ok_quartic;
        println!(
            "{}",
            serde_json::json!({
                "bits": 16,
                "curve": curve.name,
                "seed": seed,
                "public_target_q": [qx, qy],
                "s3_ok": ok_s3,
                "mitm_ok": ok_mitm,
                "quartic_ok": ok_quartic,
                "same_scalar": s3.as_ref().map(|r| &r.0) == mitm.as_ref().map(|r| &r.0)
                    && s3.as_ref().map(|r| &r.0) == quartic.as_ref().map(|r| &r.0),
                "mitm_pairs": mitm.as_ref().map(|r| r.1.relations_collected),
                "claim_boundary": "synthetic known-answer oracle equivalence; not a speedup claim",
            })
        );
    }
    if !all_ok {
        std::process::exit(1);
    }
}
