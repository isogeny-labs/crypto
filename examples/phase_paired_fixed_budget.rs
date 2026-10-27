//! Native paired fixed-budget comparison: Frobenius phase carried as solver
//! unknowns (arm B) against representatives only (arm A), on identical frozen
//! targets, every attempt charged, every relation verified on the curve.
//!
//! Protocol: research/phase_paired_native_20261026/PROTOCOL.md
//! Independent replay: examples/phase_paired_replay.rs
//!
//! Subcommands (see `usage()`): `gate`, `run`, `freeze`. Tables and the chart are
//! rendered by the replay example from verified receipts only.
//!
//! The solver is `crypto_lib::cryptanalysis::sat::Solver` (single thread,
//! native XOR rows, deterministic). It is not CryptoMiniSat; the contrast
//! measured here is within one solver. The solver arms read the manifest only.
//! They never open `witness_sealed.json`; only the checker (`gate --sealed`)
//! and the replay do.

use crypto_lib::binary_ecc::curve::{point_add, point_neg, BinaryCurve, BinaryPoint};
use crypto_lib::binary_ecc::f2m::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::koblitz_relation_solver::U64RankTracker;
use crypto_lib::cryptanalysis::sat::{SolveResult, Solver};
use crypto_lib::hash::sha256::sha256;
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::time::{Duration, Instant};

/// SHA-256 of the frozen legacy manifest, copied unchanged into
/// `fixtures/manifest.json`.
const FROZEN_MANIFEST_SHA: &str =
    "8cb7f7989e5ea35656b34e7ba8bf7104e0c081277b8dbbd6b0f7507369b01231";
const DEFAULT_MANIFEST: &str = "research/phase_paired_native_20261026/fixtures/manifest.json";
const SCHEMA: &str = "phase_paired_native/receipt/v1";
const THIS_SOURCE: &[u8] = include_bytes!("phase_paired_fixed_budget.rs");

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

fn hx(v: u64) -> String {
    format!("0x{v:x}")
}

fn unhex(s: &str) -> u64 {
    u64::from_str_radix(s.trim_start_matches("0x"), 16).expect("hex")
}

fn sha_hex(bytes: &[u8]) -> String {
    hex::encode(sha256(bytes))
}

fn cpu_ns() -> u64 {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: plain libc call writing into a local struct.
    unsafe {
        libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut ts);
    }
    ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64
}

fn maxrss_kb() -> u64 {
    // SAFETY: getrusage writes into a zeroed local struct.
    unsafe {
        let mut ru: libc::rusage = std::mem::zeroed();
        libc::getrusage(libc::RUSAGE_SELF, &mut ru);
        ru.ru_maxrss as u64
    }
}

fn loadavg() -> f64 {
    fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|s| s.split_whitespace().next().and_then(|v| v.parse().ok()))
        .unwrap_or(-1.0)
}

fn cpu_model() -> String {
    fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("model name"))
                .map(|l| l.split(':').nth(1).unwrap_or("").trim().to_string())
        })
        .unwrap_or_else(|| "unknown".into())
}

fn cmd_out(prog: &str, args: &[&str]) -> String {
    std::process::Command::new(prog)
        .args(args)
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

/// splitmix64: deterministic RNG for native freezing and phase seeds.
struct SplitMix(u64);

impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        // Rejection sampling keeps the draw exactly uniform.
        let lim = u64::MAX - (u64::MAX % n);
        loop {
            let v = self.next();
            if v < lim {
                return v % n;
            }
        }
    }
}

/// Solver phase seed of repetition `rep` (1-based). Same for both arms.
fn phase_seed(rep: u64) -> u64 {
    SplitMix(0x5048_4153_4550_4149 ^ rep).next() | 1
}

// ---------------------------------------------------------------------------
// GF(2^n), n < 63, polynomial basis (encoder tables, Frobenius, lifting)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Gf {
    n: u32,
    modulus: u64,
}

impl Gf {
    fn new(n: u32, modulus: u64) -> Gf {
        assert!(n < 63 && (modulus >> n) == 1, "modulus must have degree n");
        Gf { n, modulus }
    }
    fn mul(&self, mut a: u64, mut b: u64) -> u64 {
        let mut r = 0u64;
        while b != 0 {
            if b & 1 == 1 {
                r ^= a;
            }
            b >>= 1;
            a <<= 1;
            if (a >> self.n) & 1 == 1 {
                a ^= self.modulus;
            }
        }
        r
    }
    fn sq(&self, a: u64) -> u64 {
        self.mul(a, a)
    }
    fn frob_k(&self, mut a: u64, k: u32) -> u64 {
        for _ in 0..k {
            a = self.sq(a);
        }
        a
    }
    fn sqrt(&self, a: u64) -> u64 {
        self.frob_k(a, self.n - 1)
    }
    fn tr(&self, a: u64) -> u64 {
        let mut s = 0;
        let mut x = a;
        for _ in 0..self.n {
            s ^= x;
            x = self.sq(x);
        }
        assert!(s <= 1, "trace must lie in F_2");
        s
    }
    fn pow(&self, a: u64, mut e: u64) -> u64 {
        let (mut r, mut b) = (1u64, a);
        while e != 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.sq(b);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: u64) -> u64 {
        assert!(a != 0);
        self.pow(a, (1u64 << self.n) - 2)
    }
    /// Solves w^2 + w = c for odd n when Tr(c) = 0.
    fn half_trace(&self, c: u64) -> u64 {
        assert!(self.n % 2 == 1);
        let mut s = 0;
        let mut x = c;
        for i in 0..=(self.n - 1) / 2 {
            if i > 0 {
                x = self.sq(self.sq(x));
            }
            s ^= x;
        }
        s
    }
}

// ---------------------------------------------------------------------------
// Curve y^2 + xy = x^3 + 1 over GF(2^n): the crate's group law behind u64 pairs
// ---------------------------------------------------------------------------

type Pt = Option<(u64, u64)>;

struct Ec {
    n: u32,
    gf: Gf,
    curve: BinaryCurve,
}

impl Ec {
    fn new(n: u32, modulus: u64, r: u64, cofactor: u64) -> Ec {
        let gf = Gf::new(n, modulus);
        let low_terms: Vec<u32> = (0..n).filter(|b| (modulus >> b) & 1 == 1).collect();
        let curve = BinaryCurve {
            m: n,
            irreducible: IrreduciblePoly {
                degree: n,
                low_terms,
            },
            a: F2mElement::zero(n),
            b: F2mElement::one(n),
            generator: BinaryPoint::Infinity,
            order: BigUint::from(r),
            cofactor: BigUint::from(cofactor),
        };
        Ec { n, gf, curve }
    }
    fn fe(&self, x: u64) -> F2mElement {
        F2mElement::from_words(&[x], self.n)
    }
    fn bp(&self, p: Pt) -> BinaryPoint {
        match p {
            None => BinaryPoint::Infinity,
            Some((x, y)) => BinaryPoint::Affine {
                x: self.fe(x),
                y: self.fe(y),
            },
        }
    }
    fn back(bp: &BinaryPoint) -> Pt {
        match bp {
            BinaryPoint::Infinity => None,
            BinaryPoint::Affine { x, y } => Some((x.raw_bits()[0], y.raw_bits()[0])),
        }
    }
    fn on(&self, p: Pt) -> bool {
        self.curve.is_on_curve(&self.bp(p))
    }
    fn add(&self, p: Pt, q: Pt) -> Pt {
        Self::back(&point_add(&self.curve, &self.bp(p), &self.bp(q)))
    }
    fn neg(&self, p: Pt) -> Pt {
        Self::back(&point_neg(&self.bp(p)))
    }
    fn mul(&self, k: u64, p: Pt) -> Pt {
        let mut acc: Pt = None;
        for i in (0..64).rev() {
            acc = self.add(acc, acc);
            if (k >> i) & 1 == 1 {
                acc = self.add(acc, p);
            }
        }
        acc
    }
    fn frob(&self, p: Pt) -> Pt {
        p.map(|(x, y)| (self.gf.sq(x), self.gf.sq(y)))
    }
    fn frob_k(&self, p: Pt, k: u32) -> Pt {
        let mut q = p;
        for _ in 0..k {
            q = self.frob(q);
        }
        q
    }
    /// A point with abscissa `x` (the one with the smaller ordinate), if any.
    fn lift(&self, x: u64) -> Option<(u64, u64)> {
        if x == 0 {
            return Some((0, 1)); // b = 1: the 2-torsion point
        }
        let gf = &self.gf;
        // y = x w with w^2 + w = x + b / x^2   (a = 0, b = 1)
        let c = x ^ gf.inv(gf.sq(x));
        if gf.tr(c) != 0 {
            return None;
        }
        let w = gf.half_trace(c);
        let y0 = gf.mul(x, w);
        let y1 = y0 ^ x;
        let y = y0.min(y1);
        debug_assert!(self.on(Some((x, y))));
        Some((x, y))
    }
}

// ---------------------------------------------------------------------------
// Frozen instance: curve, factor space, representatives, targets
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Target {
    id: String,
    set: String,
    r: (u64, u64),
}

#[allow(dead_code)]
struct Instance {
    n: u32,
    ec: Ec,
    g: (u64, u64),
    r: u64,
    lam: u64,
    lam_pow: Vec<u64>,
    basis: Vec<u64>,
    reps: Vec<(u64, u64)>,
    rep_logs: Vec<u64>,
    /// x -> (representative index, Frobenius phase)
    orbit: HashMap<u64, (usize, u32)>,
    /// discrete logs of the non-identity points of H to the base g
    log: HashMap<(u64, u64), u64>,
    targets: Vec<Target>,
    manifest_sha: String,
}

