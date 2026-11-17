//! Frozen support-matched sparse-null screen for the E1 equality quotient.

use hashed_ecdl_tensor::census::{expanded_rank_summary, BoundaryTensor, FunctionalBoundaryTensor};
use hashed_ecdl_tensor::{BigUint, Limits, Meter};
use num_traits::Zero;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

const NATURAL_SHA256: &str = "045eb440e4a4348930b07f63418b2c98dfb8264e1771b7206bbfaa8c2bb4b0b6";
const DRAWS: usize = 32;

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn support(
    separator: &[u8],
    field_bits: u64,
    draw: usize,
    width: usize,
    count: usize,
) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let assignments = 1usize
        .checked_shl(u32::try_from(width)?)
        .ok_or("null assignment width overflow")?;
    if count > assignments {
        return Err("null support exceeds quotient space".into());
    }
    let mut ordered = (0..assignments)
        .map(|assignment| {
            let mut hasher = Sha256::new();
            hasher.update(separator);
            hasher.update(field_bits.to_le_bytes());
            hasher.update(u64::try_from(draw).unwrap().to_le_bytes());
            hasher.update(u64::try_from(assignment).unwrap().to_le_bytes());
            (hasher.finalize(), assignment)
        })
        .collect::<Vec<_>>();
    ordered.sort_unstable();
    let mut selected = ordered
        .into_iter()
        .take(count)
        .map(|entry| entry.1)
        .collect::<Vec<_>>();
    selected.sort_unstable();
    Ok(selected)
}

fn tensor(
    class_sizes: &[usize],
    selected: &[usize],
) -> Result<FunctionalBoundaryTensor, Box<dyn std::error::Error>> {
    if selected.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err("null support is not sorted and unique".into());
    }
    let quotient_width = class_sizes.len();
    let entries = 1usize
        .checked_shl(u32::try_from(quotient_width)?)
        .ok_or("null tensor width overflow")?;
    if selected.iter().any(|&assignment| assignment >= entries) {
        return Err("null support assignment out of range".into());
    }
    let mut values = vec![BigUint::zero(); entries];
    for &assignment in selected {
        values[assignment] = BigUint::from(1u8);
    }
    let mut next_edge = 0usize;
    let classes = class_sizes
        .iter()
        .map(|&size| {
            let class = (next_edge..next_edge + size).collect::<Vec<_>>();
            next_edge += size;
            class
        })
        .collect::<Vec<_>>();
    Ok(FunctionalBoundaryTensor {
        raw_cut_edges: (0..next_edge).collect(),
        cut_edge_classes: classes.clone(),
        selected_free_input_positions: Vec::new(),
        quotient: BoundaryTensor {
            cut_edges: classes.iter().map(|class| class[0]).collect(),
            internal_edges: Vec::new(),
            values,
            support: selected.len(),
            assignments_examined: 0,
        },
    })
}

