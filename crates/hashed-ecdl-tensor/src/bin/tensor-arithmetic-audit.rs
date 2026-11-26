//! Print exact width and coarse operation envelopes for one active tensor call.
//!
//! The report prices the current direct ring interpolation and Hermite rational
//! recovery representation.  It does not compose the recursive call tree.

use hashed_ecdl_tensor::admission::dimensions::{audit_arithmetic_widths, NodeDimensions};
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
    let (input, extension_degree) = if args.is_empty() {
        (
            NodeDimensions {
                edges: 64u8.into(),
                packets: 8u8.into(),
                a: 2,
                u: 2,
                coordinate: 0,
                precision: 128u8.into(),
            },
            32u8.into(),
        )
    } else if args.len() == 7 {
        (
            NodeDimensions {
                edges: decimal(&args[0], "edge count")?,
                packets: decimal(&args[1], "packet count")?,
                a: small(&args[2], "packet exponent")?,
                u: small(&args[3], "retained valuation levels")?,
                coordinate: small(&args[4], "active coordinate")?,
                precision: decimal(&args[5], "precision")?,
            },
            decimal(&args[6], "extension degree")?,
        )
    } else {
        return Err(
            "usage: tensor-arithmetic-audit [EDGES PACKETS A U COORDINATE PRECISION EXTENSION_DEGREE]"
                .into(),
        );
    };
    let audit =
        audit_arithmetic_widths(&input, &extension_degree).map_err(|error| error.to_string())?;
    let report = json!({
        "schema": "hashed-ecdl-tensor-arithmetic-audit/v1",
        "scope": "exact width and coarse direct-operation envelopes for one active call; no recursive-tree composition",
        "input": {
            "edges": input.edges.to_string(),
            "packets": input.packets.to_string(),
            "a": input.a,
            "u": input.u,
            "coordinate": input.coordinate,
            "precision": input.precision.to_string(),
            "extension_degree": extension_degree.to_string()
        },
        "bit_width_bounds": {
            "stored_coefficient": audit.coefficient_bits_bound.to_string(),
            "ring_multiplication_temporary": audit.ring_multiplication_temporary_bits_bound.to_string(),
            "binomial_integer": audit.binomial_integer_bits_bound.to_string(),
            "maximum_multiplicity": audit.maximum_multiplicity_bound.to_string(),
            "rational_multiplier": audit.rational_multiplier_bits_bound.to_string(),
            "rational_accumulator": audit.rational_accumulator_bits_bound.to_string()
        },
        "operation_bounds": {
            "interpolation_ring": audit.interpolation_ring_operations_bound.to_string(),
            "recovery_ring": audit.recovery_ring_operations_bound.to_string(),
            "recovery_rational": audit.recovery_rational_operations_bound.to_string()
        }
    });
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("tensor-arithmetic-audit: {error}");
        std::process::exit(2);
    }
}