impl Instance {
    fn from_manifest_text(text: &str) -> Result<Instance, String> {
        let m: Value = serde_json::from_str(text).map_err(|e| format!("manifest json: {e}"))?;
        let c = &m["curve"];
        let n = c["n"].as_u64().ok_or("curve.n")? as u32;
        let modulus = unhex(c["modulus"].as_str().ok_or("curve.modulus")?);
        let r = c["odd_subgroup_order"]
            .as_u64()
            .ok_or("odd_subgroup_order")?;
        let group_order = c["group_order"].as_u64().ok_or("group_order")?;
        if group_order % r != 0 {
            return Err("odd subgroup order does not divide the group order".into());
        }
        let ec = Ec::new(n, modulus, r, group_order / r);
        let gx = unhex(c["generator"][0].as_str().ok_or("generator x")?);
        let gy = unhex(c["generator"][1].as_str().ok_or("generator y")?);
        let g = (gx, gy);
        if !ec.on(Some(g)) || ec.mul(r, Some(g)).is_some() || ec.mul(1, Some(g)).is_none() {
            return Err("generator is not a point of order r".into());
        }
        // Exhaustive table of H = <g> using the crate group law.
        let mut log = HashMap::new();
        let mut p: Pt = None;
        for k in 0..r {
            if let Some(q) = p {
                if log.insert(q, k).is_some() {
                    return Err("generator order is smaller than r".into());
                }
            }
            p = ec.add(p, Some(g));
        }
        if p.is_some() || log.len() as u64 != r - 1 {
            return Err("<g> does not have exactly r elements".into());
        }
        let lam = *log
            .get(&ec.frob(Some(g)).ok_or("frob(g)")?)
            .ok_or("pi(g) is not in <g>")?;
        let t = c["trace_t"].as_i64().ok_or("trace_t")?;
        let rr = r as i128;
        let lhs = (lam as i128 * lam as i128 - t as i128 * lam as i128 + 2).rem_euclid(rr);
        if lhs != 0 {
            return Err("lambda^2 - t lambda + 2 != 0 mod r".into());
        }
        let mut lam_pow = vec![1u64];
        for _ in 1..n {
            lam_pow.push(lam_pow.last().unwrap() * lam % r);
        }
        let basis: Vec<u64> = m["factor_space"]["basis"]
            .as_array()
            .ok_or("basis")?
            .iter()
            .map(|v| unhex(v.as_str().unwrap()))
            .collect();
        let mut reps = Vec::new();
        for rep in m["factor_space"]["representatives"]
            .as_array()
            .ok_or("representatives")?
        {
            let x = unhex(rep["point"][0].as_str().ok_or("rep x")?);
            let y = unhex(rep["point"][1].as_str().ok_or("rep y")?);
            if unhex(rep["x"].as_str().unwrap()) != x || !ec.on(Some((x, y))) {
                return Err("representative off the curve".into());
            }
            if !log.contains_key(&(x, y)) {
                return Err("representative outside the odd subgroup".into());
            }
            reps.push((x, y));
        }
        let mut orbit = HashMap::new();
        for (j, &(v, _)) in reps.iter().enumerate() {
            let mut x = v;
            for k in 0..n {
                if orbit.insert(x, (j, k)).is_some() {
                    return Err("representatives share a Frobenius orbit".into());
                }
                x = ec.gf.sq(x);
            }
        }
        let rep_logs: Vec<u64> = reps.iter().map(|p| log[p]).collect();
        let mut targets = Vec::new();
        for key in ["a", "b", "c"] {
            let set = &m["target_sets"][key];
            for t in set["targets"].as_array().ok_or("targets")? {
                let rx = unhex(t["R"][0].as_str().unwrap());
                let ry = unhex(t["R"][1].as_str().unwrap());
                if !ec.on(Some((rx, ry))) || !log.contains_key(&(rx, ry)) {
                    return Err("target outside the odd subgroup".into());
                }
                targets.push(Target {
                    id: t["id"].as_str().unwrap().to_string(),
                    set: key.to_string(),
                    r: (rx, ry),
                });
            }
        }
        Ok(Instance {
            n,
            ec,
            g,
            r,
            lam,
            lam_pow,
            basis,
            reps,
            rep_logs,
            orbit,
            log,
            targets,
            manifest_sha: sha_hex(text.as_bytes()),
        })
    }

    fn load(path: &str, expect_sha: Option<&str>) -> Result<Instance, String> {
        let text = fs::read_to_string(path).map_err(|e| format!("read {path}: {e}"))?;
        let inst = Instance::from_manifest_text(&text)?;
        if let Some(want) = expect_sha {
            if inst.manifest_sha != want {
                return Err(format!(
                    "{path}: manifest SHA-256 {} differs from pinned {want}",
                    inst.manifest_sha
                ));
            }
        }
        Ok(inst)
    }

    /// pi^k(V_j)
    fn phase_point(&self, rep: usize, k: u32) -> (u64, u64) {
        self.ec.frob_k(Some(self.reps[rep]), k).unwrap()
    }
}

// ---------------------------------------------------------------------------
// Relations: signed points -> (representative, phase, sign) -> row mod r
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Term {
    x: u64,
    rep: usize,
    phase: u32,
    sign: i8,
}

#[derive(Clone, Debug)]
struct Relation {
    terms: Vec<Term>,
    coeffs: Vec<u64>,
}

fn relation_from_points(inst: &Instance, pts: &[(u64, u64)]) -> Result<Relation, String> {
    let r = inst.r;
    let mut coeffs = vec![0u64; inst.reps.len()];
    let mut terms = Vec::new();
    for &p in pts {
        let &(j, k) = inst
            .orbit
            .get(&p.0)
            .ok_or_else(|| format!("x={} is in no representative's orbit", hx(p.0)))?;
        let base = inst.phase_point(j, k);
        let sign: i8 = if p == base {
            1
        } else if Some(p) == inst.ec.neg(Some(base)) {
            -1
        } else {
            return Err(format!("point at x={} is not +-pi^{k}(V_{j})", hx(p.0)));
        };
        let c = inst.lam_pow[k as usize];
        coeffs[j] = (coeffs[j] + if sign == 1 { c } else { r - c }) % r;
        terms.push(Term {
            x: p.0,
            rep: j,
            phase: k,
            sign,
        });
    }
    Ok(Relation { terms, coeffs })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Checks {
    terms_on_curve: bool,
    curve_sum: bool,
    coeffs_consistent: bool,
    lambda_log: bool,
}

impl Checks {
    fn all(&self) -> bool {
        self.terms_on_curve && self.curve_sum && self.coeffs_consistent && self.lambda_log
    }
}

fn check_relation(inst: &Instance, rel: &Relation, target: (u64, u64)) -> Checks {
    let r = inst.r;
    let mut sum: Pt = None;
    let mut on = true;
    let mut co = vec![0u64; inst.reps.len()];
    for t in &rel.terms {
        if t.rep >= inst.reps.len() || t.phase >= inst.n || !(t.sign == 1 || t.sign == -1) {
            on = false;
            continue;
        }
        let base = Some(inst.phase_point(t.rep, t.phase));
        let q = if t.sign == 1 { base } else { inst.ec.neg(base) };
        on &= inst.ec.on(q) && q.map(|p| p.0) == Some(t.x);
        sum = inst.ec.add(sum, q);
        let c = inst.lam_pow[t.phase as usize];
        co[t.rep] = (co[t.rep] + if t.sign == 1 { c } else { r - c }) % r;
    }
    let consistent =
        rel.coeffs.len() == co.len() && co.iter().zip(&rel.coeffs).all(|(a, b)| *a == b % r);
    let lhs: u128 = rel
        .coeffs
        .iter()
        .zip(&inst.rep_logs)
        .map(|(c, l)| (*c as u128 % r as u128) * *l as u128)
        .sum::<u128>()
        % r as u128;
    let rhs = *inst.log.get(&target).unwrap_or(&u64::MAX) as u128;
    Checks {
        terms_on_curve: on,
        curve_sum: sum == Some(target),
        coeffs_consistent: consistent,
        lambda_log: lhs == rhs,
    }
}

/// Legacy `encode.verify`: four distinct abscissae of rational points of H
/// with some signed sum equal to the target; returns the signed points.
fn verify_xs(inst: &Instance, xs: &[u64], target: (u64, u64)) -> Option<Vec<(u64, u64)>> {
    let mut seen = BTreeSet::new();
    if xs.iter().any(|x| !seen.insert(*x)) {
        return None;
    }
    let mut pts = Vec::new();
    for &x in xs {
        let p = inst.ec.lift(x)?;
        if x == 0 || !inst.log.contains_key(&p) {
            return None;
        }
        pts.push(Some(p));
    }
    for signs in 0u32..(1 << xs.len()) {
        let mut sum: Pt = None;
        let mut signed = Vec::new();
        for (i, p) in pts.iter().enumerate() {
            let q = if (signs >> i) & 1 == 1 {
                inst.ec.neg(*p)
            } else {
                *p
            };
            signed.push(q.unwrap());
            sum = inst.ec.add(sum, q);
        }
        if sum == Some(target) {
            return Some(signed);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Boolean encoder: gate-level, on sat::Solver (Tseitin AND + native XOR)
// ---------------------------------------------------------------------------

#[derive(Clone, Default, Debug)]
struct Lin {
    v: Vec<u32>,
    c: bool,
}

impl Lin {
    fn var(x: u32) -> Lin {
        Lin {
            v: vec![x],
            c: false,
        }
    }
    fn add(&self, o: &Lin) -> Lin {
        let (mut i, mut j) = (0, 0);
        let mut v = Vec::with_capacity(self.v.len() + o.v.len());
        while i < self.v.len() && j < o.v.len() {
            match self.v[i].cmp(&o.v[j]) {
                std::cmp::Ordering::Less => {
                    v.push(self.v[i]);
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    v.push(o.v[j]);
                    j += 1;
                }
                std::cmp::Ordering::Equal => {
                    i += 1;
                    j += 1;
                }
            }
        }
        v.extend_from_slice(&self.v[i..]);
        v.extend_from_slice(&o.v[j..]);
        Lin { v, c: self.c ^ o.c }
    }
}

fn lsum<'a>(forms: impl Iterator<Item = &'a Lin>) -> Lin {
    let mut acc = Lin::default();
    for f in forms {
        acc = acc.add(f);
    }
    acc
}

/// Final link of the chain, as in the legacy formulations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Final {
    /// S3(t, u, x_R) = 0 with u a free field unknown (legacy `specialized`).
    Specialized,
    /// A slope unknown for the line through the two partial sums and -R
    /// (legacy `line`).
    Line,
}

/// A formulation: the final link, and whether the total order x1 < x2 < x3 < x4
/// is imposed (breaking the 24-fold ordering symmetry of the summands).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Form {
    fin: Final,
    sym: bool,
}

/// Formulation names. `subgroup_xor` and `line_subgroup` are the legacy forms
/// (odd-subgroup constraint, native XOR rows, no symmetry breaking); the
/// `_sym` variants add the total order on the summands to both arms.
fn formulation(name: &str) -> Option<Form> {
    let (fin, sym) = match name {
        "subgroup_xor" => (Final::Specialized, false),
        "line_subgroup" => (Final::Line, false),
        "subgroup_xor_sym" => (Final::Specialized, true),
        "line_subgroup_sym" => (Final::Line, true),
        _ => return None,
    };
    Some(Form { fin, sym })
}

struct Model {
    n: usize,
    gf: Gf,
    nv: u32,
    clauses: Vec<Vec<i32>>,
    xors: Vec<(Vec<u32>, bool)>,
    and_gates: u64,
    products: u64,
    sq_cols: Vec<u64>,
    sqrt_cols: Vec<u64>,
    tr_row: Vec<bool>,
    mul_tab: Vec<u64>,
    /// Unknowns that no equation defines: coordinates, phase one-hots,
    /// subgroup witnesses and chain unknowns.
    free: Vec<u32>,
    /// The core unknowns of the decomposition: coordinates and phase one-hots.
    /// Once these are fixed the rest is a linear (XOR) system.
    core: Vec<u32>,
    /// Linear form -> the variable already defined to equal it, so equal forms
    /// share one variable (the legacy encoder defined one per use).
    form_vars: HashMap<(Vec<u32>, bool), u32>,
}

/// Which variables the solver branches on first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Priority {
    /// Solver default (VSIDS over every variable).
    None,
    /// Coordinates and phase one-hots only (the decomposition's own unknowns).
    Core,
    /// Every unknown no equation defines (core plus subgroup witnesses and
    /// chain unknowns).
    All,
}

