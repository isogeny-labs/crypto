//! Conditional, cold single-target cost models for prime-field index calculus.
//!
//! This module evaluates stated costs; it does not implement a relation solver
//! or certify that a hypothetical solver exists. Fixed-arity exponents assume
//! a full-factor-base workflow requiring Theta(B) independent rank increments.
//! Finite budgets keep the population B separate from the required rank: signed
//! points, distinct columns, encodings and an isogeny's degree need not agree.
//! Each attempt supplies at most one independent rank increment in this model.
//!
//! For B = n^beta, trial work n^gamma B^delta, and retention
//! n^(-eta_n) B^(-eta_b), collection has the conditional exponent
//! gamma + eta_n + (delta + eta_b + 1) beta + max(0, 1 - m beta).
//! The optimizer minimizes a maximum of affine lines by checking endpoints and
//! intersections, without a sampling grid. Factorials are constants only in
//! this fixed-m limit; the finite model retains them and makes no growing-m
//! asymptotic claim. All caller-supplied phase work must use the same unit.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;

/// A finite-model evaluation limit, not a mathematical restriction on arity.
pub const MAX_FINITE_ARITY: u32 = 100_000;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelError {
    pub code: String,
    pub field: String,
    pub message: String,
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} ({})", self.field, self.message, self.code)
    }
}

impl std::error::Error for ModelError {}

fn error(code: &str, field: &str, message: &str) -> ModelError {
    ModelError {
        code: code.to_owned(),
        field: field.to_owned(),
        message: message.to_owned(),
    }
}

fn finite(value: f64, field: &str) -> Result<f64, ModelError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(error("invalid_input", field, "must be finite"))
    }
}

fn nonnegative(value: f64, field: &str) -> Result<f64, ModelError> {
    finite(value, field)?;
    if value < 0.0 {
        Err(error("invalid_input", field, "must be nonnegative"))
    } else {
        Ok(value)
    }
}

fn computed(value: f64, field: &str) -> Result<f64, ModelError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(error(
            "numeric_range",
            field,
            "calculation exceeds the finite floating-point range",
        ))
    }
}

fn valid_name(name: &str, field: &str) -> Result<(), ModelError> {
    if name.trim().is_empty() {
        Err(error("invalid_input", field, "name must not be empty"))
    } else {
        Ok(())
    }
}

/// A cost exponent `intercept + slope * beta`. Negative slopes are permitted
/// for explicitly supplied tradeoffs; the overall envelope includes B setup.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AffineCost {
    pub name: String,
    pub intercept: f64,
    pub slope: f64,
}

/// Feasibility requires `intercept + slope * beta <= max_exponent`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemoryConstraint {
    pub name: String,
    pub intercept: f64,
    pub slope: f64,
    pub max_exponent: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExponentModel {
    pub arity: u32,
    pub trial_n_exponent: f64,
    pub trial_b_exponent: f64,
    pub retention_n_penalty: f64,
    pub retention_b_penalty: f64,
    /// Two for the fixed-arity sparse Wiedemann work model, three for cubic
    /// dense elimination. Additional rank-auditing work belongs in extra_costs.
    pub linear_algebra_b_exponent: f64,
    pub beta_min: f64,
    pub beta_max: f64,
    pub extra_costs: Vec<AffineCost>,
    pub memory_constraints: Vec<MemoryConstraint>,
}

