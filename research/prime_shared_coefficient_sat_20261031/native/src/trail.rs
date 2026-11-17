//! Independently replayable finite-field unit-equation trail explanations.
//!
//! This is a deliberately small proof kernel, not a CDCL(T) solver.  After a
//! set of field variables is fixed, it accepts only two deductions from one
//! canonical source equation:
//!
//! - a nonzero constant residual proves a conflict; or
//! - an affine residual `a*x+b`, with `a != 0`, proves the unique value of `x`.
//!
//! Nonlinear residuals, multiple unknowns, zero equations and algebraic facts
//! not bound to the canonical IR fail closed.  A production propagator may use
//! stronger lemmas, but each additional lemma class needs its own checked
//! certificate rule.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{add_mod, inverse, mul_mod, parse_decimal, pow_mod, CanonicalIr, Equation};

/// One canonical field-variable value used by a trail explanation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrailValue {
    /// Canonical IR variable identifier.
    pub variable: u32,
    /// Canonical residue in `[0,p)`.
    pub value: u64,
}

/// The only two explanation kinds admitted by this proof kernel.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplanationKind {
    /// The premises make the named source equation a nonzero constant.
    Conflict,
    /// The premises reduce the named source equation to one affine unknown.
    Propagation,
}

/// Source-bound finite-field explanation for one trail consequence.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrailExplanation {
    /// SHA-256 of the exact canonical IR.
    pub ir_sha256: String,
    /// Exact source-equation label.
    pub equation: String,
    /// Strictly increasing assignments that form the Boolean/theory premise.
    pub premises: Vec<TrailValue>,
    /// Conflict has no conclusion; propagation has exactly one new value.
    pub conclusion: Option<TrailValue>,
    /// Claimed proof rule.
    pub kind: ExplanationKind,
}

type ResidualMonomial = Vec<(u32, u8)>;

fn validate_premises(
    ir: &CanonicalIr,
    premises: &[TrailValue],
    modulus: u64,
) -> Result<BTreeMap<u32, u64>, String> {
    let mut values = BTreeMap::new();
    let mut previous = None;
    for premise in premises {
        if premise.variable as usize >= ir.variables.len() || premise.value >= modulus {
            return Err("trail premise has an unknown variable or noncanonical value".to_owned());
        }
        if previous.is_some_and(|old| old >= premise.variable) {
            return Err("trail premises must be strictly ordered and unique".to_owned());
        }
        previous = Some(premise.variable);
        values.insert(premise.variable, premise.value);
    }
    Ok(values)
}

fn residual(
    equation: &Equation,
    premises: &BTreeMap<u32, u64>,
    modulus: u64,
) -> Result<BTreeMap<ResidualMonomial, u64>, String> {
    let mut residual = BTreeMap::new();
    for term in &equation.terms {
        let mut coefficient = parse_decimal(&term.coefficient, "trail coefficient")?;
        let mut monomial = Vec::new();
        for power in &term.powers {
            if let Some(value) = premises.get(&power.variable) {
                coefficient = mul_mod(
                    coefficient,
                    pow_mod(*value, u64::from(power.exponent), modulus),
                    modulus,
                );
            } else {
                monomial.push((power.variable, power.exponent));
            }
        }
        let sum = add_mod(*residual.get(&monomial).unwrap_or(&0), coefficient, modulus);
        if sum == 0 {
            residual.remove(&monomial);
        } else {
            residual.insert(monomial, sum);
        }
    }
    Ok(residual)
}

fn derive(
    ir: &CanonicalIr,
    equation_label: &str,
    premises: &[TrailValue],
) -> Result<(ExplanationKind, Option<TrailValue>), String> {
    ir.validate()?;
    let modulus = parse_decimal(&ir.spec.modulus, "trail modulus")?;
    let premise_map = validate_premises(ir, premises, modulus)?;
    let equation = ir
        .equations
        .iter()
        .find(|equation| equation.label == equation_label)
        .ok_or_else(|| "trail explanation names an unknown source equation".to_owned())?;
    let residual = residual(equation, &premise_map, modulus)?;
    let constant = *residual.get(&Vec::new()).unwrap_or(&0);
    let unknown_terms = residual
        .iter()
        .filter(|(monomial, _)| !monomial.is_empty())
        .collect::<Vec<_>>();
    if unknown_terms.is_empty() {
        if constant == 0 {
            return Err("satisfied source equation proves no conflict".to_owned());
        }
        return Ok((ExplanationKind::Conflict, None));
    }
    if unknown_terms.len() != 1 {
        return Err("source equation does not reduce to one affine unknown".to_owned());
    }
    let (monomial, coefficient) = unknown_terms[0];
    if monomial.as_slice().len() != 1 || monomial[0].1 != 1 || *coefficient == 0 {
        return Err("source equation residual is not affine".to_owned());
    }
    let variable = monomial[0].0;
    let inverse = inverse(*coefficient, modulus)
        .ok_or_else(|| "affine trail coefficient is not invertible".to_owned())?;
    let neg_constant = if constant == 0 { 0 } else { modulus - constant };
    let value = mul_mod(neg_constant, inverse, modulus);
    Ok((
        ExplanationKind::Propagation,
        Some(TrailValue { variable, value }),
    ))
}

