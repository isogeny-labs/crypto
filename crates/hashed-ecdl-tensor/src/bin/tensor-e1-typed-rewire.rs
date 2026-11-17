//! Frozen typed functional topology-null screen for the E1 equality quotient.

use hashed_ecdl_tensor::canonical::{CanonicalNetwork, ExportPolicy, IrProvenance};
use hashed_ecdl_tensor::census::{
    boundary_shape, contract_functional_boundary, expanded_rank_summary,
    rewire_functional_definitions, FunctionalRewireReceipt,
};
use hashed_ecdl_tensor::curve::{Curve, Point};
use hashed_ecdl_tensor::reduction::{
    decomposition_network_with_semantic_cut, FactorBase, HashSeed,
};
use hashed_ecdl_tensor::{Limits, Meter};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

const NATURAL_SHA256: &str = "045eb440e4a4348930b07f63418b2c98dfb8264e1771b7206bbfaa8c2bb4b0b6";
const DRAWS: usize = 32;
const UNCONDITIONED_DRAWS: usize = 8;

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn seed(separator: &[u8], field_bits: u64, draw: usize) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(separator);
    hasher.update(field_bits.to_le_bytes());
    hasher.update(u64::try_from(draw).unwrap().to_le_bytes());
    let digest = hasher.finalize();
    u64::from_le_bytes(digest[..8].try_into().unwrap())
}

fn required_u64(value: &Value, name: &str) -> Result<u64, Box<dyn std::error::Error>> {
    value
        .as_u64()
        .ok_or_else(|| format!("missing or invalid {name}").into())
}

fn canonical_snapshot(
    network: &hashed_ecdl_tensor::network::Network,
    meter: &mut Meter,
) -> Result<CanonicalNetwork, Box<dyn std::error::Error>> {
    Ok(CanonicalNetwork::capture(
        network,
        &vec![None; network.edge_count()],
        IrProvenance {
            source: "rank-1 equality-quotient held-out screen".into(),
            generator: "hashed-ecdl-tensor::reduction::decomposition_network_with_semantic_cut"
                .into(),
            revision: "rank1-quotient-protocol-v1".into(),
        },
        ExportPolicy::RequireTagged,
        meter,
    )?)
}

fn receipt_json(receipt: &FunctionalRewireReceipt) -> Value {
    json!({
        "candidate_definitions": receipt.candidate_definitions,
        "groups_with_pairs": receipt.groups_with_pairs,
        "definition_pairs": receipt.definition_pairs,
        "edge_endpoint_swaps": receipt.edge_endpoint_swaps
    })
}

