//! `gae-reference`: a tuned affine addition timed next to the addition that
//! defines the GAE cost unit.
//!
//! The unit of the index-calculus ledger is one affine group addition as
//! `ic_boundary::calibrate_group` times it: 200,000 chained
//! [`CountedGroup::add`] calls, which on a prime curve is `u128 %` and an
//! extended-Euclid inversion per addition and on a binary curve a table-or
//! fold multiplication and a Fermat-chain inversion per addition.  That is
//! the cost of a *naive* addition; a rho walker pays a fraction of it.  This
//! tool measures the fraction, so that the unit can be redefined as the
//! tuned step, and refuses to report a time for an addition it has not
//! checked.
//!
//! **Tuned step.**  `W` independent lanes (default 256), each
//! `P_i <- P_i + T[x_i mod 32]` from a 32-entry table of random multiples of
//! the generator: the shape of one r-adding rho step without the
//! distinguished-point test.  The `W` denominators share one inversion
//! (Montgomery's trick), which costs `3 + 1/W` multiplications a lane.
//!
//! * **Prime** (`p < 2^62`): the repository's Montgomery field
//!   [`prime_fast::Fp64`] (REDC on a 128-bit product, branch-free final
//!   subtraction), its `batch_inv` and `FastCurve::add_with_inv`.  Nothing
//!   is duplicated; this file owns the lane loop only.
//! * **Binary** (`n <= 62`, the limit of `koblitz_fast`): `koblitz_fast`'s
//!   `FastCurve::add_pairwise`, which folds carry-less products with
//!   `pclmulqdq` when [`Gf2::folds`] (the CPU has it and the modulus has a
//!   short tail) and otherwise reduces through byte tables, and inverts all
//!   lanes with one Fermat chain.
//!
//! A lane that reaches `O` (`P_i = -T`) restarts at `T[(i + 3) mod 32]`, as
//! `calibrate_group` restarts; a lane with `x_i = x_T` takes the ordinary
//! doubling or cancellation path.  Both are counted in the timed loop.
//!
//! **Correctness.**  Before any timing, at least `--verify` (default 10,000)
//! tuned additions are recomputed one by one with the repository's
//! [`CountedGroup`] on the same operands (canonical coordinates) and must
//! agree exactly, and every result must be on the curve.  A mismatch exits
//! with status 2.
//!
//! **Timing.**  One warm-up pass, then `--passes` (default 15) passes of
//! `--iters` steps over all lanes; ns per addition is the median pass's
//! wall time over `W x iters`.  The baseline is `calibrate_group` on the
//! same curve with 64 random subgroup points, median of three calls (each
//! call is the repository's one 200,000-addition measurement).
//!
//! ```text
//! gae-reference --prime P,A,B,GX,GY            decimal
//! gae-reference --binary N,MODULUS_HEX,A,B,GX,GY   hex; MODULUS includes z^N
//! gae-reference --from-calibration [--list]    every docs/ic/calibration.json
//!                                              instance and full_prices.json
//!                                              curve; --list resolves only
//! options: --lanes W  --passes N  --iters N  --verify N  --only SUBSTR
//! ```
//!
//! One JSON object per curve on stdout; skipped rows are reported on stderr.

#[path = "gae_reference/field.rs"]
mod field;
#[path = "gae_reference/tables.rs"]
mod tables;

use std::time::Instant;

use num_bigint::BigUint;
use num_traits::Num;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

use crypto_lib::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::curve_id;
use crypto_lib::cryptanalysis::ic_boundary::{
    calibrate_group, find_prime_order_curve, koblitz_instance, random_binary_instance,
    roster_prime_instance, BinaryGroup, BinaryInstance, BoundaryConfig, Calibration, CountedGroup,
    GroupOps, PrimeCurve, PrimePoint,
};
use crypto_lib::cryptanalysis::koblitz_fast::{BatchScratch, FastCurve as BinFast, FastPoint};
use crypto_lib::cryptanalysis::prime_fast::{
    FastCurve as PrimeFast, Pt, MAX_BITS as PRIME_MAX_BITS,
};

