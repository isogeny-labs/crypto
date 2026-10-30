//! Bounded APS instrument. Cold wall time includes every attempt and error;
//! native operation pricing remains explicitly incomplete. No planted scalar
//! is accepted, and the ordinary-residue backend checks recovered witnesses.

use super::{
    canonical::digest,
    methods::{OnlineWindow, ParamDecl, PhaseRecord, ResolvedMethod, SolveReport},
};
use crate::cryptanalysis::{
    aps_single_relation::{self as core, AlgebraReport, Encoding, LabeledBase, Labels, Recovery},
    ic_boundary::{CountedGroup, GroupOps, PrimeInstance, PrimePoint},
    prime_fast::FastCurve,
    residual_walk::is_prime_u64,
};
use serde::Serialize;
use serde_json::json;
use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

pub const ID: &str = "ic.aps_v1";
pub const PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "encoding",
        default: None,
        help: "partitioned3, unordered3, line3, expanded4, split4 or block_symmetric4",
    },
    ParamDecl {
        name: "roots",
        default: None,
        help: "exact (domains/factoring) or legacy (full-field scan)",
    },
    ParamDecl {
        name: "stop",
        default: None,
        help: "exhaustive or first_informative; legacy requires exhaustive",
    },
    ParamDecl {
        name: "labels",
        default: None,
        help: "random or guaranteed",
    },
    ParamDecl {
        name: "subset_size",
        default: None,
        help: "1..4 seed abscissae per original partition",
    },
    ParamDecl {
        name: "attempts",
        default: None,
        help: "1..128 independent bases, including failed draws",
    },
    ParamDecl {
        name: "degree",
        default: None,
        help: "F4 degree cap, 1..24",
    },
    ParamDecl {
        name: "budget_ms",
        default: None,
        help: "1..60000 cooperative milliseconds per complete attempt",
    },
    ParamDecl {
        name: "sampling_draws",
        default: None,
        help: "1..10000 draws per attempt; aborted attempts retained",
    },
    ParamDecl {
        name: "orbit",
        default: None,
        help: "none or cubic (certified order-three orbit expansion)",
    },
    ParamDecl {
        name: "beta",
        default: None,
        help: "field automorphism parameter; 0 when orbit=none",
    },
    ParamDecl {
        name: "lambda",
        default: None,
        help: "subgroup eigenvalue; 0 when orbit=none",
    },
    ParamDecl {
        name: "prune",
        default: None,
        help: "true only for certified cubic line3; otherwise false",
    },
];

#[derive(Clone, Debug)]
struct Config {
    encoding: Encoding,
    legacy: bool,
    first: bool,
    labels: Labels,
    size: usize,
    attempts: usize,
    degree: u32,
    budget: Duration,
    draws: usize,
    orbit: bool,
    beta: u64,
    lambda: u64,
    prune: bool,
}

impl Config {
    fn parse(m: &ResolvedMethod) -> Result<Self, String> {
        let text = |key: &str| {
            m.params
                .get(key)
                .map(String::as_str)
                .ok_or_else(|| format!("missing APS {key}"))
        };
        let number = |key: &str, low, high| -> Result<u64, String> {
            let n = text(key)?
                .parse::<u64>()
                .map_err(|_| format!("invalid APS {key}"))?;
            if !(low..=high).contains(&n) {
                return Err(format!("APS {key} must be {low}..={high}"));
            }
            Ok(n)
        };
        let choose = |key: &str, a, b| -> Result<bool, String> {
            match text(key)? {
                value if value == a => Ok(false),
                value if value == b => Ok(true),
                _ => Err(format!("APS {key} must be {a} or {b}")),
            }
        };
        let encoding = match text("encoding")? {
            "partitioned3" => Encoding::Partitioned3,
            "unordered3" => Encoding::Unordered3,
            "line3" => Encoding::Line3,
            "expanded4" => Encoding::Expanded4,
            "split4" => Encoding::Split4,
            "block_symmetric4" => Encoding::BlockSymmetric4,
            _ => return Err("unsupported APS encoding".into()),
        };
        let out = Self {
            encoding,
            legacy: choose("roots", "exact", "legacy")?,
            first: choose("stop", "exhaustive", "first_informative")?,
            labels: if choose("labels", "random", "guaranteed")? {
                Labels::Guaranteed
            } else {
                Labels::Random
            },
            size: number("subset_size", 1, 4)? as usize,
            attempts: number("attempts", 1, 128)? as usize,
            degree: number("degree", 1, 24)? as u32,
            budget: Duration::from_millis(number("budget_ms", 1, 60_000)?),
            draws: number("sampling_draws", 1, 10_000)? as usize,
            orbit: choose("orbit", "none", "cubic")?,
            beta: number("beta", 0, core::MAX_FIELD - 1)?,
            lambda: number("lambda", 0, 1 << 20)?,
            prune: choose("prune", "false", "true")?,
        };
        if out.legacy && out.first {
            return Err("legacy APS roots require exhaustive stopping".into());
        }
        if !out.orbit && (out.beta != 0 || out.lambda != 0 || out.prune) {
            return Err("non-orbit APS requires beta=lambda=0 and prune=false".into());
        }
        if out.orbit && (out.arity() != 3 || out.size > 3) {
            return Err(
                "cubic orbit APS requires arity three and at most three seed points per partition"
                    .into(),
            );
        }
        if out.prune && out.encoding != Encoding::Line3 {
            return Err("APS pruning requires line3".into());
        }
        Ok(out)
    }

