//! Run the exhaustive and the overlap-sensitive MST large-prime filter on one
//! frozen relation stream (JSONL of `PartialRelation`) with identical options,
//! check that the outputs are identical and replay from the input, and print
//! both arms' counters plus interleaved wall-clock samples as JSON.
//!
//! Usage: lp_mst_strategies --input FILE --modulus P [--label L] [--reps N]
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
    let exhaustive = run(MstStrategy::Exhaustive)?;
    let overlap = run(MstStrategy::OverlapSensitive)?;
    let identical_relations = exhaustive.relations == overlap.relations;
    let identical_report = exhaustive.report == overlap.report;
    let mut legacy = overlap.stats.clone();
    legacy.explicit_edge_evaluations = 0;
    legacy.implicit_selected_edges = 0;
    legacy.overlapping_pairs = 0;
    legacy.exhaustive_fallback_trees = 0;
    legacy.fallback_edge_evaluations = 0;
    legacy.implicit_key_mismatches = 0;
    let identical_legacy_stats = legacy == exhaustive.stats;
    let mut replay_failures = 0usize;
    let out = overlap.relations.as_deref().unwrap_or(&[]);
    for r in out {
        if replay_complete_relation(r, &relations, modulus).as_ref() != Some(r) {
            replay_failures += 1;
        }
    }
    // Interleaved timing (A B A B ...), the clone of the input inside each
    // sample is common to both arms. Informational only.
    let mut ns = [Vec::with_capacity(reps), Vec::with_capacity(reps)];
    for _ in 0..reps {
        for (i, s) in [MstStrategy::Exhaustive, MstStrategy::OverlapSensitive]
            .into_iter()
            .enumerate()
        {
            let t = Instant::now();
            let a = run(s)?;
            ns[i].push(t.elapsed().as_nanos() as u64);
            std::hint::black_box(a);
        }
    }
    for v in &mut ns {
        v.sort_unstable();
    }
    let s = &overlap.stats;
    let formed =
        s.explicit_edge_evaluations + s.implicit_selected_edges + s.fallback_edge_evaluations;
    let report = serde_json::json!({
        "schema": "lp-mst-strategies/v1",
        "label": label,
        "modulus": modulus,
        "rows": relations.len(),
        "identical_relations": identical_relations,
        "identical_report": identical_report,
        "identical_legacy_stats": identical_legacy_stats,
        "refused": overlap.relations.is_none(),
        "complete_relations": out.len(),
        "replay_failures": replay_failures,
        "report": overlap.report,
        "exhaustive_stats": exhaustive.stats,
        "overlap_sensitive_stats": overlap.stats,
        "differences_formed": {
            "exhaustive": exhaustive.stats.edge_evaluations,
            "overlap_sensitive": formed,
        },
        "wall_ns_median": { "exhaustive": ns[0][reps / 2], "overlap_sensitive": ns[1][reps / 2] },
        "wall_ns_min": { "exhaustive": ns[0][0], "overlap_sensitive": ns[1][0] },
        "reps": reps,
    });
    println!("{}", serde_json::to_string_pretty(&report)?);
    if !(identical_relations && identical_report && identical_legacy_stats && replay_failures == 0)
    {
        return Err("strategies disagree or replay failed".into());
    }
    Ok(())
}