/// Entries in the random jump table.
const TABLE: usize = 32;

/// The docs table whose instance rows `--from-calibration` enumerates.
const CALIBRATION: &str = include_str!("../../docs/ic/calibration.json");
/// The ecbench price table whose curve rows it enumerates.
const FULL_PRICES: &str = include_str!("../cryptanalysis/ecbench/full_prices.json");

#[derive(Clone, Debug)]
struct Cfg {
    lanes: usize,
    passes: usize,
    iters: usize,
    verify: usize,
}

impl Default for Cfg {
    fn default() -> Self {
        Cfg {
            lanes: 256,
            passes: 15,
            iters: 1000,
            verify: 10_000,
        }
    }
}

/// A curve to measure, in the repository's own single-word types.
enum Spec {
    Prime {
        id: String,
        curve: PrimeCurve,
        g: (u64, u64),
        /// Subgroup order when known (random scalars are drawn below it).
        order: Option<u64>,
    },
    Binary {
        id: String,
        n: u32,
        fast: Box<BinFast>,
        g: FastPoint,
        order: Option<u64>,
    },
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).expect("finite timings"));
    v[v.len() / 2]
}

fn cpu_model() -> String {
    std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("model name"))
                .and_then(|l| l.split(':').nth(1))
                .map(|m| m.trim().to_string())
        })
        .unwrap_or_else(|| "unknown".into())
}

/// Median ns per addition over `passes` timed passes after one warm-up;
/// returns `(median, min, max)`.
fn time_passes(cfg: &Cfg, mut step: impl FnMut() -> usize) -> (f64, f64, f64) {
    let mut sink = 0usize;
    for _ in 0..cfg.iters {
        sink += step();
    }
    let adds = (cfg.lanes * cfg.iters) as f64;
    let mut ns = Vec::with_capacity(cfg.passes);
    for _ in 0..cfg.passes {
        let t = Instant::now();
        for _ in 0..cfg.iters {
            sink += step();
        }
        ns.push(t.elapsed().as_nanos() as f64 / adds);
    }
    std::hint::black_box(sink);
    let lo = ns.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = ns.iter().cloned().fold(0.0, f64::max);
    (median(ns), lo, hi)
}

/// The repository's baseline, `calibrate_group`, median of three calls.
fn baseline<G: CountedGroup>(g: &G, pts: &[G::Elt]) -> (f64, Vec<f64>) {
    let runs: Vec<f64> = (0..3)
        .map(|_| {
            let mut c = Calibration::default();
            calibrate_group(g, pts, &mut c);
            c.ns_per_add
        })
        .collect();
    (median(runs.clone()), runs)
}

// ── Prime ───────────────────────────────────────────────────────────────

/// `W` lanes of `P <- P + T[h(x)]` over the Montgomery field, one shared
/// inversion per step.
struct PrimeLanes<'a> {
    fc: &'a PrimeFast,
    table: Vec<Pt>,
    pts: Vec<Pt>,
    qs: Vec<Pt>,
    dens: Vec<u64>,
    scratch: Vec<u64>,
    /// Lanes whose denominator was zero this step, with their sum.
    exc: Vec<(usize, Option<Pt>)>,
}

impl PrimeLanes<'_> {
    /// One step of every lane; returns the number of exceptional lanes.
    fn step(&mut self) -> usize {
        let f = &self.fc.f;
        self.exc.clear();
        for i in 0..self.pts.len() {
            let p = self.pts[i];
            let q = self.table[(p.x as usize) % TABLE];
            self.qs[i] = q;
            let d = f.sub(q.x, p.x);
            self.dens[i] = if d == 0 {
                self.exc.push((i, self.fc.add(Some(p), Some(q))));
                1
            } else {
                d
            };
        }
        f.batch_inv(&mut self.dens, &mut self.scratch);
        for i in 0..self.pts.len() {
            self.pts[i] = self.fc.add_with_inv(self.pts[i], self.qs[i], self.dens[i]);
        }
        for &(i, r) in &self.exc {
            self.pts[i] = r.unwrap_or(self.table[(i + 3) % TABLE]);
        }
        self.exc.len()
    }
}

