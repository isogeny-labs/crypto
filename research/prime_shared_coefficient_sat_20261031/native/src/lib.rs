//! Canonical quadratic IR for the prime-field shared-coefficient experiment.
//!
//! This crate is development infrastructure.  It does not claim a solver
//! speedup and deliberately contains no timing-based acceptance decision.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

pub mod boolean;
pub mod e1;
pub mod e1_msolve;
pub mod msolve;

pub const IR_SCHEMA: &str = "prime-shared-coefficient.ir/v1";
const CONSTRUCTION: &str = "odd-arity-selected-chart-remainder-tower";
const MAX_ARITY: usize = 255;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InstanceSpec {
    pub modulus: u64,
    pub curve_a: u64,
    pub curve_b: u64,
    pub arity: usize,
    pub depth: usize,
    pub terminal_c: u64,
    pub target_x: u64,
    pub target_y: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalSpec {
    pub modulus: String,
    pub curve_a: String,
    pub curve_b: String,
    pub arity: u32,
    pub depth: u32,
    pub terminal_c: String,
    pub target_x: String,
    pub target_y: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VariableRole {
    DivisorA,
    DivisorC,
    RootPolynomial,
    RemainderQuotient,
    Remainder,
    CoprimeWitness,
    SquarefreeWitness,
    TargetSeparationWitness,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Variable {
    pub id: u32,
    pub name: String,
    pub role: VariableRole,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Power {
    pub variable: u32,
    pub exponent: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Term {
    pub coefficient: String,
    pub powers: Vec<Power>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Equation {
    pub label: String,
    pub terms: Vec<Term>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalIr {
    pub schema: String,
    pub version: u32,
    pub construction: String,
    pub spec: CanonicalSpec,
    pub variables: Vec<Variable>,
    pub equations: Vec<Equation>,
    pub projected_variables: Vec<u32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PanelSummary {
    pub schema: String,
    pub kind: String,
    pub modulus: u64,
    pub curve: [u64; 2],
    pub arity: usize,
    pub depth: usize,
    pub terminal_c: u64,
    pub targets: usize,
    pub sat_targets: usize,
    pub unsat_targets: usize,
    pub candidate_models: usize,
    pub oracle_models: usize,
    pub exact_model_set_matches: usize,
    pub timing_collected: bool,
}

type Monomial = Vec<(u32, u8)>;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Expr(BTreeMap<Monomial, u64>);

impl Expr {
    fn constant(value: u64, modulus: u64) -> Self {
        let value = value % modulus;
        if value == 0 {
            Self::default()
        } else {
            Self(BTreeMap::from([(Vec::new(), value)]))
        }
    }

    fn variable(id: u32) -> Self {
        Self(BTreeMap::from([(vec![(id, 1)], 1)]))
    }

    fn add(&self, other: &Self, modulus: u64) -> Self {
        let mut terms = self.0.clone();
        for (monomial, coefficient) in &other.0 {
            let sum = add_mod(*terms.get(monomial).unwrap_or(&0), *coefficient, modulus);
            if sum == 0 {
                terms.remove(monomial);
            } else {
                terms.insert(monomial.clone(), sum);
            }
        }
        Self(terms)
    }

    fn neg(&self, modulus: u64) -> Self {
        Self(
            self.0
                .iter()
                .map(|(monomial, coefficient)| {
                    (
                        monomial.clone(),
                        if *coefficient == 0 {
                            0
                        } else {
                            modulus - coefficient
                        },
                    )
                })
                .collect(),
        )
    }

    fn sub(&self, other: &Self, modulus: u64) -> Self {
        self.add(&other.neg(modulus), modulus)
    }

    fn scale(&self, scalar: u64, modulus: u64) -> Self {
        let scalar = scalar % modulus;
        if scalar == 0 {
            return Self::default();
        }
        Self(
            self.0
                .iter()
                .filter_map(|(monomial, coefficient)| {
                    let coefficient = mul_mod(*coefficient, scalar, modulus);
                    (coefficient != 0).then(|| (monomial.clone(), coefficient))
                })
                .collect(),
        )
    }

    fn mul(&self, other: &Self, modulus: u64) -> Result<Self, String> {
        let mut terms = BTreeMap::new();
        for (left_monomial, left_coefficient) in &self.0 {
            for (right_monomial, right_coefficient) in &other.0 {
                let monomial = multiply_monomials(left_monomial, right_monomial)?;
                let product = mul_mod(*left_coefficient, *right_coefficient, modulus);
                let sum = add_mod(*terms.get(&monomial).unwrap_or(&0), product, modulus);
                if sum == 0 {
                    terms.remove(&monomial);
                } else {
                    terms.insert(monomial, sum);
                }
            }
        }
        Ok(Self(terms))
    }

    #[cfg(test)]
    fn evaluate(&self, assignment: &[u64], modulus: u64) -> u64 {
        self.0.iter().fold(0, |sum, (monomial, coefficient)| {
            let value = monomial
                .iter()
                .fold(*coefficient, |value, (variable, exponent)| {
                    mul_mod(
                        value,
                        pow_mod(
                            assignment[*variable as usize],
                            u64::from(*exponent),
                            modulus,
                        ),
                        modulus,
                    )
                });
            add_mod(sum, value, modulus)
        })
    }

    fn into_terms(self) -> Vec<Term> {
        self.0
            .into_iter()
            .map(|(monomial, coefficient)| Term {
                coefficient: coefficient.to_string(),
                powers: monomial
                    .into_iter()
                    .map(|(variable, exponent)| Power { variable, exponent })
                    .collect(),
            })
            .collect()
    }
}

fn multiply_monomials(left: &Monomial, right: &Monomial) -> Result<Monomial, String> {
    let mut exponents = BTreeMap::<u32, u8>::new();
    for (variable, exponent) in left.iter().chain(right) {
        let entry = exponents.entry(*variable).or_default();
        *entry = entry
            .checked_add(*exponent)
            .ok_or_else(|| "monomial exponent overflow".to_owned())?;
    }
    let degree: u16 = exponents.values().map(|value| u16::from(*value)).sum();
    if degree > 2 || exponents.values().any(|value| *value > 2) {
        return Err("construction produced a term above quadratic degree".to_owned());
    }
    Ok(exponents.into_iter().collect())
}

type XPoly = Vec<Expr>;

#[derive(Default)]
struct Builder {
    modulus: u64,
    variables: Vec<Variable>,
    equations: Vec<Equation>,
    projected_variables: Vec<u32>,
}

impl Builder {
    fn new(modulus: u64) -> Self {
        Self {
            modulus,
            ..Self::default()
        }
    }

    fn variable(&mut self, name: String, role: VariableRole, projected: bool) -> Expr {
        let id = u32::try_from(self.variables.len()).expect("variable count fits u32");
        self.variables.push(Variable { id, name, role });
        if projected {
            self.projected_variables.push(id);
        }
        Expr::variable(id)
    }

    fn coefficients(
        &mut self,
        prefix: &str,
        count: usize,
        role: VariableRole,
        projected: bool,
        monic: bool,
    ) -> XPoly {
        let mut result = (0..count)
            .map(|index| self.variable(format!("{prefix}{index}"), role, projected))
            .collect::<Vec<_>>();
        if monic {
            result.push(Expr::constant(1, self.modulus));
        }
        result
    }

    fn equation(&mut self, label: String, expression: Expr) {
        if !expression.0.is_empty() {
            self.equations.push(Equation {
                label,
                terms: expression.into_terms(),
            });
        }
    }

    fn equality(&mut self, label: &str, left: &XPoly, right: &XPoly) {
        for index in 0..left.len().max(right.len()) {
            let expression = xat(left, index).sub(xat(right, index), self.modulus);
            self.equation(format!("{label}_{index}"), expression);
        }
    }
}

/// Build the canonical quadratic system for one selected-chart instance.
///
/// # Errors
///
/// Returns an error when the field, curve, target, split coset, or generated
/// quadratic system fails validation.
#[allow(clippy::many_single_char_names)]
pub fn build_ir(spec: InstanceSpec) -> Result<CanonicalIr, String> {
    let ir = build_ir_inner(spec)?;
    ir.validate_structure()?;
    Ok(ir)
}

#[allow(clippy::many_single_char_names)]
fn build_ir_inner(spec: InstanceSpec) -> Result<CanonicalIr, String> {
    validate_spec(spec)?;
    let p = spec.modulus;
    let m = spec.arity;
    let h = m.div_ceil(2);
    let mut builder = Builder::new(p);

    let a = builder.coefficients("a", h, VariableRole::DivisorA, true, true);
    let c = builder.coefficients("c", h - 1, VariableRole::DivisorC, true, false);
    let u = builder.coefficients("u0_", m, VariableRole::RootPolynomial, true, true);

    let target_equation =
        xeval(&a, spec.target_x, p).sub(&xeval(&c, spec.target_x, p).scale(spec.target_y, p), p);
    builder.equation("target_zero".to_owned(), target_equation);

    let curve = xconstant(&[spec.curve_b, spec.curve_a, 0, 1], p);
    let norm = xtimes(&a, &a, p)?.sub_poly(&xtimes(&curve, &xtimes(&c, &c, p)?, p)?, p);
    let target_factor = xconstant(&[sub_mod(0, spec.target_x, p), 1], p);
    builder.equality("norm", &norm, &xtimes(&target_factor, &u, p)?);

    let mut previous = xconstant(&[0, 1], p);
    for layer in 1..=spec.depth {
        let quotient = builder.coefficients(
            &format!("q{layer}_"),
            m - 1,
            VariableRole::RemainderQuotient,
            false,
            false,
        );
        let next = if layer == spec.depth {
            xconstant(&[spec.terminal_c], p)
        } else {
            builder.coefficients(
                &format!("r{layer}_"),
                m,
                VariableRole::Remainder,
                false,
                false,
            )
        };
        let square = xtimes(&previous, &previous, p)?;
        let quotient_identity = xtimes(&quotient, &u, p)?.add_poly(&next, p);
        builder.equality(&format!("remainder_{layer}"), &square, &quotient_identity);
        previous = next;
    }

    let coprime_s = builder.coefficients("bc_s", h - 1, VariableRole::CoprimeWitness, false, false);
    let coprime_t = builder.coefficients("bc_t", m, VariableRole::CoprimeWitness, false, false);
    let coprime = xtimes(&u, &coprime_s, p)?.add_poly(&xtimes(&c, &coprime_t, p)?, p);
    builder.equality("coprime_c", &coprime, &xconstant(&[1], p));

    let derivative = xderivative(&u, p);
    let squarefree_s =
        builder.coefficients("bd_s", m - 1, VariableRole::SquarefreeWitness, false, false);
    let squarefree_t =
        builder.coefficients("bd_t", m, VariableRole::SquarefreeWitness, false, false);
    let squarefree =
        xtimes(&u, &squarefree_s, p)?.add_poly(&xtimes(&derivative, &squarefree_t, p)?, p);
    builder.equality("squarefree_u", &squarefree, &xconstant(&[1], p));

    let inverse = builder.variable(
        "target_inverse".to_owned(),
        VariableRole::TargetSeparationWitness,
        false,
    );
    let separate = inverse
        .mul(&xeval(&u, spec.target_x, p), p)?
        .sub(&Expr::constant(1, p), p);
    builder.equation("target_separate".to_owned(), separate);

    Ok(CanonicalIr {
        schema: IR_SCHEMA.to_owned(),
        version: 1,
        construction: CONSTRUCTION.to_owned(),
        spec: CanonicalSpec::from(spec),
        variables: builder.variables,
        equations: builder.equations,
        projected_variables: builder.projected_variables,
    })
}

impl From<InstanceSpec> for CanonicalSpec {
    fn from(spec: InstanceSpec) -> Self {
        Self {
            modulus: spec.modulus.to_string(),
            curve_a: spec.curve_a.to_string(),
            curve_b: spec.curve_b.to_string(),
            arity: u32::try_from(spec.arity).expect("validated arity fits u32"),
            depth: u32::try_from(spec.depth).expect("validated depth fits u32"),
            terminal_c: spec.terminal_c.to_string(),
            target_x: spec.target_x.to_string(),
            target_y: spec.target_y.to_string(),
        }
    }
}

impl CanonicalSpec {
    fn to_instance(&self) -> Result<InstanceSpec, String> {
        Ok(InstanceSpec {
            modulus: parse_decimal(&self.modulus, "spec.modulus")?,
            curve_a: parse_decimal(&self.curve_a, "spec.curve_a")?,
            curve_b: parse_decimal(&self.curve_b, "spec.curve_b")?,
            arity: usize::try_from(self.arity)
                .map_err(|_| "arity does not fit usize".to_owned())?,
            depth: usize::try_from(self.depth)
                .map_err(|_| "depth does not fit usize".to_owned())?,
            terminal_c: parse_decimal(&self.terminal_c, "spec.terminal_c")?,
            target_x: parse_decimal(&self.target_x, "spec.target_x")?,
            target_y: parse_decimal(&self.target_y, "spec.target_y")?,
        })
    }
}

impl CanonicalIr {
    /// Serialize a validated IR without insignificant whitespace.
    ///
    /// # Errors
    ///
    /// Returns an error if IR validation or JSON serialization fails.
    pub fn canonical_json(&self) -> Result<String, String> {
        self.validate()?;
        serde_json::to_string(self).map_err(|error| error.to_string())
    }

    /// Serialize a validated IR as indented JSON for inspection.
    ///
    /// # Errors
    ///
    /// Returns an error if IR validation or JSON serialization fails.
    pub fn canonical_json_pretty(&self) -> Result<String, String> {
        self.validate()?;
        serde_json::to_string_pretty(self).map_err(|error| error.to_string())
    }

    /// Parse exactly one strict JSON IR document and validate all invariants.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed JSON, unknown fields, trailing input, or
    /// any invalid IR invariant.
    pub fn parse_json(input: &str) -> Result<Self, String> {
        let mut deserializer = serde_json::Deserializer::from_str(input);
        let ir = Self::deserialize(&mut deserializer).map_err(|error| error.to_string())?;
        deserializer.end().map_err(|error| error.to_string())?;
        ir.validate()?;
        Ok(ir)
    }

    /// Hash the compact canonical JSON representation with SHA-256.
    ///
    /// # Errors
    ///
    /// Returns an error when the IR cannot be validated or serialized.
    pub fn sha256(&self) -> Result<String, String> {
        let digest = Sha256::digest(self.canonical_json()?.as_bytes());
        Ok(format!("{digest:x}"))
    }

    /// Validate schema, field, ordering, degree, and projection invariants.
    ///
    /// # Errors
    ///
    /// Returns a descriptive error for the first failed invariant.
    pub fn validate(&self) -> Result<(), String> {
        self.validate_structure()?;
        let expected = build_ir_inner(self.spec.to_instance()?)?;
        expected.validate_structure()?;
        if self != &expected {
            return Err(
                "IR is structurally valid but not the canonical system for its spec".to_owned(),
            );
        }
        Ok(())
    }

    fn validate_structure(&self) -> Result<(), String> {
        if self.schema != IR_SCHEMA || self.version != 1 || self.construction != CONSTRUCTION {
            return Err("unsupported IR schema, version, or construction".to_owned());
        }
        let spec = self.spec.to_instance()?;
        validate_spec(spec)?;
        if self.variables.is_empty() || self.equations.is_empty() {
            return Err("IR must contain variables and equations".to_owned());
        }
        let mut names = BTreeSet::new();
        for (index, variable) in self.variables.iter().enumerate() {
            if variable.id != u32::try_from(index).map_err(|_| "too many variables".to_owned())? {
                return Err("variable IDs must be contiguous and ordered".to_owned());
            }
            if !valid_symbol(&variable.name) || !names.insert(&variable.name) {
                return Err(format!(
                    "invalid or duplicate variable name: {}",
                    variable.name
                ));
            }
        }
        let mut labels = BTreeSet::new();
        for equation in &self.equations {
            if !valid_symbol(&equation.label) || !labels.insert(&equation.label) {
                return Err(format!(
                    "invalid or duplicate equation label: {}",
                    equation.label
                ));
            }
            if equation.terms.is_empty() {
                return Err(format!("empty equation: {}", equation.label));
            }
            let mut previous: Option<Vec<(u32, u8)>> = None;
            for term in &equation.terms {
                let coefficient = parse_decimal(&term.coefficient, "term.coefficient")?;
                if coefficient == 0 || coefficient >= spec.modulus {
                    return Err("term coefficient must be canonical and nonzero".to_owned());
                }
                let monomial = term
                    .powers
                    .iter()
                    .map(|power| (power.variable, power.exponent))
                    .collect::<Vec<_>>();
                if monomial.windows(2).any(|pair| pair[0].0 >= pair[1].0)
                    || monomial.iter().any(|(variable, exponent)| {
                        *variable as usize >= self.variables.len()
                            || *exponent == 0
                            || *exponent > 2
                    })
                    || monomial
                        .iter()
                        .map(|(_, exponent)| u16::from(*exponent))
                        .sum::<u16>()
                        > 2
                {
                    return Err("invalid quadratic monomial".to_owned());
                }
                if previous.as_ref().is_some_and(|value| value >= &monomial) {
                    return Err("equation terms must be strictly ordered".to_owned());
                }
                previous = Some(monomial);
            }
        }
        if self.projected_variables.is_empty()
            || self
                .projected_variables
                .windows(2)
                .any(|pair| pair[0] >= pair[1])
            || self
                .projected_variables
                .iter()
                .any(|id| *id as usize >= self.variables.len())
        {
            return Err("projected variable IDs must be nonempty, ordered, and valid".to_owned());
        }
        Ok(())
    }

    /// Evaluate every equation under one complete canonical assignment.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid IR or an incomplete, unknown, or
    /// noncanonical assignment.
    pub fn failed_equations(
        &self,
        assignment: &BTreeMap<String, u64>,
    ) -> Result<Vec<String>, String> {
        self.validate()?;
        if assignment.len() != self.variables.len() {
            return Err(format!(
                "assignment has {} entries; expected {}",
                assignment.len(),
                self.variables.len()
            ));
        }
        let modulus = parse_decimal(&self.spec.modulus, "spec.modulus")?;
        let mut values = vec![0; self.variables.len()];
        for variable in &self.variables {
            let value = assignment
                .get(&variable.name)
                .ok_or_else(|| format!("missing assignment for {}", variable.name))?;
            if *value >= modulus {
                return Err(format!("noncanonical assignment for {}", variable.name));
            }
            values[variable.id as usize] = *value;
        }
        if let Some(extra) = assignment
            .keys()
            .find(|name| !self.variables.iter().any(|v| &v.name == *name))
        {
            return Err(format!("unknown assignment variable: {extra}"));
        }
        Ok(self
            .equations
            .iter()
            .filter(|equation| evaluate_terms(&equation.terms, &values, modulus) != 0)
            .map(|equation| equation.label.clone())
            .collect())
    }

    /// Export the system in msolve's prime-field polynomial input format.
    ///
    /// # Errors
    ///
    /// Returns an error when IR validation fails.
    pub fn to_msolve(&self) -> Result<String, String> {
        self.validate()?;
        let mut output = String::new();
        output.push_str(
            &self
                .variables
                .iter()
                .map(|v| v.name.as_str())
                .collect::<Vec<_>>()
                .join(","),
        );
        output.push('\n');
        output.push_str(&self.spec.modulus);
        output.push('\n');
        for (index, equation) in self.equations.iter().enumerate() {
            output.push_str(&format_msolve_equation(equation, &self.variables));
            output.push_str(if index + 1 == self.equations.len() {
                "\n"
            } else {
                ",\n"
            });
        }
        Ok(output)
    }

    /// Export the system as a cvc5-compatible `QF_FF` query.
    ///
    /// # Errors
    ///
    /// Returns an error when IR validation or canonical hashing fails.
    pub fn to_qf_ff(&self) -> Result<String, String> {
        self.validate()?;
        let mut output = format!(
            "; generated from {}\n(set-logic QF_FF)\n(set-option :produce-models true)\n(define-sort F () (_ FiniteField {}))\n",
            self.sha256()?,
            self.spec.modulus
        );
        for variable in &self.variables {
            writeln!(output, "(declare-const {} F)", variable.name)
                .expect("String writes cannot fail");
        }
        for equation in &self.equations {
            writeln!(
                output,
                "; {}\n(assert (= {} (as ff0 F)))",
                equation.label,
                format_qf_ff_sum(&equation.terms, &self.variables)
            )
            .expect("String writes cannot fail");
        }
        output.push_str("(check-sat)\n");
        let projected = self
            .projected_variables
            .iter()
            .map(|id| self.variables[*id as usize].name.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(output, "(get-value ({projected}))").expect("String writes cannot fail");
        Ok(output)
    }
}

fn format_msolve_equation(equation: &Equation, variables: &[Variable]) -> String {
    equation
        .terms
        .iter()
        .map(|term| {
            let mut factors = Vec::new();
            let coefficient = term.coefficient.parse::<u64>().expect("validated decimal");
            if coefficient != 1 || term.powers.is_empty() {
                factors.push(term.coefficient.clone());
            }
            factors.extend(term.powers.iter().map(|power| {
                let name = &variables[power.variable as usize].name;
                if power.exponent == 1 {
                    name.clone()
                } else {
                    format!("{name}^{}", power.exponent)
                }
            }));
            factors.join("*")
        })
        .collect::<Vec<_>>()
        .join("+")
}

fn format_qf_ff_sum(terms: &[Term], variables: &[Variable]) -> String {
    let values = terms
        .iter()
        .map(|term| {
            let mut factors = vec![format!("(as ff{} F)", term.coefficient)];
            for power in &term.powers {
                let name = &variables[power.variable as usize].name;
                for _ in 0..power.exponent {
                    factors.push(name.clone());
                }
            }
            if factors.len() == 1 {
                factors.remove(0)
            } else {
                format!("(ff.mul {})", factors.join(" "))
            }
        })
        .collect::<Vec<_>>();
    if values.len() == 1 {
        values[0].clone()
    } else {
        format!("(ff.add {})", values.join(" "))
    }
}

fn evaluate_terms(terms: &[Term], assignment: &[u64], modulus: u64) -> u64 {
    terms.iter().fold(0, |sum, term| {
        let coefficient = term.coefficient.parse::<u64>().expect("validated decimal");
        let value = term.powers.iter().fold(coefficient, |value, power| {
            mul_mod(
                value,
                pow_mod(
                    assignment[power.variable as usize],
                    u64::from(power.exponent),
                    modulus,
                ),
                modulus,
            )
        });
        add_mod(sum, value, modulus)
    })
}

trait XPolyOps {
    fn add_poly(&self, other: &Self, modulus: u64) -> Self;
    fn sub_poly(&self, other: &Self, modulus: u64) -> Self;
}

impl XPolyOps for XPoly {
    fn add_poly(&self, other: &Self, modulus: u64) -> Self {
        (0..self.len().max(other.len()))
            .map(|index| xat(self, index).add(xat(other, index), modulus))
            .collect()
    }

    fn sub_poly(&self, other: &Self, modulus: u64) -> Self {
        (0..self.len().max(other.len()))
            .map(|index| xat(self, index).sub(xat(other, index), modulus))
            .collect()
    }
}

fn xat(polynomial: &XPoly, index: usize) -> &Expr {
    static ZERO: std::sync::OnceLock<Expr> = std::sync::OnceLock::new();
    polynomial
        .get(index)
        .unwrap_or_else(|| ZERO.get_or_init(Expr::default))
}

fn xconstant(coefficients: &[u64], modulus: u64) -> XPoly {
    coefficients
        .iter()
        .map(|coefficient| Expr::constant(*coefficient, modulus))
        .collect()
}

fn xtimes(left: &XPoly, right: &XPoly, modulus: u64) -> Result<XPoly, String> {
    let mut result = vec![Expr::default(); left.len() + right.len() - 1];
    for (left_index, left_coefficient) in left.iter().enumerate() {
        for (right_index, right_coefficient) in right.iter().enumerate() {
            let product = left_coefficient.mul(right_coefficient, modulus)?;
            result[left_index + right_index] =
                result[left_index + right_index].add(&product, modulus);
        }
    }
    Ok(result)
}

fn xeval(polynomial: &XPoly, value: u64, modulus: u64) -> Expr {
    polynomial
        .iter()
        .rev()
        .fold(Expr::default(), |result, coefficient| {
            result.scale(value, modulus).add(coefficient, modulus)
        })
}

fn xderivative(polynomial: &XPoly, modulus: u64) -> XPoly {
    polynomial
        .iter()
        .enumerate()
        .skip(1)
        .map(|(index, coefficient)| coefficient.scale(index as u64, modulus))
        .collect()
}

fn validate_spec(spec: InstanceSpec) -> Result<(), String> {
    if spec.modulus < 5 || spec.modulus >= (1_u64 << 31) || !is_prime(spec.modulus) {
        return Err("modulus must be a prime in [5, 2^31)".to_owned());
    }
    if [
        spec.curve_a,
        spec.curve_b,
        spec.terminal_c,
        spec.target_x,
        spec.target_y,
    ]
    .iter()
    .any(|value| *value >= spec.modulus)
    {
        return Err("field elements must be canonical residues".to_owned());
    }
    if spec.arity < 3 || spec.arity % 2 == 0 || spec.arity as u64 >= spec.modulus {
        return Err("arity must be odd, at least three, and below the modulus".to_owned());
    }
    if spec.arity > MAX_ARITY
        || spec.depth == 0
        || spec.depth >= 63
        || spec.depth > u32::MAX as usize
    {
        return Err("arity/depth outside supported range".to_owned());
    }
    let divisor = 1_u64 << spec.depth;
    if (spec.modulus - 1) % divisor != 0
        || spec.terminal_c == 0
        || pow_mod(spec.terminal_c, (spec.modulus - 1) / divisor, spec.modulus) != 1
    {
        return Err("terminal must define a nonzero split 2^depth coset".to_owned());
    }
    let discriminant = add_mod(
        mul_mod(4, pow_mod(spec.curve_a, 3, spec.modulus), spec.modulus),
        mul_mod(27, pow_mod(spec.curve_b, 2, spec.modulus), spec.modulus),
        spec.modulus,
    );
    if discriminant == 0 {
        return Err("curve must be nonsingular".to_owned());
    }
    if pow_mod(spec.target_y, 2, spec.modulus)
        != add_mod(
            add_mod(
                pow_mod(spec.target_x, 3, spec.modulus),
                mul_mod(spec.curve_a, spec.target_x, spec.modulus),
                spec.modulus,
            ),
            spec.curve_b,
            spec.modulus,
        )
    {
        return Err("target must be an affine point on the curve".to_owned());
    }
    Ok(())
}

fn parse_decimal(value: &str, field: &str) -> Result<u64, String> {
    if value.is_empty()
        || (value.len() > 1 && value.starts_with('0'))
        || !value.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(format!("{field} is not a canonical unsigned decimal"));
    }
    value
        .parse::<u64>()
        .map_err(|_| format!("{field} exceeds u64"))
}

fn valid_symbol(value: &str) -> bool {
    let mut bytes = value.bytes();
    bytes.next().is_some_and(|byte| byte.is_ascii_alphabetic())
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

const fn is_prime(value: u64) -> bool {
    if value < 2 {
        return false;
    }
    if value % 2 == 0 {
        return value == 2;
    }
    let mut divisor = 3;
    while divisor * divisor <= value {
        if value % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}

fn add_mod(left: u64, right: u64, modulus: u64) -> u64 {
    u64::try_from((u128::from(left) + u128::from(right)) % u128::from(modulus))
        .expect("reduced residue fits u64")
}

fn sub_mod(left: u64, right: u64, modulus: u64) -> u64 {
    add_mod(left, modulus - right % modulus, modulus)
}

fn mul_mod(left: u64, right: u64, modulus: u64) -> u64 {
    u64::try_from((u128::from(left) * u128::from(right)) % u128::from(modulus))
        .expect("reduced residue fits u64")
}

fn pow_mod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 == 1 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exponent >>= 1;
    }
    result
}

fn inverse(value: u64, modulus: u64) -> Option<u64> {
    (value != 0).then(|| pow_mod(value, modulus - 2, modulus))
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    x: u64,
    y: u64,
}

fn on_curve(point: Point, spec: InstanceSpec) -> bool {
    pow_mod(point.y, 2, spec.modulus)
        == add_mod(
            add_mod(
                pow_mod(point.x, 3, spec.modulus),
                mul_mod(spec.curve_a, point.x, spec.modulus),
                spec.modulus,
            ),
            spec.curve_b,
            spec.modulus,
        )
}

fn point_add(left: Option<Point>, right: Option<Point>, spec: InstanceSpec) -> Option<Point> {
    let (Some(left), Some(right)) = (left, right) else {
        return left.or(right);
    };
    if left.x == right.x && add_mod(left.y, right.y, spec.modulus) == 0 {
        return None;
    }
    let slope = if left == right {
        let numerator = add_mod(
            mul_mod(3, pow_mod(left.x, 2, spec.modulus), spec.modulus),
            spec.curve_a,
            spec.modulus,
        );
        let denominator = inverse(mul_mod(2, left.y, spec.modulus), spec.modulus)?;
        mul_mod(numerator, denominator, spec.modulus)
    } else {
        let numerator = sub_mod(right.y, left.y, spec.modulus);
        let denominator = inverse(sub_mod(right.x, left.x, spec.modulus), spec.modulus)?;
        mul_mod(numerator, denominator, spec.modulus)
    };
    let x = sub_mod(
        sub_mod(pow_mod(slope, 2, spec.modulus), left.x, spec.modulus),
        right.x,
        spec.modulus,
    );
    let y = sub_mod(
        mul_mod(slope, sub_mod(left.x, x, spec.modulus), spec.modulus),
        left.y,
        spec.modulus,
    );
    Some(Point { x, y })
}

fn candidate_m3_models(spec: InstanceSpec) -> Result<BTreeSet<Vec<u64>>, String> {
    if spec.arity != 3 {
        return Err("candidate enumerator is intentionally restricted to arity three".to_owned());
    }
    let p = spec.modulus;
    let mut models = BTreeSet::new();
    for a0 in 0..p {
        for a1 in 0..p {
            for c0 in 1..p {
                let a = vec![a0, a1, 1];
                if poly_eval(&a, spec.target_x, p) != mul_mod(spec.target_y, c0, p) {
                    continue;
                }
                let a_square = poly_mul(&a, &a, p);
                let c_square_curve = vec![
                    mul_mod(spec.curve_b, mul_mod(c0, c0, p), p),
                    mul_mod(spec.curve_a, mul_mod(c0, c0, p), p),
                    0,
                    mul_mod(c0, c0, p),
                ];
                let norm = poly_sub(&a_square, &c_square_curve, p);
                let (u, remainder) = poly_divide(&norm, &[sub_mod(0, spec.target_x, p), 1], p)?;
                if remainder.iter().any(|value| *value != 0) || u.len() != 4 || u[3] != 1 {
                    continue;
                }
                let mut points = Vec::new();
                for x in 0..p {
                    if poly_eval(&u, x, p) == 0 {
                        if x == spec.target_x
                            || pow_mod(x, 1_u64 << spec.depth, p) != spec.terminal_c
                        {
                            points.clear();
                            break;
                        }
                        let y = mul_mod(
                            sub_mod(0, poly_eval(&a, x, p), p),
                            inverse(c0, p).expect("c0 nonzero"),
                            p,
                        );
                        let point = Point { x, y };
                        if !on_curve(point, spec) {
                            points.clear();
                            break;
                        }
                        points.push(point);
                    }
                }
                if points.len() == 3 {
                    models.insert(vec![a0, a1, c0, u[0], u[1], u[2]]);
                }
            }
        }
    }
    Ok(models)
}

fn oracle_m3_models(spec: InstanceSpec) -> Result<BTreeSet<Vec<u64>>, String> {
    let p = spec.modulus;
    let target = Point {
        x: spec.target_x,
        y: spec.target_y,
    };
    let points = (0..p)
        .flat_map(|x| (0..p).map(move |y| Point { x, y }))
        .filter(|point| {
            point.x != target.x
                && pow_mod(point.x, 1_u64 << spec.depth, p) == spec.terminal_c
                && on_curve(*point, spec)
        })
        .collect::<Vec<_>>();
    let mut models = BTreeSet::new();
    for first in 0..points.len() {
        for second in first + 1..points.len() {
            if points[first].x == points[second].x {
                continue;
            }
            for third in second + 1..points.len() {
                if points[first].x == points[third].x || points[second].x == points[third].x {
                    continue;
                }
                let sum = point_add(
                    point_add(Some(points[first]), Some(points[second]), spec),
                    Some(points[third]),
                    spec,
                );
                if sum == Some(target) {
                    let summands = [points[first], points[second], points[third]];
                    let mut equations = summands
                        .iter()
                        .map(|point| {
                            vec![1, point.x, point.y, sub_mod(0, pow_mod(point.x, 2, p), p)]
                        })
                        .collect::<Vec<_>>();
                    equations.push(vec![
                        1,
                        target.x,
                        sub_mod(0, target.y, p),
                        sub_mod(0, pow_mod(target.x, 2, p), p),
                    ]);
                    let divisor = solve_unique(equations, 3, p)?;
                    let root_polynomial = summands.iter().fold(vec![1], |polynomial, point| {
                        poly_mul(&polynomial, &[sub_mod(0, point.x, p), 1], p)
                    });
                    models.insert(vec![
                        divisor[0],
                        divisor[1],
                        divisor[2],
                        root_polynomial[0],
                        root_polynomial[1],
                        root_polynomial[2],
                    ]);
                }
            }
        }
    }
    Ok(models)
}

fn solve_unique(
    mut equations: Vec<Vec<u64>>,
    variables: usize,
    modulus: u64,
) -> Result<Vec<u64>, String> {
    if equations.iter().any(|row| row.len() != variables + 1) {
        return Err("malformed linear system".to_owned());
    }
    let mut pivot_row = 0;
    let mut where_column = vec![None; variables];
    for column in 0..variables {
        let Some(pivot) = (pivot_row..equations.len()).find(|row| equations[*row][column] != 0)
        else {
            continue;
        };
        equations.swap(pivot, pivot_row);
        let pivot_inverse = inverse(equations[pivot_row][column], modulus)
            .ok_or_else(|| "zero linear-system pivot".to_owned())?;
        for entry in &mut equations[pivot_row][column..=variables] {
            *entry = mul_mod(*entry, pivot_inverse, modulus);
        }
        let normalized_pivot = equations[pivot_row].clone();
        for (row_index, row) in equations.iter_mut().enumerate() {
            if row_index == pivot_row || row[column] == 0 {
                continue;
            }
            let factor = row[column];
            for (entry, pivot_entry) in row[column..=variables]
                .iter_mut()
                .zip(&normalized_pivot[column..=variables])
            {
                *entry = sub_mod(*entry, mul_mod(factor, *pivot_entry, modulus), modulus);
            }
        }
        where_column[column] = Some(pivot_row);
        pivot_row += 1;
    }
    if equations.iter().any(|row| {
        row[..variables].iter().all(|coefficient| *coefficient == 0) && row[variables] != 0
    }) {
        return Err("inconsistent oracle interpolation".to_owned());
    }
    where_column
        .into_iter()
        .map(|row| {
            row.map(|index| equations[index][variables])
                .ok_or_else(|| "nonunique oracle interpolation".to_owned())
        })
        .collect()
}

/// Exhaustively compare the arity-three algebraic predicate with an
/// independently implemented elliptic-curve group oracle for every affine
/// target on the fixed F17 development curve.
///
/// # Errors
///
/// Returns an error if an instance is invalid or any exact projected model set
/// differs between the two implementations.
pub fn e0_f17_panel() -> Result<PanelSummary, String> {
    let base = InstanceSpec {
        modulus: 17,
        curve_a: 2,
        curve_b: 2,
        arity: 3,
        depth: 3,
        terminal_c: 1,
        target_x: 0,
        target_y: 0,
    };
    let targets = (0..17)
        .flat_map(|x| (0..17).map(move |y| Point { x, y }))
        .filter(|point| on_curve(*point, base))
        .collect::<Vec<_>>();
    let mut sat_targets = 0;
    let mut candidate_models = 0;
    let mut oracle_models = 0;
    let mut exact_model_set_matches = 0;
    for target in &targets {
        let spec = InstanceSpec {
            target_x: target.x,
            target_y: target.y,
            ..base
        };
        validate_spec(spec)?;
        let candidate = candidate_m3_models(spec)?;
        let oracle = oracle_m3_models(spec)?;
        if candidate == oracle {
            exact_model_set_matches += 1;
        } else {
            return Err(format!(
                "candidate/oracle model mismatch at target ({}, {}): candidate={}, oracle={}",
                target.x,
                target.y,
                candidate.len(),
                oracle.len()
            ));
        }
        if !candidate.is_empty() {
            sat_targets += 1;
        }
        candidate_models += candidate.len();
        oracle_models += oracle.len();
    }
    Ok(PanelSummary {
        schema: "prime-shared-coefficient.e0-panel/v1".to_owned(),
        kind: "development correctness census; no performance claim".to_owned(),
        modulus: 17,
        curve: [2, 2],
        arity: 3,
        depth: 3,
        terminal_c: 1,
        targets: targets.len(),
        sat_targets,
        unsat_targets: targets.len() - sat_targets,
        candidate_models,
        oracle_models,
        exact_model_set_matches,
        timing_collected: false,
    })
}

fn poly_trim(polynomial: &mut Vec<u64>) {
    while polynomial.len() > 1 && polynomial.last() == Some(&0) {
        polynomial.pop();
    }
    if polynomial.is_empty() {
        polynomial.push(0);
    }
}

fn poly_eval(polynomial: &[u64], value: u64, modulus: u64) -> u64 {
    polynomial.iter().rev().fold(0, |result, coefficient| {
        add_mod(mul_mod(result, value, modulus), *coefficient, modulus)
    })
}

fn poly_mul(left: &[u64], right: &[u64], modulus: u64) -> Vec<u64> {
    let mut result = vec![0; left.len() + right.len() - 1];
    for (left_index, left_coefficient) in left.iter().enumerate() {
        for (right_index, right_coefficient) in right.iter().enumerate() {
            result[left_index + right_index] = add_mod(
                result[left_index + right_index],
                mul_mod(*left_coefficient, *right_coefficient, modulus),
                modulus,
            );
        }
    }
    poly_trim(&mut result);
    result
}

fn poly_sub(left: &[u64], right: &[u64], modulus: u64) -> Vec<u64> {
    let mut result = vec![0; left.len().max(right.len())];
    for (index, coefficient) in result.iter_mut().enumerate() {
        *coefficient = sub_mod(
            *left.get(index).unwrap_or(&0),
            *right.get(index).unwrap_or(&0),
            modulus,
        );
    }
    poly_trim(&mut result);
    result
}

fn poly_divide(
    dividend: &[u64],
    divisor: &[u64],
    modulus: u64,
) -> Result<(Vec<u64>, Vec<u64>), String> {
    let mut remainder = dividend.to_vec();
    poly_trim(&mut remainder);
    let mut divisor = divisor.to_vec();
    poly_trim(&mut divisor);
    if divisor == [0] {
        return Err("polynomial division by zero".to_owned());
    }
    let quotient_len = remainder.len().saturating_sub(divisor.len()) + 1;
    let mut quotient = vec![0; quotient_len.max(1)];
    let divisor_inverse = inverse(*divisor.last().expect("nonempty divisor"), modulus)
        .ok_or_else(|| "noninvertible leading coefficient".to_owned())?;
    while remainder != [0] && remainder.len() >= divisor.len() {
        let shift = remainder.len() - divisor.len();
        let coefficient = mul_mod(
            *remainder.last().expect("nonempty remainder"),
            divisor_inverse,
            modulus,
        );
        quotient[shift] = coefficient;
        for (index, divisor_coefficient) in divisor.iter().enumerate() {
            remainder[shift + index] = sub_mod(
                remainder[shift + index],
                mul_mod(coefficient, *divisor_coefficient, modulus),
                modulus,
            );
        }
        poly_trim(&mut remainder);
    }
    poly_trim(&mut quotient);
    Ok((quotient, remainder))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct LegacyWitness {
        schema: String,
        kind: String,
        p: u64,
        a: u64,
        b: u64,
        curve_order: u64,
        m: usize,
        t: usize,
        terminal_c: u64,
        target: [u64; 2],
        summands: Vec<[u64; 2]>,
        #[serde(rename = "A")]
        divisor_a: Vec<u64>,
        #[serde(rename = "C")]
        divisor_c: Vec<u64>,
        #[serde(rename = "U0")]
        root_polynomial: Vec<u64>,
        equation_count: usize,
        variable_count: usize,
        max_syntactic_input_degree: usize,
        assignment: BTreeMap<String, u64>,
    }

    const M3: &str = include_str!(
        "../../../prime_one_shot_20261029/prototype/exports/m3_t4_remainder.witness.json"
    );
    const M5: &str = include_str!(
        "../../../prime_one_shot_20261029/prototype/exports/m5_t4_remainder.witness.json"
    );
    const M7: &str = include_str!(
        "../../../prime_one_shot_20261029/prototype/exports/m7_t4_remainder.witness.json"
    );

    fn fixture(input: &str) -> (CanonicalIr, LegacyWitness) {
        let witness: LegacyWitness = serde_json::from_str(input).expect("legacy fixture parses");
        assert_eq!(witness.schema, "norm-tower-development-witness/v1");
        assert!(witness.kind.contains("known-answer"));
        assert_eq!(witness.curve_order, 19);
        assert_eq!(witness.summands.len(), witness.m);
        assert_eq!(witness.divisor_a.len(), witness.m.div_ceil(2) + 1);
        assert_eq!(witness.divisor_c.len(), (witness.m - 1) / 2);
        assert_eq!(witness.root_polynomial.len(), witness.m + 1);
        assert_eq!(witness.max_syntactic_input_degree, 2);
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
        .expect("fixture IR builds");
        (ir, witness)
    }

    #[test]
    fn legacy_witnesses_conform_exactly() {
        for (input, expected_variables, expected_equations) in
            [(M3, 33, 35), (M5, 58, 60), (M7, 83, 85)]
        {
            let (ir, witness) = fixture(input);
            assert_eq!(ir.variables.len(), expected_variables);
            assert_eq!(ir.equations.len(), expected_equations);
            assert_eq!(witness.variable_count, expected_variables);
            assert_eq!(witness.equation_count, expected_equations);
            assert_eq!(
                ir.failed_equations(&witness.assignment).unwrap(),
                Vec::<String>::new()
            );

            let mut mutation = witness.assignment.clone();
            *mutation.get_mut("a0").unwrap() = (mutation["a0"] + 1) % witness.p;
            assert!(!ir.failed_equations(&mutation).unwrap().is_empty());
        }
    }

    #[test]
    fn canonical_round_trip_hash_and_strict_parser() {
        let (ir, _) = fixture(M3);
        let encoded = ir.canonical_json().unwrap();
        let decoded = CanonicalIr::parse_json(&encoded).unwrap();
        assert_eq!(decoded, ir);
        assert_eq!(decoded.canonical_json().unwrap(), encoded);
        assert_eq!(decoded.sha256().unwrap(), ir.sha256().unwrap());
        assert!(CanonicalIr::parse_json(&(encoded.clone() + " trailing")).is_err());
        assert!(CanonicalIr::parse_json(&encoded.replacen(
            "\"version\":1",
            "\"version\":1,\"unknown\":0",
            1
        ))
        .is_err());

        let mut malformed = ir.clone();
        malformed.equations[0].terms[0].coefficient = "00".to_owned();
        assert!(malformed.validate().is_err());
        let mut malformed = ir;
        malformed.equations[0].terms.reverse();
        assert!(malformed.validate().is_err());

        let (mut noncanonical, _) = fixture(M3);
        let coefficient = noncanonical.equations[0].terms[0]
            .coefficient
            .parse::<u64>()
            .unwrap();
        noncanonical.equations[0].terms[0].coefficient =
            ((coefficient + 1) % 17).max(1).to_string();
        assert!(noncanonical.validate_structure().is_ok());
        assert!(noncanonical.validate().is_err());
    }

    #[test]
    fn exporters_are_deterministic_and_well_shaped() {
        let (ir, _) = fixture(M3);
        let msolve = ir.to_msolve().unwrap();
        assert!(msolve.starts_with("a0,a1,c0,u0_0"));
        assert!(msolve.lines().nth(1).is_some_and(|line| line == "17"));
        assert_eq!(msolve, ir.to_msolve().unwrap());
        let qf_ff = ir.to_qf_ff().unwrap();
        assert!(qf_ff.contains("(set-logic QF_FF)"));
        assert!(qf_ff.contains("(define-sort F () (_ FiniteField 17))"));
        assert!(qf_ff.contains("(check-sat)"));
        assert!(qf_ff.contains("(get-value (a0 a1 c0 u0_0"));
        assert_eq!(qf_ff, ir.to_qf_ff().unwrap());
    }

    #[test]
    fn f17_candidate_matches_independent_group_oracle_for_every_target() {
        let summary = e0_f17_panel().unwrap();
        assert_eq!(summary.targets, 18);
        assert_eq!(summary.sat_targets, 6);
        assert_eq!(summary.unsat_targets, 12);
        assert_eq!(summary.candidate_models, 6);
        assert_eq!(summary.oracle_models, 6);
        assert_eq!(summary.exact_model_set_matches, 18);
        assert!(!summary.timing_collected);
    }

    #[test]
    fn malformed_specs_and_assignments_are_rejected() {
        let (ir, witness) = fixture(M3);
        let base = InstanceSpec {
            modulus: 17,
            curve_a: 2,
            curve_b: 2,
            arity: 3,
            depth: 4,
            terminal_c: 1,
            target_x: 0,
            target_y: 6,
        };
        assert!(build_ir(InstanceSpec {
            modulus: 15,
            ..base
        })
        .is_err());
        assert!(build_ir(InstanceSpec { arity: 4, ..base }).is_err());
        assert!(build_ir(InstanceSpec { depth: 5, ..base }).is_err());
        assert!(build_ir(InstanceSpec {
            terminal_c: 0,
            ..base
        })
        .is_err());
        assert!(build_ir(InstanceSpec {
            target_y: 1,
            ..base
        })
        .is_err());
        assert!(build_ir(InstanceSpec {
            curve_a: 0,
            curve_b: 0,
            target_y: 0,
            ..base
        })
        .is_err());

        let mut missing = witness.assignment.clone();
        missing.remove("a0");
        assert!(ir.failed_equations(&missing).is_err());
        let mut out_of_range = witness.assignment;
        out_of_range.insert("a0".to_owned(), 17);
        assert!(ir.failed_equations(&out_of_range).is_err());
    }

    #[test]
    fn internal_expression_evaluator_matches_serialized_terms() {
        let modulus = 17;
        let expression = Expr::variable(0)
            .mul(&Expr::variable(1), modulus)
            .unwrap()
            .add(&Expr::constant(3, modulus), modulus);
        let assignment = [4, 5];
        assert_eq!(expression.evaluate(&assignment, modulus), 6);
        assert_eq!(
            evaluate_terms(&expression.into_terms(), &assignment, modulus),
            6
        );
    }
}