fn parse_usize_array(value: &Value) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let values = value.as_array().ok_or("expected array")?;
    values
        .iter()
        .map(|entry| {
            usize::try_from(entry.as_u64().ok_or("expected unsigned integer")?).map_err(Into::into)
        })
        .collect()
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: tensor-e1-sparse-null NATURAL_JSON CREATE_NEW_OUTPUT".into());
    }
    let natural_bytes = fs::read(&args[0])?;
    let natural_sha256 = sha256_hex(&natural_bytes);
    if natural_sha256 != NATURAL_SHA256 {
        return Err(format!("natural result SHA-256 mismatch: {natural_sha256}").into());
    }
    let natural: Value = serde_json::from_slice(&natural_bytes)?;
    if natural["schema"] != "tensor-e1-equality-quotient-heldout/v1"
        || natural["verdict"] != "advance_to_matched_null_implementation"
        || natural["heldout_cells"] != 4
    {
        return Err("natural result schema or gate mismatch".into());
    }
    let rows = natural["rows"]
        .as_array()
        .ok_or("natural result rows are absent")?;
    let mut meter = Meter::new(Limits {
        steps: 12_000_000_000,
        vector: 50_000_000,
        bits: 2_000_000,
    });
    let mut results = Vec::new();
    for row in rows {
        let field_bits = row["field_bits"].as_u64().ok_or("missing field bits")?;
        let class_sizes = parse_usize_array(&row["signal_class_sizes"])?;
        let quotient_width = usize::try_from(
            row["signal_quotient_width"]
                .as_u64()
                .ok_or("missing quotient width")?,
        )?;
        if class_sizes.len() != quotient_width {
            return Err("natural class-size width mismatch".into());
        }
        let natural_support = row["quotient_nonzero_entries"]
            .as_array()
            .ok_or("missing natural support")?
            .iter()
            .map(|entry| {
                if entry["completion_count"] != "1" {
                    return Err("natural support is not unit-valued");
                }
                usize::try_from(
                    entry["assignment"]
                        .as_u64()
                        .ok_or("missing natural assignment")?,
                )
                .map_err(|_| "natural assignment exceeds usize")
            })
            .collect::<Result<Vec<_>, _>>()?;
        let natural_tensor = tensor(&class_sizes, &natural_support)?;
        let natural_rank = expanded_rank_summary(&natural_tensor, &mut meter)?.best_balanced_rank;
        let recorded_natural_rank = usize::try_from(
            row["best_raw_balanced_rank"]
                .as_u64()
                .ok_or("missing recorded natural rank")?,
        )?;
        if natural_rank != recorded_natural_rank {
            return Err("reconstructed natural rank mismatch".into());
        }
        let mutated = support(
            b"rank1 sparse null mutation v1\0",
            field_bits,
            0,
            quotient_width,
            natural_support.len(),
        )?;
        let first_regular = support(
            b"rank1 sparse null v1\0",
            field_bits,
            0,
            quotient_width,
            natural_support.len(),
        )?;
        if mutated == first_regular {
            return Err("sparse-null seed mutation was not detected".into());
        }
        let mut draws = Vec::new();
        let mut null_ranks = Vec::new();
        for draw in 0..DRAWS {
            let selected = support(
                b"rank1 sparse null v1\0",
                field_bits,
                draw,
                quotient_width,
                natural_support.len(),
            )?;
            let repeated = support(
                b"rank1 sparse null v1\0",
                field_bits,
                draw,
                quotient_width,
                natural_support.len(),
            )?;
            if selected != repeated || selected.len() != natural_support.len() {
                return Err("sparse-null repeated support mismatch".into());
            }
            let null_tensor = tensor(&class_sizes, &selected)?;
            let rank = expanded_rank_summary(&null_tensor, &mut meter)?.best_balanced_rank;
            null_ranks.push(rank);
            draws.push(json!({
                "draw": draw,
                "support": selected,
                "support_count": null_tensor.quotient.support,
                "tensor_sha256": null_tensor.quotient.sha256_hex()?,
                "best_raw_balanced_rank": rank
            }));
        }
        let mut sorted_ranks = null_ranks.clone();
        sorted_ranks.sort_unstable();
        let q1 = sorted_ranks[7];
        let lower_median = sorted_ranks[15];
        let q3 = sorted_ranks[23];
        let null_at_or_below_natural = null_ranks
            .iter()
            .filter(|&&rank| rank <= natural_rank)
            .count();
        results.push(json!({
            "field_bits": field_bits,
            "quotient_width": quotient_width,
            "class_sizes": class_sizes,
            "support_count": natural_support.len(),
            "natural_best_rank": natural_rank,
            "null_ranks": null_ranks,
            "null_q1": q1,
            "null_lower_median": lower_median,
            "null_q3": q3,
            "null_at_or_below_natural": null_at_or_below_natural,
            "p_lower": (1 + null_at_or_below_natural) as f64 / 33.0,
            "natural_to_null_median_ratio": natural_rank as f64 / lower_median as f64,
            "draws": draws,
            "repeated_supports_equal": true,
            "seed_mutation_detected": true
        }));
    }
    let advancing_cells = results
        .iter()
        .filter(|row| {
            row["null_at_or_below_natural"] == 0
                && row["natural_to_null_median_ratio"]
                    .as_f64()
                    .is_some_and(|ratio| ratio <= 0.8)
        })
        .count();
    let sparsity_explained_cells = results
        .iter()
        .filter(|row| {
            let natural = row["natural_best_rank"].as_u64();
            let q1 = row["null_q1"].as_u64();
            let q3 = row["null_q3"].as_u64();
            matches!((natural, q1, q3), (Some(n), Some(low), Some(high)) if low <= n && n <= high)
        })
        .count();
    let verdict = if advancing_cells >= 3 {
        "advance_to_topology_rewired_null_implementation"
    } else if sparsity_explained_cells >= 3 {
        "reject_low_rank_beyond_support_sparsity"
    } else {
        "inconclusive"
    };
    let rendered = serde_json::to_string_pretty(&json!({
        "schema": "tensor-e1-support-matched-sparse-null/v1",
        "protocol": "research/prime_ic_elimination_20261101/RANK1_SPARSE_NULL_PROTOCOL.md",
        "natural_result_sha256": natural_sha256,
        "draws_per_cell": DRAWS,
        "cells": results,
        "advancing_cells": advancing_cells,
        "sparsity_explained_cells": sparsity_explained_cells,
        "verdict": verdict,
        "timing_collected": false,
        "charged_steps": meter.used,
        "claim_scope": "support-matched quotient null; not a topology-preserving rewired null, scaling theorem, runtime, relation-oracle, or ECDLP claim"
    }))?;
    let output_path = Path::new(&args[1]);
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output_path)?;
    output.write_all(rendered.as_bytes())?;
    output.write_all(b"\n")?;
    output.sync_all()?;
    println!("wrote {}", output_path.display());
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("tensor-e1-sparse-null: {error}");
        std::process::exit(1);
    }
}