fn random_multiples<G: CountedGroup>(
    g: &G,
    base: G::Elt,
    count: usize,
    bound: u64,
    rng: &mut StdRng,
) -> Vec<G::Elt> {
    let mut ops = GroupOps::default();
    let mut out = Vec::with_capacity(count);
    while out.len() < count {
        let m = g.mul(&mut ops, base, rng.gen_range(1..bound));
        if !g.is_identity(&m) {
            out.push(m);
        }
    }
    out
}

fn run_prime(
    id: &str,
    curve: &PrimeCurve,
    g: (u64, u64),
    order: Option<u64>,
    cfg: &Cfg,
) -> Result<Value, String> {
    let p = curve.p;
    if p <= 3 || p & 1 == 0 || p >= (1u64 << PRIME_MAX_BITS) {
        return Err(format!("p = {p} is outside odd 3 < p < 2^{PRIME_MAX_BITS}"));
    }
    let g_pt = PrimePoint::affine(g.0, g.1);
    if g.0 >= p || g.1 >= p || !curve.is_on_curve(g_pt) {
        return Err(format!("generator ({}, {}) is not on the curve", g.0, g.1));
    }
    let bound = order.filter(|&r| r > 2).unwrap_or(p);
    let mut rng = StdRng::seed_from_u64(p ^ 0x6AE0);
    let table_ref = random_multiples(curve, g_pt, TABLE, bound, &mut rng);
    let lanes_ref = random_multiples(curve, g_pt, cfg.lanes, bound, &mut rng);
    let fc = PrimeFast::new(id, p, curve.a, curve.b, 1);
    let to_fast = |q: &PrimePoint| fc.point(q.x, q.y);
    let mut lanes = PrimeLanes {
        fc: &fc,
        table: table_ref.iter().map(to_fast).collect(),
        pts: lanes_ref.iter().map(to_fast).collect(),
        qs: vec![Pt { x: 0, y: 0 }; cfg.lanes],
        dens: vec![1; cfg.lanes],
        scratch: Vec::new(),
        exc: Vec::new(),
    };

    // Correctness: every lane step against `CountedGroup::add`.
    let rounds = cfg.verify.div_ceil(cfg.lanes).max(1);
    let mut ops = GroupOps::default();
    let mut verified = 0u64;
    let mut restarts = 0u64;
    for round in 0..rounds {
        let before = lanes.pts.clone();
        lanes.step();
        for i in 0..cfg.lanes {
            let (px, py) = fc.canonical(before[i]);
            let (qx, qy) = fc.canonical(lanes.qs[i]);
            let sum = curve.add(
                &mut ops,
                PrimePoint::affine(px, py),
                PrimePoint::affine(qx, qy),
            );
            let want = if sum.infinity {
                restarts += 1;
                fc.canonical(lanes.table[(i + 3) % TABLE])
            } else {
                (sum.x, sum.y)
            };
            let got = fc.canonical(lanes.pts[i]);
            if got != want {
                return Err(format!(
                    "MISMATCH {id} round {round} lane {i}: ({px},{py}) + ({qx},{qy}): tuned {got:?}, reference {want:?}"
                ));
            }
            if !curve.is_on_curve(PrimePoint::affine(got.0, got.1)) {
                return Err(format!("OFF-CURVE {id} round {round} lane {i}: {got:?}"));
            }
            verified += 1;
        }
    }

    let (tuned, lo, hi) = time_passes(cfg, || lanes.step());

    let pts = random_multiples(curve, g_pt, 64, bound, &mut rng);
    let (reference, runs) = baseline(curve, &pts);
    Ok(report(
        id,
        "prime",
        64 - p.leading_zeros(),
        reference,
        &runs,
        (tuned, lo, hi),
        cfg,
        verified,
        restarts,
        "montgomery-u64 REDC + batched inversion",
    ))
}

