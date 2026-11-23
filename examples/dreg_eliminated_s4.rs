//! F4 solving degree of the eliminated, target-free `S₄` system
//! ([`build_eliminated_s4_system`] with no target: `x₁..x₄ ∈ V`, `4ℓ`
//! unknowns, `n` equations) beside the same presentation with a random target
//! (`x₄ = x_R`, `3ℓ` unknowns) on the same factor-base subspace.  The
//! follow-up to lead L1 (`research/target_free_s4_20261018/`).
//!
//! ```sh
//! cargo run --release --example dreg_eliminated_s4 -- --cells 16:4,20:5 \
//!   --basis gp|random --twists 0,0,0,0 --draws 4 --seed S --budget-secs 1200 [--count]
//! ```
//!
//! One JSON line per arm and draw.  Besides the F4 statistics each line
//! records the leading-form structure the lemma is about: `top_degree` (the
//! largest total degree among the `n` equations), `top_forms` (equations of
//! that degree) and `top_rank` (the `F₂`-rank of their degree-`top_degree`
//! homogeneous parts); `top_forms − top_rank` dependencies exist at the top
//! degree.  `exact_count` (with `--count`, `4ℓ ≤ 20`) is the number of
//! Boolean zeros, by evaluation.  A timed-out or oversize F4 run is
//! unmeasured.

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::koblitz_bench::random_subspace_basis;
use crypto_lib::cryptanalysis::koblitz_groebner::{build_eliminated_s4_system, FieldStructure};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::pq_f4_f2::groebner_basis_f4;
use crypto_lib::cryptanalysis::pq_groebner_f2::F2BoolPoly;
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::time::Duration;

/// (top degree, equations of that degree, rank of their top homogeneous parts,
/// degree histogram as "d:count" pairs).
fn top_forms(eqs: &[F2BoolPoly]) -> (u32, usize, usize, String) {
    let deg = |p: &F2BoolPoly| p.terms.iter().map(|m| m.degree()).max().unwrap_or(0);
    let top = eqs.iter().map(deg).max().unwrap_or(0);
    let mut hist: std::collections::BTreeMap<u32, usize> = Default::default();
    for p in eqs {
        *hist.entry(deg(p)).or_default() += 1;
    }
    let mut index: HashMap<u64, usize> = HashMap::new();
    let rows: Vec<Vec<usize>> = eqs
        .iter()
        .filter(|p| deg(p) == top && !p.is_zero())
        .map(|p| {
            p.terms
                .iter()
                .filter(|m| m.degree() == top)
                .map(|m| {
                    let k = index.len();
                    *index.entry(m.mask).or_insert(k)
                })
                .collect()
        })
        .collect();
    let words = index.len().div_ceil(64).max(1);
    let mut mat: Vec<Vec<u64>> = rows
        .iter()
        .map(|r| {
            let mut v = vec![0u64; words];
            for &c in r {
                v[c / 64] ^= 1 << (c % 64);
            }
            v
        })
        .collect();
    let mut rank = 0;
    for col in 0..index.len() {
        let (w, bit) = (col / 64, 1u64 << (col % 64));
        let Some(p) = (rank..mat.len()).find(|&i| mat[i][w] & bit != 0) else {
            continue;
        };
        mat.swap(rank, p);
        let pivot = mat[rank].clone();
        for (i, row) in mat.iter_mut().enumerate() {
            if i != rank && row[w] & bit != 0 {
                for (a, b) in row.iter_mut().zip(&pivot) {
                    *a ^= b;
                }
            }
        }
        rank += 1;
    }
    let h = hist
        .iter()
        .map(|(d, c)| format!("\"{d}\":{c}"))
        .collect::<Vec<_>>()
        .join(",");
    (top, rows.len(), rank, h)
}

