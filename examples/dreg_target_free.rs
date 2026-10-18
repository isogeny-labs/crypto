//! F4 solving degree of the target-free 4-sum Weil-descended Semaev system
//! ([`build_target_free_system`]) beside the chained `m = 3` target system on
//! the same factor-base subspace.
//!
//! ```sh
//! cargo run --release --example dreg_target_free -- --cells 16:4,20:5 \
//!   --basis gp|random --twists 0,0,0,0 --draws 4 --seed S --budget-secs 1200 [--target] [--no-count]
//! ```
//!
//! One JSON line per cell and draw (and per `--target` arm).  `exact_count`
//! is the number of solutions of the target-free system, counted with
//! [`S3LinkRoots`]; it is skipped (null) with `--no-count` or when `4ℓ > 24`.
//! A timed-out or oversize run is unmeasured.

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::koblitz_bench::{random_subspace_basis, S3LinkRoots};
use crypto_lib::cryptanalysis::koblitz_groebner::{
    build_decomposition_system, build_target_free_system, FieldStructure,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::pq_f4_f2::groebner_basis_f4;
use crypto_lib::cryptanalysis::pq_groebner_f2::F2BoolPoly;
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::Duration;

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
    let (gb, stats) = groebner_basis_f4(equations, n_vars, Some(Duration::from_secs(budget)));
    println!(
        r#"{{"arm":"{arm}","n":{n},"ell":{ell},"twists":[{},{},{},{}],"basis":"{basis_kind}","draw":{draw},{extra}"n_vars":{n_vars},"n_eqs":{n_eqs},"timed_out":{},"oversize":{},"degree_reached":{},"solving_degree":{},"basis_len":{},"matrix_rows_max":{},"matrix_cols_max":{},"peak_matrix_bytes":{},"secs":{:.3}}}"#,
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
    let with_target = args.iter().any(|a| a == "--target");
    let no_count = args.iter().any(|a| a == "--no-count");

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
            let Some(sys) = build_target_free_system(&basis, &b, twists, &st) else {
                eprintln!("({n},{ell}): 4ell+n > 64, skipped");
                continue;
            };
            let exact = if no_count || 4 * ell > 24 {
                "null".to_string()
            } else {
                let spans: Vec<Vec<u64>> = (0..4)
                    .map(|i| {
                        let w: Vec<u64> = basis
                            .iter()
                            .map(|v| {
                                let mut x = word(v);
                                for _ in 0..twists[i] {
                                    x = gf.sqr(x);
                                }
                                x
                            })
                            .collect();
                        (0..1u64 << ell)
                            .map(|c| {
                                (0..ell)
                                    .filter(|t| c >> t & 1 == 1)
                                    .fold(0, |a, t| a ^ w[t])
                            })
                            .collect()
                    })
                    .collect();
                let solver = S3LinkRoots::new(&irr, word(&b));
                let mut count = 0u64;
                for c in 0..1u64 << (4 * ell) {
                    let cs: Vec<usize> = (0..4)
                        .map(|i| ((c >> (i * ell)) & ((1 << ell) - 1)) as usize)
                        .collect();
                    let r1 = solver.roots(spans[0][cs[0]], spans[1][cs[1]]);
                    if r1.is_empty() {
                        continue;
                    }
                    let r2 = solver.roots(spans[2][cs[2]], spans[3][cs[3]]);
                    count += r1.iter().filter(|u| r2.contains(u)).count() as u64;
                }
                count.to_string()
            };
            run_arm(
                "target_free",
                n,
                ell,
                &twists,
                &basis_kind,
                draw,
                &format!(r#""exact_count":{exact},"#),
                sys.equations,
                sys.n_vars,
                budget,
            );
            if with_target {
                let x_r = F2mElement::from_biguint(
                    &BigUint::from(rng.gen::<u64>() & ((1u64 << n) - 1)),
                    n,
                );
                let Some(t) = build_decomposition_system(&basis, &x_r, &b, 3, &st) else {
                    eprintln!("({n},{ell}): m=3 target system too large, skipped");
                    continue;
                };
                run_arm(
                    "target_m3",
                    n,
                    ell,
                    &twists,
                    &basis_kind,
                    draw,
                    &format!(r#""x_r":{},"#, word(&x_r)),
                    t.equations,
                    t.n_vars,
                    budget,
                );
            }
        }
    }
}