    fn arity(&self) -> usize {
        match self.encoding {
            Encoding::Partitioned3 | Encoding::Unordered3 | Encoding::Line3 => 3,
            _ => 4,
        }
    }
}

fn fast(inst: &PrimeInstance) -> FastCurve {
    FastCurve::new(
        "ecbench-aps",
        inst.curve.p,
        inst.curve.a,
        inst.curve.b,
        inst.r,
    )
}

/// Plan-time refusals. Orbit certification uses the actual subgroup generator,
/// never a known logarithm, and the same check is repeated on the live target.
pub fn applicable(m: &ResolvedMethod, inst: &PrimeInstance) -> Result<(), String> {
    let c = Config::parse(m)?;
    if inst.curve.p > core::MAX_FIELD || inst.curve.p <= 3 || !is_prime_u64(inst.curve.p) {
        return Err("APS supports prime fields 5..=65521 only".into());
    }
    if inst.generator.0 >= inst.curve.p || inst.generator.1 >= inst.curve.p {
        return Err("APS generator must have canonical field coordinates".into());
    }
    if c.arity() * c.size * if c.orbit { 3 } else { 1 } > 32 {
        return Err("APS expanded base exceeds 32 abscissae".into());
    }
    if (c.arity() * c.size * if c.orbit { 3 } else { 1 }) as u64 > inst.r.saturating_sub(1) / 2 {
        return Err("APS base exceeds subgroup abscissa population".into());
    }
    if c.labels == Labels::Guaranteed {
        core::guaranteed_weights(c.arity(), inst.r)?;
    }
    let f = fast(inst);
    let g = f.point(inst.generator.0, inst.generator.1);
    core::validate_instance(&f, g, g)?;
    if c.orbit {
        // A one-point seed is sufficient to certify the map and eigenvalue;
        // no stochastic base-generation or attack is run while planning.
        let seed = core::LabeledPoint {
            x: inst.generator.0,
            y: inst.generator.1,
            u: 1,
            v: 0,
        };
        let base = LabeledBase {
            parts: vec![vec![seed.x]],
            points: [(seed.x, seed)].into(),
            draws: 0,
        };
        core::orbit_base(&f, g, g, &base, c.beta, c.lambda)?;
    }
    Ok(())
}

const ONLINE: [&str; 5] = [
    "target_query",
    "target_PDP",
    "target_relation_check",
    "target_descent",
    "target_recovery_check",
];

