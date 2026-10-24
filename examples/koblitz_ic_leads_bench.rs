//! Leads L1, L3 and L4 of `research/koblitz_ic_new_leads_20261018/`,
//! measured ([`koblitz_ic_leads`]).
//!
//! ```sh
//! cargo run --release --example koblitz_ic_leads_bench -- lambda    > lambda.jsonl
//! cargo run --release --example koblitz_ic_leads_bench -- gluing    > gluing.jsonl
//! cargo run --release --example koblitz_ic_leads_bench -- bounds    > bounds.jsonl
//! ```
//!
//! - `lambda`: the λ-closure check on both Koblitz curves over
//!   `F_{2^11}`, `F_{2^13}`, `F_{2^17}`, `F_{2^19}`, twelve subspaces each.
//! - `gluing`: counted pairs-and-solve against counted Agreeing–Gluing on
//!   the chained `S₃` system, `m = 3` and `m = 4`, four draws per cell,
//!   with the two solution counts required to agree.
//! - `bounds`: the Caminata–Gorla Macaulay bound of the descended system of
//!   every cell the closure studies measured, built and read off the
//!   system, and the shape formula for ECC2K-130.

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::koblitz_bench::ladder_sample;
use crypto_lib::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
use crypto_lib::cryptanalysis::koblitz_ic_leads::{agreeing_gluing, degree_bounds, lambda};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::Instant;

const SEED: u64 = 20261024;

fn word(e: &F2mElement) -> u64 {
    e.raw_bits().first().copied().unwrap_or(0)
}

fn run_lambda() {
    let mut rng = StdRng::seed_from_u64(SEED);
    for n in [11u32, 13, 17, 19] {
        let irr = find_irreducible_sparse(n).expect("field");
        let mask = (1u64 << n) - 1;
        for a in 0..2u64 {
            let bases: Vec<Vec<u64>> = [3usize, n as usize / 2, n as usize - 3]
                .iter()
                .cycle()
                .take(12)
                .map(|&d| (0..d).map(|_| rng.gen::<u64>() & mask).collect())
                .collect();
            let started = Instant::now();
            let r = lambda::check(&irr, a, &bases);
            println!(
                r#"{{"lead":"L1","n":{n},"a":{a},"points":{},"doubling_identity":{},"negation_identity":{},"subspaces":{},"closure_identity":{},"base_sizes":{:?},"secs":{:.3}}}"#,
                r.points,
                r.doubling_identity,
                r.negation_identity,
                r.subspaces,
                r.closure_identity,
                r.base_sizes,
                started.elapsed().as_secs_f64()
            );
        }
    }
}

fn run_gluing() {
    let mut rng = StdRng::seed_from_u64(SEED);
    // n chosen near the yield balance m·ℓ ≈ n so draws have O(1) solutions.
    let cells: Vec<(usize, u32, usize)> = (3..=10)
        .map(|l| (3usize, (3 * l) as u32, l))
        .chain((3..=7).map(|l| (4usize, (4 * l) as u32, l)))
        .collect();
    for (m, n, ell) in cells {
        let irr = find_irreducible_sparse(n).expect("field");
        for draw in 0..4 {
            let (basis, x_r) = ladder_sample(n, ell, &mut rng);
            let words: Vec<u64> = basis.iter().map(word).collect();
            let xr = word(&x_r);
            let t0 = Instant::now();
            let ps = agreeing_gluing::pairs_and_solve(m, &words, xr, 1, &irr);
            let t1 = t0.elapsed().as_secs_f64();
            let t2 = Instant::now();
            let ag = agreeing_gluing::agreeing_gluing(m, &words, xr, 1, &irr);
            let t3 = t2.elapsed().as_secs_f64();
            assert_eq!(
                ps.solutions, ag.solutions,
                "m={m} n={n} ell={ell} draw={draw}"
            );
            println!(
                r#"{{"lead":"L3","m":{m},"n":{n},"ell":{ell},"draw":{draw},"solutions":{},"pairs":{{"root_solves":{},"probes":{},"secs":{t1:.4}}},"gluing":{{"root_solves":{},"probes":{},"table_entries":{},"discarded":{},"secs":{t3:.4}}}}}"#,
                ps.solutions,
                ps.root_solves,
                ps.probes,
                ag.root_solves,
                ag.probes,
                ag.table_entries,
                ag.discarded
            );
        }
    }
}

