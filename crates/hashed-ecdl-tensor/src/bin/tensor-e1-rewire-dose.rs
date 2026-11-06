//! Frozen capped-dose typed topology-null screen for the E1 equality quotient.

use hashed_ecdl_tensor::canonical::{CanonicalNetwork, ExportPolicy, IrProvenance};
use hashed_ecdl_tensor::census::{
    boundary_shape, contract_functional_boundary, expanded_rank_summary,
    rewire_functional_definitions_capped, FunctionalRewireReceipt,
};
use hashed_ecdl_tensor::curve::{Curve, Point};
use hashed_ecdl_tensor::network::Network;
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
const CAPS: [usize; 4] = [1, 2, 4, 8];
const DRAWS: usize = 8;

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn seed(separator: &[u8], field_bits: u64, cap: usize, draw: usize) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(separator);
    hasher.update(field_bits.to_le_bytes());
    hasher.update(u64::try_from(cap).unwrap().to_le_bytes());
    hasher.update(u64::try_from(draw).unwrap().to_le_bytes());
    let digest = hasher.finalize();
    u64::from_le_bytes(digest[..8].try_into().unwrap())
}

fn required_u64(value: &Value, name: &str) -> Result<u64, Box<dyn std::error::Error>> {
    value
        .as_u64()
        .ok_or_else(|| format!("missing or invalid {name}").into())
}