// ── Binary ──────────────────────────────────────────────────────────────

/// `W` lanes over `GF(2^n)` through `koblitz_fast`'s batched pairwise add.
struct BinLanes<'a> {
    fc: &'a BinFast,
    table: Vec<FastPoint>,
    pts: Vec<FastPoint>,
    qs: Vec<FastPoint>,
    out: Vec<FastPoint>,
    scratch: BatchScratch,
}

impl BinLanes<'_> {
    /// One step of every lane; returns the number of lanes that restarted.
    fn step(&mut self) -> usize {
        for (q, p) in self.qs.iter_mut().zip(&self.pts) {
            *q = self.table[(p.x as usize) % TABLE];
        }
        self.out.clear();
        self.fc
            .add_pairwise(&self.pts, &self.qs, &mut self.out, &mut self.scratch);
        let mut restarts = 0;
        for (i, o) in self.out.iter_mut().enumerate() {
            if o.infinity {
                *o = self.table[(i + 3) % TABLE];
                restarts += 1;
            }
        }
        std::mem::swap(&mut self.pts, &mut self.out);
        restarts
    }
}

fn run_binary(
    id: &str,
    n: u32,
    fc: &BinFast,
    g: FastPoint,
    order: Option<u64>,
    cfg: &Cfg,
) -> Result<Value, String> {
    if g.infinity || !fc.is_on_curve(g) {
        return Err("generator is not an affine point of the curve".into());
    }
    let group = BinaryGroup(fc);
    let bound = order.filter(|&r| r > 2).unwrap_or(1u64 << n.min(62));
    let mut rng = StdRng::seed_from_u64(((n as u64) << 40) ^ fc.field.irr ^ 0x6AE0);
    let table = random_multiples(&group, g, TABLE, bound, &mut rng);
    let start = random_multiples(&group, g, cfg.lanes, bound, &mut rng);
    let mut lanes = BinLanes {
        fc,
        table,
        pts: start,
        qs: vec![FastPoint::INFINITY; cfg.lanes],
        out: Vec::with_capacity(cfg.lanes),
        scratch: BatchScratch::default(),
    };

    let rounds = cfg.verify.div_ceil(cfg.lanes).max(1);
    let mut ops = GroupOps::default();
    let mut verified = 0u64;
    let mut restarts = 0u64;
    for round in 0..rounds {
        let before = lanes.pts.clone();
        lanes.step();
        for i in 0..cfg.lanes {
            let q = lanes.table[(before[i].x as usize) % TABLE];
            let sum = group.add(&mut ops, before[i], q);
            let want = if sum.infinity {
                restarts += 1;
                lanes.table[(i + 3) % TABLE]
            } else {
                sum
            };
            let got = lanes.pts[i];
            if got != want {
                return Err(format!(
                    "MISMATCH {id} round {round} lane {i}: {:?} + {:?}: tuned {got:?}, reference {want:?}",
                    before[i], q
                ));
            }
            if !fc.is_on_curve(got) {
                return Err(format!("OFF-CURVE {id} round {round} lane {i}: {got:?}"));
            }
            verified += 1;
        }
    }

    let (tuned, lo, hi) = time_passes(cfg, || lanes.step());

    let pts = random_multiples(&group, g, 64, bound, &mut rng);
    let (reference, runs) = baseline(&group, &pts);
    let kernel = match (fc.field.folds(), fc.field.kernel_name()) {
        (true, "pclmulqdq") => "pclmulqdq fold multiply + batched Fermat inversion",
        (true, "pmull") => "pmull fold multiply + batched Fermat inversion",
        _ => "portable table reduction + batched Fermat inversion",
    };
    Ok(report(
        id,
        "binary",
        n,
        reference,
        &runs,
        (tuned, lo, hi),
        cfg,
        verified,
        restarts,
        kernel,
    ))
}

