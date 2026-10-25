//! Fail-closed source inventory for the common weighted 2+2 prefix.
//!
//! This compares the literal Rust statements that the Lean model abstracts.
//! It is a source-shape check, not a verified compiler translation or a proof
//! that the concrete `Field` implementation satisfies the Lean `Field` laws.

use serde_json::json;
use std::fs;
use std::path::Path;

const PREFIX: &str = r#"let [q1, r1, s1, t1, z1, w1] = a;
    let [q2, r2, s2, t2, z2, w2] = b;
    assert_ne!(z1, 0);
    assert_ne!(z2, 0);
    assert_ne!(w1, 0);
    assert_ne!(w2, 0);
    let z1_2 = f.sq(z1);
    let z2_2 = f.sq(z2);
    let z1_3 = f.mul(z1_2, z1);
    let z2_3 = f.mul(z2_2, z2);
    let z1_4 = f.sq(z1_2);
    let z2_4 = f.sq(z2_2);
    let z1_5 = f.mul(z1_4, z1);
    let z2_5 = f.mul(z2_4, z2);
    let q1c = f.mul(q1, z2_2);
    let q2c = f.mul(q2, z1_2);
    let r1c = f.mul(r1, z2_4);
    let r2c = f.mul(r2, z1_4);
    let s1c = f.mul(f.mul(s1, z2_3), w2);
    let s2c = f.mul(f.mul(s2, z1_3), w1);
    let t1c = f.mul(f.mul(t1, z2_5), w2);
    let t2c = f.mul(f.mul(t2, z1_5), w1);
    let dq = f.sub(q1c, q2c);
    let dr = f.sub(r1c, r2c);
    let ds = f.sub(s1c, s2c);
    let dt = f.sub(t1c, t2c);
    let bcoef = f.sub(f.mul(dq, f.sub(f.mul(q2c, r1c), f.mul(q1c, r2c))), f.sq(dr));
    if bcoef == 0 {
        return None;
    }
    let acoef = f.sub(
        f.mul(dq, f.sub(f.mul(dt, q2c), f.mul(ds, r2c))),
        f.mul(dr, dt),
    );
    let ccoef = f.sub(f.mul(dq, dt), f.mul(dr, ds));
    if ccoef == 0 {
        return None;
    }"#;

fn source_prefix(source: &str) -> Result<&str, String> {
    let marker = "fn add22_weighted_common(a: [u64; 6], b: [u64; 6], f: &Field) -> Option<ResultPoint> {";
    let (_, body) = source.split_once(marker).ok_or("function signature missing")?;
    let (prefix, _) = body.split_once("    let c2 = f.sq(ccoef);").ok_or("prefix end missing")?;
    Ok(prefix.trim())
}

fn validate(source: &str, lean: &str) -> Result<(), String> {
    if source.matches("fn add22_weighted_common(").count() != 1 {
        return Err("expected one common branch".into());
    }
    if source_prefix(source)? != PREFIX {
        return Err("Rust prefix or guard order changed".into());
    }
    for fragment in [
        "def coefficients (c : CrossScale22.CrossInput K)",
        "def classifyPrefix (i : CrossScale22.WeightedInput K)",
        "theorem classifyPrefix_some_iff",
        "theorem coefficients_eq_affine_scaled",
        "theorem classifyPrefix_some_iff_affine",
    ] {
        if !lean.contains(fragment) {
            return Err(format!("Lean declaration missing: {fragment}"));
        }
    }
    if lean.contains("sorry") || lean.contains("admit") || lean.contains("axiom ") {
        return Err("forbidden Lean proof token".into());
    }
    Ok(())
}

fn validate_lean_log(log: &str) -> Result<(), String> {
    if log.contains("error:") || log.contains("sorryAx") {
        return Err("Lean compile or axiom audit failed".into());
    }
    for theorem in [
        "classifyPrefix_some_iff",
        "coefficients_eq_affine_scaled",
        "classifyPrefix_some_iff_affine",
    ] {
        let receipt = format!("'WeightedSource22.{theorem}' depends on axioms:");
        if !log.contains(&receipt) {
            return Err(format!("missing kernel receipt: {theorem}"));
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = fs::read_to_string(root.join("src/bin/projective_law_gate.rs"))?;
    let lean = fs::read_to_string(root.join("formal/WeightedSource22.lean"))?;
    validate(&source, &lean)?;
    let log = fs::read_to_string(root.join("formal/weighted-source22-build.log"))?;
    validate_lean_log(&log)?;
    let bad_sign = source.replacen("f.sq(dr));", "f.neg(f.sq(dr)));", 1);
    if validate(&bad_sign, &lean).is_ok() {
        return Err("B-sign mutation accepted".into());
    }
    let bad_guard = source.replacen("if ccoef == 0 {", "if ccoef != 0 {", 1);
    if validate(&bad_guard, &lean).is_ok() {
        return Err("C-guard mutation accepted".into());
    }
    let report = json!({
        "schema": "weighted-source22-prefix/v1",
        "rust_function": "add22_weighted_common",
        "statement_scope": "input destructuring, four nonzero assertions, 16 cross-scaling statements, four differences, B/A/C expressions, ordered B=0 and C=0 exits",
        "source_prefix_exact": true,
        "lean_model_declarations_present": true,
        "lean_kernel_receipts_checked": true,
        "negative_checks": ["B sign rejected", "C guard rejected"],
        "rust_machine_semantics_proved": false,
        "field_laws_for_concrete_Field_proved": false,
        "output_constructor_proved": false,
        "divisor_class_bridge_proved": false,
        "cost_ratio_change": null
    });
    fs::write(root.join("formal/weighted-source22.json"), serde_json::to_string_pretty(&report)? + "\n")?;
    println!("weighted source prefix: exact statements and guard order match; two mutations rejected");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_mutations_fail() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let source = fs::read_to_string(root.join("src/bin/projective_law_gate.rs")).unwrap();
        let lean = fs::read_to_string(root.join("formal/WeightedSource22.lean")).unwrap();
        assert!(validate(&source, &lean).is_ok());
        assert!(validate(&source.replacen("if bcoef == 0 {", "if bcoef != 0 {", 1), &lean).is_err());
        assert!(validate(&source.replacen("f.mul(q1, z2_2)", "f.mul(q1, z1_2)", 1), &lean).is_err());
        let log = fs::read_to_string(root.join("formal/weighted-source22-build.log")).unwrap();
        assert!(validate_lean_log(&log).is_ok());
        assert!(validate_lean_log(&log.replacen("classifyPrefix_some_iff_affine'", "other'", 1)).is_err());
    }
}
