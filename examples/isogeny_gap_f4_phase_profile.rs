//! # Per-system F4 profile for research/isogeny_conductor_gap_round6_20261018 (E8, E9a)
//!
//! For every Koblitz curve `koblitz_instance(a, n)` with 17 ≤ n ≤ 29 and
//! cofactor 2 or 4, two `binary-subspace` bases (default prefix and trace
//! coset) at d = 8, 9, 10, and 200 shared targets `[k]G`, it descends the
//! `S₃` system and solves it twice:
//! - full `f4-f2` ([`groebner_basis_f4`] and the reduced-basis read-off);
//! - early (`f4-f2-early`, free = 8: [`groebner_basis_f4_until`] and
//!   [`solutions_from_linear_part`]).
//!
//! It writes one JSON line per system (work, extraction tests, steps,
//! verdicts, lifted decompositions), and one per configuration (base size,
//! fraction of base points in 2E).  A solution-set mismatch between the two
//! runs aborts (PROTOCOL P-E8b).
//!
//! ```bash
//! cargo run --release --example isogeny_gap_f4_phase_profile -- \
//!   --out research/isogeny_conductor_gap_round6_20261018/e8/records.jsonl
//! ```
//!
//! Resumable: configurations already in `--out` are skipped.  A
//! configuration stops starting new targets after 30 minutes of wall time
//! and is then recorded as truncated (PROTOCOL amendment 1).

use std::collections::BTreeSet;
use std::io::Write;
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    binary_affine_subspace_factor_base, koblitz_instance, lift_abscissae, BinaryGroup,
    CountedGroup, GroupOps,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::trace_coset_basis;