#[allow(clippy::too_many_arguments)]
fn report(
    id: &str,
    family: &str,
    bits: u32,
    reference: f64,
    runs: &[f64],
    (tuned, lo, hi): (f64, f64, f64),
    cfg: &Cfg,
    verified: u64,
    restarts: u64,
    kernel: &str,
) -> Value {
    json!({
        "curve": id,
        "family": family,
        "field_bits": bits,
        "ns_per_add_reference": reference,
        "ns_per_add_reference_runs": runs,
        "ns_per_add_tuned": tuned,
        "ns_per_add_tuned_min": lo,
        "ns_per_add_tuned_max": hi,
        "ratio_reference_over_tuned": reference / tuned,
        "lanes": cfg.lanes,
        "passes": cfg.passes,
        "iters_per_pass": cfg.iters,
        "verified_additions": verified,
        "verified_restarts": restarts,
        "kernel": kernel,
        "reference": "ic_boundary::calibrate_group (CountedGroup::add), median of 3",
        "host_cpu_model": cpu_model(),
    })
}

fn run_spec(spec: &Spec, cfg: &Cfg) -> Result<Value, String> {
    match spec {
        Spec::Prime {
            id,
            curve,
            g,
            order,
        } => run_prime(id, curve, *g, *order, cfg),
        Spec::Binary {
            id,
            n,
            fast,
            g,
            order,
        } => run_binary(id, *n, fast, *g, *order, cfg),
    }
}

// ── Resolution ──────────────────────────────────────────────────────────

fn prime_spec(label: &str, inst: &crypto_lib::cryptanalysis::ic_boundary::PrimeInstance) -> Spec {
    Spec::Prime {
        id: format!("{label} [{}]", inst.name),
        curve: inst.curve.clone(),
        g: inst.generator,
        order: Some(inst.r),
    }
}

fn binary_spec(label: &str, inst: BinaryInstance) -> Spec {
    let slug = inst.curve_id().slug;
    Spec::Binary {
        id: format!("{label} [{slug}]"),
        n: inst.n,
        g: inst.generator,
        order: Some(inst.r),
        fast: Box::new(inst.fast),
    }
}

/// The instance a `docs/ic/calibration.json` key names, rebuilt by the
/// constructor the ledger used (`BoundaryConfig::default().seed`).
fn resolve_calibration(key: &str) -> Result<Spec, String> {
    resolve_calibration_inst(key).map(Inst::into_spec)
}

/// A resolved curve with its instance kept whole (the `field` command
/// needs the instance's field, subgroup order and calibrators, not only
/// the curve).
pub enum Inst {
    Prime(
        String,
        Box<crypto_lib::cryptanalysis::ic_boundary::PrimeInstance>,
    ),
    Binary(String, Box<BinaryInstance>),
}

impl Inst {
    fn into_spec(self) -> Spec {
        match self {
            Inst::Prime(label, inst) => prime_spec(&label, &inst),
            Inst::Binary(label, inst) => binary_spec(&label, *inst),
        }
    }
}

