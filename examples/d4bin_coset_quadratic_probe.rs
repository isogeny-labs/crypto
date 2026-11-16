//! Lane B probes on the char-2 D4 (k = 4) Frobenius form
//! (research/leads_20261111/lane_B_binary_ic.md):
//!
//! * `coset`: LEAD-ICB-d4bin-exponent-coset — do the monomials carrying the
//!   symmetrised H, in the reduced unknowns (S2, S3, s4, P), lie in a coset of
//!   an index-2 sublattice (F_2-rank of exponent differences 3, or some linear
//!   form mod 2 constant)?  100 instances `generate_instance(m, seed)` per m.
//! * `quad`: LEAD-ICB-d4bin-constant-quadratic-elimination step 1 — on the
//!   `system_frobenius` components of random targets: (a) monomial support,
//!   (b) rank of the 4 x 3 matrix of S2^2, S2*S3, S3^2 coefficients,
//!   (c) the Macaulay-11 staircase per (S2, S3) pattern.
//!
//! ```sh
//! RAYON_NUM_THREADS=1 cargo run --release --example d4bin_coset_quadratic_probe -- coset 8 100
//! RAYON_NUM_THREADS=1 cargo run --release --example d4bin_coset_quadratic_probe -- quad 12 300
//! ```
use std::collections::{BTreeMap, BTreeSet};
use std::env;

use crypto_lib::cryptanalysis::gaudry_quartic_bin::{
    find_bin_curve, generate_instance, macaulay_staircase, solve_d4_bin, BinOptions, Gf2q,
    SymmetrisedBin, FROBENIUS_WEIGHTS,
};
use rand::rngs::StdRng;
use rand::SeedableRng;

type Fe = u32;

/// F_2-rank of a set of 4-bit vectors.
fn f2_rank(vs: &[u8]) -> usize {
    let mut basis = [0u8; 4];
    let mut r = 0;
    for &v0 in vs {
        let mut v = v0 & 15;
        for b in (0..4).rev() {
            if v >> b & 1 == 0 {
                continue;
            }
            if basis[b] == 0 {
                basis[b] = v;
                r += 1;
                break;
            }
            v ^= basis[b];
        }
    }
    r
}

/// Parity vector of a reduced exponent (a, b, c, d): bit 0 = a mod 2, ...
fn par(e: &[u8; 4]) -> u8 {
    (e[0] & 1) | (e[1] & 1) << 1 | (e[2] & 1) << 2 | (e[3] & 1) << 3
}

/// Coset analysis of a monomial set: (rank of differences, forms l (1..15)
/// constant on the set with their constant value, #monos with a even, #b even).
fn coset(ms: &[[u8; 4]]) -> (usize, Vec<(u8, u8)>, usize, usize) {
    let p: Vec<u8> = ms.iter().map(par).collect();
    let diffs: Vec<u8> = p.iter().map(|&x| x ^ p[0]).collect();
    let rank = f2_rank(&diffs);
    let mut forms = Vec::new();
    for l in 1u8..16 {
        let vals: BTreeSet<u8> = p
            .iter()
            .map(|&x| ((x & l).count_ones() & 1) as u8)
            .collect();
        if vals.len() == 1 {
            forms.push((l, *vals.iter().next().unwrap()));
        }
    }
    let ae = ms.iter().filter(|e| e[0] % 2 == 0).count();
    let be = ms.iter().filter(|e| e[1] % 2 == 0).count();
    (rank, forms, ae, be)
}

