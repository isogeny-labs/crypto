//! Strict parsing and base-field extraction for pinned msolve output.
//!
//! msolve parametrizes solutions over an algebraic closure.  A finite-field
//! study must therefore extract roots whose parameter is in the base field,
//! reconstruct every coordinate, and replay the resulting assignment against
//! the canonical IR.  A bare msolve SAT/UNSAT status is not enough.

use crate::CanonicalIr;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Value {
    Integer(i64),
    Name(String),
    List(Vec<Self>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Polynomial {
    coefficients: Vec<u64>,
}

impl Polynomial {
    fn evaluate(&self, value: u64, modulus: u64) -> u64 {
        self.coefficients
            .iter()
            .rev()
            .fold(0_u64, |sum, coefficient| {
                add_mod(mul_mod(sum, value, modulus), *coefficient, modulus)
            })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Parametrization {
    modulus: u64,
    quotient_dimension: usize,
    names: Vec<String>,
    linear_form: Vec<u64>,
    elimination: Polynomial,
    denominator: Polynomial,
    coordinates: Vec<Polynomial>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Outcome {
    EmptyAlgebraicClosure,
    PositiveDimensional,
    Finite(Parametrization),
}

/// Canonical summary of base-field models extracted from one msolve result.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExtractedModels {
    pub algebraic_closure_empty: bool,
    pub quotient_dimension: Option<usize>,
    pub base_field_parameter_roots: usize,
    pub full_assignment_count: usize,
    pub extra_solver_variables: Vec<String>,
    pub canonical_assignments: Vec<BTreeMap<String, u64>>,
    pub projected_models: Vec<Vec<u64>>,
}

struct Parser<'a> {
    input: &'a [u8],
    position: usize,
}

impl<'a> Parser<'a> {
    const fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            position: 0,
        }
    }

    fn parse(mut self) -> Result<Value, String> {
        self.skip_whitespace();
        let value = self.parse_value()?;
        self.skip_whitespace();
        self.expect_byte(b':', "top-level result terminator ':'")?;
        self.skip_whitespace();
        if self.position != self.input.len() {
            return Err(format!("trailing msolve output at byte {}", self.position));
        }
        Ok(value)
    }

    fn parse_value(&mut self) -> Result<Value, String> {
        self.skip_whitespace();
        match self.peek() {
            Some(b'[') => self.parse_list(),
            Some(b'\'') => self.parse_name(),
            Some(b'-' | b'0'..=b'9') => self.parse_integer(),
            Some(byte) => Err(format!(
                "unexpected byte '{}' at byte {}",
                char::from(byte),
                self.position
            )),
            None => Err("unexpected end of msolve output".to_owned()),
        }
    }

    fn parse_list(&mut self) -> Result<Value, String> {
        self.expect_byte(b'[', "list opener '['")?;
        self.skip_whitespace();
        let mut values = Vec::new();
        if self.peek() == Some(b']') {
            self.position += 1;
            return Ok(Value::List(values));
        }
        loop {
            values.push(self.parse_value()?);
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => {
                    self.position += 1;
                    self.skip_whitespace();
                    if self.peek() == Some(b']') {
                        return Err(format!("trailing list comma at byte {}", self.position));
                    }
                }
                Some(b']') => {
                    self.position += 1;
                    break;
                }
                Some(byte) => {
                    return Err(format!(
                        "expected ',' or ']', found '{}' at byte {}",
                        char::from(byte),
                        self.position
                    ));
                }
                None => return Err("unterminated msolve list".to_owned()),
            }
        }
        Ok(Value::List(values))
    }

    fn parse_name(&mut self) -> Result<Value, String> {
        self.expect_byte(b'\'', "name opener quote")?;
        let start = self.position;
        while self.peek().is_some_and(is_name_byte) {
            self.position += 1;
        }
        if self.position == start {
            return Err(format!("empty msolve name at byte {start}"));
        }
        if self.peek() != Some(b'\'') {
            return Err(format!(
                "invalid or unterminated msolve name at byte {}",
                self.position
            ));
        }
        let name = std::str::from_utf8(&self.input[start..self.position])
            .map_err(|_| "msolve name is not UTF-8".to_owned())?
            .to_owned();
        self.position += 1;
        Ok(Value::Name(name))
    }

    fn parse_integer(&mut self) -> Result<Value, String> {
        let start = self.position;
        if self.peek() == Some(b'-') {
            self.position += 1;
        }
        let digits = self.position;
        while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
            self.position += 1;
        }
        if self.position == digits {
            return Err(format!("missing integer digits at byte {start}"));
        }
        let token = std::str::from_utf8(&self.input[start..self.position])
            .map_err(|_| "msolve integer is not UTF-8".to_owned())?;
        let integer = token
            .parse::<i64>()
            .map_err(|_| format!("msolve integer is out of range: {token}"))?;
        Ok(Value::Integer(integer))
    }

    fn expect_byte(&mut self, expected: u8, description: &str) -> Result<(), String> {
        if self.peek() != Some(expected) {
            return Err(format!("expected {description} at byte {}", self.position));
        }
        self.position += 1;
        Ok(())
    }

    const fn peek(&self) -> Option<u8> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|byte| byte.is_ascii_whitespace()) {
            self.position += 1;
        }
    }
}

