//! Index calculus vs Pollard rho on the same hyperelliptic Jacobian, both
//! arms priced by one instrument: deterministic `bitop/v1` primitive counts
//! (`docs/units/BITOPS.md`, `src/cryptanalysis/bitops.rs`), per arm and per
//! phase, instead of the wall-clock conversion of
//! `examples/hyperelliptic_ic_vs_rho.rs` (whose committed outputs this
//! example leaves untouched; the in-situ `S` values are reported beside the
//! counts for reference).
//!
//! The instance rule is the thread's (`thread_instance`), the index-calculus
//! arms and seeds are those of `hyperelliptic_ic_vs_rho`, and the rho
//! reference is the round-twelve default, the full-width distinguished-point
//! walk (`RhoVariant::default()`), unless `--rho-variant` says otherwise.
//!
//! Counts are thread-local and every counted path runs on the calling
//! thread, so they are identical across runs with the same seeds whatever
//! `RAYON_NUM_THREADS` is.
//!
//! ```sh
//! cargo run --release --example hyperelliptic_ic_vs_rho_bitops -- \
//!     --genus 2 --p 41 --arms fb-walk --ic-trials 2 --rho-trials 4 \
//!     --json /tmp/rows.jsonl
//! ```
//!
//! `--json` writes one JSON object per row (JSON Lines).

use std::io::Write;

use num_bigint::BigUint;
use serde_json::{json, Map, Value};

use crypto_lib::cryptanalysis::bitops::{Widths, UNIT};
use crypto_lib::cryptanalysis::hyperelliptic_ic_bench::{
    head_to_head, thread_instance, HeadToHeadRow, HeadToHeadTrials, RhoVariant,
};
use crypto_lib::cryptanalysis::hyperelliptic_index_calculus::{
    HecIndexCalculusParams, LinearAlgebra, RelationSearch, SmoothnessTest,
};
use crypto_lib::prime_hyperelliptic::ops::{Kind, OpTally, Phase, PhaseTally};

const IC_SEED: u64 = 20260916;
const RHO_SEED: u64 = 0xC0FFEE;
const RHO_MAX_STEPS: usize = 200_000_000;

fn arm(label: &str) -> Option<(RelationSearch, LinearAlgebra, SmoothnessTest)> {
    Some(match label {
        "baseline" => (
            RelationSearch::Random,
            LinearAlgebra::Dense,
            SmoothnessTest::Scan,
        ),
        "ab-walk" => (
            RelationSearch::walk(),
            LinearAlgebra::Sparse,
            SmoothnessTest::Gcd,
        ),
        "fb-walk" => (
            RelationSearch::factor_base_walk(),
            LinearAlgebra::Sparse,
            SmoothnessTest::Gcd,
        ),
        _ => return None,
    })
}

fn rho_variant(name: &str) -> Option<RhoVariant> {
    Some(match name {
        "full-width" => RhoVariant::DistinguishedPointsFullWidth,
        "dp" => RhoVariant::DistinguishedPoints,
        "dp-negation" => RhoVariant::DistinguishedPointsNegation,
        "dp-mixed" => RhoVariant::DistinguishedPointsMixed,
        "dp-negation-mixed" => RhoVariant::DistinguishedPointsNegationMixed,
        "floyd" => RhoVariant::Floyd,
        _ => return None,
    })
}

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

fn phases_json(p: &PhaseTally, w: &Widths) -> Value {
    let mut out = Map::new();
    for ph in Phase::ALL {
        out.insert(ph.name().to_string(), tally_json(p.phase(ph), w));
    }
    out.insert("total".to_string(), tally_json(&p.total(), w));
    Value::Object(out)
}

fn arm_json(
    phases: &PhaseTally,
    validation: &PhaseTally,
    per_run: &[u128],
    algorithm_per_run: &[u128],
    runs: usize,
    seeds: Vec<u64>,
    correct: bool,
    w: &Widths,
) -> Value {
    let mean = per_run.iter().map(|&x| x as f64).sum::<f64>() / per_run.len().max(1) as f64;
    json!({
        "runs": runs,
        "seeds": seeds,
        "verified": correct,
        "phases_summed_over_runs": phases_json(phases, w),
        "validation_subset_summed_over_runs": phases_json(validation, w),
        "bitops_per_run": per_run.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
        "bitops_mean": mean,
        // Without the checked arithmetic's operand re-validation: the cost
        // the self-check policy compares (docs/units/BITOPS.md).
        "algorithm_bitops_per_run": algorithm_per_run.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
        "algorithm_bitops_mean": algorithm_per_run.iter().map(|&x| x as f64).sum::<f64>()
            / algorithm_per_run.len().max(1) as f64,
    })
}

