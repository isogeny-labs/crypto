//! The experiment spec and the plan it expands to.
//!
//! A spec names *what* is measured: curves, how many independent
//! single-target workloads per curve, the arms (methods), and how many
//! interleaved rounds.  *Where* it runs (CPUs, lock, thresholds) is a
//! property of the host and a flag of `ecbench run`, recorded in the
//! session, so the same spec runs unchanged on a laptop, a lab box and an
//! isolab worker.
//!
//! Expansion is deterministic: the same spec gives the same workloads,
//! the same execution order and the same algorithm seeds everywhere.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::cryptanalysis::ecbench::canonical::{derive_u64, short_id};
use crate::cryptanalysis::ecbench::methods::{
    applicable, decl, resolve, Applies, MethodSpec, ResolvedMethod,
};
use crate::cryptanalysis::ecbench::workload::{CurveSpec, Instance, TargetKind, Workload};

/// Policy-bearing specs written by current tooling.
pub const SPEC_SCHEMA: &str = "ecbench.spec/v2";
/// Frozen historical specs.  They remain parseable for replay and audit.
pub const LEGACY_SPEC_SCHEMA: &str = "ecbench.spec/v1";

fn default_phase_lane() -> String {
    super::panel::DEFAULT_PHASE_LANE.to_string()
}

fn is_default_phase_lane(lane: &str) -> bool {
    lane == super::panel::DEFAULT_PHASE_LANE
}

/// Why a spec is allowed to select its curves.
///
/// Phase measurements must select canonical panel cells.  Tiny and off-panel
/// fixtures remain useful, but are explicitly non-claim smoke or diagnostic
/// work instead of silently entering the scaling ladder.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum CurvePolicy {
    /// Present only on frozen v1 specs.
    #[default]
    Legacy,
    /// Correctness, plumbing, and unit-test fixtures.  Not claim eligible.
    Smoke,
    /// Off-panel research that records why the canonical ladder is inapplicable.
    Diagnostic { off_panel_reason: String },
    /// A scaling measurement on one or more cells in one independently
    /// promoted benchmark lane of the named phase. Omission preserves the
    /// pre-lane v2 identity and means `elliptic_algebraic`. A typed transfer
    /// lane also names its reviewed route.
    Phase {
        phase: u8,
        #[serde(
            default = "default_phase_lane",
            skip_serializing_if = "is_default_phase_lane"
        )]
        lane: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        route: Option<String>,
    },
}

impl CurvePolicy {
    pub fn is_legacy(&self) -> bool {
        matches!(self, Self::Legacy)
    }

    pub fn is_phase(&self) -> bool {
        matches!(self, Self::Phase { .. })
    }
}

/// What an arm is in the comparison.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// The method the boundary is measured against (normally rho).
    Reference,
    /// The unmodified method a candidate changes.
    Baseline,
    Candidate,
    /// A repeat of another arm: the noise floor (A/A).
    Control,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArmSpec {
    pub name: String,
    pub role: Role,
    pub method: MethodSpec,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkloadPlan {
    pub curves: Vec<CurveSpec>,
    /// Independent single-target workloads per curve.  Each is its own
    /// row; they are never pooled into a multi-target run.
    pub targets_per_curve: u64,
    pub target_seed: u64,
    /// `planted` (the default: `Q = [k]G`, checked against `k`),
    /// `planted_shared` (the same `k` on every curve of one order, for
    /// paired quotient statistics) or `public` (`Q` hashed to the
    /// subgroup, checked by `[d]G = Q`): what a `vs_rho` claim needs;
    /// `planted_hec_thread` reproduces the hyperelliptic thread's fixed
    /// target on a `hec_thread` curve.
    /// Not serialised when planted, so a spec written before the field
    /// keeps its id.
    #[serde(default, skip_serializing_if = "TargetKind::is_plain_planted")]
    pub target_kind: TargetKind,
}

/// Order of the arms inside a round.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Order {
    /// Spec order on even rounds, reversed on odd ones: a linear drift
    /// (thermal, a slowly filling page cache) cancels over a pair.  The
    /// parity counts the warm-up rounds, so the measured rounds start
    /// reversed after an odd number of warm-ups.  Kept exactly as it was,
    /// because it fixes the plan of every existing spec.
    Alternate,
    /// A seeded permutation per round.
    Shuffle,
    /// Like `alternate`, but the parity counts the measured rounds only:
    /// the first measured round is always in spec order, whatever the
    /// warm-up.  (Warm-up rounds alternate among themselves.)  With an
    /// even number of measured rounds each arm leads and trails equally
    /// often.
    AlternateMeasured,
}

pub use super::isolation::Level;

fn default_warmup() -> u32 {
    1
}
fn default_order() -> Order {
    Order::Alternate
}
fn default_level() -> Level {
    Level::L2
}
fn default_timeout() -> u64 {
    600
}

/// Index used in the measured algorithm-seed hash. Scheduling rounds remain
/// absolute; warm-ups always retain their separate domain and absolute index.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlgorithmSeedRound {
    /// Historical law: warm-ups advance the measured seed index.
    #[default]
    Absolute,
    /// Measured streams start at zero independently of the warm-up count.
    MeasuredIndex,
}

