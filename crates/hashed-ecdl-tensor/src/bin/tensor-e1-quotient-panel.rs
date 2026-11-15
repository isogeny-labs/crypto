//! Frozen held-out natural-network screen for the E1 equality quotient.

use hashed_ecdl_tensor::arithmetic::Seeded;
use hashed_ecdl_tensor::canonical::{CanonicalNetwork, ExportPolicy, IrProvenance};
use hashed_ecdl_tensor::census::{
    boundary_shape, contract_functional_boundary, expanded_rank_summary,
};
use hashed_ecdl_tensor::curve::{Curve, Point};
use hashed_ecdl_tensor::reduction::{
    decomposition_network_with_semantic_cut, FactorBase, HashSeed,
};
use hashed_ecdl_tensor::{BigUint, Limits, Meter};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(Clone, Copy)]
struct Fixture {
    p: u64,
    a: u64,
    b: u64,
    r: u64,
    gx: u64,
    gy: u64,
    hash_prime: u64,
    threshold: u64,
}

const FIXTURES: [Fixture; 4] = [
    Fixture {
        p: 11,
        a: 1,
        b: 5,
        r: 11,
        gx: 0,
        gy: 4,
        hash_prime: 23,
        threshold: 11,
    },
    Fixture {
        p: 17,
        a: 1,
        b: 3,
        r: 17,
        gx: 2,
        gy: 8,
        hash_prime: 37,
        threshold: 18,
    },
    Fixture {
        p: 37,
        a: 0,
        b: 5,
        r: 37,
        gx: 6,
        gy: 6,
        hash_prime: 79,
        threshold: 39,
    },
    Fixture {
        p: 67,
        a: 0,
        b: 2,
        r: 73,
        gx: 2,
        gy: 12,
        hash_prime: 137,
        threshold: 68,
    },
];