#[allow(clippy::too_many_arguments)]
fn row_json(
    row: &HeadToHeadRow,
    c: u64,
    label: &str,
    params: &HecIndexCalculusParams,
    k: &BigUint,
) -> Value {
    let w = &row.bitop_widths;
    let ic_seeds: Vec<u64> = (0..row.ic_runs)
        .map(|t| params.seed.wrapping_add(t as u64))
        .collect();
    let rho_seeds: Vec<u64> = (0..row.rho_runs)
        .map(|t| RHO_SEED.wrapping_add(t as u64))
        .collect();
    let ic_mean = row.ic_bitops_mean();
    let rho_mean = row.rho_bitops_mean();
    json!({
        "schema": "hec_ic_vs_rho_bitops/v1",
        "unit": UNIT,
        "p": row.p,
        "genus": row.genus,
        "c": c,
        "n": row.n.to_string(),
        "k": k.to_string(),
        "factor_base_size": row.factor_base_size,
        "widths": {"field_bits": w.element_bits(), "r_bits": w.r_bits},
        "ic_arm": label,
        "ic_params": {
            "search": format!("{:?}", params.search),
            "linear_algebra": format!("{:?}", params.linear_algebra),
            "smoothness": format!("{:?}", params.smoothness),
            "extra_relations": params.extra_relations,
            "large_primes": params.large_primes,
            "max_trials": params.max_trials,
        },
        "rho_variant": format!("{:?}", row.rho_variant),
        "rho_max_steps": RHO_MAX_STEPS,
        "ic": arm_json(
            &row.ic_bitop_phases,
            &row.ic_bitop_validation,
            &row.ic_bitops_per_run,
            &row.ic_algorithm_bitops_per_run,
            row.ic_runs,
            ic_seeds,
            row.ic_correct,
            w,
        ),
        "rho": arm_json(
            &row.rho_bitop_phases,
            &row.rho_bitop_validation,
            &row.rho_bitops_per_run,
            &row.rho_algorithm_bitops_per_run,
            row.rho_runs,
            rho_seeds,
            row.rho_correct,
            w,
        ),
        "ic_over_rho_bitops": ic_mean / rho_mean.max(f64::MIN_POSITIVE),
        "in_situ_reference": {
            "unit": "group operations / sqrt(N), oracle and solve converted by in-situ wall clock",
            "ic_s": row.ic_s,
            "rho_s": row.rho_s,
            "rho_walk_s": row.rho_walk_s,
            "s_ic_over_s_rho": row.ratio_to_reference(),
            "ic_total_group_ops": row.ic_total_group_ops,
            "rho_group_ops": row.rho_group_ops,
            "modmuls_per_group_op": row.modmuls_per_group_op,
            "ic_wall_ms": row.ic_wall_ms,
            "rho_wall_ms": row.rho_wall_ms,
        },
    })
}

fn usage() -> ! {
    eprintln!(
        "usage: hyperelliptic_ic_vs_rho_bitops [--genus G] [--p P[,P..]] \
         [--arms baseline,ab-walk,fb-walk] [--ic-trials N] [--rho-trials N] \
         [--rho-variant full-width|dp|dp-negation|dp-mixed|dp-negation-mixed|floyd] \
         [--json PATH]"
    );
    std::process::exit(2)
}

fn main() {
    let mut genus = 2u32;
    let mut primes: Vec<u64> = vec![41];
    let mut arms: Vec<String> = vec!["fb-walk".into()];
    let mut trials = HeadToHeadTrials::default();
    let mut variant = RhoVariant::default();
    let mut json_path: Option<String> = None;

    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut val = || args.next().unwrap_or_else(|| usage());
        match a.as_str() {
            "--genus" => genus = val().parse().unwrap_or_else(|_| usage()),
            "--p" => {
                primes = val()
                    .split(',')
                    .map(|s| s.parse().unwrap_or_else(|_| usage()))
                    .collect()
            }
            "--arms" => arms = val().split(',').map(str::to_string).collect(),
            "--ic-trials" => trials.ic = val().parse().unwrap_or_else(|_| usage()),
            "--rho-trials" => trials.rho = val().parse().unwrap_or_else(|_| usage()),
            "--rho-variant" => variant = rho_variant(&val()).unwrap_or_else(|| usage()),
            "--json" => json_path = Some(val()),
            _ => usage(),
        }
    }

    let mut out = json_path.as_ref().map(|path| {
        std::io::BufWriter::new(std::fs::File::create(path).expect("create --json file"))
    });

    println!(
        "unit {UNIT}; rho reference {variant:?}; {} IC runs, {} rho runs per row",
        trials.ic, trials.rho
    );
    println!(
        "{:>5} {:>2} {:>9} {:>9} {:>5} {:>14} {:>14} {:>9} {:>8} {:>8}",
        "p", "g", "arm", "N", "m", "IC bitops/run", "rho bitops/run", "IC/rho", "S_ic", "S_rho"
    );
    for &p in &primes {
        let Some((curve, d1, n, c)) = thread_instance(p, genus) else {
            println!("{p:>5} skipped: no usable prime-order subgroup");
            continue;
        };
        let k = &n / BigUint::from(3u32) + BigUint::from(7u32);
        let d2 = d1.scalar_mul(&k, &curve);
        for label in &arms {
            let (search, linear_algebra, smoothness) = arm(label).unwrap_or_else(|| usage());
            let params = HecIndexCalculusParams {
                fb_size: usize::MAX,
                extra_relations: 8,
                max_trials: 5_000_000,
                seed: IC_SEED,
                search,
                linear_algebra,
                smoothness,
                large_primes: false,
            };
            let row = head_to_head(
                &curve,
                &d1,
                &d2,
                &n,
                &params,
                RHO_SEED,
                RHO_MAX_STEPS,
                &k,
                &trials,
                &variant,
            );
            let mark = if row.ic_correct && row.rho_correct {
                ""
            } else {
                "  [UNVERIFIED - not a result]"
            };
            println!(
                "{:>5} {:>2} {:>9} {:>9} {:>5} {:>14.4e} {:>14.4e} {:>9.3} {:>8.2} {:>8.2}{}",
                p,
                genus,
                label,
                row.n,
                row.factor_base_size,
                row.ic_bitops_mean(),
                row.rho_bitops_mean(),
                row.ic_bitops_mean() / row.rho_bitops_mean().max(f64::MIN_POSITIVE),
                row.ic_s,
                row.rho_s,
                mark
            );
            if let Some(f) = out.as_mut() {
                let v = row_json(&row, c, label, &params, &k);
                writeln!(f, "{}", serde_json::to_string(&v).expect("json")).expect("write");
            }
        }
    }
    if let Some(mut f) = out {
        f.flush().expect("flush");
    }
}
