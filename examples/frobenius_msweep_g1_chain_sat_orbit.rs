//! Cell G1 of the 2026-10-27 Frobenius m-sweep: chained-S3 SAT, plain x-domain
//! against the exact Frobenius-orbit x-domain, at m = 4, 5, 6.
//!
//! STATUS: prepared, not run (amended 2026-10-27 before any data: PROTOCOL.md section 11). Read
//! `research/frobenius_msweep_20261027/g1-chain-sat-orbit/PROTOCOL.md` first.
//!
//! The driver has five subcommands (see RUN_CARD.md for the exact lines):
//!   oracle   exact sumset verdicts for every target of one cell (no solver)
//!   run      one arm over every target of one cell, one JSONL row per solve
//!   cap      per-cell conflict caps from the pilot rows (frozen rule)
//!   analyze  cost-per-relation comparison, A/A band, resolution check, decision rule
//!   plan     static design numbers (sizes, expected yield); solves nothing
//!
//! Instrument. Every arm runs through `shadow_solve`, a line-for-line copy of
//! the production loop `sat_decompose_chained_s3_with_domain`
//! (koblitz_index_calculus.rs) built only from public API, because that
//! function is private and its stats omit propagations and `word_ops`. The
//! copy is tied to the production entry points (`sat_decompose_chained_s3`,
//! `..._frobenius_orbit`, `..._frobenius_orbit_ordered`) by an exact equality
//! of verdict class, conflicts, patterns and models on `--tieout` targets per
//! cell, and by the unit tests. The solver is deterministic, so equality is
//! exact, not statistical.
//!
//! Certificates and the oracle are computed by code that does not use the SAT
//! solver: `verify_cert` re-adds the returned points in the group, and
//! `Oracle` is a meet-in-the-middle sumset over the factor-base points.

#![recursion_limit = "256"]

use crypto_lib::binary_ecc::curve::point_neg;
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::binary_semaev_chain_sat::{ChainBuildError, ChainedS3Encoding};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_explicit_frobenius_orbit_factor_base, points_with_x, sat_decompose_chained_s3,
    sat_decompose_chained_s3_frobenius_orbit, sat_decompose_chained_s3_frobenius_orbit_ordered,
    ChainS3Limits, ChainS3Stats, FrobeniusFactorBase, KoblitzCurve,
};
use crypto_lib::cryptanalysis::sat::SolveResult;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::time::Instant;

const MASTER_SEED: u64 = 2026_1027_01;

// ---------------------------------------------------------------- randomness

fn splitmix(z: u64) -> u64 {
    let mut z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn derive(parts: &[u64]) -> u64 {
    let mut h = splitmix(MASTER_SEED);
    for p in parts {
        h = splitmix(h ^ splitmix(*p));
    }
    h
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = splitmix(self.0);
        self.0
    }
    fn below(&mut self, n: u64) -> u64 {
        assert!(n > 0);
        // Modulo bias is below 2^-32 for the n < 2^31 used here.
        self.next() % n
    }
    #[cfg_attr(not(test), allow(dead_code))]
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

// --------------------------------------------------------------------- cells

#[derive(Clone, Copy, Debug)]
struct CellSpec {
    a: u8,
    n: u32,
    m: usize,
    k: usize,
    seed: u64,
}

impl CellSpec {
    fn id(&self) -> String {
        format!(
            "a{}-n{}-m{}-k{}-s{}",
            self.a, self.n, self.m, self.k, self.seed
        )
    }
}

struct CellData {
    spec: CellSpec,
    kc: KoblitzCurve,
    fb: FrobeniusFactorBase,
    xcodes: HashSet<u64>,
    r: u64,
}

fn code(x: &F2mElement) -> u64 {
    x.to_biguint().to_u64().expect("n <= 62")
}

fn pkey(p: &BinaryPoint) -> (u64, u64) {
    match p {
        BinaryPoint::Infinity => (u64::MAX, u64::MAX),
        BinaryPoint::Affine { x, y } => (code(x), code(y)),
    }
}

fn build_cell(spec: CellSpec) -> Result<CellData, String> {
    if spec.n > 62 {
        return Err("n > 62 unsupported by the u64 keys".into());
    }
    let kc = KoblitzCurve::new(spec.a, spec.n).ok_or("KoblitzCurve::new returned None")?;
    let r = kc
        .subgroup_order
        .to_u64()
        .ok_or("subgroup order exceeds u64")?;
    let mut rng = Rng(derive(&[spec.seed, spec.n as u64, spec.k as u64, 0xFB]));
    let mut reps: Vec<F2mElement> = Vec::new();
    let mut seen: HashSet<u64> = HashSet::new();
    let mut tries = 0u32;
    while reps.len() < spec.k {
        tries += 1;
        if tries > 100_000 {
            return Err("could not draw enough full-length orbits".into());
        }
        let s = 1 + rng.below(r - 1);
        let p = kc.mul(kc.generator(), &BigUint::from(s));
        let BinaryPoint::Affine { x, .. } = p else {
            continue;
        };
        let mut orbit = vec![code(&x)];
        let mut y = kc.frobenius_x(&x);
        while code(&y) != code(&x) && orbit.len() <= spec.n as usize {
            orbit.push(code(&y));
            y = kc.frobenius_x(&y);
        }
        if orbit.len() != spec.n as usize || orbit.iter().any(|c| seen.contains(c)) {
            continue;
        }
        seen.extend(orbit);
        reps.push(x);
    }
    let fb = build_explicit_frobenius_orbit_factor_base(&kc, &reps)
        .ok_or("build_explicit_frobenius_orbit_factor_base returned None")?;
    if fb.signed_orbits.len() != spec.k || fb.points.len() != 2 * spec.n as usize * spec.k {
        return Err(format!(
            "unexpected factor base: {} signed orbits, {} points",
            fb.signed_orbits.len(),
            fb.points.len()
        ));
    }
    let xcodes = fb
        .points
        .iter()
        .filter_map(|p| match p {
            BinaryPoint::Affine { x, .. } => Some(code(x)),
            BinaryPoint::Infinity => None,
        })
        .collect();
    Ok(CellData {
        spec,
        kc,
        fb,
        xcodes,
        r,
    })
}

// ------------------------------------------------------------------- targets

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Class {
    Planted,
    Natural,
    Coset,
}

impl Class {
    fn name(self) -> &'static str {
        match self {
            Class::Planted => "planted",
            Class::Natural => "natural",
            Class::Coset => "coset",
        }
    }
    fn tag(self) -> u64 {
        match self {
            Class::Planted => 1,
            Class::Natural => 2,
            Class::Coset => 3,
        }
    }
}

#[derive(Clone, Debug)]
struct Target {
    id: String,
    class: Class,
    point: BinaryPoint,
}

fn make_targets(cd: &CellData, n_nat: usize, n_pl: usize, n_coset: usize) -> Vec<Target> {
    let cell = cd.spec.id();
    let kc = &cd.kc;
    let mut out = Vec::new();
    for i in 0..n_nat {
        let mut rng = Rng(derive(&[cd.spec.seed, Class::Natural.tag(), i as u64]));
        let s = 1 + rng.below(cd.r - 1);
        out.push(Target {
            id: format!("{cell}-natural-{i}"),
            class: Class::Natural,
            point: kc.mul(kc.generator(), &BigUint::from(s)),
        });
    }
    for i in 0..n_pl {
        let mut rng = Rng(derive(&[cd.spec.seed, Class::Planted.tag(), i as u64]));
        let point = loop {
            let mut acc = BinaryPoint::Infinity;
            for _ in 0..cd.spec.m {
                let j = rng.below(cd.fb.points.len() as u64) as usize;
                acc = kc.add(&acc, &cd.fb.points[j]);
            }
            if acc != BinaryPoint::Infinity {
                break acc;
            }
        };
        out.push(Target {
            id: format!("{cell}-planted-{i}"),
            class: Class::Planted,
            point,
        });
    }
    for i in 0..n_coset {
        let mut rng = Rng(derive(&[cd.spec.seed, Class::Coset.tag(), i as u64]));
        let r_big = &kc.subgroup_order;
        let point = loop {
            let xv = rng.below(1u64 << cd.spec.n);
            let x = F2mElement::from_biguint(&BigUint::from(xv), cd.spec.n);
            let pts = points_with_x(&kc.curve, &x);
            if pts.is_empty() {
                continue;
            }
            let p = pts[(rng.next() & 1) as usize % pts.len()].clone();
            if p != BinaryPoint::Infinity && kc.mul(&p, r_big) != BinaryPoint::Infinity {
                break p;
            }
        };
        out.push(Target {
            id: format!("{cell}-coset-{i}"),
            class: Class::Coset,
            point,
        });
    }
    out
}

// -------------------------------------------------------------------- oracle

/// Exact verdict "target is a sum of exactly m factor-base points", by
/// meet in the middle over ordered tuples with repetition. No SAT solver.
struct Oracle {
    sa: Vec<BinaryPoint>,
    sb: HashSet<(u64, u64)>,
}

fn sum_levels(
    kc: &KoblitzCurve,
    base: &[BinaryPoint],
    up_to: usize,
    cap: usize,
) -> Result<Vec<Vec<BinaryPoint>>, String> {
    let mut seen: HashMap<(u64, u64), BinaryPoint> = HashMap::new();
    for p in base {
        seen.entry(pkey(p)).or_insert_with(|| p.clone());
    }
    let mut levels: Vec<Vec<BinaryPoint>> = vec![seen.into_values().collect()];
    for _ in 1..up_to {
        let mut next: HashMap<(u64, u64), BinaryPoint> = HashMap::new();
        for p in levels.last().unwrap() {
            for q in base {
                let s = kc.add(p, q);
                next.entry(pkey(&s)).or_insert(s);
            }
            if next.len() > cap {
                return Err(format!("oracle level exceeds cap {cap}"));
            }
        }
        levels.push(next.into_values().collect());
    }
    Ok(levels)
}

impl Oracle {
    fn new(cd: &CellData, cap: usize) -> Result<Self, String> {
        let m = cd.spec.m;
        let (a, b) = (m.div_ceil(2), m / 2);
        let mut levels = sum_levels(&cd.kc, &cd.fb.points, a, cap)?;
        let sb = levels[b - 1].iter().map(pkey).collect();
        let sa = levels.swap_remove(a - 1);
        Ok(Oracle { sa, sb })
    }
    fn decide(&self, kc: &KoblitzCurve, target: &BinaryPoint) -> bool {
        self.sa.iter().any(|s| {
            let d = kc.add(target, &point_neg(s));
            self.sb.contains(&pkey(&d))
        })
    }
}

// --------------------------------------------------------------- shadow solve

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ArmSpec {
    orbit: bool,
    ordered: bool,
}

