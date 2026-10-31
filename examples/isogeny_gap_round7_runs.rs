//! # Round-7 runs for research/isogeny_conductor_gap_round7_20261019 (E10, E11a)
//!
//! - `--mode e10`: the 6,000 E8 systems of round 6 (same curves, bases,
//!   dimensions and targets, regenerated from the same seed), each solved
//!   by `f4-f2-early` at free ∈ {0, 2, 4}.  Full-F4 figures come from the
//!   E8 records.
//! - `--mode e11a`: random binary curves (n ∈ {19, 23}, seeds 1–4,
//!   cofactor ≤ 4).  For d ∈ {9, 10} it compares the default base at d with
//!   the "half base" (trace coset at d − 1).  Each system is solved by full
//!   `f4-f2` and its lifted decompositions are counted.
//!
//! One JSON line per system and one `config_done` line per configuration.
//! Resumable at configuration level.
//!
//! ```bash
//! cargo run --release --example isogeny_gap_round7_runs -- --mode e10 \
//!   --out research/isogeny_conductor_gap_round7_20261019/e10/records.jsonl
//! cargo run --release --example isogeny_gap_round7_runs -- --mode e11a \
//!   --out research/isogeny_conductor_gap_round7_20261019/e11a/records.jsonl
//! ```

use std::collections::BTreeSet;
use std::io::Write;
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    binary_affine_subspace_factor_base, koblitz_instance, lift_abscissae, random_binary_instance,
    BinaryGroup, BinaryInstance, CountedGroup, GroupOps,
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

/// E8's frozen target count and seed.
const TARGETS: usize = 200;
const SEED: u64 = 20261018;

fn arg(args: &[String], name: &str) -> String {
    let i = args
        .iter()
        .position(|a| a == name)
        .unwrap_or_else(|| panic!("missing {name}"));
    args[i + 1].clone()
}

/// E8's targets for a curve: the same generator stream.
fn targets(
    inst: &BinaryInstance,
    n: u32,
    a: u64,
) -> Vec<crypto_lib::cryptanalysis::koblitz_fast::FastPoint> {
    let g = BinaryGroup(&inst.fast);
    let mut rng = StdRng::seed_from_u64(SEED ^ u64::from(n) ^ (a << 8));
    let mut gops = GroupOps::default();
    (0..TARGETS)
        .map(|_| {
            let k = rng.gen_range(1..inst.r);
            g.mul(&mut gops, inst.generator, k)
        })
        .collect()
}

fn base_of(inst: &BinaryInstance, kind: &str, d: u32) -> (Vec<u64>, u64) {
    match kind {
        "default" => ((0..d).map(|k| 1u64 << k).collect(), 0),
        "coset" => trace_coset_basis(inst, d).expect("ker Tr is large enough"),
        "half" => trace_coset_basis(inst, d - 1).expect("ker Tr is large enough"),
        _ => unreachable!(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = arg(&args, "--mode");
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
    // (instance, configurations as (d, base kind)).
    let mut plan: Vec<(BinaryInstance, Vec<(u32, &str)>)> = Vec::new();
    if mode == "e10" {
        for n in 17..=29u32 {
            for a in 0..=1u8 {
                let Some(inst) = koblitz_instance(a, n) else {
                    continue;
                };
                if inst.cofactor != 2 && inst.cofactor != 4 {
                    continue;
                }
                let cfgs = (8..=10u32)
                    .flat_map(|d| [(d, "default"), (d, "coset")])
                    .collect();
                plan.push((inst, cfgs));
            }
        }
    } else if mode == "e11a" {
        for n in [19u32, 23] {
            for seed in 1..=4u64 {
                let Some(inst) = random_binary_instance(n, seed, 4) else {
                    continue;
                };
                if inst.cofactor != 2 && inst.cofactor != 4 {
                    continue;
                }
                let cfgs = [9u32, 10]
                    .into_iter()
                    .flat_map(|d| [(d, "default"), (d, "half")])
                    .collect();
                plan.push((inst, cfgs));
            }
        }
    } else {
        panic!("--mode e10 | e11a");
    }
    for (inst, cfgs) in &plan {
        let g = BinaryGroup(&inst.fast);
        let mut gops = GroupOps::default();
        let tgts = targets(inst, inst.n, inst.a);
        for &(d, kind) in cfgs {
            let config = format!("{}:d{d}:{kind}", inst.name);
            if done.contains(&config) {
                continue;
            }
            let (basis, offset) = base_of(inst, kind, d);
            let fb = binary_affine_subspace_factor_base(inst, &basis, offset);
            let started = Instant::now();
            for (t, &target) in tgts.iter().enumerate() {
                let bases = vec![basis.clone(), basis.clone()];
                let sys = descend_affine(&inst.gf, inst.b, target.x, &bases, offset)
                    .expect("dimension fits");
                let eqs = &sys.equations;
                let nv = sys.n_vars;
                let rec = if mode == "e10" {
                    let mut runs = Vec::new();
                    for free in [0usize, 2, 4] {
                        let (part, st) = groebner_basis_f4_until(eqs.clone(), nv, None, Some(free));
                        let (mut tests, mut xors) = (0u64, 0u64);
                        let sols =
                            solutions_from_linear_part(&part, eqs, nv, 24, &mut tests, &mut xors)
                                .expect("at most 24 free");
                        runs.push(json!({"free": free, "word_xors": st.word_xors,
                            "extraction_tests": tests, "extraction_xors": xors,
                            "early_stopped": st.early_stopped, "solutions": sols.len(),
                            "wall_ns": st.wall_ns}));
                    }
                    json!({"kind": "system", "config": config, "target": t, "early": runs})
                } else {
                    let (gb, full) = groebner_basis_f4(eqs.clone(), nv, None);
                    let mut tests = 0u64;
                    let mut sols = solutions_from_reduced_basis(&gb, nv, 24, &mut tests)
                        .expect("reduced basis");
                    sols.retain(|&v| {
                        eqs.iter().all(|e| {
                            tests += e.terms.len() as u64;
                            e.eval(v) == 0
                        })
                    });
                    let lifted = sols
                        .iter()
                        .filter(|&&v| {
                            lift_abscissae(&g, &fb, &mut gops, &sys.lift(v), target).is_some()
                        })
                        .count();
                    json!({"kind": "system", "config": config, "target": t,
                        "solutions": sols.len(), "lifted": lifted,
                        "full": {"word_xors": full.word_xors, "extraction_tests": tests,
                                 "solving_degree": full.solving_degree, "wall_ns": full.wall_ns}})
                };
                writeln!(file, "{rec}").unwrap();
            }
            let cfg = json!({"kind": "config_done", "config": config, "curve": inst.name,
                "n": inst.n, "a": inst.a, "cofactor": inst.cofactor, "d": d, "base": kind,
                "basis_dim": basis.len(), "offset": offset, "base_points": fb.points.len(),
                "base_abscissae": fb.abscissae, "targets": TARGETS, "seed": SEED,
                "wall_s": started.elapsed().as_secs_f64()});
            writeln!(file, "{cfg}").unwrap();
            file.flush().unwrap();
            eprintln!("{config} done in {:.1}s", started.elapsed().as_secs_f64());
        }
    }
}