const fn is_name_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn as_list<'a>(value: &'a Value, context: &str) -> Result<&'a [Value], String> {
    if let Value::List(values) = value {
        Ok(values)
    } else {
        Err(format!("{context} must be a list"))
    }
}

fn as_integer(value: &Value, context: &str) -> Result<i64, String> {
    if let Value::Integer(integer) = value {
        Ok(*integer)
    } else {
        Err(format!("{context} must be an integer"))
    }
}

fn to_usize(value: &Value, context: &str) -> Result<usize, String> {
    let integer = as_integer(value, context)?;
    usize::try_from(integer).map_err(|_| format!("{context} must be a nonnegative usize"))
}

fn to_u64(value: &Value, context: &str) -> Result<u64, String> {
    let integer = as_integer(value, context)?;
    u64::try_from(integer).map_err(|_| format!("{context} must be a nonnegative u64"))
}

fn parse_polynomial(value: &Value, modulus: u64, context: &str) -> Result<Polynomial, String> {
    let polynomial = as_list(value, context)?;
    if polynomial.len() != 2 {
        return Err(format!("{context} must have degree and coefficient list"));
    }
    let degree = as_integer(&polynomial[0], &format!("{context} degree"))?;
    let raw_coefficients = as_list(&polynomial[1], &format!("{context} coefficients"))?;
    let coefficients = raw_coefficients
        .iter()
        .enumerate()
        .map(|(index, coefficient)| {
            let coefficient = to_u64(coefficient, &format!("{context} coefficient {index}"))?;
            if coefficient >= modulus {
                return Err(format!(
                    "{context} coefficient {index} is not canonical modulo {modulus}"
                ));
            }
            Ok(coefficient)
        })
        .collect::<Result<Vec<_>, String>>()?;
    if degree == -1 {
        if coefficients != [0] {
            return Err(format!("{context} noncanonical zero polynomial"));
        }
        return Ok(Polynomial {
            coefficients: vec![0],
        });
    }
    let degree = usize::try_from(degree)
        .map_err(|_| format!("{context} degree must be -1 or nonnegative"))?;
    if coefficients.len() != degree + 1 {
        return Err(format!(
            "{context} degree {degree} disagrees with {} coefficients",
            coefficients.len()
        ));
    }
    if coefficients.last() == Some(&0) {
        return Err(format!("{context} has a zero leading coefficient"));
    }
    Ok(Polynomial { coefficients })
}

fn parse_outcome(output: &str) -> Result<Outcome, String> {
    let root = Parser::new(output).parse()?;
    let top = as_list(&root, "top-level msolve result")?;
    if top == [Value::Integer(-1)] {
        return Ok(Outcome::EmptyAlgebraicClosure);
    }
    if top.len() == 4
        && top[0] == Value::Integer(1)
        && top[2] == Value::Integer(-1)
        && top[3] == Value::List(Vec::new())
    {
        let _ = to_usize(&top[1], "positive-dimensional variable count")?;
        return Ok(Outcome::PositiveDimensional);
    }
    if top.len() != 2 || top[0] != Value::Integer(0) {
        return Err("unrecognized top-level msolve outcome".to_owned());
    }
    let payload = as_list(&top[1], "finite msolve payload")?;
    Ok(Outcome::Finite(parse_parametrization(payload)?))
}

