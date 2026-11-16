//! Replay the historical `bounded-degree-refutation-v1` contract natively.
//!
//! The verifier performs only sparse polynomial arithmetic. It does not search
//! for a refutation and does not trust closure flags or recorded intermediate
//! polynomials. A successful replay proves an ideal identity over Z/pZ. A field
//! last-fall-degree interpretation additionally requires that p is prime.

pub mod bigprime;
pub mod large_curves;

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt;

type Exponents = Vec<u32>;
type Polynomial = BTreeMap<Exponents, u64>;

const MAX_VARIABLES: usize = 256;
const MAX_NODES: usize = 1_000_000;
const MAX_TERMS: usize = 1_000_000;
const MAX_DEGREE: u64 = 1_000_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayError(String);

impl fmt::Display for ReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for ReplayError {}

fn error(message: impl Into<String>) -> ReplayError {
    ReplayError(message.into())
}

fn unsigned(value: &Value, label: &str) -> Result<u64, ReplayError> {
    value
        .as_u64()
        .ok_or_else(|| error(format!("{label} must be an unsigned 64-bit integer")))
}

fn member<'a>(value: &'a Value, key: &str) -> Result<&'a Value, ReplayError> {
    value
        .as_object()
        .and_then(|object| object.get(key))
        .ok_or_else(|| error(format!("missing object member {key}")))
}

fn index(value: &Value, label: &str) -> Result<usize, ReplayError> {
    usize::try_from(unsigned(value, label)?)
        .map_err(|_| error(format!("{label} does not fit this platform")))
}

fn add_mod(a: u64, b: u64, modulus: u64) -> u64 {
    ((u128::from(a) + u128::from(b)) % u128::from(modulus)) as u64
}

fn multiply_mod(a: u64, b: u64, modulus: u64) -> u64 {
    ((u128::from(a) * u128::from(b)) % u128::from(modulus)) as u64
}

fn input_coefficient(value: &Value, modulus: u64) -> Result<u64, ReplayError> {
    let number = value
        .as_number()
        .ok_or_else(|| error("input coefficient must be an integer"))?
        .to_string();
    if number.contains(['.', 'e', 'E']) {
        return Err(error("input coefficient must be an integer"));
    }
    if number.starts_with('-') {
        let coefficient = number
            .parse::<i64>()
            .map_err(|_| error("negative input coefficient exceeds the signed 64-bit range"))?;
        Ok(i128::from(coefficient).rem_euclid(i128::from(modulus)) as u64)
    } else {
        let coefficient = number
            .parse::<u64>()
            .map_err(|_| error("input coefficient exceeds the unsigned 64-bit range"))?;
        Ok(coefficient % modulus)
    }
}