/// One monotonic timeline. Every boundary charges the entire intervening span
/// to exactly one phase, including bookkeeping, errors and clock overhead.
struct Clock {
    start: Instant,
    last: Instant,
    phase: &'static str,
    elapsed: BTreeMap<String, u64>,
}
impl Clock {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            start: now,
            last: now,
            phase: "setup",
            elapsed: [("setup".into(), 0)]
                .into_iter()
                .chain(ONLINE.map(|s| (s.into(), 0)))
                .collect(),
        }
    }
    fn mark(&mut self, next: &'static str) {
        let now = Instant::now();
        let ns =
            u64::try_from(now.duration_since(self.last).as_nanos()).expect("bounded APS interval");
        *self
            .elapsed
            .get_mut(self.phase)
            .expect("registered APS phase") += ns;
        self.last = now;
        self.phase = next;
    }
    fn recovery(&mut self, p: core::RecoveryPhase) {
        self.mark(match p {
            core::RecoveryPhase::RelationCheck => ONLINE[2],
            core::RecoveryPhase::Descent => ONLINE[3],
            core::RecoveryPhase::Verification => ONLINE[4],
        });
    }
    fn finish(mut self) -> (u64, BTreeMap<String, u64>) {
        self.mark(self.phase);
        let total = self.last.duration_since(self.start).as_nanos() as u64;
        assert_eq!(self.elapsed.values().sum::<u64>(), total);
        (total, self.elapsed)
    }
}

/// Literal equations, with canonical field residues and named variables. These
/// bytes bind the external comparison to the actual constructed system.
#[derive(Clone, Debug, Serialize)]
pub struct SystemExport {
    pub schema: &'static str,
    pub p: u64,
    pub variables: Vec<String>,
    pub ordering: &'static str,
    pub equations: Vec<crate::cryptanalysis::f4_fp::Poly>,
}
impl SystemExport {
    pub fn new(system: &core::System) -> Self {
        Self {
            schema: "aps.system/v1",
            p: system.modulus(),
            variables: (0..system.variables()).map(|i| format!("x{i}")).collect(),
            ordering: "grevlex",
            equations: system.equations().to_vec(),
        }
    }
    pub fn sha256(&self) -> Result<String, String> {
        digest(&serde_json::to_value(self).map_err(|e| e.to_string())?)
    }
    /// msolve's native input grammar: variable list, characteristic, equations.
    pub fn msolve(&self) -> String {
        let polys: Vec<String> = self
            .equations
            .iter()
            .map(|poly| {
                if poly.is_empty() {
                    return "0".into();
                }
                poly.iter()
                    .map(|(powers, coefficient)| {
                        let mut factors = vec![coefficient.to_string()];
                        for (name, power) in self.variables.iter().zip(powers) {
                            match power {
                                0 => {}
                                1 => factors.push(name.clone()),
                                _ => factors.push(format!("{name}^{power}")),
                            }
                        }
                        factors.join("*")
                    })
                    .collect::<Vec<_>>()
                    .join("+")
            })
            .collect();
        format!(
            "{}\n{}\n{}\n",
            self.variables.join(","),
            self.p,
            polys.join(",\n")
        )
    }
}

#[derive(Clone, Serialize)]
struct Attempt {
    index: usize,
    seed: u64,
    base: Option<LabeledBase>,
    system: Option<SystemExport>,
    system_sha256: Option<String>,
    algebra: Option<AlgebraReport>,
    visited_coordinate_sets: Vec<Vec<u64>>,
    recovery: Option<Recovery>,
    independent_witness_verified: bool,
    outcome: &'static str,
    error: Option<String>,
    error_stage: Option<&'static str>,
    deadline_exhausted: bool,
}

/// The mathematical witness and the timed outcome are separate. A valid late
/// witness remains replayable evidence, but never becomes a successful solve.
fn finish_attempt(row: &mut Attempt, spent: Duration, budget: Duration) -> Option<u128> {
    row.deadline_exhausted |= spent >= budget;
    row.outcome = if row.error.is_some() {
        "error"
    } else if row.deadline_exhausted {
        if row.independent_witness_verified {
            "verified_after_deadline"
        } else {
            "deadline"
        }
    } else if row.independent_witness_verified && row.recovery.is_some() {
        "verified"
    } else if row.algebra.as_ref().is_some_and(|a| a.complete) {
        "no_informative_relation"
    } else {
        "incomplete"
    };
    (row.outcome == "verified").then(|| u128::from(row.recovery.as_ref().unwrap().logarithm))
}

