//! Registry-wide optimization coverage and campaign auditing.
//!
//! This module does not turn adaptive search observations into performance
//! claims.  It makes the search surface explicit, keeps every registered
//! method in one inventory, and checks the boundary between development and
//! confirmatory holdout specifications.  Actual solves still run through
//! [`super::runner`], with the same records and audit path as any other
//! `ecbench` session.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::{json, Value};

use super::methods::{self, MethodDecl};
use super::spec::{Level, Role, Spec};
use crate::cryptanalysis::ic_framework::solvers::solver_registry;

pub const INVENTORY_SCHEMA: &str = "ecbench.optimization-inventory/v1";
pub const AUDIT_SCHEMA: &str = "ecbench.optimization-audit/v1";

/// How configuration candidates are proposed.  Every method is also eligible
/// for a source-level A/B loop; this field describes its configuration loop.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchStrategy {
    /// Conditional, scale-relative search with rungs (the native `ic sweep`
    /// pattern).  Development observations are never promotion evidence.
    NativeAdaptive,
    /// A prospectively frozen finite grid or successive-halving schedule.
    NativeGrid,
    /// No algorithmic configuration knob: optimize source against the exact
    /// same method configuration and output.
    ImplementationAb,
    /// The measured implementation is supplied as a hash-bound native binary.
    ExternalBackendAb,
    /// Historical identity retained only for replay and drift controls.
    ReplayControl,
}

/// What a successful loop is allowed to establish before any method-specific
/// correctness, pricing, replay, and external-reference gates are applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimScope {
    FrontierEligible,
    SpecializedDomain,
    DiagnosticOnly,
    PricingSensitivity,
    FrozenHistorical,
}

/// Accounting scopes are never silently compared as though they were one
/// unit.  `StandardBand` can still contain named unpriced work; that work must
/// be priced symmetrically before promotion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountingScope {
    StandardBand,
    FullPrice,
    WordRam,
    JacobianImageNative,
    ExternalNative,
    DiagnosticModel,
}

/// A method parameter's role in an optimization campaign.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterRole {
    /// A legitimate candidate dimension, with a prospectively frozen domain.
    Knob,
    /// A timeout, cap, or censoring envelope.  It is fixed across arms and is
    /// not an objective the search may game.
    ResourceEnvelope,
    /// A controlled nuisance seed.  Training and holdout values stay disjoint.
    Seed,
    /// Changes the advice or target multiplicity and therefore starts a new
    /// comparison scope rather than becoming a speed knob.
    WorkloadScope,
    /// Path or command of an externally supplied native implementation.
    ExternalBinding,
    /// Hash or accounting coefficient that binds identity/pricing.
    IdentityOrPrice,
}

#[derive(Clone, Copy, Debug)]
struct PolicyDecl {
    id: &'static str,
    strategy: SearchStrategy,
    claim_scope: ClaimScope,
    accounting_scope: AccountingScope,
}

macro_rules! policy {
    ($id:literal, $strategy:ident, $claim:ident, $accounting:ident) => {
        PolicyDecl {
            id: $id,
            strategy: SearchStrategy::$strategy,
            claim_scope: ClaimScope::$claim,
            accounting_scope: AccountingScope::$accounting,
        }
    };
}