fn resolve_calibration_inst(key: &str) -> Result<Inst, String> {
    let seed = BoundaryConfig::default().seed;
    let num = |s: &str| s.parse::<u32>().map_err(|e| format!("{key}: {e}"));
    if let Some(rest) = key.strip_prefix("prime/bench-") {
        let bits = num(rest.trim_end_matches("bit"))?;
        let inst = roster_prime_instance(bits).ok_or(format!("{key}: no roster curve"))?;
        return Ok(Inst::Prime(key.to_string(), Box::new(inst)));
    }
    if let Some(rest) = key.strip_prefix("prime/generated-") {
        let (bits, tag) = rest.split_once("bit-").ok_or(format!("{key}: odd name"))?;
        let bits = num(bits)?;
        let inst = find_prime_order_curve(bits, seed);
        let want: u64 = tag.parse().map_err(|_| format!("{key}: odd tag"))?;
        return if inst.curve.p == want {
            Ok(Inst::Prime(key.to_string(), Box::new(inst)))
        } else {
            Err(format!(
                "{key}: find_prime_order_curve({bits}, ledger seed) gives p = {}, not {want}",
                inst.curve.p
            ))
        };
    }
    if let Some(rest) = key.strip_prefix("char2/random-binary-n") {
        let (n, tag) = rest.split_once("-b").ok_or(format!("{key}: odd name"))?;
        let n = num(n)?;
        let inst = random_binary_instance(n, seed, 8).ok_or(format!("{key}: no curve found"))?;
        return if format!("{:x}", inst.b) == tag {
            Ok(Inst::Binary(key.to_string(), Box::new(inst)))
        } else {
            Err(format!(
                "{key}: random_binary_instance gives b = {:x}, not {tag}",
                inst.b
            ))
        };
    }
    if let Some(rest) = key.strip_prefix("koblitz/K_") {
        let (a, rest) = rest
            .split_once(" / GF(2^")
            .ok_or(format!("{key}: odd name"))?;
        let a: u8 = a.parse().map_err(|_| format!("{key}: odd a"))?;
        let n = num(rest.trim_end_matches(')'))?;
        let inst = koblitz_instance(a, n).ok_or(format!("{key}: no usable subgroup"))?;
        return Ok(Inst::Binary(key.to_string(), Box::new(inst)));
    }
    Err(format!("{key}: no resolver for this row"))
}

/// The curve an ICV1 slug of `full_prices.json` names: a Koblitz curve of
/// that degree, or the ledger's random binary curve, whose rebuilt ICV1
/// identity equals the slug.
fn resolve_slug(slug: &str) -> Result<Spec, String> {
    resolve_slug_inst(slug).map(Inst::into_spec)
}

fn resolve_slug_inst(slug: &str) -> Result<Inst, String> {
    let n = curve_id::binary_degree(slug).ok_or(format!("{slug}: no degree in the slug"))?;
    let mut candidates: Vec<BinaryInstance> = Vec::new();
    candidates.extend(koblitz_instance(0, n));
    candidates.extend(koblitz_instance(1, n));
    if candidates.is_empty() || n <= 27 {
        candidates.extend(random_binary_instance(n, BoundaryConfig::default().seed, 8));
    }
    for inst in candidates {
        if curve_id::same_curve(&inst.curve_id().slug, slug) {
            return Ok(Inst::Binary(slug.to_string(), Box::new(inst)));
        }
    }
    Err(format!(
        "{slug}: neither K_0, K_1 nor the ledger's random curve over GF(2^{n}) has this identity"
    ))
}

fn calibration_rows() -> Vec<(String, bool)> {
    let mut rows = Vec::new();
    let cal: Value = serde_json::from_str(CALIBRATION).expect("calibration.json parses");
    for k in cal["instances"].as_object().expect("instances").keys() {
        rows.push((k.clone(), false));
    }
    let full: Value = serde_json::from_str(FULL_PRICES).expect("full_prices.json parses");
    for k in full["instances"].as_object().expect("instances").keys() {
        rows.push((k.clone(), true));
    }
    rows
}

// ── CLI ─────────────────────────────────────────────────────────────────

fn parse_prime(arg: &str) -> Result<Spec, String> {
    let v: Vec<&str> = arg.split(',').collect();
    if v.len() != 5 {
        return Err("--prime wants P,A,B,GX,GY (decimal)".into());
    }
    let n: Vec<u64> = v
        .iter()
        .map(|s| s.trim().parse::<u64>().map_err(|e| format!("{s}: {e}")))
        .collect::<Result<_, _>>()?;
    Ok(Spec::Prime {
        id: format!("prime-p{}", n[0]),
        curve: PrimeCurve {
            p: n[0],
            a: n[1],
            b: n[2],
        },
        g: (n[3], n[4]),
        order: None,
    })
}