fn run_coset(m: u32, n: u64) {
    // Controls on a synthetic set: an "a even" set passes, a planted odd-a
    // monomial is rejected; a raw monomial planted odd in s2 trips the
    // existing parity rule.
    let synth: Vec<[u8; 4]> = vec![
        [0, 0, 0, 0],
        [2, 0, 0, 0],
        [0, 1, 1, 0],
        [0, 0, 2, 2],
        [2, 1, 0, 1],
    ];
    let (_, f0, ae0, _) = coset(&synth);
    assert!(
        f0.contains(&(1, 0)) && ae0 == synth.len(),
        "control: a-even set must pass"
    );
    let mut planted = synth.clone();
    planted.push([1, 0, 0, 0]);
    let (_, f1, ae1, _) = coset(&planted);
    assert!(
        !f1.iter().any(|&(l, _)| l == 1) && ae1 == synth.len(),
        "control: planted odd-a must fail"
    );
    println!("control synthetic: a-even set passes (forms {f0:?}); planted [1,0,0,0] rejected (forms {f1:?})");
    let raw_odd = |e: &[u8; 4]| e[0] % 2 == 1 || e[1] % 2 == 1 || e[3] % 2 == 1;
    assert!(raw_odd(&[1, 0, 0, 0]) && !raw_odd(&[2, 0, 1, 0]));

    let mut hist_rank: BTreeMap<usize, u64> = BTreeMap::new();
    let mut hist_forms: BTreeMap<String, u64> = BTreeMap::new();
    let mut hist_ncarry: BTreeMap<usize, u64> = BTreeMap::new();
    let (mut a_all, mut b_all, mut fails, mut nonfrob) = (0u64, 0u64, 0u64, 0u64);
    let mut supports: BTreeMap<Vec<[u8; 4]>, u64> = BTreeMap::new();
    let mut tgt_rank: BTreeMap<usize, u64> = BTreeMap::new();
    for seed in 1..=n {
        let curve = generate_instance(m, seed);
        let mut rng = StdRng::seed_from_u64(seed ^ 0x77);
        let pre = match SymmetrisedBin::precompute(&curve, &mut rng, 8) {
            Ok(p) => p,
            Err(e) => {
                println!("m {m} seed {seed}: precompute failed: {e}");
                fails += 1;
                continue;
            }
        };
        let (nc, odd) = pre.parity_counts();
        if odd != 0 || !pre.frobenius_reducible() {
            nonfrob += 1;
            println!("m {m} seed {seed}: parity_counts ({nc}, {odd}) NOT frobenius-reducible");
            continue;
        }
        let mut red: Vec<[u8; 4]> = pre
            .monos
            .iter()
            .zip(&pre.coef)
            .filter(|(_, c)| c.iter().any(|v| !v.is_zero()))
            .map(|(e, _)| [e[0] / 2, e[1] / 2, e[2], e[3] / 2])
            .collect();
        red.sort_unstable();
        assert_eq!(red.len(), nc);
        let (rank, forms, ae, be) = coset(&red);
        *hist_rank.entry(rank).or_default() += 1;
        *hist_forms.entry(format!("{forms:?}")).or_default() += 1;
        *hist_ncarry.entry(nc).or_default() += 1;
        a_all += u64::from(ae == red.len());
        b_all += u64::from(be == red.len());
        *supports.entry(red.clone()).or_default() += 1;
        // Per-target support of the four Weil components (3 targets).
        let f = &curve.f;
        let mut trng = StdRng::seed_from_u64(seed ^ 0x55);
        for _ in 0..3 {
            let r = loop {
                let x = f.random(&mut trng);
                if let Some(p) = curve.lift_x(&x, &mut trng) {
                    break p;
                }
            };
            let comps = pre.system_frobenius(f, &curve.t_of_x(&r.x));
            let sup: BTreeSet<[u8; 4]> = comps.iter().flat_map(|c| c.keys().copied()).collect();
            let sup: Vec<[u8; 4]> = sup.into_iter().collect();
            *tgt_rank.entry(coset(&sup).0).or_default() += 1;
        }
        if seed <= 2 {
            println!(
                "m {m} seed {seed}: carrying {nc} (odd raw {odd}), reduced exps {red:?}\n  rank {rank}, constant forms {forms:?}, a-even {ae}/{nc}, b-even {be}/{nc}"
            );
        }
    }
    println!("SUMMARY m {m} instances {n}: precompute failures {fails}, not frobenius-reducible {nonfrob}");
    println!("  #carrying histogram {hist_ncarry:?}");
    println!("  F2-rank of exponent differences histogram {hist_rank:?}");
    println!("  constant-form sets histogram {hist_forms:?}");
    println!("  instances with all carrying monomials a-even (S2 exp even): {a_all}; b-even (S3): {b_all}");
    println!("  per-target Weil support rank histogram (3 targets/instance) {tgt_rank:?}");
    println!("  distinct carrying supports: {}", supports.len());
    if let Some((s, c)) = supports.iter().max_by_key(|(_, c)| **c) {
        println!("  most common support ({c} instances): {s:?}");
    }
}

fn rank_fq(f: &Gf2q, mut a: Vec<Vec<Fe>>) -> usize {
    let (nr, nc) = (a.len(), a[0].len());
    let mut r = 0;
    for c in 0..nc {
        let Some(p) = (r..nr).find(|&i| a[i][c] != 0) else {
            continue;
        };
        a.swap(r, p);
        let inv = f.inv_nc(a[r][c]);
        for i in 0..nr {
            if i != r && a[i][c] != 0 {
                let fct = f.mul_nc(a[i][c], inv);
                for j in 0..nc {
                    let v = f.mul_nc(fct, a[r][j]);
                    a[i][j] ^= v;
                }
            }
        }
        r += 1;
    }
    r
}