fn digest_indices(indices: &[usize]) -> Result<String, Box<dyn std::error::Error>> {
    let mut hasher = Sha256::new();
    hasher.update(b"hashed-ecdl-tensor semantic left vertices v1\0");
    for &index in indices {
        hasher.update(u64::try_from(index)?.to_le_bytes());
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn cell(fixture: Fixture, meter: &mut Meter) -> Result<Value, Box<dyn std::error::Error>> {
    let generator = Point::Affine {
        x: fixture.gx.into(),
        y: fixture.gy.into(),
    };
    let curve = Curve::new(
        fixture.p.into(),
        fixture.a.into(),
        fixture.b.into(),
        fixture.r.into(),
        generator,
        meter,
    )?;
    let bases = [(2u64, 1u64), (3, 4)]
        .into_iter()
        .map(|(alpha, beta)| {
            FactorBase::new(
                &curve,
                fixture.hash_prime.into(),
                fixture.threshold.into(),
                HashSeed {
                    alpha: alpha.into(),
                    beta: beta.into(),
                },
                meter,
            )
        })
        .collect::<hashed_ecdl_tensor::Result<Vec<_>>>()?;
    let independent_points = bases[0].enumerate(&curve, &mut Seeded(0xe1_2026_1101), meter)?;
    let (network, left_vertices) =
        decomposition_network_with_semantic_cut(&curve, &bases, curve.generator(), meter)?;
    let canonical = CanonicalNetwork::capture(
        &network,
        &vec![None; network.edge_count()],
        IrProvenance {
            source: "rank-1 equality-quotient held-out screen".into(),
            generator: "hashed-ecdl-tensor::reduction::decomposition_network_with_semantic_cut"
                .into(),
            revision: "rank1-quotient-protocol-v1".into(),
        },
        ExportPolicy::RequireTagged,
        meter,
    )?;
    let shape = boundary_shape(&canonical, &left_vertices, meter)?;
    let functional = contract_functional_boundary(&network, &left_vertices, 20, 20, meter)?;
    let ranks = expanded_rank_summary(&functional, meter)?;
    let quotient_width = functional.quotient.width();
    let maximum_balanced_dimension = 1usize
        .checked_shl(u32::try_from(quotient_width / 2)?)
        .ok_or("maximum balanced dimension overflow")?;
    let all_completion_counts_one = functional
        .quotient
        .values
        .iter()
        .filter(|value| **value != BigUint::from(0u8))
        .all(|value| *value == BigUint::from(1u8));
    if functional.quotient.support != independent_points.len() || !all_completion_counts_one {
        return Err("independent factor-base support control failed".into());
    }
    let nonzero_entries = functional
        .quotient
        .values
        .iter()
        .enumerate()
        .filter(|(_, value)| **value != BigUint::from(0u8))
        .map(|(assignment, value)| {
            json!({"assignment": assignment, "completion_count": value.to_string()})
        })
        .collect::<Vec<_>>();
    Ok(json!({
        "field_bits": fixture.p.ilog2() + 1,
        "curve": {
            "p": fixture.p,
            "a": fixture.a,
            "b": fixture.b,
            "r": fixture.r,
            "generator": [fixture.gx, fixture.gy]
        },
        "factor_base": {
            "prime": fixture.hash_prime,
            "threshold": fixture.threshold,
            "seeds": [[2,1],[3,4]],
            "independent_left_point_count": independent_points.len()
        },
        "slots": 2,
        "cut_after_slots": 1,
        "network_vertices": network.vertex_count(),
        "network_edges": network.edge_count(),
        "maximum_vertex_degree": network.maximum_degree(),
        "canonical_ir_sha256": canonical.sha256_hex(meter)?,
        "left_vertices": left_vertices.len(),
        "left_vertices_sha256": digest_indices(&left_vertices)?,
        "raw_boundary_width": shape.cut_edges.len(),
        "raw_left_free_internal_edges": shape.free_internal_edges.len(),
        "legacy_enumerated_free_width": shape.enumerated_free_width,
        "functional_free_input_positions": functional.selected_free_input_positions,
        "functional_assignments_examined": functional.quotient.assignments_examined,
        "signal_quotient_width": quotient_width,
        "signal_class_sizes": functional.cut_edge_classes.iter().map(Vec::len).collect::<Vec<_>>(),
        "quotient_support": functional.quotient.support,
        "all_completion_counts_one": all_completion_counts_one,
        "quotient_nonzero_entries": nonzero_entries,
        "quotient_tensor_sha256": functional.quotient.sha256_hex()?,
        "canonical_raw_balanced_rank": ranks.canonical_rank,
        "best_raw_balanced_rank": ranks.best_balanced_rank,
        "maximum_balanced_quotient_dimension": maximum_balanced_dimension,
        "best_rank_fraction": ranks.best_balanced_rank as f64 / maximum_balanced_dimension as f64,
        "balanced_class_patterns_examined": ranks.balanced_class_patterns_examined,
        "best_class_states": ranks.best_class_states,
        "rank_operations": {
            "additions": ranks.operations.additions,
            "multiplications": ranks.operations.multiplications,
            "comparisons": ranks.operations.comparisons,
            "row_normalizations": ranks.operations.row_normalizations
        }
    }))
}

fn fitted_slope(rows: &[Value]) -> Result<f64, Box<dyn std::error::Error>> {
    let points = rows
        .iter()
        .map(|row| {
            let x = row["field_bits"].as_u64().ok_or("missing field bits")? as f64;
            let rank = row["best_raw_balanced_rank"]
                .as_u64()
                .ok_or("missing rank")?;
            Ok((x, (rank as f64).log2()))
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
    let n = points.len() as f64;
    let mean_x = points.iter().map(|point| point.0).sum::<f64>() / n;
    let mean_y = points.iter().map(|point| point.1).sum::<f64>() / n;
    let numerator = points
        .iter()
        .map(|point| (point.0 - mean_x) * (point.1 - mean_y))
        .sum::<f64>();
    let denominator = points
        .iter()
        .map(|point| (point.0 - mean_x).powi(2))
        .sum::<f64>();
    if denominator == 0.0 {
        return Err("zero held-out size variance".into());
    }
    Ok(numerator / denominator)
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() > 1 {
        return Err("usage: tensor-e1-quotient-panel [CREATE_NEW_OUTPUT]".into());
    }
    let mut meter = Meter::new(Limits {
        steps: 12_000_000_000,
        vector: 50_000_000,
        bits: 2_000_000,
    });
    let mut rows = Vec::new();
    for fixture in FIXTURES {
        let first = cell(fixture, &mut meter)?;
        let second = cell(fixture, &mut meter)?;
        if first != second {
            return Err(
                format!("held-out repeated construction mismatch at p={}", fixture.p).into(),
            );
        }
        let mut retained = first;
        retained["repeated_construction_equal"] = Value::Bool(true);
        rows.push(retained);
    }
    let advances = rows.iter().all(|row| {
        row["signal_quotient_width"]
            .as_u64()
            .is_some_and(|width| width <= 15)
            && row["best_rank_fraction"]
                .as_f64()
                .is_some_and(|fraction| fraction <= 0.5)
    });
    let over_width = rows.iter().any(|row| {
        row["signal_quotient_width"]
            .as_u64()
            .is_some_and(|width| width > 20)
    });
    let near_full = rows
        .iter()
        .filter(|row| {
            row["best_rank_fraction"]
                .as_f64()
                .is_some_and(|fraction| fraction > 0.9)
        })
        .count();
    let verdict = if advances {
        "advance_to_matched_null_implementation"
    } else if over_width || near_full >= 3 {
        "reject_quotient_as_practical_exact_diagnostic"
    } else {
        "inconclusive"
    };
    let slope = fitted_slope(&rows)?;
    let rendered = serde_json::to_string_pretty(&json!({
        "schema": "tensor-e1-equality-quotient-heldout/v1",
        "protocol": "research/prime_ic_elimination_20261101/RANK1_QUOTIENT_PROTOCOL.md",
        "development_f5_excluded": true,
        "heldout_cells": rows.len(),
        "repeated_construction_matches": rows.iter().all(|row| row["repeated_construction_equal"] == true),
        "rows": rows,
        "log2_best_rank_per_field_bit_slope": slope,
        "verdict": verdict,
        "timing_collected": false,
        "charged_steps": meter.used,
        "claim_scope": "exact natural-network equality quotient and raw ranks; no rewired null, scaling theorem, runtime, relation-oracle, or ECDLP claim"
    }))?;
    if let Some(path) = args.first() {
        let path = Path::new(path);
        let mut output = OpenOptions::new().write(true).create_new(true).open(path)?;
        output.write_all(rendered.as_bytes())?;
        output.write_all(b"\n")?;
        output.sync_all()?;
        println!("wrote {}", path.display());
    } else {
        println!("{rendered}");
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("tensor-e1-quotient-panel: {error}");
        std::process::exit(1);
    }
}
