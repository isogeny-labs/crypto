//! Finite action-coordinate controls on exact fixtures and a fixed saved prefix.
//! Scoped builds link pinned Root41 dependencies while compiling these changed
//! source modules directly. This is not a timing or fresh-target benchmark.
pub use crypto_lib::cryptanalysis;
use crypto_lib::cryptanalysis::{f4_fp, jv_bitops, jv_cover, residual_walk};
#[path = "../src/cryptanalysis/jv_action_operator.rs"]
mod action_operator;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/fx_hash.rs"]
mod fx_hash;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/action-operator-h1-20261123/scoped_poly_ring.rs"]
mod gaudry_cubic;
// The inherited module has three pre-existing diagnostics under Clippy 1.100:
// two chunks_exact suggestions and the BM sequence loop. Preserve its kernel
// implementation; keep the new adapter and driver under strict linting.
#[allow(
    dead_code,
    clippy::chunks_exact_to_as_chunks,
    clippy::needless_range_loop
)]
#[path = "../src/cryptanalysis/zero_dim_la.rs"]
mod zero_dim_la;

use rand::{rngs::StdRng, RngCore, SeedableRng};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path, time::Instant};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const RAW_SHA: &str = "6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d";

fn ensure(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}

fn digest<T: serde::Serialize>(value: &T) -> Result<String> {
    Ok(format!("{:x}", Sha256::digest(serde_json::to_vec(value)?)))
}

fn without_clocks<T: serde::Serialize>(value: &T) -> Result<Value> {
    let mut v = serde_json::to_value(value)?;
    v.as_object_mut()
        .ok_or("stats object")?
        .retain(|k, _| !k.ends_with("_ms"));
    Ok(v)
}

