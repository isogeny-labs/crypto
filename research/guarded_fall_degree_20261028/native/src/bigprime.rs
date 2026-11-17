//! Exact arbitrary-precision replay of `bounded-degree-refutation-v1`.
//!
//! Numeric JSON tokens are retained with `RawValue`, never converted through
//! floating point or strings masquerading as numbers. Normalized input hashing
//! is byte-compatible with the historical small-modulus verifier. Replay proves
//! an ideal identity over Z/pZ; it does not prove primality or curve semantics.

use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use serde::Deserialize;
use serde_json::{json, value::RawValue, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt, fmt::Write};

type Exponents = Vec<u32>;
type Polynomial = BTreeMap<Exponents, BigUint>;

const MAX_VARIABLES: usize = 256;
const MAX_NODES: usize = 1_000_000;
const MAX_TERMS: usize = 1_000_000;
const MAX_DEGREE: u64 = 1_000_000;
const MAX_INTEGER_DIGITS: usize = 10_000;
pub const MAX_DOCUMENT_BYTES: usize = 256 * 1024 * 1024;

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

/// Keeping the token raw avoids both IEEE-754 conversion and serde_json's
/// arbitrary-precision private-number map representation.
#[derive(Debug, Deserialize)]
struct IntegerToken(Box<RawValue>);

impl IntegerToken {
    fn magnitude(&self, label: &str, signed: bool) -> Result<(bool, BigUint), ReplayError> {
        let token = self.0.get();
        let negative = token.starts_with('-');
        let digits = token.strip_prefix('-').unwrap_or(token);
        if (!signed && negative)
            || digits.is_empty()
            || digits.len() > MAX_INTEGER_DIGITS
            || !digits.bytes().all(|byte| byte.is_ascii_digit())
            || (digits.len() > 1 && digits.starts_with('0'))
            || (negative && digits == "0")
        {
            return Err(error(format!(
                "{label} must be a {}decimal JSON integer with at most {MAX_INTEGER_DIGITS} digits",
                if signed { "signed " } else { "nonnegative " }
            )));
        }
        let magnitude = BigUint::parse_bytes(digits.as_bytes(), 10)
            .ok_or_else(|| error(format!("invalid decimal integer for {label}")))?;
        Ok((negative, magnitude))
    }

    fn unsigned(&self, label: &str) -> Result<BigUint, ReplayError> {
        self.magnitude(label, false).map(|(_, value)| value)
    }

    fn u64(&self, label: &str) -> Result<u64, ReplayError> {
        self.unsigned(label)?
            .to_u64()
            .ok_or_else(|| error(format!("{label} must fit an unsigned 64-bit integer")))
    }

    fn index(&self, label: &str) -> Result<usize, ReplayError> {
        usize::try_from(self.u64(label)?)
            .map_err(|_| error(format!("{label} does not fit this platform")))
    }

    fn input_coefficient(&self, p: &BigUint) -> Result<BigUint, ReplayError> {
        let (negative, magnitude) = self.magnitude("input coefficient", true)?;
        let reduced = magnitude % p;
        Ok(if negative && !reduced.is_zero() {
            p - reduced
        } else {
            reduced
        })
    }
}

#[derive(Deserialize)]
struct RawProblem {
    p: IntegerToken,
    n: IntegerToken,
    polynomials: Vec<Vec<(IntegerToken, Vec<IntegerToken>)>>,
}

#[derive(Deserialize)]
struct RawNode {
    op: String,
    index: Option<IntegerToken>,
    parent: Option<IntegerToken>,
    left: Option<IntegerToken>,
    right: Option<IntegerToken>,
    coefficient: Option<IntegerToken>,
    variable: Option<IntegerToken>,
}

#[derive(Deserialize)]
struct RawCertificate {
    schema: String,
    degree: IntegerToken,
    problem_sha256: String,
    nodes: Vec<RawNode>,
    goal: IntegerToken,
}

