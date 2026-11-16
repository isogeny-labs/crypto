//! Certificate types for `transfer-audit`.
//!
//! One certificate describes one candidate transfer of one curve's discrete
//! logarithm.  The vocabulary is closed on purpose: evidence statuses,
//! check results, cost-cell labels and verdicts are enums, so a certificate
//! cannot say anything the protocol does not define.  Free-form detail lives
//! in `serde_json::Value` objects, whose keys serialize sorted, so a
//! regenerated certificate is byte-identical.

use serde::Serialize;
use serde_json::{json, Map, Value};

/// v2 (Amendment 3): adds `evaluation`, `costs.log2_total_frozen_model`,
/// renames `baseline.log2_total_without_automorphisms` to
/// `log2_total_negation_only` (the cost model's reference already includes
/// negation), and redefines `obligations.computational_advantage` (see
/// [`crate::ctx::judge`]).  v1 certificates are not readable as v2.
pub const SCHEMA: &str = "transfer-audit/certificate/v2";
pub const TOOL: &str = "transfer-audit/2";

/// Status of the subgroup-preservation obligation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Evidence {
    /// The map was executed on the actual generator and every check passed.
    Verified,
    /// A proof-level argument whose antecedents were checked, map not executed.
    Arithmetic,
    /// No code in the repository can discharge the obligation.
    NotEvaluated,
    /// Executed or arithmetic evidence shows the subgroup is not carried.
    Refuted,
    /// The transfer does not exist for this curve.
    NotApplicable,
}

/// Result of one check inside a certificate (`audit-curve` vocabulary).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum CheckResult {
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "NOT_RUN")]
    NotRun,
    #[serde(rename = "INDETERMINATE")]
    Indeterminate,
    #[serde(rename = "NOT_APPLICABLE")]
    NotApplicable,
}

/// How a cost cell was obtained.  `Measured` is part of the vocabulary; no
/// cost row of this audit is measured (the measured control is a separate
/// record), and the protocol says so.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum CostStatus {
    Modeled,
    Measured,
    NotEvaluated,
    NotApplicable,
}

/// The closed verdict vocabulary.  `ImplementationWeak` is part of it but this
/// tool audits transfers only and never assigns it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[allow(dead_code)]
pub enum Verdict {
    #[serde(rename = "class-weak")]
    ClassWeak,
    #[serde(rename = "weak representative exists")]
    WeakRepresentativeExists,
    #[serde(rename = "source-transfer-weak")]
    SourceTransferWeak,
    #[serde(rename = "implementation-weak")]
    ImplementationWeak,
    #[serde(rename = "no weakness found in scope")]
    NoWeaknessFoundInScope,
}

impl Verdict {
    #[cfg(test)]
    pub fn label(self) -> &'static str {
        match self {
            Verdict::ClassWeak => "class-weak",
            Verdict::WeakRepresentativeExists => "weak representative exists",
            Verdict::SourceTransferWeak => "source-transfer-weak",
            Verdict::ImplementationWeak => "implementation-weak",
            Verdict::NoWeaknessFoundInScope => "no weakness found in scope",
        }
    }

    /// Strength order used to aggregate a curve's verdict: any weakness
    /// outranks `no weakness found in scope`.
    pub fn strength(self) -> u8 {
        match self {
            Verdict::NoWeaknessFoundInScope => 0,
            Verdict::ImplementationWeak => 1,
            Verdict::WeakRepresentativeExists => 2,
            Verdict::SourceTransferWeak => 3,
            Verdict::ClassWeak => 4,
        }
    }
}

/// Where a formula-level price sits against the baseline
/// (`jacobian_transfers` classes, plus `NotPriced`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LeadClass {
    FormulaAdvantage,
    FormulaMarginal,
    Boundary,
    NotCarried,
    NotPriced,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferType {
    GhsWeilDescent,
    IsogenyMove,
    GenusCover,
    PairingTransfer,
}