impl Default for ExponentModel {
    fn default() -> Self {
        Self {
            arity: 2,
            trial_n_exponent: 0.0,
            trial_b_exponent: 1.0,
            retention_n_penalty: 0.0,
            retention_b_penalty: 0.0,
            linear_algebra_b_exponent: 2.0,
            beta_min: 0.0,
            beta_max: 1.0,
            extra_costs: Vec::new(),
            memory_constraints: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhaseExponent {
    pub name: String,
    pub exponent: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExponentOptimum {
    pub beta: f64,
    pub work_exponent: f64,
    /// Slope of log(T / sqrt(n)) against log(n), not the raw-work slope.
    pub normalized_slope: f64,
    pub feasible_beta_min: f64,
    pub feasible_beta_max: f64,
    pub phase_exponents: Vec<PhaseExponent>,
    pub dominant_phases: Vec<String>,
    pub evaluated_candidates: usize,
    pub assumptions: Vec<String>,
}

fn line_value(line: &AffineCost, beta: f64) -> Result<f64, ModelError> {
    computed(
        line.intercept + line.slope * beta,
        &format!("phase.{}", line.name),
    )
}

fn near_equal(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-10 * left.abs().max(right.abs()).max(1.0)
}

/// Optimize the stated fixed-m power-law envelope. This is an analytic
/// candidate-set minimization in f64, not a formal arithmetic proof.
pub fn optimize_exponent(model: &ExponentModel) -> Result<ExponentOptimum, ModelError> {
    if model.arity == 0 {
        return Err(error("invalid_input", "arity", "must be at least one"));
    }
    for (name, value) in [
        ("trial_n_exponent", model.trial_n_exponent),
        ("trial_b_exponent", model.trial_b_exponent),
        ("retention_n_penalty", model.retention_n_penalty),
        ("retention_b_penalty", model.retention_b_penalty),
        ("linear_algebra_b_exponent", model.linear_algebra_b_exponent),
        ("beta_min", model.beta_min),
        ("beta_max", model.beta_max),
    ] {
        nonnegative(value, name)?;
    }
    if model.linear_algebra_b_exponent < 1.0 {
        return Err(error(
            "invalid_input",
            "linear_algebra_b_exponent",
            "must be at least one for a materialized full-factor-base system",
        ));
    }
    if model.beta_min > model.beta_max || model.beta_max > 1.0 {
        return Err(error(
            "invalid_input",
            "beta_bounds",
            "require 0 <= beta_min <= beta_max <= 1",
        ));
    }

    let mut lower = model.beta_min;
    let mut upper = model.beta_max;
    let mut memory_names = BTreeSet::new();
    for memory in &model.memory_constraints {
        valid_name(&memory.name, "memory_constraints.name")?;
        if !memory_names.insert(memory.name.clone()) {
            return Err(error(
                "invalid_input",
                "memory_constraints.name",
                "duplicate constraint name",
            ));
        }
        finite(memory.intercept, "memory_constraints.intercept")?;
        finite(memory.slope, "memory_constraints.slope")?;
        nonnegative(memory.max_exponent, "memory_constraints.max_exponent")?;
        if memory.slope == 0.0 {
            if memory.intercept > memory.max_exponent {
                return Err(error(
                    "infeasible",
                    &memory.name,
                    "memory constraint is violated for every beta",
                ));
            }
        } else {
            let bound = (memory.max_exponent - memory.intercept) / memory.slope;
            if bound.is_nan() {
                return Err(error("numeric_range", &memory.name, "invalid memory bound"));
            }
            if memory.slope > 0.0 {
                upper = upper.min(bound);
            } else {
                lower = lower.max(bound);
            }
        }
    }
    if lower > upper || !lower.is_finite() || !upper.is_finite() {
        return Err(error(
            "infeasible",
            "memory_constraints",
            "no beta satisfies the requested interval and memory constraints",
        ));
    }

    let a = computed(
        model.trial_n_exponent + model.retention_n_penalty,
        "relation_intercept",
    )?;
    let d = computed(
        model.trial_b_exponent + model.retention_b_penalty + 1.0,
        "relation_slope",
    )?;
    let mut lines = vec![
        AffineCost {
            name: "factor_base".to_owned(),
            intercept: 0.0,
            slope: 1.0,
        },
        AffineCost {
            name: "relation_collection".to_owned(),
            intercept: a,
            slope: d,
        },
        AffineCost {
            name: "relation_collection".to_owned(),
            intercept: computed(a + 1.0, "relation_sparse_intercept")?,
            slope: computed(d - f64::from(model.arity), "relation_sparse_slope")?,
        },
        AffineCost {
            name: "linear_algebra".to_owned(),
            intercept: 0.0,
            slope: model.linear_algebra_b_exponent,
        },
    ];
    let mut phase_names: BTreeSet<String> = lines.iter().map(|v| v.name.clone()).collect();
    for cost in &model.extra_costs {
        valid_name(&cost.name, "extra_costs.name")?;
        finite(cost.intercept, "extra_costs.intercept")?;
        finite(cost.slope, "extra_costs.slope")?;
        if !phase_names.insert(cost.name.clone()) {
            return Err(error(
                "invalid_input",
                "extra_costs.name",
                "duplicate or reserved phase name",
            ));
        }
        lines.push(cost.clone());
    }

    // A convex maximum of affine lines has a minimum at an endpoint or an
    // intersection (or along a flat interval whose endpoint is included).
    let mut candidates = vec![lower, upper];
    for (i, left) in lines.iter().enumerate() {
        for right in lines.iter().skip(i + 1) {
            let denominator = computed(left.slope - right.slope, "intersection_slope_difference")?;
            if denominator == 0.0 {
                continue;
            }
            let numerator = computed(
                right.intercept - left.intercept,
                "intersection_intercept_difference",
            )?;
            let beta = numerator / denominator;
            if beta.is_finite() && beta >= lower && beta <= upper {
                candidates.push(beta);
            }
        }
    }
    candidates.sort_by(f64::total_cmp);
    candidates.dedup_by(|left, right| *left == *right);
    let mut best_beta = lower;
    let mut best_work = f64::INFINITY;
    for &beta in &candidates {
        let mut work = 0.0_f64;
        for line in &lines {
            work = work.max(line_value(line, beta)?);
        }
        // Sorted candidates select the smallest beta on an exactly flat optimum.
        if work < best_work {
            best_beta = beta;
            best_work = work;
        }
    }
    let phase_exponents = phase_names
        .into_iter()
        .map(|name| {
            let mut exponent = f64::NEG_INFINITY;
            for line in lines.iter().filter(|line| line.name == name) {
                exponent = exponent.max(line_value(line, best_beta)?);
            }
            Ok(PhaseExponent { name, exponent })
        })
        .collect::<Result<Vec<_>, ModelError>>()?;
    let dominant_phases = phase_exponents
        .iter()
        .filter(|phase| near_equal(phase.exponent, best_work))
        .map(|phase| phase.name.clone())
        .collect();

    Ok(ExponentOptimum {
        beta: best_beta,
        work_exponent: best_work,
        normalized_slope: best_work - 0.5,
        feasible_beta_min: lower,
        feasible_beta_max: upper,
        phase_exponents,
        dominant_phases,
        evaluated_candidates: candidates.len(),
        assumptions: vec![
            "Conditional fixed-arity power-law model; hypothetical solver costs are not existence claims.".to_owned(),
            "Uniform residuals independent of a fixed factor base, with assumed random-sum coverage exponent.".to_owned(),
            "The standard full-factor-base workflow needs Theta(B) independent rank increments.".to_owned(),
            "At most one independent rank increment is credited per attempt; retention includes failures and dependencies.".to_owned(),
            "Cold single-target setup is charged in full; all additional implementation phases require explicit costs.".to_owned(),
        ],
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TupleMode {
    Distinct,
    WithRepetition,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FiniteRelationInput {
    pub group_order_log2: f64,
    /// Population of distinct admissible points, after all sign/encoding rules.
    pub factor_base_population_log2: f64,
    pub arity: u32,
    pub tuple_mode: TupleMode,
    pub trial_work_log2: f64,
    /// Independent requirement; it need not equal the point population.
    pub required_rank_increments_log2: f64,
    /// Conditional retention after solvability, including actual solver recall,
    /// validation, duplicate removal and independent-rank yield.
    pub conditional_useful_retention: f64,
    /// An exact/supplied solvability probability. None selects an explicitly
    /// heuristic Poisson-occupancy estimate. A sample frequency is not a proof
    /// of this probability and must carry its uncertainty in the caller.
    pub solvability_probability: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FiniteRelationBudget {
    pub factorial_log2: f64,
    pub admissible_tuples_log2: f64,
    pub expected_representations_log2: f64,
    pub coverage_upper_bound_log2: f64,
    pub solvability_probability_log2: f64,
    pub expected_attempts_log2: f64,
    pub relation_work_log2: f64,
    pub probability_source: String,
    pub assumptions: Vec<String>,
}

fn kahan_sum(values: impl Iterator<Item = f64>) -> f64 {
    let mut sum = 0.0;
    let mut correction = 0.0;
    for value in values {
        let adjusted = value - correction;
        let next = sum + adjusted;
        correction = (next - sum) - adjusted;
        sum = next;
    }
    sum
}

fn probability(value: f64, field: &str) -> Result<f64, ModelError> {
    finite(value, field)?;
    if value == 0.0 {
        Err(error(
            "no_finite_estimate",
            field,
            "zero success/retention supplies no finite expected-work estimate",
        ))
    } else if !(0.0..=1.0).contains(&value) {
        Err(error("invalid_input", field, "must lie in (0, 1]"))
    } else {
        Ok(value)
    }
}

fn poisson_probability_log2(intensity_log2: f64) -> f64 {
    if intensity_log2 <= -1000.0 {
        // The correction is far below f64 precision, while exp2 may underflow.
        intensity_log2
    } else if intensity_log2 >= 10.0 {
        // exp(-1024) is already below representable precision for 1 - exp(-mu).
        0.0
    } else {
        let intensity = intensity_log2.exp2();
        (-(-intensity).exp_m1()).log2()
    }
}

/// Evaluate finite expected relation work in log2 space. The tuple cardinality
/// is a combinatorial upper bound on distinct sums; equality of sums and rank
/// distribution still require evidence. Nonintegral log-population inputs give
/// the continuous binomial model, not an enumeration of a finite point set.
pub fn finite_relation_budget(
    input: &FiniteRelationInput,
) -> Result<FiniteRelationBudget, ModelError> {
    finite(input.group_order_log2, "group_order_log2")?;
    if input.group_order_log2 <= 0.0 {
        return Err(error(
            "invalid_input",
            "group_order_log2",
            "must be positive",
        ));
    }
    nonnegative(
        input.factor_base_population_log2,
        "factor_base_population_log2",
    )?;
    if input.factor_base_population_log2 > input.group_order_log2 {
        return Err(error(
            "invalid_input",
            "factor_base_population_log2",
            "the distinct subgroup-point population cannot exceed the subgroup order",
        ));
    }
    if input.arity == 0 || input.arity > MAX_FINITE_ARITY {
        return Err(error(
            "invalid_input",
            "arity",
            "finite evaluation requires arity in 1..=100000",
        ));
    }
    finite(input.trial_work_log2, "trial_work_log2")?;
    nonnegative(
        input.required_rank_increments_log2,
        "required_rank_increments_log2",
    )?;
    probability(
        input.conditional_useful_retention,
        "conditional_useful_retention",
    )?;
    if input.tuple_mode == TupleMode::Distinct
        && input.factor_base_population_log2 < f64::from(input.arity).log2()
    {
        return Err(error(
            "no_finite_estimate",
            "factor_base_population_log2",
            "fewer than m points means there are no unordered distinct m-tuples",
        ));
    }

    let factorial_log2 = kahan_sum((1..=input.arity).map(|j| f64::from(j).log2()));
    let inverse_population = (-input.factor_base_population_log2).exp2();
    let correction = kahan_sum((0..input.arity).map(|j| {
        let offset = f64::from(j) * inverse_population;
        match input.tuple_mode {
            TupleMode::Distinct => (-offset).ln_1p() / std::f64::consts::LN_2,
            TupleMode::WithRepetition => offset.ln_1p() / std::f64::consts::LN_2,
        }
    }));
    let tuple_log2 = computed(
        f64::from(input.arity) * input.factor_base_population_log2 - factorial_log2 + correction,
        "admissible_tuples_log2",
    )?;
    // Valid inputs have at least one tuple. Clamp only this known cardinality
    // floor to absorb cancellation near C(m,m)=1 or C(1+m-1,m)=1.
    let admissible_tuples_log2 = tuple_log2.max(0.0);
    let expected_representations_log2 = computed(
        admissible_tuples_log2 - input.group_order_log2,
        "expected_representations_log2",
    )?;
    let coverage_upper_bound_log2 = expected_representations_log2.min(0.0);
    let (solvability_probability_log2, probability_source) =
        if let Some(supplied) = input.solvability_probability {
            probability(supplied, "solvability_probability")?;
            let supplied_log2 = supplied.log2();
            if supplied_log2 > coverage_upper_bound_log2 + 1e-10 {
                return Err(error(
                "invalid_input",
                "solvability_probability",
                "exceeds the tuple-count coverage upper bound; check point/sign/tuple conventions",
            ));
            }
            (supplied_log2, "supplied_probability".to_owned())
        } else {
            (
                poisson_probability_log2(expected_representations_log2),
                "poisson_occupancy_heuristic".to_owned(),
            )
        };
    let expected_attempts_log2 = computed(
        input.required_rank_increments_log2
            - solvability_probability_log2
            - input.conditional_useful_retention.log2(),
        "expected_attempts_log2",
    )?;
    let relation_work_log2 = computed(
        expected_attempts_log2 + input.trial_work_log2,
        "relation_work_log2",
    )?;

    Ok(FiniteRelationBudget {
        factorial_log2,
        admissible_tuples_log2,
        expected_representations_log2,
        coverage_upper_bound_log2,
        solvability_probability_log2,
        expected_attempts_log2,
        relation_work_log2,
        probability_source,
        assumptions: vec![
            "Point population and required independent rank increments are separate inputs; isogeny degree is neither by definition.".to_owned(),
            "At most one independent rank increment is credited per attempt.".to_owned(),
            "Uniform residuals are independent of the fixed admissible point set; tuple counts bound, but do not equal, distinct-sum coverage.".to_owned(),
            "Constant aggregate useful retention is assumed across collection; changing rank-state yield needs a state-dependent model.".to_owned(),
            "This is a finite expected-work estimate retaining log2(m!); it is not a growing-arity asymptotic result.".to_owned(),
        ],
    })
}

/// Unknown cost is not zero. Known values are log2 of positive work in the
/// common unit chosen by the caller; Zero explicitly denotes no work.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", content = "log2_work", rename_all = "snake_case")]
pub enum PhaseWork {
    Known(f64),
    Zero,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhaseBudget {
    pub name: String,
    pub work: PhaseWork,
    /// True only if this work can actually be reused across the same instance's
    /// targets. Target-dependent relation collection must be false.
    pub reusable: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndToEndBudget {
    pub target_count_log2: f64,
    /// None in work fields means exact zero; unknown inputs are rejected.
    pub cold_work_log2: Option<f64>,
    pub reusable_work_log2: Option<f64>,
    pub online_work_log2: Option<f64>,
    pub total_work_log2: Option<f64>,
    pub amortized_work_log2: Option<f64>,
    pub largest_cold_phase_log2: Option<f64>,
    pub largest_total_contribution_log2: Option<f64>,
    pub dominant_cold_phases: Vec<String>,
    pub dominant_total_phases: Vec<String>,
}

fn log2_sum(values: &[f64], field: &str) -> Result<Option<f64>, ModelError> {
    let Some(maximum) = values.iter().copied().max_by(f64::total_cmp) else {
        return Ok(None);
    };
    let relative_sum = kahan_sum(values.iter().map(|value| (value - maximum).exp2()));
    Ok(Some(computed(maximum + relative_sum.log2(), field)?))
}

fn dominant_phases(values: &[(String, f64)]) -> (Option<f64>, Vec<String>) {
    let maximum = values.iter().map(|(_, v)| *v).max_by(f64::total_cmp);
    let names = maximum
        .map(|maximum| {
            values
                .iter()
                .filter(|(_, value)| near_equal(*value, maximum))
                .map(|(name, _)| name.clone())
                .collect()
        })
        .unwrap_or_default();
    (maximum, names)
}

/// Sum a complete caller-declared work ledger. Setup is charged once in the
/// cold single-target total, then once over K targets in the separate amortized
/// result. Maxima are reported alongside log-sums and never replace them.
pub fn aggregate_phase_budgets(
    phases: &[PhaseBudget],
    required_phase_names: &[String],
    log2_targets: f64,
) -> Result<EndToEndBudget, ModelError> {
    nonnegative(log2_targets, "log2_targets")?;
    if phases.is_empty() {
        return Err(error("missing_phase", "phases", "the work ledger is empty"));
    }
    let mut names = BTreeSet::new();
    let mut cold = Vec::new();
    let mut reusable = Vec::new();
    let mut online = Vec::new();
    let mut total = Vec::new();
    for phase in phases {
        valid_name(&phase.name, "phases.name")?;
        if !names.insert(phase.name.clone()) {
            return Err(error("invalid_input", &phase.name, "duplicate phase name"));
        }
        match phase.work {
            PhaseWork::Unknown => {
                return Err(error(
                    "unknown_phase_cost",
                    &phase.name,
                    "unknown work prevents a complete total",
                ));
            }
            PhaseWork::Zero => {}
            PhaseWork::Known(work) => {
                finite(work, &phase.name)?;
                cold.push((phase.name.clone(), work));
                let contribution = if phase.reusable {
                    reusable.push(work);
                    work
                } else {
                    online.push(work);
                    computed(work + log2_targets, &phase.name)?
                };
                total.push((phase.name.clone(), contribution));
            }
        }
    }
    let mut required_names = BTreeSet::new();
    for required in required_phase_names {
        valid_name(required, "required_phase_names")?;
        if !required_names.insert(required.clone()) {
            return Err(error("invalid_input", required, "duplicate required phase"));
        }
        if !names.contains(required) {
            return Err(error("missing_phase", required, "required work is absent"));
        }
    }

    let cold_work_log2 = log2_sum(
        &cold.iter().map(|(_, v)| *v).collect::<Vec<_>>(),
        "cold_work_log2",
    )?;
    let total_work_log2 = log2_sum(
        &total.iter().map(|(_, v)| *v).collect::<Vec<_>>(),
        "total_work_log2",
    )?;
    let amortized_work_log2 = total_work_log2
        .map(|value| computed(value - log2_targets, "amortized_work_log2"))
        .transpose()?;
    let (largest_cold_phase_log2, dominant_cold_phases) = dominant_phases(&cold);
    let (largest_total_contribution_log2, dominant_total_phases) = dominant_phases(&total);
    Ok(EndToEndBudget {
        target_count_log2: log2_targets,
        cold_work_log2,
        reusable_work_log2: log2_sum(&reusable, "reusable_work_log2")?,
        online_work_log2: log2_sum(&online, "online_work_log2")?,
        total_work_log2,
        amortized_work_log2,
        largest_cold_phase_log2,
        largest_total_contribution_log2,
        dominant_cold_phases,
        dominant_total_phases,
    })
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlopeComparison {
    pub raw_slope: f64,
    pub rho_slope: f64,
    pub normalized_slope: f64,
    /// Arithmetic comparison of the supplied slopes, without a confidence or
    /// asymptotic-achievability claim about any measured data.
    pub below_rho_exponent: bool,
}

pub fn compare_slopes(raw_slope: f64, rho_slope: f64) -> Result<SlopeComparison, ModelError> {
    finite(raw_slope, "raw_slope")?;
    nonnegative(rho_slope, "rho_slope")?;
    Ok(SlopeComparison {
        raw_slope,
        rho_slope,
        normalized_slope: computed(raw_slope - rho_slope, "normalized_slope")?,
        below_rho_exponent: raw_slope < rho_slope,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-10,
            "actual {actual}, expected {expected}"
        );
    }

    fn model(m: u32, delta: f64, a: f64, ell: f64) -> ExponentModel {
        ExponentModel {
            arity: m,
            trial_b_exponent: delta,
            trial_n_exponent: a,
            linear_algebra_b_exponent: ell,
            ..ExponentModel::default()
        }
    }

    #[test]
    fn analytical_fixed_arity_optima() {
        for (m, delta, a, ell, beta, exponent) in [
            (2, 0.0, 0.0, 2.0, 1.0 / 3.0, 2.0 / 3.0),
            (3, 0.0, 0.0, 2.0, 1.0 / 4.0, 1.0 / 2.0),
            (4, 0.0, 0.0, 2.0, 1.0 / 5.0, 2.0 / 5.0),
            (2, 1.0, 0.0, 2.0, 0.0, 1.0),
            (3, 1.0, 0.0, 2.0, 1.0 / 3.0, 2.0 / 3.0),
            (4, 1.0, 0.0, 2.0, 1.0 / 4.0, 1.0 / 2.0),
            (5, 1.0, 0.0, 2.0, 1.0 / 5.0, 2.0 / 5.0),
            (5, 1.0, 0.0, 3.0, 1.0 / 6.0, 1.0 / 2.0),
            (6, 1.0, 0.0, 3.0, 1.0 / 7.0, 3.0 / 7.0),
            (5, 1.0, 0.1, 2.0, 1.0 / 5.0, 1.0 / 2.0),
            (6, 1.0, 0.1, 2.0, 1.0 / 6.0, 13.0 / 30.0),
            (4, 2.0, 0.0, 2.0, 1.0 / 4.0, 3.0 / 4.0),
            (6, 2.0, 0.0, 2.0, 1.0 / 6.0, 1.0 / 2.0),
            (7, 2.0, 0.0, 2.0, 1.0 / 7.0, 3.0 / 7.0),
        ] {
            let result = optimize_exponent(&model(m, delta, a, ell)).unwrap();
            close(result.beta, beta);
            close(result.work_exponent, exponent);
            close(result.normalized_slope, exponent - 0.5);
        }
    }

    #[test]
    fn retention_losses_are_charged() {
        let mut with_loss = model(5, 1.0, 0.0, 2.0);
        with_loss.retention_n_penalty = 0.1;
        close(optimize_exponent(&with_loss).unwrap().work_exponent, 0.5);
        with_loss.retention_n_penalty = 0.0;
        with_loss.retention_b_penalty = 1.0;
        close(optimize_exponent(&with_loss).unwrap().work_exponent, 0.6);
    }

    #[test]
    fn dense_rank_audit_and_preprocessing_are_not_dropped() {
        for name in ["dense_rank_audit", "preprocessing"] {
            let mut input = model(3, 1.0, 0.0, 2.0);
            input.extra_costs.push(AffineCost {
                name: name.to_owned(),
                intercept: 0.0,
                slope: 3.0,
            });
            let result = optimize_exponent(&input).unwrap();
            close(result.beta, 0.25);
            close(result.work_exponent, 0.75);
            assert!(result.dominant_phases.iter().any(|v| v == name));
        }
    }

    #[test]
    fn affine_setup_constant_can_dominate() {
        let mut input = model(5, 1.0, 0.0, 2.0);
        input.extra_costs.push(AffineCost {
            name: "setup".to_owned(),
            intercept: 0.8,
            slope: 0.0,
        });
        close(optimize_exponent(&input).unwrap().work_exponent, 0.8);
    }

    #[test]
    fn memory_cap_changes_the_optimum() {
        let mut input = model(5, 1.0, 0.0, 2.0);
        input.memory_constraints.push(MemoryConstraint {
            name: "factor_base_storage".to_owned(),
            intercept: 0.0,
            slope: 1.0,
            max_exponent: 3.0 / 20.0,
        });
        let result = optimize_exponent(&input).unwrap();
        close(result.beta, 3.0 / 20.0);
        close(result.work_exponent, 11.0 / 20.0);
        close(result.feasible_beta_max, 3.0 / 20.0);
    }

    #[test]
    fn decreasing_memory_constraint_supplies_a_lower_bound() {
        let mut input = model(5, 1.0, 0.0, 2.0);
        input.memory_constraints.push(MemoryConstraint {
            name: "tradeoff".to_owned(),
            intercept: 0.5,
            slope: -1.0,
            max_exponent: 0.2,
        });
        let result = optimize_exponent(&input).unwrap();
        close(result.feasible_beta_min, 0.3);
        close(result.beta, 0.3);
        close(result.work_exponent, 0.6);
    }

    #[test]
    fn infeasible_memory_is_an_error() {
        let mut input = ExponentModel::default();
        input.memory_constraints.push(MemoryConstraint {
            name: "fixed_storage".to_owned(),
            intercept: 0.4,
            slope: 0.0,
            max_exponent: 0.3,
        });
        assert_eq!(optimize_exponent(&input).unwrap_err().code, "infeasible");
    }

    #[test]
    fn malformed_exponent_inputs_are_rejected() {
        let inputs = [
            ExponentModel {
                arity: 0,
                ..ExponentModel::default()
            },
            ExponentModel {
                trial_n_exponent: f64::NAN,
                ..ExponentModel::default()
            },
            ExponentModel {
                trial_b_exponent: -1.0,
                ..ExponentModel::default()
            },
            ExponentModel {
                beta_max: 1.1,
                ..ExponentModel::default()
            },
            ExponentModel {
                beta_min: 0.6,
                beta_max: 0.5,
                ..ExponentModel::default()
            },
            ExponentModel {
                extra_costs: vec![AffineCost {
                    name: "factor_base".to_owned(),
                    intercept: 0.0,
                    slope: 1.0,
                }],
                ..ExponentModel::default()
            },
        ];
        assert!(inputs.iter().all(|input| optimize_exponent(input).is_err()));
    }

    #[test]
    fn finite_extra_costs_cannot_hide_intersection_overflow() {
        for (rising, falling, expected_field) in [
            (
                AffineCost {
                    name: "rising".to_owned(),
                    intercept: 0.0,
                    slope: 1.79e308,
                },
                AffineCost {
                    name: "falling".to_owned(),
                    intercept: 8.9e307,
                    slope: -1.79e308,
                },
                "intersection_slope_difference",
            ),
            (
                AffineCost {
                    name: "rising".to_owned(),
                    intercept: -1.0e308,
                    slope: 1.0e308,
                },
                AffineCost {
                    name: "falling".to_owned(),
                    intercept: 1.0e308,
                    slope: -0.5e308,
                },
                "intersection_intercept_difference",
            ),
        ] {
            // Every input coefficient is finite. Without checked subtraction,
            // an infinite slope difference silently discards the genuine
            // interior optimum of the first case near beta = 0.24860335.
            let input = ExponentModel {
                arity: 5,
                extra_costs: vec![rising, falling],
                ..ExponentModel::default()
            };
            let failure = optimize_exponent(&input).unwrap_err();
            assert_eq!(failure.code, "numeric_range");
            assert_eq!(failure.field, expected_field);
        }
    }

    fn finite_input() -> FiniteRelationInput {
        FiniteRelationInput {
            group_order_log2: 13.0,
            factor_base_population_log2: 5.0,
            arity: 2,
            tuple_mode: TupleMode::Distinct,
            trial_work_log2: 5.0,
            required_rank_increments_log2: 4.0,
            conditional_useful_retention: 1.0,
            solvability_probability: None,
        }
    }

    #[test]
    fn finite_tuple_count_keeps_factorial_and_rank_separate() {
        let input = finite_input();
        let result = finite_relation_budget(&input).unwrap();
        close(result.factorial_log2, 1.0);
        close(result.admissible_tuples_log2, 496.0_f64.log2());
        let probability = 1.0 - (-496.0_f64 / 8192.0).exp();
        close(result.solvability_probability_log2, probability.log2());
        close(result.expected_attempts_log2, (16.0 / probability).log2());
        close(
            result.relation_work_log2,
            (16.0 * 32.0 / probability).log2(),
        );
        let mut doubled_rank = input;
        doubled_rank.required_rank_increments_log2 += 1.0;
        close(
            finite_relation_budget(&doubled_rank)
                .unwrap()
                .relation_work_log2,
            result.relation_work_log2 + 1.0,
        );
    }

    #[test]
    fn finite_repetition_count_and_single_tuple_boundaries() {
        let mut input = finite_input();
        input.tuple_mode = TupleMode::WithRepetition;
        close(
            finite_relation_budget(&input)
                .unwrap()
                .admissible_tuples_log2,
            528.0_f64.log2(),
        );
        input.factor_base_population_log2 = 0.0;
        input.arity = 8;
        close(
            finite_relation_budget(&input)
                .unwrap()
                .admissible_tuples_log2,
            0.0,
        );
        input.factor_base_population_log2 = 3.0;
        input.tuple_mode = TupleMode::Distinct;
        close(
            finite_relation_budget(&input)
                .unwrap()
                .admissible_tuples_log2,
            0.0,
        );
    }

    #[test]
    fn finite_rare_and_saturated_coverage_stay_finite() {
        let mut rare = finite_input();
        rare.group_order_log2 = 4096.0;
        let result = finite_relation_budget(&rare).unwrap();
        assert!(result.relation_work_log2.is_finite());
        close(
            result.solvability_probability_log2,
            496.0_f64.log2() - 4096.0,
        );
        let mut saturated = finite_input();
        saturated.factor_base_population_log2 = 13.0;
        let result = finite_relation_budget(&saturated).unwrap();
        close(result.solvability_probability_log2, 0.0);
        close(result.expected_attempts_log2, 4.0);
    }

    #[test]
    fn supplied_coverage_and_retention_are_charged_once() {
        let mut input = finite_input();
        input.solvability_probability = Some(1.0 / 32.0);
        input.conditional_useful_retention = 0.5;
        let result = finite_relation_budget(&input).unwrap();
        close(result.expected_attempts_log2, 10.0);
        close(result.relation_work_log2, 15.0);
        assert_eq!(result.probability_source, "supplied_probability");
    }

    #[test]
    fn malformed_finite_inputs_and_impossible_yields_are_rejected() {
        let mut inputs = Vec::new();
        let mut input = finite_input();
        input.arity = 0;
        inputs.push(input);
        let mut input = finite_input();
        input.arity = MAX_FINITE_ARITY + 1;
        inputs.push(input);
        let mut input = finite_input();
        input.factor_base_population_log2 = 0.0;
        inputs.push(input);
        let mut input = finite_input();
        input.conditional_useful_retention = 0.0;
        inputs.push(input);
        let mut input = finite_input();
        input.conditional_useful_retention = 1.1;
        inputs.push(input);
        let mut input = finite_input();
        input.solvability_probability = Some(1.0);
        inputs.push(input);
        let mut input = finite_input();
        input.trial_work_log2 = f64::INFINITY;
        inputs.push(input);
        let mut input = finite_input();
        input.group_order_log2 = f64::NAN;
        inputs.push(input);
        assert!(inputs
            .iter()
            .all(|input| finite_relation_budget(input).is_err()));
    }

    fn ledger() -> Vec<PhaseBudget> {
        vec![
            PhaseBudget {
                name: "setup".to_owned(),
                work: PhaseWork::Known(20.0),
                reusable: true,
            },
            PhaseBudget {
                name: "online".to_owned(),
                work: PhaseWork::Known(5.0),
                reusable: false,
            },
            PhaseBudget {
                name: "absent_descent".to_owned(),
                work: PhaseWork::Zero,
                reusable: false,
            },
        ]
    }

    #[test]
    fn cold_work_and_amortization_are_separate_log_sums() {
        let required = vec!["setup".to_owned(), "online".to_owned()];
        let result = aggregate_phase_budgets(&ledger(), &required, 10.0).unwrap();
        close(result.cold_work_log2.unwrap(), 1_048_608.0_f64.log2());
        close(result.total_work_log2.unwrap(), 1_081_344.0_f64.log2());
        close(result.amortized_work_log2.unwrap(), 1056.0_f64.log2());
        close(result.reusable_work_log2.unwrap(), 20.0);
        close(result.online_work_log2.unwrap(), 5.0);
        close(result.largest_cold_phase_log2.unwrap(), 20.0);
        assert_eq!(result.dominant_cold_phases, vec!["setup"]);
        let single = aggregate_phase_budgets(&ledger(), &required, 0.0).unwrap();
        assert_eq!(single.cold_work_log2, single.total_work_log2);
        assert_eq!(single.cold_work_log2, single.amortized_work_log2);
    }

    #[test]
    fn target_dependent_collection_is_never_automatically_amortized() {
        let phases = vec![PhaseBudget {
            name: "relations".to_owned(),
            work: PhaseWork::Known(60.0),
            reusable: false,
        }];
        let result = aggregate_phase_budgets(&phases, &["relations".to_owned()], 8.0).unwrap();
        close(result.total_work_log2.unwrap(), 68.0);
        close(result.amortized_work_log2.unwrap(), 60.0);
        assert_eq!(result.reusable_work_log2, None);
    }

    #[test]
    fn log_sum_does_not_overflow_for_large_work() {
        let phases = vec![
            PhaseBudget {
                name: "a".to_owned(),
                work: PhaseWork::Known(4096.0),
                reusable: false,
            },
            PhaseBudget {
                name: "b".to_owned(),
                work: PhaseWork::Known(4096.0),
                reusable: false,
            },
        ];
        let result = aggregate_phase_budgets(&phases, &[], 0.0).unwrap();
        close(result.cold_work_log2.unwrap(), 4097.0);
        assert_eq!(result.dominant_cold_phases, vec!["a", "b"]);
    }

    #[test]
    fn missing_and_unknown_phases_do_not_become_zero() {
        assert_eq!(
            aggregate_phase_budgets(&ledger(), &["verification".to_owned()], 0.0)
                .unwrap_err()
                .code,
            "missing_phase"
        );
        let mut phases = ledger();
        phases[0].work = PhaseWork::Unknown;
        assert_eq!(
            aggregate_phase_budgets(&phases, &[], 0.0).unwrap_err().code,
            "unknown_phase_cost"
        );
        phases[0].work = PhaseWork::Known(f64::NEG_INFINITY);
        assert!(aggregate_phase_budgets(&phases, &[], 0.0).is_err());
        assert!(aggregate_phase_budgets(&ledger(), &[], -1.0).is_err());
        let mut phases = ledger();
        phases.push(phases[0].clone());
        assert!(aggregate_phase_budgets(&phases, &[], 0.0).is_err());
    }

    #[test]
    fn exact_zero_work_is_explicit_and_serializable() {
        let phases = vec![PhaseBudget {
            name: "none".to_owned(),
            work: PhaseWork::Zero,
            reusable: false,
        }];
        let result = aggregate_phase_budgets(&phases, &["none".to_owned()], 2.0).unwrap();
        assert_eq!(result.cold_work_log2, None);
        assert_eq!(result.total_work_log2, None);
        assert_eq!(result.amortized_work_log2, None);
        assert!(result.dominant_cold_phases.is_empty());
    }

    #[test]
    fn normalized_slope_threshold_is_zero() {
        let slow = compare_slopes(0.6, 0.5).unwrap();
        close(slow.normalized_slope, 0.1);
        assert!(!slow.below_rho_exponent);
        let fast = compare_slopes(0.4, 0.5).unwrap();
        close(fast.normalized_slope, -0.1);
        assert!(fast.below_rho_exponent);
        let equal = compare_slopes(0.5, 0.5).unwrap();
        assert_eq!(equal.normalized_slope, 0.0);
        assert!(!equal.below_rho_exponent);
        assert!(compare_slopes(f64::NAN, 0.5).is_err());
    }
}