/// Construct a checked unit-equation explanation from canonical source data.
///
/// # Errors
///
/// Returns an error unless substitution leaves a nonzero constant conflict or
/// exactly one affine unknown.
pub fn derive_unit_explanation(
    ir: &CanonicalIr,
    equation: &str,
    premises: Vec<TrailValue>,
) -> Result<TrailExplanation, String> {
    let (kind, conclusion) = derive(ir, equation, &premises)?;
    Ok(TrailExplanation {
        ir_sha256: ir.sha256()?,
        equation: equation.to_owned(),
        premises,
        conclusion,
        kind,
    })
}

/// Independently replay one finite-field trail explanation against the exact
/// canonical IR.
///
/// # Errors
///
/// Returns an error for a different IR, malformed premises, an unsupported
/// residual, or any changed conclusion/kind.
pub fn check_unit_explanation(
    ir: &CanonicalIr,
    explanation: &TrailExplanation,
) -> Result<(), String> {
    if explanation.ir_sha256 != ir.sha256()? {
        return Err("trail explanation is bound to a different canonical IR".to_owned());
    }
    let (kind, conclusion) = derive(ir, &explanation.equation, &explanation.premises)?;
    if kind != explanation.kind || conclusion != explanation.conclusion {
        return Err("trail explanation consequence does not replay".to_owned());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{build_ir, InstanceSpec};

    fn fixture() -> CanonicalIr {
        build_ir(InstanceSpec {
            modulus: 17,
            curve_a: 2,
            curve_b: 2,
            arity: 3,
            depth: 3,
            terminal_c: 1,
            target_x: 0,
            target_y: 6,
        })
        .unwrap()
    }

    #[test]
    fn affine_source_equation_propagates_and_rejects_mutation() {
        let ir = fixture();
        let c0 = ir.variables.iter().find(|v| v.name == "c0").unwrap().id;
        let a0 = ir.variables.iter().find(|v| v.name == "a0").unwrap().id;
        let proof = derive_unit_explanation(
            &ir,
            "target_zero",
            vec![TrailValue {
                variable: c0,
                value: 3,
            }],
        )
        .unwrap();
        assert_eq!(proof.kind, ExplanationKind::Propagation);
        assert_eq!(
            proof.conclusion,
            Some(TrailValue {
                variable: a0,
                value: 1
            })
        );
        check_unit_explanation(&ir, &proof).unwrap();
        let mut corrupted = proof;
        corrupted.conclusion.as_mut().unwrap().value = 2;
        assert!(check_unit_explanation(&ir, &corrupted).is_err());
    }

    #[test]
    fn inconsistent_full_source_equation_proves_conflict() {
        let ir = fixture();
        let c0 = ir.variables.iter().find(|v| v.name == "c0").unwrap().id;
        let a0 = ir.variables.iter().find(|v| v.name == "a0").unwrap().id;
        let mut premises = vec![
            TrailValue {
                variable: a0,
                value: 0,
            },
            TrailValue {
                variable: c0,
                value: 1,
            },
        ];
        premises.sort();
        let proof = derive_unit_explanation(&ir, "target_zero", premises).unwrap();
        assert_eq!(proof.kind, ExplanationKind::Conflict);
        assert_eq!(proof.conclusion, None);
        check_unit_explanation(&ir, &proof).unwrap();
    }

    #[test]
    fn nonlinear_or_underassigned_residual_fails_closed() {
        let ir = fixture();
        assert!(derive_unit_explanation(&ir, "norm_0", Vec::new()).is_err());
    }
}