impl Priority {
    fn name(&self) -> &'static str {
        match self {
            Priority::None => "none",
            Priority::Core => "core",
            Priority::All => "all",
        }
    }
    fn parse(s: &str) -> Priority {
        match s {
            "none" => Priority::None,
            "core" => Priority::Core,
            "all" => Priority::All,
            other => panic!("--priority none|core|all, got {other}"),
        }
    }
}

impl Model {
    fn new(gf: &Gf) -> Model {
        let n = gf.n as usize;
        Model {
            n,
            gf: gf.clone(),
            nv: 0,
            clauses: Vec::new(),
            xors: Vec::new(),
            and_gates: 0,
            products: 0,
            sq_cols: (0..n).map(|i| gf.sq(1 << i)).collect(),
            sqrt_cols: (0..n).map(|i| gf.sqrt(1 << i)).collect(),
            tr_row: (0..n).map(|i| gf.tr(1 << i) == 1).collect(),
            mul_tab: (0..n * n)
                .map(|ij| gf.mul(1 << (ij / n), 1 << (ij % n)))
                .collect(),
            free: Vec::new(),
            core: Vec::new(),
            form_vars: HashMap::new(),
        }
    }
    fn var(&mut self) -> u32 {
        self.nv += 1;
        self.nv
    }
    fn free_var(&mut self) -> u32 {
        let v = self.var();
        self.free.push(v);
        v
    }
    fn core_var(&mut self) -> u32 {
        let v = self.free_var();
        self.core.push(v);
        v
    }
    fn elem(&mut self) -> Vec<Lin> {
        (0..self.n).map(|_| Lin::var(self.free_var())).collect()
    }
    fn constant(&self, a: u64) -> Vec<Lin> {
        (0..self.n)
            .map(|k| Lin {
                v: Vec::new(),
                c: (a >> k) & 1 == 1,
            })
            .collect()
    }
    fn lin_map(&self, x: &[Lin], cols: &[u64]) -> Vec<Lin> {
        (0..self.n)
            .map(|k| {
                lsum(
                    (0..self.n)
                        .filter(|i| (cols[*i] >> k) & 1 == 1)
                        .map(|i| &x[i]),
                )
            })
            .collect()
    }
    fn add(&self, x: &[Lin], y: &[Lin]) -> Vec<Lin> {
        x.iter().zip(y).map(|(a, b)| a.add(b)).collect()
    }
    fn sq(&self, x: &[Lin]) -> Vec<Lin> {
        self.lin_map(x, &self.sq_cols)
    }
    fn sqrt(&self, x: &[Lin]) -> Vec<Lin> {
        self.lin_map(x, &self.sqrt_cols)
    }
    fn cmul(&self, a: u64, x: &[Lin]) -> Vec<Lin> {
        let cols: Vec<u64> = (0..self.n).map(|i| self.gf.mul(a, 1 << i)).collect();
        self.lin_map(x, &cols)
    }
    fn trace(&self, x: &[Lin]) -> Lin {
        lsum((0..self.n).filter(|i| self.tr_row[*i]).map(|i| &x[i]))
    }
    /// form == rhs
    fn xor_eq(&mut self, form: &Lin, rhs: bool) {
        let r = form.c ^ rhs;
        if form.v.is_empty() {
            assert!(!r, "inconsistent constant constraint");
            return;
        }
        self.xors.push((form.v.clone(), r));
    }
    fn zero(&mut self, x: &[Lin]) {
        for f in x {
            self.xor_eq(f, false);
        }
    }
    fn as_var(&mut self, form: &Lin) -> u32 {
        if form.v.len() == 1 && !form.c {
            return form.v[0];
        }
        if let Some(v) = self.form_vars.get(&(form.v.clone(), form.c)) {
            return *v;
        }
        let v = self.var();
        let def = form.add(&Lin::var(v));
        self.xor_eq(&def, false);
        self.form_vars.insert((form.v.clone(), form.c), v);
        v
    }
    fn mul(&mut self, x: &[Lin], y: &[Lin]) -> Vec<Lin> {
        let n = self.n;
        self.products += 1;
        let xv: Vec<u32> = x.iter().map(|a| self.as_var(a)).collect();
        let yv: Vec<u32> = y.iter().map(|b| self.as_var(b)).collect();
        let mut terms: Vec<Vec<u32>> = vec![Vec::new(); n];
        for i in 0..n {
            for j in 0..n {
                let g = self.var();
                let (a, b) = (xv[i] as i32, yv[j] as i32);
                let g32 = g as i32;
                self.clauses.push(vec![-g32, a]);
                self.clauses.push(vec![-g32, b]);
                self.clauses.push(vec![g32, -a, -b]);
                self.and_gates += 1;
                let m = self.mul_tab[i * n + j];
                for (k, t) in terms.iter_mut().enumerate() {
                    if (m >> k) & 1 == 1 {
                        t.push(g);
                    }
                }
            }
        }
        terms
            .into_iter()
            .map(|mut t| {
                t.sort_unstable();
                Lin { v: t, c: false }
            })
            .collect()
    }
    /// Lexicographic `a <= b` over variable ids, most significant bit last.
    /// With `differ` this is `a < b`.
    fn order_le(&mut self, a: &[u32], b: &[u32]) {
        let mut p: Option<u32> = None; // prefix of higher bits equal; None = true
        for i in (0..a.len()).rev() {
            let (ai, bi) = (a[i] as i32, b[i] as i32);
            match p {
                None => self.clauses.push(vec![-ai, bi]),
                Some(pv) => self.clauses.push(vec![-(pv as i32), -ai, bi]),
            }
            if i == 0 {
                break;
            }
            let d = self.var();
            let mut vs = vec![a[i], b[i], d];
            vs.sort_unstable();
            self.xors.push((vs, false));
            let np = self.var();
            let (n32, d32) = (np as i32, d as i32);
            match p {
                None => {
                    self.clauses.push(vec![n32, d32]);
                    self.clauses.push(vec![-n32, -d32]);
                }
                Some(pv) => {
                    let p32 = pv as i32;
                    self.clauses.push(vec![-n32, p32]);
                    self.clauses.push(vec![-n32, -d32]);
                    self.clauses.push(vec![n32, -p32, d32]);
                }
            }
            p = Some(np);
        }
    }
    /// The bit vectors `a` and `b` (variable ids) are not equal.
    fn differ(&mut self, a: &[u32], b: &[u32]) {
        let mut ds = Vec::new();
        for (p, q) in a.iter().zip(b) {
            let d = self.var();
            let mut vs = vec![*p, *q, d];
            vs.sort_unstable();
            self.xors.push((vs, false));
            ds.push(d as i32);
        }
        self.clauses.push(ds);
    }
    fn load(&self, priority: Priority) -> Solver {
        let mut s = Solver::new(self.nv);
        for c in &self.clauses {
            s.add_clause(c.clone());
        }
        for (vs, r) in &self.xors {
            s.add_xor(vs, *r);
        }
        match priority {
            Priority::None => {}
            Priority::Core => s.set_branch_priority(&self.core),
            Priority::All => s.set_branch_priority(&self.free),
        }
        s
    }
    fn size(&self) -> Value {
        json!({"vars": self.nv, "clauses": self.clauses.len(), "xors": self.xors.len(),
               "and_gates": self.and_gates, "products": self.products,
               "free_vars": self.free.len(), "core_vars": self.core.len()})
    }
}

/// Semaev S3 for y^2 + xy = x^3 + 1: (x1x2 + x1x3 + x2x3)^2 + x1x2x3 + 1 = 0.
fn f3(m: &mut Model, x1: &[Lin], x2: &[Lin], x3: &[Lin]) {
    let p = m.mul(x1, x2);
    let w = m.mul(x3, &m.add(x1, x2));
    let v = m.mul(&p, x3);
    let s = m.add(&p, &w);
    let e = m.add(&m.add(&m.sq(&s), &v), &m.constant(1));
    m.zero(&e);
}

/// Handles to the unknowns of a built model, for decoding and for tests.
#[allow(dead_code)]
struct Vars {
    c: Vec<Vec<u32>>,
    e: Option<Vec<Vec<u32>>>,
    q: Vec<Vec<u32>>,
}