fn insert_term(polynomial: &mut Polynomial, exponents: Exponents, coefficient: u64, p: u64) {
    let result = add_mod(*polynomial.get(&exponents).unwrap_or(&0), coefficient, p);
    if result == 0 {
        polynomial.remove(&exponents);
    } else {
        polynomial.insert(exponents, result);
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub modulus: u64,
    pub variables: usize,
    polynomials: Vec<Polynomial>,
}

impl Problem {
    pub fn from_value(value: &Value) -> Result<Self, ReplayError> {
        let modulus = unsigned(member(value, "p")?, "p")?;
        if modulus < 2 {
            return Err(error("p must be at least 2"));
        }
        let variables = index(member(value, "n")?, "n")?;
        if !(1..=MAX_VARIABLES).contains(&variables) {
            return Err(error(format!("n must be in 1..={MAX_VARIABLES}")));
        }
        let inputs = member(value, "polynomials")?
            .as_array()
            .ok_or_else(|| error("polynomials must be an array"))?;
        if inputs.len() > MAX_NODES {
            return Err(error("too many input polynomials"));
        }
        let mut polynomials = Vec::with_capacity(inputs.len());
        for input in inputs {
            let terms = input
                .as_array()
                .ok_or_else(|| error("each input polynomial must be an array"))?;
            if terms.len() > MAX_TERMS {
                return Err(error("too many input terms"));
            }
            let mut polynomial = Polynomial::new();
            for term in terms {
                let pair = term
                    .as_array()
                    .filter(|pair| pair.len() == 2)
                    .ok_or_else(|| error("a term must be [coefficient, exponent_vector]"))?;
                let coefficient = input_coefficient(&pair[0], modulus)?;
                let powers = pair[1]
                    .as_array()
                    .filter(|powers| powers.len() == variables)
                    .ok_or_else(|| error("exponent vector has the wrong variable count"))?;
                let mut exponents = Vec::with_capacity(variables);
                for power in powers {
                    exponents.push(
                        u32::try_from(unsigned(power, "exponent")?)
                            .map_err(|_| error("exponent exceeds the unsigned 32-bit range"))?,
                    );
                }
                insert_term(&mut polynomial, exponents, coefficient, modulus);
            }
            polynomials.push(polynomial);
        }
        Ok(Self {
            modulus,
            variables,
            polynomials,
        })
    }

    /// Match Python's canonical normalized JSON exactly, without invoking Python.
    pub fn canonical_json(&self) -> String {
        let polynomials: Vec<Vec<(u64, &Exponents)>> = self
            .polynomials
            .iter()
            .map(|polynomial| {
                polynomial
                    .iter()
                    .map(|(exponents, coefficient)| (*coefficient, exponents))
                    .collect()
            })
            .collect();
        // Python sort_keys=True orders the three keys as n,p,polynomials.
        format!(
            "{{\"n\":{},\"p\":{},\"polynomials\":{}}}",
            self.variables,
            self.modulus,
            serde_json::to_string(&polynomials).expect("integer tuples are serializable")
        )
    }

    pub fn digest(&self) -> String {
        let digest = Sha256::digest(self.canonical_json().as_bytes());
        digest.iter().map(|byte| format!("{byte:02x}")).collect()
    }
}

fn degree(polynomial: &Polynomial) -> u64 {
    polynomial
        .keys()
        .map(|powers| powers.iter().map(|power| u64::from(*power)).sum())
        .max()
        .unwrap_or(0)
}

fn prior<'a>(
    values: &'a [Polynomial],
    node: &Value,
    key: &str,
) -> Result<&'a Polynomial, ReplayError> {
    let reference = index(member(node, key)?, key)?;
    values
        .get(reference)
        .ok_or_else(|| error(format!("{key} must reference an earlier node")))
}

fn trace_coefficient(node: &Value, p: u64) -> Result<u64, ReplayError> {
    let coefficient = unsigned(member(node, "coefficient")?, "coefficient")?;
    if coefficient >= p {
        return Err(error(
            "trace coefficient must be canonical: 0 <= coefficient < p",
        ));
    }
    Ok(coefficient)
}