fn parse<'a, T: Deserialize<'a>>(input: &'a str, label: &str) -> Result<T, ReplayError> {
    if input.len() > MAX_DOCUMENT_BYTES {
        return Err(error(format!("{label} exceeds the document byte limit")));
    }
    serde_json::from_str(input).map_err(|err| error(format!("invalid {label} JSON: {err}")))
}

fn insert_term(polynomial: &mut Polynomial, powers: Exponents, coefficient: BigUint, p: &BigUint) {
    let value = (polynomial.get(&powers).cloned().unwrap_or_default() + coefficient) % p;
    if value.is_zero() {
        polynomial.remove(&powers);
    } else {
        polynomial.insert(powers, value);
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub modulus: BigUint,
    pub variables: usize,
    polynomials: Vec<Polynomial>,
}

impl Problem {
    pub fn from_json(input: &str) -> Result<Self, ReplayError> {
        let raw: RawProblem = parse(input, "problem")?;
        let modulus = raw.p.unsigned("p")?;
        if modulus < BigUint::from(2u8) {
            return Err(error("p must be at least 2"));
        }
        let variables = raw.n.index("n")?;
        if !(1..=MAX_VARIABLES).contains(&variables) {
            return Err(error(format!("n must be in 1..={MAX_VARIABLES}")));
        }
        if raw.polynomials.len() > MAX_NODES {
            return Err(error("too many input polynomials"));
        }
        let mut polynomials = Vec::with_capacity(raw.polynomials.len());
        for terms in raw.polynomials {
            if terms.len() > MAX_TERMS {
                return Err(error("too many input terms"));
            }
            let mut polynomial = Polynomial::new();
            for (coefficient, powers) in terms {
                if powers.len() != variables {
                    return Err(error("exponent vector has the wrong variable count"));
                }
                let exponents = powers
                    .iter()
                    .map(|power| {
                        u32::try_from(power.u64("exponent")?)
                            .map_err(|_| error("exponent exceeds the unsigned 32-bit range"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                insert_term(
                    &mut polynomial,
                    exponents,
                    coefficient.input_coefficient(&modulus)?,
                    &modulus,
                );
            }
            polynomials.push(polynomial);
        }
        Ok(Self {
            modulus,
            variables,
            polynomials,
        })
    }

    /// Canonical numeric JSON, including arbitrary-size decimal coefficients.
    pub fn canonical_json(&self) -> String {
        let mut output = format!(
            "{{\"n\":{},\"p\":{},\"polynomials\":[",
            self.variables, self.modulus
        );
        for (i, polynomial) in self.polynomials.iter().enumerate() {
            if i != 0 {
                output.push(',');
            }
            output.push('[');
            for (j, (powers, coefficient)) in polynomial.iter().enumerate() {
                if j != 0 {
                    output.push(',');
                }
                write!(output, "[{coefficient},[").expect("writing to a String cannot fail");
                for (k, exponent) in powers.iter().enumerate() {
                    if k != 0 {
                        output.push(',');
                    }
                    write!(output, "{exponent}").expect("writing to a String cannot fail");
                }
                output.push_str("]]");
            }
            output.push(']');
        }
        output.push_str("]}");
        output
    }

    pub fn digest(&self) -> String {
        hex::encode(Sha256::digest(self.canonical_json().as_bytes()))
    }
}

fn degree(polynomial: &Polynomial) -> u64 {
    polynomial
        .keys()
        .map(|powers| powers.iter().map(|power| u64::from(*power)).sum())
        .max()
        .unwrap_or(0)
}

fn required<'a>(
    field: &'a Option<IntegerToken>,
    name: &str,
) -> Result<&'a IntegerToken, ReplayError> {
    field
        .as_ref()
        .ok_or_else(|| error(format!("missing object member {name}")))
}

fn prior<'a>(
    values: &'a [Polynomial],
    reference: &Option<IntegerToken>,
    label: &str,
) -> Result<&'a Polynomial, ReplayError> {
    let position = required(reference, label)?.index(label)?;
    values
        .get(position)
        .ok_or_else(|| error(format!("{label} must reference an earlier node")))
}

fn trace_coefficient(node: &RawNode, p: &BigUint) -> Result<BigUint, ReplayError> {
    let coefficient = required(&node.coefficient, "coefficient")?.unsigned("coefficient")?;
    if &coefficient >= p {
        return Err(error(
            "trace coefficient must be canonical: 0 <= coefficient < p",
        ));
    }
    Ok(coefficient)
}

/// Replay every node, including unused nodes, against separately supplied JSON.
pub fn verify_json(expected: &str, certificate: &str) -> Result<Value, ReplayError> {
    let problem = Problem::from_json(expected)?;
    let proof: RawCertificate = parse(certificate, "proof")?;
    if proof.schema != "bounded-degree-refutation-v1" {
        return Err(error("unsupported proof schema"));
    }
    let bound = proof.degree.u64("degree")?;
    if bound > MAX_DEGREE {
        return Err(error(format!(
            "degree exceeds the verifier limit {MAX_DEGREE}"
        )));
    }
    let digest = problem.digest();
    if proof.problem_sha256 != digest {
        return Err(error(
            "proof digest does not match the expected normalized instance",
        ));
    }
    if proof.nodes.len() > MAX_NODES {
        return Err(error("proof exceeds the node-count limit"));
    }
    let p = &problem.modulus;
    let mut values: Vec<Polynomial> = Vec::with_capacity(proof.nodes.len());
    let mut maximum_degree = 0;
    for (position, node) in proof.nodes.iter().enumerate() {
        let result = match node.op.as_str() {
            "input" => {
                let index = required(&node.index, "index")?.index("input index")?;
                problem
                    .polynomials
                    .get(index)
                    .ok_or_else(|| error("input index is outside the expected problem"))?
                    .clone()
            }
            "scale" => {
                let coefficient = trace_coefficient(node, p)?;
                prior(&values, &node.parent, "parent")?
                    .iter()
                    .filter_map(|(powers, value)| {
                        let scaled = (value * &coefficient) % p;
                        (!scaled.is_zero()).then(|| (powers.clone(), scaled))
                    })
                    .collect()
            }
            "add" => {
                let coefficient = trace_coefficient(node, p)?;
                let mut result = prior(&values, &node.left, "left")?.clone();
                for (powers, value) in prior(&values, &node.right, "right")? {
                    insert_term(&mut result, powers.clone(), (value * &coefficient) % p, p);
                }
                result
            }
            "multiply" => {
                let variable = required(&node.variable, "variable")?.index("variable")?;
                if variable >= problem.variables {
                    return Err(error(
                        "multiplication variable is outside the expected problem",
                    ));
                }
                let mut result = Polynomial::new();
                for (powers, coefficient) in prior(&values, &node.parent, "parent")? {
                    let mut powers = powers.clone();
                    powers[variable] = powers[variable]
                        .checked_add(1)
                        .ok_or_else(|| error("exponent overflow"))?;
                    result.insert(powers, coefficient.clone());
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
    let goal = proof.goal.index("goal")?;
    let result = values
        .get(goal)
        .ok_or_else(|| error("goal is outside the replayed nodes"))?;
    let one = Polynomial::from([(vec![0; problem.variables], BigUint::one())]);
    if result != &one {
        return Err(error("the goal polynomial is not exactly one"));
    }
    Ok(json!({
        "valid_refutation": true,
        "schema": "bounded-degree-refutation-v1",
        "modulus": p.to_str_radix(10),
        "modulus_encoding": "decimal_string",
        "modulus_bits": p.bits(),
        "variables": problem.variables,
        "degree_bound": bound,
        "maximum_intermediate_degree": maximum_degree,
        "nodes_replayed": proof.nodes.len(),
        "problem_sha256": digest,
        "arithmetic": "exact arbitrary-precision BigUint modular arithmetic",
        "scope": "ideal identity over Z/pZ; field interpretation additionally requires prime p"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECP_P: &str =
        "115792089237316195423570985008687907853269984665640564039457584007908834671663";

    fn simple(p: &str) -> (String, String) {
        let problem =
            format!("{{\"p\":{p},\"n\":1,\"polynomials\":[[[1,[1]]],[[1,[1]],[1,[0]]]]}}");
        let parsed = Problem::from_json(&problem).unwrap();
        let coefficient = &parsed.modulus - BigUint::one();
        let proof = format!(
            "{{\"schema\":\"bounded-degree-refutation-v1\",\"degree\":1,\"problem_sha256\":\"{}\",\"nodes\":[{{\"op\":\"input\",\"index\":1}},{{\"op\":\"input\",\"index\":0}},{{\"op\":\"add\",\"left\":0,\"right\":1,\"coefficient\":{coefficient}}}],\"goal\":2}}",
            parsed.digest()
        );
        (problem, proof)
    }

    #[test]
    fn accepts_256_bit_modulus_without_rounding() {
        let (problem, proof) = simple(SECP_P);
        let report = verify_json(&problem, &proof).unwrap();
        assert_eq!(report["modulus"], SECP_P);
        assert_eq!(report["modulus_bits"], 256);
        assert_eq!(report["maximum_intermediate_degree"], 1);
    }

    #[test]
    fn arbitrary_precision_product_is_reduced_exactly() {
        let p = BigUint::parse_bytes(SECP_P.as_bytes(), 10).unwrap();
        let minus_one = &p - BigUint::one();
        let problem = format!("{{\"p\":{p},\"n\":1,\"polynomials\":[[[{minus_one},[0]]]]}}");
        let digest = Problem::from_json(&problem).unwrap().digest();
        let proof = format!("{{\"schema\":\"bounded-degree-refutation-v1\",\"degree\":0,\"problem_sha256\":\"{digest}\",\"nodes\":[{{\"op\":\"input\",\"index\":0}},{{\"op\":\"scale\",\"parent\":0,\"coefficient\":{minus_one}}}],\"goal\":1}}");
        assert!(verify_json(&problem, &proof).is_ok());
    }

    #[test]
    fn canonical_binding_agrees_with_existing_small_verifier() {
        for (problem, proof) in [
            (
                include_str!("../tests/fixtures/p101_problem.json"),
                include_str!("../tests/fixtures/p101_proof.json"),
            ),
            (
                include_str!("../tests/fixtures/p5_problem.json"),
                include_str!("../tests/fixtures/p5_proof.json"),
            ),
        ] {
            let expected: Value = serde_json::from_str(problem).unwrap();
            let certificate: Value = serde_json::from_str(proof).unwrap();
            let small = crate::Problem::from_value(&expected).unwrap();
            let big = Problem::from_json(problem).unwrap();
            assert_eq!(small.canonical_json(), big.canonical_json());
            assert_eq!(small.digest(), big.digest());
            let a = crate::verify(&expected, &certificate).unwrap();
            let b = verify_json(problem, proof).unwrap();
            for field in [
                "degree_bound",
                "maximum_intermediate_degree",
                "nodes_replayed",
            ] {
                assert_eq!(a[field], b[field]);
            }
        }
    }

    #[test]
    fn signed_huge_coefficients_cancel_before_degree_filter() {
        let p = BigUint::parse_bytes(SECP_P.as_bytes(), 10).unwrap();
        let huge = &p * &p + BigUint::from(7u8);
        let problem = format!("{{\"p\":{p},\"n\":1,\"polynomials\":[[[{huge},[999]],[-{huge},[999]],[-1,[1]],[{p},[0]]]]}}");
        let canonical = Problem::from_json(&problem).unwrap().canonical_json();
        assert_eq!(
            canonical,
            format!(
                "{{\"n\":1,\"p\":{p},\"polynomials\":[[[{},[1]]]]}}",
                &p - BigUint::one()
            )
        );
    }

    #[test]
    fn rejects_floats_strings_boolean_and_disguised_numbers() {
        for value in [
            "5.0",
            "5e0",
            "\"5\"",
            "true",
            "-0",
            "-5",
            "1",
            "{\"$serde_json::private::Number\":\"5\"}",
        ] {
            let problem = format!("{{\"p\":{value},\"n\":1,\"polynomials\":[]}}");
            assert!(Problem::from_json(&problem).is_err(), "accepted p={value}");
        }
    }

    #[test]
    fn rejects_noninteger_input_coefficients_and_wrong_exponents() {
        for coefficient in [
            "1.0",
            "1e0",
            "\"1\"",
            "false",
            "{\"$serde_json::private::Number\":\"1\"}",
        ] {
            let problem =
                format!("{{\"p\":{SECP_P},\"n\":1,\"polynomials\":[[[{coefficient},[1]]]]}}");
            assert!(Problem::from_json(&problem).is_err());
        }
        for exponent in ["1.0", "-1", "4294967296", "\"1\""] {
            let problem =
                format!("{{\"p\":{SECP_P},\"n\":1,\"polynomials\":[[[1,[{exponent}]]]]}}");
            assert!(Problem::from_json(&problem).is_err());
        }
    }

    #[test]
    fn rejects_tampered_problem_digest_goal_and_trace_coefficients() {
        let (problem, proof) = simple(SECP_P);
        assert!(verify_json(&problem.replace("[1,[0]]", "[2,[0]]"), &proof).is_err());
        let digest = Problem::from_json(&problem).unwrap().digest();
        assert!(verify_json(&problem, &proof.replace(&digest, &"0".repeat(64))).is_err());
        assert!(verify_json(&problem, &proof.replace("\"goal\":2", "\"goal\":0")).is_err());
        let p = BigUint::parse_bytes(SECP_P.as_bytes(), 10).unwrap();
        let old = format!("\"coefficient\":{}", &p - BigUint::one());
        for bad in [
            format!("\"coefficient\":{p}"),
            "\"coefficient\":-1".to_owned(),
            "\"coefficient\":1.0".to_owned(),
        ] {
            assert!(verify_json(&problem, &proof.replace(&old, &bad)).is_err());
        }
    }

    #[test]
    fn rejects_forward_references_and_unknown_operations() {
        let (problem, proof) = simple(SECP_P);
        for bad in [
            proof.replace("\"left\":0", "\"left\":2"),
            proof.replace("\"right\":1", "\"right\":-1"),
            proof.replace("\"op\":\"add\"", "\"op\":\"trust_me\""),
            proof.replace("\"degree\":1", "\"degree\":0"),
        ] {
            assert!(verify_json(&problem, &bad).is_err());
        }
    }

    #[test]
    fn rejects_unused_overdegree_nodes() {
        let (problem, proof) = simple(SECP_P);
        let bad = proof.replace(
            "}],\"goal\":2",
            "},{\"op\":\"multiply\",\"parent\":0,\"variable\":0}],\"goal\":2",
        );
        assert!(verify_json(&problem, &bad).is_err());
    }

    #[test]
    fn rejects_wrong_schema_missing_fields_and_invalid_variables() {
        let (problem, proof) = simple(SECP_P);
        for bad in [
            proof.replace("bounded-degree-refutation-v1", "trust-the-author"),
            proof.replace(
                "\"op\":\"input\",\"index\":1",
                "\"op\":\"scale\",\"coefficient\":1",
            ),
            proof.replace(
                "\"op\":\"input\",\"index\":1",
                "\"op\":\"scale\",\"parent\":0",
            ),
            proof.replace(
                "\"op\":\"add\",\"left\":0,\"right\":1",
                "\"op\":\"multiply\",\"parent\":0,\"variable\":1",
            ),
        ] {
            assert!(verify_json(&problem, &bad).is_err());
        }
    }

    #[test]
    fn rejects_integer_tokens_over_the_documented_digit_limit() {
        let modulus = format!("1{}", "0".repeat(MAX_INTEGER_DIGITS));
        let problem = format!("{{\"p\":{modulus},\"n\":1,\"polynomials\":[]}}");
        assert!(Problem::from_json(&problem).is_err());
    }
}