fn arm_spec(name: &str) -> Option<ArmSpec> {
    match name {
        "P" => Some(ArmSpec {
            orbit: false,
            ordered: false,
        }),
        "O" => Some(ArmSpec {
            orbit: true,
            ordered: false,
        }),
        "R" => Some(ArmSpec {
            orbit: false,
            ordered: true,
        }),
        "Q" => Some(ArmSpec {
            orbit: true,
            ordered: true,
        }),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Verdict {
    Found,
    Refuted,
    Censored,
}

impl Verdict {
    fn name(self) -> &'static str {
        match self {
            Verdict::Found => "found",
            Verdict::Refuted => "refuted",
            Verdict::Censored => "censored",
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Counters {
    conflicts: u64,
    decisions: u64,
    propagations: u64,
    word_ops: u64,
    patterns_attempted: usize,
    patterns_refuted: usize,
    solver_calls: usize,
    models: usize,
    group_rejected: usize,
    invalid: usize,
    peak_vars: u32,
    peak_clauses: usize,
    peak_xors: usize,
    domain_clauses: usize,
    lift_adds: u64,
}

struct Outcome {
    verdict: Verdict,
    reason: &'static str,
    points: Option<Vec<BinaryPoint>>,
    c: Counters,
    wall_s: f64,
    build_s: f64,
}

fn lift(
    kc: &KoblitzCurve,
    xs: &[F2mElement],
    depth: usize,
    acc: &BinaryPoint,
    chosen: &mut Vec<BinaryPoint>,
    target: &BinaryPoint,
    adds: &mut u64,
) -> bool {
    if depth == xs.len() {
        return acc == target;
    }
    for p in points_with_x(&kc.curve, &xs[depth]) {
        *adds += 1;
        let next = kc.add(acc, &p);
        chosen.push(p);
        if lift(kc, xs, depth + 1, &next, chosen, target, adds) {
            return true;
        }
        chosen.pop();
    }
    false
}

fn absorb(c: &mut Counters, enc: &ChainedS3Encoding) {
    let s = &enc.solver.stats;
    c.decisions += s.decisions;
    c.propagations += s.propagations;
    c.word_ops += enc.solver.word_ops();
}

/// Copy of the production chained-S3 loop from public API only.
fn shadow_solve(
    cd: &CellData,
    target: &BinaryPoint,
    limits: ChainS3Limits,
    arm: ArmSpec,
) -> Outcome {
    use crypto_lib::cryptanalysis::binary_semaev_chain_sat::{
        finite_domain_clause_count, frobenius_orbit_domain_size,
    };
    let t0 = Instant::now();
    let (kc, fb, m) = (&cd.kc, &cd.fb, cd.spec.m);
    let mut c = Counters::default();
    let mut build_s = 0.0f64;
    let done = |verdict, reason, points, c: Counters, build_s: f64| Outcome {
        verdict,
        reason,
        points,
        c,
        wall_s: t0.elapsed().as_secs_f64(),
        build_s,
    };
    if !(4..=6).contains(&m)
        || limits.max_models == 0
        || limits.conflict_budget == 0
        || !kc.curve.is_on_curve(target)
        || !fb.uses_ambient_basis()
    {
        return done(Verdict::Censored, "precondition", None, c, build_s);
    }
    let mut codes: Vec<BigUint> = fb
        .points
        .iter()
        .filter_map(|p| match p {
            BinaryPoint::Affine { x, .. } => Some(x.to_biguint()),
            BinaryPoint::Infinity => None,
        })
        .collect();
    codes.sort_unstable();
    codes.dedup();
    let mut representatives: Vec<BigUint> = Vec::new();
    if arm.orbit {
        for orbit in &fb.signed_orbits {
            let Some(BinaryPoint::Affine { x, .. }) = orbit.first().and_then(|&i| fb.points.get(i))
            else {
                return done(Verdict::Censored, "orbit_representative", None, c, build_s);
            };
            representatives.push(x.to_biguint());
        }
        let mut closure = std::collections::BTreeSet::new();
        for rep in &representatives {
            let mut x = F2mElement::from_biguint(rep, kc.n);
            for _ in 0..kc.n {
                closure.insert(x.to_biguint());
                x = x.square_k_times(1, &kc.curve.irreducible);
            }
        }
        if !closure.iter().eq(codes.iter()) {
            return done(Verdict::Censored, "orbit_closure", None, c, build_s);
        }
    }
    let domain_clauses = if arm.orbit {
        frobenius_orbit_domain_size(kc.n, m, representatives.len()).map(|s| s.clauses)
    } else {
        finite_domain_clause_count(&codes, kc.n, m)
    };
    let Some(domain_clauses) = domain_clauses else {
        return done(Verdict::Censored, "domain_size", None, c, build_s);
    };
    if domain_clauses > limits.max_domain_clauses {
        return done(Verdict::Censored, "domain_cap", None, c, build_s);
    }
    c.domain_clauses = domain_clauses;
    let target_x = match target {
        BinaryPoint::Affine { x, .. } => Some(x),
        BinaryPoint::Infinity => None,
    };
    for identity_mask in 0..(1u32 << (m - 2)) {
        if c.conflicts >= limits.conflict_budget {
            return done(Verdict::Censored, "conflict_budget", None, c, build_s);
        }
        let tb = Instant::now();
        let mut enc = match ChainedS3Encoding::build(
            m,
            &kc.curve.irreducible,
            &kc.curve.b,
            target_x,
            identity_mask,
            limits.max_variables,
        ) {
            Ok(e) => e,
            Err(
                ChainBuildError::VariableCap { .. }
                | ChainBuildError::DomainClauseCap { .. }
                | ChainBuildError::Unsupported,
            ) => return done(Verdict::Censored, "var_cap", None, c, build_s),
        };
        let installed = if arm.orbit {
            enc.constrain_frobenius_orbit_summands(
                &representatives,
                limits.max_variables,
                limits.max_domain_clauses,
            )
            .map(|s| s.clauses)
        } else {
            enc.constrain_summands(&codes, limits.max_domain_clauses)
        };
        if installed != Ok(domain_clauses) {
            return done(Verdict::Censored, "domain_install", None, c, build_s);
        }
        if arm.ordered && enc.order_first_pair(limits.max_variables).is_err() {
            return done(Verdict::Censored, "order_first_pair", None, c, build_s);
        }
        build_s += tb.elapsed().as_secs_f64();
        c.patterns_attempted += 1;
        c.peak_vars = c.peak_vars.max(enc.solver.n_vars());
        c.peak_clauses = c.peak_clauses.max(enc.solver.n_clauses());
        c.peak_xors = c.peak_xors.max(enc.solver.n_xors());
        let earlier = c.conflicts;
        enc.solver.conflict_budget = limits.conflict_budget - earlier;
        loop {
            c.solver_calls += 1;
            let outcome = enc.solver.solve();
            c.conflicts = earlier.saturating_add(enc.solver.conflicts());
            match outcome {
                SolveResult::Unsat => {
                    c.patterns_refuted += 1;
                    absorb(&mut c, &enc);
                    break;
                }
                SolveResult::Unknown => {
                    absorb(&mut c, &enc);
                    return done(Verdict::Censored, "conflict_budget", None, c, build_s);
                }
                SolveResult::Sat => {
                    let xs = enc.decode_summands();
                    c.models += 1;
                    if !enc.verify_model() || xs.iter().any(|x| !cd.xcodes.contains(&code(x))) {
                        c.invalid += 1;
                        absorb(&mut c, &enc);
                        return done(Verdict::Censored, "invalid_model", None, c, build_s);
                    }
                    let mut chosen = Vec::new();
                    let mut adds = 0u64;
                    let ok = lift(
                        kc,
                        &xs,
                        0,
                        &BinaryPoint::Infinity,
                        &mut chosen,
                        target,
                        &mut adds,
                    );
                    c.lift_adds += adds;
                    if ok {
                        absorb(&mut c, &enc);
                        return done(Verdict::Found, "", Some(chosen), c, build_s);
                    }
                    c.group_rejected += 1;
                    if c.models >= limits.max_models {
                        absorb(&mut c, &enc);
                        return done(Verdict::Censored, "model_cap", None, c, build_s);
                    }
                    let model = enc.solver.model();
                    enc.solver.reset_search();
                    let block: Vec<i32> = (0..m * kc.n as usize)
                        .map(|i| {
                            let w = (i + 1) as i32;
                            if model[i] {
                                -w
                            } else {
                                w
                            }
                        })
                        .collect();
                    enc.solver.add_clause(block);
                }
            }
        }
    }
    done(Verdict::Refuted, "", None, c, build_s)
}

/// The production entry point for the arms that have one.
fn public_solve(
    cd: &CellData,
    target: &BinaryPoint,
    limits: ChainS3Limits,
    arm: ArmSpec,
) -> Option<(Option<Vec<usize>>, ChainS3Stats)> {
    let (kc, fb, m) = (&cd.kc, &cd.fb, cd.spec.m);
    match (arm.orbit, arm.ordered) {
        (false, false) => Some(sat_decompose_chained_s3(kc, fb, target, m, limits)),
        (true, false) => Some(sat_decompose_chained_s3_frobenius_orbit(
            kc, fb, target, m, limits,
        )),
        (true, true) => Some(sat_decompose_chained_s3_frobenius_orbit_ordered(
            kc, fb, target, m, limits,
        )),
        (false, true) => None,
    }
}

/// True iff the shadow loop and the production entry point agree exactly.
fn tie_out(
    cd: &CellData,
    target: &BinaryPoint,
    limits: ChainS3Limits,
    arm: ArmSpec,
) -> Option<bool> {
    let (found, stats) = public_solve(cd, target, limits, arm)?;
    let shadow = shadow_solve(cd, target, limits, arm);
    let class = if found.is_some() {
        Verdict::Found
    } else if stats.refuted {
        Verdict::Refuted
    } else {
        Verdict::Censored
    };
    Some(
        class == shadow.verdict
            && stats.conflicts == shadow.c.conflicts
            && stats.patterns_attempted == shadow.c.patterns_attempted
            && stats.models == shadow.c.models
            && stats.solver_calls == shadow.c.solver_calls,
    )
}

/// Certificate check that never touches the SAT solver.
fn verify_cert(cd: &CellData, target: &BinaryPoint, pts: &[BinaryPoint]) -> bool {
    if pts.len() != cd.spec.m {
        return false;
    }
    let mut acc = BinaryPoint::Infinity;
    for p in pts {
        let BinaryPoint::Affine { x, .. } = p else {
            return false;
        };
        if !cd.kc.curve.is_on_curve(p) || !cd.xcodes.contains(&code(x)) {
            return false;
        }
        acc = cd.kc.add(&acc, p);
    }
    &acc == target
}

// ------------------------------------------------------- certificates on disk

fn pt_json(p: &BinaryPoint) -> Value {
    match p {
        BinaryPoint::Affine { x, y } => json!([code(x), code(y)]),
        BinaryPoint::Infinity => Value::Null,
    }
}

struct RowCtx<'a> {
    run_id: &'a str,
    cd: &'a CellData,
    id: &'a str,
    class: &'a str,
    arm: &'a str,
    conj: u32,
    cap: u64,
    point: &'a BinaryPoint,
    o: &'a Outcome,
    want: bool,
    agree: bool,
    cert_ok: bool,
    tie: Option<bool>,
}

/// One JSONL row. A `found` row carries the m certificate points (x, y) and
/// the solved target point, so that the native verifier crate `verify_certs/` can re-add them with the
/// curve in the cell manifest, without the solver and without rerunning.
fn row_json(c: &RowCtx) -> Value {
    let (cd, o, spec) = (c.cd, c.o, c.cd.spec);
    let cert = match (&o.verdict, &o.points) {
        (Verdict::Found, Some(p)) => Value::Array(p.iter().map(pt_json).collect()),
        _ => Value::Null,
    };
    json!({
        "schema": "g1-chain-sat-orbit/row/v1", "run_id": c.run_id, "host": host(),
        "cell": spec.id(), "a": spec.a, "n": spec.n, "m": spec.m, "k": spec.k,
        "seed": spec.seed, "r": cd.r as f64, "fb_x": cd.xcodes.len(), "fb_points": cd.fb.points.len(),
        "target_id": c.id, "class": c.class, "arm": c.arm, "conj": c.conj,
        "target_xy": pt_json(c.point), "cert_points": cert,
        "verdict": o.verdict.name(), "censor_reason": o.reason, "oracle": c.want,
        "agree": c.agree, "cert_ok": c.cert_ok, "tieout": c.tie,
        "conflict_cap": c.cap,
        "conflicts": o.c.conflicts, "decisions": o.c.decisions,
        "propagations": o.c.propagations, "word_ops": o.c.word_ops,
        "patterns_attempted": o.c.patterns_attempted, "patterns_refuted": o.c.patterns_refuted,
        "solver_calls": o.c.solver_calls, "models": o.c.models,
        "group_rejected": o.c.group_rejected, "invalid_models": o.c.invalid,
        "peak_vars": o.c.peak_vars, "peak_clauses": o.c.peak_clauses,
        "peak_xors": o.c.peak_xors, "domain_clauses": o.c.domain_clauses,
        "lift_adds": o.c.lift_adds, "wall_s": o.wall_s, "build_s": o.build_s,
    })
}

/// Per-cell manifest: everything an independent verifier needs to re-add a
/// certificate (field polynomial, curve, generator, order, the factor-base
/// points and their orbits). Written by `oracle` next to the oracle file.
fn manifest_json(cd: &CellData) -> Value {
    let kc = &cd.kc;
    let mut xs: Vec<u64> = cd.xcodes.iter().copied().collect();
    xs.sort_unstable();
    json!({
        "schema": "g1-chain-sat-orbit/manifest/v1", "cell": cd.spec.id(),
        "n": cd.spec.n, "m": cd.spec.m, "k": cd.spec.k, "a": cd.spec.a, "seed": cd.spec.seed,
        "poly_degree": kc.curve.irreducible.degree, "poly_low_terms": kc.curve.irreducible.low_terms,
        "curve_a": code(&kc.curve.a), "curve_b": code(&kc.curve.b),
        "generator": pt_json(&kc.curve.generator), "subgroup_order_r": cd.r,
        "factor_base_points": cd.fb.points.iter().map(pt_json).collect::<Vec<_>>(),
        "factor_base_x_codes": xs,
        "signed_orbits": cd.fb.signed_orbits.iter().map(|o| o.iter().map(|&i| i as u64).collect::<Vec<_>>()).collect::<Vec<_>>(),
        "orbit_representative_x": cd.fb.signed_orbits.iter()
            .filter_map(|o| o.first().and_then(|&i| cd.fb.points.get(i)))
            .filter_map(|p| match p { BinaryPoint::Affine { x, .. } => Some(code(x)), _ => None })
            .collect::<Vec<_>>(),
    })
}

fn limits_for(cap: u64) -> ChainS3Limits {
    ChainS3Limits {
        max_variables: 400_000,
        max_domain_clauses: 20_000_000,
        max_models: 10_000,
        conflict_budget: cap,
    }
}

// ------------------------------------------------------------------- planning

/// Order r of the prime-order subgroup that holds every target and every sum
/// (r = #E / 4 for the a = 0 Koblitz curve, about 2^n / 4; not 2^n).
fn subgroup_r(n: u32) -> u64 {
    KoblitzCurve::new(0, n)
        .and_then(|kc| kc.subgroup_order.to_u64())
        .expect("Koblitz a=0 curve with u64 subgroup order")
}

/// Orbits needed so that the expected number of m-sums, (2kn)^m / m!, is about
/// 0.7 of the SUBGROUP order r (natural-target yield about 0.5). At least 2,
/// because the orbit arm needs two representatives; where k = 2 already
/// overshoots, the cell is saturated and stays at k = 2.
fn suggest_k(r: u64, n: u32, m: usize) -> usize {
    let fact: f64 = (1..=m).map(|i| i as f64).product();
    for k in 2..=64usize {
        let s = (2.0 * k as f64 * n as f64).powi(m as i32) / fact;
        if s >= 0.7 * r as f64 {
            return k;
        }
    }
    64
}

fn expected_yield(r: u64, n: u32, m: usize, k: usize) -> f64 {
    let fact: f64 = (1..=m).map(|i| i as f64).product();
    let s = (2.0 * k as f64 * n as f64).powi(m as i32) / fact;
    1.0 - (-s / r as f64).exp()
}

// ------------------------------------------------------------------ cap rule

const CAP_GRID: [u64; 7] = [
    20_000, 50_000, 100_000, 200_000, 500_000, 1_000_000, 2_000_000,
];

/// Frozen cap rule (PROTOCOL amendment 1). From the pilot rows of each cell
/// (every pilot row of arms P and O: n, m and k come from the cell id), per
/// cell:
///   t_conf = max over arms of  sum(wall_s - build_s) / sum(conflicts)
///            (ratio of sums over that arm's rows with conflicts > 0, so
///            construction time and the near-zero-conflict solves that bias a
///            median of per-row ratios upward do not enter),
///   b      = max build_s over those rows (CNF construction per target),
///   CAP    = the largest grid value with b + CAP * t_conf <= budget seconds.
/// If even the smallest grid value exceeds the budget, the cell is an
/// O-IMPEDIMENT(budget) and is not run. Output: one `n:m:k:CAP` line per cell
/// (or `n:m:k:IMPEDIMENT`, or `n:m:k:NODATA` for a cell whose pilot rows have
/// no solve with conflicts > 0), the exact cell tokens `run_stage.sh` takes.
fn cap_lines(dir: &str, budget: f64) -> Result<Vec<String>, String> {
    // cell id -> arm -> (sum solver seconds, sum conflicts), and max build_s
    let mut acc: HashMap<String, HashMap<String, (f64, f64)>> = HashMap::new();
    let mut maxb: HashMap<String, f64> = HashMap::new();
    for e in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let path = e.map_err(|e| e.to_string())?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("jsonl")
            || !path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("rows-")
        {
            continue;
        }
        let text = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
            let f = |k: &str| {
                v.get(k)
                    .and_then(Value::as_f64)
                    .ok_or(format!("row lacks {k}"))
            };
            let cell = v["cell"].as_str().ok_or("row lacks cell")?.to_string();
            let arm = v["arm"].as_str().ok_or("row lacks arm")?.to_string();
            if v["conj"].as_u64() != Some(0) || !(arm == "P" || arm == "O") {
                continue;
            }
            let (conf, wall, build) = (f("conflicts")?, f("wall_s")?, f("build_s")?);
            let b = maxb.entry(cell.clone()).or_insert(0.0);
            *b = b.max(build);
            if conf > 0.0 {
                let a = acc
                    .entry(cell)
                    .or_default()
                    .entry(arm)
                    .or_insert((0.0, 0.0));
                a.0 += (wall - build).max(0.0);
                a.1 += conf;
            }
        }
    }
    // Every cell with any pilot row is listed: one with no row of conflicts > 0
    // (a systematic precondition censor, say) gets an explicit NODATA token
    // instead of silently vanishing.
    let mut cells: Vec<&String> = maxb.keys().collect();
    cells.sort();
    let mut out = Vec::new();
    for cell in cells {
        let (n, m, k) = parse_cell(cell).ok_or("bad cell id")?;
        if !acc.contains_key(cell) {
            out.push(format!("{n}:{m}:{k}:NODATA"));
            continue;
        }
        let t_conf = acc[cell]
            .values()
            .map(|(t, c)| t / c)
            .fold(0.0f64, f64::max);
        let b = maxb[cell];
        let cap = CAP_GRID
            .iter()
            .rev()
            .find(|&&c| b + c as f64 * t_conf <= budget);
        out.push(match cap {
            Some(c) => format!("{n}:{m}:{k}:{c}"),
            None => format!("{n}:{m}:{k}:IMPEDIMENT"),
        });
    }
    Ok(out)
}

// ------------------------------------------------------------------- analysis

#[derive(Clone, Debug)]
struct Row {
    cell: String,
    target_id: String,
    class: String,
    arm: String,
    conj: u32,
    verdict: String,
    oracle: bool,
    agree: bool,
    cert_ok: bool,
    conflicts: f64,
    propagations: f64,
    word_ops: f64,
    wall_s: f64,
    lift_adds: f64,
    r: f64,
    k: f64,
}

fn parse_row(v: &Value) -> Option<Row> {
    let s = |k: &str| v.get(k)?.as_str().map(str::to_string);
    let f = |k: &str| v.get(k)?.as_f64();
    Some(Row {
        cell: s("cell")?,
        target_id: s("target_id")?,
        class: s("class")?,
        arm: s("arm")?,
        conj: f("conj")? as u32,
        verdict: s("verdict")?,
        oracle: v.get("oracle")?.as_bool()?,
        agree: v.get("agree")?.as_bool()?,
        cert_ok: v.get("cert_ok")?.as_bool()?,
        conflicts: f("conflicts")?,
        propagations: f("propagations")?,
        word_ops: f("word_ops")?,
        wall_s: f("wall_s")?,
        lift_adds: f("lift_adds")?,
        r: f("r")?,
        k: f("k")?,
    })
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn sd(v: &[f64]) -> f64 {
    if v.len() < 2 {
        return f64::NAN;
    }
    let mu = mean(v);
    (v.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / (v.len() - 1) as f64).sqrt()
}

/// Percentile bootstrap of the mean of `v` (95 percent), seed fixed.
fn boot_mean(v: &[f64], seed: u64) -> (f64, f64, f64) {
    if v.is_empty() {
        return (f64::NAN, f64::NAN, f64::NAN);
    }
    let mut rng = Rng(seed);
    let b = 2000;
    let mut means: Vec<f64> = (0..b)
        .map(|_| {
            let mut s = 0.0;
            for _ in 0..v.len() {
                s += v[rng.below(v.len() as u64) as usize];
            }
            s / v.len() as f64
        })
        .collect();
    means.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (
        mean(v),
        means[(0.025 * b as f64) as usize],
        means[(0.975 * b as f64) as usize],
    )
}

fn metric(r: &Row, name: &str, price: f64) -> f64 {
    match name {
        "conflicts" => r.conflicts,
        "propagations" => r.propagations,
        "word_ops" => r.word_ops,
        "wall_s" => r.wall_s,
        "gae" => r.lift_adds + price * r.word_ops,
        _ => f64::NAN,
    }
}

/// SECONDARY, survivors-only statistic: paired log ratio X/Y over targets
/// where both arms finished and agree with the oracle. It drops every pair in
/// which an arm was censored, so it is biased toward the arm censored more and
/// never decides anything (PROTOCOL amendment 1). The dropped pairs are
/// counted per direction so the bias is visible.
fn paired(x: &[&Row], y: &[&Row], name: &str, price: f64, seed: u64) -> Value {
    // Keyed by target only: the A/A pairs a target with its own sigma-conjugate.
    let ymap: HashMap<&str, &Row> = y.iter().map(|r| (r.target_id.as_str(), *r)).collect();
    let mut logs = Vec::new();
    let (mut sx, mut sy) = (0.0, 0.0);
    let (mut d_x_only, mut d_y_only, mut d_both, mut d_other, mut absent) =
        (0usize, 0usize, 0usize, 0usize, 0usize);
    for r in x {
        let Some(q) = ymap.get(r.target_id.as_str()) else {
            absent += 1;
            continue;
        };
        let (a, b) = (metric(r, name, price), metric(q, name, price));
        let (cx, cy) = (r.verdict == "censored", q.verdict == "censored");
        let ok = !cx && !cy && r.agree && q.agree;
        if !ok || !(a > 0.0) || !(b > 0.0) {
            match (cx, cy) {
                (true, false) => d_x_only += 1,
                (false, true) => d_y_only += 1,
                (true, true) => d_both += 1,
                _ => d_other += 1,
            }
            continue;
        }
        logs.push((a / b).ln());
        sx += a;
        sy += b;
    }
    let (m, lo, hi) = boot_mean(&logs, seed);
    json!({
        "metric": name, "price": price, "pairs": logs.len(),
        "dropped_x_censored_only": d_x_only, "dropped_y_censored_only": d_y_only,
        "dropped_both_censored": d_both, "dropped_other": d_other, "absent_from_other_arm": absent,
        "geo_ratio": m.exp(), "ci_lo": lo.exp(), "ci_hi": hi.exp(),
        "total_ratio": if sy > 0.0 { sx / sy } else { f64::NAN },
        "sd_log": sd(&logs),
        "mde_ratio_95": (1.96 * sd(&logs) / (logs.len() as f64).sqrt()).exp(),
    })
}

/// DECIDING statistic (PROTOCOL amendment 1): cost per relation of X over
/// cost per relation of Y on the same targets,
///   (sum over ALL targets of cost(X) / relations(X)) / (same for Y),
/// where a censored solve contributes its spend (a lower bound of its true
/// cost, so it flatters the censored arm; `decide` blocks any claim in favour
/// of the arm censored more) and contributes no relation. A relation is a
/// `found` solve in the satisfiable strata and a `refuted` solve in the
/// `unsat` stratum. No pair is dropped; the interval is a percentile bootstrap
/// over targets resampled jointly for both arms (2000 resamples).
fn per_relation(x: &[&Row], y: &[&Row], name: &str, price: f64, unsat: bool, seed: u64) -> Value {
    let ymap: HashMap<&str, &Row> = y.iter().map(|r| (r.target_id.as_str(), *r)).collect();
    let pairs: Vec<(&Row, &Row)> = x
        .iter()
        .filter_map(|r| ymap.get(r.target_id.as_str()).map(|q| (*r, *q)))
        .collect();
    let unit = |r: &Row| {
        if unsat {
            r.verdict == "refuted"
        } else {
            r.verdict == "found"
        }
    };
    let ratio = |idx: &[usize]| -> (f64, f64, f64, f64, f64) {
        let (mut cx, mut cy, mut ux, mut uy) = (0.0, 0.0, 0.0, 0.0);
        for &i in idx {
            let (a, b) = pairs[i];
            cx += metric(a, name, price);
            cy += metric(b, name, price);
            ux += unit(a) as u8 as f64;
            uy += unit(b) as u8 as f64;
        }
        let ok = ux > 0.0 && uy > 0.0 && cy > 0.0;
        (
            if ok { (cx / ux) / (cy / uy) } else { f64::NAN },
            cx,
            cy,
            ux,
            uy,
        )
    };
    let all: Vec<usize> = (0..pairs.len()).collect();
    let (point, cx, cy, ux, uy) = ratio(&all);
    let b = 2000usize;
    let mut rng = Rng(seed);
    let mut boots: Vec<f64> = Vec::with_capacity(b);
    for _ in 0..b {
        if pairs.is_empty() {
            break;
        }
        let idx: Vec<usize> = (0..pairs.len())
            .map(|_| rng.below(pairs.len() as u64) as usize)
            .collect();
        let v = ratio(&idx).0;
        if v.is_finite() && v > 0.0 {
            boots.push(v);
        }
    }
    // A resample without a relation in an arm has no ratio; if more than 5
    // percent of resamples lack one, the interval is declared undefined.
    let (lo, hi, sdl) = if point.is_finite() && boots.len() as f64 >= 0.95 * b as f64 {
        boots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let l: Vec<f64> = boots.iter().map(|v| v.ln()).collect();
        (
            boots[(0.025 * boots.len() as f64) as usize],
            boots[(0.975 * boots.len() as f64) as usize],
            sd(&l),
        )
    } else {
        (f64::NAN, f64::NAN, f64::NAN)
    };
    let cens = |f: &dyn Fn(&(&Row, &Row)) -> (bool, bool)| -> (usize, usize, usize) {
        let (mut xo, mut yo, mut bo) = (0, 0, 0);
        for p in &pairs {
            match f(p) {
                (true, false) => xo += 1,
                (false, true) => yo += 1,
                (true, true) => bo += 1,
                _ => {}
            }
        }
        (xo, yo, bo)
    };
    let (xo, yo, bo) = cens(&|(a, b)| (a.verdict == "censored", b.verdict == "censored"));
    json!({
        "metric": name, "price": price, "statistic": "cost_per_relation_ratio", "unit": if unsat { "refuted" } else { "found" },
        "targets": pairs.len(), "ratio": point, "ci_lo": lo, "ci_hi": hi,
        "relations_x": ux, "relations_y": uy, "cost_x": cx, "cost_y": cy,
        "cost_per_relation_x": if ux > 0.0 { cx / ux } else { f64::NAN },
        "cost_per_relation_y": if uy > 0.0 { cy / uy } else { f64::NAN },
        "censored_x_only": xo, "censored_y_only": yo, "censored_both": bo,
        "boot_sd_log": sdl, "mde_ratio_95": (1.96 * sdl).exp(),
    })
}

/// Minimum number of targets (shared by both arms) for any claim.
const MIN_TARGETS: usize = 10;
/// Per-arm censored fraction above which a stratum is unresolved.
const MAX_CENSORED: f64 = 0.2;

/// Facts about one (cell, stratum, arm X against P) comparison that gate it.
#[derive(Clone, Copy, Debug)]
struct Gate {
    /// every row of the cell (all arms, A/A included) agrees with the oracle
    agree_all: bool,
    /// every (arm, conj) job of the cell has the same target set
    complete: bool,
    targets: usize,
    cens_x: usize,
    cens_p: usize,
}

impl Gate {
    fn frac(&self, c: usize) -> f64 {
        if self.targets == 0 {
            1.0
        } else {
            c as f64 / self.targets as f64
        }
    }
}

/// Frozen decision rule (PROTOCOL section 7 as amended). The interval is that
/// of the cost-per-relation ratio X/P. Censoring is gated per arm, and a claim
/// in favour of the arm that was censored more is never made.
fn decide(ci_lo: f64, ci_hi: f64, aa_ci_lo: f64, aa_ci_hi: f64, g: &Gate) -> &'static str {
    if !g.agree_all {
        return "invalid_oracle_disagreement";
    }
    if !g.complete {
        return "invalid_incomplete_rows";
    }
    if g.targets < MIN_TARGETS {
        return "unresolved_too_few_targets";
    }
    if g.frac(g.cens_x) > MAX_CENSORED || g.frac(g.cens_p) > MAX_CENSORED {
        return "unresolved_censored";
    }
    if !(ci_lo.is_finite() && ci_hi.is_finite()) {
        return "unresolved_no_relations";
    }
    if !(aa_ci_lo <= 1.0 && 1.0 <= aa_ci_hi) {
        return "invalid_aa_excludes_one";
    }
    if ci_hi < 1.0 {
        if g.cens_x <= g.cens_p {
            "improvement"
        } else {
            "unresolved_asymmetric_censoring"
        }
    } else if ci_lo > 1.0 {
        if g.cens_p <= g.cens_x {
            "regression"
        } else {
            "unresolved_asymmetric_censoring"
        }
    } else {
        "unresolved_no_difference"
    }
}

/// Audit claim C3: the arm is at least 0.95x as fast as plain, i.e. its cost
/// per relation relative to plain is at most 1/0.95. Non-inferiority needs the
/// whole interval below the margin, a valid stratum, and the arm not censored
/// more than plain (censored spend understates cost).
const C3_COST_MARGIN: f64 = 1.0 / 0.95;

fn c3_noninferior(ci_hi: f64, decision: &str, g: &Gate) -> bool {
    let usable = !(decision.starts_with("invalid")
        || matches!(
            decision,
            "unresolved_censored" | "unresolved_too_few_targets" | "unresolved_no_relations"
        ));
    usable && g.cens_x <= g.cens_p && ci_hi <= C3_COST_MARGIN
}

/// One stratum's word_ops result for arm O, as the C3 grade consumes it.
#[derive(Clone, Debug)]
struct C3Entry {
    stratum: String,
    decision: String,
    ci_lo: f64,
    ci_hi: f64,
    /// `c3_noninferior` on the word_ops interval
    flag: bool,
    /// priced legs (see `c3_priced_ok`) all within the margin
    priced_ok: bool,
}

/// Priced-GAE legs that decide C3: only prices above 0. At price 0 the cost is
/// `lift_adds` alone, whose per-target noise (a depth-first search over 2^m sign
/// patterns, CV about 0.5, independent between arms) makes a 5 percent
/// non-inferiority bound unreachable even for identical encodings (PROTOCOL
/// section 11, Amendment 2). Price 0 stays in the 5a band for rankings.
/// `legs` is (price, ci_hi); both positive prices must be present.
fn c3_priced_ok(legs: &[(f64, f64)]) -> bool {
    let pos: Vec<&(f64, f64)> = legs.iter().filter(|l| l.0 > 0.0).collect();
    pos.len() >= 2 && pos.iter().all(|l| l.1 <= C3_COST_MARGIN)
}

/// Per-cell C3 grade for arm O. `supported`: natural_all and natural_sat both
/// non-inferior (upper bound <= 1/0.95, usable stratum, cens_O <= cens_P) and
/// the positive-price legs within the margin. `contradicted`: some stratum is a
/// `regression` (ci_lo > 1, which `decide` returns only for a valid, gated
/// stratum with cens_P <= cens_O) whose LOWER bound exceeds the margin, i.e.
/// O is dearer than 1/0.95 with 95 percent confidence. Otherwise `unresolved`.
fn c3_grade(entries: &[C3Entry]) -> &'static str {
    if entries.len() == 2 && entries.iter().all(|t| t.flag && t.priced_ok) {
        "supported"
    } else if entries
        .iter()
        .any(|t| t.decision == "regression" && t.ci_lo > C3_COST_MARGIN)
    {
        "contradicted"
    } else {
        "unresolved"
    }
}