impl AlgorithmSeedRound {
    fn is_absolute(&self) -> bool {
        *self == Self::Absolute
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Measurement {
    /// Measured rounds: every arm once per workload per round.
    pub rounds: u32,
    /// Unmeasured rounds first, to take one-time costs off round one.
    #[serde(default = "default_warmup")]
    pub warmup: u32,
    #[serde(default = "default_order")]
    pub order: Order,
    /// Master seed of the algorithm seeds.  Each (workload, round) gets
    /// one seed shared by every arm, so arms are paired.
    pub seed: u64,
    /// Opt-in measured-index seed law. Omission preserves historical identities.
    #[serde(default, skip_serializing_if = "AlgorithmSeedRound::is_absolute")]
    pub algorithm_seed_round: AlgorithmSeedRound,
    /// Below this level a wall-clock figure is descriptive, never a result.
    #[serde(default = "default_level")]
    pub isolation_required: Level,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spec {
    pub schema: String,
    pub label: String,
    #[serde(default)]
    pub description: String,
    #[serde(default, skip_serializing_if = "CurvePolicy::is_legacy")]
    pub curve_policy: CurvePolicy,
    pub workloads: WorkloadPlan,
    pub arms: Vec<ArmSpec>,
    pub measurement: Measurement,
}

/// One arm with its method resolved.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arm {
    pub name: String,
    pub role: Role,
    pub method: ResolvedMethod,
}

/// One planned execution.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Execution {
    pub seq: u64,
    pub round: u32,
    pub warmup: bool,
    pub arm: usize,
    pub workload: usize,
    pub algorithm_seed: u64,
}

/// A spec expanded.
pub struct Plan {
    pub spec: Spec,
    /// `ECS1h` + 12 hex of the spec without its label and description.
    pub spec_id: String,
    pub spec_sha256: String,
    pub arms: Vec<Arm>,
    pub workloads: Vec<Workload>,
    /// Things a reader should know about the spec that do not make it
    /// invalid (e.g. an order that cannot be balanced).  Not hashed, not
    /// recorded in the plan.
    pub warnings: Vec<String>,
    /// One built curve per spec curve, in spec order.
    pub instances: Vec<Instance>,
    /// The index into `instances` of each workload's curve.
    pub curve_of: Vec<usize>,
    pub executions: Vec<Execution>,
}

impl Plan {
    /// The built curve workload `w` runs on.
    pub fn instance(&self, w: usize) -> &Instance {
        &self.instances[self.curve_of[w]]
    }
}

impl Spec {
    pub fn from_json(text: &str) -> Result<Self, String> {
        let spec: Spec = serde_json::from_str(text).map_err(|e| format!("spec: {e}"))?;
        match spec.schema.as_str() {
            LEGACY_SPEC_SCHEMA if spec.curve_policy.is_legacy() => {}
            LEGACY_SPEC_SCHEMA => {
                return Err("ecbench.spec/v1 cannot carry curve_policy; use ecbench.spec/v2".into())
            }
            SPEC_SCHEMA if spec.curve_policy.is_legacy() => {
                return Err(
                    "ecbench.spec/v2 requires curve_policy: smoke, diagnostic, or phase".into(),
                )
            }
            SPEC_SCHEMA => {}
            other => {
                return Err(format!(
                    "spec schema is `{other}`, expected `{LEGACY_SPEC_SCHEMA}` or `{SPEC_SCHEMA}`"
                ))
            }
        }
        Ok(spec)
    }

    pub(crate) fn identity_view(&self) -> Value {
        let mut view = json!({
            "schema": self.schema,
            "workloads": serde_json::to_value(&self.workloads).unwrap_or(Value::Null),
            "arms": serde_json::to_value(&self.arms).unwrap_or(Value::Null),
            "measurement": serde_json::to_value(&self.measurement).unwrap_or(Value::Null),
        });
        if !self.curve_policy.is_legacy() {
            view["curve_policy"] = serde_json::to_value(&self.curve_policy).unwrap_or(Value::Null);
        }
        view
    }

