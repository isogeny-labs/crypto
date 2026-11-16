//! Re-derive the rho arm of the hec bitop study (`research/hec_bitop_20261020`)
//! under any `RhoVariant`, from the committed rows and seeds, and divide the
//! committed index-calculus means by it.
//!
//! The study priced both arms in `bitop/v1` with the full-width walk
//! (`DistinguishedPointsFullWidth`).  The default reference is now the
//! collide-first folded walk, whose recent-hash cycle window was not charged
//! until `research/rho_cycle_window_bitops_20261107`.  This example runs only
//! the rho arm (counts are deterministic functions of the seeds; no wall time
//! is read), on the same instances (`thread_instance`, `k = N/3 + 7`) and the
//! same 40 seeds from `0xC0FFEE`.  Charging the window changes no step of the
//! walk; each run reports what the window adds (`RhoResult::window_charges`),
//! so the total before it was charged is the total minus those primitives,
//! priced at the row's widths, and both ratios come from one run.
//!
//! Control: with `--variant full-width` the rho mean must equal the
//! committed `rho.bitops_mean` exactly; the example prints `control` per row.
//!
//! ```sh
//! cargo run --release --example hec_rho_window_reprice -- \
//!     --raw research/hec_bitop_20261020/raw --variant collide-first \
//!     --json /tmp/rows.jsonl
//! ```

use std::io::Write;

use num_bigint::BigUint;
use serde_json::{json, Map, Value};

use crypto_lib::cryptanalysis::bitops::{Prim, UNIT};
use crypto_lib::cryptanalysis::hyperelliptic_ic_bench::{
    bitop_widths, pollard_rho_jacobian_with, thread_instance, RhoVariant,
};
use crypto_lib::prime_hyperelliptic::ops::{self, Kind, Phase, PhaseTally};

const RHO_SEED: u64 = 0xC0FFEE;
const RHO_MAX_STEPS: usize = 200_000_000;

fn variant(name: &str) -> Option<RhoVariant> {
    Some(match name {
        "full-width" => RhoVariant::DistinguishedPointsFullWidth,
        "negation-full-width" => RhoVariant::DistinguishedPointsNegationFullWidth,
        "cycle-escape" => RhoVariant::DistinguishedPointsNegationFullWidthCycleEscape,
        "collide-first" => RhoVariant::DistinguishedPointsNegationFullWidthCollideFirst,
        _ => return None,
    })
}

fn usage() -> ! {
    eprintln!(
        "usage: hec_rho_window_reprice --raw DIR [--variant full-width|negation-full-width|\
         cycle-escape|collide-first] [--runs N] [--json PATH]"
    );
    std::process::exit(2)
}