/// Anytime view in one currency (solver word_ops). A row's finish is the
/// work at which its solve ended; it is a lower bound when censored. At a
/// budget B no larger than every censored finish, the outcome of a run at the
/// largest cap is exactly the outcome of a run capped at B (the solver is
/// deterministic and the cap only truncates; see `budget_only_truncates_the_search`).
fn anytime(rows: &[&Row], budget: f64) -> (usize, f64) {
    let found = rows
        .iter()
        .filter(|r| r.verdict == "found" && r.word_ops <= budget)
        .count();
    let cost = rows.iter().map(|r| r.word_ops.min(budget)).sum();
    (found, cost)
}

fn censor_horizon(rows: &[&Row]) -> f64 {
    rows.iter()
        .filter(|r| r.verdict == "censored")
        .map(|r| r.word_ops)
        .fold(f64::INFINITY, f64::min)
}

/// Yield and cost per relation on a geometric budget grid up to the common
/// horizon. `arms` are (name, rows) over the same targets.
fn anytime_table(arms: &[(&str, Vec<&Row>)]) -> Value {
    let horizon = arms
        .iter()
        .map(|(_, r)| censor_horizon(r))
        .fold(f64::INFINITY, f64::min);
    let top = arms
        .iter()
        .flat_map(|(_, r)| r.iter().map(|x| x.word_ops))
        .fold(0.0f64, f64::max)
        .min(horizon);
    let low = arms
        .iter()
        .flat_map(|(_, r)| r.iter().map(|x| x.word_ops))
        .filter(|w| *w > 0.0)
        .fold(f64::INFINITY, f64::min);
    if !(top.is_finite() && low.is_finite() && top > low) {
        return json!({"horizon_word_ops": horizon, "grid": []});
    }
    let steps = 8;
    let grid: Vec<Value> = (0..=steps)
        .map(|i| {
            let b = low * (top / low).powf(i as f64 / steps as f64);
            let per: Vec<Value> = arms
                .iter()
                .map(|(name, rows)| {
                    let (found, cost) = anytime(rows, b);
                    json!({"arm": name, "yield": found as f64 / rows.len() as f64, "found": found,
                           "cost": cost, "cost_per_relation": if found > 0 { cost / found as f64 } else { f64::INFINITY }})
                })
                .collect();
            json!({"budget_word_ops": b, "arms": per})
        })
        .collect();
    json!({"horizon_word_ops": horizon, "grid": grid})
}