fn run_quad(m: u32, trials: u64) {
    let c = find_bin_curve(m, 1, 16).expect("curve");
    let curve = c.curve;
    let mut rng = StdRng::seed_from_u64(1 ^ 0x9e);
    let pre = SymmetrisedBin::precompute(&curve, &mut rng, 8).expect("pre");
    let opts = BinOptions {
        weighted: true,
        frobenius: true,
        sparsest: true,
        replay: true,
        frob_roots: false,
    };
    let fq = &curve.f.f;
    let quad = [[2u8, 0, 0, 0], [1, 1, 0, 0], [0, 2, 0, 0]];
    let mut rank_hist: BTreeMap<usize, u64> = BTreeMap::new();
    let mut support_union: BTreeMap<[u8; 4], u64> = BTreeMap::new();
    let mut nterms_hist: BTreeMap<usize, u64> = BTreeMap::new();
    let mut leads_hist: BTreeMap<String, u64> = BTreeMap::new();
    let mut pattern_hist: BTreeMap<String, u64> = BTreeMap::new();
    let mut lt_s2s3_hist: BTreeMap<String, u64> = BTreeMap::new();
    let (mut dim_match, mut dim_mismatch, mut solver_fail, mut nost) = (0u64, 0u64, 0u64, 0u64);
    let mut dims: BTreeMap<usize, u64> = BTreeMap::new();
    let mut rrng = StdRng::seed_from_u64(77);
    for _ in 0..trials {
        let r = loop {
            let x = curve.f.random(&mut rrng);
            if let Some(p) = curve.lift_x(&x, &mut rrng) {
                break p;
            }
        };
        let comps = pre.system_frobenius(&curve.f, &curve.t_of_x(&r.x));
        for comp in &comps {
            *nterms_hist.entry(comp.len()).or_default() += 1;
            for k in comp.keys() {
                *support_union.entry(*k).or_default() += 1;
            }
        }
        let mat: Vec<Vec<Fe>> = comps
            .iter()
            .map(|cp| quad.iter().map(|q| *cp.get(q).unwrap_or(&0)).collect())
            .collect();
        *rank_hist.entry(rank_fq(fq, mat)).or_default() += 1;
        let Some((leads, standard)) = macaulay_staircase(fq, &comps, &FROBENIUS_WEIGHTS, 11, true)
        else {
            nost += 1;
            continue;
        };
        *leads_hist.entry(format!("{leads:?}")).or_default() += 1;
        let mut pat: BTreeMap<(u8, u8), usize> = BTreeMap::new();
        for s in &standard {
            *pat.entry((s[0], s[1])).or_default() += 1;
        }
        *pattern_hist.entry(format!("{pat:?}")).or_default() += 1;
        // Leading terms' (S2, S3) parts.
        let lt: Vec<String> = leads
            .iter()
            .map(|l| l.map_or("-".into(), |l| format!("S2^{} S3^{}", l[0], l[1])))
            .collect();
        *lt_s2s3_hist.entry(lt.join(" | ")).or_default() += 1;
        // Control: the solver's dim on the same target.
        let mut srng = StdRng::seed_from_u64(5);
        match solve_d4_bin(&curve, &pre, &r, opts, 11, &mut srng, None) {
            Ok(s) => {
                *dims.entry(s.solve.dim).or_default() += 1;
                if s.solve.dim == standard.len() {
                    dim_match += 1;
                } else {
                    dim_mismatch += 1;
                }
            }
            Err(_) => solver_fail += 1,
        }
    }
    println!("QUAD m {m} curve seed 1 trials {trials}");
    println!("(a) terms per component histogram {nterms_hist:?}");
    println!(
        "(a) monomial support union (S2,S3,s4,P) -> #components containing it, {} monomials:",
        support_union.len()
    );
    for (k, v) in &support_union {
        let wd = 2 * k[0] as u32 + 2 * k[1] as u32 + k[2] as u32 + k[3] as u32;
        println!("    {k:?} wdeg {wd}: {v}");
    }
    println!("(b) rank of 4x3 quadratic-part matrix histogram {rank_hist:?}");
    println!("(c) staircase unavailable (equation above degree): {nost}");
    println!("(c) triangularised leads histogram:");
    for (k, v) in &leads_hist {
        println!("    {v}: {k}");
    }
    println!("(c) leading terms in (S2,S3):");
    for (k, v) in &lt_s2s3_hist {
        println!("    {v}: {k}");
    }
    println!("(c) standard monomials per (S2,S3) pattern histogram:");
    for (k, v) in &pattern_hist {
        println!("    {v}: {k}");
    }
    println!("control: solver dim histogram {dims:?}; staircase size == solver dim {dim_match}, mismatch {dim_mismatch}, solver Err {solver_fail}");
}

fn main() {
    let a: Vec<String> = env::args().skip(1).collect();
    let mode = a.first().map(String::as_str).unwrap_or("coset");
    let m: u32 = a.get(1).map_or(8, |v| v.parse().unwrap());
    let n: u64 = a.get(2).map_or(100, |v| v.parse().unwrap());
    match mode {
        "coset" => run_coset(m, n),
        "quad" => run_quad(m, n),
        _ => panic!("mode coset|quad"),
    }
}
