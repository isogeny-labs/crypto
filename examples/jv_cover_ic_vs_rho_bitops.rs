//! The weak-class cover route end to end (ledger §18.4, `run_end_to_end`)
//! against an executed Pollard rho on the **same challenge curve and
//! seed**, both arms counted in `bitop/v1` primitives
//! (`docs/units/BITOPS.md`), per phase.
//!
//! * `--arm ic`: the whole method from the non-weak challenge curve `C`
//!   (walk, transport, model change and cover set-up, factor base, sieve,
//!   descent, linear algebra, verification on `C`), with the trace cache
//!   cold for every run and every counted path on one thread (a one-thread
//!   rayon pool per run; `--jobs` runs instances side by side, each in its
//!   own pool).  The legacy "`F_p` multiplication" outputs of the run are
//!   recorded beside the counts, unchanged.
//! * `--arm rho`: `jv_rho_strong` (`R0`: no negation, one inversion per
//!   step — the ledger's reference walk; `R3`: negation map and batched
//!   inversion, the strong rho) on `C` itself, with `C`'s `G`, `Q` and the
//!   planted `d`; its native operation counts are priced through the
//!   harness's own `F_{p⁶}` arithmetic, measured by the same hooks
//!   (`jv_rho_strong::rho_tally`), and the recovered scalar is verified.
//!
//! Every attempt is written, solved or not.  One JSON object per line.
//!
//! ```sh
//! JV_SIEVE_ROUND_THREADS=4 cargo run --release --example jv_cover_ic_vs_rho_bitops -- \
//!     --arm ic --p 251 --seeds 1,2 --moves 8 --jobs 2 --json /tmp/ic.jsonl
//! cargo run --release --example jv_cover_ic_vs_rho_bitops -- \
//!     --arm rho --p 251 --seeds 1,2 --moves 8 --rho R3 --runs 0,1 --jobs 4 --json /tmp/rho.jsonl
//! ```

use std::io::Write;
use std::sync::Mutex;
use std::time::Instant;

use serde_json::{json, Map, Value};

use crypto_lib::cryptanalysis::bitops::Widths;
use crypto_lib::cryptanalysis::jv_bitops::{self, JvTally, Phase};
use crypto_lib::cryptanalysis::jv_cover::set_trace_scope;
use crypto_lib::cryptanalysis::jv_isogeny_walk::{challenge_instance, run_end_to_end_with};
use crypto_lib::cryptanalysis::jv_rho_strong::{
    measure_fq3_prims, rho_tally, strong_rho_counted, CurveInstance, StrongOpts,
};
use crypto_lib::prime_hyperelliptic::ops::{Kind, OpTally};

/// §18.4's driver defaults (`examples/jv_isogeny_walk.rs --e2e`).
const JUMPS: [u64; 3] = [3, 5, 7];
const CAP_MULT: u64 = 3;
const MARGIN: f64 = 1.25;
const RHO_S_REF: f64 = 1.3609;

fn tally_json(t: &OpTally, w: &Widths) -> Value {
    let mut counts = Map::new();
    for k in Kind::ALL {
        counts.insert(k.prim().name().to_string(), json!(t.get(k)));
    }
    json!({
        "counts": counts,
        "rng_words": t.rng_words,
        "bitops": t.bitops(w).expect("priced").to_string(),
    })
}

fn phases_json(t: &JvTally, phases: &[Phase], w: &Widths) -> Value {
    let mut out = Map::new();
    for &ph in phases {
        out.insert(ph.name().to_string(), tally_json(t.phase(ph), w));
    }
    Value::Object(out)
}

fn parse_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
    s.split(',')
        .map(|v| v.parse().ok().expect("list value"))
        .collect()
}

fn rho_opts(name: &str, p: u64) -> StrongOpts {
    // the strong-rho study's parameters (PROTOCOL of
    // research/jv_sieve_strong_rho_20261019): r = 256, DP bits by p
    let dp_bits = match p {
        0..=300 => 10,
        301..=600 => 12,
        601..=1100 => 14,
        _ => 15,
    };
    match name {
        "R0" => StrongOpts {
            negation: false,
            batch: 1,
            dp_bits,
            r: 256,
        },
        "R3" => StrongOpts {
            negation: true,
            batch: 16,
            dp_bits,
            r: 256,
        },
        _ => panic!("--rho takes R0 or R3"),
    }
}

