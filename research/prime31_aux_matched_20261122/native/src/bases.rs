//! Whole auxiliary fibers, their target lifts, and independent exact certificates.
//!
//! The complete fiber is never truncated to match a requested cardinality. Each
//! accepted fiber is paired with a plain control having its actual target-column
//! count. Production construction counters remain explicitly partial: the outer
//! process timer must include construction, certification, and failed attempts.

use crate::oracle::TowerDescription;
use crate::prime_tower::{self, AuxKind, IsogenyTower, IsogenyTowerCost};
use crate::{CountedGroup, FactorBase, GroupOps, PrimeCurve, PrimePoint};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const MAX_CERTIFIED_DEPTH: u32 = 10;

/// Existing ordinary prime31 baseline, pinned in the parent study's sources.
pub fn prime31_fixture() -> (PrimeCurve, u64, PrimePoint) {
    (
        PrimeCurve {
            p: 2_147_483_647,
            a: 847_514_846,
            b: 2_087_444_724,
        },
        2_147_446_831,
        PrimePoint::affine(625_037_715, 2_100_627_251),
    )
}

pub fn verify_target_fixture(
    curve: &PrimeCurve,
    r: u64,
    generator: PrimePoint,
) -> Result<(), String> {
    validate_target(curve, r)?;
    let e = CurveDefinition::from(curve);
    let g = PointDefinition::from(generator);
    if g.infinity || !independent_on_curve(&e, g) || !independent_scalar(&e, g, r).infinity {
        return Err("target generator fails the independent prime-order certificate".into());
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaseFamily {
    Ordinary,
    SsJ1728,
    Dickson,
    Plain,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurveDefinition {
    pub p: u64,
    pub a: u64,
    pub b: u64,
}

impl From<&PrimeCurve> for CurveDefinition {
    fn from(e: &PrimeCurve) -> Self {
        Self {
            p: e.p,
            a: e.a,
            b: e.b,
        }
    }
}

impl CurveDefinition {
    pub fn production(&self) -> PrimeCurve {
        PrimeCurve {
            p: self.p,
            a: self.a,
            b: self.b,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointDefinition {
    pub x: u64,
    pub y: u64,
    pub infinity: bool,
}

impl From<PrimePoint> for PointDefinition {
    fn from(p: PrimePoint) -> Self {
        Self {
            x: p.x,
            y: p.y,
            infinity: p.infinity,
        }
    }
}

impl PointDefinition {
    pub fn production(self) -> PrimePoint {
        PrimePoint {
            x: self.x,
            y: self.y,
            infinity: self.infinity,
        }
    }
}

/// Unlike the production GroupOps serializer, this explicitly preserves field
/// operations and Montgomery reductions. Field counters exclude field additions.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupWork {
    pub adds: u64,
    pub doubles: u64,
    pub scalar_mults: u64,
    pub field_muls: u64,
    pub field_sqrs: u64,
    pub field_invs: u64,
    pub montgomery_reductions: u64,
}

impl GroupWork {
    pub fn from_ops(o: &GroupOps) -> Self {
        Self {
            adds: o.adds,
            doubles: o.doubles,
            scalar_mults: o.scalar_mults,
            field_muls: o.field_ops.muls,
            field_sqrs: o.field_ops.sqrs,
            field_invs: o.field_ops.invs,
            montgomery_reductions: o.montgomery_reductions,
        }
    }

    pub fn merge(&mut self, other: &Self) {
        self.adds += other.adds;
        self.doubles += other.doubles;
        self.scalar_mults += other.scalar_mults;
        self.field_muls += other.field_muls;
        self.field_sqrs += other.field_sqrs;
        self.field_invs += other.field_invs;
        self.montgomery_reductions += other.montgomery_reductions;
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuxiliaryCounters {
    pub curves_tried: u64,
    pub singular: u64,
    pub filtered_out: u64,
    pub orders_computed: u64,
    pub filter_false_positives: u64,
    pub orders_undetermined: u64,
    pub two_sylow_rejected: u64,
    /// Includes count_gae: these native stage counters must not be summed.
    pub search_gae: u64,
    pub count_gae: u64,
    pub build_gae: u64,
    pub legendre_symbols: u64,
    pub sqrt_solves: u64,
    pub velu_evaluations: u64,
    pub bsgs_table_entries: u64,
}

impl From<&IsogenyTowerCost> for AuxiliaryCounters {
    fn from(c: &IsogenyTowerCost) -> Self {
        Self {
            curves_tried: c.curves_tried,
            singular: c.singular,
            filtered_out: c.filtered_out,
            orders_computed: c.orders_computed,
            filter_false_positives: c.filter_false_positives,
            orders_undetermined: c.orders_undetermined,
            two_sylow_rejected: c.two_sylow_rejected,
            search_gae: c.search_gae,
            count_gae: c.count_gae,
            build_gae: c.build_gae,
            legendre_symbols: c.legendre_symbols,
            sqrt_solves: c.sqrt_solves,
            velu_evaluations: c.velu_evaluations,
            bsgs_table_entries: c.bsgs_table_entries,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildAttempt {
    pub seed: u64,
    pub disposition: String,
    pub group_work: GroupWork,
    /// Native detail is unavailable when the production constructor returns Err;
    /// its available GroupOps are nevertheless retained in group_work.
    pub auxiliary_counters: Option<AuxiliaryCounters>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildCost {
    pub attempted_seed_builds: u64,
    pub rejected_seed_builds: u64,
    pub auxiliary_group_work: GroupWork,
    pub target_group_work: GroupWork,
    pub coordinate_trials: u64,
    pub duplicate_coordinate_trials: u64,
    pub target_legendre_calls: u64,
    pub target_sqrt_calls: u64,
    pub target_sqrt_modular_muls: u64,
    pub target_subgroup_checks: u64,
    /// Number of successful Dickson reverse-lifting square roots. The imported
    /// implementation does not expose its field multiplication count.
    pub dickson_sqrt_calls: u64,
    pub attempts: Vec<BuildAttempt>,
    /// Always true: polynomial certification, native sampling/field additions,
    /// allocation and some imported construction operations are not fully counted.
    pub native_operation_counts_partial: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuxiliaryStep {
    pub xi: u64,
    pub tau: u64,
    pub a: u64,
    pub b: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuxiliaryDefinition {
    pub curve: CurveDefinition,
    pub order: u64,
    pub kernel_generator: PointDefinition,
    pub coset_point: PointDefinition,
    pub steps: Vec<AuxiliaryStep>,
    pub terminal: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiberCertificate {
    /// Coefficients in ascending powers, reduced modulo p.
    pub numerator: Vec<u64>,
    pub denominator: Vec<u64>,
    pub fiber_polynomial: Vec<u64>,
    pub target_membership_polynomial: Vec<u64>,
    pub fiber_cardinality: usize,
    pub target_columns: usize,
    pub kernel_order: Option<u64>,
    pub auxiliary_order_method: Option<String>,
    pub target_order_method: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BaseDefinition {
    pub curve: CurveDefinition,
    pub subgroup_order: u64,
    pub family: BaseFamily,
    pub seed: u64,
    pub accepted_seed: u64,
    pub depth: u32,
    /// Complete, sorted coordinate fiber, including non-lifting target x-values.
    pub v: Vec<u64>,
    pub usable_x: Vec<u64>,
    pub canonical_points: Vec<PointDefinition>,
    pub columns: usize,
    pub unusable_x_count: usize,
    pub tower: Option<TowerDescription>,
    pub auxiliary: Option<AuxiliaryDefinition>,
    pub cost: BuildCost,
    pub certificate: FiberCertificate,
}

impl BaseDefinition {
    pub fn target_curve(&self) -> PrimeCurve {
        self.curve.production()
    }

    pub fn factor_base(&self) -> Result<FactorBase<PrimePoint>, String> {
        // Validate the frozen definition before it becomes a solver input.
        verify_base_definition(self)?;
        let curve = self.target_curve();
        let mut points = Vec::with_capacity(2 * self.columns);
        let mut col_of = Vec::with_capacity(2 * self.columns);
        let mut coef_of = Vec::with_capacity(2 * self.columns);
        for (column, encoded) in self.canonical_points.iter().enumerate() {
            let point = encoded.production();
            points.push(point);
            col_of.push(column);
            coef_of.push(1);
            let negative = curve.neg(point);
            if negative != point {
                points.push(negative);
                col_of.push(column);
                coef_of.push(self.subgroup_order - 1);
            }
        }
        FactorBase::from_column_map(
            format!(
                "matched {:?}, seed {}, {} full-fiber roots, {} target columns",
                self.family,
                self.accepted_seed,
                self.v.len(),
                self.columns
            ),
            points,
            col_of,
            coef_of,
            self.columns,
            |p| curve.key(p),
            |p| curve.key(&curve.neg(*p)),
            |p| p.x,
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuildFailure {
    pub message: String,
    pub cost: BuildCost,
}

impl std::fmt::Display for BuildFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({} seeded attempts)",
            self.message, self.cost.attempted_seed_builds
        )
    }
}
impl std::error::Error for BuildFailure {}

fn fail(message: impl Into<String>, cost: BuildCost) -> BuildFailure {
    BuildFailure {
        message: message.into(),
        cost,
    }
}

fn validate_target(curve: &PrimeCurve, r: u64) -> Result<(), String> {
    if curve.p <= 3 || curve.p > u32::MAX as u64 || !is_prime(curve.p) {
        return Err("certified builder requires a prime field 3 < p <= 2^32-1".into());
    }
    if curve.a >= curve.p || curve.b >= curve.p || discriminant_zero(&CurveDefinition::from(curve))
    {
        return Err("target coefficients must be reduced and the curve nonsingular".into());
    }
    let (lo, hi) = hasse_bounds(curve.p);
    if r < lo || r > hi || r <= 2 || r == curve.p || !is_prime(r) || 2 * r <= hi {
        return Err(
            "this experiment requires a prime-order target, not an unchecked cofactor model".into(),
        );
    }
    Ok(())
}

/// Bounded number of production constructor invocations. Each production call
/// has its own curve-search cap; process deadlines bound point-sampling loops.
pub fn build_base(
    curve: &PrimeCurve,
    r: u64,
    depth: u32,
    seed: u64,
    family: BaseFamily,
    max_attempts: u32,
) -> Result<BaseDefinition, BuildFailure> {
    if family == BaseFamily::Dickson {
        return build_dickson_base(curve, r, depth, seed, max_attempts);
    }
    let mut cost = BuildCost {
        native_operation_counts_partial: true,
        ..Default::default()
    };
    validate_target(curve, r).map_err(|e| fail(e, cost.clone()))?;
    if family == BaseFamily::Plain {
        return Err(fail(
            "use build_plain_control with the reference's actual column count",
            cost,
        ));
    }
    if !(1..=MAX_CERTIFIED_DEPTH).contains(&depth) || !(1..=1024).contains(&max_attempts) {
        return Err(fail(
            "depth outside 1..=10 or max_attempts outside 1..=1024",
            cost,
        ));
    }
    for attempt in 0..max_attempts {
        let attempt_seed = seed
            .checked_add(u64::from(attempt))
            .ok_or_else(|| fail("seed sequence overflows u64", cost.clone()))?;
        cost.attempted_seed_builds += 1;
        let mut ops = GroupOps::default();
        let built = match family {
            BaseFamily::Ordinary => {
                prime_tower::isogeny_tower(curve.p, depth, attempt_seed, &mut ops)
            }
            BaseFamily::SsJ1728 => prime_tower::supersingular_tower(
                curve.p,
                depth,
                attempt_seed,
                AuxKind::SsJ1728,
                &mut ops,
            ),
            _ => unreachable!(),
        };
        let tower = match built {
            Ok(tower) => tower,
            Err(error) => {
                let work = GroupWork::from_ops(&ops);
                cost.auxiliary_group_work.merge(&work);
                cost.rejected_seed_builds += 1;
                cost.attempts.push(BuildAttempt {
                    seed: attempt_seed,
                    disposition: format!("constructor_error: {error}"),
                    group_work: work,
                    auxiliary_counters: None,
                });
                continue;
            }
        };
        let work = GroupWork::from_ops(&ops);
        cost.auxiliary_group_work.merge(&work);
        let detail = AuxiliaryCounters::from(&tower.cost);
        if family == BaseFamily::Ordinary && tower.aux_order == curve.p + 1 {
            cost.rejected_seed_builds += 1;
            cost.attempts.push(BuildAttempt {
                seed: attempt_seed,
                disposition: "rejected_trace_zero_auxiliary".into(),
                group_work: work,
                auxiliary_counters: Some(detail),
            });
            continue;
        }
        let auxiliary = auxiliary_definition(&tower);
        let description = TowerDescription::Velu {
            steps: tower.steps.iter().map(|s| (s.xi, s.tau)).collect(),
            terminal: tower.c,
        };
        let result = assemble_definition(
            curve,
            r,
            family,
            seed,
            attempt_seed,
            depth,
            tower.v,
            Some(description),
            Some(auxiliary),
            &mut cost,
        );
        match result {
            Ok(mut definition) => {
                cost.attempts.push(BuildAttempt {
                    seed: attempt_seed,
                    disposition: "accepted".into(),
                    group_work: work,
                    auxiliary_counters: Some(detail),
                });
                definition.cost = cost;
                return Ok(definition);
            }
            Err(error) => {
                cost.rejected_seed_builds += 1;
                cost.attempts.push(BuildAttempt {
                    seed: attempt_seed,
                    disposition: format!("rejected_definition: {error}"),
                    group_work: work,
                    auxiliary_counters: Some(detail),
                });
            }
        }
    }
    Err(fail(
        "no admissible certified whole fiber within the fixed seed budget",
        cost,
    ))
}

/// Whole Dickson fibers. The seed selects only a public additive coordinate
/// shift; the terminal is fixed at zero and no target-dependent tuning occurs.
pub fn build_dickson_base(
    curve: &PrimeCurve,
    r: u64,
    depth: u32,
    seed: u64,
    max_attempts: u32,
) -> Result<BaseDefinition, BuildFailure> {
    let mut cost = BuildCost {
        native_operation_counts_partial: true,
        ..Default::default()
    };
    validate_target(curve, r).map_err(|e| fail(e, cost.clone()))?;
    if !(1..=MAX_CERTIFIED_DEPTH).contains(&depth) || !(1..=1024).contains(&max_attempts) {
        return Err(fail(
            "depth outside 1..=10 or max_attempts outside 1..=1024",
            cost,
        ));
    }
    for attempt in 0..max_attempts {
        let selected = seed
            .checked_add(u64::from(attempt))
            .ok_or_else(|| fail("seed sequence overflows u64", cost.clone()))?;
        cost.attempted_seed_builds += 1;
        let shift = selected % curve.p;
        let v = match crate::dickson_abscissae(curve.p, depth, shift, 0) {
            Ok(v) => v,
            Err(error) => {
                cost.rejected_seed_builds += 1;
                cost.attempts.push(BuildAttempt {
                    seed: selected,
                    disposition: format!("constructor_error: {error}"),
                    group_work: GroupWork::default(),
                    auxiliary_counters: None,
                });
                // Splitting is shift-independent: further seeds cannot fix it.
                return Err(fail(
                    "Dickson fiber does not split at this field and depth",
                    cost,
                ));
            }
        };
        cost.dickson_sqrt_calls += (1u64 << depth) - 1;
        let result = assemble_definition(
            curve,
            r,
            BaseFamily::Dickson,
            seed,
            selected,
            depth,
            v,
            Some(TowerDescription::Dickson {
                depth: depth as usize,
                shift,
                terminal: 0,
            }),
            None,
            &mut cost,
        );
        match result {
            Ok(mut definition) => {
                cost.attempts.push(BuildAttempt {
                    seed: selected,
                    disposition: "accepted".into(),
                    group_work: GroupWork::default(),
                    auxiliary_counters: None,
                });
                definition.cost = cost;
                return Ok(definition);
            }
            Err(error) => {
                cost.rejected_seed_builds += 1;
                cost.attempts.push(BuildAttempt {
                    seed: selected,
                    disposition: format!("rejected_definition: {error}"),
                    group_work: GroupWork::default(),
                    auxiliary_counters: None,
                });
            }
        }
    }
    Err(fail(
        "no nonempty certified Dickson lift within seed budget",
        cost,
    ))
}

fn auxiliary_definition(t: &IsogenyTower) -> AuxiliaryDefinition {
    AuxiliaryDefinition {
        curve: CurveDefinition::from(&t.aux),
        order: t.aux_order,
        kernel_generator: t.q.into(),
        coset_point: t.r.into(),
        terminal: t.c,
        steps: t
            .steps
            .iter()
            .map(|s| AuxiliaryStep {
                xi: s.xi,
                tau: s.tau,
                a: s.a,
                b: s.b,
            })
            .collect(),
    }
}

/// Plain x-interval control, starting at seed mod p and wrapping once. Its
/// cardinality is the accepted reference fiber's actual number of target columns.
pub fn build_plain_control(
    curve: &PrimeCurve,
    r: u64,
    columns: usize,
    seed: u64,
    max_x_trials: u64,
) -> Result<BaseDefinition, BuildFailure> {
    let mut cost = BuildCost {
        native_operation_counts_partial: true,
        ..Default::default()
    };
    validate_target(curve, r).map_err(|e| fail(e, cost.clone()))?;
    if columns == 0 || columns > 1usize << MAX_CERTIFIED_DEPTH || max_x_trials == 0 {
        return Err(fail(
            "plain control requires 1..=1024 columns and a positive trial cap",
            cost,
        ));
    }
    let mut v = Vec::new();
    for i in 0..max_x_trials.min(curve.p) {
        let x = ((seed % curve.p) + i) % curve.p;
        cost.coordinate_trials += 1;
        cost.target_legendre_calls += 1;
        let rhs = curve.rhs(x);
        if curve.legendre(rhs) < 0 || rhs == 0 {
            continue;
        }
        // This prefilter performs no square root. Actual lifting and subgroup
        // checks occur exactly once in assemble_definition below.
        v.push(x);
        if v.len() == columns {
            break;
        }
    }
    if v.len() != columns {
        return Err(fail("plain control exhausted its coordinate budget", cost));
    }
    assemble_definition(
        curve,
        r,
        BaseFamily::Plain,
        seed,
        seed,
        0,
        v,
        None,
        None,
        &mut cost,
    )
    .map(|mut d| {
        d.cost = cost.clone();
        d
    })
    .map_err(|e| fail(e, cost))
}

#[allow(clippy::too_many_arguments)]
fn assemble_definition(
    curve: &PrimeCurve,
    r: u64,
    family: BaseFamily,
    seed: u64,
    accepted_seed: u64,
    depth: u32,
    mut v: Vec<u64>,
    tower: Option<TowerDescription>,
    auxiliary: Option<AuxiliaryDefinition>,
    cost: &mut BuildCost,
) -> Result<BaseDefinition, String> {
    v.sort_unstable();
    if v.windows(2).any(|w| w[0] == w[1]) || v.iter().any(|&x| x >= curve.p) {
        return Err("fiber contains repeated or unreduced coordinates".into());
    }
    let mut usable_x = Vec::new();
    let mut canonical_points = Vec::new();
    for &x in &v {
        cost.coordinate_trials += 1;
        cost.target_legendre_calls += 1;
        let rhs = curve.rhs(x);
        if curve.legendre(rhs) < 0 {
            continue;
        }
        cost.target_sqrt_calls += 1;
        let y = curve
            .sqrt_counted(rhs, &mut cost.target_sqrt_modular_muls)
            .ok_or("Legendre and square-root routines disagree")?;
        if y == 0 {
            return Err("2-torsion lift contradicts the declared odd prime-order target".into());
        }
        let pt = PrimePoint::affine(x, y.min(curve.p - y));
        cost.target_subgroup_checks += 1;
        let mut point_ops = GroupOps::default();
        let killed = curve.mul(&mut point_ops, pt, r).infinity;
        cost.target_group_work
            .merge(&GroupWork::from_ops(&point_ops));
        if !killed {
            return Err("factor-base lift is outside the declared target subgroup".into());
        }
        usable_x.push(x);
        canonical_points.push(PointDefinition::from(pt));
    }
    if usable_x.is_empty() {
        return Err("whole fiber has no usable target points".into());
    }
    let mut d = BaseDefinition {
        curve: CurveDefinition::from(curve),
        subgroup_order: r,
        family,
        seed,
        accepted_seed,
        depth,
        columns: usable_x.len(),
        unusable_x_count: v.len() - usable_x.len(),
        v,
        usable_x,
        canonical_points,
        tower,
        auxiliary,
        cost: cost.clone(),
        certificate: FiberCertificate::default(),
    };
    d.certificate = certificate_for(&d)?;
    Ok(d)
}

/// Recompute from serialized mathematical inputs, without production group,
/// square-root, rational-evaluation or polynomial routines.
pub fn verify_base_definition(d: &BaseDefinition) -> Result<(), String> {
    let computed = certificate_for(d)?;
    if computed != d.certificate {
        return Err("stored fiber certificate differs from recomputation".into());
    }
    Ok(())
}

fn certificate_for(d: &BaseDefinition) -> Result<FiberCertificate, String> {
    let e = &d.curve;
    let p = e.p;
    validate_target(&e.production(), d.subgroup_order)?;
    if d.v.is_empty()
        || d.v.len() > 1usize << MAX_CERTIFIED_DEPTH
        || d.v.iter().any(|&x| x >= p)
        || d.v.windows(2).any(|w| w[0] >= w[1])
    {
        return Err("invalid complete fiber inventory".into());
    }
    if d.columns == 0
        || d.columns != d.usable_x.len()
        || d.columns != d.canonical_points.len()
        || d.unusable_x_count + d.columns != d.v.len()
    {
        return Err("target cardinality fields disagree".into());
    }
    let mut expected_x = Vec::new();
    for &x in &d.v {
        let rhs = independent_rhs(e, x);
        if rhs == 0 {
            return Err("unexpected target 2-torsion root".into());
        }
        if pow_mod(rhs, (p - 1) / 2, p) == 1 {
            expected_x.push(x);
        }
    }
    if expected_x != d.usable_x {
        return Err("usable set omits or adds a liftable fiber root".into());
    }
    for (&x, &pt) in d.usable_x.iter().zip(&d.canonical_points) {
        if pt.infinity || pt.x != x || pt.y == 0 || pt.y > p / 2 || !independent_on_curve(e, pt) {
            return Err("malformed canonical target lift".into());
        }
        if !independent_scalar(e, pt, d.subgroup_order).infinity {
            return Err("independent target subgroup check failed".into());
        }
    }
    let product = polynomial_from_roots(&d.v, p);
    let target_poly = polynomial_from_roots(&d.usable_x, p);
    let mut numerator = vec![0, 1];
    let mut denominator = vec![1];
    let mut kernel_order = None;
    let mut order_method = None;
    let fiber_poly = match &d.tower {
        Some(TowerDescription::Velu { steps, terminal }) => {
            if !matches!(d.family, BaseFamily::Ordinary | BaseFamily::SsJ1728)
                || d.depth == 0
                || d.depth > MAX_CERTIFIED_DEPTH
                || steps.len() != d.depth as usize
                || d.v.len() != 1usize << d.depth
            {
                return Err("Vélu family/depth/cardinality mismatch".into());
            }
            let aux = d.auxiliary.as_ref().ok_or("missing auxiliary definition")?;
            if aux.curve.p != p
                || aux.curve.a >= p
                || aux.curve.b >= p
                || discriminant_zero(&aux.curve)
                || aux.steps.len() != steps.len()
                || aux.terminal != *terminal
            {
                return Err("invalid auxiliary curve or step inventory".into());
            }
            if d.family == BaseFamily::Ordinary && aux.order == p + 1 {
                return Err("ordinary arm contains a trace-zero auxiliary curve".into());
            }
            if d.family == BaseFamily::SsJ1728
                && (p % 4 != 3 || aux.curve.a != 1 || aux.curve.b != 0 || aux.order != p + 1)
            {
                return Err("supersingular arm does not match its certified family".into());
            }
            let q = aux.kernel_generator;
            let coset = aux.coset_point;
            if q.infinity
                || coset.infinity
                || !independent_on_curve(&aux.curve, q)
                || !independent_on_curve(&aux.curve, coset)
            {
                return Err("invalid auxiliary kernel or coset point".into());
            }
            let order = 1u64 << d.depth;
            if !independent_scalar(&aux.curve, q, order).infinity
                || independent_scalar(&aux.curve, q, order / 2).infinity
            {
                return Err("kernel generator has the wrong exact order".into());
            }
            kernel_order = Some(order);
            let mut coset_roots = BTreeSet::new();
            let mut current = coset;
            for _ in 0..order {
                if current.infinity {
                    return Err("auxiliary coset contains infinity".into());
                }
                coset_roots.insert(current.x);
                current = independent_add(&aux.curve, current, q);
            }
            if current != coset || coset_roots.into_iter().collect::<Vec<_>>() != d.v {
                return Err("independent auxiliary coset differs from the whole fiber".into());
            }
            order_method = Some(if d.family == BaseFamily::SsJ1728 {
                "p=3 mod 4; y^2=x^3+x; character-pair cancellation gives p+1".into()
            } else {
                certify_auxiliary_order(aux)?
            });
            let mut curve_j = aux.curve.clone();
            for (record, &(xi, tau)) in aux.steps.iter().zip(steps) {
                if record.a != curve_j.a
                    || record.b != curve_j.b
                    || record.xi != xi
                    || record.tau != tau
                    || xi >= p
                    || tau == 0
                    || tau >= p
                    || independent_rhs(&curve_j, xi) != 0
                    || add(mul(3, mul(xi, xi, p), p), curve_j.a, p) != tau
                {
                    return Err("invalid Vélu kernel/curve transition".into());
                }
                let nd = poly_mul(&numerator, &denominator, p);
                let next_n = poly_add(
                    &poly_sub(
                        &poly_mul(&numerator, &numerator, p),
                        &poly_scale(&nd, xi, p),
                        p,
                    ),
                    &poly_scale(&poly_mul(&denominator, &denominator, p), tau, p),
                    p,
                );
                let next_d = poly_mul(
                    &denominator,
                    &poly_sub(&numerator, &poly_scale(&denominator, xi, p), p),
                    p,
                );
                numerator = next_n;
                denominator = next_d;
                curve_j.a = sub(curve_j.a, mul(5, tau, p), p);
                curve_j.b = sub(curve_j.b, mul(7, mul(xi, tau, p), p), p);
            }
            poly_sub(&numerator, &poly_scale(&denominator, *terminal, p), p)
        }
        Some(TowerDescription::Dickson {
            depth,
            shift,
            terminal,
        }) => {
            if d.family != BaseFamily::Dickson
                || *depth != d.depth as usize
                || d.depth == 0
                || d.depth > MAX_CERTIFIED_DEPTH
                || d.v.len() != 1usize << d.depth
                || *shift >= p
                || *terminal >= p
                || d.auxiliary.is_some()
            {
                return Err("Dickson family/depth/cardinality mismatch".into());
            }
            numerator = vec![sub(0, *shift, p), 1];
            for _ in 0..*depth {
                numerator = poly_sub(&poly_mul(&numerator, &numerator, p), &[2 % p], p);
            }
            poly_sub(&numerator, &[*terminal], p)
        }
        None => {
            if d.family != BaseFamily::Plain
                || d.depth != 0
                || d.auxiliary.is_some()
                || d.unusable_x_count != 0
            {
                return Err("plain control contains unexpected tower metadata".into());
            }
            numerator = product.clone();
            product.clone()
        }
    };
    if fiber_poly != product {
        return Err("fiber equation differs from the full root product".into());
    }
    for &x in &d.v {
        if poly_eval(&denominator, x, p) == 0 {
            return Err("complete fiber contains a rational-map pole".into());
        }
    }
    let complement: Vec<u64> =
        d.v.iter()
            .copied()
            .filter(|x| d.usable_x.binary_search(x).is_err())
            .collect();
    if poly_mul(&target_poly, &polynomial_from_roots(&complement, p), p) != product {
        return Err("target membership polynomial is not the exact admissible factor".into());
    }
    Ok(FiberCertificate {
        numerator,
        denominator,
        fiber_polynomial: product,
        target_membership_polynomial: target_poly,
        fiber_cardinality: d.v.len(),
        target_columns: d.columns,
        kernel_order,
        auxiliary_order_method: order_method,
        target_order_method:
            "prime r; nonzero target lift killed by r; 2r exceeds Hasse upper bound".into(),
    })
}

fn certify_auxiliary_order(aux: &AuxiliaryDefinition) -> Result<String, String> {
    let n = aux.order;
    let (lo, hi) = hasse_bounds(aux.curve.p);
    if n < lo || n > hi {
        return Err("auxiliary order lies outside Hasse interval".into());
    }
    let factors = prime_factors(n);
    let mut orders = Vec::new();
    for pt in [aux.kernel_generator, aux.coset_point] {
        if !independent_scalar(&aux.curve, pt, n).infinity {
            return Err("reported auxiliary order fails to annihilate a supplied point".into());
        }
        let mut o = n;
        for &ell in &factors {
            while o.is_multiple_of(ell) && independent_scalar(&aux.curve, pt, o / ell).infinity {
                o /= ell;
            }
        }
        orders.push(o);
    }
    let modulus = orders[0] / gcd(orders[0], orders[1]) * orders[1];
    let first = lo.div_ceil(modulus) * modulus;
    if first != n || first + modulus <= hi {
        return Err("auxiliary points do not certify a unique Hasse-interval order".into());
    }
    Ok(format!(
        "exact point orders {} and {}; unique Hasse multiple of {}",
        orders[0], orders[1], modulus
    ))
}

fn add(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + b as u128) % p as u128) as u64
}
fn sub(a: u64, b: u64, p: u64) -> u64 {
    add(a, p - b % p, p)
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    (a as u128 * b as u128 % p as u128) as u64
}
fn pow_mod(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut out = 1;
    while n != 0 {
        if n & 1 != 0 {
            out = mul(out, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    out
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn is_prime(n: u64) -> bool {
    n >= 2 && (2..=n.isqrt()).all(|d| !n.is_multiple_of(d))
}
fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut d = 2;
    while d <= n / d {
        if n.is_multiple_of(d) {
            out.push(d);
            while n.is_multiple_of(d) {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        out.push(n);
    }
    out
}
fn hasse_bounds(p: u64) -> (u64, u64) {
    let w = (4 * p).isqrt();
    (p + 1 - w, p + 1 + w)
}
fn discriminant_zero(e: &CurveDefinition) -> bool {
    add(
        mul(4, mul(mul(e.a, e.a, e.p), e.a, e.p), e.p),
        mul(27, mul(e.b, e.b, e.p), e.p),
        e.p,
    ) == 0
}
fn independent_rhs(e: &CurveDefinition, x: u64) -> u64 {
    add(
        add(mul(mul(x, x, e.p), x, e.p), mul(e.a, x, e.p), e.p),
        e.b,
        e.p,
    )
}
pub fn independent_on_curve(e: &CurveDefinition, point: PointDefinition) -> bool {
    point.infinity
        || (point.x < e.p
            && point.y < e.p
            && mul(point.y, point.y, e.p) == independent_rhs(e, point.x))
}
pub fn independent_add(
    e: &CurveDefinition,
    a: PointDefinition,
    b: PointDefinition,
) -> PointDefinition {
    if a.infinity {
        return b;
    }
    if b.infinity {
        return a;
    }
    let infinity = PointDefinition {
        x: 0,
        y: 0,
        infinity: true,
    };
    if a.x == b.x && add(a.y, b.y, e.p) == 0 {
        return infinity;
    }
    let (numerator, denominator) = if a.x == b.x {
        (
            add(mul(3, mul(a.x, a.x, e.p), e.p), e.a, e.p),
            mul(2, a.y, e.p),
        )
    } else {
        (sub(b.y, a.y, e.p), sub(b.x, a.x, e.p))
    };
    let slope = mul(numerator, pow_mod(denominator, e.p - 2, e.p), e.p);
    let x = sub(sub(mul(slope, slope, e.p), a.x, e.p), b.x, e.p);
    PointDefinition {
        x,
        y: sub(mul(slope, sub(a.x, x, e.p), e.p), a.y, e.p),
        infinity: false,
    }
}
pub fn independent_scalar(
    e: &CurveDefinition,
    mut a: PointDefinition,
    mut n: u64,
) -> PointDefinition {
    let mut out = PointDefinition {
        x: 0,
        y: 0,
        infinity: true,
    };
    while n != 0 {
        if n & 1 != 0 {
            out = independent_add(e, out, a);
        }
        a = independent_add(e, a, a);
        n >>= 1;
    }
    out
}
fn trim(mut a: Vec<u64>) -> Vec<u64> {
    while a.len() > 1 && a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn poly_add(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let mut out = vec![0; a.len().max(b.len())];
    for (i, &v) in a.iter().enumerate() {
        out[i] = v;
    }
    for (i, &v) in b.iter().enumerate() {
        out[i] = add(out[i], v, p);
    }
    trim(out)
}
fn poly_sub(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    poly_add(a, &poly_scale(b, p - 1, p), p)
}
fn poly_scale(a: &[u64], c: u64, p: u64) -> Vec<u64> {
    trim(a.iter().map(|&x| mul(x, c, p)).collect())
}
fn poly_mul(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] = add(out[i + j], mul(x, y, p), p);
        }
    }
    trim(out)
}
pub fn polynomial_from_roots(xs: &[u64], p: u64) -> Vec<u64> {
    xs.iter()
        .fold(vec![1], |acc, &x| poly_mul(&acc, &[sub(0, x, p), 1], p))
}
fn poly_eval(a: &[u64], x: u64, p: u64) -> u64 {
    a.iter().rev().fold(0, |acc, &v| add(mul(acc, x, p), v, p))
}