// Deliberately explicit.  A new method must choose a search strategy and a
// claim/accounting scope; the registry-exhaustiveness test fails otherwise.
const POLICIES: &[PolicyDecl] = &[
    policy!("ic.aps_v1", NativeAdaptive, SpecializedDomain, StandardBand),
    policy!(
        "ic.three_leads_genus2_pencil_symbolic_v2",
        NativeGrid,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "ic.three_leads_genus2_choice2_v1",
        ImplementationAb,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "ic.three_leads_genus2_projective_chart_v1",
        ImplementationAb,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "ic.three_leads_genus2_target_rank_v1",
        ImplementationAb,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "rho.three_leads_plain_v1",
        ImplementationAb,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "rho.three_leads_negation_v1",
        ImplementationAb,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "rho.three_leads_jacobian_v1",
        ImplementationAb,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "ic.three_leads_genus2_v2",
        NativeAdaptive,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "ic.three_leads_genus2_v1",
        ReplayControl,
        FrozenHistorical,
        StandardBand
    ),
    policy!(
        "ic.three_leads_decomposition_v2",
        NativeAdaptive,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "ic.three_leads_decomposition_v1",
        ReplayControl,
        FrozenHistorical,
        StandardBand
    ),
    policy!(
        "rho.plus_model_v1",
        NativeGrid,
        DiagnosticOnly,
        DiagnosticModel
    ),
    policy!(
        "rho.best2_model_v1",
        NativeGrid,
        DiagnosticOnly,
        DiagnosticModel
    ),
    policy!(
        "rho.frozen_reference",
        ReplayControl,
        FrozenHistorical,
        StandardBand
    ),
    policy!("rho.plain", ReplayControl, FrozenHistorical, StandardBand),
    policy!(
        "rho.negation",
        ReplayControl,
        FrozenHistorical,
        StandardBand
    ),
    policy!(
        "rho.negation_strong",
        NativeGrid,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "rho.multitarget_pooled",
        ImplementationAb,
        DiagnosticOnly,
        StandardBand
    ),
    policy!(
        "rho.preprocessing",
        NativeAdaptive,
        DiagnosticOnly,
        StandardBand
    ),
    policy!(
        "ic.k4_gaudry",
        NativeAdaptive,
        SpecializedDomain,
        StandardBand
    ),
    policy!("rho.cubic", ReplayControl, FrozenHistorical, StandardBand),
    policy!("rho.quartic", ReplayControl, FrozenHistorical, StandardBand),
    policy!(
        "rho.frobenius_orbit",
        ImplementationAb,
        SpecializedDomain,
        StandardBand
    ),
    policy!("rho.gls", ImplementationAb, SpecializedDomain, StandardBand),
    policy!(
        "rho.signed_frobenius",
        ReplayControl,
        FrozenHistorical,
        StandardBand
    ),
    policy!(
        "rho.signed_frobenius_strong",
        ReplayControl,
        FrozenHistorical,
        StandardBand
    ),
    policy!(
        "rho.signed_frobenius_strong_legacy",
        ReplayControl,
        FrozenHistorical,
        StandardBand
    ),
    policy!(
        "rho.signed_frobenius_strong_escape",
        NativeGrid,
        FrontierEligible,
        StandardBand
    ),
    policy!(
        "jacobian_image.rho3",
        ImplementationAb,
        SpecializedDomain,
        JacobianImageNative
    ),
    policy!(
        "jacobian_image.bsgs",
        ImplementationAb,
        SpecializedDomain,
        JacobianImageNative
    ),
    policy!(
        "jacobian_image.rho3_field_tally",
        ImplementationAb,
        DiagnosticOnly,
        JacobianImageNative
    ),
    policy!(
        "jacobian_image.bsgs_field_tally",
        ImplementationAb,
        DiagnosticOnly,
        JacobianImageNative
    ),
    policy!(
        "bsgs.textbook",
        ReplayControl,
        FrozenHistorical,
        StandardBand
    ),
    policy!(
        "bsgs.interleaved",
        ReplayControl,
        FrozenHistorical,
        StandardBand
    ),
    policy!(
        "bsgs.negation",
        ReplayControl,
        FrozenHistorical,
        StandardBand
    ),
    policy!(
        "bsgs.negation_table",
        NativeGrid,
        DiagnosticOnly,
        StandardBand
    ),
    policy!(
        "kangaroo.vow",
        ReplayControl,
        FrozenHistorical,
        StandardBand
    ),
    policy!(
        "transfer.isogeny",
        NativeGrid,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "transfer.quartic_cover",
        ReplayControl,
        FrozenHistorical,
        StandardBand
    ),
    policy!(
        "transfer.quartic_cover_v2",
        NativeGrid,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "transfer.ghs_cover",
        NativeGrid,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "ic.hyperelliptic",
        ReplayControl,
        FrozenHistorical,
        StandardBand
    ),
    policy!(
        "ic.hyperelliptic_v2",
        NativeAdaptive,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "claw.pair_table",
        ReplayControl,
        FrozenHistorical,
        StandardBand
    ),
    policy!(
        "ic.cm_folded_native_v1",
        ReplayControl,
        FrozenHistorical,
        ExternalNative
    ),
    policy!(
        "ic.cm_folded_recurrence_v1",
        ReplayControl,
        FrozenHistorical,
        ExternalNative
    ),
    policy!(
        "ic.cm_folded_native_d43_v1",
        ReplayControl,
        FrozenHistorical,
        ExternalNative
    ),
    policy!(
        "ic.cm_folded_native_v2",
        ExternalBackendAb,
        SpecializedDomain,
        ExternalNative
    ),
    policy!(
        "ic.cm_folded_recurrence_v2",
        ExternalBackendAb,
        SpecializedDomain,
        ExternalNative
    ),
    policy!(
        "ic.cm_folded_native_d43_v2",
        ExternalBackendAb,
        SpecializedDomain,
        ExternalNative
    ),
    policy!(
        "ic.hyperelliptic_thread",
        NativeAdaptive,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "ic.pipeline",
        NativeAdaptive,
        FrontierEligible,
        StandardBand
    ),
    policy!(
        "ic.pipeline_full",
        NativeAdaptive,
        PricingSensitivity,
        FullPrice
    ),
    policy!(
        "rho.signed_frobenius_full",
        ImplementationAb,
        PricingSensitivity,
        FullPrice
    ),
    policy!(
        "rho.signed_frobenius_strong_full",
        NativeGrid,
        PricingSensitivity,
        FullPrice
    ),
    policy!(
        "ic.pipeline_wordram",
        NativeAdaptive,
        PricingSensitivity,
        WordRam
    ),
    policy!(
        "rho.signed_frobenius_wordram",
        ImplementationAb,
        PricingSensitivity,
        WordRam
    ),
    policy!(
        "rho.signed_frobenius_strong_wordram",
        NativeGrid,
        PricingSensitivity,
        WordRam
    ),
    policy!(
        "ic.pipeline_counted",
        NativeAdaptive,
        DiagnosticOnly,
        StandardBand
    ),
    policy!(
        "ic.tensor_contraction_v1",
        NativeGrid,
        DiagnosticOnly,
        StandardBand
    ),
    policy!(
        "ic.known_log_mitm2_v1",
        NativeAdaptive,
        DiagnosticOnly,
        StandardBand
    ),
    policy!(
        "ic.known_log_ksum_v1",
        NativeAdaptive,
        DiagnosticOnly,
        StandardBand
    ),
    policy!(
        "ic.large_prime",
        ReplayControl,
        FrozenHistorical,
        StandardBand
    ),
    policy!(
        "ic.trace_line_2lp",
        NativeAdaptive,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "ic.shared_rank",
        NativeAdaptive,
        DiagnosticOnly,
        StandardBand
    ),
    policy!(
        "ic.koblitz_descent",
        NativeAdaptive,
        SpecializedDomain,
        StandardBand
    ),
    policy!(
        "rho.negation_v2",
        ImplementationAb,
        FrontierEligible,
        StandardBand
    ),
    policy!(
        "rho.cubic_v2",
        ImplementationAb,
        FrontierEligible,
        StandardBand
    ),
    policy!(
        "rho.quartic_v2",
        ImplementationAb,
        FrontierEligible,
        StandardBand
    ),
    policy!(
        "rho.signed_frobenius_v2",
        ImplementationAb,
        FrontierEligible,
        StandardBand
    ),
    policy!(
        "rho.signed_frobenius_strong_v2",
        NativeGrid,
        FrontierEligible,
        StandardBand
    ),
    policy!(
        "bsgs.textbook_v2",
        ImplementationAb,
        FrontierEligible,
        StandardBand
    ),
    policy!(
        "bsgs.interleaved_v2",
        ImplementationAb,
        FrontierEligible,
        StandardBand
    ),
    policy!(
        "bsgs.negation_v2",
        ImplementationAb,
        FrontierEligible,
        StandardBand
    ),
    policy!(
        "kangaroo.vow_v2",
        ImplementationAb,
        FrontierEligible,
        StandardBand
    ),
    policy!(
        "claw.pair_table_v2",
        NativeGrid,
        FrontierEligible,
        StandardBand
    ),
    policy!(
        "ic.large_prime_v2",
        NativeAdaptive,
        FrontierEligible,
        StandardBand
    ),
];

