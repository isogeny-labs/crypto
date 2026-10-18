//! Item 2: extend the committed descent model to named binary curves.
//! Every number is derived under assumptions, not a measured attack cost.
//! Protocol: research/ideas_execution_20261018/cost_protocol.json.
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde::Serialize;
use serde_json::json;
use std::io::Write;

fn log_add(a: f64, b: f64) -> f64 {
    a.max(b) + (1.0 + 2f64.powf(-(a - b).abs())).log2()
}

fn columns(n: usize, degree: usize) -> f64 {
    let mut log_term = 0.0;
    let mut log_sum = 0.0;
    for j in 1..=degree.min(n) {
        log_term += ((n - j + 1) as f64).log2() - (j as f64).log2();
        log_sum = log_add(log_sum, log_term);
    }
    log_sum
}

fn big_log2(n: &BigUint) -> f64 {
    assert!(n.bits() > 0);
    let shift = n.bits().saturating_sub(53);
    ((n >> shift).to_u64().unwrap() as f64).log2() + shift as f64
}

// Read the subgroup orders from the pinned constructor source, without
// linking the currently uncompilable crypto_lib. This is a textual input
// binding, not an independent validation of the curve parameters.
fn constructor_order(name: &str) -> BigUint {
    let source = include_str!("../../../src/binary_ecc/curve.rs");
    let start = source
        .find(&format!("pub fn {name}() -> Self {{"))
        .expect("constructor");
    let body = source[start..].split("\n    }").next().expect("body");
    let quoted: Vec<&str> = body.split('"').skip(1).step_by(2).collect();
    assert_eq!(quoted.len(), 5, "constructor layout changed for {name}");
    BigUint::parse_bytes(quoted[4].as_bytes(), 16).expect("hex subgroup order")
}

#[derive(Clone, Copy)]
enum Model {
    Free,
    Constant(usize),
    GuessOne,
    GuessLinear,
}
impl Model {
    fn name(self) -> String {
        match self {
            Self::Free => "free".into(),
            Self::Constant(d) => format!("constant{d}"),
            Self::GuessOne => "guess1_degree4".into(),
            Self::GuessLinear => "guess_m_times_ell_minus6_degree4".into(),
        }
    }
    fn parameters(self, m: usize, ell: usize) -> (usize, usize) {
        match self {
            Self::Free => (0, 0),
            Self::Constant(d) => (d, 0),
            Self::GuessOne => (4, 1),
            Self::GuessLinear => (4, m * ell.saturating_sub(6)),
        }
    }
}

