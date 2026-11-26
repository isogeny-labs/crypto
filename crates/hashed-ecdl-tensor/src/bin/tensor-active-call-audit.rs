//! Print exact dimension and coarse work envelopes for one Appendix D.4 call.
//!
//! The output is conditional on the supplied dimensions and the packet-capacity
//! check.  It does not allocate a graph, prove source admission, or certify the
//! full recursive contraction theorem.

use hashed_ecdl_tensor::admission::dimensions::{audit_node_dimensions, NodeDimensions};
use hashed_ecdl_tensor::BigUint;
use serde_json::json;

fn decimal(value: &str, name: &str) -> Result<BigUint, String> {
    BigUint::parse_bytes(value.as_bytes(), 10).ok_or_else(|| format!("invalid {name}"))
}

fn small(value: &str, name: &str) -> Result<u32, String> {
    value.parse().map_err(|_| format!("invalid {name}"))
}

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let input = if args.is_empty() {
        NodeDimensions {
            edges: 64u8.into(),
            packets: 8u8.into(),
            a: 2,
            u: 2,
            coordinate: 0,
            precision: 128u8.into(),
        }
    } else if args.len() == 6 {
        NodeDimensions {
            edges: decimal(&args[0], "edge count")?,
            packets: decimal(&args[1], "packet count")?,
            a: small(&args[2], "packet exponent")?,
            u: small(&args[3], "retained valuation levels")?,
            coordinate: small(&args[4], "active coordinate")?,
            precision: decimal(&args[5], "precision")?,
        }
    } else {
        return Err(
            "usage: tensor-active-call-audit [EDGES PACKETS A U COORDINATE PRECISION]".into(),
        );
    };
    let audit = audit_node_dimensions(&input).map_err(|error| error.to_string())?;
    let report = json!({
        "schema": "hashed-ecdl-tensor-active-call-audit/v1",
        "scope": "conditional exact dimensions and coarse source-work envelopes; no graph allocation, source admission or full-theorem claim",
        "input": {
            "edges": input.edges.to_string(),
            "packets": input.packets.to_string(),
            "a": input.a,
            "u": input.u,
            "coordinate": input.coordinate,
            "precision": input.precision.to_string()
        },
        "dimensions": {
            "packet_length": audit.packet_length.to_string(),
            "tiles": audit.tiles.to_string(),
            "interaction_distance": audit.interaction_distance.to_string(),
            "modes_per_weight_bound": audit.modes_per_weight_bound.to_string(),
            "attempted_modes_bound": audit.attempted_modes_bound.to_string(),
            "samples_per_node": audit.samples_per_node.to_string(),
            "active_nodes": audit.active_nodes.to_string(),
            "retained_jet_coefficients": audit.retained_jet_coefficients.to_string(),
            "retained_jet_vector_slots_lower_bound": audit.retained_jet_vector_slots_lower_bound.to_string()
        },
        "work_envelopes": {
            "column_modpow_calls_bound": audit.column_modpow_calls_bound.to_string(),
            "row_coefficient_calls_bound": audit.row_coefficient_calls_bound.to_string(),
            "mode_tuple_log2_bound": audit.mode_tuple_log2_bound.to_string(),
            "active_call_child_invocations_log2_bound": audit.active_call_child_invocations_log2_bound.to_string(),
            "packet_description_slots_bound": audit.packet_description_slots_bound.to_string(),
            "packet_entry_choice_log2_bound": audit.packet_entry_choice_log2_bound.to_string(),
            "precision_md_over_u_coefficient": audit.precision_md_over_u_coefficient.to_string()
        }
    });
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("tensor-active-call-audit: {error}");
        std::process::exit(2);
    }
}