fn hash_str(s: &str) -> u64 {
    s.bytes().fold(0x5EED_u64, |h, b| splitmix(h ^ b as u64))
}

/// (n, m, k) parsed from a cell id `a0-n17-m4-k2-s1027`.
fn parse_cell(id: &str) -> Option<(u64, u64, u64)> {
    let get = |p: char| {
        id.split('-')
            .find_map(|t| t.strip_prefix(p)?.parse::<u64>().ok())
    };
    Some((get('n')?, get('m')?, get('k')?))
}

/// Number of orbits of the k^m n^m tuples (k Frobenius orbits of length n,
/// m summands, each summand one of n rotations of a representative) under
/// simultaneous rotation, n prime: k^m n^(m-1). Information only (PROTOCOL
/// section 3); no cost in this cell is divided by it.
fn class_count(n: u64, m: u64, k: u64) -> f64 {
    (k as f64).powi(m as i32) * (n as f64).powi(m as i32 - 1)
}

fn analyze(dir: &str, out: &str) -> Result<(), String> {
    let mut rows: Vec<Row> = Vec::new();
    for e in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let path = e.map_err(|e| e.to_string())?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("jsonl")
            || !path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("rows-")
        {
            continue;
        }
        let text = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
            if v.get("tieout") == Some(&Value::Bool(false)) {
                return Err(format!("tie-out failure in {path:?}: shadow loop != production entry point; analysis refused"));
            }
            rows.push(parse_row(&v).ok_or_else(|| format!("bad row in {path:?}"))?);
        }
    }
    let mut cells: Vec<String> = rows.iter().map(|r| r.cell.clone()).collect();
    cells.sort();
    cells.dedup();
    let mut report = Vec::new();
    let mut c3_cells = Vec::new();
    for cell in &cells {
        // Cell-level integrity: any agree=false row of any arm (A/A included)
        // or any certificate that failed invalidates every stratum of the cell,
        // and every (arm, conj) job must cover the same target set.
        let crow: Vec<&Row> = rows.iter().filter(|r| &r.cell == cell).collect();
        let agree_cell = crow
            .iter()
            .all(|r| r.agree && (r.verdict != "found" || r.cert_ok));
        let mut groups: HashMap<(String, u32), HashSet<&str>> = HashMap::new();
        for r in &crow {
            groups
                .entry((r.arm.clone(), r.conj))
                .or_default()
                .insert(r.target_id.as_str());
        }
        let union: HashSet<&str> = groups.values().flatten().copied().collect();
        let complete_cell = ["O", "R", "Q"]
            .iter()
            .all(|a| groups.contains_key(&(a.to_string(), 0)))
            && groups.contains_key(&("P".to_string(), 0))
            && groups.contains_key(&("P".to_string(), 1))
            && groups.values().all(|g| g.len() == union.len());
        let sel = |arm: &str, conj0: bool, pred: &dyn Fn(&Row) -> bool| -> Vec<&Row> {
            rows.iter()
                .filter(|r| &r.cell == cell && r.arm == arm && (r.conj == 0) == conj0 && pred(r))
                .collect()
        };
        let strata: [(&str, Box<dyn Fn(&Row) -> bool>); 4] = [
            ("planted", Box::new(|r: &Row| r.class == "planted")),
            (
                "natural_sat",
                Box::new(|r: &Row| r.class == "natural" && r.oracle),
            ),
            (
                "unsat",
                Box::new(|r: &Row| r.class != "planted" && !r.oracle),
            ),
            ("natural_all", Box::new(|r: &Row| r.class == "natural")),
        ];
        let mut cell_out = Vec::new();
        // C3Entry per natural stratum for arm O, word_ops
        let mut o_c3: Vec<C3Entry> = Vec::new();
        for (sname, pred) in &strata {
            let unsat = *sname == "unsat";
            let seed = derive(&[hash_str(cell), hash_str(sname), 0xA11A]);
            let base = sel("P", true, pred.as_ref());
            if base.is_empty() {
                continue;
            }
            // A/A: plain on the sigma-conjugate target against plain on the target.
            let aa_rows: Vec<Row> = rows
                .iter()
                .filter(|r| &r.cell == cell && r.arm == "P" && r.conj > 0 && pred(r))
                .cloned()
                .collect();
            let aa_refs: Vec<&Row> = aa_rows.iter().collect();
            let mut arms = Vec::new();
            for arm in ["O", "R", "Q"] {
                let x = sel(arm, true, pred.as_ref());
                if x.is_empty() {
                    continue;
                }
                let ids_x: HashSet<&str> = x.iter().map(|r| r.target_id.as_str()).collect();
                let ids_p: HashSet<&str> = base.iter().map(|r| r.target_id.as_str()).collect();
                let gate = Gate {
                    agree_all: agree_cell,
                    complete: complete_cell && ids_x == ids_p,
                    targets: ids_p.intersection(&ids_x).count(),
                    cens_x: x.iter().filter(|r| r.verdict == "censored").count(),
                    cens_p: base.iter().filter(|r| r.verdict == "censored").count(),
                };
                let mut metrics = Vec::new();
                let mut word_ops_entry = None;
                for m in ["word_ops", "conflicts", "propagations", "wall_s"] {
                    let p = per_relation(&x, &base, m, 0.0, unsat, seed);
                    let aa = per_relation(&aa_refs, &base, m, 0.0, unsat, seed ^ 7);
                    let f = |v: &Value, k: &str| v[k].as_f64().unwrap_or(f64::NAN);
                    let verdict = decide(
                        f(&p, "ci_lo"),
                        f(&p, "ci_hi"),
                        f(&aa, "ci_lo"),
                        f(&aa, "ci_hi"),
                        &gate,
                    );
                    let c3 = c3_noninferior(f(&p, "ci_hi"), verdict, &gate);
                    if m == "word_ops" {
                        word_ops_entry = Some((verdict, f(&p, "ci_lo"), f(&p, "ci_hi"), c3));
                    }
                    metrics.push(json!({"vs_P": p, "aa_conj_vs_P": aa, "decision": verdict,
                        "c3_noninferior_at_0.95": c3,
                        "survivors_only_secondary": paired(&x, &base, m, 0.0, seed ^ 3)}));
                }
                // Section 5a pricing band: lift group adds + price * word_ops,
                // same cost-per-relation statistic at the three prices.
                let priced: Vec<Value> = [0.0, 0.1, 1.0]
                    .iter()
                    .map(|&price| per_relation(&x, &base, "gae", price, unsat, seed ^ 11))
                    .collect();
                let hi = |v: &Value| v["ci_hi"].as_f64().unwrap_or(f64::NAN);
                let lo = |v: &Value| v["ci_lo"].as_f64().unwrap_or(f64::NAN);
                // Section 5a: an order is ranked only if it holds at all three prices.
                let band = if priced.iter().all(|v| hi(v) < 1.0) {
                    "arm_cheaper_across_band"
                } else if priced.iter().all(|v| lo(v) > 1.0) {
                    "arm_dearer_across_band"
                } else {
                    "order_not_stable_across_band"
                };
                let c3_band = c3_priced_ok(
                    &[0.0f64, 0.1, 1.0]
                        .iter()
                        .zip(&priced)
                        .map(|(&p, v)| (p, hi(v)))
                        .collect::<Vec<_>>(),
                );
                // A/A resolution of each priced leg (plain on sigma(T) against plain on T).
                let aa_priced: Vec<Value> = [0.0, 0.1, 1.0]
                    .iter()
                    .map(|&price| per_relation(&aa_refs, &base, "gae", price, unsat, seed ^ 17))
                    .collect();
                // Final graded outcome: decision (word_ops) merged with the band.
                let (wd, wlo, whi, wc3) = word_ops_entry.unwrap();
                let final_grade = match (wd, band) {
                    ("improvement", "arm_cheaper_across_band") => "improvement_ranked",
                    ("regression", "arm_dearer_across_band") => "regression_ranked",
                    ("improvement" | "regression", _) => "not_ranked_unstable_across_5a_band",
                    (d, _) => d,
                };
                if arm == "O" && (*sname == "natural_all" || *sname == "natural_sat") {
                    o_c3.push(C3Entry {
                        stratum: sname.to_string(),
                        decision: wd.to_string(),
                        ci_lo: wlo,
                        ci_hi: whi,
                        flag: wc3,
                        priced_ok: c3_band,
                    });
                }
                arms.push(json!({"arm": arm, "gate": {"agree_all_cell": gate.agree_all, "complete": gate.complete,
                    "targets": gate.targets, "censored_x": gate.cens_x, "censored_P": gate.cens_p},
                    "metrics": metrics, "priced_gae": priced, "aa_priced_gae": aa_priced, "section_5a_band": band,
                    "c3_noninferior_at_positive_prices": c3_band, "final_grade_word_ops": final_grade}));
            }
            // Detector self-check: scale the A/A word_ops by a known ratio and
            // see whether the same cost-per-relation statistic detects it.
            let aa0 = per_relation(&aa_refs, &base, "word_ops", 0.0, unsat, seed ^ 13);
            let mut inject = Vec::new();
            for delta in [1.05f64, 1.1, 1.25, 1.5, 2.0] {
                let scaled: Vec<Row> = aa_rows
                    .iter()
                    .cloned()
                    .map(|mut r| {
                        r.word_ops *= delta;
                        r
                    })
                    .collect();
                let sr: Vec<&Row> = scaled.iter().collect();
                let v = per_relation(&sr, &base, "word_ops", 0.0, unsat, seed ^ delta.to_bits());
                let (lo, hi) = (
                    v["ci_lo"].as_f64().unwrap_or(f64::NAN),
                    v["ci_hi"].as_f64().unwrap_or(f64::NAN),
                );
                inject.push(json!({"injected": delta, "detected": lo > 1.0 || hi < 1.0}));
            }
            // Absolute cost per relation per arm and price, and the distance to
            // the matched rho (analytic placeholder, arm P).
            let (r0, k0) = base
                .first()
                .map(|r| (r.r, r.k))
                .unwrap_or((f64::NAN, f64::NAN));
            let rho = (std::f64::consts::PI * r0 / 2.0).sqrt();
            let mut per_rel = Vec::new();
            for arm in ["P", "O", "R", "Q"] {
                let ar = sel(arm, true, pred.as_ref());
                if ar.is_empty() {
                    continue;
                }
                let found = ar
                    .iter()
                    .filter(|r| {
                        if unsat {
                            r.verdict == "refuted"
                        } else {
                            r.verdict == "found"
                        }
                    })
                    .count();
                for price in [0.0f64, 0.1, 1.0] {
                    let total: f64 = ar.iter().map(|r| metric(r, "gae", price)).sum();
                    let cpr = if found > 0 {
                        total / found as f64
                    } else {
                        f64::INFINITY
                    };
                    per_rel.push(json!({
                        "arm": arm, "price": price, "relations": found, "targets": ar.len(),
                        "gae_per_relation": cpr,
                        "gap_to_rho_analytic": if arm == "P" && !unsat { json!((k0 + 1.0) * cpr / rho) } else { Value::Null },
                        "rho_analytic_adds": if arm == "P" { json!(rho) } else { Value::Null },
                    }));
                }
            }
            let any_arms: Vec<(&str, Vec<&Row>)> = ["P", "O", "R", "Q"]
                .iter()
                .map(|a| (*a, sel(a, true, pred.as_ref())))
                .filter(|(_, r)| !r.is_empty())
                .collect();
            let anytime_view = anytime_table(&any_arms);
            cell_out.push(json!({
                "anytime": anytime_view,
                "stratum": sname, "targets_P": base.len(),
                "aa_boot_sd_log_word_ops": aa0["boot_sd_log"],
                "aa_mde_ratio_95": aa0["mde_ratio_95"],
                "detector_injection": inject, "arms": arms, "cost_per_relation": per_rel,
            }));
        }
        // Per-cell C3 grade for arm O (see `c3_grade`).
        let grade = c3_grade(&o_c3);
        c3_cells.push(grade);
        let cc = parse_cell(cell).map(|(n, m, k)| class_count(n, m, k));
        report.push(json!({"cell": cell, "class_count_info_only": cc, "c3_grade_O": grade,
            "c3_detail": o_c3.iter().map(|t| json!({"stratum": t.stratum, "decision": t.decision, "ci_lo": t.ci_lo, "ci_hi": t.ci_hi, "flag": t.flag, "positive_price_legs_ok": t.priced_ok})).collect::<Vec<_>>(),
            "strata": cell_out}));
    }
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let supported = c3_cells.iter().filter(|g| **g == "supported").count();
    let text = serde_json::to_string_pretty(&json!({
        "schema": "g1-chain-sat-orbit/summary/v2",
        "c3_cells_supported": supported, "c3_cells_total": c3_cells.len(),
        "c3_k_of_9": format!("{supported} of {} analysed (9 planned)", c3_cells.len()),
        "cells": report}))
    .map_err(|e| e.to_string())?;
    std::fs::write(format!("{out}/summary.json"), text).map_err(|e| e.to_string())
}