fn snapshot(
    network: &Network,
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

fn stratum(support: usize) -> u32 {
    (support + 1).ilog2()
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: tensor-e1-rewire-dose NATURAL_JSON CREATE_NEW_OUTPUT".into());
    }
    let source_bytes = fs::read(&args[0])?;
    let source_sha256 = sha256_hex(&source_bytes);
    if source_sha256 != NATURAL_SHA256 {
        return Err(format!("natural result SHA-256 mismatch: {source_sha256}").into());
    }
    let source: Value = serde_json::from_slice(&source_bytes)?;
    let source_rows = source["rows"]
        .as_array()
        .filter(|rows| rows.len() == 4)
        .ok_or("natural result must contain four rows")?;
    let mut meter = Meter::new(Limits {
        steps: 100_000_000_000,
        vector: 50_000_000,
        bits: 2_000_000,
    });
    let mut cells = Vec::new();
    for source_row in source_rows {
        let field_bits = required_u64(&source_row["field_bits"], "field bits")?;
        let curve_row = &source_row["curve"];
        let generator = curve_row["generator"]
            .as_array()
            .ok_or("missing generator")?;
        let curve = Curve::new(
            required_u64(&curve_row["p"], "p")?.into(),
            required_u64(&curve_row["a"], "a")?.into(),
            required_u64(&curve_row["b"], "b")?.into(),
            required_u64(&curve_row["r"], "r")?.into(),
            Point::Affine {
                x: required_u64(&generator[0], "generator x")?.into(),
                y: required_u64(&generator[1], "generator y")?.into(),
            },
            &mut meter,
        )?;
        let base_row = &source_row["factor_base"];
        let hash_prime = required_u64(&base_row["prime"], "hash prime")?;
        let threshold = required_u64(&base_row["threshold"], "threshold")?;
        let bases = base_row["seeds"]
            .as_array()
            .ok_or("missing factor-base seeds")?
            .iter()
            .map(|entry| {
                let pair = entry.as_array().ok_or("invalid factor-base seed")?;
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
        let (natural, left_vertices) =
            decomposition_network_with_semantic_cut(&curve, &bases, curve.generator(), &mut meter)?;
        let natural_canonical = snapshot(&natural, &mut meter)?;
        if natural_canonical.sha256_hex(&mut meter)?
            != source_row["canonical_ir_sha256"]
                .as_str()
                .ok_or("missing natural hash")?
        {
            return Err("natural canonical hash did not reproduce".into());
        }
        let natural_shape = boundary_shape(&natural_canonical, &left_vertices, &mut meter)?;
        let natural_tensor =
            contract_functional_boundary(&natural, &left_vertices, 20, 20, &mut meter)?;
        if natural_tensor.quotient.sha256_hex()?
            != source_row["quotient_tensor_sha256"]
                .as_str()
                .ok_or("missing natural tensor hash")?
        {
            return Err("natural tensor did not reproduce".into());
        }
        let natural_rank = expanded_rank_summary(&natural_tensor, &mut meter)?.best_balanced_rank;
        if u64::try_from(natural_rank)?
            != required_u64(&source_row["best_raw_balanced_rank"], "natural rank")?
        {
            return Err("natural rank did not reproduce".into());
        }
        let natural_support = natural_tensor.quotient.support;
        let natural_stratum = stratum(natural_support);
        let natural_degrees = natural_canonical
            .vertices
            .iter()
            .map(|vertex| vertex.edges.len())
            .collect::<Vec<_>>();
        let mut dose_rows = Vec::new();
        for cap in CAPS {
            let mut draws = Vec::new();
            let mut conditioned: Option<Value> = None;
            let mut draw_zero: Option<(FunctionalRewireReceipt, String, String, usize)> = None;
            for draw in 0..DRAWS {
                let draw_seed = seed(b"rank1 typed rewire dose v1\0", field_bits, cap, draw);
                let (rewired, receipt) = rewire_functional_definitions_capped(
                    &natural,
                    &left_vertices,
                    draw_seed,
                    cap,
                    &mut meter,
                )?;
                if receipt.definition_pairs != cap {
                    return Err("typed rewiring did not reach the frozen pair cap".into());
                }
                let canonical = snapshot(&rewired, &mut meter)?;
                if canonical.interface != natural_canonical.interface
                    || canonical.pins != natural_canonical.pins
                    || canonical.edges.len() != natural_canonical.edges.len()
                    || canonical
                        .vertices
                        .iter()
                        .map(|vertex| vertex.edges.len())
                        .ne(natural_degrees.iter().copied())
                    || canonical
                        .vertices
                        .iter()
                        .zip(&natural_canonical.vertices)
                        .any(|(left, right)| left.predicate != right.predicate)
                {
                    return Err("capped rewiring changed a paired local invariant".into());
                }
                let shape = boundary_shape(&canonical, &left_vertices, &mut meter)?;
                if shape.cut_edges != natural_shape.cut_edges
                    || shape.internal_edges != natural_shape.internal_edges
                {
                    return Err("capped rewiring changed cut shape".into());
                }
                let canonical_sha256 = canonical.sha256_hex(&mut meter)?;
                let tensor =
                    contract_functional_boundary(&rewired, &left_vertices, 20, 20, &mut meter)?;
                let tensor_sha256 = tensor.quotient.sha256_hex()?;
                let rank = expanded_rank_summary(&tensor, &mut meter)?.best_balanced_rank;
                let row = json!({
                    "draw": draw,
                    "seed": draw_seed,
                    "receipt": receipt_json(&receipt),
                    "canonical_ir_sha256": canonical_sha256,
                    "quotient_tensor_sha256": tensor_sha256,
                    "support": tensor.quotient.support,
                    "support_stratum": stratum(tensor.quotient.support),
                    "best_raw_balanced_rank": rank
                });
                if conditioned.is_none() && stratum(tensor.quotient.support) == natural_stratum {
                    conditioned = Some(row.clone());
                }
                if draw == 0 {
                    draw_zero = Some((
                        receipt,
                        row["canonical_ir_sha256"].as_str().unwrap().to_owned(),
                        row["quotient_tensor_sha256"].as_str().unwrap().to_owned(),
                        rank,
                    ));
                }
                draws.push(row);
            }
            let zero_seed = seed(b"rank1 typed rewire dose v1\0", field_bits, cap, 0);
            let (repeated, repeated_receipt) = rewire_functional_definitions_capped(
                &natural,
                &left_vertices,
                zero_seed,
                cap,
                &mut meter,
            )?;
            let repeated_canonical = snapshot(&repeated, &mut meter)?;
            let repeated_tensor =
                contract_functional_boundary(&repeated, &left_vertices, 20, 20, &mut meter)?;
            let repeated_rank =
                expanded_rank_summary(&repeated_tensor, &mut meter)?.best_balanced_rank;
            let zero = draw_zero.ok_or("missing dose draw zero")?;
            if repeated_receipt != zero.0
                || repeated_canonical.sha256_hex(&mut meter)? != zero.1
                || repeated_tensor.quotient.sha256_hex()? != zero.2
                || repeated_rank != zero.3
            {
                return Err("dose draw-zero repeat mismatch".into());
            }
            let conditioned_rank = conditioned
                .as_ref()
                .and_then(|row| row["best_raw_balanced_rank"].as_u64())
                .map(usize::try_from)
                .transpose()?;
            dose_rows.push(json!({
                "pair_cap": cap,
                "draws": draws,
                "conditioned": conditioned,
                "conditioned_matched": conditioned_rank.is_some(),
                "natural_to_conditioned_rank_ratio": conditioned_rank.map(|rank| natural_rank as f64 / rank as f64),
                "draw_zero_repeated_exactly": true
            }));
        }
        let mutation_seed = seed(b"rank1 typed rewire dose mutation v1\0", field_bits, 1, 0);
        let (mutation, _) = rewire_functional_definitions_capped(
            &natural,
            &left_vertices,
            mutation_seed,
            1,
            &mut meter,
        )?;
        let mutation_hash = snapshot(&mutation, &mut meter)?.sha256_hex(&mut meter)?;
        if mutation_hash == dose_rows[0]["draws"][0]["canonical_ir_sha256"] {
            return Err("dose seed mutation was not detected".into());
        }
        cells.push(json!({
            "field_bits": field_bits,
            "natural_support": natural_support,
            "natural_support_stratum": natural_stratum,
            "natural_best_rank": natural_rank,
            "doses": dose_rows,
            "seed_mutation_detected": true
        }));
    }
    let primary = cells
        .iter()
        .map(|cell| (&cell["natural_best_rank"], &cell["doses"][0]))
        .collect::<Vec<_>>();
    let matched_cells = primary
        .iter()
        .filter(|(_, dose)| dose["conditioned_matched"] == true)
        .count();
    let advancing_cells = primary
        .iter()
        .filter(|(_, dose)| {
            dose["natural_to_conditioned_rank_ratio"]
                .as_f64()
                .is_some_and(|ratio| ratio <= 0.8)
        })
        .count();
    let rejecting_cells = primary
        .iter()
        .filter(|(_, dose)| {
            dose["natural_to_conditioned_rank_ratio"]
                .as_f64()
                .is_some_and(|ratio| (0.9..=1.1).contains(&ratio))
        })
        .count();
    let verdict = if matched_cells >= 3 && advancing_cells >= 3 {
        "advance_minimal_typed_topology_mechanism"
    } else if matched_cells >= 3 && rejecting_cells >= 3 {
        "reject_minimal_typed_topology_mechanism"
    } else {
        "inconclusive"
    };
    let rendered = serde_json::to_string_pretty(&json!({
        "schema": "tensor-e1-typed-rewire-dose/v1",
        "protocol": "research/prime_ic_elimination_20261101/RANK1_TYPED_REWIRE_DOSE_PROTOCOL.md",
        "natural_result_sha256": source_sha256,
        "pair_caps": CAPS,
        "draws_per_cap": DRAWS,
        "cells": cells,
        "primary_matched_cells": matched_cells,
        "primary_advancing_cells": advancing_cells,
        "primary_rejecting_cells": rejecting_cells,
        "verdict": verdict,
        "timing_collected": false,
        "charged_steps": meter.used,
        "claim_scope": "capped typed acyclic functional topology rewiring; narrower than full E1 and no scaling, runtime, relation-oracle, or ECDLP claim"
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
        eprintln!("tensor-e1-rewire-dose: {error}");
        std::process::exit(1);
    }
}