fn parse_parametrization(payload: &[Value]) -> Result<Parametrization, String> {
    if payload.len() != 6 {
        return Err(format!(
            "finite msolve payload has {} fields; expected 6",
            payload.len()
        ));
    }
    let modulus = to_u64(&payload[0], "msolve characteristic")?;
    if modulus < 2 {
        return Err("msolve characteristic must be at least 2".to_owned());
    }
    let variable_count = to_usize(&payload[1], "msolve variable count")?;
    if variable_count == 0 {
        return Err("finite msolve result must have a variable".to_owned());
    }
    let quotient_dimension = to_usize(&payload[2], "msolve quotient dimension")?;
    if quotient_dimension == 0 {
        return Err("finite msolve quotient dimension must be positive".to_owned());
    }
    let raw_names = as_list(&payload[3], "msolve variable names")?;
    if raw_names.len() != variable_count {
        return Err("msolve variable count disagrees with names".to_owned());
    }
    let names = raw_names
        .iter()
        .map(|value| {
            if let Value::Name(name) = value {
                Ok(name.clone())
            } else {
                Err("msolve variable name must be quoted".to_owned())
            }
        })
        .collect::<Result<Vec<_>, String>>()?;
    if names.iter().collect::<BTreeSet<_>>().len() != names.len() {
        return Err("msolve variable names are not unique".to_owned());
    }
    let raw_linear_form = as_list(&payload[4], "msolve linear form")?;
    if raw_linear_form.len() != variable_count {
        return Err("msolve variable count disagrees with linear form".to_owned());
    }
    let modulus_i64 =
        i64::try_from(modulus).map_err(|_| "msolve characteristic does not fit i64".to_owned())?;
    let linear_form = raw_linear_form
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let coefficient = as_integer(value, &format!("linear-form coefficient {index}"))?;
            let _ = i32::try_from(coefficient)
                .map_err(|_| format!("linear-form coefficient {index} does not fit i32"))?;
            u64::try_from(coefficient.rem_euclid(modulus_i64))
                .map_err(|_| "normalized linear-form coefficient overflow".to_owned())
        })
        .collect::<Result<Vec<_>, String>>()?;

    let parametrizations = as_list(&payload[5], "msolve parametrization block")?;
    if parametrizations.len() != 2 || parametrizations[0] != Value::Integer(1) {
        return Err("msolve output must contain exactly one parametrization".to_owned());
    }
    let body = as_list(&parametrizations[1], "msolve parametrization body")?;
    if body.len() != 3 {
        return Err("msolve parametrization body must have three fields".to_owned());
    }
    let elimination = parse_polynomial(&body[0], modulus, "elimination polynomial")?;
    let denominator = parse_polynomial(&body[1], modulus, "denominator polynomial")?;
    if denominator.coefficients == [0] {
        return Err("msolve denominator polynomial must be nonzero".to_owned());
    }
    let raw_coordinates = as_list(&body[2], "coordinate list")?;
    if raw_coordinates.len() + 1 != variable_count {
        return Err("msolve coordinate count must be variable count minus one".to_owned());
    }
    let coordinates = raw_coordinates
        .iter()
        .enumerate()
        .map(|(index, wrapped)| {
            let wrapped = as_list(wrapped, &format!("coordinate wrapper {index}"))?;
            if wrapped.len() != 1 {
                return Err(format!(
                    "coordinate wrapper {index} must contain one polynomial"
                ));
            }
            parse_polynomial(&wrapped[0], modulus, &format!("coordinate {index}"))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let elimination_degree = elimination.coefficients.len().saturating_sub(1);
    if elimination_degree == 0 || elimination_degree > quotient_dimension {
        return Err(format!(
            "elimination degree {elimination_degree} is incompatible with quotient dimension {quotient_dimension}"
        ));
    }
    Ok(Parametrization {
        modulus,
        quotient_dimension,
        names,
        linear_form,
        elimination,
        denominator,
        coordinates,
    })
}

fn add_mod(left: u64, right: u64, modulus: u64) -> u64 {
    u64::try_from((u128::from(left) + u128::from(right)) % u128::from(modulus))
        .expect("reduced sum fits u64")
}

fn mul_mod(left: u64, right: u64, modulus: u64) -> u64 {
    u64::try_from((u128::from(left) * u128::from(right)) % u128::from(modulus))
        .expect("reduced product fits u64")
}

fn pow_mod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1_u64;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exponent >>= 1;
    }
    result
}

