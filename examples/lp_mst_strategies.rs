//! Run the exhaustive, overlap-sensitive and shared-key MST large-prime filter on one
//! frozen relation stream (JSONL of `PartialRelation`) with identical options,
//! check that the outputs are identical and replay from the input, and print
//! both arms' counters plus interleaved wall-clock samples as JSON.
//!
//! Usage: lp_mst_strategies --input FILE --modulus P [--label L] [--reps N]
//!        [--time-only ARM]  (time one arm only, e.g. under callgrind)
use crypto_lib::cryptanalysis::large_prime_filter::{
    filter_large_primes_mst_with, replay_complete_relation, LargePrimeFilterOptions, MstLimits,
    MstStrategy, PartialRelation,
};
use std::collections::BTreeMap;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: BTreeMap<String, String> = BTreeMap::new();
    let mut it = std::env::args().skip(1);
    while let Some(k) = it.next() {
        let v = it.next().ok_or(format!("missing value for {k}"))?;
        args.insert(k, v);
    }
    let input = args.get("--input").ok_or("--input required")?;
    let modulus: u64 = args.get("--modulus").ok_or("--modulus required")?.parse()?;
    let label = args.get("--label").cloned().unwrap_or_default();
    let reps: usize = args.get("--reps").map_or(Ok(31), |s| s.parse())?;
    let time_only = args.get("--time-only").cloned();
    let text = std::fs::read_to_string(input)?;
    let relations: Vec<PartialRelation> = text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    // Same defaults as `lp_graph_eval`'s MST arm (EvalParams::default()).
    let options = LargePrimeFilterOptions {
        modulus,
        ..LargePrimeFilterOptions::default()
    };
    let limits = MstLimits::default();
    let run = |s| {
        filter_large_primes_mst_with(relations.clone(), &options, &limits, s)
            .map_err(|e| format!("{e:?}"))
    };
    let strategies = [
        ("exhaustive", MstStrategy::Exhaustive),
        ("overlap_sensitive", MstStrategy::OverlapSensitive),
        ("shared_key", MstStrategy::SharedKey),
    ];
    let attempts = strategies
        .iter()
        .map(|&(_, s)| run(s))
        .collect::<Result<Vec<_>, _>>()?;
    let exhaustive = &attempts[0];
    let mut all_ok = true;
    let mut arms = serde_json::Map::new();
    // Interleaved timing (A B C A B C ...); the input clone inside each
    // sample is common to every arm. Informational only.
    let mut ns = vec![Vec::with_capacity(reps); strategies.len()];
    for _ in 0..reps {
        for (i, &(name, s)) in strategies.iter().enumerate() {
            if time_only.as_deref().is_some_and(|o| o != name) {
                ns[i].push(0);
                continue;
            }
            let t = Instant::now();
            let a = run(s)?;
            ns[i].push(t.elapsed().as_nanos() as u64);
            std::hint::black_box(a);
        }
    }
    for v in &mut ns {
        v.sort_unstable();
    }
    for (i, (name, _)) in strategies.iter().enumerate() {
        let a = &attempts[i];
        let mut legacy = a.stats.clone();
        legacy.explicit_edge_evaluations = 0;
        legacy.implicit_selected_edges = 0;
        legacy.overlapping_pairs = 0;
        legacy.exhaustive_fallback_trees = 0;
        legacy.fallback_edge_evaluations = 0;
        legacy.implicit_key_mismatches = 0;
        legacy.shared_key_pair_updates = 0;
        let out = a.relations.as_deref().unwrap_or(&[]);
        let replay_failures = out
            .iter()
            .filter(|r| replay_complete_relation(r, &relations, modulus).as_ref() != Some(*r))
            .count();
        let identical = a.relations == exhaustive.relations
            && a.report == exhaustive.report
            && legacy == exhaustive.stats;
        all_ok &= identical && replay_failures == 0 && a.stats.implicit_key_mismatches == 0;
        let s = &a.stats;
        let formed = if i == 0 {
            s.edge_evaluations
        } else {
            s.explicit_edge_evaluations + s.implicit_selected_edges + s.fallback_edge_evaluations
        };
        arms.insert(
            name.to_string(),
            serde_json::json!({
                "identical_to_exhaustive": identical,
                "refused": a.relations.is_none(),
                "complete_relations": out.len(),
                "replay_failures": replay_failures,
                "differences_formed": formed,
                "stats": a.stats,
                "wall_ns_median": ns[i][reps / 2],
                "wall_ns_min": ns[i][0],
            }),
        );
    }
    let report = serde_json::json!({
        "schema": "lp-mst-strategies/v2",
        "label": label,
        "modulus": modulus,
        "rows": relations.len(),
        "report": exhaustive.report,
        "arms": arms,
        "reps": reps,
    });
    println!("{}", serde_json::to_string_pretty(&report)?);
    if !all_ok {
        return Err("strategies disagree or replay failed".into());
    }
    Ok(())
}