// ------------------------------------------------------------------------ CLI

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

fn num<T: std::str::FromStr>(args: &[String], name: &str, default: Option<T>) -> T {
    match flag(args, name) {
        Some(v) => v.parse().unwrap_or_else(|_| panic!("bad value for {name}")),
        None => default.unwrap_or_else(|| panic!("missing {name}")),
    }
}

fn cell_from(args: &[String]) -> CellSpec {
    CellSpec {
        a: num(args, "--a", Some(0)),
        n: num(args, "--n", None),
        m: num(args, "--m", None),
        k: num(args, "--k", None),
        seed: num(args, "--seed", None),
    }
}

fn host() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::fs::read_to_string("/etc/hostname").map(|s| s.trim().to_string()))
        .unwrap_or_else(|_| "unknown".into())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first().cloned() else {
        eprintln!("usage: oracle|run|cap|analyze|plan (see RUN_CARD.md)");
        std::process::exit(2);
    };
    match cmd.as_str() {
        "plan" => {
            for n in [19u32, 23, 31] {
                let r = subgroup_r(n);
                for m in [4usize, 5, 6] {
                    let k = suggest_k(r, n, m);
                    println!(
                        "n={n} m={m} r={r} k={k} points={} yield~{:.4} class_count_info_only={:.4e}",
                        2 * n as usize * k,
                        expected_yield(r, n, m, k),
                        class_count(n as u64, m as u64, k as u64)
                    );
                }
            }
        }
        "oracle" => {
            let spec = cell_from(&args);
            let out: String = num(&args, "--out", None);
            let cd = build_cell(spec).unwrap_or_else(|e| panic!("{e}"));
            let targets = make_targets(
                &cd,
                num(&args, "--natural", Some(40)),
                num(&args, "--planted", Some(20)),
                num(&args, "--coset", Some(20)),
            );
            let t0 = Instant::now();
            let oracle = Oracle::new(&cd, num(&args, "--cap", Some(6_000_000)))
                .unwrap_or_else(|e| panic!("{e}"));
            std::fs::create_dir_all(&out).unwrap();
            std::fs::write(
                format!("{out}/manifest-{}.json", spec.id()),
                serde_json::to_string_pretty(&manifest_json(&cd)).unwrap(),
            )
            .unwrap();
            let mut f = std::fs::File::create(format!("{out}/oracle-{}.jsonl", spec.id())).unwrap();
            for t in &targets {
                let verdict = oracle.decide(&cd.kc, &t.point);
                let (x, y) = match &t.point {
                    BinaryPoint::Affine { x, y } => (code(x), code(y)),
                    BinaryPoint::Infinity => (0, 0),
                };
                writeln!(f, "{}", json!({"cell": spec.id(), "target_id": t.id, "class": t.class.name(),
                    "x": x, "y": y, "oracle": verdict,
                    "fb_points": cd.fb.points.len(), "r": cd.r, "elapsed_s": t0.elapsed().as_secs_f64()})).unwrap();
            }
        }
        "run" => {
            let spec = cell_from(&args);
            let arm_name: String = num(&args, "--arm", None);
            let arm = arm_spec(&arm_name).unwrap_or_else(|| panic!("arm must be P, O, R or Q"));
            let conj: u32 = num(&args, "--conj", Some(0));
            let cap: u64 = num(&args, "--conflicts", None);
            let tieout: usize = num(&args, "--tieout", Some(3));
            let out: String = num(&args, "--out", None);
            let oracle_path: String = num(&args, "--oracle", None);
            let run_id: String = num(&args, "--run-id", None);
            let cd = build_cell(spec).unwrap_or_else(|e| panic!("{e}"));
            let targets = make_targets(
                &cd,
                num(&args, "--natural", Some(40)),
                num(&args, "--planted", Some(20)),
                num(&args, "--coset", Some(20)),
            );
            let oracle: HashMap<String, bool> = std::fs::read_to_string(&oracle_path)
                .unwrap_or_else(|e| panic!("oracle file: {e}"))
                .lines()
                .filter_map(|l| serde_json::from_str::<Value>(l).ok())
                .filter_map(|v| {
                    Some((v["target_id"].as_str()?.to_string(), v["oracle"].as_bool()?))
                })
                .collect();
            let limits = limits_for(cap);
            std::fs::create_dir_all(&out).unwrap();
            let name = format!("rows-{}-{}-c{}.jsonl", spec.id(), arm_name, conj);
            let mut f = std::fs::File::create(format!("{out}/{name}")).unwrap();
            for (i, t) in targets.iter().enumerate() {
                let mut point = t.point.clone();
                for _ in 0..conj {
                    point = cd.kc.frobenius(&point);
                }
                let tie = if i < tieout && conj == 0 {
                    tie_out(&cd, &point, limits, arm)
                } else {
                    None
                };
                let o = shadow_solve(&cd, &point, limits, arm);
                let want = *oracle
                    .get(&t.id)
                    .unwrap_or_else(|| panic!("target {} missing from the oracle file", t.id));
                let cert_ok = o
                    .points
                    .as_ref()
                    .is_some_and(|p| verify_cert(&cd, &point, p));
                let agree = match o.verdict {
                    Verdict::Found => want && cert_ok,
                    Verdict::Refuted => !want,
                    Verdict::Censored => true,
                };
                writeln!(
                    f,
                    "{}",
                    row_json(&RowCtx {
                        run_id: &run_id,
                        cd: &cd,
                        id: &t.id,
                        class: t.class.name(),
                        arm: &arm_name,
                        conj,
                        cap,
                        point: &point,
                        o: &o,
                        want,
                        agree,
                        cert_ok,
                        tie
                    })
                )
                .unwrap();
                f.flush().unwrap();
            }
        }
        "cap" => {
            let rows: String = num(&args, "--rows", None);
            let budget: f64 = num(&args, "--budget", Some(180.0));
            for l in cap_lines(&rows, budget).unwrap_or_else(|e| panic!("{e}")) {
                println!("{l}");
            }
        }
        "analyze" => {
            let rows: String = num(&args, "--rows", None);
            let out: String = num(&args, "--out", None);
            analyze(&rows, &out).unwrap_or_else(|e| panic!("{e}"));
        }
        other => {
            eprintln!("unknown subcommand {other}");
            std::process::exit(2);
        }
    }
}

// ---------------------------------------------------------------------- tests

#[cfg(test)]
mod tests {
    use super::*;

    fn cell(n: u32, m: usize, k: usize) -> CellData {
        build_cell(CellSpec {
            a: 0,
            n,
            m,
            k,
            seed: 7,
        })
        .unwrap()
    }