fn main() {
    let mut raw: Option<String> = None;
    let mut name = "collide-first".to_string();
    let mut runs = 40u64;
    let mut json_path: Option<String> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut val = || args.next().unwrap_or_else(|| usage());
        match a.as_str() {
            "--raw" => raw = Some(val()),
            "--variant" => name = val(),
            "--runs" => runs = val().parse().unwrap_or_else(|_| usage()),
            "--json" => json_path = Some(val()),
            _ => usage(),
        }
    }
    let raw = raw.unwrap_or_else(|| usage());
    let v = variant(&name).unwrap_or_else(|| usage());
    let mut out = json_path
        .map(|p| std::io::BufWriter::new(std::fs::File::create(p).expect("create --json file")));
    println!("unit {UNIT}; rho {v:?}; {runs} runs per row from seed {RHO_SEED:#x}");
    println!(
        "{:>2} {:>4} {:>9} {:>14} {:>14} {:>8} {:>8} {:>8} {:>9}",
        "g", "p", "N", "rho bitops/run", "committed rho", "IC/rho", "uncharg", "commit", "control"
    );
    for genus in [2u32, 3, 4] {
        let path = format!("{raw}/g{genus}.jsonl");
        let text = std::fs::read_to_string(&path).expect("read committed rows");
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let row: Value = serde_json::from_str(line).expect("row json");
            let p = row["p"].as_u64().expect("p");
            let n_committed = row["n"].as_str().expect("n").to_string();
            let k_committed = row["k"].as_str().expect("k").to_string();
            let ic_mean = row["ic"]["bitops_mean"].as_f64().expect("ic mean");
            let rho_committed = row["rho"]["bitops_mean"].as_f64().expect("rho mean");
            let committed_ratio = row["ic_over_rho_bitops"].as_f64().expect("ratio");
            let (curve, d1, n, _c) = thread_instance(p, genus).expect("thread instance");
            assert_eq!(n.to_string(), n_committed, "g{genus} p{p}: N differs");
            let k = &n / BigUint::from(3u32) + BigUint::from(7u32);
            assert_eq!(k.to_string(), k_committed, "g{genus} p{p}: k differs");
            let d2 = d1.scalar_mul(&k, &curve);
            let widths = bitop_widths(&curve, &n);
            let mut phases = PhaseTally::default();
            let mut validation = PhaseTally::default();
            let (mut per_run, mut algo) = (Vec::new(), Vec::new());
            let (mut group_ops, mut walk_ops, mut fruitless, mut cycle) = (0u64, 0u64, 0u64, 0u64);
            let mut verified = true;
            // The window's charges, priced: subtracting them gives the total the
            // uncharged window reported (the walk itself is unchanged).
            let (mut window_bitops, mut window_word_ops) = (0u128, 0u64);
            let word = widths.gates(Prim::WordOp).expect("word_op");
            let modr_add = widths.gates(Prim::ModrAdd).expect("modr_add");
            let mut before_per_run = Vec::new();
            for t in 0..runs {
                let (t0, v0) = (ops::snapshot(), ops::snapshot_validation());
                let r = pollard_rho_jacobian_with(
                    &curve,
                    &d1,
                    &d2,
                    &n,
                    RHO_SEED.wrapping_add(t),
                    RHO_MAX_STEPS,
                    &v,
                );
                let run = ops::snapshot().since(&t0);
                let checks = ops::snapshot_validation().since(&v0);
                let all = run.total().bitops(&widths).expect("priced");
                per_run.push(all);
                algo.push(all - checks.total().bitops(&widths).expect("priced"));
                phases.add(&run);
                validation.add(&checks);
                group_ops += r.group_ops as u64;
                walk_ops += r.walk_ops as u64;
                fruitless += r.fruitless_ops as u64;
                cycle += r.cycle_ops as u64;
                verified &= r.k.as_ref() == Some(&k);
                let w = r.window_charges;
                let words = w.word_ops + w.rng_words + w.escape_index_ops;
                let charged = u128::from(words) * word + u128::from(w.modr_adds) * modr_add;
                window_bitops += charged;
                window_word_ops += words;
                before_per_run.push(all - charged);
            }
            let mean = per_run.iter().map(|&x| x as f64).sum::<f64>() / runs as f64;
            let algo_mean = algo.iter().map(|&x| x as f64).sum::<f64>() / runs as f64;
            let before_mean = before_per_run.iter().map(|&x| x as f64).sum::<f64>() / runs as f64;
            let before_ratio = ic_mean / before_mean;
            let ratio = ic_mean / mean;
            let control = if mean == rho_committed {
                "exact"
            } else {
                "differs"
            };
            println!(
                "{:>2} {:>4} {:>9} {:>14.6e} {:>14.6e} {:>8.4} {:>8.4} {:>8.4} {:>9}{}",
                genus,
                p,
                n,
                mean,
                rho_committed,
                ratio,
                before_ratio,
                committed_ratio,
                control,
                if verified { "" } else { "  [UNVERIFIED]" }
            );
            let mut ph = Map::new();
            for phase in Phase::ALL {
                let t = phases.phase(phase);
                let mut counts = Map::new();
                for kind in Kind::ALL {
                    counts.insert(kind.prim().name().to_string(), json!(t.get(kind)));
                }
                ph.insert(
                    phase.name().to_string(),
                    json!({
                        "counts": counts,
                        "rng_words": t.rng_words,
                        "bitops": t.bitops(&widths).expect("priced").to_string(),
                    }),
                );
            }
            if let Some(f) = out.as_mut() {
                let v = json!({
                    "schema": "hec_rho_window_reprice/v1",
                    "unit": UNIT,
                    "genus": genus,
                    "p": p,
                    "n": n.to_string(),
                    "k": k.to_string(),
                    "rho_variant": format!("{v:?}"),
                    "runs": runs,
                    "seeds_from": RHO_SEED,
                    "verified": verified,
                    "rho_bitops_per_run": per_run.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                    "rho_bitops_mean": mean,
                    "rho_algorithm_bitops_mean": algo_mean,
                    "rho_phases_summed_over_runs": ph,
                    "rho_validation_bitops_summed": validation.total().bitops(&widths).expect("priced").to_string(),
                    "group_ops_summed": group_ops,
                    "walk_ops_summed": walk_ops,
                    "fruitless_ops_summed": fruitless,
                    "cycle_ops_summed": cycle,
                    "committed_ic_bitops_mean": ic_mean,
                    "committed_rho_bitops_mean": rho_committed,
                    "committed_rho_variant": row["rho_variant"],
                    "committed_ic_over_rho": committed_ratio,
                    "ic_over_rho": ratio,
                    "window_bitops_summed": window_bitops.to_string(),
                    "window_word_ops_summed": window_word_ops,
                    "rho_bitops_mean_window_uncharged": before_mean,
                    "rho_bitops_per_run_window_uncharged": before_per_run.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                    "ic_over_rho_window_uncharged": before_ratio,
                    "control_rho_equals_committed": mean == rho_committed,
                });
                writeln!(f, "{}", serde_json::to_string(&v).expect("json")).expect("write");
            }
        }
    }
    if let Some(mut f) = out {
        f.flush().expect("flush");
    }
}
