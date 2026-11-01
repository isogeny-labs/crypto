//! Exact boundary-tensor flattening-rank census for canonical BTNIR instances.

use hashed_ecdl_tensor::canonical::CanonicalNetwork;
use hashed_ecdl_tensor::census::{contract_boundary, rank_summary};
use hashed_ecdl_tensor::{Limits, Meter};
use num_traits::Zero;
use serde_json::json;
use std::fs;

fn parse_vertices(text: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let vertices: Vec<usize> = text
        .split(',')
        .map(|part| {
            if part.is_empty() || !part.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err("left vertices must be comma-separated unsigned integers");
            }
            part.parse()
                .map_err(|_| "left vertex index is out of range")
        })
        .collect::<Result<_, _>>()?;
    if vertices.is_empty() || vertices.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err("left vertices must be nonempty, sorted, and unique".into());
    }
    Ok(vertices)
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() == 1 && args[0] == "--help" {
        println!(
            "tensor-census BTNIR LEFT_VERTICES [MAX_FREE_EDGES] [EXHAUSTIVE_WIDTH]\n\
             LEFT_VERTICES is a sorted comma-separated list. Exact enumeration is capped; \
             exhaustion exits nonzero."
        );
        return Ok(());
    }
    if !(2..=4).contains(&args.len()) {
        return Err(
            "usage: tensor-census BTNIR LEFT_VERTICES [MAX_FREE_EDGES] [EXHAUSTIVE_WIDTH]".into(),
        );
    }
    let max_free_edges = args.get(2).map_or(Ok(24usize), |value| value.parse())?;
    let exhaustive_width = args.get(3).map_or(Ok(16usize), |value| value.parse())?;
    if max_free_edges > 30 || exhaustive_width > 20 {
        return Err("development caps exceed the CLI safety ceiling".into());
    }
    let left_vertices = parse_vertices(&args[1])?;
    let bytes = fs::read(&args[0])?;
    let vector_cap = 1usize
        .checked_shl(max_free_edges as u32)
        .ok_or("free-edge cap exceeds machine index")?;
    let mut meter = Meter::new(Limits {
        steps: 2_000_000_000,
        vector: vector_cap,
        bits: 8_000_000,
    });
    let network = CanonicalNetwork::from_bytes(&bytes, &mut meter)?;
    let ir_sha256 = network.sha256_hex(&mut meter)?;
    let tensor = contract_boundary(&network, &left_vertices, max_free_edges, &mut meter)?;
    let ranks = rank_summary(&tensor, exhaustive_width, &mut meter)?;
    let minimum_side_bits = ranks.width / 2;
    let normalized_log_rank = if ranks.best_balanced_rank == 0 || minimum_side_bits == 0 {
        None
    } else {
        Some((ranks.best_balanced_rank as f64).log2() / minimum_side_bits as f64)
    };
    let maximum_count = tensor
        .values
        .iter()
        .max()
        .filter(|value| !value.is_zero())
        .map(ToString::to_string)
        .unwrap_or_else(|| "0".into());
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema": "tensor-boundary-census/v1",
            "canonical_ir_sha256": ir_sha256,
            "left_vertices": left_vertices,
            "cut_edges": tensor.cut_edges,
            "internal_edges": tensor.internal_edges,
            "boundary_width": ranks.width,
            "boundary_entries": tensor.values.len(),
            "boundary_support": tensor.support,
            "maximum_completion_count": maximum_count,
            "assignments_examined": tensor.assignments_examined,
            "boundary_tensor_sha256": tensor.sha256_hex()?,
            "canonical_rank": ranks.canonical_rank,
            "best_balanced_rank": ranks.best_balanced_rank,
            "best_row_positions": ranks.best_row_positions,
            "balanced_splits_examined": ranks.balanced_splits_examined,
            "normalized_log_rank": normalized_log_rank,
            "rank_operations": {
                "additions": ranks.operations.additions,
                "multiplications": ranks.operations.multiplications,
                "comparisons": ranks.operations.comparisons,
                "row_normalizations": ranks.operations.row_normalizations
            },
            "charged_steps": meter.used,
            "status": "exact",
            "claim_scope": "bounded boundary tensor and exact flattening ranks; no asymptotic or ECDLP speedup claim"
        }))?
    );
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("tensor-census: {error}");
        std::process::exit(1);
    }
}