#[derive(Serialize)]
struct Row {
    curve: String,
    n: usize,
    subgroup_order_hex: Option<String>,
    subgroup_order_log2: f64,
    idealized_automorphism_order: usize,
    rho_group_operations_log2: f64,
    m: usize,
    model: String,
    w: f64,
    assumed_conversion: f64,
    ell: usize,
    boolean_unknowns: usize,
    guessed_variables: usize,
    attempts_log2: f64,
    oracle_column_work_log2: f64,
    relation_algebra_group_equivalents_log2: f64,
    modeled_partial_cost_log2: f64,
    conditional_gap_bits: f64,
    factor_base_coordinate_storage_estimate_log2_bytes: f64,
    missing_phases: [&'static str; 3],
    status: &'static str,
}

fn row(
    n: usize,
    r: f64,
    a: usize,
    m: usize,
    ell: usize,
    model: Model,
    w: f64,
    conversion: f64,
) -> Row {
    let (degree, guesses) = model.parameters(m, ell);
    let unknowns = (m - 2) * n + m * ell;
    assert!(guesses <= unknowns);
    let log_factorial: f64 = (1..=m).map(|j| (j as f64).log2()).sum();
    let attempts = ell as f64 + (log_factorial + n as f64 - (m * ell) as f64).max(0.0);
    let oracle = guesses as f64 + w * columns(unknowns - guesses, degree);
    let algebra = (m as f64).log2() + 2.0 * ell as f64;
    let total = log_add(attempts + oracle + conversion.log2(), algebra);
    let rho = 0.5 * (r + (std::f64::consts::PI / 2.0).log2() - (a as f64).log2());
    Row {
        curve: String::new(),
        n,
        subgroup_order_hex: None,
        subgroup_order_log2: r,
        idealized_automorphism_order: a,
        rho_group_operations_log2: rho,
        m,
        model: model.name(),
        w,
        assumed_conversion: conversion,
        ell,
        boolean_unknowns: unknowns,
        guessed_variables: guesses,
        attempts_log2: attempts,
        oracle_column_work_log2: oracle,
        relation_algebra_group_equivalents_log2: algebra,
        modeled_partial_cost_log2: total,
        conditional_gap_bits: total - rho,
        factor_base_coordinate_storage_estimate_log2_bytes: ell as f64
            + ((2 * n.div_ceil(8)) as f64).log2(),
        missing_phases: [
            "factor_base_construction",
            "successful_root_extraction",
            "other_overheads",
        ],
        status: "derived_conditional_partial_cost_no_security_claim",
    }
}

fn optimum(n: usize, r: f64, a: usize, m: usize, model: Model, w: f64, conversion: f64) -> Row {
    (4..=n)
        .map(|ell| row(n, r, a, m, ell, model, w, conversion))
        .min_by(|a, b| {
            a.modeled_partial_cost_log2
                .total_cmp(&b.modeled_partial_cost_log2)
        })
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::env::args()
        .nth(1)
        .ok_or("usage: deployed_binary_descent_cost OUTPUT.json")?;
    if std::path::Path::new(&output).exists() {
        return Err("output exists; preserve the prior run".into());
    }
    let mut curves = vec![("historical-ecc2k130-model", 131, 129.0, 262, None)];
    for (name, n, koblitz) in [
        ("sect233k1", 233, true),
        ("sect233r1", 233, false),
        ("sect283k1", 283, true),
        ("sect283r1", 283, false),
        ("sect409k1", 409, true),
        ("sect409r1", 409, false),
        ("sect571k1", 571, true),
        ("sect571r1", 571, false),
    ] {
        let order = constructor_order(name);
        curves.push((
            name,
            n,
            big_log2(&order),
            if koblitz { 2 * n } else { 2 },
            Some(order.to_str_radix(16)),
        ));
    }
    let models = [
        Model::Free,
        Model::Constant(4),
        Model::Constant(5),
        Model::Constant(6),
        Model::GuessOne,
        Model::GuessLinear,
    ];
    let mut rows = Vec::new();
    for (name, n, r, a, order) in curves {
        for m in [3, 4, 5, 6] {
            for model in models {
                for w in [1.0, 2.0, 2.2] {
                    for conversion in [0.0, 0.1, 1.0, 10.0] {
                        let mut x = optimum(n, r, a, m, model, w, conversion);
                        x.curve = name.into();
                        x.subgroup_order_hex = order.clone();
                        rows.push(x);
                    }
                }
            }
        }
    }
    let result = json!({
        "schema":"deployed-binary-descent-cost/v1", "item":2,
        "classification":"derived model; no timing or security claim",
        "protocol":"research/ideas_execution_20261018/cost_protocol.json",
        "amendment":"research/ideas_execution_20261018/cost_protocol_amendment_01.json",
        "rows":rows,
        "calibration":"unmeasured; conversion is sensitivity only",
        "degree4_status":"counterfactual sensitivity, not a validated deployed-size model"
    });
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    serde_json::to_writer_pretty(&mut file, &result)?;
    file.write_all(b"\n")?;
    println!("wrote {} derived conditional rows", rows.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn historical_model_anchors() {
        let lower = optimum(131, 129.0, 262, 4, Model::Constant(4), 1.0, 1.0);
        let quadratic = optimum(131, 129.0, 262, 4, Model::Constant(4), 2.0, 1.0);
        assert!((lower.modeled_partial_cost_log2 - 68.43).abs() < 0.02);
        assert!((quadratic.modeled_partial_cost_log2 - 93.91).abs() < 0.02);
        assert_eq!(lower.ell, 33);
        assert_eq!(quadratic.ell, 34);
    }
    #[test]
    fn column_counts_and_oversaturated_relations() {
        assert!((columns(5, 2).exp2() - 16.0).abs() < 1e-10);
        assert!((columns(5, 9).exp2() - 32.0).abs() < 1e-10);
        let x = row(23, 21.0, 46, 4, 20, Model::Free, 1.0, 1.0);
        assert_eq!(x.attempts_log2, 20.0);
    }
    #[test]
    fn group_order_log_and_guess_charge() {
        assert!((big_log2(&(BigUint::from(1u64) << 571u32)) - 571.0).abs() < 1e-10);
        let x = row(131, 129.0, 262, 4, 33, Model::GuessLinear, 2.0, 1.0);
        assert_eq!(x.guessed_variables, 108);
        assert!(x.oracle_column_work_log2 >= 108.0);
        let zero = row(131, 129.0, 262, 4, 33, Model::Constant(4), 2.0, 0.0);
        assert_eq!(zero.modeled_partial_cost_log2, zero.relation_algebra_group_equivalents_log2);
    }
    #[test]
    fn constructor_orders_are_bound_and_have_expected_sizes() {
        for (name, bits) in [
            ("sect233k1", 232),
            ("sect233r1", 233),
            ("sect283k1", 281),
            ("sect283r1", 282),
            ("sect409k1", 407),
            ("sect409r1", 409),
            ("sect571k1", 570),
            ("sect571r1", 570),
        ] {
            assert_eq!(constructor_order(name).bits(), bits, "{name}");
        }
    }
}