use crypto_lib::cryptanalysis::pq_descent_symbolic::descend_affine;
use crypto_lib::cryptanalysis::pq_f4_f2::{
    groebner_basis_f4, groebner_basis_f4_until, solutions_from_linear_part,
    solutions_from_reduced_basis,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;

const TARGETS: usize = 200;
const SEED: u64 = 20261018;
const FREE: usize = 8;
/// Wall budget per configuration (PROTOCOL amendment 1): no new target
/// starts after it; the configuration is then marked truncated.
const BUDGET_S: f64 = 1800.0;

fn arg(args: &[String], name: &str) -> String {
    let i = args
        .iter()
        .position(|a| a == name)
        .unwrap_or_else(|| panic!("missing {name}"));
    args[i + 1].clone()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = arg(&args, "--out");
    let done: BTreeSet<String> = std::fs::read_to_string(&out)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .filter(|v| v["kind"] == "config_done")
        .map(|v| v["config"].as_str().unwrap().to_string())
        .collect();
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&out)
        .expect("open --out");
    for n in 17..=29u32 {
        for a in 0..=1u8 {
            let Some(inst) = koblitz_instance(a, n) else {
                continue;
            };
            if inst.cofactor != 2 && inst.cofactor != 4 {
                continue;
            }
            let g = BinaryGroup(&inst.fast);
            let mut rng = StdRng::seed_from_u64(SEED ^ u64::from(n) ^ (u64::from(a) << 8));
            let mut gops = GroupOps::default();
            let targets: Vec<_> = (0..TARGETS)
                .map(|_| {
                    let k = rng.gen_range(1..inst.r);
                    g.mul(&mut gops, inst.generator, k)
                })
                .collect();
            for d in 8..=10u32 {
                for base in ["default", "coset"] {
                    let config = format!("{}:d{d}:{base}", inst.name);
                    if done.contains(&config) {
                        continue;
                    }
                    let (basis, offset) = if base == "default" {
                        ((0..d).map(|k| 1u64 << k).collect::<Vec<_>>(), 0)
                    } else {
                        trace_coset_basis(&inst, d).expect("ker Tr is large enough")
                    };
                    let fb = binary_affine_subspace_factor_base(&inst, &basis, offset);
                    let half = inst.r * inst.cofactor / 2;
                    let in_two_e = fb
                        .points
                        .iter()
                        .filter(|&&p| g.mul(&mut gops, p, half).infinity)
                        .count();
                    let started = Instant::now();
                    let mut systems_run = 0usize;
                    for (t, &target) in targets.iter().enumerate() {
                        if started.elapsed().as_secs_f64() > BUDGET_S {
                            break;
                        }
                        systems_run += 1;
                        let bases = vec![basis.clone(), basis.clone()];
                        let sys = descend_affine(&inst.gf, inst.b, target.x, &bases, offset)
                            .expect("dimension fits");
                        let eqs = sys.equations.clone();
                        let nv = sys.n_vars;
                        let (gb, full) = groebner_basis_f4(eqs.clone(), nv, None);
                        let mut full_tests = 0u64;
                        let mut full_sol =
                            solutions_from_reduced_basis(&gb, nv, 24, &mut full_tests)
                                .expect("reduced basis");
                        full_sol.retain(|&v| {
                            eqs.iter().all(|e| {
                                full_tests += e.terms.len() as u64;
                                e.eval(v) == 0
                            })
                        });
                        let (part, early) =
                            groebner_basis_f4_until(eqs.clone(), nv, None, Some(FREE));
                        let (mut early_tests, mut early_xors) = (0u64, 0u64);
                        let early_sol = solutions_from_linear_part(
                            &part,
                            &eqs,
                            nv,
                            24,
                            &mut early_tests,
                            &mut early_xors,
                        )
                        .expect("at most 24 free");
                        let (mut a_s, mut b_s) = (full_sol.clone(), early_sol.clone());
                        a_s.sort_unstable();
                        b_s.sort_unstable();
                        assert_eq!(
                            a_s, b_s,
                            "P-E8b: solution sets differ on {config}, target {t}"
                        );
                        let lifted = full_sol
                            .iter()
                            .filter(|&&v| {
                                lift_abscissae(&g, &fb, &mut gops, &sys.lift(v), target).is_some()
                            })
                            .count();
                        let rec = json!({
                            "kind": "system", "config": config, "curve": inst.name, "a": a, "n": n,
                            "cofactor": inst.cofactor, "d": d, "base": base, "target": t,
                            "n_vars": nv, "n_equations": eqs.len(),
                            "solutions": full_sol.len(), "lifted": lifted,
                            "full": {"word_xors": full.word_xors, "extraction_tests": full_tests,
                                     "steps": full.steps, "solving_degree": full.solving_degree,
                                     "degree_reached": full.degree_reached, "wall_ns": full.wall_ns,
                                     "matrix_rows_sum": full.matrix_rows_sum},
                            "early": {"word_xors": early.word_xors, "extraction_tests": early_tests,
                                      "extraction_xors": early_xors, "steps": early.steps,
                                      "early_stopped": early.early_stopped,
                                      "free_at_stop": early.free_at_stop, "wall_ns": early.wall_ns},
                        });
                        writeln!(file, "{rec}").unwrap();
                    }
                    let cfg = json!({
                        "kind": "config_done", "config": config, "curve": inst.name, "a": a, "n": n,
                        "cofactor": inst.cofactor, "r": inst.r, "d": d, "base": base,
                        "offset": offset, "basis": basis, "base_points": fb.points.len(),
                        "base_abscissae": fb.abscissae, "base_points_in_2e": in_two_e,
                        "targets": TARGETS, "systems_run": systems_run,
                        "truncated": systems_run < TARGETS, "budget_s": BUDGET_S,
                        "free": FREE, "seed": SEED,
                        "wall_s": started.elapsed().as_secs_f64(),
                    });
                    writeln!(file, "{cfg}").unwrap();
                    file.flush().unwrap();
                    eprintln!("{config} done in {:.1}s", started.elapsed().as_secs_f64());
                }
            }
        }
    }
}