/// Run `jobs` closures at a time over `items`, each on its own thread.
fn run_jobs<I: Send + Clone + 'static>(
    items: Vec<I>,
    jobs: usize,
    f: impl Fn(I) -> Value + Send + Sync + 'static,
    sink: &Mutex<Box<dyn Write + Send>>,
) {
    let f = std::sync::Arc::new(f);
    let queue = std::sync::Arc::new(Mutex::new(items.into_iter().collect::<Vec<_>>()));
    std::thread::scope(|s| {
        for _ in 0..jobs.max(1) {
            let f = f.clone();
            let queue = queue.clone();
            s.spawn(move || loop {
                let item = {
                    let mut q = queue.lock().unwrap();
                    if q.is_empty() {
                        break;
                    }
                    q.remove(0)
                };
                let row = f(item);
                let mut out = sink.lock().unwrap();
                writeln!(out, "{row}").unwrap();
                out.flush().unwrap();
            });
        }
    });
}

fn ic_row(p: u64, seed: u64, moves: usize, stop: Option<usize>) -> Value {
    // a one-thread pool: every nested parallel iterator of the route runs on
    // this pool's thread, which owns the tally
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .expect("pool");
    pool.install(|| {
        let start = Instant::now();
        jv_bitops::claim();
        let spec_l = {
            let _g = jv_bitops::enter(Phase::Instance);
            crypto_lib::cryptanalysis::jv_cover::generate_spec(p, seed).l
        };
        jv_bitops::set_moduli(p, spec_l);
        // a trace scope of this run alone: the F4 trace cache is cold
        set_trace_scope(0x1C_0000_0000 ^ (p << 24) ^ (seed << 8) ^ moves as u64);
        let other0 = jv_bitops::snapshot_other();
        let before = jv_bitops::snapshot();
        let r = run_end_to_end_with(
            p, seed, moves, &JUMPS, CAP_MULT, RHO_S_REF, MARGIN, stop, false,
        );
        let t = jv_bitops::snapshot().since(&before);
        let other1 = jv_bitops::snapshot_other();
        let other_in = |phases: &[Phase]| -> u64 {
            phases
                .iter()
                .map(|&ph| other1[ph as usize] - other0[ph as usize])
                .sum()
        };
        let other = other_in(&Phase::IC);
        let other_excluded = other_in(&[Phase::Instance, Phase::Harness]);
        let w = jv_bitops::widths(p, r.l.max(spec_l));
        let ic = t.sum(&Phase::IC);
        json!({
            "arm": "ic",
            "p": p,
            "seed": seed,
            "moves": moves,
            "l": r.l,
            "bits": r.bits,
            "stage": r.stage,
            "solved": r.route_solved,
            "correct": r.route_correct,
            "verified_on_challenge": r.verified_on_challenge,
            "challenge_non_weak": r.challenge_non_weak,
            "path_steps": r.path_steps,
            "widths": {"field_bits": jv_bitops::widths(p, r.l).element_bits(), "r_bits": w.r_bits},
            "phases": phases_json(&t, &Phase::IC, &w),
            "total": tally_json(&ic, &w),
            "total_bitops": ic.bitops(&w).expect("priced").to_string(),
            "excluded": phases_json(&t, &[Phase::Instance, Phase::Harness, Phase::Unattributed], &w),
            "other_modulus_ops": other,
            "other_modulus_ops_excluded_phases": other_excluded,
            "foreign_hook_calls": jv_bitops::foreign(),
            "legacy": {
                "walk_muls": r.walk_muls,
                "transport_muls": r.transport_muls,
                "model_muls": r.model_muls,
                "route_muls": r.route_muls,
                "total_muls": r.total_muls,
                "c_add_e": r.c_add_e,
                "s": r.s,
                "s_over_pooled_rho": r.s_over_rho,
                "pooled_rho_s": RHO_S_REF,
            },
            "round_threads": std::env::var("JV_SIEVE_ROUND_THREADS").ok(),
            "lin_mode": crypto_lib::cryptanalysis::jv_cover::lin_mode_name(),
            "wall_ms": start.elapsed().as_secs_f64() * 1e3,
        })
    })
}