/// A hex word.
fn hex_u64(s: &str) -> Result<u64, String> {
    u64::from_str_radix(s.trim().trim_start_matches("0x"), 16).map_err(|e| format!("{s}: {e}"))
}

fn parse_binary(arg: &str) -> Result<Spec, String> {
    let v: Vec<&str> = arg.split(',').collect();
    if v.len() != 6 {
        return Err("--binary wants N,MODULUS_HEX,A,B,GX,GY (hex)".into());
    }
    let n: u32 = v[0].trim().parse().map_err(|e| format!("{}: {e}", v[0]))?;
    let modulus = BigUint::from_str_radix(v[1].trim().trim_start_matches("0x"), 16)
        .map_err(|e| format!("modulus: {e}"))?;
    if modulus.bits() != u64::from(n) + 1 {
        return Err(format!(
            "modulus must have degree {n} (bit {n} set, none above)"
        ));
    }
    let low_terms: Vec<u32> = (0..n).filter(|&i| modulus.bit(u64::from(i))).collect();
    let irreducible = IrreduciblePoly {
        degree: n,
        low_terms,
    };
    let el = |s: &str| -> Result<F2mElement, String> {
        Ok(F2mElement::from_biguint(&BigUint::from(hex_u64(s)?), n))
    };
    let curve = BinaryCurve {
        m: n,
        irreducible,
        a: el(v[2])?,
        b: el(v[3])?,
        generator: BinaryPoint::Infinity,
        order: BigUint::from(1u8),
        cofactor: BigUint::from(1u8),
    };
    let fast = BinFast::new(&curve).ok_or(format!(
        "GF(2^{n}) is outside the single-word range (n <= {})",
        BinFast::MAX_DEGREE
    ))?;
    Ok(Spec::Binary {
        id: format!("binary-n{n}-{}", v[1].trim()),
        n,
        g: FastPoint::affine(hex_u64(v[4])?, hex_u64(v[5])?),
        fast: Box::new(fast),
        order: None,
    })
}