/// Replay all recorded nodes, including unused nodes, against an external input.
pub fn verify(expected: &Value, certificate: &Value) -> Result<Value, ReplayError> {
    let problem = Problem::from_value(expected)?;
    let p = problem.modulus;
    if member(certificate, "schema")?.as_str() != Some("bounded-degree-refutation-v1") {
        return Err(error("unsupported proof schema"));
    }
    let bound = unsigned(member(certificate, "degree")?, "degree")?;
    if bound > MAX_DEGREE {
        return Err(error(format!(
            "degree exceeds the reference verifier limit {MAX_DEGREE}"
        )));
    }
    let digest = problem.digest();
    if member(certificate, "problem_sha256")?.as_str() != Some(digest.as_str()) {
        return Err(error(
            "proof digest does not match the expected normalized instance",
        ));
    }
    let nodes = member(certificate, "nodes")?
        .as_array()
        .ok_or_else(|| error("nodes must be an array"))?;
    if nodes.len() > MAX_NODES {
        return Err(error("proof exceeds the node-count limit"));
    }
    let mut values: Vec<Polynomial> = Vec::with_capacity(nodes.len());
    let mut maximum_degree = 0;
    for (position, node) in nodes.iter().enumerate() {
        let result = match member(node, "op")?.as_str() {
            Some("input") => {
                let input = index(member(node, "index")?, "input index")?;
                problem
                    .polynomials
                    .get(input)
                    .ok_or_else(|| error("input index is outside the expected problem"))?
                    .clone()
            }
            Some("scale") => {
                let coefficient = trace_coefficient(node, p)?;
                prior(&values, node, "parent")?
                    .iter()
                    .filter_map(|(powers, value)| {
                        let scaled = multiply_mod(*value, coefficient, p);
                        (scaled != 0).then(|| (powers.clone(), scaled))
                    })
                    .collect()
            }
            Some("add") => {
                let coefficient = trace_coefficient(node, p)?;
                let mut result = prior(&values, node, "left")?.clone();
                for (powers, value) in prior(&values, node, "right")? {
                    insert_term(
                        &mut result,
                        powers.clone(),
                        multiply_mod(*value, coefficient, p),
                        p,
                    );
                }
                result
            }
            Some("multiply") => {
                let variable = index(member(node, "variable")?, "variable")?;
                if variable >= problem.variables {
                    return Err(error(
                        "multiplication variable is outside the expected problem",
                    ));
                }
                let mut result = Polynomial::new();
                for (powers, coefficient) in prior(&values, node, "parent")? {
                    let mut powers = powers.clone();
                    powers[variable] = powers[variable]
                        .checked_add(1)
                        .ok_or_else(|| error("exponent overflow"))?;
                    result.insert(powers, *coefficient);
                }
                result
            }
            _ => return Err(error("unsupported proof operation")),
        };
        if result.len() > MAX_TERMS {
            return Err(error(
                "intermediate polynomial exceeds the term-count limit",
            ));
        }
        let actual_degree = degree(&result);
        if actual_degree > bound {
            return Err(error(format!(
                "node {position} has ordinary degree {actual_degree}, exceeding bound {bound}"
            )));
        }
        maximum_degree = maximum_degree.max(actual_degree);
        values.push(result);
    }
    let goal = index(member(certificate, "goal")?, "goal")?;
    let result = values
        .get(goal)
        .ok_or_else(|| error("goal is outside the replayed nodes"))?;
    let one = Polynomial::from([(vec![0; problem.variables], 1)]);
    if result != &one {
        return Err(error("the goal polynomial is not exactly one"));
    }
    Ok(json!({
        "valid_refutation": true,
        "schema": "bounded-degree-refutation-v1",
        "modulus": p,
        "variables": problem.variables,
        "degree_bound": bound,
        "maximum_intermediate_degree": maximum_degree,
        "nodes_replayed": nodes.len(),
        "problem_sha256": digest,
        "arithmetic": "exact u64 residues with u128 modular intermediates",
        "scope": "ideal identity over Z/pZ; field interpretation additionally requires prime p"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple() -> (Value, Value) {
        let problem = json!({"p": 5, "n": 1, "polynomials": [
            [[1,[1]]],
            [[1,[1]],[1,[0]]]
        ]});
        let digest = Problem::from_value(&problem).unwrap().digest();
        let proof = json!({
            "schema":"bounded-degree-refutation-v1", "degree":1,
            "problem_sha256":digest,
            "nodes":[{"op":"input","index":1},{"op":"input","index":0},
                     {"op":"add","left":0,"right":1,"coefficient":4}], "goal":2
        });
        (problem, proof)
    }

    #[test]
    fn accepts_a_valid_identity() {
        let (problem, proof) = simple();
        assert_eq!(
            verify(&problem, &proof).unwrap()["maximum_intermediate_degree"],
            1
        );
    }

    #[test]
    fn canonicalizes_duplicates_negative_coefficients_and_zero_terms() {
        let problem = json!({"p":5,"n":1,"polynomials":[[[1,[9]],[-1,[9]],[-1,[1]],[6,[0]]]]});
        let normalized = Problem::from_value(&problem).unwrap();
        assert_eq!(
            normalized.canonical_json(),
            "{\"n\":1,\"p\":5,\"polynomials\":[[[1,[0]],[4,[1]]]]}"
        );
    }

    #[test]
    fn accepts_equivalent_input_encodings() {
        let (mut problem, proof) = simple();
        problem["polynomials"][0]
            .as_array_mut()
            .unwrap()
            .extend([json!([1, [99]]), json!([-1, [99]])]);
        assert!(verify(&problem, &proof).is_ok());
    }

    #[test]
    fn rejects_lowered_degree_and_instance_substitution() {
        let (mut problem, mut proof) = simple();
        proof["degree"] = json!(0);
        assert!(verify(&problem, &proof).is_err());
        proof["degree"] = json!(1);
        problem["polynomials"][1] = json!([[1, [1]], [2, [0]]]);
        assert!(verify(&problem, &proof).is_err());
    }

    #[test]
    fn rejects_invalid_dag_references() {
        let (problem, mut proof) = simple();
        proof["nodes"][0] = json!({"op":"scale","parent":0,"coefficient":1});
        assert!(verify(&problem, &proof).is_err());
        proof["nodes"][0] = json!({"op":"scale","parent":-1,"coefficient":1});
        assert!(verify(&problem, &proof).is_err());
    }

    #[test]
    fn rejects_forged_digest_nonunit_goals_and_unknown_operations() {
        let (problem, proof) = simple();
        let mut bad = proof.clone();
        bad["problem_sha256"] = json!("0".repeat(64));
        assert!(verify(&problem, &bad).is_err());
        let mut bad = proof.clone();
        bad["goal"] = json!(0);
        assert!(verify(&problem, &bad).is_err());
        let mut bad = proof.clone();
        bad["goal"] = json!(-1);
        assert!(verify(&problem, &bad).is_err());
        let mut bad = proof;
        bad["nodes"][2]["op"] = json!("trust_me");
        assert!(verify(&problem, &bad).is_err());
    }

    #[test]
    fn rejects_an_object_disguised_as_a_number() {
        let problem: Value = serde_json::from_str(
            r#"{"p":{"$serde_json::private::Number":"5"},"n":1,"polynomials":[]}"#,
        )
        .unwrap();
        assert!(Problem::from_value(&problem).is_err());
    }

    #[test]
    fn rejects_out_of_range_inputs_coefficients_and_variables() {
        let (problem, proof) = simple();
        let mut bad = proof.clone();
        bad["nodes"][0]["index"] = json!(50);
        assert!(verify(&problem, &bad).is_err());
        let mut bad = proof.clone();
        bad["nodes"][2]["coefficient"] = json!(5);
        assert!(verify(&problem, &bad).is_err());
        let mut bad = proof;
        bad["nodes"][2] = json!({"op":"multiply","parent":0,"variable":1});
        assert!(verify(&problem, &bad).is_err());
    }

    #[test]
    fn rejects_unused_high_degree_nodes() {
        let (problem, mut proof) = simple();
        proof["nodes"]
            .as_array_mut()
            .unwrap()
            .push(json!({"op":"multiply","parent":1,"variable":0}));
        assert!(verify(&problem, &proof).is_err());
    }

    #[test]
    fn rejects_noninteger_and_oversized_moduli() {
        let (mut problem, proof) = simple();
        for modulus in [json!(1), json!(-5), json!(5.0), json!(true)] {
            problem["p"] = modulus;
            assert!(verify(&problem, &proof).is_err());
        }
        let oversized: Value =
            serde_json::from_str("{\"p\":18446744073709551616,\"n\":1,\"polynomials\":[]}")
                .unwrap();
        assert!(Problem::from_value(&oversized).is_err());
    }

    #[test]
    fn modular_products_do_not_overflow_at_u64_boundary() {
        let p = u64::MAX;
        assert_eq!(multiply_mod(p - 1, p - 1, p), 1);
        assert_eq!(add_mod(p - 1, p - 1, p), p - 2);
    }

    #[test]
    fn replays_the_historical_p101_constructive_certificate() {
        let problem: Value =
            serde_json::from_str(include_str!("../tests/fixtures/p101_problem.json")).unwrap();
        let proof: Value =
            serde_json::from_str(include_str!("../tests/fixtures/p101_proof.json")).unwrap();
        let report = verify(&problem, &proof).unwrap();
        assert_eq!(report["nodes_replayed"], 391);
        assert_eq!(report["maximum_intermediate_degree"], 5);
    }

    #[test]
    fn replays_the_historical_characteristic_five_edge_certificate() {
        let problem: Value =
            serde_json::from_str(include_str!("../tests/fixtures/p5_problem.json")).unwrap();
        let proof: Value =
            serde_json::from_str(include_str!("../tests/fixtures/p5_proof.json")).unwrap();
        let report = verify(&problem, &proof).unwrap();
        assert_eq!(report["maximum_intermediate_degree"], 5);
    }
}
