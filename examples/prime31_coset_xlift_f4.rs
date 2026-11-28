//! Prime31 run52: public-only y-coset roots via the existing native Rust F4 solver.
//! This is a factor-base primitive, NOT an eight-leaf decomposition or an ECDLP solve.
//! cargo run --release --example prime31_coset_xlift_f4
use crypto_lib::cryptanalysis::f4_fp::{solve, F4Options, Ordering, Poly, Verdict};
use std::collections::BTreeSet;
use std::time::Duration;

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
// Each tuple is (public y^7 coset constant, one x anchor, its signed y).
// All other x roots must be discovered from the polynomial itself.
const ANCHORS: [(u64, u64, u64); 8] = [
    (1970273222, 944025173, 391601180),
    (1011118577, 896554146, 1045445284),
    (784868900, 1353807781, 1088714879),
    (1902095001, 2091142572, 839007678),
    (131394216, 1095587065, 1893426316),
    (49444078, 1332271677, 1215727561),
    (53346144, 120830567, 1554347565),
    (1852957275, 227959902, 136769428),
];

fn add(a: u64, b: u64) -> u64 { (a + b) % P }
fn mul(a: u64, b: u64) -> u64 { ((a as u128 * b as u128) % P as u128) as u64 }
fn pow(mut x: u64, mut n: u64) -> u64 {
    let mut v = 1;
    while n != 0 {
        if n & 1 != 0 { v = mul(v, x); }
        x = mul(x, x);
        n >>= 1;
    }
    v
}
fn rhs(x: u64) -> u64 { add(add(mul(mul(x, x), x), mul(A, x)), B) }

// Exact and unique signed lift: if f(x)^7=c^2, then y=f(x)^4/c.
fn lift_x(x: u64, c: u64) -> Option<u64> {
    if x >= P || c == 0 || c >= P || pow(rhs(x), 7) != mul(c, c) {
        return None;
    }
    let y = mul(pow(rhs(x), 4), pow(c, P - 2));
    (mul(y, y) == rhs(x) && pow(y, 7) == c).then_some(y)
}

fn poly_mul(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut c = vec![0; a.len() + b.len() - 1];
    for (i, &ai) in a.iter().enumerate() {
        for (j, &bj) in b.iter().enumerate() {
            c[i + j] = add(c[i + j], mul(ai, bj));
        }
    }
    c
}
fn coset_poly(c: u64) -> Vec<u64> {
    assert!(c > 0 && c < P);
    let mut f7 = vec![1];
    for _ in 0..7 { f7 = poly_mul(&f7, &[B, A, 0, 1]); }
    f7[0] = add(f7[0], P - mul(c, c));
    assert_eq!(f7.len(), 22);
    assert_eq!(f7[21], 1);
    f7
}

fn main() {
    let options = F4Options::new(Ordering::Grevlex, 24)
        .with_budget(Duration::from_secs(10));
    let mut all_x = BTreeSet::new();
    for (i, &(c, anchor_x, anchor_y)) in ANCHORS.iter().enumerate() {
        assert_eq!(lift_x(anchor_x, c), Some(anchor_y));
        assert_ne!(lift_x(anchor_x, c), Some((P - anchor_y) % P));
        let coefficients = coset_poly(c);
        let polynomial: Poly = coefficients.iter().enumerate().rev()
            .filter(|(_, value)| **value != 0)
            .map(|(degree, value)| (vec![degree as u32], *value))
            .collect();
        // Univariate F4 root-finding uses only public coefficients and c.
        // The anchor is checked only after solving (never entered into F4).
        let report = solve(&[polynomial], 1, P, &options);
        let roots: BTreeSet<u64> = match &report.verdict {
            Verdict::Solutions(solutions) => solutions.iter().map(|v| {
                assert_eq!(v.len(), 1);
                v[0]
            }).collect(),
            other => panic!("coset {i}: one-variable F4 did not extract roots: {other:?}"),
        };
        assert_eq!(roots.len(), 21, "coset {i}: missing Fp roots");
        assert!(roots.contains(&anchor_x), "anchor not recovered");
        let mut product = vec![1];
        for &x in &roots {
            let y = lift_x(x, c).expect("wrong signed root");
            assert_eq!(mul(y, y), rhs(x));
            assert_eq!(pow(y, 7), c);
            assert!(all_x.insert(x), "overlapping x supports");
            product = poly_mul(&product, &[(P - x) % P, 1]);
        }
        // Coefficient equality and degree 21 prove these are all roots even
        // over the algebraic closure (not merely a sample of Fp roots).
        assert_eq!(product, coefficients, "incorrect polynomial factorization");
        println!("coset={i} public_roots={} wall_ms={:.3} f4_row_muls={}",
            roots.len(), report.ms, report.field_ops);
    }
    assert_eq!(all_x.len(), 168);
    println!("PUBLIC_ONLY_PRIME31_COSET_ROOTS_PASS: 168/168; no ECDLP extracted");
}