fn parameter_role(name: &str) -> Option<ParameterRole> {
    use ParameterRole::*;
    Some(match name {
        "base_seed" | "rank_seed" | "seed" | "table_seed" => Seed,
        "batch" => WorkloadScope,
        "backend" => ExternalBinding,
        "backend_sha256" | "c4" => IdentityOrPrice,
        "attempts"
        | "budget_ms"
        | "cap_multiple"
        | "matrix_coefficient_cap"
        | "max_pair_work"
        | "max_restarts"
        | "max_states"
        | "max_sums"
        | "max_trials"
        | "max_u"
        | "max_units"
        | "precompute_cap"
        | "query_limit"
        | "rank_max_trials"
        | "sampling_draws"
        | "seconds"
        | "solver_budget_seconds"
        | "step_cap_factor"
        | "target_max_attempts"
        | "trial_cap"
        | "unit_trials"
        | "units" => ResourceEnvelope,
        "baby" | "base" | "base_multiplier" | "base_scale" | "beta" | "branches"
        | "collection_aim" | "collection_window" | "d" | "d_step_in" | "degree"
        | "descent_summands" | "dp_bits" | "ell" | "encoding" | "entries"
        | "envelope_dimension" | "extra_relations" | "factor_base" | "factor_base_points"
        | "fb_size" | "independent" | "inner" | "jumps" | "k" | "labels" | "lambda" | "lanes"
        | "large_primes" | "linalg" | "line" | "lp_exponent" | "m" | "mode" | "oracle"
        | "orbit" | "prune" | "random_base" | "roots" | "size" | "small_denominator"
        | "small_dimension" | "solver" | "stop" | "subset_size" | "summands" | "table_scale"
        | "targets" | "unique_relations" | "walks" => Knob,
        _ => return None,
    })
}