/// Check labels and the signed zero sum with ordinary-residue affine arithmetic
/// distinct from FastCurve's Montgomery/Jacobian implementation. Count every
/// group operation this independent check performs; core work stays unknown.
fn check_witness(
    inst: &PrimeInstance,
    q: PrimePoint,
    base: &LabeledBase,
    r: &Recovery,
    ops: &mut GroupOps,
) -> Result<(), String> {
    if r.coordinates.len() != r.signs.len()
        || r.coordinates.is_empty()
        || r.b == 0
        || r.b >= inst.r
        || r.a >= inst.r
        || r.logarithm >= inst.r
    {
        return Err("malformed APS witness".into());
    }
    let curve = &inst.curve;
    let mut sum = curve.identity();
    let (mut a, mut b) = (0u64, 0u64);
    let mut seen = std::collections::BTreeSet::new();
    for (&x, &sign) in r.coordinates.iter().zip(&r.signs) {
        if !seen.insert(x) || !matches!(sign, -1 | 1) {
            return Err("duplicate coordinate or invalid sign".into());
        }
        let point = base.points.get(&x).ok_or("APS witness outside base")?;
        if point.x != x
            || point.u >= inst.r
            || point.v >= inst.r
            || point.x >= curve.p
            || point.y >= curve.p
        {
            return Err("noncanonical APS point or label".into());
        }
        let p = PrimePoint::affine(point.x, point.y);
        if !curve.is_on_curve(p) {
            return Err("APS witness point off curve".into());
        }
        let ug = curve.mul(ops, inst.generator_point(), point.u);
        let vq = curve.mul(ops, q, point.v);
        if curve.add(ops, ug, vq) != p {
            return Err("APS point label mismatch".into());
        }
        sum = curve.add(ops, sum, if sign == 1 { p } else { curve.neg(p) });
        let signed = |v| if sign == 1 || v == 0 { v } else { inst.r - v };
        a = (a + signed(point.u)) % inst.r;
        b = (b + signed(point.v)) % inst.r;
    }
    if !curve.is_identity(&sum)
        || a != r.a
        || b != r.b
        || !(a as u128 + b as u128 * r.logarithm as u128).is_multiple_of(inst.r as u128)
    {
        return Err("APS signed relation or modular recovery mismatch".into());
    }
    Ok(())
}