/// Build the four-summand decomposition system for target `r = (a, b)`.
/// `phases = false` is arm A (x_i = v_i); `phases = true` is arm B
/// (x_i = v_i^(2^k_i) with k_i selected by a one-hot vector).
/// Only the field, the factor-space basis and the target enter; no witness.
fn build_model(gf: &Gf, basis: &[u64], r: (u64, u64), phases: bool, form: Form) -> (Model, Vars) {
    let fin = form.fin;
    let mut m = Model::new(gf);
    let n = m.n;
    let (a, b) = r;
    let s = basis.len();
    let cvars: Vec<Vec<u32>> = (0..4)
        .map(|_| (0..s).map(|_| m.core_var()).collect())
        .collect();
    let vs: Vec<Vec<Lin>> = cvars
        .iter()
        .map(|ci| {
            (0..n)
                .map(|k| {
                    let mut v: Vec<u32> = (0..s)
                        .filter(|j| (basis[*j] >> k) & 1 == 1)
                        .map(|j| ci[j])
                        .collect();
                    v.sort_unstable();
                    Lin { v, c: false }
                })
                .collect()
        })
        .collect();
    let mut qvars = Vec::new();
    for v in &vs {
        // v lies in ker Tr; the subgroup constraint is q^2 + sqrt(v) q + 1 = 0, Tr(q) = 0.
        let t = m.trace(v);
        m.xor_eq(&t, false);
        let q = m.elem();
        let sq = m.sqrt(v);
        let g = m.mul(&sq, &q);
        let e = m.add(&m.add(&m.sq(&q), &g), &m.constant(1));
        m.zero(&e);
        let tq = m.trace(&q);
        m.xor_eq(&tq, false);
        qvars.push(
            q.iter()
                .map(|l| {
                    assert!(l.v.len() == 1);
                    l.v[0]
                })
                .collect::<Vec<u32>>(),
        );
    }
    let mut evars: Option<Vec<Vec<u32>>> = None;
    let xs: Vec<Vec<Lin>>;
    if phases {
        let mut ev = Vec::new();
        let mut xl = Vec::new();
        for v in &vs {
            let e: Vec<u32> = (0..n).map(|_| m.core_var()).collect();
            m.clauses.push(e.iter().map(|x| *x as i32).collect());
            for i in 0..n {
                for j in (i + 1)..n {
                    m.clauses.push(vec![-(e[i] as i32), -(e[j] as i32)]);
                }
            }
            ev.push(e.clone());
            // frob[k] = Frob^k(v), linear in the bits of v
            let v0: Vec<Lin> = v.iter().map(|f| Lin::var(m.as_var(f))).collect();
            let mut frob = vec![v0];
            for _ in 1..n {
                let next = m.sq(frob.last().unwrap());
                frob.push(next);
            }
            let mut x = Vec::new();
            for bit in 0..n {
                let mut terms = Lin::default();
                for k in 0..n {
                    let form = frob[k][bit].clone();
                    let g = m.var();
                    if form.v.is_empty() {
                        if form.c {
                            m.xor_eq(&Lin::var(g).add(&Lin::var(e[k])), false);
                        } else {
                            m.clauses.push(vec![-(g as i32)]);
                        }
                    } else {
                        let fv = m.as_var(&form) as i32;
                        let (g32, e32) = (g as i32, e[k] as i32);
                        m.clauses.push(vec![-g32, e32]);
                        m.clauses.push(vec![-g32, fv]);
                        m.clauses.push(vec![g32, -e32, -fv]);
                        m.and_gates += 1;
                    }
                    terms = terms.add(&Lin::var(g));
                }
                x.push(terms);
            }
            xl.push(x);
        }
        let xvars: Vec<Vec<u32>> = xl
            .iter()
            .map(|x| x.iter().map(|f| m.as_var(f)).collect())
            .collect();
        for i in 0..4 {
            for j in (i + 1)..4 {
                m.differ(&xvars[i], &xvars[j]);
            }
        }
        if form.sym {
            for i in 0..3 {
                m.order_le(&xvars[i], &xvars[i + 1]);
            }
        }
        evars = Some(ev);
        xs = xl;
    } else {
        for i in 0..4 {
            for j in (i + 1)..4 {
                m.differ(&cvars[i], &cvars[j]);
            }
        }
        if form.sym {
            for i in 0..3 {
                m.order_le(&cvars[i], &cvars[i + 1]);
            }
        }
        xs = vs;
    }
    let t = m.elem();
    match fin {
        Final::Line => {
            let ell = m.elem();
            // u = t + l^2 + l + a ;  t u = a l^2 + a^2 + b + a
            let u = m.add(&m.add(&m.add(&t, &m.sq(&ell)), &ell), &m.constant(a));
            let p = m.mul(&t, &u);
            let rhs = m.add(&m.cmul(a, &m.sq(&ell)), &m.constant(gf.sq(a) ^ b ^ a));
            let e = m.add(&p, &rhs);
            m.zero(&e);
            f3(&mut m, &xs[0], &xs[1], &t);
            f3(&mut m, &xs[2], &xs[3], &u);
        }
        Final::Specialized => {
            let u = m.elem();
            // (tu + a(t+u))^2 + a tu + 1 = 0
            let p = m.mul(&t, &u);
            let inner = m.add(&p, &m.cmul(a, &m.add(&t, &u)));
            let e = m.add(&m.add(&m.sq(&inner), &m.cmul(a, &p)), &m.constant(1));
            m.zero(&e);
            f3(&mut m, &xs[0], &xs[1], &t);
            f3(&mut m, &xs[2], &xs[3], &u);
        }
    }
    (
        m,
        Vars {
            c: cvars,
            e: evars,
            q: qvars,
        },
    )
}

/// Decode the four abscissae and the per-summand (v, phase) from a model.
fn decode(gf: &Gf, basis: &[u64], vars: &Vars, sol: &[bool]) -> (Vec<u64>, Vec<(u64, u32)>) {
    let mut xs = Vec::new();
    let mut detail = Vec::new();
    for (i, ci) in vars.c.iter().enumerate() {
        let mut v = 0u64;
        for (j, var) in ci.iter().enumerate() {
            if sol[*var as usize - 1] {
                v ^= basis[j];
            }
        }
        let mut k = 0u32;
        if let Some(ev) = &vars.e {
            k = ev[i]
                .iter()
                .position(|e| sol[*e as usize - 1])
                .expect("one-hot phase") as u32;
        }
        detail.push((v, k));
        xs.push(gf.frob_k(v, k));
    }
    (xs, detail)
}

// ---------------------------------------------------------------------------
// One attempt
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
struct Budget {
    conflicts: u64,
    deadline_secs: f64,
    priority: Priority,
}

struct AttemptOut {
    status: &'static str,
    conflicts: u64,
    decisions: u64,
    propagations: u64,
    restarts: u64,
    learnt: u64,
    xor_propagations: u64,
    xor_conflicts: u64,
    word_ops: u64,
    rejected: u64,
    solver_calls: u64,
    found: Option<Vec<(u64, u64)>>,
    summands: Option<Vec<(u64, u32)>>,
    size: Value,
    build_wall_ns: u64,
    build_cpu_ns: u64,
    solve_wall_ns: u64,
    solve_cpu_ns: u64,
    verify_wall_ns: u64,
    verify_cpu_ns: u64,
    total_wall_ns: u64,
    total_cpu_ns: u64,
}

fn run_attempt(
    inst: &Instance,
    target: (u64, u64),
    phases: bool,
    form: Form,
    budget: Budget,
    seed: u64,
) -> AttemptOut {
    let (w0, c0) = (Instant::now(), cpu_ns());
    let (model, vars) = build_model(&inst.ec.gf, &inst.basis, target, phases, form);
    let mut solver = model.load(budget.priority);
    solver.conflict_budget = budget.conflicts;
    if seed != 0 {
        solver.scramble_saved_phases(seed);
    }
    let (w1, c1) = (Instant::now(), cpu_ns());
    let deadline = w1 + Duration::from_secs_f64(budget.deadline_secs);
    solver.deadline = Some(deadline);
    let (mut sw, mut sc, mut vw, mut vc) = (0u64, 0u64, 0u64, 0u64);
    let mut rejected = 0u64;
    let mut calls = 0u64;
    let mut found = None;
    let mut summands = None;
    let status;
    loop {
        let (a0, b0) = (Instant::now(), cpu_ns());
        calls += 1;
        let res = solver.solve();
        let (a1, b1) = (Instant::now(), cpu_ns());
        sw += (a1 - a0).as_nanos() as u64;
        sc += b1 - b0;
        match res {
            SolveResult::Unsat => {
                status = "unsat";
                break;
            }
            SolveResult::Unknown => {
                status = if solver.conflicts() >= budget.conflicts {
                    "budget"
                } else {
                    "deadline"
                };
                break;
            }
            SolveResult::Sat => {
                let sol = solver.model();
                let (xs, detail) = decode(&inst.ec.gf, &inst.basis, &vars, &sol);
                let ok = verify_xs(inst, &xs, target);
                let (a2, b2) = (Instant::now(), cpu_ns());
                vw += (a2 - a1).as_nanos() as u64;
                vc += b2 - b1;
                if let Some(pts) = ok {
                    found = Some(pts);
                    summands = Some(detail);
                    status = "verified";
                    break;
                }
                rejected += 1;
                // Block this (coordinates, phases) tuple and continue in the
                // same budget.
                let mut block: Vec<i32> = Vec::new();
                for ci in &vars.c {
                    for v in ci {
                        block.push(if sol[*v as usize - 1] {
                            -(*v as i32)
                        } else {
                            *v as i32
                        });
                    }
                }
                if let Some(ev) = &vars.e {
                    for e in ev.iter().flatten() {
                        block.push(if sol[*e as usize - 1] {
                            -(*e as i32)
                        } else {
                            *e as i32
                        });
                    }
                }
                solver.reset_search();
                solver.add_clause(block);
            }
        }
    }
    let (wend, cend) = (Instant::now(), cpu_ns());
    let st = solver.stats;
    AttemptOut {
        status,
        conflicts: solver.conflicts(),
        decisions: st.decisions,
        propagations: st.propagations,
        restarts: st.restarts,
        learnt: st.learnt_clauses,
        xor_propagations: st.xor_propagations,
        xor_conflicts: st.xor_conflicts,
        word_ops: solver.word_ops(),
        rejected,
        solver_calls: calls,
        found,
        summands,
        size: model.size(),
        build_wall_ns: (w1 - w0).as_nanos() as u64,
        build_cpu_ns: c1 - c0,
        solve_wall_ns: sw,
        solve_cpu_ns: sc,
        verify_wall_ns: vw,
        verify_cpu_ns: vc,
        total_wall_ns: (wend - w0).as_nanos() as u64,
        total_cpu_ns: cend - c0,
    }
}

// ---------------------------------------------------------------------------
// Exhaustive pair matching (no solver): the structural ground truth
// ---------------------------------------------------------------------------

/// For each target, the set of unordered x-quadruples (as sorted index
/// quadruples into `xs`) whose signed sum is the target. `phases = false`
/// takes the representatives only, `true` all `n` Frobenius images.
fn exhaustive(inst: &Instance, phases: bool) -> (Vec<u64>, Vec<BTreeSet<[usize; 4]>>) {
    let ph = if phases { inst.n } else { 1 };
    let mut xs = Vec::new();
    let mut pts = Vec::new();
    for j in 0..inst.reps.len() {
        for k in 0..ph {
            let p = inst.phase_point(j, k);
            xs.push(p.0);
            pts.push(p);
        }
    }
    // table: signed pair sum -> pairs (a, b) of x-classes
    let mut table: HashMap<Pt, Vec<(usize, usize)>> = HashMap::new();
    for a in 0..xs.len() {
        for b in (a + 1)..xs.len() {
            for sa in [false, true] {
                for sb in [false, true] {
                    let pa = if sa {
                        inst.ec.neg(Some(pts[a]))
                    } else {
                        Some(pts[a])
                    };
                    let pb = if sb {
                        inst.ec.neg(Some(pts[b]))
                    } else {
                        Some(pts[b])
                    };
                    let s = inst.ec.add(pa, pb);
                    table.entry(s).or_default().push((a, b));
                }
            }
        }
    }
    let mut out = Vec::new();
    for t in &inst.targets {
        let mut found: BTreeSet<[usize; 4]> = BTreeSet::new();
        for (s1, pairs1) in &table {
            let need = inst.ec.add(Some(t.r), inst.ec.neg(*s1));
            if let Some(pairs2) = table.get(&need) {
                for &(a, b) in pairs1 {
                    for &(c, d) in pairs2 {
                        let mut q = [a, b, c, d];
                        q.sort_unstable();
                        if q[0] != q[1] && q[1] != q[2] && q[2] != q[3] {
                            found.insert(q);
                        }
                    }
                }
            }
        }
        out.push(found);
    }
    (xs, out)
}

// ---------------------------------------------------------------------------
// Receipts
// ---------------------------------------------------------------------------

fn schedule(inst: &Instance, forms: &[String], reps: u64) -> Vec<Value> {
    let mut seq = Vec::new();
    for rep in 1..=reps {
        for form in forms {
            for (i, t) in inst.targets.iter().enumerate() {
                let order = if (i as u64 + rep - 1).is_multiple_of(2) {
                    ["A", "B"]
                } else {
                    ["B", "A"]
                };
                for (pos, arm) in order.iter().enumerate() {
                    seq.push(json!({"rep": rep, "form": form, "pair": i, "position": pos,
                                    "arm": arm, "target_id": t.id, "set": t.set}));
                }
            }
        }
    }
    seq
}