    fn tiny_limits() -> ChainS3Limits {
        ChainS3Limits {
            max_variables: 5_000,
            max_domain_clauses: 100_000,
            max_models: 256,
            conflict_budget: 20_000,
        }
    }

    #[test]
    fn derivation_is_deterministic_and_separates_inputs() {
        assert_eq!(derive(&[1, 2, 3]), derive(&[1, 2, 3]));
        assert_ne!(derive(&[1, 2, 3]), derive(&[1, 2, 4]));
        assert_ne!(derive(&[1, 2]), derive(&[2, 1]));
        let mut r = Rng(5);
        assert!((0..1000).all(|_| r.below(29) < 29 && r.unit() < 1.0));
    }

    #[test]
    fn cell_has_k_full_orbits_and_targets_are_arm_independent() {
        let cd = cell(7, 4, 2);
        assert_eq!(cd.fb.signed_orbits.len(), 2);
        assert_eq!(cd.fb.points.len(), 28);
        assert_eq!(cd.xcodes.len(), 14);
        assert_eq!(cd.r, 29, "n=7, a=0: #E=116=4*29 from the Lucas recurrence");
        let t1 = make_targets(&cd, 5, 4, 3);
        let t2 = make_targets(&cell(7, 4, 2), 5, 4, 3);
        assert_eq!(t1.len(), 12);
        for (a, b) in t1.iter().zip(&t2) {
            assert_eq!((a.id.as_str(), &a.point), (b.id.as_str(), &b.point));
        }
        for t in &t1 {
            assert!(cd.kc.curve.is_on_curve(&t.point));
            let in_sub = cd.kc.mul(&t.point, &cd.kc.subgroup_order) == BinaryPoint::Infinity;
            assert_eq!(in_sub, t.class != Class::Coset, "{}", t.id);
        }
    }

    #[test]
    fn oracle_equals_brute_force_over_every_target() {
        // k = 1 keeps the brute force at 14^m tuples.
        for m in [4usize, 5] {
            let cd = cell(7, m, 1);
            let l = &cd.fb.points;
            let mut all: HashSet<(u64, u64)> = HashSet::new();
            let mut idx = vec![0usize; m];
            loop {
                let mut acc = BinaryPoint::Infinity;
                for &i in &idx {
                    acc = cd.kc.add(&acc, &l[i]);
                }
                all.insert(pkey(&acc));
                let mut d = 0;
                while d < m {
                    idx[d] += 1;
                    if idx[d] < l.len() {
                        break;
                    }
                    idx[d] = 0;
                    d += 1;
                }
                if d == m {
                    break;
                }
            }
            let oracle = Oracle::new(&cd, 1_000_000).unwrap();
            let mut seen_true = 0;
            let mut seen_false = 0;
            // every subgroup element, plus coset points
            for s in 1..cd.r {
                let t = cd.kc.mul(cd.kc.generator(), &BigUint::from(s));
                let want = all.contains(&pkey(&t));
                assert_eq!(oracle.decide(&cd.kc, &t), want, "m={m} s={s}");
                if want {
                    seen_true += 1
                } else {
                    seen_false += 1
                }
            }
            for t in make_targets(&cd, 0, 0, 6) {
                let want = all.contains(&pkey(&t.point));
                assert!(!want, "coset points are unreachable from the subgroup");
                assert_eq!(oracle.decide(&cd.kc, &t.point), want);
                seen_false += 1;
            }
            assert!(seen_true > 0 && seen_false > 0, "both verdicts exercised");
        }
        // Different splits of the same sumset agree: m=6 as 3+3 against 4+2.
        let cd = cell(7, 6, 1);
        let o33 = Oracle::new(&cd, 1_000_000).unwrap();
        let lv = sum_levels(&cd.kc, &cd.fb.points, 4, 1_000_000).unwrap();
        let s42: HashSet<(u64, u64)> = lv[1].iter().map(pkey).collect();
        for s in 1..cd.r {
            let t = cd.kc.mul(cd.kc.generator(), &BigUint::from(s));
            let by42 = lv[3]
                .iter()
                .any(|p| s42.contains(&pkey(&cd.kc.add(&t, &point_neg(p)))));
            assert_eq!(o33.decide(&cd.kc, &t), by42, "s={s}");
        }
    }

    #[test]
    fn shadow_loop_ties_out_to_production_entry_points_exactly() {
        for m in [4usize, 5, 6] {
            let cd = cell(7, m, 2);
            let ts = make_targets(&cd, 2, 2, 2);
            for arm in ["P", "O", "Q"] {
                for t in &ts {
                    let tie = tie_out(&cd, &t.point, tiny_limits(), arm_spec(arm).unwrap());
                    assert_eq!(tie, Some(true), "m={m} arm={arm} {}", t.id);
                }
            }
            assert_eq!(
                tie_out(&cd, &ts[0].point, tiny_limits(), arm_spec("R").unwrap()),
                None,
                "no production entry point for plain+ordered"
            );
        }
    }

    #[test]
    fn every_arm_agrees_with_the_oracle_and_certifies_found() {
        for m in [4usize, 5, 6] {
            let cd = cell(7, m, 2);
            let oracle = Oracle::new(&cd, 2_000_000).unwrap();
            let mut ts = make_targets(&cd, 4, 3, 3);
            let (mut sat, mut unsat, mut censored) = (0, 0, 0);
            for arm in ["P", "O", "R", "Q"] {
                for t in &mut ts {
                    let want = oracle.decide(&cd.kc, &t.point);
                    let o = shadow_solve(&cd, &t.point, tiny_limits(), arm_spec(arm).unwrap());
                    match o.verdict {
                        Verdict::Found => {
                            assert!(
                                want,
                                "m={m} arm={arm} {}: solver found, oracle says none",
                                t.id
                            );
                            assert!(verify_cert(&cd, &t.point, o.points.as_ref().unwrap()));
                            sat += 1;
                        }
                        Verdict::Refuted => {
                            assert!(
                                !want,
                                "m={m} arm={arm} {}: solver refuted a reachable target",
                                t.id
                            );
                            unsat += 1;
                        }
                        Verdict::Censored => {
                            // A censored solve is inconclusive, never a verdict.
                            assert_eq!(o.reason, "conflict_budget", "m={m} arm={arm} {}", t.id);
                            censored += 1;
                        }
                    }
                    if t.class == Class::Planted {
                        assert!(want, "a planted target is reachable by construction");
                    }
                }
            }
            assert!(
                sat > 0,
                "m={m}: found verdicts exercised ({censored} censored, {unsat} refuted)"
            );
        }
    }

    #[test]
    fn plain_arm_refutes_a_proven_negative_and_matches_the_oracle() {
        // The one UNSAT fixture: a coset target at n=7, m=4. Refutation is
        // expensive even here (hundreds of thousands of conflicts), which is
        // itself a design fact: PROTOCOL.md treats UNSAT cost as censored data.
        let cd = cell(7, 4, 2);
        let oracle = Oracle::new(&cd, 2_000_000).unwrap();
        let t = &make_targets(&cd, 0, 0, 1)[0];
        assert!(!oracle.decide(&cd.kc, &t.point));
        let mut l = tiny_limits();
        l.conflict_budget = 3_000_000;
        let o = shadow_solve(&cd, &t.point, l, arm_spec("P").unwrap());
        assert_eq!(o.verdict, Verdict::Refuted, "{:?}", o.reason);
        assert!(o.points.is_none());
        // A coset target has no x-level model: SAT alone refutes it, the lift never runs.
        assert_eq!(
            (o.c.models, o.c.lift_adds),
            (0, 0),
            "coset refutation must not reach the lift"
        );
        eprintln!(
            "UNSAT_FIXTURE conflicts={} wall={:.1}s",
            o.c.conflicts, o.wall_s
        );
    }

    #[test]
    fn budget_only_truncates_the_search() {
        // Anytime curves in the analysis rely on this: a smaller conflict cap
        // gives the same prefix of the same search, so one run at the largest
        // cap yields the outcome at every smaller cap.
        let cd = cell(7, 5, 2);
        for t in make_targets(&cd, 0, 3, 0) {
            for arm in ["P", "O"] {
                let big = shadow_solve(&cd, &t.point, tiny_limits(), arm_spec(arm).unwrap());
                if big.verdict != Verdict::Found || big.c.conflicts < 2 {
                    continue;
                }
                let mut l = tiny_limits();
                // The solver stops when conflicts reach the cap, so a run that
                // needed c conflicts to find a model needs a cap of c + 1.
                l.conflict_budget = big.c.conflicts + 1;
                let same = shadow_solve(&cd, &t.point, l, arm_spec(arm).unwrap());
                assert_eq!(same.verdict, Verdict::Found, "{arm} {}", t.id);
                assert_eq!(same.c.conflicts, big.c.conflicts);
                assert_eq!(same.c.word_ops, big.c.word_ops);
                l.conflict_budget = big.c.conflicts / 2; // well short
                let short = shadow_solve(&cd, &t.point, l, arm_spec(arm).unwrap());
                assert_ne!(short.verdict, Verdict::Found, "{arm} {}", t.id);
            }
        }
    }

    #[test]
    fn sigma_conjugate_target_has_the_same_verdict() {
        let cd = cell(7, 5, 2);
        for t in make_targets(&cd, 3, 2, 2) {
            let base = shadow_solve(&cd, &t.point, tiny_limits(), arm_spec("P").unwrap());
            let conj = cd.kc.frobenius(&t.point);
            assert!(cd.kc.curve.is_on_curve(&conj));
            let c = shadow_solve(&cd, &conj, tiny_limits(), arm_spec("P").unwrap());
            assert_eq!(base.verdict, c.verdict, "{}", t.id);
        }
    }

    #[test]
    fn solver_is_deterministic_so_a_rerun_is_not_an_a_a_arm() {
        let cd = cell(7, 5, 2);
        let t = &make_targets(&cd, 0, 1, 0)[0];
        let a = shadow_solve(&cd, &t.point, tiny_limits(), arm_spec("O").unwrap());
        let b = shadow_solve(&cd, &t.point, tiny_limits(), arm_spec("O").unwrap());
        assert_eq!(
            (a.c.conflicts, a.c.propagations, a.c.word_ops),
            (b.c.conflicts, b.c.propagations, b.c.word_ops)
        );
    }

    #[test]
    fn caps_censor_and_never_refute() {
        let cd = cell(7, 5, 2);
        let t = &make_targets(&cd, 0, 1, 0)[0];
        let mut l = tiny_limits();
        l.max_variables = 100;
        let o = shadow_solve(&cd, &t.point, l, arm_spec("P").unwrap());
        assert_eq!((o.verdict, o.reason), (Verdict::Censored, "var_cap"));
        let mut l = tiny_limits();
        l.max_domain_clauses = 1;
        let o = shadow_solve(&cd, &t.point, l, arm_spec("O").unwrap());
        assert_eq!(o.verdict, Verdict::Censored);
        // A coset target needs a proof of UNSAT; a zero-room budget must censor it.
        let coset = &make_targets(&cd, 0, 0, 1)[0];
        let mut l = tiny_limits();
        l.conflict_budget = 1;
        let o = shadow_solve(&cd, &coset.point, l, arm_spec("P").unwrap());
        assert_ne!(o.verdict, Verdict::Found);
    }

    #[test]
    fn certificate_checker_rejects_tampering() {
        let cd = cell(7, 4, 2);
        let t = &make_targets(&cd, 0, 1, 0)[0];
        let o = shadow_solve(&cd, &t.point, tiny_limits(), arm_spec("P").unwrap());
        let pts = o.points.unwrap();
        assert!(verify_cert(&cd, &t.point, &pts));
        let mut neg = pts.clone();
        neg[0] = point_neg(&neg[0]);
        let negated_sum = neg
            .iter()
            .fold(BinaryPoint::Infinity, |a, p| cd.kc.add(&a, p));
        assert_eq!(verify_cert(&cd, &t.point, &neg), negated_sum == t.point);
        assert_ne!(negated_sum, t.point, "negating one leg must move the sum");
        assert!(!verify_cert(&cd, &t.point, &pts[..3]));
        assert!(!verify_cert(
            &cd,
            &cd.kc.add(&t.point, cd.kc.generator()),
            &pts
        ));
        let outside = cd.kc.generator().clone();
        let outside_in_fb = match &outside {
            BinaryPoint::Affine { x, .. } => cd.xcodes.contains(&code(x)),
            _ => true,
        };
        if !outside_in_fb {
            let mut bad = pts.clone();
            bad[0] = outside;
            assert!(!verify_cert(&cd, &t.point, &bad));
        }
    }