#[allow(clippy::too_many_arguments)]
fn run_arm(
    arm: &str,
    n: u32,
    ell: usize,
    twists: &[u32; 4],
    basis_kind: &str,
    draw: u64,
    extra: &str,
    equations: Vec<F2BoolPoly>,
    n_vars: usize,
    budget: u64,
) {
    let n_eqs = equations.len();
    let (top, forms, rank, hist) = top_forms(&equations);
    let terms: usize = equations.iter().map(|p| p.terms.len()).sum();
    let (gb, stats) = groebner_basis_f4(equations, n_vars, Some(Duration::from_secs(budget)));
    println!(
        r#"{{"arm":"{arm}","n":{n},"ell":{ell},"twists":[{},{},{},{}],"basis":"{basis_kind}","draw":{draw},{extra}"n_vars":{n_vars},"n_eqs":{n_eqs},"terms":{terms},"eq_degrees":{{{hist}}},"top_degree":{top},"top_forms":{forms},"top_rank":{rank},"timed_out":{},"oversize":{},"degree_reached":{},"solving_degree":{},"basis_len":{},"matrix_rows_max":{},"matrix_cols_max":{},"peak_matrix_bytes":{},"secs":{:.3}}}"#,
        twists[0],
        twists[1],
        twists[2],
        twists[3],
        stats.timed_out,
        stats.oversize,
        stats.degree_reached,
        stats.solving_degree,
        gb.len(),
        stats.matrix_rows_max,
        stats.matrix_cols_max,
        stats.peak_matrix_bytes,
        stats.wall_ns as f64 / 1e9
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flag = |name: &str| -> Option<String> {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let cells: Vec<(u32, usize)> = flag("--cells")
        .expect("--cells n:ell,n:ell")
        .split(',')
        .map(|c| {
            let (a, b) = c.split_once(':').expect("n:ell");
            (a.parse().expect("n"), b.parse().expect("ell"))
        })
        .collect();
    let basis_kind = flag("--basis").unwrap_or_else(|| "gp".into());
    assert!(
        basis_kind == "gp" || basis_kind == "random",
        "--basis gp|random"
    );
    let tw: Vec<u32> = flag("--twists")
        .unwrap_or_else(|| "0,0,0,0".into())
        .split(',')
        .map(|t| t.parse().expect("twist"))
        .collect();
    let twists: [u32; 4] = tw.try_into().expect("--twists a1,a2,a3,a4");
    let draws: u64 = flag("--draws").map_or(1, |v| v.parse().expect("--draws"));
    let seed: u64 = flag("--seed").map_or(1, |v| v.parse().expect("--seed"));
    let budget: u64 = flag("--budget-secs").map_or(1200, |v| v.parse().expect("--budget-secs"));
    let count = args.iter().any(|a| a == "--count");

    for (n, ell) in cells {
        let irr = find_irreducible_sparse(n).expect("field");
        let st = FieldStructure::new(n, &irr);
        let gf = Gf2::new(&irr);
        let b = F2mElement::one(n);
        let word = |e: &F2mElement| e.raw_bits().first().copied().unwrap_or(0);
        for draw in 0..draws {
            let mut rng = StdRng::seed_from_u64(
                seed ^ ((n as u64) << 40) ^ ((ell as u64) << 32) ^ draw.wrapping_mul(0x9E37_79B9),
            );
            let basis: Vec<F2mElement> = if basis_kind == "gp" {
                let mut p = 1u64;
                (0..ell)
                    .map(|_| {
                        let e = F2mElement::from_biguint(&BigUint::from(p), n);
                        p = gf.mul(p, 2);
                        e
                    })
                    .collect()
            } else {
                random_subspace_basis(n, ell, &mut rng)
            };
            let x_r =
                F2mElement::from_biguint(&BigUint::from(rng.gen::<u64>() & ((1u64 << n) - 1)), n);
            let v_basis = basis
                .iter()
                .map(|e| word(e).to_string())
                .collect::<Vec<_>>()
                .join(",");
            let Some(sys) = build_eliminated_s4_system(&basis, &b, None, twists, &st) else {
                eprintln!("({n},{ell}): 4ell > 64, skipped");
                continue;
            };
            // The progression base is deterministic: its target-free system is
            // the same on every draw, so it is solved once (draw 0); only the
            // target arm's x_R changes between draws.
            let solve_free = basis_kind == "random" || draw == 0;
            let exact = if solve_free && count && 4 * ell <= 20 {
                (0..1u64 << sys.n_vars)
                    .filter(|&a| sys.equations.iter().all(|p| p.eval(a) == 0))
                    .count()
                    .to_string()
            } else {
                "null".to_string()
            };
            if solve_free {
                run_arm(
                    "target_free_s4",
                    n,
                    ell,
                    &twists,
                    &basis_kind,
                    draw,
                    &format!(r#""v_basis":[{v_basis}],"exact_count":{exact},"#),
                    sys.equations,
                    sys.n_vars,
                    budget,
                );
            }
            let t = build_eliminated_s4_system(&basis, &b, Some(&x_r), twists, &st)
                .expect("3ell <= 64");
            let exact_t = if count && 3 * ell <= 20 {
                (0..1u64 << t.n_vars)
                    .filter(|&a| t.equations.iter().all(|p| p.eval(a) == 0))
                    .count()
                    .to_string()
            } else {
                "null".to_string()
            };
            run_arm(
                "target_s4",
                n,
                ell,
                &twists,
                &basis_kind,
                draw,
                &format!(r#""x_r":{},"exact_count":{exact_t},"#, word(&x_r)),
                t.equations,
                t.n_vars,
                budget,
            );
        }
    }
}