impl TransferType {
    pub fn label(self) -> &'static str {
        match self {
            TransferType::GhsWeilDescent => "Weil/GHS descent",
            TransferType::IsogenyMove => "isogeny move",
            TransferType::GenusCover => "genus-g cover",
            TransferType::PairingTransfer => "pairing (MOV/Frey-Ruck)",
        }
    }
}

/// Whether a candidate route was evaluated far enough for its verdict to
/// carry information.  A field of the certificate, not a verdict label:
/// `no weakness found in scope` is the verdict of all three, and this says
/// which kind of "no weakness" it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteStatus {
    /// The route was constructed or covered by a checked theorem, priced
    /// completely and compared with the matched baseline (the comparison may
    /// still be inconclusive: a marginal lead or a straddling band).
    Evaluated,
    /// The route cannot yield a weakness: it does not carry `<G>` (or does
    /// not exist for this curve), or its price, a lower bound where phases
    /// are unpriced, is not below the matched baseline.
    Excluded,
    /// The route may exist but the repository cannot construct or price it
    /// far enough to decide; the verdict is vacuous for this candidate.
    NotEvaluable,
}

/// [`RouteStatus`] with the reason, in words.
#[derive(Clone, Debug, Serialize)]
pub struct Evaluation {
    pub status: RouteStatus,
    pub reason: String,
}

/// How the map itself exists in this run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Construction {
    #[serde(rename = "executed")]
    Executed,
    #[serde(rename = "arithmetic, not executed")]
    ArithmeticNotExecuted,
    #[serde(rename = "not evaluated")]
    NotEvaluated,
    #[serde(rename = "out of scope")]
    OutOfScope,
    #[serde(rename = "not applicable")]
    NotApplicable,
}

#[derive(Clone, Debug, Serialize)]
pub struct Check {
    pub name: String,
    pub result: CheckResult,
    pub evidence: String,
}