    #[test]
    fn analysis_detects_a_known_ratio_and_passes_a_a() {
        let mut rng = Rng(99);
        let mk = |arm: &str, conj: u32, i: usize, w: f64| Row {
            cell: "c".into(),
            target_id: format!("t{i}"),
            class: "natural".into(),
            arm: arm.into(),
            conj,
            verdict: "refuted".into(),
            oracle: false,
            agree: true,
            cert_ok: false,
            conflicts: w,
            propagations: w,
            word_ops: w,
            wall_s: w,
            lift_adds: 0.0,
            r: 1e6,
            k: 2.0,
        };
        let (mut p, mut o, mut aa) = (vec![], vec![], vec![]);
        for i in 0..80 {
            let base = (1000.0 * (1.0 + 3.0 * rng.unit())).max(1.0);
            let noise = |rng: &mut Rng| (0.2 * (rng.unit() - 0.5)).exp();
            p.push(mk("P", 0, i, base));
            o.push(mk("O", 0, i, 0.5 * base * noise(&mut rng)));
            aa.push(mk("P", 1, i, base * noise(&mut rng)));
        }
        let (pr, or, ar): (Vec<&Row>, Vec<&Row>, Vec<&Row>) =
            (p.iter().collect(), o.iter().collect(), aa.iter().collect());
        let eff = per_relation(&or, &pr, "word_ops", 0.0, true, 1);
        let null = per_relation(&ar, &pr, "word_ops", 0.0, true, 2);
        let (lo, hi) = (
            eff["ci_lo"].as_f64().unwrap(),
            eff["ci_hi"].as_f64().unwrap(),
        );
        assert!(hi < 0.6 && lo > 0.4, "{eff}");
        let (nlo, nhi) = (
            null["ci_lo"].as_f64().unwrap(),
            null["ci_hi"].as_f64().unwrap(),
        );
        assert!(nlo <= 1.0 && 1.0 <= nhi, "{null}");
        let g = Gate {
            agree_all: true,
            complete: true,
            targets: 80,
            cens_x: 0,
            cens_p: 0,
        };
        assert_eq!(decide(lo, hi, nlo, nhi, &g), "improvement");
        assert_eq!(decide(1.2, 1.4, nlo, nhi, &g), "regression");
        assert_eq!(decide(0.9, 1.1, nlo, nhi, &g), "unresolved_no_difference");
        assert_eq!(
            decide(
                0.4,
                0.5,
                nlo,
                nhi,
                &Gate {
                    agree_all: false,
                    ..g
                }
            ),
            "invalid_oracle_disagreement"
        );
        assert_eq!(
            decide(
                0.4,
                0.5,
                nlo,
                nhi,
                &Gate {
                    complete: false,
                    ..g
                }
            ),
            "invalid_incomplete_rows"
        );
        assert_eq!(
            decide(0.4, 0.5, nlo, nhi, &Gate { targets: 5, ..g }),
            "unresolved_too_few_targets"
        );
        assert_eq!(
            decide(0.4, 0.5, nlo, nhi, &Gate { cens_x: 17, ..g }),
            "unresolved_censored"
        );
        assert_eq!(
            decide(0.4, 0.5, nlo, nhi, &Gate { cens_p: 17, ..g }),
            "unresolved_censored"
        );
        assert_eq!(
            decide(f64::NAN, f64::NAN, nlo, nhi, &g),
            "unresolved_no_relations"
        );
        assert_eq!(decide(0.4, 0.5, 1.1, 1.3, &g), "invalid_aa_excludes_one");
        // One-sided censoring below the 20 percent gate: a claim in favour of the arm
        // censored more is never made, in either direction.
        assert_eq!(
            decide(
                0.4,
                0.5,
                nlo,
                nhi,
                &Gate {
                    cens_x: 10,
                    cens_p: 2,
                    ..g
                }
            ),
            "unresolved_asymmetric_censoring"
        );
        assert_eq!(
            decide(
                1.2,
                1.4,
                nlo,
                nhi,
                &Gate {
                    cens_x: 2,
                    cens_p: 10,
                    ..g
                }
            ),
            "unresolved_asymmetric_censoring"
        );
        assert_eq!(
            decide(
                0.4,
                0.5,
                nlo,
                nhi,
                &Gate {
                    cens_x: 2,
                    cens_p: 10,
                    ..g
                }
            ),
            "improvement"
        );
        // C3 margin: a cost ratio of 1.05 is within 0.95x speed, 1.06 is not.
        assert!(c3_noninferior(1.05, "unresolved_no_difference", &g));
        assert!(
            !c3_noninferior(1.06, "unresolved_no_difference", &g)
                && !c3_noninferior(f64::NAN, "unresolved_no_difference", &g)
        );
        assert!(!c3_noninferior(0.9, "unresolved_censored", &g));
        assert!(!c3_noninferior(
            0.9,
            "unresolved_asymmetric_censoring",
            &Gate {
                cens_x: 10,
                cens_p: 2,
                ..g
            }
        ));
    }

    /// The failure the survivors-only statistic had: O censored on 30 percent of
    /// targets on exactly the expensive ones. Survivors-only calls O cheaper;
    /// the per-relation statistic charges the censored spend and does not.
    #[test]
    fn censoring_asymmetry_cannot_flatter_the_arm_censored_more() {
        let mk = |arm: &str, i: usize, verdict: &str, w: f64| Row {
            cell: "c".into(),
            target_id: format!("t{i}"),
            class: "planted".into(),
            arm: arm.into(),
            conj: 0,
            verdict: verdict.into(),
            oracle: true,
            agree: true,
            cert_ok: true,
            conflicts: w,
            propagations: w,
            word_ops: w,
            wall_s: w,
            lift_adds: 0.0,
            r: 1e6,
            k: 2.0,
        };
        let (mut p, mut o) = (vec![], vec![]);
        for i in 0..40 {
            let hard = i < 12; // 30 percent
            p.push(mk("P", i, "found", if hard { 1000.0 } else { 100.0 }));
            // O is 1.5x dearer everywhere it finishes but is censored (at cap 1000) on the hard targets.
            o.push(if hard {
                mk("O", i, "censored", 1000.0)
            } else {
                mk("O", i, "found", 150.0)
            });
        }
        let (pr, or): (Vec<&Row>, Vec<&Row>) = (p.iter().collect(), o.iter().collect());
        let surv = paired(&or, &pr, "word_ops", 0.0, 1);
        assert!(
            surv["ci_lo"].as_f64().unwrap() > 1.4,
            "survivors-only sees 1.5x: {surv}"
        );
        assert_eq!(surv["dropped_x_censored_only"].as_u64().unwrap(), 12);
        let cpr = per_relation(&or, &pr, "word_ops", 0.0, false, 1);
        // O: (12*1000 + 28*150) / 28 = 579; P: (12*1000 + 28*100) / 40 = 370.
        assert!(
            (cpr["ratio"].as_f64().unwrap() - (16200.0 / 28.0) / (14800.0 / 40.0)).abs() < 1e-9,
            "{cpr}"
        );
        assert_eq!(cpr["censored_x_only"].as_u64().unwrap(), 12);
        let g = Gate {
            agree_all: true,
            complete: true,
            targets: 40,
            cens_x: 12,
            cens_p: 0,
        };
        // 12/40 = 0.3 > 0.2: gated per arm even though the pooled fraction is 0.15.
        assert_eq!(decide(0.5, 0.9, 0.9, 1.1, &g), "unresolved_censored");
        // O finds nothing: no ratio, no claim.
        let none: Vec<Row> = (0..40).map(|i| mk("O", i, "censored", 1000.0)).collect();
        let nr: Vec<&Row> = none.iter().collect();
        assert!(per_relation(&nr, &pr, "word_ops", 0.0, false, 1)["ci_lo"].is_null());
    }

    #[test]
    fn anytime_curve_is_exact_below_the_horizon() {
        let mk = |t: &str, verdict: &str, w: f64| Row {
            cell: "c".into(),
            target_id: t.into(),
            class: "natural".into(),
            arm: "P".into(),
            conj: 0,
            verdict: verdict.into(),
            oracle: true,
            agree: true,
            cert_ok: true,
            conflicts: w,
            propagations: w,
            word_ops: w,
            wall_s: w,
            lift_adds: 0.0,
            r: 1.0,
            k: 2.0,
        };
        let rows = vec![
            mk("a", "found", 10.0),
            mk("b", "found", 100.0),
            mk("c", "refuted", 50.0),
            mk("d", "censored", 1000.0),
        ];
        let r: Vec<&Row> = rows.iter().collect();
        assert_eq!(censor_horizon(&r), 1000.0);
        assert_eq!(anytime(&r, 60.0), (1, 10.0 + 60.0 + 50.0 + 60.0));
        assert_eq!(anytime(&r, 500.0), (2, 10.0 + 100.0 + 50.0 + 500.0));
        let table = anytime_table(&[("P", r)]);
        assert_eq!(table["grid"].as_array().unwrap().len(), 9);
        assert_eq!(table["horizon_word_ops"].as_f64().unwrap(), 1000.0);
    }

    #[test]
    fn design_numbers_match_the_protocol_table() {
        // r is the order of the generator (the largest prime factor of #E, which must
        // exceed the cofactor), not 2^n and not #E/4: n = 31 has #E = 4 * 373 * 1439393
        // and r = 1439393. a = 0 has no usable subgroup at n = 17 (#E = 4 * 137 * 239,
        // r = 239 < h = 548, `KoblitzCurve::new` returns None), so n = 19 replaces it.
        assert!(KoblitzCurve::new(0, 17).is_none());
        let rs = [(19u32, 130_873u64), (23, 2_095_853), (31, 1_439_393)];
        let mut table = Vec::new();
        for &(n, r) in &rs {
            assert_eq!(subgroup_r(n), r, "n={n}");
            for m in [4usize, 5, 6] {
                let k = suggest_k(r, n, m);
                assert!(k >= 2);
                let y = expected_yield(r, n, m, k);
                assert!((0.0..=1.0).contains(&y));
                if k > 2 {
                    assert!(expected_yield(r, n, m, k - 1) < 1.0 - (-0.7f64).exp() + 1e-9);
                }
                table.push((n, m, k, (y * 1e4).round() / 1e4));
            }
        }
        // k = 2 (the minimum the orbit arm allows) already overshoots in 8 of 9 cells.
        assert_eq!(
            table,
            vec![
                (19, 4, 2, 1.0),
                (19, 5, 2, 1.0),
                (19, 6, 2, 1.0),
                (23, 4, 2, 0.7593),
                (23, 5, 2, 1.0),
                (23, 6, 2, 1.0),
                (31, 4, 2, 0.9989),
                (31, 5, 2, 1.0),
                (31, 6, 2, 1.0),
            ]
        );
    }

    #[test]
    fn every_planned_cell_builds_with_full_orbits() {
        for n in [19u32, 23, 31] {
            let cd = build_cell(CellSpec {
                a: 0,
                n,
                m: 4,
                k: 2,
                seed: 1027,
            })
            .unwrap();
            assert_eq!(cd.r, subgroup_r(n));
            assert_eq!(cd.fb.points.len(), 4 * n as usize);
            assert_eq!(cd.xcodes.len(), 2 * n as usize);
        }
    }

    fn synth_row(
        cell: &str,
        arm: &str,
        conj: u32,
        i: usize,
        class: &str,
        oracle: bool,
        verdict: &str,
        w: f64,
        conf: f64,
        build: f64,
    ) -> String {
        json!({"cell": cell, "target_id": format!("{cell}-{class}-{i}"), "class": class, "arm": arm, "conj": conj,
            "verdict": verdict, "oracle": oracle, "agree": true, "cert_ok": verdict == "found", "tieout": Value::Null,
            "conflicts": conf, "propagations": w, "word_ops": w, "wall_s": w * 1e-3 + build, "build_s": build,
            "lift_adds": 3.0 + ((i * 13 + arm.len() * 7 + conj as usize * 5 + arm.as_bytes()[0] as usize * 3) % 40) as f64,
            "r": 1e6, "k": 2.0}).to_string()
    }