fn run_bounds() {
    // Cells the closure studies measured: (m, n, ℓ, measured closure
    // degree, source).  ≥5 means degree 4 did not refute.
    let cells: &[(usize, u32, usize, &str, &str)] = &[
        (3, 16, 6, "4", "dreg_closure_scale_20261016"),
        (3, 18, 6, "4", "dreg_closure_scale_20261016"),
        (3, 19, 7, ">=5", "dreg_closure_scale_20261016"),
        (3, 21, 7, ">=5", "dreg_closure_scale_20261016"),
        (3, 22, 6, "4", "dreg_closure_driver_20261016"),
        (3, 24, 6, "4", "dreg_closure_driver_20261016"),
        (4, 11, 5, "4", "dreg_closure_driver_20261016"),
        (4, 12, 5, "4", "dreg_closure_driver_20261016"),
        (4, 15, 6, "4", "dreg_closure_m4l6_20261018"),
    ];
    let mut rng = StdRng::seed_from_u64(SEED);
    for &(m, n, ell, measured, source) in cells {
        let irr = find_irreducible_sparse(n).expect("field");
        let st = FieldStructure::new(n, &irr);
        let (basis, x_r) = ladder_sample(n, ell, &mut rng);
        let sys =
            build_decomposition_system(&basis, &x_r, &F2mElement::one(n), m, &st).expect("system");
        let b = degree_bounds::macaulay_bound(&sys.equations, sys.n_vars);
        let cubic = sys
            .equations
            .iter()
            .filter(|p| p.terms.iter().map(|t| t.mask.count_ones()).max() == Some(3))
            .count();
        println!(
            r#"{{"lead":"L4","m":{m},"n":{n},"ell":{ell},"n_vars":{},"equations":{},"cubic_equations":{cubic},"max_degree":{},"generic_coordinates":{},"macaulay_bound":{},"shape_bound":{},"measured_closure_degree":"{measured}","source":"{source}","field_level_bound_log2":{:.1}}}"#,
            b.n_vars,
            b.equations,
            b.max_degree,
            b.generic_coordinates,
            b.bound,
            shape_bound(b.n_vars, cubic, 3),
            degree_bounds::field_level_bound_log2(m, n, ell as u32)
        );
    }
    // ECC2K-130 at m = 4, ℓ = 33: N = 2·131 + 4·33 and 3·131 links, all
    // taken cubic (an upper value: a quadratic link lowers the bound).
    let (m, n, ell) = (4usize, 131u32, 33usize);
    let nv = (m - 2) * n as usize + m * ell;
    let eqs = (m - 1) * n as usize;
    println!(
        r#"{{"lead":"L4","m":{m},"n":{n},"ell":{ell},"n_vars":{nv},"equations":{eqs},"cubic_equations":{eqs},"max_degree":3,"generic_coordinates":true,"macaulay_bound":null,"shape_bound":{},"measured_closure_degree":"unmeasured","source":"ECC2K-130 shape","field_level_bound_log2":{:.1}}}"#,
        shape_bound(nv, eqs, 3),
        degree_bounds::field_level_bound_log2(m, n, ell as u32)
    );
}

/// The Macaulay bound from the shape alone: `top` equations of degree `d`,
/// the rest of degree 2 (quadratic links and the `nv` field equations),
/// `l = nv + 1`.
fn shape_bound(nv: usize, top: usize, d: u32) -> u64 {
    let l = nv + 1;
    let top_d = top.min(l);
    let rest = l - top_d;
    (top_d as u64) * d as u64 + (rest as u64) * 2 - l as u64 + 1
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("lambda") => run_lambda(),
        Some("gluing") => run_gluing(),
        Some("bounds") => run_bounds(),
        _ => panic!("usage: koblitz_ic_leads_bench lambda|gluing|bounds"),
    }
}
