//! Development-only semantic-cut shape preflight for the E1 rank census.

use hashed_ecdl_tensor::canonical::{CanonicalNetwork, ExportPolicy, IrProvenance};
use hashed_ecdl_tensor::census::{
    boundary_shape, contract_functional_boundary, expanded_rank_summary,
};
use hashed_ecdl_tensor::curve::{Curve, Point};
use hashed_ecdl_tensor::reduction::{
    decomposition_network_with_semantic_cut, FactorBase, HashSeed,
};
use hashed_ecdl_tensor::{BigUint, Limits, Meter};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn left_vertices_sha256(vertices: &[usize]) -> Result<String, Box<dyn std::error::Error>> {
    let mut hasher = Sha256::new();
    hasher.update(b"hashed-ecdl-tensor semantic left vertices v1\0");
    for &vertex in vertices {
        hasher.update(u64::try_from(vertex)?.to_le_bytes());
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() > 1 {
        return Err("usage: tensor-e1-shape [CREATE_NEW_OUTPUT]".into());
    }
    let mut meter = Meter::new(Limits {
        steps: 4_000_000_000,
        vector: 50_000_000,
        bits: 2_000_000,
    });
    let curve = Curve::new(
        5u8.into(),
        2u8.into(),
        1u8.into(),
        7u8.into(),
        Point::Affine {
            x: 0u8.into(),
            y: 1u8.into(),
        },
        &mut meter,
    )?;
    let seeds = [(7u8, 3u8), (2, 1), (3, 4), (5, 0)];
    let mut rows = Vec::new();
    for slots in 2..=4 {
        let bases = seeds[..slots]
            .iter()
            .map(|&(alpha, beta)| {
                FactorBase::new(
                    &curve,
                    BigUint::from(11u8),
                    BigUint::from(5u8),
                    HashSeed {
                        alpha: alpha.into(),
                        beta: beta.into(),
                    },
                    &mut meter,
                )
            })
            .collect::<hashed_ecdl_tensor::Result<Vec<_>>>()?;
        let (network, left_vertices) =
            decomposition_network_with_semantic_cut(&curve, &bases, curve.generator(), &mut meter)?;
        let pins = vec![None; network.edge_count()];
        let canonical = CanonicalNetwork::capture(
            &network,
            &pins,
            IrProvenance {
                source: "E1 disjoint F5 semantic-cut shape preflight".into(),
                generator: "hashed-ecdl-tensor::reduction::decomposition_network_with_semantic_cut"
                    .into(),
                revision: "development-preflight-v1".into(),
            },
            ExportPolicy::RequireTagged,
            &mut meter,
        )?;
        let shape = boundary_shape(&canonical, &left_vertices, &mut meter)?;
        let functional =
            contract_functional_boundary(&network, &left_vertices, 20, 20, &mut meter)?;
        let ranks = expanded_rank_summary(&functional, &mut meter)?;
        let minimum_side_bits = ranks.raw_width / 2;
        let normalized_log_rank = if ranks.best_balanced_rank == 0 || minimum_side_bits == 0 {
            None
        } else {
            Some((ranks.best_balanced_rank as f64).log2() / minimum_side_bits as f64)
        };
        let rank_storage_entries = if minimum_side_bits == 0 {
            BigUint::from(ranks.best_balanced_rank)
        } else {
            BigUint::from(ranks.best_balanced_rank)
                * ((BigUint::from(1u8) << minimum_side_bits)
                    + (BigUint::from(1u8) << (ranks.raw_width - minimum_side_bits)))
        };
        let mut region_vertex_counts = BTreeMap::<String, usize>::new();
        for vertex in &canonical.vertices {
            let region = vertex
                .predicate
                .tag
                .parameters
                .iter()
                .find_map(|(key, value)| (key == "semantic_region").then_some(value))
                .ok_or("canonical semantic-cut vertex lacks region")?;
            *region_vertex_counts.entry(region.clone()).or_default() += 1;
        }
        rows.push(json!({
            "field_modulus": 5,
            "factor_base_prime": 11,
            "factor_base_threshold": 5,
            "slots": slots,
            "cut_after_slots": slots.div_ceil(2),
            "network_vertices": network.vertex_count(),
            "network_edges": network.edge_count(),
            "interface_edges": network.interface().len(),
            "maximum_vertex_degree": network.maximum_degree(),
            "canonical_ir_sha256": canonical.sha256_hex(&mut meter)?,
            "left_vertices": left_vertices.len(),
            "left_vertices_sha256": left_vertices_sha256(&left_vertices)?,
            "cut_edges": shape.cut_edges,
            "boundary_width": shape.cut_edges.len(),
            "left_internal_edges": shape.internal_edges.len(),
            "left_free_internal_edges": shape.free_internal_edges.len(),
            "left_pinned_internal_edges": shape.pinned_internal_edges.len(),
            "current_contractor_enumerated_free_width": shape.enumerated_free_width,
            "current_contractor_assignment_exponent": shape.enumerated_free_width,
            "admitted_by_cli_safety_ceiling_30": shape.enumerated_free_width <= 30,
            "functional_free_input_width": functional.selected_free_input_positions.len(),
            "functional_assignments_examined": functional.quotient.assignments_examined,
            "signal_quotient_width": functional.quotient.width(),
            "signal_class_sizes": functional.cut_edge_classes.iter().map(Vec::len).collect::<Vec<_>>(),
            "quotient_support": functional.quotient.support,
            "quotient_tensor_sha256": functional.quotient.sha256_hex()?,
            "exact_raw_rank_via_equality_class_symmetry": true,
            "canonical_raw_balanced_rank": ranks.canonical_rank,
            "best_raw_balanced_rank": ranks.best_balanced_rank,
            "normalized_log_rank": normalized_log_rank,
            "rank_storage_entries": rank_storage_entries.to_string(),
            "balanced_class_patterns_examined": ranks.balanced_class_patterns_examined,
            "best_class_states": ranks.best_class_states,
            "rank_operations": {
                "additions": ranks.operations.additions,
                "multiplications": ranks.operations.multiplications,
                "comparisons": ranks.operations.comparisons,
                "row_normalizations": ranks.operations.row_normalizations
            },
            "region_vertex_counts": region_vertex_counts
        }));
    }
    let rendered = serde_json::to_string_pretty(&json!({
        "schema": "tensor-e1-semantic-cut-shape-preflight/v1",
        "status": "development_shape_only",
        "fixture_scope": "disjoint F5 production construction; exact natural rank diagnostic, no null, timing, scaling, or ECDLP claim",
        "semantic_cut_law": "after slot ceil(m/2); producer-side equality/fanout wiring remains left",
        "current_contractor_cli_safety_ceiling": 30,
        "rows": rows,
        "charged_steps": meter.used
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
        eprintln!("tensor-e1-shape: {error}");
        std::process::exit(1);
    }
}
