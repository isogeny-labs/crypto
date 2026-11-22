//! Export the actual ECC2K-130 full-kernel Boolean model; no solve is implied.
use full_trace_zero_backend::binary_ecc::{
    BinaryCurve, BinaryPoint, TraceZeroAffineMode, TraceZeroVariety,
};
use serde_json::json;
use std::{env, fs::OpenOptions, io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() > 2 {
        return Err(
            "usage: full-trace-zero-validation [raw|compact|factored] [new-export.json]".into(),
        );
    }
    let mode = match args.first().map(String::as_str).unwrap_or("factored") {
        "raw" => TraceZeroAffineMode::Raw,
        "compact" => TraceZeroAffineMode::Compact,
        "factored" => TraceZeroAffineMode::Factored,
        _ => return Err("expected raw, compact or factored".into()),
    };
    let (curve, target) =
        BinaryCurve::certicom_challenge("ECC2K-130").ok_or("missing challenge")?;
    let variety = TraceZeroVariety::new(&curve).map_err(|e| format!("admission: {e:?}"))?;
    let model = variety
        .affine_system(mode, 2_000_000)
        .map_err(|e| format!("model: {e:?}"))?;
    let mut output = json!({
        "schema":"full-trace-zero-affine/v1", "curve":"ECC2K-130", "field_degree":curve.m,
        "field_polynomial_low_terms":curve.irreducible.low_terms,
        "geometric_dimension":variety.dimension(), "rational_kernel_order":variety.rational_point_order().to_string(),
        "scope":"F2-rational affine points; explicit identity branch outside affine equations",
        "ring":"F2[v]/(v_i^2+v_i)", "equation_rhs":0, "mode":format!("{mode:?}"),
        "variables":model.variables, "equations":model.equations.len(), "term_occurrences":model.monomials(),
        "maximum_boolean_degree":model.maximum_degree(), "and_auxiliaries":model.auxiliary_variables(),
        "generator_full_trace_zero":variety.contains(&curve.generator),
        "target_full_trace_zero":variety.contains(&target),
        "generator_point_trace_identity":variety.point_trace(&curve.generator).map_err(|e|format!("trace: {e:?}"))?==BinaryPoint::Infinity,
        "target_point_trace_identity":variety.point_trace(&target).map_err(|e|format!("trace: {e:?}"))?==BinaryPoint::Infinity,
        "independent_standing":"unverified", "solver_outcome":null
    });
    println!("{}", serde_json::to_string_pretty(&output)?);
    if let Some(path) = args.get(1) {
        output["polynomials"] = json!(model
            .equations
            .iter()
            .map(|p| p.terms.iter().collect::<Vec<_>>())
            .collect::<Vec<_>>());
        let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
        serde_json::to_writer(&mut file, &output)?;
        file.write_all(b"\n")?;
    }
    Ok(())
}