#[derive(Debug, Serialize)]
pub struct ParameterProfile<'a> {
    pub name: &'a str,
    pub role: ParameterRole,
    pub default: Option<&'a str>,
    pub help: &'a str,
}

#[derive(Debug, Serialize)]
pub struct MethodProfile<'a> {
    pub id: &'a str,
    pub family: &'a str,
    pub applies: super::methods::Applies,
    pub entry: &'a str,
    pub strategy: SearchStrategy,
    pub claim_scope: ClaimScope,
    pub accounting_scope: AccountingScope,
    pub parameters: Vec<ParameterProfile<'a>>,
}

fn policy_for(id: &str) -> Result<&'static PolicyDecl, String> {
    let mut found = POLICIES.iter().filter(|p| p.id == id);
    let first = found
        .next()
        .ok_or_else(|| format!("registered method `{id}` has no optimization policy"))?;
    if found.next().is_some() {
        return Err(format!("method `{id}` has duplicate optimization policies"));
    }
    Ok(first)
}

fn profile(method: &'static MethodDecl) -> Result<MethodProfile<'static>, String> {
    let policy = policy_for(method.id)?;
    let parameters = method
        .params
        .iter()
        .map(|p| {
            let role = parameter_role(p.name).ok_or_else(|| {
                format!(
                    "method `{}` parameter `{}` has no optimization role",
                    method.id, p.name
                )
            })?;
            Ok(ParameterProfile {
                name: p.name,
                role,
                default: p.default,
                help: p.help,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(MethodProfile {
        id: method.id,
        family: method.family,
        applies: method.applies,
        entry: method.entry,
        strategy: policy.strategy,
        claim_scope: policy.claim_scope,
        accounting_scope: policy.accounting_scope,
        parameters,
    })
}

pub fn profiles() -> Result<Vec<MethodProfile<'static>>, String> {
    let registry_ids: BTreeSet<_> = methods::registry().iter().map(|m| m.id).collect();
    let policy_ids: BTreeSet<_> = POLICIES.iter().map(|p| p.id).collect();
    if registry_ids != policy_ids {
        let missing: Vec<_> = registry_ids.difference(&policy_ids).copied().collect();
        let stale: Vec<_> = policy_ids.difference(&registry_ids).copied().collect();
        return Err(format!(
            "optimization policy/registry mismatch; missing={missing:?}, stale={stale:?}"
        ));
    }
    methods::registry().iter().map(profile).collect()
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum NestedKind {
    SystemSolver,
    Pdp3Engine,
    PkmTowerEngine,
}

#[derive(Debug, Serialize)]
struct NestedAlgorithm {
    key: String,
    name: String,
    kind: NestedKind,
    accounting: &'static str,
}

const PDP3_ENGINES: &[&str] = &[
    "inherited-f4",
    "f6-ic",
    "f6-ic-pair",
    "matrix-f4",
    "matrix-f5",
    "rootfind",
    "rootfind-window",
    "rootfind-negfold",
    "rootfind-window-negfold",
];

const PKM_TOWER_ENGINES: &[&str] = &["f4", "sig-f5", "f6", "enum"];

fn nested_algorithms() -> Vec<NestedAlgorithm> {
    let mut out: Vec<_> = solver_registry()
        .into_iter()
        .map(|solver| NestedAlgorithm {
            key: format!("system_solver/{}", solver.name()),
            name: solver.name().into(),
            kind: NestedKind::SystemSolver,
            accounting: "backend-native operation unit; rank only where the unit and scope match",
        })
        .collect();
    out.extend(PDP3_ENGINES.iter().map(|name| NestedAlgorithm {
        key: format!("pdp3_engine/{name}"),
        name: (*name).into(),
        kind: NestedKind::Pdp3Engine,
        accounting: if name.starts_with("rootfind") {
            "pair-table setup/probe route; not an algebraic-solver claim"
        } else {
            "solver-local work plus the enclosing pipeline accounting"
        },
    }));
    out.extend(PKM_TOWER_ENGINES.iter().map(|name| NestedAlgorithm {
        key: format!("pkm_tower_engine/{name}"),
        name: (*name).into(),
        kind: NestedKind::PkmTowerEngine,
        accounting: if *name == "enum" {
            "geometric enumeration through the enclosing pipeline accounting"
        } else {
            "prime-tower solver-local work plus the enclosing pipeline accounting"
        },
    }));
    out
}

pub fn inventory_json() -> Result<Value, String> {
    let profiles = profiles()?;
    let nested = nested_algorithms();
    let mut strategies: BTreeMap<SearchStrategy, usize> = BTreeMap::new();
    let mut claims: BTreeMap<ClaimScope, usize> = BTreeMap::new();
    for profile in &profiles {
        *strategies.entry(profile.strategy).or_default() += 1;
        *claims.entry(profile.claim_scope).or_default() += 1;
    }
    Ok(json!({
        "schema": INVENTORY_SCHEMA,
        "method_count": profiles.len(),
        "nested_algorithm_count": nested.len(),
        "coverage_rule": "exact equality with the live method registry; new methods fail closed",
        "development_rule": "adaptive/grid observations select candidates only and are never promotion evidence",
        "promotion_rule": "fresh holdout, same output/counts, A/A admission, complete symmetric pricing, independent replay, and the repository validation gate",
        "strategy_counts": strategies,
        "claim_scope_counts": claims,
        "methods": profiles,
        "nested_algorithms": nested,
    }))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    Development,
    Holdout,
}

impl Stage {
    fn name(self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Holdout => "holdout",
        }
    }
}

#[derive(Debug, Serialize)]
struct SpecAudit {
    path: PathBuf,
    label: String,
    methods: Vec<String>,
    nested_algorithms: Vec<String>,
    target_seed: u64,
    measurement_seed: u64,
    rounds: u32,
    warmup: u32,
    isolation_required: String,
    aa_pairs: usize,
    errors: Vec<String>,
    warnings: Vec<String>,
}

fn nested_coverage(spec: &Spec) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    for arm in &spec.arms {
        if let Some(solver) = arm.method.params.get("solver") {
            let name = solver.split(':').next().unwrap_or(solver);
            if solver_registry().iter().any(|s| s.name() == name) {
                found.insert(format!("system_solver/{name}"));
            }
        }
        if let Some(oracle) = arm.method.params.get("oracle") {
            for engine in PDP3_ENGINES {
                if oracle
                    .split(',')
                    .any(|part| part == format!("engine={engine}"))
                {
                    found.insert(format!("pdp3_engine/{engine}"));
                }
            }
            if oracle.split(':').next() == Some("pkm-tower") {
                for engine in PKM_TOWER_ENGINES {
                    if oracle
                        .split(',')
                        .any(|part| part == format!("engine={engine}"))
                    {
                        found.insert(format!("pkm_tower_engine/{engine}"));
                    }
                }
            }
        }
    }
    found
}

fn audit_spec(path: &Path, stage: Stage) -> Result<(Spec, SpecAudit), String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let spec = Spec::from_json(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    if spec.measurement.rounds < if stage == Stage::Holdout { 6 } else { 4 } {
        errors.push(format!(
            "{} needs at least {} measured rounds",
            stage.name(),
            if stage == Stage::Holdout { 6 } else { 4 }
        ));
    }
    if spec.measurement.rounds % 2 != 0 {
        errors.push("measured rounds must be even for balanced arm order".into());
    }
    if spec.measurement.warmup == 0 {
        errors.push("at least one warm-up round is required".into());
    }
    if stage == Stage::Holdout && spec.measurement.isolation_required < Level::L2 {
        errors.push("holdout clock evidence requires isolation_required >= L2".into());
    }
    if stage == Stage::Development && spec.measurement.isolation_required < Level::L2 {
        warnings.push(
            "development clocks are descriptive below L2; only deterministic counts may rank"
                .into(),
        );
    }
    if stage == Stage::Holdout && spec.workloads.target_kind.is_planted() {
        errors.push("confirmatory holdout must use public targets, not planted targets".into());
    }
    let mut names = BTreeSet::new();
    let mut resolved = Vec::new();
    for arm in &spec.arms {
        if !names.insert(&arm.name) {
            errors.push(format!("duplicate arm name `{}`", arm.name));
        }
        match methods::resolve(&arm.method) {
            Ok(method) => resolved.push((arm.role, method.method_sha256)),
            Err(e) => errors.push(format!("arm `{}`: {e}", arm.name)),
        }
    }
    let non_controls: BTreeSet<_> = resolved
        .iter()
        .filter(|(role, _)| *role != Role::Control)
        .map(|(_, id)| id)
        .collect();
    let aa_pairs = resolved
        .iter()
        .filter(|(role, id)| *role == Role::Control && non_controls.contains(id))
        .count();
    if aa_pairs == 0 {
        errors.push(
            "no control arm exactly duplicates a non-control method configuration (A/A)".into(),
        );
    }
    if !spec.arms.iter().any(|arm| arm.role == Role::Reference) {
        errors.push("no reference arm".into());
    }
    if !spec
        .arms
        .iter()
        .any(|arm| matches!(arm.role, Role::Baseline | Role::Candidate))
    {
        errors.push("no baseline or candidate arm".into());
    }

    // Within one method, a candidate may vary real knobs.  It may not gain by
    // changing censoring, seed, advice, external identity, or accounting price
    // relative to a baseline in the same spec.
    for candidate in spec.arms.iter().filter(|a| a.role == Role::Candidate) {
        for baseline in spec
            .arms
            .iter()
            .filter(|a| a.role == Role::Baseline && a.method.id == candidate.method.id)
        {
            if let (Ok(baseline_method), Ok(candidate_method)) = (
                methods::resolve(&baseline.method),
                methods::resolve(&candidate.method),
            ) {
                for key in baseline_method.params.keys() {
                    if parameter_role(key) != Some(ParameterRole::Knob)
                        && baseline_method.params.get(key) != candidate_method.params.get(key)
                    {
                        errors.push(format!(
                            "candidate `{}` changes non-knob `{key}` relative to baseline `{}`",
                            candidate.name, baseline.name
                        ));
                    }
                }
            }
        }
    }

    let methods = spec
        .arms
        .iter()
        .map(|a| a.method.id.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let nested_algorithms = nested_coverage(&spec).into_iter().collect();
    let audit = SpecAudit {
        path: path.to_path_buf(),
        label: spec.label.clone(),
        methods,
        nested_algorithms,
        target_seed: spec.workloads.target_seed,
        measurement_seed: spec.measurement.seed,
        rounds: spec.measurement.rounds,
        warmup: spec.measurement.warmup,
        isolation_required: spec.measurement.isolation_required.name().into(),
        aa_pairs,
        errors,
        warnings,
    };
    Ok((spec, audit))
}

fn audit_stage(paths: &[PathBuf], stage: Stage) -> Result<(Vec<Spec>, Vec<SpecAudit>), String> {
    paths
        .iter()
        .map(|path| audit_spec(path, stage))
        .collect::<Result<Vec<_>, _>>()
        .map(|rows| rows.into_iter().unzip())
}

/// Audit the split between adaptive development and confirmatory holdout.
/// When `require_all` is true, every live method and every registered system
/// solver/PDP3/PKM-tower engine must occur in development. Holdout is
/// intentionally a winner-only subset.
pub fn audit_campaign(
    development: &[PathBuf],
    holdout: &[PathBuf],
    require_all: bool,
) -> Result<Value, String> {
    profiles()?;
    let (development_specs, development_rows) = audit_stage(development, Stage::Development)?;
    let (holdout_specs, holdout_rows) = audit_stage(holdout, Stage::Holdout)?;

    let method_coverage = |specs: &[Spec]| {
        specs
            .iter()
            .flat_map(|s| s.arms.iter().map(|a| a.method.id.clone()))
            .collect::<BTreeSet<_>>()
    };
    let nested_coverage_all = |specs: &[Spec]| {
        specs
            .iter()
            .flat_map(nested_coverage)
            .collect::<BTreeSet<_>>()
    };
    let development_methods = method_coverage(&development_specs);
    let holdout_methods = method_coverage(&holdout_specs);
    let development_nested = nested_coverage_all(&development_specs);
    let holdout_nested = nested_coverage_all(&holdout_specs);
    let required_methods: BTreeSet<_> = methods::registry()
        .iter()
        .map(|m| m.id.to_string())
        .collect();
    let required_nested: BTreeSet<_> = nested_algorithms().into_iter().map(|a| a.key).collect();
    let missing_methods: Vec<_> = required_methods
        .difference(&development_methods)
        .cloned()
        .collect();
    let missing_nested: Vec<_> = required_nested
        .difference(&development_nested)
        .cloned()
        .collect();

    let development_target_seeds: BTreeSet<_> = development_specs
        .iter()
        .map(|s| s.workloads.target_seed)
        .collect();
    let holdout_target_seeds: BTreeSet<_> = holdout_specs
        .iter()
        .map(|s| s.workloads.target_seed)
        .collect();
    let development_measurement_seeds: BTreeSet<_> = development_specs
        .iter()
        .map(|s| s.measurement.seed)
        .collect();
    let holdout_measurement_seeds: BTreeSet<_> =
        holdout_specs.iter().map(|s| s.measurement.seed).collect();
    let reused_target_seeds: Vec<_> = development_target_seeds
        .intersection(&holdout_target_seeds)
        .copied()
        .collect();
    let reused_measurement_seeds: Vec<_> = development_measurement_seeds
        .intersection(&holdout_measurement_seeds)
        .copied()
        .collect();
    let row_errors = development_rows
        .iter()
        .chain(&holdout_rows)
        .any(|r| !r.errors.is_empty());
    let admissible = !row_errors
        && reused_target_seeds.is_empty()
        && reused_measurement_seeds.is_empty()
        && (!require_all || (missing_methods.is_empty() && missing_nested.is_empty()));
    Ok(json!({
        "schema": AUDIT_SCHEMA,
        "admissible": admissible,
        "require_all_development_coverage": require_all,
        "development": {
            "specs": development_rows,
            "method_count": development_methods.len(),
            "nested_algorithm_count": development_nested.len(),
            "methods": development_methods,
            "nested_algorithms": development_nested,
        },
        "holdout": {
            "specs": holdout_rows,
            "method_count": holdout_methods.len(),
            "nested_algorithm_count": holdout_nested.len(),
            "methods": holdout_methods,
            "nested_algorithms": holdout_nested,
        },
        "coverage": {
            "required_method_count": required_methods.len(),
            "required_nested_algorithm_count": required_nested.len(),
            "missing_development_methods": missing_methods,
            "missing_development_nested_algorithms": missing_nested,
        },
        "leakage": {
            "reused_target_seeds": reused_target_seeds,
            "reused_measurement_seeds": reused_measurement_seeds,
        },
        "interpretation": "admissible freezes campaign structure only; it is not a benchmark result or leaderboard promotion",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_live_method_and_parameter_has_an_optimization_policy() {
        let profiles = profiles().unwrap();
        assert_eq!(profiles.len(), methods::registry().len());
        assert_eq!(
            profiles.iter().map(|p| p.id).collect::<BTreeSet<_>>().len(),
            profiles.len()
        );
        for profile in &profiles {
            let knobs = profile
                .parameters
                .iter()
                .filter(|parameter| parameter.role == ParameterRole::Knob)
                .count();
            if matches!(
                profile.strategy,
                SearchStrategy::NativeAdaptive | SearchStrategy::NativeGrid
            ) {
                assert!(
                    knobs > 0,
                    "{} has a search strategy but no knob",
                    profile.id
                );
            }
        }
    }

    #[test]
    fn every_native_solver_and_nested_engine_is_in_the_inventory() {
        let nested = nested_algorithms();
        let keys: BTreeSet<_> = nested.iter().map(|a| a.key.as_str()).collect();
        for name in [
            "f4-f2",
            "buchberger-f2",
            "matrix-f4",
            "matrix-f5",
            "inherited-f4",
            "crossbred-f2",
            "xl-f2",
            "sat-cdcl",
            "fes-f2",
            "fes-f2-wide",
            "exhaustive",
        ] {
            assert!(keys.contains(format!("system_solver/{name}").as_str()));
        }
        for name in PDP3_ENGINES {
            assert!(keys.contains(format!("pdp3_engine/{name}").as_str()));
        }
        for name in PKM_TOWER_ENGINES {
            assert!(keys.contains(format!("pkm_tower_engine/{name}").as_str()));
        }
    }

    #[test]
    fn censoring_and_seeds_cannot_be_tuning_knobs() {
        for name in [
            "max_trials",
            "trial_cap",
            "solver_budget_seconds",
            "base_seed",
            "rank_seed",
        ] {
            assert_ne!(parameter_role(name), Some(ParameterRole::Knob));
        }
    }
}