fn fixed_prefix(original_path: &str, output_dir: &str) -> Result<()> {
    ensure(
        !Path::new(output_dir).exists(),
        "exclusive new output directory",
    )?;
    fs::create_dir(output_dir)?;
    let bytes = fs::read(original_path)?;
    ensure(
        format!("{:x}", Sha256::digest(&bytes)) == RAW_SHA,
        "original raw digest",
    )?;
    let raw: Value = serde_json::from_slice(&bytes)?;
    let attempts = raw["work"]["nagao_attempts"]
        .as_array()
        .ok_or("saved attempts")?;
    ensure(attempts.len() == 3415, "original denominator")?;
    let mut cases = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(Path::new(output_dir).join("cases.jsonl"))?;
    let mut bases = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(Path::new(output_dir).join("bases.jsonl"))?;
    let mut dev = [0u128; 6];
    let mut held = [0u128; 6];
    let mut incumbent_held = 0u128;
    let mut selected = 5usize;
    let mut fallbacks = [0usize; 6];
    for (index, attempt) in attempts.iter().take(32).enumerate() {
        // Only the polynomial system is passed to the solver; recorded roots
        // remain in this qualifier and never influence an action-stage call.
        let input: Vec<f4_fp::Poly> = serde_json::from_value(attempt["audit"]["original"].clone())?;
        let mut expected: Vec<Vec<u64>> =
            serde_json::from_value(attempt["audit"]["lifted_roots"].clone())?;
        expected.sort();
        let mut opts = f4_fp::F4Options::new(f4_fp::Ordering::Grevlex, 24);
        opts.deadline = Some(Instant::now() + std::time::Duration::from_secs(30));
        let basis_report = f4_fp::f4(&input, 6, 1009, &opts);
        ensure(
            !basis_report.timed_out
                && basis_report.pairs_above_bound == 0
                && basis_report.staircase_at_stop.is_none()
                && !basis_report.inconsistent,
            "full F4 completion flags",
        )?;
        let basis = &basis_report.basis;
        writeln!(
            bases,
            "{}",
            json!({"index":index,"p":1009,"variables":6,
            "input":input,"basis":basis,"expected_roots":expected,
            "f4_completion_flags":{"timed_out":basis_report.timed_out,
                "pairs_above_bound":basis_report.pairs_above_bound,
                "staircase_at_stop":basis_report.staircase_at_stop}})
        )?;
        bases.flush()?;
        let mut old_rng = StdRng::seed_from_u64(389);
        let mut old_stats = jv_cover::ZeroDimStats::default();
        let old_roots = jv_cover::linear_stage_from_basis(
            basis,
            &input,
            6,
            1009,
            &mut old_rng,
            jv_cover::LinMode::Fast,
            &mut old_stats,
            Instant::now(),
        );
        ensure(
            old_roots == expected,
            "incumbent roots equal saved coordinates",
        )?;
        let old_next_rng = old_rng.next_u64();
        let mut actions = Vec::new();
        for action in 0..6 {
            let mut rng = StdRng::seed_from_u64(389);
            let mut stats = jv_cover::ZeroDimStats::default();
            let result = action_operator::linear_stage_from_basis_with_action(
                basis,
                &input,
                6,
                1009,
                action,
                &mut rng,
                &mut stats,
                Instant::now(),
            )?;
            ensure(
                result.roots == expected,
                "action/fallback roots equal saved coordinates",
            )?;
            ensure(
                !stats.incomplete && !stats.timed_out && stats.nonseparating == 0,
                "qualified final linear-stage flags",
            )?;
            if action == 5 {
                ensure(
                    without_clocks(&stats)? == without_clocks(&old_stats)?,
                    "default stats compatible",
                )?;
                ensure(rng.next_u64() == old_next_rng, "default RNG compatible")?;
            }
            fallbacks[action] += usize::from(result.used_dense_fallback);
            let sums = if index < 16 { &mut dev } else { &mut held };
            sums[action] = sums[action]
                .checked_add(u128::from(stats.lin_muls))
                .ok_or("sum overflow")?;
            actions.push(
                json!({"action":result.action_variable,"roots_sha256":digest(&result.roots)?,
                "dense_fallback":result.used_dense_fallback,"lin_muls_partial":stats.lin_muls,
                "fast":without_clocks(&result.fast_stats)?,"passed":true}),
            );
        }
        if index >= 16 {
            incumbent_held = incumbent_held
                .checked_add(u128::from(old_stats.lin_muls))
                .ok_or("baseline sum overflow")?;
        }
        writeln!(
            cases,
            "{}",
            json!({"index":index,"input_sha256":digest(&input)?,
            "basis_sha256":digest(basis)?,"expected_roots_sha256":digest(&expected)?,
            "fp_roots":expected.len(),"f4_calls":1,"incumbent":without_clocks(&old_stats)?,
            "actions":actions,"passed":true})
        )?;
        cases.flush()?;
        if index == 15 {
            // The fixed rule prefers the incumbent on a tie, then the
            // lowest index. Freeze selection before processing held-out rows.
            selected = (0..6)
                .min_by_key(|&a| (dev[a], a != 5, a))
                .ok_or("action set")?;
            let mut out = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(Path::new(output_dir).join("selection.json"))?;
            writeln!(
                out,
                "{}",
                json!({"development_indices":[0,15],"selected":selected,
                "partial_multiplication_sums":dev.map(|x| x.to_string()),"rule_frozen_before_held_out":true})
            )?;
        }
    }
    let development_search = dev
        .iter()
        .try_fold(0u128, |sum, &value| sum.checked_add(value))
        .ok_or("development search sum overflow")?;
    let selected_with_search = development_search
        .checked_add(held[selected])
        .ok_or("search plus held-out sum overflow")?;
    let summary = json!({"schema":"cover.action-coordinate.fixed-prefix/v1","input_sha256":RAW_SHA,
        "development_indices":[0,15],"held_out_indices":[16,31],"selected":selected,
        "development_partial_muls":dev.map(|x|x.to_string()),"held_out_partial_muls":held.map(|x|x.to_string()),
        "held_out_incumbent_partial_muls":incumbent_held.to_string(),
        "held_out_selected_partial_muls":held[selected].to_string(),"dense_fallback_counts":fallbacks,
        "development_all_candidate_partial_muls":development_search.to_string(),
        "selected_with_development_search_partial_muls":selected_with_search.to_string(),
        "f4_calls":32,"candidate_linear_calls":192,"incumbent_linear_calls":32,
        "passed":true,"new_targets":0,"runtime_measurement":false,"elapsed_values_exported":false,
        "complete_cost":false,"fresh_instance_evidence":false,"signed_verdict":false,
        "scope":"fixed saved prefix; partial multiplication proxy with fallback, not a speed or full-cost comparison"});
    let mut out = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(Path::new(output_dir).join("summary.json"))?;
    writeln!(out, "{}", serde_json::to_string_pretty(&summary)?)?;
    println!("{}", summary);
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    ensure(
        args.len() == 3 && args[0] == "--fixed-prefix",
        "usage: CHECK --fixed-prefix ORIGINAL_UNIT_RAW NEW_OUTPUT_DIR",
    )?;
    fixed_prefix(&args[1], &args[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shape_basis(n: usize, p: u64) -> (Vec<f4_fp::Poly>, Vec<Vec<u64>>) {
        let mut basis = Vec::new();
        for i in 0..n - 1 {
            let mut x = vec![0; n];
            x[i] = 1;
            let mut y = vec![0; n];
            y[n - 1] = 1;
            basis.push(vec![
                (x, 1),
                (y, p - (i as u64 + 1)),
                (vec![0; n], p - (i as u64 + 3)),
            ]);
        }
        let mut cubic = vec![0; n];
        cubic[n - 1] = 3;
        let mut linear = vec![0; n];
        linear[n - 1] = 1;
        basis.push(vec![(cubic, 1), (linear, p - 1)]);
        let mut roots: Vec<_> = [0, 1, p - 1]
            .iter()
            .map(|&t| {
                (0..n)
                    .map(|i| {
                        if i == n - 1 {
                            t
                        } else {
                            ((i as u64 + 1) * t + i as u64 + 3) % p
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        roots.sort();
        (basis, roots)
    }

    #[test]
    fn every_coordinate_reads_out_in_original_order() {
        for n in [2, 3, 6, 8] {
            let (basis, roots) = shape_basis(n, 101);
            for action in 0..n {
                let mut rng = StdRng::seed_from_u64(389);
                let mut stats = zero_dim_la::FastStats::default();
                assert_eq!(
                    zero_dim_la::fast_stage_with_action(
                        &basis, &basis, n, 101, action, &mut rng, &mut stats
                    ),
                    Some(roots.clone()),
                    "n={n} action={action}"
                );
            }
        }
    }

    #[test]
    fn incumbent_default_preserves_outputs_counters_and_rng() {
        let (basis, _) = shape_basis(6, 101);
        let mut old_rng = StdRng::seed_from_u64(389);
        let mut new_rng = old_rng.clone();
        let mut old = crypto_lib::cryptanalysis::zero_dim_la::FastStats::default();
        let mut new = zero_dim_la::FastStats::default();
        let a = crypto_lib::cryptanalysis::zero_dim_la::fast_stage(
            &basis,
            &basis,
            6,
            101,
            &mut old_rng,
            &mut old,
        );
        let b = zero_dim_la::fast_stage(&basis, &basis, 6, 101, &mut new_rng, &mut new);
        assert_eq!(a, b);
        assert_eq!(without_clocks(&old).unwrap(), without_clocks(&new).unwrap());
        assert_eq!(old_rng.next_u64(), new_rng.next_u64());
    }

    #[test]
    fn nonseparating_coordinate_refuses_then_dense_fallback_recovers() {
        let basis = vec![
            vec![(vec![1, 0, 0], 1), (vec![0, 0, 1], 10)],
            vec![(vec![0, 1, 0], 1), (vec![0, 0, 0], 4)],
            vec![(vec![0, 0, 2], 1), (vec![0, 0, 0], 10)],
        ];
        let mut brute = Vec::new();
        for x in 0..11 {
            for y in 0..11 {
                for z in 0..11 {
                    let point = vec![x, y, z];
                    if basis.iter().all(|f| f4_fp::eval(f, &point, 11) == 0) {
                        brute.push(point);
                    }
                }
            }
        }
        assert_eq!(brute, vec![vec![1, 7, 1], vec![10, 7, 10]]);
        let mut rng = StdRng::seed_from_u64(389);
        let mut stats = jv_cover::ZeroDimStats::default();
        let out = action_operator::linear_stage_from_basis_with_action(
            &basis,
            &basis,
            3,
            11,
            1,
            &mut rng,
            &mut stats,
            Instant::now(),
        )
        .unwrap();
        assert_eq!(out.roots, brute);
        assert!(out.used_dense_fallback);
        assert_eq!(
            out.fast_stats.gave_up,
            Some("minimal polynomial short of delta")
        );
        assert_eq!(out.fast_stats.functionals_tried, 4);
        assert_eq!(stats.lin_fallback, 1);
        assert!(stats.lin_muls >= out.fast_stats.muls());
        let mut dense_rng = StdRng::seed_from_u64(389);
        // The fast stage draws one seed for its private RNG, including on
        // separation failure; the dense reference starts from that state.
        dense_rng.next_u64();
        let mut dense_stats = jv_cover::ZeroDimStats::default();
        let dense_roots = jv_cover::linear_stage_from_basis(
            &basis,
            &basis,
            3,
            11,
            &mut dense_rng,
            jv_cover::LinMode::Dense,
            &mut dense_stats,
            Instant::now(),
        );
        assert_eq!(dense_roots, brute);
        assert_eq!(stats.lin_muls, dense_stats.lin_muls + out.fast_stats.muls());
    }

    #[test]
    fn no_rational_roots_is_distinct_from_refusal() {
        let basis = vec![
            vec![(vec![1, 0], 1), (vec![0, 1], 10)],
            vec![(vec![0, 2], 1), (vec![0, 0], 9)],
        ];
        for action in 0..2 {
            let mut stats = zero_dim_la::FastStats::default();
            let mut rng = StdRng::seed_from_u64(389);
            assert_eq!(
                zero_dim_la::fast_stage_with_action(
                    &basis, &basis, 2, 11, action, &mut rng, &mut stats
                ),
                Some(Vec::new())
            );
            assert_eq!(stats.gave_up, None);
        }
    }

    #[test]
    fn multiple_point_fixture_matches_incumbent_exact_recovery() {
        let basis = vec![
            vec![(vec![1, 0], 1), (vec![0, 1], 10)],
            vec![(vec![0, 2], 1)],
        ];
        for action in 0..2 {
            let mut stats = zero_dim_la::FastStats::default();
            let mut rng = StdRng::seed_from_u64(389);
            let roots = zero_dim_la::fast_stage_with_action(
                &basis, &basis, 2, 11, action, &mut rng, &mut stats,
            );
            let mut old_rng = StdRng::seed_from_u64(389);
            let mut old_stats = crypto_lib::cryptanalysis::zero_dim_la::FastStats::default();
            let incumbent = crypto_lib::cryptanalysis::zero_dim_la::fast_stage(
                &basis,
                &basis,
                2,
                11,
                &mut old_rng,
                &mut old_stats,
            );
            assert_eq!(roots, Some(vec![vec![0, 0]]));
            assert_eq!(roots, incumbent);
            assert_eq!(stats.gave_up, old_stats.gave_up);
        }
    }

    #[test]
    fn invalid_coordinate_does_not_consume_rng_or_become_an_empty_answer() {
        let (basis, _) = shape_basis(3, 101);
        let mut rng = StdRng::seed_from_u64(389);
        let mut control = rng.clone();
        let mut stats = zero_dim_la::FastStats::default();
        assert!(zero_dim_la::fast_stage_with_action(
            &basis, &basis, 3, 101, 3, &mut rng, &mut stats
        )
        .is_none());
        assert_eq!(stats.gave_up, Some("action variable out of range"));
        assert_eq!(rng.next_u64(), control.next_u64());
        let mut out = jv_cover::ZeroDimStats::default();
        assert!(action_operator::linear_stage_from_basis_with_action(
            &basis,
            &basis,
            3,
            101,
            3,
            &mut rng,
            &mut out,
            Instant::now()
        )
        .is_err());
        assert_eq!(out.lin_fallback, 0);
    }
}