fn base_field_points(
    parametrization: &Parametrization,
) -> Result<Vec<BTreeMap<String, u64>>, String> {
    let modulus = parametrization.modulus;
    let mut points = Vec::new();
    let mut unique = BTreeSet::new();
    for parameter in 0..modulus {
        if parametrization.elimination.evaluate(parameter, modulus) != 0 {
            continue;
        }
        let denominator = parametrization.denominator.evaluate(parameter, modulus);
        if denominator == 0 {
            return Err(format!(
                "msolve denominator vanishes at base-field parameter {parameter}"
            ));
        }
        let inverse = pow_mod(denominator, modulus - 2, modulus);
        let mut point = BTreeMap::new();
        let mut ordered = Vec::with_capacity(parametrization.names.len());
        for (name, coordinate) in parametrization
            .names
            .iter()
            .take(parametrization.names.len() - 1)
            .zip(&parametrization.coordinates)
        {
            let coordinate = coordinate.evaluate(parameter, modulus);
            let negated = if coordinate == 0 {
                0
            } else {
                modulus - coordinate
            };
            let value = mul_mod(negated, inverse, modulus);
            point.insert(name.clone(), value);
            ordered.push(value);
        }
        let parameter_name = parametrization
            .names
            .last()
            .expect("finite parametrization has at least one name");
        point.insert(parameter_name.clone(), parameter);
        ordered.push(parameter);
        if !unique.insert(ordered) {
            return Err(format!(
                "msolve parametrization repeats a base-field point at parameter {parameter}"
            ));
        }
        points.push(point);
    }
    Ok(points)
}