fn support_stratum(support: usize) -> u32 {
    (support + 1).ilog2()
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: tensor-e1-typed-rewire NATURAL_JSON CREATE_NEW_OUTPUT".into());
    }
    let natural_bytes = fs::read(&args[0])?;
    let natural_sha256 = sha256_hex(&natural_bytes);
    if natural_sha256 != NATURAL_SHA256 {
        return Err(format!("natural result SHA-256 mismatch: {natural_sha256}").into());
    }
    let source: Value = serde_json::from_slice(&natural_bytes)?;
    if source["schema"] != "tensor-e1-equality-quotient-heldout/v1" || source["heldout_cells"] != 4
    {
        return Err("natural result schema mismatch".into());
    }
    let source_rows = source["rows"]
        .as_array()
        .ok_or("natural result rows are absent")?;
    let mut meter = Meter::new(Limits {
        steps: 100_000_000_000,
        vector: 50_000_000,
        bits: 2_000_000,
    });
    let mut cells = Vec::new();
    for source_row in source_rows {
        let field_bits = required_u64(&source_row["field_bits"], "field_bits")?;
        let curve_row = &source_row["curve"];
        let generator_row = curve_row["generator"]
            .as_array()
            .ok_or("missing generator")?;
        let generator = Point::Affine {
            x: required_u64(&generator_row[0], "generator x")?.into(),
            y: required_u64(&generator_row[1], "generator y")?.into(),
        };
        let curve = Curve::new(
            required_u64(&curve_row["p"], "p")?.into(),
            required_u64(&curve_row["a"], "a")?.into(),
            required_u64(&curve_row["b"], "b")?.into(),
            required_u64(&curve_row["r"], "r")?.into(),
            generator,
            &mut meter,
        )?;
        let base_row = &source_row["factor_base"];
        let hash_prime = required_u64(&base_row["prime"], "hash prime")?;
        let threshold = required_u64(&base_row["threshold"], "threshold")?;
        let seed_rows = base_row["seeds"].as_array().ok_or("missing base seeds")?;
        let bases = seed_rows
            .iter()
            .map(|entry| {
                let pair = entry.as_array().ok_or("invalid base seed")?;
                FactorBase::new(
                    &curve,
                    hash_prime.into(),
                    threshold.into(),
                    HashSeed {
                        alpha: required_u64(&pair[0], "alpha")?.into(),
                        beta: required_u64(&pair[1], "beta")?.into(),
                    },
                    &mut meter,
                )
                .map_err(Into::into)
            })
            .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
        let (natural_network, left_vertices) =
            decomposition_network_with_semantic_cut(&curve, &bases, curve.generator(), &mut meter)?;
        let natural_canonical = canonical_snapshot(&natural_network, &mut meter)?;
        let natural_ir_sha256 = natural_canonical.sha256_hex(&mut meter)?;
        if natural_ir_sha256
            != source_row["canonical_ir_sha256"]
                .as_str()
                .ok_or("missing natural canonical hash")?
        {
            return Err("natural canonical hash did not reproduce".into());
        }
        let natural_shape = boundary_shape(&natural_canonical, &left_vertices, &mut meter)?;
        let natural_tensor =
            contract_functional_boundary(&natural_network, &left_vertices, 20, 20, &mut meter)?;
        if natural_tensor.quotient.sha256_hex()?
            != source_row["quotient_tensor_sha256"]
                .as_str()
                .ok_or("missing natural tensor hash")?
        {
            return Err("natural quotient tensor did not reproduce".into());
        }
        let natural_rank = expanded_rank_summary(&natural_tensor, &mut meter)?.best_balanced_rank;
        if u64::try_from(natural_rank)?
            != required_u64(&source_row["best_raw_balanced_rank"], "natural rank")?
        {
            return Err("natural rank did not reproduce".into());
        }
        let natural_support = natural_tensor.quotient.support;
        let natural_stratum = support_stratum(natural_support);
        let natural_degrees = natural_canonical
            .vertices
            .iter()
            .map(|vertex| vertex.edges.len())
            .collect::<Vec<_>>();
        let mut draws = Vec::new();
        let mut unconditioned_ranks = Vec::new();
        let mut conditioned: Option<Value> = None;
        let mut first_identity: Option<(FunctionalRewireReceipt, String, String, usize)> = None;
        for draw in 0..DRAWS {
            let draw_seed = seed(b"rank1 typed rewire v1\0", field_bits, draw);
            let (rewired, receipt) = rewire_functional_definitions(
                &natural_network,
                &left_vertices,
                draw_seed,
                &mut meter,
            )?;
            let rewired_canonical = canonical_snapshot(&rewired, &mut meter)?;
            if rewired_canonical.vertices.len() != natural_canonical.vertices.len()
                || rewired_canonical.edges.len() != natural_canonical.edges.len()
                || rewired_canonical.interface != natural_canonical.interface
                || rewired_canonical.pins != natural_canonical.pins
                || rewired_canonical
                    .vertices
                    .iter()
                    .map(|vertex| vertex.edges.len())
                    .ne(natural_degrees.iter().copied())
                || rewired_canonical
                    .vertices
                    .iter()
                    .zip(&natural_canonical.vertices)
                    .any(|(left, right)| left.predicate != right.predicate)
            {
                return Err("typed rewiring changed a paired local invariant".into());
            }
            let rewired_shape = boundary_shape(&rewired_canonical, &left_vertices, &mut meter)?;
            if rewired_shape.cut_edges != natural_shape.cut_edges
                || rewired_shape.internal_edges != natural_shape.internal_edges
            {
                return Err("typed rewiring changed the semantic cut shape".into());
            }
            let canonical_sha256 = rewired_canonical.sha256_hex(&mut meter)?;
            if canonical_sha256 == natural_ir_sha256 {
                return Err("typed rewiring retained the original wiring".into());
            }
            let tensor =
                contract_functional_boundary(&rewired, &left_vertices, 20, 20, &mut meter)?;
            let tensor_sha256 = tensor.quotient.sha256_hex()?;
            let rank = expanded_rank_summary(&tensor, &mut meter)?.best_balanced_rank;
            if draw < UNCONDITIONED_DRAWS {
                unconditioned_ranks.push(rank);
            }
            let row = json!({
                "draw": draw,
                "seed": draw_seed,
                "receipt": receipt_json(&receipt),
                "canonical_ir_sha256": canonical_sha256,
                "quotient_tensor_sha256": tensor_sha256,
                "support": tensor.quotient.support,
                "support_stratum": support_stratum(tensor.quotient.support),
                "best_raw_balanced_rank": rank
            });
            if conditioned.is_none() && support_stratum(tensor.quotient.support) == natural_stratum
            {
                conditioned = Some(row.clone());
            }
            if draw == 0 {
                first_identity = Some((
                    receipt.clone(),
                    row["canonical_ir_sha256"].as_str().unwrap().to_owned(),
                    row["quotient_tensor_sha256"].as_str().unwrap().to_owned(),
                    rank,
                ));
            }
            draws.push(row);
        }
        let first_seed = seed(b"rank1 typed rewire v1\0", field_bits, 0);
        let (repeated, repeated_receipt) = rewire_functional_definitions(
            &natural_network,
            &left_vertices,
            first_seed,
            &mut meter,
        )?;
        let repeated_canonical = canonical_snapshot(&repeated, &mut meter)?;
        let repeated_tensor =
            contract_functional_boundary(&repeated, &left_vertices, 20, 20, &mut meter)?;
        let repeated_rank = expanded_rank_summary(&repeated_tensor, &mut meter)?.best_balanced_rank;
        let first_identity = first_identity.ok_or("draw zero was not retained")?;
        if repeated_receipt != first_identity.0
            || repeated_canonical.sha256_hex(&mut meter)? != first_identity.1
            || repeated_tensor.quotient.sha256_hex()? != first_identity.2
            || repeated_rank != first_identity.3
        {
            return Err("typed-rewiring draw-zero repeat mismatch".into());
        }
        let mutation_seed = seed(b"rank1 typed rewire mutation v1\0", field_bits, 0);
        let (mutation, _) = rewire_functional_definitions(
            &natural_network,
            &left_vertices,
            mutation_seed,
            &mut meter,
        )?;
        if canonical_snapshot(&mutation, &mut meter)?.sha256_hex(&mut meter)? == first_identity.1 {
            return Err("typed-rewiring seed mutation was not detected".into());
        }
        let mut sorted = unconditioned_ranks.clone();
        sorted.sort_unstable();
        let q1 = sorted[1];
        let lower_median = sorted[3];
        let q3 = sorted[5];
        let unconditioned_min = sorted[0];
        let conditioned_rank = conditioned
            .as_ref()
            .and_then(|row| row["best_raw_balanced_rank"].as_u64())
            .map(usize::try_from)
            .transpose()?;
        cells.push(json!({
            "field_bits": field_bits,
            "natural_support": natural_support,
            "natural_support_stratum": natural_stratum,
            "natural_best_rank": natural_rank,
            "unconditioned_ranks": unconditioned_ranks,
            "unconditioned_min": unconditioned_min,
            "unconditioned_q1": q1,
            "unconditioned_lower_median": lower_median,
            "unconditioned_q3": q3,
            "conditioned": conditioned,
            "conditioned_matched": conditioned_rank.is_some(),
            "natural_to_conditioned_rank_ratio": conditioned_rank.map(|rank| natural_rank as f64 / rank as f64),
            "draws": draws,
            "draw_zero_repeated_exactly": true,
            "seed_mutation_detected": true
        }));
    }
    let advancing_cells = cells
        .iter()
        .filter(|cell| {
            let natural = cell["natural_best_rank"].as_u64();
            let minimum = cell["unconditioned_min"].as_u64();
            matches!((natural, minimum), (Some(left), Some(right)) if left < right)
                && cell["conditioned_matched"] == true
                && cell["natural_to_conditioned_rank_ratio"]
                    .as_f64()
                    .is_some_and(|ratio| ratio <= 0.8)
        })
        .count();
    let rejected_cells = cells
        .iter()
        .filter(|cell| {
            let natural = cell["natural_best_rank"].as_u64();
            let q1 = cell["unconditioned_q1"].as_u64();
            let q3 = cell["unconditioned_q3"].as_u64();
            matches!((natural, q1, q3), (Some(value), Some(low), Some(high)) if low <= value && value <= high)
        })
        .count();
    let verdict = if advancing_cells >= 3 {
        "advance_typed_topology_mechanism"
    } else if rejected_cells >= 3 {
        "reject_typed_topology_mechanism"
    } else {
        "inconclusive"
    };
    let rendered = serde_json::to_string_pretty(&json!({
        "schema": "tensor-e1-typed-functional-rewire/v1",
        "protocol": "research/prime_ic_elimination_20261101/RANK1_TYPED_REWIRE_PROTOCOL.md",
        "natural_result_sha256": natural_sha256,
        "draws_per_cell": DRAWS,
        "unconditioned_draws_per_cell": UNCONDITIONED_DRAWS,
        "cells": cells,
        "advancing_cells": advancing_cells,
        "rejected_cells": rejected_cells,
        "verdict": verdict,
        "timing_collected": false,
        "charged_steps": meter.used,
        "claim_scope": "typed acyclic functional topology rewiring; narrower than the draft E1 all-interior rewired null and no scaling, runtime, relation-oracle, or ECDLP claim"
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
        eprintln!("tensor-e1-typed-rewire: {error}");
        std::process::exit(1);
    }
}