fn emit(spec: Result<Spec, String>, cfg: &Cfg, failed: &mut bool) {
    match spec.and_then(|s| run_spec(&s, cfg)) {
        Ok(v) => println!("{v}"),
        Err(e) if e.starts_with("MISMATCH") || e.starts_with("OFF-CURVE") => {
            eprintln!("FATAL: {e}");
            std::process::exit(2);
        }
        Err(e) => {
            eprintln!("error: {e}");
            *failed = true;
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("field") => std::process::exit(field::run(&args[1..])),
        Some("retable") => std::process::exit(tables::retable_cli(&args[1..])),
        Some("reprice-ledger") => std::process::exit(tables::reprice_cli(&args[1..])),
        _ => {}
    }
    let mut cfg = Cfg::default();
    let (mut prime, mut binary, mut from_cal, mut list) = (None, None, false, false);
    let mut only: Option<String> = None;
    let mut i = 0;
    let value = |i: &mut usize| -> String {
        *i += 1;
        args.get(*i).cloned().unwrap_or_else(|| {
            eprintln!("{} wants a value", args[*i - 1]);
            std::process::exit(64)
        })
    };
    while i < args.len() {
        let num = |s: String| -> usize { s.parse().expect("a number") };
        match args[i].as_str() {
            "--prime" => prime = Some(value(&mut i)),
            "--binary" => binary = Some(value(&mut i)),
            "--from-calibration" => from_cal = true,
            "--list" => list = true,
            "--only" => only = Some(value(&mut i)),
            "--lanes" => cfg.lanes = num(value(&mut i)).max(1),
            "--passes" => cfg.passes = num(value(&mut i)).max(1),
            "--iters" => cfg.iters = num(value(&mut i)).max(1),
            "--verify" => cfg.verify = num(value(&mut i)),
            other => {
                eprintln!("unknown argument {other}; see the module docs");
                std::process::exit(64);
            }
        }
        i += 1;
    }
    let mut failed = false;
    if let Some(a) = prime {
        emit(parse_prime(&a), &cfg, &mut failed);
    }
    if let Some(a) = binary {
        emit(parse_binary(&a), &cfg, &mut failed);
    }
    if from_cal {
        for (key, is_slug) in calibration_rows() {
            if only.as_ref().is_some_and(|o| !key.contains(o.as_str())) {
                continue;
            }
            let spec = if is_slug {
                resolve_slug(&key)
            } else {
                resolve_calibration(&key)
            };
            if list {
                match &spec {
                    Ok(Spec::Prime { id, .. }) | Ok(Spec::Binary { id, .. }) => {
                        println!(
                            "{} {key} -> {id}",
                            if is_slug {
                                "full_prices"
                            } else {
                                "calibration"
                            }
                        )
                    }
                    Err(e) => println!("SKIP {key}: {e}"),
                }
                continue;
            }
            match spec {
                Err(e) => eprintln!("skip {key}: {e}"),
                spec => emit(spec, &cfg, &mut failed),
            }
        }
    }
    if prime_or_binary_missing(&args) {
        eprintln!("nothing to do: give --prime, --binary or --from-calibration");
        std::process::exit(64);
    }
    if failed {
        std::process::exit(1);
    }
}

fn prime_or_binary_missing(args: &[String]) -> bool {
    !args
        .iter()
        .any(|a| a == "--prime" || a == "--binary" || a == "--from-calibration")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small() -> Cfg {
        Cfg {
            lanes: 32,
            passes: 2,
            iters: 20,
            verify: 3000,
        }
    }

    /// p = 53411, a = 4589, b = 186, G = (43982, 22687), order 53117 (prime):
    /// small enough that lanes collide, double and cancel inside the check.
    #[test]
    fn tuned_prime_add_matches_reference() {
        let spec = parse_prime("53411,4589,186,43982,22687").unwrap();
        let v = run_spec(&spec, &small()).expect("tuned matches reference");
        assert!(v["verified_additions"].as_u64().unwrap() >= 3000);
        assert!(v["ns_per_add_tuned"].as_f64().unwrap() > 0.0);
    }

    /// Exceptional lanes (equal abscissas) are exercised: a 2-lane, 1-entry
    /// effective table on a tiny curve hits `P = T` and `P = -T`.
    #[test]
    fn tuned_prime_add_handles_collisions() {
        let spec = parse_prime("53411,4589,186,43982,22687").unwrap();
        let cfg = Cfg {
            lanes: 512,
            verify: 60_000,
            ..small()
        };
        run_spec(&spec, &cfg).expect("collisions agree with the reference");
    }

    /// `K_0 / GF(2^13)` from the repository's own constructor.
    #[test]
    fn tuned_binary_add_matches_reference() {
        let inst = koblitz_instance(0, 13).expect("K_0 over GF(2^13)");
        let spec = binary_spec("K_0/13", inst);
        let v = run_spec(&spec, &small()).expect("tuned matches reference");
        assert!(v["verified_additions"].as_u64().unwrap() >= 3000);
    }

    /// The CLI path builds the same curve from its modulus: x^13+x^4+x^3+x+1.
    #[test]
    fn binary_cli_modulus_parses() {
        let inst = koblitz_instance(1, 11).expect("K_1 over GF(2^11)");
        let m = curve_id::modulus_integer(&inst.irreducible);
        let g = inst.generator;
        let arg = format!("11,{m:x},{:x},{:x},{:x},{:x}", inst.a, inst.b, g.x, g.y);
        let spec = parse_binary(&arg).unwrap();
        run_spec(&spec, &small()).expect("CLI-built curve verifies");
    }
}