    fn scratch(name: &str) -> String {
        let d = std::env::temp_dir().join(format!("g1test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d.to_string_lossy().into_owned()
    }

    #[test]
    fn analyze_end_to_end_on_synthetic_rows_grades_c3_and_censoring() {
        let dir = scratch("an");
        let cell = "a0-n19-m4-k2-s1";
        let mut rng = Rng(5);
        for (arm, conj, scale) in [
            ("P", 0u32, 1.0),
            ("O", 0, 1.0),
            ("R", 0, 0.9),
            ("Q", 0, 0.9),
            ("P", 1, 1.0),
        ] {
            let mut lines = vec![];
            for i in 0..30 {
                let base = 1000.0 * (1.0 + 3.0 * ((i * 7) % 11) as f64 / 11.0);
                let noise = (0.1 * (rng.unit() - 0.5)).exp();
                lines.push(synth_row(
                    cell,
                    arm,
                    conj,
                    i,
                    "natural",
                    true,
                    "found",
                    base * scale * noise,
                    100.0,
                    0.01,
                ));
            }
            for i in 0..8 {
                lines.push(synth_row(
                    cell, arm, conj, i, "coset", false, "censored", 5.0e5, 20000.0, 0.01,
                ));
            }
            std::fs::write(
                format!("{dir}/rows-{cell}-{arm}-c{conj}.jsonl"),
                lines.join("\n"),
            )
            .unwrap();
        }
        analyze(&dir, &format!("{dir}/out")).unwrap();
        let v: Value = serde_json::from_str(
            &std::fs::read_to_string(format!("{dir}/out/summary.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(v["schema"], "g1-chain-sat-orbit/summary/v2");
        let c = &v["cells"][0];
        assert_eq!(c["c3_grade_O"], "supported", "{c}");
        assert_eq!(v["c3_cells_supported"], 1);
        let strata = c["strata"].as_array().unwrap();
        let nat = strata
            .iter()
            .find(|s| s["stratum"] == "natural_all")
            .unwrap();
        assert_eq!(nat["arms"][0]["arm"], "O");
        assert_eq!(
            nat["arms"][0]["metrics"][0]["decision"],
            "unresolved_no_difference"
        );
        let uns = strata.iter().find(|s| s["stratum"] == "unsat").unwrap();
        assert_eq!(
            uns["arms"][0]["metrics"][0]["decision"],
            "unresolved_too_few_targets"
        );
        assert!(c["class_count_info_only"].as_f64().unwrap() > 0.0);
        // R and Q were made 10 percent cheaper in word_ops: the cost-per-relation statistic sees it.
        let r = nat["arms"]
            .as_array()
            .unwrap()
            .iter()
            .find(|a| a["arm"] == "R")
            .unwrap();
        assert_eq!(r["metrics"][0]["decision"], "improvement", "{r}");
        // ... but at price 0 only the lift additions are priced (identical here), so the
        // order is not stable across the 5a band and nothing is ranked.
        assert_eq!(
            r["final_grade_word_ops"], "not_ranked_unstable_across_5a_band",
            "{r}"
        );
        // A missing row (a job lost to a timeout) makes the cell invalid, not silently smaller.
        let lines: Vec<String> = std::fs::read_to_string(format!("{dir}/rows-{cell}-O-c0.jsonl"))
            .unwrap()
            .lines()
            .take(25)
            .map(String::from)
            .collect();
        std::fs::write(format!("{dir}/rows-{cell}-O-c0.jsonl"), lines.join("\n")).unwrap();
        analyze(&dir, &format!("{dir}/out2")).unwrap();
        let v2: Value = serde_json::from_str(
            &std::fs::read_to_string(format!("{dir}/out2/summary.json")).unwrap(),
        )
        .unwrap();
        let nat2 = v2["cells"][0]["strata"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["stratum"] == "natural_all")
            .unwrap()
            .clone();
        assert_eq!(
            nat2["arms"][0]["metrics"][0]["decision"],
            "invalid_incomplete_rows"
        );
        assert_eq!(v2["cells"][0]["c3_grade_O"], "unresolved");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn cap_rule_uses_ratio_of_sums_excluding_build_time_per_cell() {
        let dir = scratch("cap");
        // Cell A: fast solver, 1e-5 s per conflict; many tiny solves with big build time that a
        // median of wall/conflicts would blow up. Cell B: 1e-2 s per conflict (impediment).
        let mut a = vec![];
        let mut b = vec![];
        for i in 0..6 {
            a.push(json!({"cell": "a0-n19-m4-k2-s1", "arm": "P", "conj": 0, "conflicts": 5.0, "wall_s": 3.0, "build_s": 3.0}).to_string());
            let _ = i;
        }
        a.push(json!({"cell": "a0-n19-m4-k2-s1", "arm": "P", "conj": 0, "conflicts": 20000.0, "wall_s": 3.2, "build_s": 3.0}).to_string());
        b.push(json!({"cell": "a0-n31-m6-k2-s1", "arm": "O", "conj": 0, "conflicts": 20000.0, "wall_s": 201.0, "build_s": 1.0}).to_string());
        std::fs::write(format!("{dir}/rows-a.jsonl"), a.join("\n")).unwrap();
        std::fs::write(format!("{dir}/rows-b.jsonl"), b.join("\n")).unwrap();
        let lines = cap_lines(&dir, 180.0).unwrap();
        // A: t = 0.2/20030 = ~1e-5; b = 3 s; 2e6 * 1e-5 = 20 s, so the largest grid value.
        assert_eq!(
            lines,
            vec![
                "19:4:2:2000000".to_string(),
                "31:6:2:IMPEDIMENT".to_string()
            ]
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn entry(decision: &str, lo: f64, hi: f64, flag: bool, priced_ok: bool) -> C3Entry {
        C3Entry {
            stratum: "natural_all".into(),
            decision: decision.into(),
            ci_lo: lo,
            ci_hi: hi,
            flag,
            priced_ok,
        }
    }

    #[test]
    fn contradicted_needs_the_lower_bound_above_the_margin() {
        // O between 3% and 7% dearer: partly inside the 1/0.95 = 1.0526 margin, NOT contradicted.
        let near = [
            entry("regression", 1.03, 1.07, false, false),
            entry("regression", 1.03, 1.07, false, false),
        ];
        assert_eq!(c3_grade(&near), "unresolved");
        // O at least 6% dearer with 95% confidence: contradicted.
        let far = [
            entry("regression", 1.06, 1.12, false, false),
            entry("unresolved_no_difference", 0.9, 1.2, false, false),
        ];
        assert_eq!(c3_grade(&far), "contradicted");
        // A lower bound above the margin under a non-regression label (censoring gate) never contradicts.
        let gated = [
            entry("unresolved_asymmetric_censoring", 1.2, 1.4, false, false),
            entry("unresolved_censored", f64::NAN, f64::NAN, false, false),
        ];
        assert_eq!(c3_grade(&gated), "unresolved");
        let ok = [
            entry("unresolved_no_difference", 0.97, 1.04, true, true),
            entry("unresolved_no_difference", 0.98, 1.05, true, true),
        ];
        assert_eq!(c3_grade(&ok), "supported");
        // supported needs the positive-price legs too
        let leg = [
            entry("unresolved_no_difference", 0.97, 1.04, true, true),
            entry("unresolved_no_difference", 0.98, 1.05, true, false),
        ];
        assert_eq!(c3_grade(&leg), "unresolved");
    }

    #[test]
    fn c3_priced_legs_exclude_the_noisy_price_zero_lift_only_leg() {
        assert!(
            c3_priced_ok(&[(0.0, 1.30), (0.1, 1.02), (1.0, 1.01)]),
            "price 0 is lift_adds only and must not decide C3"
        );
        assert!(!c3_priced_ok(&[(0.0, 1.0), (0.1, 1.06), (1.0, 1.01)]));
        assert!(!c3_priced_ok(&[(0.0, 1.0), (0.1, f64::NAN), (1.0, 1.01)]));
        assert!(
            !c3_priced_ok(&[(0.0, 1.0), (1.0, 1.01)]),
            "both positive prices are required"
        );
    }

    #[test]
    fn identical_encodings_with_noisy_lift_still_grade_supported_and_report_priced_aa() {
        // word_ops equal between P and O, lift_adds random per target and arm (synth_row): the
        // price-0 interval is wide, but C3 rests on prices 0.1 and 1.
        let dir = scratch("nl");
        let cell = "a0-n19-m4-k2-s1";
        for (arm, conj) in [("P", 0u32), ("O", 0), ("R", 0), ("Q", 0), ("P", 1)] {
            let mut lines = vec![];
            for i in 0..40 {
                let w = 20000.0 * (1.0 + 3.0 * ((i * 7) % 11) as f64 / 11.0);
                lines.push(synth_row(
                    cell, arm, conj, i, "natural", true, "found", w, 100.0, 0.01,
                ));
            }
            std::fs::write(
                format!("{dir}/rows-{cell}-{arm}-c{conj}.jsonl"),
                lines.join("\n"),
            )
            .unwrap();
        }
        analyze(&dir, &format!("{dir}/out")).unwrap();
        let v: Value = serde_json::from_str(
            &std::fs::read_to_string(format!("{dir}/out/summary.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            v["cells"][0]["c3_grade_O"], "supported",
            "{}",
            v["cells"][0]
        );
        let nat = v["cells"][0]["strata"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["stratum"] == "natural_all")
            .unwrap();
        let o = &nat["arms"][0];
        assert_eq!(o["aa_priced_gae"].as_array().unwrap().len(), 3);
        assert!(o["aa_priced_gae"][0]["mde_ratio_95"].as_f64().unwrap() > 1.0);
        let hi0 = o["priced_gae"][0]["ci_hi"].as_f64().unwrap();
        assert!(
            hi0 > C3_COST_MARGIN,
            "price-0 leg is wide under random lift: {hi0}"
        );
        assert_eq!(o["c3_noninferior_at_positive_prices"], true);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn cap_lists_a_cell_without_conflict_rows_as_nodata() {
        let dir = scratch("nodata");
        let rows = [
            json!({"cell": "a0-n19-m4-k2-s1", "arm": "P", "conj": 0, "conflicts": 20000.0, "wall_s": 3.2, "build_s": 3.0}),
            json!({"cell": "a0-n23-m4-k2-s1", "arm": "P", "conj": 0, "conflicts": 0.0, "wall_s": 0.1, "build_s": 0.1}),
            json!({"cell": "a0-n23-m4-k2-s1", "arm": "O", "conj": 0, "conflicts": 0.0, "wall_s": 0.1, "build_s": 0.1}),
        ];
        let text: Vec<String> = rows.iter().map(|r| r.to_string()).collect();
        std::fs::write(format!("{dir}/rows-a.jsonl"), text.join("\n")).unwrap();
        assert_eq!(
            cap_lines(&dir, 180.0).unwrap(),
            vec!["19:4:2:2000000".to_string(), "23:4:2:NODATA".to_string()]
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn repo_file(rel: &str) -> String {
        format!(
            "{}/research/frobenius_msweep_20261027/g1-chain-sat-orbit/{rel}",
            env!("CARGO_MANIFEST_DIR")
        )
    }

    #[test]
    fn certificates_and_manifest_reach_disk_and_the_independent_native_verifier_accepts_them() {
        let cd = cell(7, 4, 2);
        let t = &make_targets(&cd, 0, 1, 0)[0];
        let arm = arm_spec("O").unwrap();
        let o = shadow_solve(&cd, &t.point, tiny_limits(), arm);
        assert!(matches!(o.verdict, Verdict::Found));
        let cert_ok = o
            .points
            .as_ref()
            .is_some_and(|p| verify_cert(&cd, &t.point, p));
        let row = row_json(&RowCtx {
            run_id: "t",
            cd: &cd,
            id: &t.id,
            class: t.class.name(),
            arm: "O",
            conj: 0,
            cap: 20_000,
            point: &t.point,
            o: &o,
            want: true,
            agree: cert_ok,
            cert_ok,
            tie: None,
        });
        assert_eq!(row["cert_points"].as_array().unwrap().len(), 4);
        assert!(row["target_xy"].is_array());
        let man = manifest_json(&cd);
        assert_eq!(
            man["factor_base_points"].as_array().unwrap().len(),
            cd.fb.points.len()
        );
        assert_eq!(man["subgroup_order_r"], cd.r);
        let dir = scratch("cert");
        let cell_id = cd.spec.id();
        std::fs::write(format!("{dir}/manifest-{cell_id}.json"), man.to_string()).unwrap();
        std::fs::write(format!("{dir}/rows-{cell_id}-O-c0.jsonl"), row.to_string()).unwrap();
        let vtarget = scratch("verify-target");
        let run = |d: &str| {
            std::process::Command::new("cargo")
                .args([
                    "run",
                    "--release",
                    "--offline",
                    "--quiet",
                    "--manifest-path",
                ])
                .arg(repo_file("verify_certs/Cargo.toml"))
                .args([
                    "--target-dir",
                    &vtarget,
                    "--",
                    "--rows",
                    d,
                    "--manifests",
                    &dir,
                ])
                .env_remove("CARGO_TARGET_DIR")
                .output()
                .expect("cargo is needed to build the independent verifier")
        };
        let good = run(&dir);
        assert!(
            good.status.success(),
            "{}",
            String::from_utf8_lossy(&good.stdout)
        );
        // Tamper: change one certificate point's y; the independent verifier must reject.
        let mut bad = row.clone();
        let y = bad["cert_points"][0][1].as_u64().unwrap();
        bad["cert_points"][0][1] = json!(y ^ 1);
        let bdir = format!("{dir}/bad");
        std::fs::create_dir_all(&bdir).unwrap();
        std::fs::write(format!("{bdir}/rows-{cell_id}-O-c0.jsonl"), bad.to_string()).unwrap();
        assert_eq!(run(&bdir).status.code(), Some(1));
        // A found row without certificate points is rejected too.
        let mut none = row.clone();
        none["cert_points"] = Value::Null;
        std::fs::write(
            format!("{bdir}/rows-{cell_id}-O-c0.jsonl"),
            none.to_string(),
        )
        .unwrap();
        assert_eq!(run(&bdir).status.code(), Some(1));
        let _ = std::fs::remove_dir_all(&dir);
        let _ = std::fs::remove_dir_all(&vtarget);
    }

    #[test]
    fn run_stage_rejects_an_empty_cell_list() {
        let out = std::process::Command::new("bash")
            .arg(repo_file("run_stage.sh"))
            .args(["t", "1", "0", "1", "1", "1", " "])
            .env("RUN", "test-empty")
            .output()
            .unwrap();
        assert_eq!(
            out.status.code(),
            Some(2),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(String::from_utf8_lossy(&out.stderr).contains("empty cell list"));
    }

    #[test]
    fn class_count_is_the_exact_free_rotation_orbit_count() {
        // n prime: simultaneous rotation acts freely on the k^m n^m tuples.
        assert_eq!(class_count(7, 3, 2), 8.0 * 49.0);
        assert_eq!(parse_cell("a0-n17-m4-k2-s1027"), Some((17, 4, 2)));
    }
}
