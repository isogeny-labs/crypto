//! Print the host-independent Appendix D.4 numeric admission boundary.

use hashed_ecdl_tensor::admission::{audit_numeric_recursion, first_public_nonzero_scale_size};
use hashed_ecdl_tensor::BigUint;
use num_traits::Num;
use serde_json::json;

fn parse_size(value: &str) -> Result<BigUint, String> {
    if let Some(rest) = value.strip_prefix("2^") {
        let (exponent, offset) = match rest.split_once('+') {
            Some((exponent, offset)) => (exponent, offset),
            None => (rest, "0"),
        };
        let exponent: usize = exponent
            .parse()
            .map_err(|_| "invalid power-of-two exponent".to_owned())?;
        let offset = BigUint::from_str_radix(offset, 10)
            .map_err(|_| "invalid power-of-two offset".to_owned())?;
        Ok((BigUint::from(1u8) << exponent) + offset)
    } else {
        BigUint::from_str_radix(value, 10).map_err(|_| "invalid decimal size".to_owned())
    }
}

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let size = match args.next() {
        Some(value) => parse_size(&value)?,
        None => first_public_nonzero_scale_size(),
    };
    let maximum_degree = match args.next() {
        Some(value) => parse_size(&value)?,
        None => BigUint::from(2u8),
    };
    let depth = match args.next() {
        Some(value) => value.parse().map_err(|_| "invalid depth".to_owned())?,
        None => 0,
    };
    if args.next().is_some() {
        return Err("usage: tensor-admission-audit [SIZE|2^K+N] [MAX_DEGREE] [DEPTH]".into());
    }
    let audit = audit_numeric_recursion(&size, &maximum_degree, depth)
        .map_err(|error| error.to_string())?;
    let report = json!({
        "schema": "hashed-ecdl-tensor-numeric-admission/v1",
        "scope": "numeric D.4 guards only; predicate description/evaluation and aggregate cost hypotheses are not certified",
        "size_bits": audit.size.bits(),
        "lambda": audit.lambda,
        "leaf_size_bits": audit.leaf_size.bits(),
        "delta_bits": audit.delta.bits(),
        "u": audit.u,
        "a": audit.a,
        "length_bits": audit.length.bits(),
        "array_bits": audit.array.bits(),
        "guards": {
            "degree_hypothesis": audit.degree_hypothesis,
            "above_leaf": audit.above_leaf,
            "scale": audit.scale_guard,
            "variance": audit.variance_guard,
            "packing": audit.packing_guard,
            "length": audit.length_guard
        },
        "source_numeric_active": audit.source_numeric_active,
        "live_host_representable": audit.live_host_representable
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?
    );
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("tensor-admission-audit: {error}");
        std::process::exit(2);
    }
}
