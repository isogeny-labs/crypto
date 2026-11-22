//! Prime31 run53: exact Kummer 7-fiber decomposition of the existing signed
//! y^7=c factor base. Reuses the repo's prime-field Cantor-Zassenhaus root
//! finder; no new root-finding or EC arithmetic backend.
//!
//! This computes the PUBLIC factor base, not blind multi-leaf decomposition
//! and not an ECDLP solve. Benchmarking is paired within one process.
//!
//! cargo run --release --example prime31_coset_cubic_factor
use crypto_lib::cryptanalysis::pkm_smooth_tower::{
    mulm, poly_mul, poly_roots, powm, Rng,
};
use std::collections::BTreeSet;
use std::time::{Duration, Instant};

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
// Each pair is (public c = y_0^7, a public y_0 witness of this coset).
// All other y values and all x roots are derived, not input.
const COSETS: [(u64, u64); 8] = [
    (1_970_273_222, 391_601_180),
    (1_011_118_577, 1_045_445_284),
    (784_868_900, 1_088_714_879),
    (1_902_095_001, 839_007_678),
    (131_394_216, 1_893_426_316),
    (49_444_078, 1_215_727_561),
    (53_346_144, 1_554_347_565),
    (1_852_955_275, 136_769_428),
];

fn rhs(x: u64) -> u64 {
    (mulm(mulm(x, x, P), x, P) + mulm(A, x, P) + B) % P
}

fn full_coset_polynomial(c: u64) -> Vec<u64> {
    let mut out = vec![1];
    for _ in 0..7 {
        out = poly_mul(&out, &vec![B, A, 0, 1], P);
    }
    out[0] = (out[0] + P - mulm(c, c, P)) % P;
    assert_eq!(out.len(), 22);
    out
}

// Both paths invoke the SAME upstream Rust poly_roots backend.
fn direct(c: u64, seed: u64) -> BTreeSet<u64> {
    let coefficients = full_coset_polynomial(c);
    poly_roots(&coefficients, P, &mut Rng(seed))
        .into_iter()
        .collect()
}

fn structured(c: u64, y0: u64, seed: u64) -> BTreeSet<u64> {
    assert_eq!(powm(y0, 7, P), c);
    assert_eq!((P - 1) % 7, 0);
    let zeta = powm(3, (P - 1) / 7, P);
    assert_ne!(zeta, 1);
    assert_eq!(powm(zeta, 7, P), 1);

    let mut factored = vec![1];
    let mut roots = BTreeSet::new();
    let mut y = y0;
    for fiber in 0..7u64 {
        let y2 = mulm(y, y, P);
        let cubic = vec![(B + P - y2) % P, A, 0, 1];
        let fiber_roots = poly_roots(&cubic, P, &mut Rng(seed ^ fiber));
        assert_eq!(fiber_roots.len(), 3, "a registered split cubic failed");
        for x in fiber_roots {
            assert_eq!(rhs(x), y2);
            // The 7th-power condition fixes the sign, unlike an x-only S3.
            assert_eq!(mulm(powm(rhs(x), 4, P), powm(c, P - 2, P), P), y);
            assert!(roots.insert(x), "overlapping x support");
        }
        factored = poly_mul(&factored, &cubic, P);
        y = mulm(y, zeta, P);
    }
    assert_eq!(y, y0);
    assert_eq!(factored, full_coset_polynomial(c));
    assert_eq!(roots.len(), 21);
    roots
}

fn median(mut times: Vec<Duration>) -> Duration {
    times.sort_unstable();
    times[times.len() / 2]
}

fn main() {
    let mut direct_times = Vec::new();
    let mut structured_times = Vec::new();
    for repetition in 0..5u64 {
        let start = Instant::now();
        let mut direct_roots = Vec::new();
        for (i, &(c, _)) in COSETS.iter().enumerate() {
            let roots = direct(c, 0x53a1_0000 ^ (i as u64) ^ (repetition << 12));
            assert_eq!(roots.len(), 21);
            direct_roots.push(roots);
        }
        direct_times.push(start.elapsed());

        let start = Instant::now();
        let mut structured_roots = Vec::new();
        for (i, &(c, y0)) in COSETS.iter().enumerate() {
            structured_roots.push(structured(
                c,
                y0,
                0x7f31_0000 ^ (i as u64) ^ (repetition << 12),
            ));
        }
        structured_times.push(start.elapsed());
        assert_eq!(direct_roots, structured_roots, "paired root sets disagree");
        let n: usize = structured_roots.iter().map(BTreeSet::len).sum();
        assert_eq!(n, 168);
    }
    let direct = median(direct_times).as_secs_f64() * 1000.0;
    let fiber = median(structured_times).as_secs_f64() * 1000.0;
    println!("PRIME31_COSET_CUBIC_PASS: 8 cosets x 7 cubics x 3 public x roots = 168");
    println!("Rust paired rootfinding median five rounds: degree21_ms={direct:.3}, seven_cubics_ms={fiber:.3}, speed_ratio={:.3}", direct / fiber);
    println!("Scope: factor-base acquisition only; no public-Q decomposition, F4 blind solve, or DLP advantage.");
}