fn read_jsonl(path: &str) -> Vec<Value> {
    fs::read_to_string(path)
        .map(|s| {
            s.lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| serde_json::from_str(l).expect("receipt line"))
                .collect()
        })
        .unwrap_or_default()
}

fn attempt_value(item: &Value, inst: &Instance, out: &AttemptOut, seq: u64, seed: u64) -> Value {
    let t = inst
        .targets
        .iter()
        .find(|t| t.id == item["target_id"])
        .unwrap();
    let mut v = json!({
        "type": "attempt", "seq": seq,
        "rep": item["rep"], "form": item["form"], "pair": item["pair"],
        "position": item["position"], "arm": item["arm"],
        "phases": item["arm"] == "B", "n": inst.n,
        "set": item["set"], "target_id": item["target_id"],
        "target": [hx(t.r.0), hx(t.r.1)], "phase_seed": seed,
        "status": out.status,
        "conflicts": out.conflicts, "decisions": out.decisions,
        "propagations": out.propagations, "restarts": out.restarts,
        "learnt_clauses": out.learnt, "xor_propagations": out.xor_propagations,
        "xor_conflicts": out.xor_conflicts, "word_ops": out.word_ops,
        "rejected_tuples": out.rejected, "solver_calls": out.solver_calls,
        "model": out.size,
        "build_wall_ns": out.build_wall_ns, "build_cpu_ns": out.build_cpu_ns,
        "solve_wall_ns": out.solve_wall_ns, "solve_cpu_ns": out.solve_cpu_ns,
        "verify_wall_ns": out.verify_wall_ns, "verify_cpu_ns": out.verify_cpu_ns,
        "total_wall_ns": out.total_wall_ns, "total_cpu_ns": out.total_cpu_ns,
        "loadavg_end": loadavg(), "maxrss_kb_process": maxrss_kb(),
    });
    if let Some(f) = &out.found {
        v["found_points"] = json!(f.iter().map(|p| [hx(p.0), hx(p.1)]).collect::<Vec<_>>());
    }
    if let Some(s) = &out.summands {
        v["solver_summands"] = json!(s
            .iter()
            .map(|(x, k)| json!({"v": hx(*x), "k": k}))
            .collect::<Vec<_>>());
    }
    v
}

fn arg_map(args: &[String]) -> HashMap<String, String> {
    let mut m = HashMap::new();
    let mut i = 0;
    while i < args.len() {
        if let Some(k) = args[i].strip_prefix("--") {
            if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                m.insert(k.to_string(), args[i + 1].clone());
                i += 2;
            } else {
                m.insert(k.to_string(), "true".into());
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    m
}

fn instance_from_args(a: &HashMap<String, String>) -> Instance {
    let path = a
        .get("manifest")
        .cloned()
        .unwrap_or_else(|| DEFAULT_MANIFEST.to_string());
    let expect = a.get("expect-sha").cloned().or_else(|| {
        if path == DEFAULT_MANIFEST {
            Some(FROZEN_MANIFEST_SHA.to_string())
        } else {
            None
        }
    });
    Instance::load(&path, expect.as_deref()).unwrap_or_else(|e| {
        eprintln!("manifest refused: {e}");
        std::process::exit(3);
    })
}

fn budget_from_args(a: &HashMap<String, String>) -> Budget {
    Budget {
        conflicts: a
            .get("conflict-budget")
            .map(|v| v.parse().expect("conflict-budget"))
            .unwrap_or(1_000_000),
        deadline_secs: a
            .get("deadline-secs")
            .map(|v| v.parse().expect("deadline-secs"))
            .unwrap_or(600.0),
        priority: Priority::parse(a.get("priority").map(String::as_str).unwrap_or("core")),
    }
}

fn run_cmd(a: &HashMap<String, String>) -> i32 {
    let inst = instance_from_args(a);
    let budget = budget_from_args(a);
    let out_path = a.get("out").expect("--out receipts.jsonl").clone();
    let forms: Vec<String> = a
        .get("forms")
        .cloned()
        .unwrap_or_else(|| "subgroup_xor,line_subgroup".into())
        .split(',')
        .map(String::from)
        .collect();
    for f in &forms {
        assert!(formulation(f).is_some(), "unknown formulation {f}");
    }
    let reps: u64 = a.get("reps").map(|v| v.parse().unwrap()).unwrap_or(3);
    let cpu_cap: f64 = a
        .get("cpu-cap")
        .map(|v| v.parse().unwrap())
        .unwrap_or(f64::INFINITY);
    let only: Option<Vec<String>> = a
        .get("targets")
        .map(|v| v.split(',').map(String::from).collect());
    let prior = read_jsonl(&out_path);
    let mut done = BTreeSet::new();
    let mut seq = 0u64;
    let mut spent = 0.0f64;
    let producer_sha = sha_hex(THIS_SOURCE);
    if let Some(h) = prior.first() {
        if !a.contains_key("resume") {
            eprintln!("{out_path} exists; pass --resume to append");
            return 3;
        }
        assert_eq!(
            h["manifest_sha256"], inst.manifest_sha,
            "different manifest"
        );
        assert_eq!(
            h["producer_sha256"], producer_sha,
            "different producer source"
        );
        assert_eq!(h["conflict_budget"], budget.conflicts, "different budget");
        assert_eq!(h["forms"], json!(forms), "different formulations");
        assert_eq!(h["branch_priority"], budget.priority.name());
        for r in &prior {
            if r["type"] == "attempt" {
                seq += 1;
                if r["status"] != "skipped" {
                    spent += r["total_cpu_ns"].as_u64().unwrap() as f64 / 1e9;
                    done.insert((
                        r["rep"].as_u64().unwrap(),
                        r["form"].as_str().unwrap().to_string(),
                        r["target_id"].as_str().unwrap().to_string(),
                        r["arm"].as_str().unwrap().to_string(),
                    ));
                }
            }
        }
    }
    let mut out = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&out_path)
        .expect("open receipts");
    let mut emit = |v: &Value| {
        writeln!(out, "{}", serde_json::to_string(v).unwrap()).unwrap();
        out.flush().unwrap();
        out.sync_data().ok();
    };
    let meta = json!({
        "created_unix": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0),
        "command": std::env::args().collect::<Vec<_>>().join(" "),
        "cpu_model": cpu_model(),
        "cpus": std::thread::available_parallelism().map(|n| n.get()).unwrap_or(0),
        "loadavg_start": loadavg(),
        "rustc": cmd_out("rustc", &["--version"]),
        "kernel": cmd_out("uname", &["-sr"]),
    });
    if prior.is_empty() {
        emit(&json!({
            "type": "header", "schema": SCHEMA,
            "manifest_sha256": inst.manifest_sha, "producer_sha256": producer_sha,
            "forms": forms, "reps_planned": reps,
            "conflict_budget": budget.conflicts, "deadline_secs": budget.deadline_secs,
            "branch_priority": budget.priority.name(),
            "phase_seeds": (1..=reps).map(phase_seed).collect::<Vec<_>>(),
            "solver": "crypto_lib::cryptanalysis::sat::Solver, one thread, native XOR rows, deterministic",
            "arms": {"A": "representatives only", "B": "one-hot Frobenius phases"},
            "budget_clock": "conflicts (deterministic primary); wall deadline is a backstop only",
            "schedule": "rep-major, formulation, target order; pair i runs A,B if (i+rep-1) even else B,A",
            "meta": meta,
        }));
    } else {
        emit(&json!({"type": "resume", "reps_planned": reps, "meta": meta}));
    }
    let mut capped = false;
    for item in schedule(&inst, &forms, reps) {
        let key = (
            item["rep"].as_u64().unwrap(),
            item["form"].as_str().unwrap().to_string(),
            item["target_id"].as_str().unwrap().to_string(),
            item["arm"].as_str().unwrap().to_string(),
        );
        if done.contains(&key) {
            continue;
        }
        if item["position"] == 0 && spent >= cpu_cap {
            capped = true;
        }
        if capped {
            let mut rec = item.clone();
            rec["type"] = json!("attempt");
            rec["seq"] = json!(seq);
            rec["status"] = json!("skipped");
            rec["reason"] = json!(format!("cpu_cap {cpu_cap} s reached"));
            seq += 1;
            emit(&rec);
            continue;
        }
        let t = inst
            .targets
            .iter()
            .find(|t| t.id == item["target_id"].as_str().unwrap())
            .unwrap();
        if let Some(only) = &only {
            if !only.contains(&t.id) {
                continue;
            }
        }
        if let Some(arms) = a.get("arms") {
            if !arms.contains(item["arm"].as_str().unwrap()) {
                continue;
            }
        }
        let fin = formulation(item["form"].as_str().unwrap()).unwrap();
        let seed = phase_seed(item["rep"].as_u64().unwrap());
        let res = run_attempt(&inst, t.r, item["arm"] == "B", fin, budget, seed);
        spent += res.total_cpu_ns as f64 / 1e9;
        emit(&attempt_value(&item, &inst, &res, seq, seed));
        seq += 1;
        // Progress lines carry no timing: the gate is read before any clock.
        println!(
            "rep {} {:14} pair {:2} {} {} {}",
            item["rep"],
            item["form"].as_str().unwrap(),
            item["pair"],
            item["arm"].as_str().unwrap(),
            t.id,
            res.status
        );
    }
    emit(&json!({"type": "end", "capped": capped,
                 "total_attempt_cpu_s": (spent * 1000.0).round() / 1000.0}));
    0
}

// ---------------------------------------------------------------------------
// Analysis used by `gate`
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Att {
    rep: u64,
    seq: u64,
    form: String,
    arm: String,
    target_id: String,
    set: String,
    status: String,
    conflicts: u64,
    decisions: u64,
    propagations: u64,
    cpu_ns: u64,
    wall_ns: u64,
    rejected: u64,
    pts: Option<Vec<(u64, u64)>>,
    target: (u64, u64),
}

fn parse_attempts(recs: &[Value]) -> Vec<Att> {
    recs.iter()
        .filter(|r| r["type"] == "attempt")
        .map(|r| Att {
            rep: r["rep"].as_u64().unwrap(),
            seq: r["seq"].as_u64().unwrap(),
            form: r["form"].as_str().unwrap().into(),
            arm: r["arm"].as_str().unwrap().into(),
            target_id: r["target_id"].as_str().unwrap().into(),
            set: r["set"].as_str().unwrap().into(),
            status: r["status"].as_str().unwrap().into(),
            conflicts: r["conflicts"].as_u64().unwrap_or(0),
            decisions: r["decisions"].as_u64().unwrap_or(0),
            propagations: r["propagations"].as_u64().unwrap_or(0),
            cpu_ns: r["total_cpu_ns"].as_u64().unwrap_or(0),
            wall_ns: r["total_wall_ns"].as_u64().unwrap_or(0),
            rejected: r["rejected_tuples"].as_u64().unwrap_or(0),
            pts: r["found_points"].as_array().map(|a| {
                a.iter()
                    .map(|p| (unhex(p[0].as_str().unwrap()), unhex(p[1].as_str().unwrap())))
                    .collect()
            }),
            target: (
                unhex(r["target"][0].as_str().unwrap()),
                unhex(r["target"][1].as_str().unwrap()),
            ),
        })
        .collect()
}