    /// Current CLI runs require an explicit curve classification.  Audit and
    /// replay still parse v1 specs byte-for-byte.
    pub fn require_current_curve_policy(&self) -> Result<(), String> {
        if self.curve_policy.is_legacy() {
            Err("ecbench.spec/v1 is historical: convert the spec to v2 with an explicit curve_policy, or pass --allow-legacy-spec only for reproduction".into())
        } else {
            Ok(())
        }
    }
}

fn validate_curve_policy(spec: &Spec) -> Result<(), String> {
    match &spec.curve_policy {
        CurvePolicy::Legacy | CurvePolicy::Smoke => Ok(()),
        CurvePolicy::Diagnostic { off_panel_reason } => {
            if off_panel_reason.trim().len() < 12 {
                Err("diagnostic curve_policy needs a substantive off_panel_reason".into())
            } else {
                Ok(())
            }
        }
        CurvePolicy::Phase { phase, lane, route } => {
            let catalog = super::panel::catalog()?;
            let phase_record = catalog
                .phase(*phase)
                .ok_or_else(|| format!("canonical panel has no phase {phase}"))?;
            let lane_definition = catalog
                .lane(lane)
                .ok_or_else(|| format!("canonical panel has no benchmark lane `{lane}`"))?;
            let lane_record = phase_record
                .lanes
                .iter()
                .find(|record| record.lane_id == *lane)
                .ok_or_else(|| format!("canonical phase {phase} has no benchmark lane `{lane}`"))?;
            if lane_record.admission_state != "ready_to_preregister" {
                return Err(format!(
                    "canonical phase {phase} lane `{lane}` is {} and cannot start until its admission blockers are cleared in the panel manifest: {}",
                    lane_record.admission_state,
                    lane_record.blockers.join("; ")
                ));
            }
            if spec.workloads.target_kind != TargetKind::Public {
                return Err(format!(
                    "phase {phase} lane `{lane}` requires public targets, not {:?}",
                    spec.workloads.target_kind
                ));
            }
            if spec.workloads.targets_per_curve < phase_record.minimum_public_targets_per_curve {
                return Err(format!(
                    "phase {phase} lane `{lane}` requires at least {} public targets per curve, not {}",
                    phase_record.minimum_public_targets_per_curve, spec.workloads.targets_per_curve
                ));
            }
            let mut selected = std::collections::BTreeSet::new();
            match lane_definition.curve_selector.as_str() {
                "source_panel_cells" => {
                    if route.is_some() {
                        return Err(format!(
                            "phase {phase} lane `{lane}` does not accept a transfer route"
                        ));
                    }
                    for curve in &spec.workloads.curves {
                        let CurveSpec::Panel { cell } = curve else {
                            return Err(format!(
                                "phase {phase} lane `{lane}` curves must use {{\"kind\":\"panel\",\"cell\":...}}; copied or generated curve parameters are off-panel diagnostics"
                            ));
                        };
                        if !selected.insert(cell.as_str()) {
                            return Err(format!(
                                "phase {phase} panel cell `{cell}` is listed twice"
                            ));
                        }
                        let (actual_phase, record) = catalog
                            .cell(cell)
                            .ok_or_else(|| format!("`{cell}` is not a canonical panel cell"))?;
                        if actual_phase.phase != *phase {
                            return Err(format!(
                                "panel cell `{cell}` belongs to phase {}, not phase {phase}",
                                actual_phase.phase
                            ));
                        }
                        if !record.runnable() {
                            return Err(format!(
                                "panel cell `{cell}` is {} and cannot enter a run",
                                record.ecbench.status
                            ));
                        }
                    }
                }
                "registered_destination_cells" => {
                    if route.is_some() {
                        return Err(format!(
                            "phase {phase} lane `{lane}` does not accept a transfer route"
                        ));
                    }
                    for curve in &spec.workloads.curves {
                        let CurveSpec::PanelDestination { cell } = curve else {
                            return Err(format!(
                                "phase {phase} lane `{lane}` curves must use {{\"kind\":\"panel_destination\",\"cell\":...}}"
                            ));
                        };
                        if !selected.insert(cell.as_str()) {
                            return Err(format!(
                                "phase {phase} destination cell `{cell}` is listed twice"
                            ));
                        }
                        let (actual_phase, record) =
                            catalog.destination(cell).ok_or_else(|| {
                                format!("`{cell}` is not a canonical destination cell")
                            })?;
                        if actual_phase.phase != *phase {
                            return Err(format!(
                                "destination cell `{cell}` belongs to phase {}, not phase {phase}",
                                actual_phase.phase
                            ));
                        }
                        if !record.runnable() {
                            return Err(format!(
                                "destination cell `{cell}` is {} and cannot enter a run",
                                record.ecbench.status
                            ));
                        }
                        let has_arm = |method: &str, role: Role| {
                            spec.arms
                                .iter()
                                .any(|arm| arm.method.id == method && arm.role == role)
                        };
                        if !has_arm(&record.ecbench.required_reference, Role::Reference)
                            || !has_arm(&record.ecbench.required_reference, Role::Control)
                        {
                            return Err(format!(
                                "destination cell `{cell}` requires reference and A/A control arms using `{}`",
                                record.ecbench.required_reference
                            ));
                        }
                        for method in &record.ecbench.required_candidates {
                            if !has_arm(method, Role::Candidate) {
                                return Err(format!(
                                    "destination cell `{cell}` requires candidate arm `{method}`"
                                ));
                            }
                        }
                    }
                }
                "registered_transfer_routes" => {
                    let route_id = route.as_deref().ok_or_else(|| {
                        format!("phase {phase} lane `{lane}` requires a typed transfer route")
                    })?;
                    let transfer = catalog
                        .transfer_route(route_id)
                        .ok_or_else(|| format!("`{route_id}` is not a canonical transfer route"))?;
                    if transfer.phase != *phase {
                        return Err(format!(
                            "transfer route `{route_id}` belongs to phase {}, not phase {phase}",
                            transfer.phase
                        ));
                    }
                    if spec.workloads.curves.len() != 1
                        || !matches!(
                            &spec.workloads.curves[0],
                            CurveSpec::Panel { cell } if cell == &transfer.source_cell
                        )
                    {
                        return Err(format!(
                            "transfer route `{route_id}` requires the exact source panel cell `{}`",
                            transfer.source_cell
                        ));
                    }
                    let has_arm = |method: &str, role: Role| {
                        spec.arms
                            .iter()
                            .any(|arm| arm.method.id == method && arm.role == role)
                    };
                    if !has_arm(&transfer.source_reference, Role::Reference)
                        || !has_arm(&transfer.source_reference, Role::Control)
                    {
                        return Err(format!(
                            "transfer route `{route_id}` requires source reference and A/A control arms using `{}`",
                            transfer.source_reference
                        ));
                    }
                    for (index, method) in transfer.destination_methods.iter().enumerate() {
                        let role = if index == 0 {
                            Role::Baseline
                        } else {
                            Role::Candidate
                        };
                        if !has_arm(method, role) {
                            return Err(format!(
                                "transfer route `{route_id}` requires destination {role:?} arm `{method}`"
                            ));
                        }
                    }
                }
                other => {
                    return Err(format!(
                        "canonical phase {phase} lane `{lane}` has unsupported selector `{other}`"
                    ))
                }
            }
            Ok(())
        }
    }
}

/// Why a spec's arm order cannot be balanced over its measured rounds, if
/// it cannot.  A warning, never a refusal: existing specs (and the plans
/// and hashes derived from them) must keep expanding as they did.
fn order_warnings(m: &Measurement) -> Vec<String> {
    let mut out = Vec::new();
    if m.rounds % 2 == 1 {
        match m.order {
            Order::Alternate => {
                let first = if m.warmup % 2 == 1 {
                    "reversed"
                } else {
                    "in spec order"
                };
                out.push(format!(
                    "order `alternate` with {} measured round(s) cannot be balanced: one arm order is used once more than the other, and the parity counts the {} warm-up round(s), so the measured rounds start {first}; use an even number of rounds, or order `alternate_measured` (which starts the measured rounds in spec order)",
                    m.rounds, m.warmup
                ));
            }
            Order::AlternateMeasured => out.push(format!(
                "order `alternate_measured` with {} measured round(s) cannot be balanced: spec order is used once more than the reverse; use an even number of rounds",
                m.rounds
            )),
            Order::Shuffle => {}
        }
    }
    out
}

/// Validate and expand.  Every refusal happens here, before anything runs.
pub fn plan(spec: Spec) -> Result<Plan, String> {
    validate_curve_policy(&spec)?;
    if spec.arms.is_empty() {
        return Err("a spec needs at least one arm".into());
    }
    if spec.measurement.rounds == 0 {
        return Err("rounds must be at least 1".into());
    }
    if spec.workloads.curves.is_empty() || spec.workloads.targets_per_curve == 0 {
        return Err("a spec needs at least one curve and one target per curve".into());
    }
    let mut names = std::collections::BTreeSet::new();
    let mut arms = Vec::new();
    for a in &spec.arms {
        if !names.insert(a.name.clone()) {
            return Err(format!("arm name `{}` is used twice", a.name));
        }
        if a.name.is_empty()
            || !a
                .name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c))
        {
            return Err(format!("arm name `{}` must be [A-Za-z0-9._-]+", a.name));
        }
        arms.push(Arm {
            name: a.name.clone(),
            role: a.role,
            method: resolve(&a.method)?,
        });
    }
    let mut workloads = Vec::new();
    let mut instances = Vec::new();
    let mut curve_of = Vec::new();
    for c in &spec.workloads.curves {
        // One build per curve: a prime search counts points in O(p) per
        // candidate, so building per target would multiply that.
        let inst = c.build()?;
        for i in 0..spec.workloads.targets_per_curve {
            let w = Workload::on(
                c,
                &inst,
                spec.workloads.target_seed,
                i,
                spec.workloads.target_kind,
            )?;
            for a in &arms {
                let d = decl(&a.method.id).expect("resolved");
                if d.applies == Applies::KoblitzOnly && w.curve.family != "koblitz" {
                    return Err(format!(
                        "arm `{}` ({}) runs on Koblitz curves only, and {} is {}; put it in a spec of its own",
                        a.name, a.method.id, w.curve.slug, w.curve.family
                    ));
                }
                if d.applies == Applies::BinaryOnly && !matches!(inst, Instance::Binary(_)) {
                    return Err(format!(
                        "arm `{}` ({}) runs on binary curves only, and {} is {}; put it in a spec of its own",
                        a.name, a.method.id, w.curve.slug, w.curve.family
                    ));
                }
                if i == 0 {
                    // Once per curve: the refusal does not depend on the target.
                    applicable(&a.method, &inst, &w.curve).map_err(|e| {
                        format!(
                            "arm `{}` ({}) cannot run on {}: {e}",
                            a.name, a.method.id, w.curve.slug
                        )
                    })?;
                }
            }
            if workloads
                .iter()
                .any(|x: &Workload| x.workload_id == w.workload_id)
            {
                return Err(format!(
                    "workload {} ({}, target {}) appears twice: a curve is listed more than once with the same target seed",
                    w.workload_id, w.curve.slug, w.target_index
                ));
            }
            workloads.push(w);
            curve_of.push(instances.len());
        }
        instances.push(inst);
    }
    let m = &spec.measurement;
    let mut executions = Vec::new();
    let mut seq = 0u64;
    for round in 0..(m.warmup + m.rounds) {
        let warmup = round < m.warmup;
        for (wi, w) in workloads.iter().enumerate() {
            // One seed per (workload, round), shared by every arm.
            let wid = u64::from_str_radix(&w.workload_id[1..], 16).unwrap_or(0);
            let seed_round =
                if !warmup && m.algorithm_seed_round == AlgorithmSeedRound::MeasuredIndex {
                    round - m.warmup
                } else {
                    round
                };
            let algorithm_seed = derive_u64(
                if warmup {
                    "ecbench.warmup_seed"
                } else {
                    "ecbench.algorithm_seed"
                },
                &[m.seed, wid, seed_round as u64],
            );
            let mut order: Vec<usize> = (0..arms.len()).collect();
            match m.order {
                Order::Alternate => {
                    if round % 2 == 1 {
                        order.reverse();
                    }
                }
                Order::AlternateMeasured => {
                    let k = if warmup { round } else { round - m.warmup };
                    if k % 2 == 1 {
                        order.reverse();
                    }
                }
                Order::Shuffle => {
                    let mut s = derive_u64("ecbench.order", &[m.seed, wid, round as u64]);
                    for i in (1..order.len()).rev() {
                        s = crate::cryptanalysis::ecbench::canonical::splitmix64(s);
                        order.swap(i, (s % (i as u64 + 1)) as usize);
                    }
                }
            }
            for arm in order {
                executions.push(Execution {
                    seq,
                    round,
                    warmup,
                    arm,
                    workload: wi,
                    algorithm_seed,
                });
                seq += 1;
            }
        }
    }
    let (spec_id, spec_sha256) = short_id("ECS1", &spec.identity_view())?;
    let warnings = order_warnings(&spec.measurement);
    Ok(Plan {
        spec,
        spec_id,
        spec_sha256,
        arms,
        workloads,
        warnings,
        instances,
        curve_of,
        executions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(arms: &[(&str, &str)]) -> Spec {
        Spec::from_json(
            &json!({
                "schema": SPEC_SCHEMA,
                "label": "t",
                "curve_policy": {"mode": "diagnostic", "off_panel_reason": "unit test uses a generated prime fixture"},
                "workloads": {"curves": [{"kind": "prime_search", "bits": 14, "seed": 59297}],
                              "targets_per_curve": 2, "target_seed": 1},
                "arms": arms.iter().map(|(n, m)| json!({"name": n, "role": "candidate", "method": {"id": m}})).collect::<Vec<_>>(),
                "measurement": {"rounds": 3, "seed": 9},
            })
            .to_string(),
        )
        .unwrap()
    }

    /// An arm whose method cannot run on the spec's curve is a spec error
    /// at plan time, not a session of `error` records.
    #[test]
    fn plan_rejects_an_arm_its_curve_cannot_run() {
        let err = plan(spec(&[("q", "rho.quartic")])).err().expect("refused");
        assert!(
            err.contains("cannot run on") && err.contains("rho.quartic"),
            "{err}"
        );
    }

    #[test]
    fn plan_pairs_seeds_and_alternates() {
        let p = plan(spec(&[("a", "rho.negation"), ("b", "bsgs.textbook")])).unwrap();
        // (1 warm-up + 3 rounds) × 2 workloads × 2 arms.
        assert_eq!(p.executions.len(), 16);
        let r1: Vec<_> = p
            .executions
            .iter()
            .filter(|e| e.round == 1 && e.workload == 0)
            .collect();
        assert_eq!(r1[0].algorithm_seed, r1[1].algorithm_seed);
        assert_eq!((r1[0].arm, r1[1].arm), (1, 0));
        assert!(p.spec_id.starts_with("ECS1h"));
    }

    #[test]
    fn default_seed_round_preserves_legacy_serialization_and_expansion() {
        let s = spec(&[("a", "rho.negation"), ("b", "bsgs.textbook")]);
        let legacy_measurement = json!({
            "rounds": 3, "warmup": 1, "order": "alternate", "seed": 9,
            "isolation_required": "L2", "timeout_seconds": 600
        });
        assert_eq!(
            serde_json::to_value(&s.measurement).unwrap(),
            legacy_measurement
        );
        let mut explicit = serde_json::to_value(&s).unwrap();
        explicit["measurement"]["algorithm_seed_round"] = json!("absolute");
        let p = plan(s).unwrap();
        let explicit = plan(Spec::from_json(&explicit.to_string()).unwrap()).unwrap();
        assert_eq!(p.spec_id, explicit.spec_id);
        assert_eq!(p.spec_sha256, explicit.spec_sha256);
        assert_eq!(
            serde_json::to_value(&p.executions).unwrap(),
            serde_json::to_value(&explicit.executions).unwrap()
        );
        for e in &p.executions {
            let wid = u64::from_str_radix(&p.workloads[e.workload].workload_id[1..], 16).unwrap();
            let domain = if e.warmup {
                "ecbench.warmup_seed"
            } else {
                "ecbench.algorithm_seed"
            };
            assert_eq!(
                e.algorithm_seed,
                derive_u64(domain, &[9, wid, u64::from(e.round)])
            );
        }
    }

    #[test]
    fn measured_seed_round_preserves_streams_across_warmup_counts() {
        let mut s = spec(&[("a", "rho.negation"), ("b", "bsgs.textbook")]);
        s.measurement.rounds = 5;
        s.measurement.warmup = 0;
        let legacy = plan(s.clone()).unwrap();
        s.measurement.algorithm_seed_round = AlgorithmSeedRound::MeasuredIndex;
        assert_eq!(
            serde_json::to_value(&s.measurement).unwrap()["algorithm_seed_round"],
            "measured_index"
        );
        assert_eq!(
            Spec::from_json(&serde_json::to_string(&s).unwrap()).unwrap(),
            s
        );
        for warmup in [0, 1, 3] {
            s.measurement.warmup = warmup;
            let p = plan(s.clone()).unwrap();
            assert_ne!(p.spec_id, legacy.spec_id);
            assert_eq!(
                serde_json::to_value(&p.workloads).unwrap(),
                serde_json::to_value(&legacy.workloads).unwrap()
            );
            for wi in 0..p.workloads.len() {
                let wid = u64::from_str_radix(&p.workloads[wi].workload_id[1..], 16).unwrap();
                for index in 0..5 {
                    let paired: Vec<_> = p
                        .executions
                        .iter()
                        .filter(|e| e.workload == wi && !e.warmup && e.round == warmup + index)
                        .collect();
                    assert_eq!(paired.len(), 2);
                    assert_ne!(paired[0].arm, paired[1].arm);
                    let expected =
                        derive_u64("ecbench.algorithm_seed", &[9, wid, u64::from(index)]);
                    for e in paired {
                        assert_eq!(e.algorithm_seed, expected);
                        let old = legacy
                            .executions
                            .iter()
                            .find(|old| {
                                old.workload == wi && old.round == index && old.arm == e.arm
                            })
                            .unwrap();
                        assert_eq!(e.algorithm_seed, old.algorithm_seed);
                    }
                }
                for e in p.executions.iter().filter(|e| e.workload == wi && e.warmup) {
                    assert!(e.round < warmup);
                    assert_eq!(
                        e.algorithm_seed,
                        derive_u64("ecbench.warmup_seed", &[9, wid, u64::from(e.round)])
                    );
                }
            }
        }
    }

    fn first_arm_per_round(p: &Plan, workload: usize) -> Vec<(u32, bool, usize)> {
        let mut seen = std::collections::BTreeSet::new();
        p.executions
            .iter()
            .filter(|e| e.workload == workload)
            .filter(|e| seen.insert(e.round))
            .map(|e| (e.round, e.warmup, e.arm))
            .collect()
    }

    #[test]
    fn alternate_measured_counts_measured_rounds_only() {
        let mut s = spec(&[("a", "rho.negation"), ("b", "bsgs.textbook")]);
        s.measurement.warmup = 1;
        s.measurement.rounds = 4;
        let alt = plan(s.clone()).unwrap();
        // `alternate` parity counts the warm-up: measured rounds 1..=4 run
        // reversed, forward, reversed, forward.
        let a: Vec<usize> = first_arm_per_round(&alt, 0)
            .into_iter()
            .filter(|r| !r.1)
            .map(|r| r.2)
            .collect();
        assert_eq!(a, vec![1, 0, 1, 0]);
        s.measurement.order = Order::AlternateMeasured;
        let alt_m = plan(s.clone()).unwrap();
        // The first measured round is in spec order whatever the warm-up.
        let m: Vec<usize> = first_arm_per_round(&alt_m, 0)
            .into_iter()
            .filter(|r| !r.1)
            .map(|r| r.2)
            .collect();
        assert_eq!(m, vec![0, 1, 0, 1]);
        // The same warm-up count, any parity: the measured pattern is fixed.
        for warmup in [0u32, 1, 2, 3] {
            s.measurement.warmup = warmup;
            let p = plan(s.clone()).unwrap();
            let m: Vec<usize> = first_arm_per_round(&p, 0)
                .into_iter()
                .filter(|r| !r.1)
                .map(|r| r.2)
                .collect();
            assert_eq!(m, vec![0, 1, 0, 1], "warmup {warmup}");
        }
        // Seeds and workloads do not depend on the order mode.
        let seeds = |p: &Plan| -> Vec<(u32, u64)> {
            let mut v: Vec<_> = p
                .executions
                .iter()
                .map(|e| (e.round, e.algorithm_seed))
                .collect();
            v.sort();
            v.dedup();
            v
        };
        s.measurement.warmup = 1;
        s.measurement.order = Order::Alternate;
        let a = plan(s.clone()).unwrap();
        s.measurement.order = Order::AlternateMeasured;
        let b = plan(s).unwrap();
        assert_eq!(seeds(&a), seeds(&b));
        assert_ne!(a.spec_id, b.spec_id);
    }

    #[test]
    fn existing_alternate_expansion_is_unchanged_and_odd_rounds_only_warn() {
        // The default spec: 1 warm-up, 3 measured rounds, `alternate`.
        let p = plan(spec(&[("a", "rho.negation"), ("b", "bsgs.textbook")])).unwrap();
        let firsts: Vec<usize> = first_arm_per_round(&p, 0)
            .into_iter()
            .map(|r| r.2)
            .collect();
        // warm-up (round 0) forward, then reversed, forward, reversed.
        assert_eq!(firsts, vec![0, 1, 0, 1]);
        assert_eq!(p.warnings.len(), 1, "{:?}", p.warnings);
        assert!(p.warnings[0].contains("alternate_measured"));
        // Even rounds: no warning.
        let mut s = spec(&[("a", "rho.negation"), ("b", "bsgs.textbook")]);
        s.measurement.rounds = 4;
        assert!(plan(s).unwrap().warnings.is_empty());
        // The enum names are part of the spec language.
        assert_eq!(
            serde_json::to_string(&Order::Alternate).unwrap(),
            "\"alternate\""
        );
        assert_eq!(
            serde_json::to_string(&Order::AlternateMeasured).unwrap(),
            "\"alternate_measured\""
        );
    }

    /// A session recorded before `alternate_measured` expands today exactly
    /// as it was planned then: same executions, same seeds, same order.
    #[test]
    fn a_committed_sessions_plan_still_expands_identically() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("research/ecbench_numa_vm_20261002/sessions/v2-auto");
        let Ok(spec_text) = std::fs::read_to_string(dir.join("spec.json")) else {
            return; // a sparse checkout without the evidence
        };
        let recorded: Value =
            serde_json::from_str(&std::fs::read_to_string(dir.join("plan.json")).unwrap()).unwrap();
        let p = plan(Spec::from_json(&spec_text).unwrap()).unwrap();
        assert_eq!(recorded["spec_id"], json!(p.spec_id));
        assert_eq!(
            recorded["executions"],
            serde_json::to_value(&p.executions).unwrap()
        );
        assert_eq!(recorded["arms"], serde_json::to_value(&p.arms).unwrap());
        assert_eq!(
            recorded["workloads"],
            serde_json::to_value(&p.workloads).unwrap()
        );
        // It is `alternate` with 3 measured rounds: warned, never refused.
        assert_eq!(p.spec.measurement.order, Order::Alternate);
        assert_eq!(p.warnings.len(), 1);
    }

    #[test]
    fn label_does_not_change_the_spec_id() {
        let a = plan(spec(&[("a", "rho.plain")])).unwrap();
        let mut s = spec(&[("a", "rho.plain")]);
        s.label = "other".into();
        assert_eq!(plan(s).unwrap().spec_id, a.spec_id);
    }

    #[test]
    fn a_curve_listed_twice_is_refused() {
        let mut s = spec(&[("a", "rho.plain")]);
        let c = s.workloads.curves[0].clone();
        s.workloads.curves.push(c);
        let err = plan(s).err().unwrap();
        assert!(err.contains("appears twice"), "{err}");
    }

    /// `rho.multitarget_pooled` is prime-only: a Koblitz or random binary
    /// workload is a plan-time spec error, a prime workload plans.
    #[test]
    fn prime_only_arm_on_a_binary_curve_is_refused_at_plan_time() {
        let with_curve = |curve: Value| {
            Spec::from_json(
                &json!({
                    "schema": SPEC_SCHEMA,
                    "label": "t",
                    "curve_policy": {"mode": "diagnostic", "off_panel_reason": "unit test exercises method applicability"},
                    "workloads": {"curves": [curve], "targets_per_curve": 1, "target_seed": 1},
                    "arms": [{"name": "pooled", "role": "candidate",
                              "method": {"id": "rho.multitarget_pooled",
                                         "params": {"batch": "4"}}}],
                    "measurement": {"rounds": 3, "seed": 9},
                })
                .to_string(),
            )
            .unwrap()
        };
        for curve in [
            json!({"kind": "koblitz", "a": 1, "n": 17}),
            json!({"kind": "binary_random", "n": 17, "seed": 3, "max_cofactor": 4}),
        ] {
            let err = plan(with_curve(curve)).err().expect("refused at plan time");
            assert!(
                err.contains("rho.multitarget_pooled") && err.contains("prime-field curves only"),
                "{err}"
            );
        }
        plan(with_curve(
            json!({"kind": "prime_search", "bits": 14, "seed": 59297}),
        ))
        .expect("the prime curve plans");
    }

    #[test]
    fn koblitz_only_arm_on_a_prime_curve_is_refused() {
        assert!(plan(spec(&[("f", "rho.signed_frobenius")])).is_err());
        assert!(plan(spec(&[("a", "rho.plain"), ("a", "rho.plain")])).is_err());
    }

    #[test]
    fn v2_requires_an_explicit_curve_policy_while_v1_remains_replayable() {
        let current = json!({
            "schema": SPEC_SCHEMA,
            "label": "missing policy",
            "workloads": {"curves": [], "targets_per_curve": 0, "target_seed": 0},
            "arms": [],
            "measurement": {"rounds": 0, "seed": 0}
        });
        assert!(Spec::from_json(&current.to_string())
            .unwrap_err()
            .contains("requires curve_policy"));
        let mut legacy = current;
        legacy["schema"] = json!(LEGACY_SPEC_SCHEMA);
        assert!(Spec::from_json(&legacy.to_string())
            .unwrap()
            .curve_policy
            .is_legacy());
    }

    #[test]
    fn phase_policy_uses_panel_cells_and_enforces_public_target_count() {
        let phase = |curve: Value, targets: u64, target_kind: &str| {
            Spec::from_json(
                &json!({
                    "schema": SPEC_SCHEMA,
                    "label": "phase policy fixture",
                    "curve_policy": {"mode": "phase", "phase": 1},
                    "workloads": {"curves": [curve], "targets_per_curve": targets,
                                  "target_seed": 1, "target_kind": target_kind},
                    "arms": [{"name": "rho", "role": "reference", "method": {"id": "rho.plain"}}],
                    "measurement": {"rounds": 1, "warmup": 0, "seed": 9}
                })
                .to_string(),
            )
            .unwrap()
        };
        plan(phase(
            json!({"kind": "panel", "cell": "P1-K0-23"}),
            32,
            "public",
        ))
        .unwrap();
        let copied = json!({"kind": "prime_explicit", "p": 5, "a": 2, "b": 1,
                            "group_order": 7, "r": 7, "gx": 0, "gy": 1});
        assert!(plan(phase(copied, 32, "public"))
            .err()
            .unwrap()
            .contains("must use"));
        assert!(plan(phase(
            json!({"kind": "panel", "cell": "P1-K0-23"}),
            31,
            "public",
        ))
        .err()
        .unwrap()
        .contains("at least 32"));
        assert!(plan(phase(
            json!({"kind": "panel", "cell": "P1-K0-23"}),
            32,
            "planted",
        ))
        .err()
        .unwrap()
        .contains("requires public targets"));

        let blocked = Spec::from_json(
            &json!({
                "schema": SPEC_SCHEMA,
                "label": "blocked phase fixture",
                "curve_policy": {"mode": "phase", "phase": 4},
                "workloads": {"curves": [{"kind": "panel", "cell": "P4-KOBLITZ-83"}],
                    "targets_per_curve": 8, "target_seed": 1, "target_kind": "public"},
                "arms": [{"name": "rho", "role": "reference",
                    "method": {"id": "rho.signed_frobenius_strong"}}],
                "measurement": {"rounds": 1, "warmup": 0, "seed": 9}
            })
            .to_string(),
        )
        .unwrap();
        assert!(plan(blocked)
            .err()
            .unwrap()
            .contains("phase 4 lane `elliptic_algebraic` is blocked"));

        let phase_hec = Spec::from_json(
            &json!({
                "schema": SPEC_SCHEMA,
                "label": "phase-matched hyperelliptic lane fixture",
                "curve_policy": {"mode": "phase", "phase": 1,
                                 "lane": "hyperelliptic_jacobian"},
                "workloads": {"curves": [{"kind": "panel_destination", "cell": "P1-JG2-18"}],
                    "targets_per_curve": 32, "target_seed": 1, "target_kind": "public"},
                "arms": [
                    {"name": "rho", "role": "reference", "method": {"id": "rho.negation"}},
                    {"name": "ic", "role": "candidate", "method": {"id": "ic.hyperelliptic_v2"}},
                    {"name": "rho-aa", "role": "control", "method": {"id": "rho.negation"}}
                ],
                "measurement": {"rounds": 1, "warmup": 0, "seed": 9}
            })
            .to_string(),
        )
        .unwrap();
        plan(phase_hec).unwrap();

        let wrong_hec_source = Spec::from_json(
            &json!({
                "schema": SPEC_SCHEMA,
                "label": "wrong hyperelliptic source fixture",
                "curve_policy": {"mode": "phase", "phase": 1,
                                 "lane": "hyperelliptic_jacobian"},
                "workloads": {"curves": [{"kind": "panel", "cell": "P1-K0-23"}],
                    "targets_per_curve": 32, "target_seed": 1, "target_kind": "public"},
                "arms": [{"name": "rho", "role": "reference", "method": {"id": "rho.negation"}}],
                "measurement": {"rounds": 1, "warmup": 0, "seed": 9}
            })
            .to_string(),
        )
        .unwrap();
        assert!(plan(wrong_hec_source)
            .err()
            .unwrap()
            .contains("panel_destination"));

        let phase_transfer = Spec::from_json(
            &json!({
                "schema": SPEC_SCHEMA,
                "label": "phase-matched cover-transfer fixture",
                "curve_policy": {"mode": "phase", "phase": 1,
                                 "lane": "cover_transfer", "route": "P1-K0-23-G3-COVER"},
                "workloads": {"curves": [{"kind": "panel", "cell": "P1-K0-23"}],
                    "targets_per_curve": 32, "target_seed": 1, "target_kind": "public"},
                "arms": [
                    {"name": "source-rho", "role": "reference",
                     "method": {"id": "rho.signed_frobenius_strong"}},
                    {"name": "destination-rho", "role": "baseline",
                     "method": {"id": "jacobian_image.rho3_v2"}},
                    {"name": "destination-bsgs", "role": "candidate",
                     "method": {"id": "jacobian_image.bsgs_v2"}},
                    {"name": "source-rho-aa", "role": "control",
                     "method": {"id": "rho.signed_frobenius_strong"}}
                ],
                "measurement": {"rounds": 1, "warmup": 0, "seed": 9}
            })
            .to_string(),
        )
        .unwrap();
        plan(phase_transfer).unwrap();

        let missing_route = Spec::from_json(
            &json!({
                "schema": SPEC_SCHEMA,
                "label": "missing cover-transfer route fixture",
                "curve_policy": {"mode": "phase", "phase": 1, "lane": "cover_transfer"},
                "workloads": {"curves": [{"kind": "panel", "cell": "P1-K0-23"}],
                    "targets_per_curve": 32, "target_seed": 1, "target_kind": "public"},
                "arms": [{"name": "source-rho", "role": "reference",
                    "method": {"id": "rho.signed_frobenius_strong"}}],
                "measurement": {"rounds": 1, "warmup": 0, "seed": 9}
            })
            .to_string(),
        )
        .unwrap();
        assert!(plan(missing_route)
            .err()
            .unwrap()
            .contains("requires a typed transfer route"));
    }

    #[test]
    fn field_five_fixture_is_smoke_only_not_a_phase_measurement() {
        let smoke = Spec::from_json(
            &json!({
                "schema": SPEC_SCHEMA,
                "label": "field five smoke fixture",
                "curve_policy": {"mode": "smoke"},
                "workloads": {"curves": [{"kind": "prime_explicit", "p": 5, "a": 2, "b": 1,
                    "group_order": 7, "r": 7, "gx": 0, "gy": 1}],
                    "targets_per_curve": 1, "target_seed": 1},
                "arms": [{"name": "bsgs", "role": "candidate", "method": {"id": "bsgs.textbook"}}],
                "measurement": {"rounds": 1, "warmup": 0, "seed": 9}
            })
            .to_string(),
        )
        .unwrap();
        plan(smoke).unwrap();
    }
}