impl Check {
    pub fn new(name: &str, result: CheckResult, evidence: impl Into<String>) -> Self {
        Check {
            name: name.to_string(),
            result,
            evidence: evidence.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CostRow {
    pub phase: &'static str,
    pub log2: Option<f64>,
    pub status: CostStatus,
    pub formula: String,
}

impl CostRow {
    pub fn modeled(phase: &'static str, log2: f64, formula: impl Into<String>) -> Self {
        CostRow {
            phase,
            log2: Some(round3(log2)),
            status: CostStatus::Modeled,
            formula: formula.into(),
        }
    }

    pub fn absent(phase: &'static str, status: CostStatus, formula: impl Into<String>) -> Self {
        CostRow {
            phase,
            log2: None,
            status,
            formula: formula.into(),
        }
    }
}

pub fn round3(x: f64) -> f64 {
    if !x.is_finite() {
        return x;
    }
    let r = (x * 1000.0).round() / 1000.0;
    // No negative zero in a written certificate.
    if r == 0.0 {
        0.0
    } else {
        r
    }
}

/// `log2(2^a + 2^b)`, tolerant of `-inf`.
pub fn log2_add(a: f64, b: f64) -> f64 {
    if a == f64::NEG_INFINITY {
        return b;
    }
    if b == f64::NEG_INFINITY {
        return a;
    }
    let (hi, lo) = if a >= b { (a, b) } else { (b, a) };
    hi + (1.0 + (lo - hi).exp2()).log2()
}

/// The five-phase cost table of one candidate, in `log2` bit operations.
#[derive(Clone, Debug, Serialize)]
pub struct CostTable {
    pub unit: &'static str,
    pub rows: Vec<CostRow>,
    /// Solver chosen for the destination, if the route has one.
    pub destination_solver: Option<String>,
    /// Every destination solver priced: `(name, log2 total)`.
    pub solvers_considered: Vec<(String, f64)>,
    pub log2_total: Option<f64>,
    /// A required phase is `not_evaluated`, so `log2_total` omits it and is a lower bound on the route.
    pub total_is_lower_bound: bool,
    /// `log2` of the matched baseline (rho with the curve's equivalence class).
    pub log2_baseline: f64,
    /// `log2(route / baseline)`: positive means the route costs more.
    pub log2_cost_ratio: Option<f64>,
    /// `reference - total` against the cost model's own rho reference
    /// (`sqrt(pi r / 4)`: negation only), the sign and reference of
    /// `docs/curves/jacobian-transfers.json`.
    pub repo_ratio_log2: Option<f64>,
    /// Present only when `log2_total` carries an equivalence-class discount
    /// on rho in the destination that the frozen model does not have (the
    /// Frobenius class of a cover image): the total under the frozen model
    /// alone, which is what the cross-check against the repository's
    /// `jacobian-transfers.json` compares.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log2_total_frozen_model: Option<f64>,
}

impl CostTable {
    /// Sum the priced rows.  A row with no value does not enter the total;
    /// if any row is `NotEvaluated` the total is flagged a lower bound.
    pub fn with_rows(rows: Vec<CostRow>, log2_baseline: f64, log2_reference_no_aut: f64) -> Self {
        let total = rows
            .iter()
            .filter_map(|r| r.log2)
            .fold(f64::NEG_INFINITY, log2_add);
        let priced = total.is_finite();
        let lower = rows.iter().any(|r| r.status == CostStatus::NotEvaluated);
        CostTable {
            unit: "log2 bit operations (frozen formula model; measured rows say so)",
            rows,
            destination_solver: None,
            solvers_considered: Vec::new(),
            log2_total: priced.then(|| round3(total)),
            total_is_lower_bound: lower,
            log2_baseline: round3(log2_baseline),
            log2_cost_ratio: priced.then(|| round3(total - log2_baseline)),
            repo_ratio_log2: priced.then(|| round3(log2_reference_no_aut - total)),
            log2_total_frozen_model: None,
        }
    }

    pub fn unpriced(log2_baseline: f64, why: &str) -> Self {
        CostTable {
            unit: "log2 bit operations (frozen formula model; measured rows say so)",
            rows: vec![CostRow::absent(
                "destination_solve_unsplit",
                CostStatus::NotApplicable,
                why,
            )],
            destination_solver: None,
            solvers_considered: Vec::new(),
            log2_total: None,
            total_is_lower_bound: false,
            log2_baseline: round3(log2_baseline),
            log2_cost_ratio: None,
            repo_ratio_log2: None,
            log2_total_frozen_model: None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SubgroupPreservation {
    pub status: Evidence,
    pub method: String,
    pub argument: String,
    pub image_order: Option<String>,
    pub kernel_meets_subgroup: Option<bool>,
    /// Antecedents of an arithmetic argument, each with its own result.
    pub antecedents: Vec<Check>,
    /// Detail of an executed check (counts of samples passed, and so on).
    pub executed: Option<Value>,
}

impl SubgroupPreservation {
    pub fn not_evaluated(method: &str, argument: &str) -> Self {
        SubgroupPreservation {
            status: Evidence::NotEvaluated,
            method: method.to_string(),
            argument: argument.to_string(),
            image_order: None,
            kernel_meets_subgroup: None,
            antecedents: Vec::new(),
            executed: None,
        }
    }
}

/// The six obligations of the `transfer` skill, each with a state.
#[derive(Clone, Debug, Serialize)]
pub struct Obligations {
    pub existence: &'static str,
    pub explicit_construction: &'static str,
    pub map_correctness: &'static str,
    pub subgroup_preservation: &'static str,
    pub computational_advantage: &'static str,
    pub scaling_support: &'static str,
}

#[derive(Clone, Debug, Serialize)]
pub struct Certificate {
    pub schema: &'static str,
    pub tool: &'static str,
    pub curve: Value,
    pub candidate: Value,
    pub map: Value,
    pub obligations: Obligations,
    pub subgroup_preservation: SubgroupPreservation,
    pub baseline: Value,
    /// The subgroup is, or may be, carried by the destination.  When false the
    /// cost table is priced for the record only: the route solves a different
    /// problem, so its ratio to the baseline is not an attack cost.
    pub carried: bool,
    pub costs: CostTable,
    pub lead_class: LeadClass,
    pub verdict: Verdict,
    pub verdict_basis: Vec<String>,
    /// Whether this candidate was evaluated, excluded or not evaluable (a
    /// field, never a verdict label).
    pub evaluation: Evaluation,
    pub checks: Vec<Check>,
    pub invariants: Map<String, Value>,
    pub executed: Option<Value>,
    pub open_obligations: Vec<String>,
    pub notes: Vec<String>,
    pub references: Vec<String>,
}

pub fn candidate_value(id: &str, ty: TransferType, label: &str) -> Value {
    json!({ "id": id, "transfer_type": ty, "label": label })
}

#[allow(clippy::too_many_arguments)]
pub fn map_value(
    map_type: &str,
    source: &str,
    destination: &str,
    degrees: Value,
    field_of_definition: &str,
    construction: Construction,
    explicit: &str,
) -> Value {
    json!({
        "type": map_type,
        "source": source,
        "destination": destination,
        "degrees": degrees,
        "field_of_definition": field_of_definition,
        "construction_status": construction,
        "explicit_formulas": explicit,
    })
}

pub fn obligation_state(e: Evidence) -> &'static str {
    match e {
        Evidence::Verified | Evidence::Arithmetic => "supported",
        Evidence::NotEvaluated => "unknown",
        Evidence::Refuted => "refuted",
        Evidence::NotApplicable => "not_applicable",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verdict_vocabulary_is_closed_and_ordered() {
        let all = [
            Verdict::NoWeaknessFoundInScope,
            Verdict::ImplementationWeak,
            Verdict::WeakRepresentativeExists,
            Verdict::SourceTransferWeak,
            Verdict::ClassWeak,
        ];
        let labels: Vec<_> = all.iter().map(|v| v.label()).collect();
        assert_eq!(
            labels,
            vec![
                "no weakness found in scope",
                "implementation-weak",
                "weak representative exists",
                "source-transfer-weak",
                "class-weak"
            ]
        );
        for (i, v) in all.iter().enumerate() {
            assert_eq!(v.strength() as usize, i);
            // The serialized form equals the label, so a certificate can
            // only carry a vocabulary string.
            assert_eq!(
                serde_json::to_value(v).unwrap(),
                Value::String(v.label().to_string())
            );
        }
    }

    #[test]
    fn cost_table_totals_priced_rows_only() {
        let rows = vec![
            CostRow::modeled("a", 10.0, "x"),
            CostRow::modeled("b", 10.0, "y"),
            CostRow::absent("c", CostStatus::NotApplicable, "z"),
        ];
        let t = CostTable::with_rows(rows, 5.0, 5.5);
        assert_eq!(t.log2_total, Some(11.0));
        assert_eq!(t.log2_cost_ratio, Some(6.0));
        assert_eq!(t.repo_ratio_log2, Some(-5.5));
        let partial = CostTable::with_rows(
            vec![
                CostRow::modeled("a", 10.0, "x"),
                CostRow::absent("b", CostStatus::NotEvaluated, "y"),
            ],
            5.0,
            5.0,
        );
        assert!(partial.total_is_lower_bound);
        assert_eq!(partial.log2_total, Some(10.0));
        let none = CostTable::with_rows(
            vec![CostRow::absent("b", CostStatus::NotEvaluated, "y")],
            5.0,
            5.0,
        );
        assert_eq!(none.log2_total, None);
        // The frozen-model total is absent unless a class discount was applied.
        assert_eq!(none.log2_total_frozen_model, None);
        let v = serde_json::to_value(&none).unwrap();
        assert!(v.get("log2_total_frozen_model").is_none());
    }

    /// The excluded / not-evaluable distinction is a field with a closed
    /// vocabulary, not a new verdict label.
    #[test]
    fn route_status_vocabulary_is_closed() {
        for (s, label) in [
            (RouteStatus::Evaluated, "evaluated"),
            (RouteStatus::Excluded, "excluded"),
            (RouteStatus::NotEvaluable, "not_evaluable"),
        ] {
            assert_eq!(
                serde_json::to_value(s).unwrap(),
                Value::String(label.into())
            );
        }
        assert_eq!(SCHEMA, "transfer-audit/certificate/v2");
        assert_eq!(TOOL, "transfer-audit/2");
    }
}