fn rho_row(p: u64, seed: u64, moves: usize, arm: &str, run: u64) -> Value {
    let start = Instant::now();
    jv_bitops::claim();
    let ch = {
        let _g = jv_bitops::enter(Phase::Instance);
        challenge_instance(p, seed, moves, &JUMPS).expect("challenge")
    };
    let inst = CurveInstance::from_challenge(&ch);
    let o = rho_opts(arm, p);
    // the rho walk's randomness: (seed, moves) pick the instance, `run` the walk
    let walk_seed = seed * 1000 + moves as u64;
    let cr = strong_rho_counted(&inst, walk_seed, run, o);
    let prices = measure_fq3_prims(p, inst.l);
    let t = rho_tally(&cr, &prices);
    let w = jv_bitops::widths(p, inst.l);
    let total = t.sum(&Phase::RHO);
    let mut price_json = Map::new();
    for (name, v) in [
        ("mul", &prices.mul),
        ("sq", &prices.sq),
        ("inv", &prices.inv),
    ] {
        price_json.insert(name.into(), tally_json(v, &w));
    }
    json!({
        "arm": "rho",
        "variant": arm,
        "p": p,
        "seed": seed,
        "moves": moves,
        "run": run,
        "walk_seed": walk_seed,
        "l": inst.l,
        "opts": o,
        "steps": cr.run.steps,
        "steps_over_sqrt_l": cr.run.steps as f64 / (inst.l as f64).sqrt(),
        "correct": cr.run.correct,
        "verified_on_challenge": cr.verified,
        "capped": cr.run.capped,
        "restarts": cr.run.restarts,
        "cycles_found": cr.run.cycles_found,
        "dps": cr.run.dps,
        "ops": cr.run.ops,
        "setup_ops": cr.setup,
        "verify_ops": cr.verify,
        "flips": cr.flips,
        "rng_draws": cr.rng_draws,
        "collision_solves": cr.collision_solves,
        "fq3_prices": price_json,
        "phases": phases_json(&t, &Phase::RHO, &w),
        "total": tally_json(&total, &w),
        "total_bitops": total.bitops(&w).expect("priced").to_string(),
        "legacy": {
            "s_harness": cr.run.s_harness,
            "s_plus_harness": cr.run.s_plus_harness,
            "s_tuned": cr.run.s_tuned,
            "muls_harness": cr.run.muls_harness,
            "muls_tuned": cr.run.muls_tuned,
        },
        "wall_ms": start.elapsed().as_secs_f64() * 1e3,
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut arm = String::new();
    let mut p = 0u64;
    let mut seeds: Vec<u64> = vec![1, 2, 3, 4];
    let mut moves: Vec<usize> = vec![8];
    let mut rho = String::from("R3");
    let mut runs: Vec<u64> = vec![0];
    let mut jobs = 1usize;
    let mut json_path: Option<String> = None;
    let mut stop: Option<usize> = None;
    let mut i = 1;
    while i < args.len() {
        let v = args.get(i + 1).cloned().unwrap_or_default();
        match args[i].as_str() {
            "--arm" => arm = v,
            "--p" => p = v.parse().expect("--p"),
            "--seeds" => seeds = parse_list(&v),
            "--moves" => moves = parse_list(&v),
            "--rho" => rho = v,
            "--runs" => runs = parse_list(&v),
            "--jobs" => jobs = v.parse().expect("--jobs"),
            "--json" => json_path = Some(v),
            "--stop" => stop = Some(v.parse().expect("--stop")),
            other => panic!("unknown argument {other}"),
        }
        i += 2;
    }
    assert!(p > 3, "--p");
    let sink: Box<dyn Write + Send> = match &json_path {
        Some(path) => Box::new(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .expect("open --json"),
        ),
        None => Box::new(std::io::stdout()),
    };
    let sink = Mutex::new(sink);
    // counting on for the whole process; each worker thread claims its own
    // tally before it computes anything
    jv_bitops::enable(true);
    match arm.as_str() {
        "ic" => {
            let items: Vec<(u64, usize)> = seeds
                .iter()
                .flat_map(|&s| moves.iter().map(move |&m| (s, m)))
                .collect();
            run_jobs(items, jobs, move |(s, m)| ic_row(p, s, m, stop), &sink);
        }
        "rho" => {
            let items: Vec<(u64, usize, u64)> = seeds
                .iter()
                .flat_map(|&s| {
                    let runs = runs.clone();
                    moves
                        .iter()
                        .flat_map(move |&m| runs.clone().into_iter().map(move |r| (s, m, r)))
                })
                .collect();
            run_jobs(
                items,
                jobs,
                move |(s, m, r)| rho_row(p, s, m, &rho, r),
                &sink,
            );
        }
        _ => panic!("--arm takes ic or rho"),
    }
}