/// Parse one complete msolve output, extract only base-field points, replay
/// each full canonical assignment, and return the exact projected model set.
///
/// # Errors
///
/// Rejects malformed output, positive-dimensional output, wrong fields,
/// singular parametrizations, unexpected variables, duplicate points, or any
/// assignment that fails canonical replay.
pub fn extract_projected_models(output: &str, ir: &CanonicalIr) -> Result<ExtractedModels, String> {
    ir.validate()?;
    let expected_modulus = ir
        .spec
        .modulus
        .parse::<u64>()
        .map_err(|_| "invalid canonical IR modulus".to_owned())?;
    let outcome = parse_outcome(output)?;
    let Outcome::Finite(parametrization) = outcome else {
        return match outcome {
            Outcome::EmptyAlgebraicClosure => Ok(ExtractedModels {
                algebraic_closure_empty: true,
                quotient_dimension: None,
                base_field_parameter_roots: 0,
                full_assignment_count: 0,
                extra_solver_variables: Vec::new(),
                canonical_assignments: Vec::new(),
                projected_models: Vec::new(),
            }),
            Outcome::PositiveDimensional => {
                Err("msolve reported a positive-dimensional solution set".to_owned())
            }
            Outcome::Finite(_) => unreachable!(),
        };
    };
    if parametrization.modulus != expected_modulus {
        return Err(format!(
            "msolve characteristic {} does not match IR modulus {expected_modulus}",
            parametrization.modulus
        ));
    }
    let expected_names = ir
        .variables
        .iter()
        .map(|variable| variable.name.as_str())
        .collect::<BTreeSet<_>>();
    let actual_names = parametrization
        .names
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if let Some(missing) = expected_names.difference(&actual_names).next() {
        return Err(format!("msolve output omits canonical variable {missing}"));
    }
    let extra_solver_variables = actual_names
        .difference(&expected_names)
        .map(|name| (*name).to_owned())
        .collect::<Vec<_>>();
    if !extra_solver_variables.is_empty() && extra_solver_variables != ["A"] {
        return Err(format!(
            "msolve output has unexpected separating variables: {}",
            extra_solver_variables.join(",")
        ));
    }
    let points = base_field_points(&parametrization)?;
    let mut projected = BTreeSet::new();
    let mut canonical_assignments = Vec::new();
    for point in &points {
        let assignment = ir
            .variables
            .iter()
            .map(|variable| {
                point
                    .get(&variable.name)
                    .copied()
                    .map(|value| (variable.name.clone(), value))
                    .ok_or_else(|| format!("msolve point omits {}", variable.name))
            })
            .collect::<Result<BTreeMap<_, _>, String>>()?;
        let failures = ir.failed_equations(&assignment)?;
        if !failures.is_empty() {
            return Err(format!(
                "msolve base-field point fails canonical equations: {}",
                failures.join(",")
            ));
        }
        projected.insert(
            ir.projected_variables
                .iter()
                .map(|id| assignment[&ir.variables[*id as usize].name])
                .collect::<Vec<_>>(),
        );
        canonical_assignments.push(assignment);
    }
    canonical_assignments.sort();
    Ok(ExtractedModels {
        algebraic_closure_empty: false,
        quotient_dimension: Some(parametrization.quotient_dimension),
        base_field_parameter_roots: points.len(),
        full_assignment_count: points.len(),
        extra_solver_variables,
        canonical_assignments,
        projected_models: projected.into_iter().collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{build_ir, InstanceSpec};
    use serde::Deserialize;

    const MULTY_F3: &str =
        "[0,[3,2,3,['x','y'],[0,1],[1,[[3,[0,2,0,1]],[0,[1]],[[[2,[0,0,2]]]]]]]]:\n";
    const EXTENSION_ONLY_F3: &str = "[0,[3,1,2,['x'],[1],[1,[[2,[1,0,1]],[0,[1]],[]]]]]:\n";
    const NONRADICAL_F31: &str =
        "[0,[31,3,3,['x','y','A'],[-33,97,92],[1,[[1,[0,1]],[0,[1]],[[[-1,[0]]],[[-1,[0]]]]]]]]:\n";

    #[derive(Deserialize)]
    struct Witness {
        p: u64,
        a: u64,
        b: u64,
        m: usize,
        t: usize,
        terminal_c: u64,
        target: [u64; 2],
        assignment: BTreeMap<String, u64>,
    }

    fn known_m3() -> (CanonicalIr, BTreeMap<String, u64>) {
        let witness: Witness = serde_json::from_str(include_str!(
            "../../../prime_one_shot_20261029/prototype/exports/m3_t4_remainder.witness.json"
        ))
        .unwrap();
        let ir = build_ir(InstanceSpec {
            modulus: witness.p,
            curve_a: witness.a,
            curve_b: witness.b,
            arity: witness.m,
            depth: witness.t,
            terminal_c: witness.terminal_c,
            target_x: witness.target[0],
            target_y: witness.target[1],
        })
        .unwrap();
        (ir, witness.assignment)
    }

    fn fixed_parametrization(ir: &CanonicalIr, assignment: &BTreeMap<String, u64>) -> String {
        let mut names = ir
            .variables
            .iter()
            .map(|variable| format!("'{}'", variable.name))
            .collect::<Vec<_>>();
        names.push("'A'".to_owned());
        let mut linear = vec!["0"; ir.variables.len()];
        linear.push("1");
        let coordinates = ir
            .variables
            .iter()
            .map(|variable| {
                let value = assignment[&variable.name];
                let negated = if value == 0 { 0 } else { 17 - value };
                if negated == 0 {
                    "[[-1,[0]]]".to_owned()
                } else {
                    format!("[[0,[{negated}]]]")
                }
            })
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "[0,[17,{},1,[{}],[{}],[1,[[1,[16,1]],[0,[1]],[{}]]]]]:\n",
            ir.variables.len() + 1,
            names.join(","),
            linear.join(","),
            coordinates
        )
    }

    #[test]
    fn finite_field_parametrization_recovers_coordinate_sign() {
        let Outcome::Finite(parametrization) = parse_outcome(MULTY_F3).unwrap() else {
            panic!("expected finite parametrization");
        };
        let points = base_field_points(&parametrization).unwrap();
        let observed = points
            .iter()
            .map(|point| (point["x"], point["y"]))
            .collect::<BTreeSet<_>>();
        assert_eq!(observed, BTreeSet::from([(0, 0), (1, 1), (1, 2)]));
    }

    #[test]
    fn printed_linear_form_coefficients_are_normalized() {
        let output = MULTY_F3.replacen("[0,1]", "[4,1]", 1);
        let Outcome::Finite(parametrization) = parse_outcome(&output).unwrap() else {
            panic!("expected finite parametrization");
        };
        assert_eq!(parametrization.linear_form, [1, 1]);
    }

    #[test]
    fn nonradical_quotient_dimension_may_exceed_elimination_degree() {
        let Outcome::Finite(parametrization) = parse_outcome(NONRADICAL_F31).unwrap() else {
            panic!("expected finite parametrization");
        };
        assert_eq!(parametrization.quotient_dimension, 3);
        assert_eq!(parametrization.elimination.coefficients.len() - 1, 1);
        assert_eq!(base_field_points(&parametrization).unwrap().len(), 1);
    }

    #[test]
    fn algebraic_closure_roots_without_base_field_roots_are_filtered() {
        let Outcome::Finite(parametrization) = parse_outcome(EXTENSION_ONLY_F3).unwrap() else {
            panic!("expected finite parametrization");
        };
        assert_eq!(parametrization.quotient_dimension, 2);
        assert!(base_field_points(&parametrization).unwrap().is_empty());
    }

    #[test]
    fn known_assignment_is_extracted_and_replayed() {
        let (ir, assignment) = known_m3();
        let output = fixed_parametrization(&ir, &assignment);
        let extracted = extract_projected_models(&output, &ir).unwrap();
        let expected = ir
            .projected_variables
            .iter()
            .map(|id| assignment[&ir.variables[*id as usize].name])
            .collect::<Vec<_>>();
        assert_eq!(extracted.projected_models, vec![expected]);
        assert_eq!(extracted.base_field_parameter_roots, 1);
        assert_eq!(extracted.extra_solver_variables, vec!["A"]);
        assert_eq!(extracted.canonical_assignments, vec![assignment]);
    }

    #[test]
    fn empty_and_positive_dimensional_outcomes_are_distinguished() {
        let (ir, _) = known_m3();
        let empty = extract_projected_models("[-1]:\n", &ir).unwrap();
        assert!(empty.algebraic_closure_empty);
        assert!(empty.projected_models.is_empty());
        assert!(extract_projected_models("[1,3,-1,[]]:\n", &ir).is_err());
    }

    #[test]
    fn malformed_outputs_fail_closed() {
        let mutations = [
            MULTY_F3.trim_end_matches(['\n', ':']),
            "[0,[3,2,3,['x','y'],[0,1],[1,[[3,[0,2,0,1]],[0,[1]],[[[2,[0,0,2]]]]]]]]]: trailing",
            "[0,[3,2,3,['x','x'],[0,1],[1,[[3,[0,2,0,1]],[0,[1]],[[[2,[0,0,2]]]]]]]]]:",
            "[0,[3,2,3,['x','y'],[0,1],[1,[[2,[0,2,0,1]],[0,[1]],[[[2,[0,0,2]]]]]]]]]:",
            "[0,[3,2,3,['x','y'],[0,1],[1,[[3,[0,3,0,1]],[0,[1]],[[[2,[0,0,2]]]]]]]]]:",
            "[0,[3,2,3,['x','y'],[0,1],[1,[[3,[0,2,0,1]],[0,[1]],[]]]]]]:",
        ];
        for mutation in mutations {
            assert!(parse_outcome(mutation).is_err(), "accepted: {mutation}");
        }
    }
}