#[derive(Default, Clone, Debug)]
struct Scope {
    attempts: u64,
    verified: u64,
    unsat: u64,
    budget: u64,
    deadline: u64,
    skipped: u64,
    rejected: u64,
    dependent: u64,
    failed_checks: u64,
    rank: usize,
    conflicts: u64,
    decisions: u64,
    propagations: u64,
    cpu_ns: u64,
    wall_ns: u64,
    /// cost charged when rank k was first reached: (attempts, conflicts, cpu_ns)
    first_hit: BTreeMap<usize, (u64, u64, u64)>,
}

/// Fold attempts of one (rep, form, arm) in receipt order, with a rank
/// tracker over the representative columns.
fn fold_scope(inst: &Instance, atts: &[&Att]) -> Scope {
    let mut sc = Scope::default();
    let mut tracker = U64RankTracker::new(inst.r, inst.reps.len());
    let mut ordered: Vec<&&Att> = atts.iter().collect();
    ordered.sort_by_key(|a| a.seq);
    for a in ordered {
        if a.status == "skipped" {
            sc.skipped += 1;
            continue;
        }
        sc.attempts += 1;
        sc.conflicts += a.conflicts;
        sc.decisions += a.decisions;
        sc.propagations += a.propagations;
        sc.cpu_ns += a.cpu_ns;
        sc.wall_ns += a.wall_ns;
        sc.rejected += a.rejected;
        match a.status.as_str() {
            "unsat" => sc.unsat += 1,
            "budget" => sc.budget += 1,
            "deadline" => sc.deadline += 1,
            "verified" => {
                sc.verified += 1;
                let ok = a
                    .pts
                    .as_ref()
                    .and_then(|p| relation_from_points(inst, p).ok())
                    .filter(|rel| check_relation(inst, rel, a.target).all());
                match ok {
                    None => sc.failed_checks += 1,
                    Some(rel) => {
                        let before = tracker.rank();
                        let after = tracker.insert(rel.coeffs.clone());
                        if after > before {
                            sc.first_hit
                                .insert(after, (sc.attempts, sc.conflicts, sc.cpu_ns));
                        } else {
                            sc.dependent += 1;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    sc.rank = tracker.rank();
    sc
}

type ScopeKey = (u64, String, String); // rep, form, arm

fn all_scopes(inst: &Instance, atts: &[Att]) -> BTreeMap<ScopeKey, Scope> {
    let mut groups: BTreeMap<ScopeKey, Vec<&Att>> = BTreeMap::new();
    for a in atts {
        groups
            .entry((a.rep, a.form.clone(), a.arm.clone()))
            .or_default()
            .push(a);
    }
    groups
        .into_iter()
        .map(|(k, v)| (k, fold_scope(inst, &v)))
        .collect()
}

// ---------------------------------------------------------------------------
// gate: structural facts, no timing is read
// ---------------------------------------------------------------------------

/// The legacy facts this run must reproduce before any clock is read
/// (research/frobenius_quotient_sat_20260930/paired_fixed_budget_20261011).
const LEGACY_A_DECOMPOSABLE: [&str; 4] = ["b0", "b1", "b2", "b3"];
const LEGACY_RANK_A: usize = 4;
const LEGACY_RANK_B: usize = 5;

fn gate_cmd(a: &HashMap<String, String>) -> i32 {
    let inst = instance_from_args(a);
    let mut ok = true;
    let (xa, fa) = exhaustive(&inst, false);
    let (xb, fb) = exhaustive(&inst, true);
    println!(
        "exhaustive pair matching: arm A pool {} x-coordinates, arm B pool {}",
        xa.len(),
        xb.len()
    );
    let mut dec_a = Vec::new();
    let mut dec_b = Vec::new();
    for (i, t) in inst.targets.iter().enumerate() {
        let (na, nb) = (fa[i].len(), fb[i].len());
        println!(
            "  {:3} set {} : decomposable arm A {:5} ({} x-quadruples), arm B {:5} ({} x-quadruples)",
            t.id, t.set, na > 0, na, nb > 0, nb
        );
        if na > 0 {
            dec_a.push(t.id.clone());
        }
        if nb > 0 {
            dec_b.push(t.id.clone());
        }
    }
    let want_a: Vec<String> = LEGACY_A_DECOMPOSABLE
        .iter()
        .map(|s| s.to_string())
        .collect();
    let pm_ok = dec_a == want_a && dec_b.len() == inst.targets.len();
    println!(
        "exhaustive: arm A decomposes {} of {} ({}), arm B decomposes {} of {}  -> {}",
        dec_a.len(),
        inst.targets.len(),
        dec_a.join(","),
        dec_b.len(),
        inst.targets.len(),
        if pm_ok {
            "MATCHES legacy"
        } else {
            "DIFFERS from legacy"
        }
    );
    ok &= pm_ok;

    if let Some(path) = a.get("sealed") {
        // Checker only: the solver arms never read this file.
        let text = fs::read_to_string(path).expect("sealed file");
        let sealed: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(sealed["manifest_sha256"], inst.manifest_sha);
        let mut agree = true;
        for (i, t) in inst.targets.iter().enumerate() {
            let bf = &sealed["targets"][&t.id]["brute_force"];
            agree &= bf["decomposable_reps_only"].as_bool() == Some(!fa[i].is_empty());
            agree &= bf["decomposable_phases_free"].as_bool() == Some(!fb[i].is_empty());
        }
        println!("sealed legacy brute force agrees with native exhaustive check: {agree}");
        ok &= agree;
    }

    if let Some(path) = a.get("receipts") {
        let atts = parse_attempts(&read_jsonl(path));
        let rep_filter: Option<u64> = a.get("rep").map(|v| v.parse().unwrap());
        let atts: Vec<Att> = atts
            .into_iter()
            .filter(|x| rep_filter.is_none_or(|r| x.rep == r))
            .collect();
        let scopes = all_scopes(&inst, &atts);
        let by_target: BTreeMap<String, usize> = inst
            .targets
            .iter()
            .enumerate()
            .map(|(i, t)| (t.id.clone(), i))
            .collect();
        for ((rep, form, arm), sc) in &scopes {
            let want_rank = if arm == "A" {
                LEGACY_RANK_A
            } else {
                LEGACY_RANK_B
            };
            let mut line = format!(
                "rep {rep} {form:14} arm {arm}: attempts {:2} verified {:2} unsat {:2} budget {} deadline {} rejected {} failed-checks {} rank {} (legacy {want_rank})",
                sc.attempts, sc.verified, sc.unsat, sc.budget, sc.deadline, sc.rejected, sc.failed_checks, sc.rank
            );
            // Every verified / unsat answer must agree with the exhaustive check.
            let mut bad = 0;
            for x in atts
                .iter()
                .filter(|x| x.rep == *rep && &x.form == form && &x.arm == arm)
            {
                let i = by_target[&x.target_id];
                let decomposable = if arm == "A" {
                    !fa[i].is_empty()
                } else {
                    !fb[i].is_empty()
                };
                match x.status.as_str() {
                    "verified" if !decomposable => bad += 1,
                    "unsat" if decomposable => bad += 1,
                    _ => {}
                }
            }
            let unresolved = sc.budget + sc.deadline;
            let want_ver = if arm == "A" { 4 } else { 12 };
            let want_uns = if arm == "A" { 8 } else { 0 };
            let pass = bad == 0
                && unresolved == 0
                && sc.failed_checks == 0
                && sc.rank == want_rank
                && sc.verified == want_ver
                && sc.unsat == want_uns
                && sc.attempts == 12;
            line.push_str(&format!(
                " | contradictions with exhaustive {bad} -> {}",
                if pass {
                    "REPRODUCES"
                } else {
                    "DOES NOT REPRODUCE"
                }
            ));
            println!("{line}");
            ok &= pass;
        }
        if scopes.is_empty() {
            println!("no attempts in receipts: structural solver gate not evaluated");
            ok = false;
        }
    }
    println!("GATE {}", if ok { "PASS" } else { "FAIL" });
    if ok {
        0
    } else {
        2
    }
}

// ---------------------------------------------------------------------------
// freeze: a new manifest, written natively, refused if the file exists
// ---------------------------------------------------------------------------

fn freeze_cmd(a: &HashMap<String, String>) -> i32 {
    // Parameters: --n --modulus --s --factor-seed --target-seed --per-set --out
    // or --like manifest.json (reuse its curve and factor space).
    let out_path = a.get("out").expect("--out").clone();
    if fs::metadata(&out_path).is_ok() {
        eprintln!("{out_path} exists: a frozen manifest is never overwritten");
        return 3;
    }
    let tseed: u64 = a
        .get("target-seed")
        .expect("--target-seed")
        .parse()
        .unwrap();
    let per_set: usize = a.get("per-set").map(|v| v.parse().unwrap()).unwrap_or(4);
    let (n, modulus, ec, g, r, basis, reps) = if let Some(like) = a.get("like") {
        let inst = Instance::load(like, None).expect("--like manifest");
        let modulus = unhex(
            serde_json::from_str::<Value>(&fs::read_to_string(like).unwrap()).unwrap()["curve"]
                ["modulus"]
                .as_str()
                .unwrap(),
        );
        let ec = Ec::new(inst.n, modulus, inst.r, 4);
        (
            inst.n,
            modulus,
            ec,
            inst.g,
            inst.r,
            inst.basis.clone(),
            inst.reps.clone(),
        )
    } else {
        let n: u32 = a.get("n").expect("--n").parse().unwrap();
        let modulus = unhex(a.get("modulus").expect("--modulus 0x..."));
        let (ec, g, r, basis, reps) = native_curve_and_space(n, modulus, a);
        (n, modulus, ec, g, r, basis, reps)
    };
    let mut rng = SplitMix(tseed.wrapping_mul(10_000).wrapping_add(n as u64 * 10));
    let mut sets = serde_json::Map::new();
    let names = [
        ("a", "planted_random_phase"),
        ("b", "planted_phase0"),
        ("c", "uniform_random"),
    ];
    let mut log: HashMap<(u64, u64), u64> = HashMap::new();
    {
        let mut p: Pt = None;
        for k in 0..r {
            if let Some(q) = p {
                log.insert(q, k);
            }
            p = ec.add(p, Some(g));
        }
    }
    let by_log: HashMap<u64, (u64, u64)> = log.iter().map(|(p, l)| (*l, *p)).collect();
    for (si, (key, name)) in names.iter().enumerate() {
        let mut ts = Vec::new();
        let mut idx = 0;
        while ts.len() < per_set {
            let target: Pt = match *key {
                "c" => {
                    let k = 1 + rng.below(r - 1);
                    Some(by_log[&k])
                }
                _ => {
                    // four distinct representatives, random signs, phases random or zero
                    let mut pool: Vec<usize> = (0..reps.len()).collect();
                    let mut pick = Vec::new();
                    for _ in 0..4 {
                        let j = rng.below(pool.len() as u64) as usize;
                        pick.push(pool.remove(j));
                    }
                    let phases: Vec<u32> = if *key == "a" {
                        let p: Vec<u32> = (0..4).map(|_| rng.below(n as u64) as u32).collect();
                        if p.iter().all(|k| *k == 0) {
                            continue;
                        }
                        p
                    } else {
                        vec![0; 4]
                    };
                    let mut sum: Pt = None;
                    for (j, k) in pick.iter().zip(&phases) {
                        let mut pt = ec.frob_k(Some(reps[*j]), *k);
                        if rng.below(2) == 1 {
                            pt = ec.neg(pt);
                        }
                        sum = ec.add(sum, pt);
                    }
                    sum
                }
            };
            let Some(tp) = target else { continue };
            ts.push(json!({"id": format!("{}{}", key, idx), "R": [hx(tp.0), hx(tp.1)]}));
            idx += 1;
        }
        let _ = si;
        sets.insert(key.to_string(), json!({"name": name, "targets": ts}));
    }
    let reps_json: Vec<Value> = reps
        .iter()
        .enumerate()
        .map(|(i, p)| json!({"index": i, "point": [hx(p.0), hx(p.1)], "x": hx(p.0)}))
        .collect();
    let manifest = json!({
        "schema": "phase_paired_native/manifest/v1",
        "curve": {"equation": "y^2 + x*y = x^3 + 1", "n": n, "modulus": hx(modulus),
                   "generator": [hx(g.0), hx(g.1)], "group_order": r * 4,
                   "odd_subgroup_order": r, "trace_t": -1,
                   "lambda_relation": "pi(P) = [lambda]P on H, lambda^2 - t*lambda + 2 = 0 mod r"},
        "factor_space": {"s": basis.len(), "basis": basis.iter().map(|b| hx(*b)).collect::<Vec<_>>(),
                          "representatives": reps_json,
                          "admissible_x_phases_free": reps.len() as u32 * n,
                          "admissible_x_phases_known_zero": reps.len()},
        "target_rng": "splitmix64 seeded with target_seed*10000 + n*10; sets a, b, c drawn in that order",
        "target_seed": tseed,
        "target_sets": Value::Object(sets),
    });
    let text = serde_json::to_string_pretty(&manifest).unwrap() + "\n";
    fs::write(&out_path, &text).unwrap();
    println!("{out_path} sha256 {}", sha_hex(text.as_bytes()));
    0
}

/// Curve data for freezing a fresh field: the curve `y^2 + xy = x^3 + 1` is defined
/// over `F_2` with Frobenius trace `-1`, so `#E(F_{2^n}) = 2^n + 1 - V_n` with the Lucas
/// sequence `V_0 = 2, V_1 = -1, V_{k+1} = -V_k - 2 V_{k-1}`; the code counts the points
/// by enumeration and checks the count against that formula.
#[allow(clippy::type_complexity)]
fn native_curve_and_space(
    n: u32,
    modulus: u64,
    a: &HashMap<String, String>,
) -> (Ec, (u64, u64), u64, Vec<u64>, Vec<(u64, u64)>) {
    // Count points by exhaustive enumeration of x (n small enough for this).
    let probe = Ec::new(n, modulus, 1, 4);
    let gf = probe.gf.clone();
    let mut count: u64 = 1; // infinity
    for x in 0..(1u64 << n) {
        if x == 0 {
            count += 1;
        } else {
            let c = x ^ gf.inv(gf.sq(x));
            count += if gf.tr(c) == 0 { 2 } else { 0 };
        }
    }
    assert!(count.is_multiple_of(4), "expected a cofactor-4 curve");
    let r = count / 4;
    let ec = Ec::new(n, modulus, r, 4);
    let (mut v0, mut v1) = (2i128, -1i128);
    for _ in 1..n {
        let v2 = -v1 - 2 * v0;
        v0 = v1;
        v1 = v2;
    }
    assert_eq!(
        count as i128,
        (1i128 << n) + 1 - v1,
        "point count must equal 2^n + 1 - V_n"
    );
    // generator of the odd subgroup: 4P for the first x with a lift
    let mut g = None;
    for x in 2..(1u64 << n) {
        if let Some(p) = ec.lift(x) {
            let q = ec.mul(4, Some(p));
            if q.is_some() {
                g = q;
                break;
            }
        }
    }
    let g = g.unwrap();
    assert!(ec.mul(r, Some(g)).is_none(), "r must annihilate g");
    // factor space: random s-dimensional subspace of ker Tr with >= 4 valid x
    let s: usize = a.get("s").unwrap().parse().unwrap();
    let fseed: u64 = a.get("factor-seed").unwrap().parse().unwrap();
    let mut rng = SplitMix(fseed.wrapping_mul(1000).wrapping_add(n as u64));
    let mut log_ok: HashMap<(u64, u64), ()> = HashMap::new();
    {
        let mut p: Pt = None;
        for _ in 0..r {
            if let Some(q) = p {
                log_ok.insert(q, ());
            }
            p = ec.add(p, Some(g));
        }
    }
    loop {
        let mut basis: Vec<u64> = Vec::new();
        while basis.len() < s {
            let v = rng.next() & ((1u64 << n) - 1);
            if gf.tr(v) == 1 || v == 0 {
                continue;
            }
            let mut span = vec![0u64];
            for b in &basis {
                let more: Vec<u64> = span.iter().map(|w| w ^ b).collect();
                span.extend(more);
            }
            if span.contains(&v) {
                continue;
            }
            basis.push(v);
        }
        let mut span = vec![0u64];
        for b in &basis {
            let more: Vec<u64> = span.iter().map(|w| w ^ b).collect();
            span.extend(more);
        }
        span.sort_unstable();
        let mut valid: Vec<(u64, u64)> = Vec::new();
        for x in span {
            if x == 0 {
                continue;
            }
            if let Some(p) = ec.lift(x) {
                if log_ok.contains_key(&p) {
                    valid.push(p);
                }
            }
        }
        if valid.len() >= 4 {
            return (ec, g, r, basis, valid);
        }
    }
}

fn usage() {
    eprintln!(
        "usage: phase_paired_fixed_budget <gate|run|freeze> [--key value ...]\n\
         gate    [--manifest P] [--sealed witness_sealed.json] [--receipts R.jsonl --rep N]\n\
         run     --out R.jsonl [--manifest P] [--forms subgroup_xor,line_subgroup] [--reps 3]\n\
                 [--conflict-budget C] [--deadline-secs D] [--cpu-cap S] [--targets a0,b1] [--arms A] [--resume]\n\
                 [--priority none|core|all]\n\
         freeze  --out M.json --target-seed S [--like M0.json | --n N --modulus 0x.. --s S --factor-seed F] [--per-set 4]"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = match args.first().map(String::as_str) {
        Some("gate") => gate_cmd(&arg_map(&args[1..])),
        Some("run") => run_cmd(&arg_map(&args[1..])),
        Some("freeze") => freeze_cmd(&arg_map(&args[1..])),
        _ => {
            usage();
            64
        }
    };
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inst() -> Instance {
        Instance::load(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/research/phase_paired_native_20261026/fixtures/manifest.json"
            ),
            Some(FROZEN_MANIFEST_SHA),
        )
        .expect("fixture manifest")
    }

    /// A planted decomposition: four distinct representatives, random phases
    /// (not all zero unless `phase0`), random signs. Returns the target and the
    /// (rep, phase) pairs.
    fn plant(inst: &Instance, seed: u64, phase0: bool) -> ((u64, u64), Vec<(usize, u32)>) {
        let mut rng = SplitMix(seed);
        loop {
            let mut pool: Vec<usize> = (0..inst.reps.len()).collect();
            let mut spec = Vec::new();
            for _ in 0..4 {
                let j = rng.below(pool.len() as u64) as usize;
                let rep = pool.remove(j);
                let k = if phase0 {
                    0
                } else {
                    rng.below(inst.n as u64) as u32
                };
                spec.push((rep, k));
            }
            if !phase0 && spec.iter().all(|(_, k)| *k == 0) {
                continue;
            }
            let mut sum: Pt = None;
            for (rep, k) in &spec {
                let mut p = Some(inst.phase_point(*rep, *k));
                if rng.below(2) == 1 {
                    p = inst.ec.neg(p);
                }
                sum = inst.ec.add(sum, p);
            }
            if let Some(t) = sum {
                return (t, spec);
            }
        }
    }

    /// Coordinates of `v` in the factor-space basis.
    fn coords(inst: &Instance, v: u64) -> Vec<bool> {
        let s = inst.basis.len();
        for mask in 0u32..(1 << s) {
            let mut acc = 0u64;
            for j in 0..s {
                if (mask >> j) & 1 == 1 {
                    acc ^= inst.basis[j];
                }
            }
            if acc == v {
                return (0..s).map(|j| (mask >> j) & 1 == 1).collect();
            }
        }
        panic!("representative outside the factor space");
    }

    /// Pin the summands: unit clauses on the coordinate bits, and on the phase
    /// one-hot bits when the model has them (`phases[i] = Some(k)`).
    fn pin(solver: &mut Solver, inst: &Instance, vars: &Vars, spec: &[(usize, Option<u32>)]) {
        for (i, (rep, k)) in spec.iter().enumerate() {
            let c = coords(inst, inst.reps[*rep].0);
            for (j, bit) in c.iter().enumerate() {
                let v = vars.c[i][j] as i32;
                solver.add_clause(vec![if *bit { v } else { -v }]);
            }
            if let (Some(ev), Some(k)) = (&vars.e, k) {
                for (kk, e) in ev[i].iter().enumerate() {
                    let l = *e as i32;
                    solver.add_clause(vec![if kk as u32 == *k { l } else { -l }]);
                }
            }
        }
    }

    fn solve(solver: &mut Solver) -> SolveResult {
        solver.conflict_budget = 200_000;
        solver.solve()
    }

    #[test]
    fn frozen_manifest_is_pinned_and_matches_legacy_facts() {
        let i = inst();
        assert_eq!(i.manifest_sha, FROZEN_MANIFEST_SHA);
        assert_eq!(
            (i.n, i.r, i.lam),
            (13, 2003, 89),
            "legacy: n=13, r=2003, lambda=89"
        );
        assert_eq!(i.reps.len(), 5);
        assert_eq!(i.orbit.len(), 65, "5 representatives x 13 phases");
        assert_eq!(i.targets.len(), 12);
    }

    #[test]
    fn small_field_agrees_with_the_crate_field() {
        let i = inst();
        let mut rng = SplitMix(7);
        let irr = &i.ec.curve.irreducible;
        for _ in 0..200 {
            let (a, b) = (rng.below(1 << 13), rng.below(1 << 13));
            let want = i.ec.fe(a).mul(&i.ec.fe(b), irr).raw_bits()[0];
            assert_eq!(i.ec.gf.mul(a, b), want);
            if a != 0 {
                assert_eq!(i.ec.gf.mul(a, i.ec.gf.inv(a)), 1);
            }
            assert_eq!(i.ec.gf.tr(a) as u8, i.ec.fe(a).trace(irr));
        }
    }

    #[test]
    fn exhaustive_pair_matching_reproduces_the_legacy_structure() {
        let i = inst();
        let (_, fa) = exhaustive(&i, false);
        let (_, fb) = exhaustive(&i, true);
        let a: Vec<&str> = i
            .targets
            .iter()
            .zip(&fa)
            .filter(|(_, f)| !f.is_empty())
            .map(|(t, _)| t.id.as_str())
            .collect();
        assert_eq!(
            a,
            ["b0", "b1", "b2", "b3"],
            "arm A decomposes the phase-0 planted targets only"
        );
        assert!(fb.iter().all(|f| !f.is_empty()), "arm B decomposes all 12");
    }

    #[test]
    fn lambda_coefficients_pass_and_perturbations_fail() {
        let i = inst();
        for seed in 0..6 {
            let (t, spec) = plant(&i, 100 + seed, false);
            let pts: Vec<(u64, u64)> = {
                // recover the signed points by trying the 16 sign vectors
                let mut found = None;
                for s in 0u32..16 {
                    let mut sum: Pt = None;
                    let mut v = Vec::new();
                    for (n, (rep, k)) in spec.iter().enumerate() {
                        let p = Some(i.phase_point(*rep, *k));
                        let q = if (s >> n) & 1 == 1 { i.ec.neg(p) } else { p };
                        v.push(q.unwrap());
                        sum = i.ec.add(sum, q);
                    }
                    if sum == Some(t) {
                        found = Some(v);
                        break;
                    }
                }
                found.unwrap()
            };
            let rel = relation_from_points(&i, &pts).unwrap();
            assert!(
                check_relation(&i, &rel, t).all(),
                "unperturbed relation must pass"
            );
            // perturbed coefficient
            let mut bad = rel.clone();
            bad.coeffs[0] = (bad.coeffs[0] + 1) % i.r;
            let c = check_relation(&i, &bad, t);
            assert!(!c.all() && !c.lambda_log && !c.coeffs_consistent, "{c:?}");
            // perturbed phase of one term
            let mut bad = rel.clone();
            bad.terms[0].phase = (bad.terms[0].phase + 1) % i.n;
            assert!(!check_relation(&i, &bad, t).all(), "wrong phase accepted");
            // flipped sign of one term
            let mut bad = rel.clone();
            bad.terms[1].sign = -bad.terms[1].sign;
            assert!(!check_relation(&i, &bad, t).all(), "wrong sign accepted");
            // wrong target
            let other = if t == i.targets[0].r {
                i.targets[1].r
            } else {
                i.targets[0].r
            };
            assert!(
                !check_relation(&i, &rel, other).all(),
                "wrong target accepted"
            );
            // rank over the representative columns
            let mut tr = U64RankTracker::new(i.r, i.reps.len());
            assert_eq!(tr.insert(rel.coeffs.clone()), 1);
            assert_eq!(
                tr.insert(rel.coeffs.clone()),
                1,
                "a repeated row adds no rank"
            );
        }
    }

    #[test]
    fn phase_selector_accepts_the_planted_witness_and_rejects_a_wrong_phase() {
        let i = inst();
        for form in [
            Form {
                fin: Final::Specialized,
                sym: false,
            },
            Form {
                fin: Final::Line,
                sym: false,
            },
        ] {
            for seed in [1u64, 2] {
                let (t, spec) = plant(&i, 1000 + seed, false);
                // Arm B, planted (rep, phase) pinned: satisfiable and decodes to the witness.
                let (m, vars) = build_model(&i.ec.gf, &i.basis, t, true, form);
                let mut s = m.load(Priority::Core);
                let pins: Vec<(usize, Option<u32>)> =
                    spec.iter().map(|(r, k)| (*r, Some(*k))).collect();
                pin(&mut s, &i, &vars, &pins);
                assert_eq!(
                    solve(&mut s),
                    SolveResult::Sat,
                    "{form:?}: planted witness must satisfy arm B"
                );
                let (xs, detail) = decode(&i.ec.gf, &i.basis, &vars, &s.model());
                for (n, (rep, k)) in spec.iter().enumerate() {
                    assert_eq!(detail[n], (i.reps[*rep].0, *k));
                    assert_eq!(xs[n], i.phase_point(*rep, *k).0);
                }
                assert!(
                    verify_xs(&i, &xs, t).is_some(),
                    "decoded model must verify on the curve"
                );

                // Arm B with one wrong phase: the pinned quadruple is not a decomposition.
                let mut wrong = pins.clone();
                let k0 = wrong[0].1.unwrap();
                wrong[0].1 = Some((k0 + 1 + (seed as u32 % 5)) % i.n);
                let wx: Vec<u64> = wrong
                    .iter()
                    .map(|(r, k)| i.phase_point(*r, k.unwrap()).0)
                    .collect();
                assert!(
                    verify_xs(&i, &wx, t).is_none(),
                    "test setup: wrong quadruple must not decompose"
                );
                let (m, vars) = build_model(&i.ec.gf, &i.basis, t, true, form);
                let mut s = m.load(Priority::Core);
                pin(&mut s, &i, &vars, &wrong);
                assert_eq!(
                    solve(&mut s),
                    SolveResult::Unsat,
                    "{form:?}: a wrong phase must not satisfy arm B"
                );

                // Arm A given the same representatives: phase-shifted witness is rejected.
                let (m, vars) = build_model(&i.ec.gf, &i.basis, t, false, form);
                let mut s = m.load(Priority::Core);
                let reps_only: Vec<(usize, Option<u32>)> =
                    spec.iter().map(|(r, _)| (*r, None)).collect();
                pin(&mut s, &i, &vars, &reps_only);
                assert_eq!(
                    solve(&mut s),
                    SolveResult::Unsat,
                    "{form:?}: arm A must reject a phase-shifted witness"
                );
            }
            // Arm A accepts a phase-0 planted witness.
            let (t, spec) = plant(&i, 77, true);
            let (m, vars) = build_model(&i.ec.gf, &i.basis, t, false, form);
            let mut s = m.load(Priority::Core);
            let reps_only: Vec<(usize, Option<u32>)> =
                spec.iter().map(|(r, _)| (*r, None)).collect();
            pin(&mut s, &i, &vars, &reps_only);
            assert_eq!(
                solve(&mut s),
                SolveResult::Sat,
                "{form:?}: arm A accepts a phase-0 witness"
            );
            let (xs, _) = decode(&i.ec.gf, &i.basis, &vars, &s.model());
            assert!(verify_xs(&i, &xs, t).is_some());
        }
    }

    #[test]
    fn symmetry_breaking_accepts_exactly_the_sorted_order() {
        let i = inst();
        for fin in [Final::Specialized, Final::Line] {
            let form = Form { fin, sym: true };
            // Arm B: order by the abscissa x_i = v^(2^k).
            let (t, spec) = plant(&i, 4242, false);
            let mut by_x: Vec<(usize, u32)> = spec.clone();
            by_x.sort_by_key(|(r, k)| i.phase_point(*r, *k).0);
            for (order, want) in [
                (by_x.clone(), SolveResult::Sat),
                (
                    by_x.iter().rev().cloned().collect::<Vec<_>>(),
                    SolveResult::Unsat,
                ),
            ] {
                let (m, vars) = build_model(&i.ec.gf, &i.basis, t, true, form);
                let mut s = m.load(Priority::Core);
                let pins: Vec<(usize, Option<u32>)> =
                    order.iter().map(|(r, k)| (*r, Some(*k))).collect();
                pin(&mut s, &i, &vars, &pins);
                assert_eq!(solve(&mut s), want, "{form:?}: arm B order {order:?}");
                if want == SolveResult::Sat {
                    let (xs, _) = decode(&i.ec.gf, &i.basis, &vars, &s.model());
                    assert!(xs.windows(2).all(|w| w[0] < w[1]));
                    assert!(verify_xs(&i, &xs, t).is_some());
                }
            }
            // Arm A: order by the coordinate vector read as an integer, bit 0 least significant.
            let (t, spec) = plant(&i, 4243, true);
            let key = |rep: usize| -> u32 {
                coords(&i, i.reps[rep].0)
                    .iter()
                    .enumerate()
                    .map(|(j, b)| (*b as u32) << j)
                    .sum()
            };
            let mut sorted: Vec<usize> = spec.iter().map(|(r, _)| *r).collect();
            sorted.sort_by_key(|r| key(*r));
            for (order, want) in [
                (sorted.clone(), SolveResult::Sat),
                (
                    sorted.iter().rev().cloned().collect::<Vec<_>>(),
                    SolveResult::Unsat,
                ),
            ] {
                let (m, vars) = build_model(&i.ec.gf, &i.basis, t, false, form);
                let mut s = m.load(Priority::Core);
                let pins: Vec<(usize, Option<u32>)> = order.iter().map(|r| (*r, None)).collect();
                pin(&mut s, &i, &vars, &pins);
                assert_eq!(solve(&mut s), want, "{form:?}: arm A order {order:?}");
            }
        }
    }

    #[test]
    fn crate_chain_builds_four_summands_but_cannot_carry_phases() {
        use crypto_lib::cryptanalysis::koblitz_groebner::{
            build_decomposition_system, FieldStructure,
        };
        use crypto_lib::cryptanalysis::pq_groebner_f2::MASK_BITS;
        use crypto_lib::cryptanalysis::semaev_sat::{encode_boolean_system_with, XorEncoding};
        let i = inst();
        let irr = &i.ec.curve.irreducible;
        let st = FieldStructure::new(i.n, irr);
        let basis: Vec<F2mElement> = i.basis.iter().map(|b| i.ec.fe(*b)).collect();
        // A phase-0 planted target, so the library's chain S3(x1,x2,e1), S3(e1,x3,e2),
        // S3(e2,x4,x_R) has a solution in the factor space.
        let (t, spec) = plant(&i, 5, true);
        let sys = build_decomposition_system(&basis, &i.ec.fe(t.0), &F2mElement::one(i.n), 4, &st)
            .expect("the crate builds m = 4 chained systems");
        assert_eq!(sys.n_vars, 4 * basis.len() + 2 * i.n as usize);
        assert!(sys.n_vars <= MASK_BITS);
        let mut enc =
            encode_boolean_system_with(sys.n_vars, &sys.equations, &[], XorEncoding::Native);
        for (n, (rep, _)) in spec.iter().enumerate() {
            let c = coords(&i, i.reps[*rep].0);
            for (j, bit) in c.iter().enumerate() {
                let v = (n * basis.len() + j + 1) as i32;
                enc.solver.add_clause(vec![if *bit { v } else { -v }]);
            }
        }
        enc.solver.conflict_budget = 200_000;
        assert_eq!(
            enc.solver.solve(),
            SolveResult::Sat,
            "library chain accepts the planted witness"
        );
        // Arm B alone has 4*s coordinate bits + 4*n one-hot bits as problem variables.
        assert!(
            4 * basis.len() + 4 * i.n as usize > MASK_BITS,
            "arm B exceeds the 64-bit variable mask"
        );
    }
}