pub fn solve(
    m: &ResolvedMethod,
    inst: &PrimeInstance,
    q: PrimePoint,
    seed: u64,
) -> Result<SolveReport, String> {
    let mut clock = Clock::new();
    applicable(m, inst)?;
    let c = Config::parse(m)?;
    let f = fast(inst);
    let g = f.point(inst.generator.0, inst.generator.1);
    let mut attempts = Vec::new();
    let mut recovered = None;
    let mut audit_ops = GroupOps::default();
    let mut verification_ops = GroupOps::default();
    let mut counters = BTreeMap::<String, u64>::new();
    clock.mark(ONLINE[0]);
    let target = f.point(q.x, q.y);
    let target_error = if q.infinity || q.x >= inst.curve.p || q.y >= inst.curve.p {
        Some("noncanonical or infinite APS target".into())
    } else {
        core::validate_instance(&f, g, target).err()
    };
    if target_error.is_none() {
        for index in 0..c.attempts {
            clock.mark(ONLINE[0]);
            let started = Instant::now();
            let mut row = Attempt {
                index,
                seed: seed.wrapping_add(index as u64),
                base: None,
                system: None,
                system_sha256: None,
                algebra: None,
                visited_coordinate_sets: Vec::new(),
                recovery: None,
                independent_witness_verified: false,
                outcome: "not_started",
                error: None,
                error_stage: None,
                deadline_exhausted: false,
            };
            let result = (|| -> Result<(), String> {
                row.error_stage = Some("sampling");
                let base = core::sample_base(
                    &f,
                    g,
                    target,
                    &vec![c.size; c.arity()],
                    c.labels,
                    row.seed,
                    c.draws,
                )?;
                row.base = Some(base);
                if c.orbit {
                    row.error_stage = Some("orbit");
                    row.base = Some(core::orbit_base(
                        &f,
                        g,
                        target,
                        row.base.as_ref().unwrap(),
                        c.beta,
                        c.lambda,
                    )?);
                }
                let base = row.base.as_ref().unwrap();
                clock.mark(ONLINE[2]);
                row.error_stage = Some("base_validation");
                let recovery_context = core::RecoveryContext::new(&f, g, target, base)?;
                clock.mark(ONLINE[1]);
                row.error_stage = Some("construction");
                let parts = match c.encoding {
                    Encoding::Unordered3 | Encoding::Line3 => {
                        vec![base.points.keys().copied().collect()]
                    }
                    Encoding::BlockSymmetric4 => vec![
                        base.parts[..2].iter().flatten().copied().collect(),
                        base.parts[2..].iter().flatten().copied().collect(),
                    ],
                    _ => base.parts.clone(),
                };
                let system = if c.prune {
                    core::build_pruned_line_system(&f, g, target, base, c.beta, c.lambda)?
                } else {
                    core::build_system(&f, &parts, c.encoding)?
                };
                let export = SystemExport::new(&system);
                row.system_sha256 = Some(export.sha256()?);
                row.system = Some(export);
                let Some(remaining) = c
                    .budget
                    .checked_sub(started.elapsed())
                    .filter(|d| !d.is_zero())
                else {
                    row.deadline_exhausted = true;
                    return Ok(());
                };
                row.error_stage = Some("solver_or_visitor");
                let mut recovery = None;
                let algebra = if c.legacy {
                    core::solve_system_reference(&system, f.f.p, c.degree, remaining)?
                } else {
                    core::visit_system(&system, f.f.p, c.degree, remaining, |coordinates| {
                        row.visited_coordinate_sets.push(coordinates.to_vec());
                        if c.first {
                            recovery = recovery_context
                                .recover_observed(&[coordinates.to_vec()], |p| clock.recovery(p))?;
                            clock.mark(ONLINE[1]);
                        }
                        Ok(recovery.is_some())
                    })?
                };
                row.deadline_exhausted = algebra.timed_out;
                if c.legacy {
                    row.visited_coordinate_sets = algebra.coordinate_sets.clone();
                }
                row.algebra = Some(algebra);
                if !c.first {
                    row.error_stage = Some("recovery");
                    recovery = recovery_context
                        .recover_observed(&row.algebra.as_ref().unwrap().coordinate_sets, |p| {
                            clock.recovery(p)
                        })?;
                }
                row.recovery = recovery;
                if let Some(r) = &row.recovery {
                    row.error_stage = Some("independent_witness_check");
                    clock.mark(ONLINE[2]);
                    check_witness(inst, q, base, r, &mut audit_ops)?;
                    clock.mark(ONLINE[4]);
                    if inst
                        .curve
                        .mul(&mut verification_ops, inst.generator_point(), r.logarithm)
                        != q
                    {
                        return Err("ordinary-residue APS scalar verification failed".into());
                    }
                    row.independent_witness_verified = true;
                }
                row.error_stage = None;
                Ok(())
            })();
            if let Err(error) = result {
                row.error = Some(error);
            }
            recovered = finish_attempt(&mut row, started.elapsed(), c.budget);
            let checked_witness = row.independent_witness_verified;
            // Sampling caps and orbit overlap are retryable. An unexpected
            // construction/solver/check error ends this execution fail closed.
            let fatal =
                row.error.is_some() && !matches!(row.error_stage, Some("sampling" | "orbit"));
            attempts.push(row);
            if checked_witness || fatal {
                break;
            }
        }
    }
    // Stop before report aggregation/serialization, as other ecbench solvers do.
    let (wall, elapsed) = clock.finish();
    let outcome = if target_error.is_some() {
        "target_error"
    } else if recovered.is_some() {
        "verified"
    } else if attempts
        .last()
        .is_some_and(|a| a.outcome == "verified_after_deadline")
    {
        "verified_after_deadline"
    } else if attempts.last().is_some_and(|a| {
        a.outcome == "error" && !matches!(a.error_stage, Some("sampling" | "orbit"))
    }) {
        "error"
    } else {
        "attempts_exhausted"
    };
    let online_ns = ONLINE.iter().map(|s| elapsed[*s]).sum();
    let online = OnlineWindow {
        wall_ns: online_ns,
        phases_ns: ONLINE.map(|s| (s.into(), elapsed[s])).into(),
        zero_phases: ONLINE.iter().filter(|s| elapsed[**s] == 0).map(|s| ((*s).into(), "no measurable interval was charged to this phase before the recorded outcome".into())).collect(),
        start_event: "live target validation before all target-dependent base sampling".into(),
        stop_event: match outcome {
            "verified" => "ordinary-residue witness and scalar checks completed within the attempt deadline",
            "verified_after_deadline" => "valid witness checked after deadline; exhausted execution retains the late witness",
            "error" => "unexpected construction/solver/check error stopped the execution",
            _ => "bounded attempts ended without an in-budget verified recovery; errors/timeouts retained",
        }.into(),
        included_stages: ONLINE.map(String::from).into(),
        mapping: "query: live target conversion/validation, seeded labeled base and certified orbits; PDP: equations, export binding, F4 and roots; relation check: base-label validation, all signed candidates and independent ordinary-residue label/sum check; descent: modular A/B recovery; recovery check: producer and independent scalar checks. Intervening search bookkeeping is charged to the current phase; all failed work stays inside the interval. Post-run report aggregation/serialization is outside it.".into(),
    };
    let mut f4_muls = 0;
    let mut root_muls = 0;
    let mut missing_roots = 0;
    let mut missing_reports = 0;
    for row in &attempts {
        if let Some(a) = &row.algebra {
            f4_muls += a.f4_field_multiplications;
            if let Some(r) = &a.roots {
                root_muls += r.root_field_multiplications;
            } else {
                missing_roots += 1;
            }
        } else if row.error_stage == Some("solver_or_visitor") {
            missing_reports += 1;
        }
    }
    counters.insert("aps_attempts".into(), attempts.len() as u64);
    counters.insert(
        "aps_failed_attempts".into(),
        attempts.iter().filter(|a| a.error.is_some()).count() as u64,
    );
    counters.insert(
        "aps_deadline_attempts".into(),
        attempts.iter().filter(|a| a.deadline_exhausted).count() as u64,
    );
    counters.insert("aps_f4_multiplications_uncharged".into(), f4_muls);
    counters.insert("aps_root_multiplications_uncharged".into(), root_muls);
    counters.insert("aps_unreported_algebra_attempts".into(), missing_reports);
    counters.insert(
        "aps_legacy_root_count_unknown_attempts".into(),
        missing_roots,
    );
    let phases = elapsed
        .iter()
        .map(|(name, &ns)| {
            let ops = if name == ONLINE[2] {
                audit_ops
            } else if name == ONLINE[4] {
                verification_ops
            } else {
                GroupOps::default()
            };
            let native = if name == ONLINE[1] {
                [
                    ("aps_f4_multiplications_uncharged".into(), f4_muls),
                    ("aps_root_multiplications_uncharged".into(), root_muls),
                ]
                .into()
            } else {
                BTreeMap::new()
            };
            PhaseRecord {
                name: name.clone(),
                wall_ns: Some(ns),
                adds: ops.adds,
                doubles: ops.doubles,
                scalar_mults: ops.scalar_mults,
                gae: ops.gae(),
                native,
                ..Default::default()
            }
        })
        .collect();
    Ok(SolveReport {
        recovered, exhausted: recovered.is_none(), phases,
        total_gae: audit_ops.gae() + verification_ops.gae(), automorphisms_used: 2, counters,
        unpriced: vec!["aps_f4_multiplications_uncharged".into(), "aps_root_multiplications_uncharged".into(), "APS core group/field operations, polynomial construction, allocation and hashing are not completely counted; absent solver reports leave diagnostic counts unknown. No complete S or bitop ranking is supported".into()],
        deterministic: false,
        nondeterminism: vec!["cooperative wall deadline may change returned roots/attempt outcomes".into()],
        solve_wall_ns: wall,
        online: Some(online),
        detail: json!({
            "schema": "ecbench.aps/v1",
            "outcome": outcome,
            "target_error": target_error,
            "attempts": attempts,
            "wall_accounting_complete": true,
            "native_work_accounting_complete": false,
            "costs_attested": false,
            "complete_sets_independently_audited": false,
            "independent_witness_backend": "ic_boundary::PrimeCurve ordinary-residue affine",
            "post_run_serialization_in_algorithm_clock": false,
            "post_run_aggregation_in_algorithm_clock": false,
        }),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests;
