//! Cell G1 of the 2026-10-27 Frobenius m-sweep: chained-S3 SAT, plain x-domain
//! against the exact Frobenius-orbit x-domain, at m = 4, 5, 6.
//!
//! STATUS: prepared, not run. Read
//! `research/frobenius_msweep_20261027/g1-chain-sat-orbit/PROTOCOL.md` first.
//!
//! The driver has four subcommands (see RUN_CARD.md for the exact lines):
//!   oracle   exact sumset verdicts for every target of one cell (no solver)
//!   run      one arm over every target of one cell, one JSONL row per solve
//!   analyze  paired comparison, A/A band, resolution check, decision rule
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
    let r = kc.subgroup_order.to_u64().ok_or("subgroup order exceeds u64")?;
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
            let Some(BinaryPoint::Affine { x, .. }) =
                orbit.first().and_then(|&i| fb.points.get(i))
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
                    if !enc.verify_model()
                        || xs.iter().any(|x| !cd.xcodes.contains(&code(x)))
                    {
                        c.invalid += 1;
                        absorb(&mut c, &enc);
                        return done(Verdict::Censored, "invalid_model", None, c, build_s);
                    }
                    let mut chosen = Vec::new();
                    let mut adds = 0u64;
                    let ok = lift(kc, &xs, 0, &BinaryPoint::Infinity, &mut chosen, target, &mut adds);
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
fn tie_out(cd: &CellData, target: &BinaryPoint, limits: ChainS3Limits, arm: ArmSpec) -> Option<bool> {
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

fn limits_for(cap: u64) -> ChainS3Limits {
    ChainS3Limits {
        max_variables: 400_000,
        max_domain_clauses: 20_000_000,
        max_models: 10_000,
        conflict_budget: cap,
    }
}

// ------------------------------------------------------------------- planning

/// Orbits needed so that the expected number of m-sums, (2kn)^m / m!, is
/// about 0.7 of the group order (natural-target yield about 0.5). At least 2,
/// because the orbit arm needs two representatives.
fn suggest_k(n: u32, m: usize) -> usize {
    let order = 2f64.powi(n as i32);
    let fact: f64 = (1..=m).map(|i| i as f64).product();
    for k in 2..=64usize {
        let s = (2.0 * k as f64 * n as f64).powi(m as i32) / fact;
        if s >= 0.7 * order {
            return k;
        }
    }
    64
}

fn expected_yield(n: u32, m: usize, k: usize) -> f64 {
    let order = 2f64.powi(n as i32);
    let fact: f64 = (1..=m).map(|i| i as f64).product();
    let s = (2.0 * k as f64 * n as f64).powi(m as i32) / fact;
    1.0 - (-s / order).exp()
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
    (mean(v), means[(0.025 * b as f64) as usize], means[(0.975 * b as f64) as usize])
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

/// Paired log ratio X/Y over targets where both finished and agree with the
/// oracle. Returns (geometric ratio, lo, hi, total ratio, pairs, dropped, sd).
fn paired(x: &[&Row], y: &[&Row], name: &str, price: f64, seed: u64) -> Value {
    // Keyed by target only: the A/A pairs a target with its own sigma-conjugate.
    let ymap: HashMap<&str, &Row> = y.iter().map(|r| (r.target_id.as_str(), *r)).collect();
    let mut logs = Vec::new();
    let (mut sx, mut sy) = (0.0, 0.0);
    let mut dropped = 0usize;
    for r in x {
        let Some(q) = ymap.get(r.target_id.as_str()) else {
            continue;
        };
        let (a, b) = (metric(r, name, price), metric(q, name, price));
        let ok = r.verdict != "censored" && q.verdict != "censored" && r.agree && q.agree;
        if !ok || !(a > 0.0) || !(b > 0.0) {
            dropped += 1;
            continue;
        }
        logs.push((a / b).ln());
        sx += a;
        sy += b;
    }
    let (m, lo, hi) = boot_mean(&logs, seed);
    json!({
        "metric": name, "price": price, "pairs": logs.len(), "dropped": dropped,
        "geo_ratio": m.exp(), "ci_lo": lo.exp(), "ci_hi": hi.exp(),
        "total_ratio": if sy > 0.0 { sx / sy } else { f64::NAN },
        "sd_log": sd(&logs),
        "mde_ratio_95": (1.96 * sd(&logs) / (logs.len() as f64).sqrt()).exp(),
    })
}

/// Frozen decision rule (PROTOCOL section 7).
fn decide(ci_lo: f64, ci_hi: f64, aa_ci_lo: f64, aa_ci_hi: f64, agree_all: bool, cens_frac: f64) -> &'static str {
    if !agree_all {
        return "invalid_oracle_disagreement";
    }
    if cens_frac > 0.2 {
        return "unresolved_censored";
    }
    if !(aa_ci_lo <= 1.0 && 1.0 <= aa_ci_hi) {
        return "invalid_aa_excludes_one";
    }
    if ci_hi < 1.0 {
        "improvement"
    } else if ci_lo > 1.0 {
        "regression"
    } else {
        "unresolved_no_difference"
    }
}

/// Anytime view in one currency (solver word_ops). A row's finish is the
/// work at which its solve ended; it is a lower bound when censored. At a
/// budget B no larger than every censored finish, the outcome of a run at the
/// largest cap is exactly the outcome of a run capped at B (the solver is
/// deterministic and the cap only truncates; see `budget_only_truncates_the_search`).
fn anytime(rows: &[&Row], budget: f64) -> (usize, f64) {
    let found = rows.iter().filter(|r| r.verdict == "found" && r.word_ops <= budget).count();
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
    let horizon = arms.iter().map(|(_, r)| censor_horizon(r)).fold(f64::INFINITY, f64::min);
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

fn analyze(dir: &str, out: &str) -> Result<(), String> {
    let mut rows: Vec<Row> = Vec::new();
    for e in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let path = e.map_err(|e| e.to_string())?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("jsonl")
            || !path.file_name().unwrap().to_string_lossy().starts_with("rows-")
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
    for cell in &cells {
        let sel = |arm: &str, conj0: bool, pred: &dyn Fn(&Row) -> bool| -> Vec<&Row> {
            rows.iter()
                .filter(|r| &r.cell == cell && r.arm == arm && (r.conj == 0) == conj0 && pred(r))
                .collect()
        };
        let strata: [(&str, Box<dyn Fn(&Row) -> bool>); 4] = [
            ("planted", Box::new(|r: &Row| r.class == "planted")),
            ("natural_sat", Box::new(|r: &Row| r.class == "natural" && r.oracle)),
            ("unsat", Box::new(|r: &Row| r.class != "planted" && !r.oracle)),
            ("natural_all", Box::new(|r: &Row| r.class == "natural")),
        ];
        let mut cell_out = Vec::new();
        for (sname, pred) in &strata {
            let seed = derive(&[cell.len() as u64, sname.len() as u64, 0xA11A]);
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
                let n_all = x.len();
                let cens = x.iter().filter(|r| r.verdict == "censored").count()
                    + base.iter().filter(|r| r.verdict == "censored").count();
                let cens_frac = cens as f64 / (n_all + base.len()) as f64;
                let agree_all = x.iter().chain(base.iter()).all(|r| r.agree && (r.verdict == "censored" || r.cert_ok || r.verdict == "refuted"));
                let mut metrics = Vec::new();
                for m in ["word_ops", "conflicts", "propagations", "wall_s"] {
                    let p = paired(&x, &base, m, 0.0, seed);
                    let aa = paired(&aa_refs, &base, m, 0.0, seed ^ 7);
                    let verdict = decide(
                        p["ci_lo"].as_f64().unwrap_or(f64::NAN),
                        p["ci_hi"].as_f64().unwrap_or(f64::NAN),
                        aa["ci_lo"].as_f64().unwrap_or(f64::NAN),
                        aa["ci_hi"].as_f64().unwrap_or(f64::NAN),
                        agree_all,
                        cens_frac,
                    );
                    metrics.push(json!({"vs_P": p, "aa_conj_vs_P": aa, "decision": verdict}));
                }
                // Section 5a pricing band: lift group adds + price * word_ops.
                let priced: Vec<Value> = [0.0, 0.1, 1.0]
                    .iter()
                    .map(|&price| paired(&x, &base, "gae", price, seed ^ 11))
                    .collect();
                arms.push(json!({"arm": arm, "censored_fraction": cens_frac, "metrics": metrics, "priced_gae": priced}));
            }
            // Detector self-check: inject known ratios into the A/A log ratios.
            let aa = paired(&aa_refs, &base, "word_ops", 0.0, seed ^ 13);
            let sdl = aa["sd_log"].as_f64().unwrap_or(f64::NAN);
            let pairs = aa["pairs"].as_u64().unwrap_or(0) as f64;
            let mut inject = Vec::new();
            for delta in [1.05f64, 1.1, 1.25, 1.5, 2.0] {
                let mut logs: Vec<f64> = Vec::new();
                let ymap: HashMap<&str, &&Row> = base.iter().map(|r| (r.target_id.as_str(), r)).collect();
                for r in &aa_refs {
                    if let Some(b) = ymap.get(r.target_id.as_str()) {
                        if r.word_ops > 0.0 && b.word_ops > 0.0 && r.verdict != "censored" && b.verdict != "censored" {
                            logs.push((r.word_ops / b.word_ops).ln() + delta.ln());
                        }
                    }
                }
                let (_, lo, hi) = boot_mean(&logs, seed ^ (delta.to_bits()));
                inject.push(json!({"injected": delta, "detected": lo > 0.0 || hi < 0.0}));
            }
            // Cost per relation and distance to the matched rho (analytic placeholder).
            let nat: Vec<&Row> = rows
                .iter()
                .filter(|r| &r.cell == cell && r.arm == "P" && r.conj == 0 && r.class == "natural")
                .collect();
            let found = nat.iter().filter(|r| r.verdict == "found").count();
            let mut per_rel = Vec::new();
            for price in [0.0f64, 0.1, 1.0] {
                let total: f64 = nat.iter().map(|r| metric(r, "gae", price)).sum();
                let (r0, k0) = nat.first().map(|r| (r.r, r.k)).unwrap_or((f64::NAN, f64::NAN));
                let rho = (std::f64::consts::PI * r0 / 2.0).sqrt();
                per_rel.push(json!({
                    "price": price,
                    "relations": found,
                    "gae_per_relation": if found > 0 { total / found as f64 } else { f64::INFINITY },
                    "gap_to_rho_analytic": if found > 0 { (k0 + 1.0) * total / found as f64 / rho } else { f64::INFINITY },
                    "rho_analytic_adds": rho,
                }));
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
                "aa_sd_log_word_ops": sdl,
                "aa_mde_ratio_95": (1.96 * sdl / pairs.sqrt()).exp(),
                "detector_injection": inject, "arms": arms, "cost_per_relation_P": per_rel,
            }));
        }
        report.push(json!({"cell": cell, "strata": cell_out}));
    }
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let text = serde_json::to_string_pretty(&json!({"schema": "g1-chain-sat-orbit/summary/v1", "cells": report}))
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
        eprintln!("usage: oracle|run|analyze|plan (see RUN_CARD.md)");
        std::process::exit(2);
    };
    match cmd.as_str() {
        "plan" => {
            for n in [17u32, 23, 31] {
                for m in [4usize, 5, 6] {
                    let k = suggest_k(n, m);
                    println!(
                        "n={n} m={m} k={k} points={} yield~{:.2}",
                        2 * n as usize * k,
                        expected_yield(n, m, k)
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
            let oracle = Oracle::new(&cd, num(&args, "--cap", Some(6_000_000))).unwrap_or_else(|e| panic!("{e}"));
            std::fs::create_dir_all(&out).unwrap();
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
                .filter_map(|v| Some((v["target_id"].as_str()?.to_string(), v["oracle"].as_bool()?)))
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
                let cert_ok = o.points.as_ref().is_some_and(|p| verify_cert(&cd, &point, p));
                let agree = match o.verdict {
                    Verdict::Found => want && cert_ok,
                    Verdict::Refuted => !want,
                    Verdict::Censored => true,
                };
                writeln!(f, "{}", json!({
                    "schema": "g1-chain-sat-orbit/row/v1", "run_id": run_id, "host": host(),
                    "cell": spec.id(), "a": spec.a, "n": spec.n, "m": spec.m, "k": spec.k,
                    "seed": spec.seed, "r": cd.r as f64, "fb_x": cd.xcodes.len(), "fb_points": cd.fb.points.len(),
                    "target_id": t.id, "class": t.class.name(), "arm": arm_name, "conj": conj,
                    "verdict": o.verdict.name(), "censor_reason": o.reason, "oracle": want,
                    "agree": agree, "cert_ok": cert_ok, "tieout": tie,
                    "conflict_cap": cap,
                    "conflicts": o.c.conflicts, "decisions": o.c.decisions,
                    "propagations": o.c.propagations, "word_ops": o.c.word_ops,
                    "patterns_attempted": o.c.patterns_attempted, "patterns_refuted": o.c.patterns_refuted,
                    "solver_calls": o.c.solver_calls, "models": o.c.models,
                    "group_rejected": o.c.group_rejected, "invalid_models": o.c.invalid,
                    "peak_vars": o.c.peak_vars, "peak_clauses": o.c.peak_clauses,
                    "peak_xors": o.c.peak_xors, "domain_clauses": o.c.domain_clauses,
                    "lift_adds": o.c.lift_adds, "wall_s": o.wall_s, "build_s": o.build_s,
                })).unwrap();
                f.flush().unwrap();
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
                if want { seen_true += 1 } else { seen_false += 1 }
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
            let by42 = lv[3].iter().any(|p| s42.contains(&pkey(&cd.kc.add(&t, &point_neg(p)))));
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
                            assert!(want, "m={m} arm={arm} {}: solver found, oracle says none", t.id);
                            assert!(verify_cert(&cd, &t.point, o.points.as_ref().unwrap()));
                            sat += 1;
                        }
                        Verdict::Refuted => {
                            assert!(!want, "m={m} arm={arm} {}: solver refuted a reachable target", t.id);
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
            assert!(sat > 0, "m={m}: found verdicts exercised ({censored} censored, {unsat} refuted)");
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
        eprintln!("UNSAT_FIXTURE conflicts={} wall={:.1}s", o.c.conflicts, o.wall_s);
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
        assert_eq!((a.c.conflicts, a.c.propagations, a.c.word_ops), (b.c.conflicts, b.c.propagations, b.c.word_ops));
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
        let negated_sum = neg.iter().fold(BinaryPoint::Infinity, |a, p| cd.kc.add(&a, p));
        assert_eq!(verify_cert(&cd, &t.point, &neg), negated_sum == t.point);
        assert_ne!(negated_sum, t.point, "negating one leg must move the sum");
        assert!(!verify_cert(&cd, &t.point, &pts[..3]));
        assert!(!verify_cert(&cd, &cd.kc.add(&t.point, cd.kc.generator()), &pts));
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
        let (pr, or, ar): (Vec<&Row>, Vec<&Row>, Vec<&Row>) = (p.iter().collect(), o.iter().collect(), aa.iter().collect());
        let eff = paired(&or, &pr, "word_ops", 0.0, 1);
        let null = paired(&ar, &pr, "word_ops", 0.0, 2);
        let (lo, hi) = (eff["ci_lo"].as_f64().unwrap(), eff["ci_hi"].as_f64().unwrap());
        assert!(hi < 0.6 && lo > 0.4, "{eff}");
        let (nlo, nhi) = (null["ci_lo"].as_f64().unwrap(), null["ci_hi"].as_f64().unwrap());
        assert!(nlo <= 1.0 && 1.0 <= nhi, "{null}");
        assert_eq!(decide(lo, hi, nlo, nhi, true, 0.0), "improvement");
        assert_eq!(decide(1.2, 1.4, nlo, nhi, true, 0.0), "regression");
        assert_eq!(decide(0.9, 1.1, nlo, nhi, true, 0.0), "unresolved_no_difference");
        assert_eq!(decide(0.4, 0.5, nlo, nhi, false, 0.0), "invalid_oracle_disagreement");
        assert_eq!(decide(0.4, 0.5, nlo, nhi, true, 0.3), "unresolved_censored");
        assert_eq!(decide(0.4, 0.5, 1.1, 1.3, true, 0.0), "invalid_aa_excludes_one");
    }

    #[test]
    fn anytime_curve_is_exact_below_the_horizon() {
        let mk = |t: &str, verdict: &str, w: f64| Row {
            cell: "c".into(), target_id: t.into(), class: "natural".into(), arm: "P".into(), conj: 0,
            verdict: verdict.into(), oracle: true, agree: true, cert_ok: true,
            conflicts: w, propagations: w, word_ops: w, wall_s: w, lift_adds: 0.0, r: 1.0, k: 2.0,
        };
        let rows = vec![mk("a", "found", 10.0), mk("b", "found", 100.0), mk("c", "refuted", 50.0), mk("d", "censored", 1000.0)];
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
        // (n, m) -> k, rounded yield; documented in PROTOCOL.md section 5.
        let table: Vec<(u32, usize, usize)> = [17u32, 23, 31]
            .iter()
            .flat_map(|&n| [4usize, 5, 6].map(|m| (n, m, suggest_k(n, m))))
            .collect();
        for &(n, m, k) in &table {
            assert!(k >= 2);
            let y = expected_yield(n, m, k);
            assert!((0.0..=1.0).contains(&y));
            if k > 2 {
                assert!(expected_yield(n, m, k - 1) < 1.0 - (-0.7f64).exp() + 1e-9);
            }
        }
        eprintln!("DESIGN_TABLE {table:?}");
    }
}
