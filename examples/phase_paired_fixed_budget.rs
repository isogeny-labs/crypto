//! Native paired fixed-budget comparison: Frobenius phase carried as solver
//! unknowns (arm B) against representatives only (arm A), on identical frozen
//! targets, every attempt charged, every relation verified on the curve.
//!
//! Protocol: research/phase_paired_native_20261026/PROTOCOL.md
//! Independent replay: examples/phase_paired_replay.rs
//!
//! Subcommands (see `usage()`): `gate`, `run`, `freeze`; and, for the follow-up
//! that changes only the solver (PROTOCOL_ADDENDUM_2.md), `cms-selftest`,
//! `cms-check` and `cms-run`, which export each instance with
//! `sat::to_dimacs_xor` and run a pinned CryptoMiniSat binary as a child process.
//! Tables and the chart are rendered by the replay example from verified
//! receipts only.
//!
//! The solver is `crypto_lib::cryptanalysis::sat::Solver` (single thread,
//! native XOR rows, deterministic). It is not CryptoMiniSat (only the `cms-*`
//! subcommands run that); the contrast measured by `run` is within one solver.
//! The solver arms read the manifest only. They never open
//! `witness_sealed.json`; only the checker (`gate --sealed`) and the replay do.

use crypto_lib::binary_ecc::curve::{point_add, point_neg, BinaryCurve, BinaryPoint};
use crypto_lib::binary_ecc::f2m::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::koblitz_relation_solver::U64RankTracker;
use crypto_lib::cryptanalysis::sat::{parse_dimacs_xor, to_dimacs_xor, SolveResult, Solver};
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

// ---------------------------------------------------------------------------
// CryptoMiniSat follow-up (PROTOCOL_ADDENDUM_2.md): the same instances, exported
// with `sat::to_dimacs_xor`, solved by a pinned external binary run as a child
// process. Nothing above this block is changed: the encoder, the instance, the
// group check and the receipt writer are the ones the native study used.
// ---------------------------------------------------------------------------

const CMS_SCHEMA: &str = "phase_paired_cms/receipt/v1";
const CMS_CONTROLS_SCHEMA: &str = "phase_paired_cms/controls/v1";
/// The pinned build, fixed in PROTOCOL_ADDENDUM_2.md before any measurement.
const CMS_TAG: &str = "release/v5.16.0";
const CMS_COMMIT: &str = "2bf89cfb3e126e559d472388486327813f735f7e";
const CADICAL_COMMIT: &str = "818c9562f114b315a9246ced943b66b60b38e8fb";
const CADIBACK_COMMIT: &str = "445ddc2f2b5e54421af64e0652c2eb8444175322";
const CMS_BINARY_SHA: &str = "8e1e539976b1fa8bff2b1e30df5c200c9a03ee343c634eadc0152e1b6cecd8f7";

/// CryptoMiniSat's `--random` seed of repetition `rep`. CryptoMiniSat cannot
/// take the native study's saved-phase scrambling, so the same per-repetition
/// seed material (`phase_seed`) feeds its own seed instead; the arms of one
/// repetition share it.
fn cms_seed(rep: u64) -> u32 {
    (phase_seed(rep) & 0x7fff_ffff) as u32
}

/// What the parser reads from CryptoMiniSat's standard output.
#[derive(Debug, Default, PartialEq)]
struct CmsOut {
    /// The text after `s `: SATISFIABLE, UNSATISFIABLE or INDETERMINATE.
    status: Option<String>,
    /// Literals of the `v` lines, without the terminating 0.
    model: Vec<i32>,
    model_closed: bool,
    /// Every `c conflicts : N` line, in order.
    conflicts: Vec<u64>,
    decisions: Option<u64>,
    propagations: Option<u64>,
    restarts: Option<u64>,
    /// `c Total time (this thread)`, CryptoMiniSat's own clock.
    total_time_s: Option<f64>,
    /// A `v` line with a token that is not an integer.
    malformed: bool,
}

/// The first token after the colon of a `c <label> : <value> ...` statistics line.
fn stat_value<'a>(line: &'a str, label: &str) -> Option<&'a str> {
    let rest = line.strip_prefix("c ")?;
    let (l, r) = rest.split_once(':')?;
    if l.trim() == label {
        r.split_whitespace().next()
    } else {
        None
    }
}

/// A statistics value as CryptoMiniSat prints it: an integer, or one rounded
/// with a K, M or G suffix (propagations are). Rounded values are approximate;
/// they feed only the exploratory columns, never a status or a budget.
fn parse_scaled(tok: &str) -> Option<u64> {
    let (num, mult) = match tok.chars().last()? {
        'K' => (&tok[..tok.len() - 1], 1_000.0),
        'M' => (&tok[..tok.len() - 1], 1_000_000.0),
        'G' => (&tok[..tok.len() - 1], 1_000_000_000.0),
        _ => (tok, 1.0),
    };
    num.parse::<f64>().ok().map(|v| (v * mult).round() as u64)
}

fn parse_cms_output(text: &str) -> CmsOut {
    let mut o = CmsOut::default();
    for line in text.lines() {
        if let Some(s) = line.strip_prefix("s ") {
            o.status = Some(s.trim().to_string());
        } else if let Some(v) = line.strip_prefix("v ") {
            for tok in v.split_whitespace() {
                match tok.parse::<i32>() {
                    Ok(0) => o.model_closed = true,
                    Ok(l) => o.model.push(l),
                    Err(_) => o.malformed = true,
                }
            }
        } else if line.starts_with("c ") {
            if let Some(x) = stat_value(line, "conflicts") {
                if let Ok(n) = x.parse() {
                    o.conflicts.push(n);
                }
            } else if let Some(x) = stat_value(line, "decisions") {
                o.decisions = parse_scaled(x);
            } else if let Some(x) = stat_value(line, "propagations") {
                o.propagations = parse_scaled(x);
            } else if let Some(x) = stat_value(line, "restarts") {
                o.restarts = parse_scaled(x);
            } else if let Some(x) = stat_value(line, "Total time (this thread)") {
                o.total_time_s = x.parse().ok();
            }
        }
    }
    o
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CmsVerdict {
    Sat,
    Unsat,
    Unknown,
}

/// The answer, from the status line and the exit code together (10, 20 and 0
/// are CryptoMiniSat's conventions); any disagreement is an error.
fn cms_verdict(o: &CmsOut, exit: Option<i32>) -> Result<CmsVerdict, String> {
    match (o.status.as_deref(), exit) {
        (Some("SATISFIABLE"), Some(10)) => Ok(CmsVerdict::Sat),
        (Some("UNSATISFIABLE"), Some(20)) => Ok(CmsVerdict::Unsat),
        (Some("INDETERMINATE"), Some(15)) => Ok(CmsVerdict::Unknown),
        (s, e) => Err(format!("status line {s:?} with exit code {e:?}")),
    }
}

/// The conflict count CryptoMiniSat reports (the last statistics line).
fn cms_conflicts(o: &CmsOut) -> Result<u64, String> {
    o.conflicts
        .last()
        .copied()
        .ok_or_else(|| "no `c conflicts` line in the output".to_string())
}

/// The assignment of variables `1..=nv` from the `v` lines; every variable
/// exactly once, terminated by 0.
fn cms_model(o: &CmsOut, nv: u32) -> Result<Vec<bool>, String> {
    if o.malformed || !o.model_closed {
        return Err("model lines are malformed or not closed by 0".into());
    }
    let mut val: Vec<Option<bool>> = vec![None; nv as usize];
    for &l in &o.model {
        let v = l.unsigned_abs() as usize;
        if v == 0 || v > nv as usize || val[v - 1].replace(l > 0).is_some() {
            return Err(format!("model literal {l} is out of range or repeated"));
        }
    }
    val.into_iter()
        .enumerate()
        .map(|(i, x)| x.ok_or_else(|| format!("variable {} missing from the model", i + 1)))
        .collect()
}

/// A CNF + XOR instance read from extended DIMACS with CryptoMiniSat's parity
/// convention (`x a b c 0` means a ^ b ^ c = 1; each negated literal flips the
/// right-hand side). Parsed here, not by the crate, so that it is an
/// independent reading of the file that was handed to the solver.
struct Cnf {
    nv: u32,
    clauses: Vec<Vec<i32>>,
    xors: Vec<(Vec<u32>, bool)>,
}

fn parse_cnf_xor(text: &str) -> Result<Cnf, String> {
    let mut nv = None;
    let (mut clauses, mut xors) = (Vec::new(), Vec::new());
    for (n, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('c') {
            continue;
        }
        if let Some(h) = line.strip_prefix("p cnf ") {
            let mut it = h.split_whitespace();
            nv = Some(
                it.next()
                    .and_then(|v| v.parse::<u32>().ok())
                    .ok_or_else(|| format!("line {}: bad header", n + 1))?,
            );
            continue;
        }
        let (is_xor, body) = match line.strip_prefix('x') {
            Some(b) => (true, b),
            None => (false, line),
        };
        let mut lits = Vec::new();
        for t in body.split_whitespace() {
            let l: i32 = t
                .parse()
                .map_err(|_| format!("line {}: bad literal", n + 1))?;
            if l == 0 {
                break;
            }
            lits.push(l);
        }
        if is_xor {
            let neg = lits.iter().filter(|l| **l < 0).count();
            let vars: Vec<u32> = lits.iter().map(|l| l.unsigned_abs()).collect();
            let distinct: BTreeSet<u32> = vars.iter().copied().collect();
            if distinct.len() != vars.len() {
                return Err(format!("line {}: repeated variable in an XOR row", n + 1));
            }
            xors.push((vars, neg % 2 == 0));
        } else {
            clauses.push(lits);
        }
    }
    Ok(Cnf {
        nv: nv.ok_or("missing `p cnf` header")?,
        clauses,
        xors,
    })
}

fn cnf_satisfied(c: &Cnf, m: &[bool]) -> bool {
    let val = |l: i32| m[l.unsigned_abs() as usize - 1] == (l > 0);
    c.clauses.iter().all(|cl| cl.iter().any(|l| val(*l)))
        && c.xors
            .iter()
            .all(|(vs, rhs)| vs.iter().fold(false, |p, v| p ^ m[*v as usize - 1]) == *rhs)
}

/// Append a clause to an extended-DIMACS text and keep the header count right.
fn append_clause(dimacs: &str, lits: &[i32]) -> String {
    let (head, rest) = dimacs.split_once('\n').unwrap_or((dimacs, ""));
    let mut p: Vec<&str> = head.split_whitespace().collect();
    let count: usize = p.last().and_then(|v| v.parse().ok()).expect("p cnf V C");
    let new = (count + 1).to_string();
    *p.last_mut().unwrap() = &new;
    let mut out = format!("{}\n{rest}", p.join(" "));
    for l in lits {
        out.push_str(&format!("{l} "));
    }
    out.push_str("0\n");
    out
}

struct ChildRun {
    exit_code: Option<i32>,
    signal: Option<i32>,
    user_ns: u64,
    sys_ns: u64,
    wall_ns: u64,
    maxrss_kb: u64,
    killed: bool,
}

/// Run `bin args` with stdout and stderr redirected to files and reap it with
/// `wait4`, which returns the child's own resource usage (user + system CPU).
/// The child is killed only if its wall time passes `kill_wall_s`, a backstop
/// far above the solver's own CPU limit.
fn run_child(
    bin: &str,
    args: &[String],
    out_path: &std::path::Path,
    err_path: &std::path::Path,
    kill_wall_s: f64,
) -> Result<ChildRun, String> {
    let out = fs::File::create(out_path).map_err(|e| format!("{}: {e}", out_path.display()))?;
    let err = fs::File::create(err_path).map_err(|e| format!("{}: {e}", err_path.display()))?;
    let t0 = Instant::now();
    let mut child = std::process::Command::new(bin)
        .args(args)
        .stdin(std::process::Stdio::null())
        .stdout(out)
        .stderr(err)
        .spawn()
        .map_err(|e| format!("spawn {bin}: {e}"))?;
    let pid = child.id() as libc::pid_t;
    let mut killed = false;
    loop {
        let mut status: libc::c_int = 0;
        // SAFETY: a zeroed rusage is a valid out-parameter for wait4.
        let mut ru: libc::rusage = unsafe { std::mem::zeroed() };
        // SAFETY: `pid` is our own unreaped child.
        let r = unsafe { libc::wait4(pid, &mut status, libc::WNOHANG, &mut ru) };
        if r == pid {
            let tv = |t: libc::timeval| t.tv_sec as u64 * 1_000_000_000 + t.tv_usec as u64 * 1_000;
            return Ok(ChildRun {
                exit_code: libc::WIFEXITED(status).then(|| libc::WEXITSTATUS(status)),
                signal: libc::WIFSIGNALED(status).then(|| libc::WTERMSIG(status)),
                user_ns: tv(ru.ru_utime),
                sys_ns: tv(ru.ru_stime),
                wall_ns: t0.elapsed().as_nanos() as u64,
                maxrss_kb: ru.ru_maxrss as u64,
                killed,
            });
        }
        if r < 0 {
            return Err(format!("wait4: {}", std::io::Error::last_os_error()));
        }
        if !killed && t0.elapsed().as_secs_f64() > kill_wall_s {
            let _ = child.kill();
            killed = true;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[derive(Clone, Debug)]
struct CmsConfig {
    bin: String,
    conflicts: u64,
    /// CPU backstop passed as `--maxtime`; the primary budget is `conflicts`.
    maxtime_s: f64,
    scratch: std::path::PathBuf,
    keep_dimacs: bool,
}

/// One child invocation and what was read from it.
struct CmsCall {
    run: ChildRun,
    out: CmsOut,
    stdout_sha: String,
    tail_sha: String,
    stdout_bytes: usize,
}

fn cms_args(max_conf: u64, max_time: f64, seed: u32, cnf: &std::path::Path) -> Vec<String> {
    vec![
        "--threads".into(),
        "1".into(),
        "--maxconfl".into(),
        max_conf.to_string(),
        "--maxtime".into(),
        format!("{max_time:.3}"),
        "--random".into(),
        seed.to_string(),
        "--verb".into(),
        "1".into(),
        cnf.display().to_string(),
    ]
}

fn cms_call(
    cfg: &CmsConfig,
    cnf: &std::path::Path,
    tag: &str,
    max_conf: u64,
    max_time: f64,
    seed: u32,
) -> Result<CmsCall, String> {
    let (op, ep) = (
        cfg.scratch.join(format!("{tag}.out")),
        cfg.scratch.join(format!("{tag}.err")),
    );
    let run = run_child(
        &cfg.bin,
        &cms_args(max_conf, max_time, seed, cnf),
        &op,
        &ep,
        max_time * 1.5 + 30.0,
    )?;
    let bytes = fs::read(&op).map_err(|e| e.to_string())?;
    let text = String::from_utf8_lossy(&bytes).into_owned();
    let tail = &bytes[bytes.len().saturating_sub(4096)..];
    let call = CmsCall {
        run,
        out: parse_cms_output(&text),
        stdout_sha: sha_hex(&bytes),
        tail_sha: sha_hex(tail),
        stdout_bytes: bytes.len(),
    };
    if !cfg.keep_dimacs {
        let _ = fs::remove_file(&op);
        let _ = fs::remove_file(&ep);
    }
    Ok(call)
}

/// Check the binary's hash and that its version banner names the pinned commits.
fn cms_pinned(bin: &str) -> Result<(String, String), String> {
    let bytes = fs::read(bin).map_err(|e| format!("read {bin}: {e}"))?;
    let sha = sha_hex(&bytes);
    if sha != CMS_BINARY_SHA {
        return Err(format!(
            "binary SHA-256 {sha} differs from the pinned {CMS_BINARY_SHA}"
        ));
    }
    let banner = cmd_out(bin, &["--version"]);
    for want in [CMS_COMMIT, CADICAL_COMMIT, CADIBACK_COMMIT] {
        if !banner.contains(want) {
            return Err(format!("version banner does not name commit {want}"));
        }
    }
    Ok((sha, banner.lines().take(4).collect::<Vec<_>>().join(" | ")))
}

/// One attempt through the external solver: build and export the instance,
/// run the child (again after a rejected tuple, within the remaining budget),
/// group-check every model. Returns the receipt fields shared with the native
/// attempts plus a `cms` block.
fn run_cms_attempt(
    inst: &Instance,
    cfg: &CmsConfig,
    tag: &str,
    target: (u64, u64),
    phases: bool,
    form: Form,
    seed: u32,
) -> (AttemptOut, Value) {
    let (w0, c0) = (Instant::now(), cpu_ns());
    let (model, vars) = build_model(&inst.ec.gf, &inst.basis, target, phases, form);
    let mut dimacs = to_dimacs_xor(&model.load(Priority::Core));
    let (dimacs_sha, dimacs_bytes) = (sha_hex(dimacs.as_bytes()), dimacs.len());
    let cnf_path = cfg.scratch.join(format!("{tag}.cnf"));
    let wrote = fs::write(&cnf_path, &dimacs);
    let (w1, c1) = (Instant::now(), cpu_ns());
    let (mut child_cpu, mut child_wall, mut user, mut sys) = (0u64, 0u64, 0u64, 0u64);
    let (mut vw, mut vc) = (0u64, 0u64);
    let (mut conflicts, mut decisions, mut props, mut restarts) = (0u64, 0u64, 0u64, 0u64);
    let (mut rejected, mut calls) = (0u64, 0u64);
    let (mut found, mut summands) = (None, None);
    let mut call_info: Vec<Value> = Vec::new();
    let mut rss = 0u64;
    let mut error: Option<String> = wrote
        .err()
        .map(|e| format!("write {}: {e}", cnf_path.display()));
    let status: &'static str = 'attempt: loop {
        if error.is_some() {
            break 'attempt "error";
        }
        if conflicts >= cfg.conflicts {
            break 'attempt "budget";
        }
        let left_t = cfg.maxtime_s - child_cpu as f64 / 1e9;
        if left_t <= 0.0 {
            break 'attempt "deadline";
        }
        calls += 1;
        let call = match cms_call(
            cfg,
            &cnf_path,
            &format!("{tag}.{calls}"),
            cfg.conflicts - conflicts,
            left_t,
            seed,
        ) {
            Ok(c) => c,
            Err(e) => {
                error = Some(e);
                break 'attempt "error";
            }
        };
        let cpu = call.run.user_ns + call.run.sys_ns;
        child_cpu += cpu;
        user += call.run.user_ns;
        sys += call.run.sys_ns;
        child_wall += call.run.wall_ns;
        rss = rss.max(call.run.maxrss_kb);
        decisions += call.out.decisions.unwrap_or(0);
        props += call.out.propagations.unwrap_or(0);
        restarts += call.out.restarts.unwrap_or(0);
        call_info.push(json!({
            "exit_code": call.run.exit_code, "signal": call.run.signal,
            "killed_at_wall_backstop": call.run.killed,
            "status_line": call.out.status, "child_cpu_ns": cpu,
            "child_wall_ns": call.run.wall_ns,
            "cms_reported_conflicts": call.out.conflicts.last(),
            "cms_reported_total_time_s": call.out.total_time_s,
            "stdout_bytes": call.stdout_bytes, "stdout_sha256": call.stdout_sha,
            "stdout_tail4096_sha256": call.tail_sha,
        }));
        if call.run.killed {
            error = Some("child killed at the wall backstop".into());
            break 'attempt "error";
        }
        let verdict = match cms_verdict(&call.out, call.run.exit_code) {
            Ok(v) => v,
            Err(e) => {
                error = Some(e);
                break 'attempt "error";
            }
        };
        match cms_conflicts(&call.out) {
            Ok(n) => conflicts += n,
            Err(e) => {
                error = Some(e);
                break 'attempt "error";
            }
        }
        match verdict {
            CmsVerdict::Unsat => break 'attempt "unsat",
            CmsVerdict::Unknown => {
                break 'attempt if conflicts >= cfg.conflicts {
                    "budget"
                } else {
                    "deadline"
                }
            }
            CmsVerdict::Sat => {
                let (a1, b1) = (Instant::now(), cpu_ns());
                let sol = match cms_model(&call.out, model.nv) {
                    Ok(s) => s,
                    Err(e) => {
                        error = Some(e);
                        break 'attempt "error";
                    }
                };
                // The model must satisfy the very file that was handed over.
                match parse_cnf_xor(&dimacs) {
                    Ok(c) if c.nv == model.nv && cnf_satisfied(&c, &sol) => {}
                    _ => {
                        error = Some("model does not satisfy the exported instance".into());
                        break 'attempt "error";
                    }
                }
                let (xs, detail) = decode(&inst.ec.gf, &inst.basis, &vars, &sol);
                let ok = verify_xs(inst, &xs, target);
                vw += a1.elapsed().as_nanos() as u64;
                vc += cpu_ns() - b1;
                if let Some(pts) = ok {
                    found = Some(pts);
                    summands = Some(detail);
                    break 'attempt "verified";
                }
                rejected += 1;
                // Block this (coordinates, phases) tuple and run again within
                // the remaining budget, as the native attempts do.
                let mut block: Vec<i32> = Vec::new();
                let core = vars
                    .c
                    .iter()
                    .flatten()
                    .chain(vars.e.iter().flatten().flatten());
                for v in core {
                    block.push(if sol[*v as usize - 1] {
                        -(*v as i32)
                    } else {
                        *v as i32
                    });
                }
                dimacs = append_clause(&dimacs, &block);
                if let Err(e) = fs::write(&cnf_path, &dimacs) {
                    error = Some(e.to_string());
                    break 'attempt "error";
                }
            }
        }
    };
    let (wend, cend) = (Instant::now(), cpu_ns());
    if !cfg.keep_dimacs {
        let _ = fs::remove_file(&cnf_path);
    }
    let parent_cpu = cend - c0;
    let out = AttemptOut {
        status,
        conflicts,
        decisions,
        propagations: props,
        restarts,
        learnt: 0,
        xor_propagations: 0,
        xor_conflicts: 0,
        word_ops: 0,
        rejected,
        solver_calls: calls,
        found,
        summands,
        size: model.size(),
        build_wall_ns: (w1 - w0).as_nanos() as u64,
        build_cpu_ns: c1 - c0,
        solve_wall_ns: child_wall,
        solve_cpu_ns: child_cpu,
        verify_wall_ns: vw,
        verify_cpu_ns: vc,
        total_wall_ns: (wend - w0).as_nanos() as u64,
        total_cpu_ns: parent_cpu + child_cpu,
    };
    let info = json!({
        "cms_seed": seed, "dimacs_sha256": dimacs_sha, "dimacs_bytes": dimacs_bytes,
        "child_user_ns": user, "child_sys_ns": sys, "child_cpu_ns": child_cpu,
        "parent_cpu_ns": parent_cpu, "child_maxrss_kb": rss,
        "calls": call_info, "error": error,
    });
    (out, info)
}

/// Receipt for one attempt through CryptoMiniSat: the native fields, the solver
/// id and the `cms` block.
fn cms_attempt_value(
    item: &Value,
    inst: &Instance,
    out: &AttemptOut,
    info: Value,
    seq: u64,
    seed: u32,
) -> Value {
    let mut v = attempt_value(item, inst, out, seq, seed as u64);
    let o = v.as_object_mut().unwrap();
    o.remove("phase_seed");
    o.insert("solver".into(), json!("cryptominisat5"));
    o.insert("cms".into(), info);
    v
}

/// Stage rule (PROTOCOL_ADDENDUM_2.md): repetition 1 always starts; a later
/// repetition runs in full if the remaining CPU allowance is at least `guard`
/// times the dearest completed full repetition, with arm A only if it is at
/// least `guard` times the dearest completed arm A, and not at all otherwise.
fn rep_mode(
    rep: u64,
    remaining: f64,
    guard: f64,
    full_cost: Option<f64>,
    a_cost: Option<f64>,
) -> &'static str {
    if rep == 1 {
        return "full";
    }
    match (full_cost, a_cost) {
        (Some(f), _) if remaining >= guard * f => "full",
        (_, Some(a)) if remaining >= guard * a => "armA",
        _ => "none",
    }
}

fn cms_cpu_sum(recs: &[Value]) -> f64 {
    recs.iter()
        .map(|r| {
            if r["status"] == "skipped" {
                0.0
            } else {
                r["total_cpu_ns"]
                    .as_u64()
                    .or_else(|| r["cpu_ns"].as_u64())
                    .unwrap_or(0) as f64
                    / 1e9
            }
        })
        .sum()
}

fn cms_config_from_args(a: &HashMap<String, String>) -> Result<CmsConfig, String> {
    let bin = a
        .get("cms")
        .cloned()
        .or_else(|| std::env::var("PHASE_PAIRED_CMS").ok())
        .ok_or("--cms BINARY (or PHASE_PAIRED_CMS) is required")?;
    let scratch = std::path::PathBuf::from(a.get("scratch").ok_or("--scratch DIR is required")?);
    fs::create_dir_all(&scratch).map_err(|e| e.to_string())?;
    Ok(CmsConfig {
        bin,
        conflicts: a
            .get("conflict-budget")
            .map(|v| v.parse().expect("conflict-budget"))
            .unwrap_or(3_000_000),
        maxtime_s: a
            .get("maxtime")
            .map(|v| v.parse().expect("maxtime"))
            .unwrap_or(120.0),
        scratch,
        keep_dimacs: a.contains_key("keep-dimacs"),
    })
}

fn cms_run_cmd(a: &HashMap<String, String>) -> i32 {
    let inst = instance_from_args(a);
    let cfg = match cms_config_from_args(a) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return 64;
        }
    };
    let (cms_sha, banner) = match cms_pinned(&cfg.bin) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("binary refused: {e}");
            return 3;
        }
    };
    let out_path = a.get("out").expect("--out receipts.jsonl").clone();
    let forms: Vec<String> = a
        .get("forms")
        .cloned()
        .unwrap_or_else(|| "subgroup_xor".into())
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
        .unwrap_or(3600.0);
    let cpu_prior: f64 = a
        .get("cpu-prior")
        .map(|v| v.parse().unwrap())
        .unwrap_or(0.0);
    let guard: f64 = a
        .get("stage-guard")
        .map(|v| v.parse().unwrap())
        .unwrap_or(1.1);
    // The worst one attempt can cost: the CPU backstop plus a margin for parsing and overshoot.
    let attempt_worst = cfg.maxtime_s * 1.05 + 3.0;
    let controls: Vec<Value> = a.get("controls").map(|p| read_jsonl(p)).unwrap_or_default();
    let controls_cpu = cms_cpu_sum(&controls);
    let only: Option<Vec<String>> = a
        .get("targets")
        .map(|v| v.split(',').map(String::from).collect());
    let prior = read_jsonl(&out_path);
    let producer_sha = sha_hex(THIS_SOURCE);
    let mut done = BTreeSet::new();
    let mut seq = 0u64;
    let mut modes: BTreeMap<u64, String> = BTreeMap::new();
    if let Some(h) = prior.first() {
        if !a.contains_key("resume") {
            eprintln!("{out_path} exists; pass --resume to append");
            return 3;
        }
        for (k, want) in [
            ("manifest_sha256", json!(inst.manifest_sha)),
            ("producer_sha256", json!(producer_sha)),
            ("cms_binary_sha256", json!(cms_sha)),
            ("conflict_budget", json!(cfg.conflicts)),
            ("maxtime_cpu_s", json!(cfg.maxtime_s)),
            ("forms", json!(forms)),
            ("cpu_cap_s", json!(cpu_cap)),
            ("cpu_prior_s", json!(cpu_prior)),
        ] {
            if h[k] != want {
                eprintln!("resume refused: header {k} differs ({} vs {want})", h[k]);
                return 3;
            }
        }
        for r in &prior {
            match r["type"].as_str() {
                Some("attempt") => {
                    seq += 1;
                    if r["status"] != "skipped" {
                        done.insert((
                            r["rep"].as_u64().unwrap(),
                            r["form"].as_str().unwrap().to_string(),
                            r["target_id"].as_str().unwrap().to_string(),
                            r["arm"].as_str().unwrap().to_string(),
                        ));
                    }
                }
                Some("stage") => {
                    modes.insert(
                        r["rep"].as_u64().unwrap(),
                        r["mode"].as_str().unwrap().to_string(),
                    );
                }
                _ => {}
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
            "type": "header", "schema": CMS_SCHEMA,
            "manifest_sha256": inst.manifest_sha, "producer_sha256": producer_sha,
            "forms": forms, "reps_planned": reps,
            "conflict_budget": cfg.conflicts, "maxtime_cpu_s": cfg.maxtime_s,
            "solver": "cryptominisat5 5.16.0, CLI child process, --threads 1, defaults otherwise",
            "cms_tag": CMS_TAG, "cms_commit": CMS_COMMIT,
            "cadical_commit": CADICAL_COMMIT, "cadiback_commit": CADIBACK_COMMIT,
            "cms_binary_sha256": cms_sha, "cms_banner": banner,
            "cms_args": "--threads 1 --maxconfl <budget left> --maxtime <cpu left> --random <cms_seed> --verb 1 <file.cnf>",
            "cms_seeds": (1..=reps).map(cms_seed).collect::<Vec<_>>(),
            "branch_priority": "not applicable (CryptoMiniSat chooses its own branching)",
            "arms": {"A": "representatives only", "B": "one-hot Frobenius phases"},
            "budget_clock": "conflicts (primary, CryptoMiniSat's own count); --maxtime is a CPU backstop",
            "cpu_clock": "child user+system CPU from wait4, plus the parent's CLOCK_PROCESS_CPUTIME_ID for build, export and verification",
            "schedule": "rep-major, formulation, target order; pair i runs A,B if (i+rep-1) even else B,A",
            "cpu_cap_s": cpu_cap, "cpu_prior_s": cpu_prior, "controls_cpu_s": controls_cpu,
            "stage_guard": guard,
            "meta": meta,
        }));
    } else {
        emit(&json!({"type": "resume", "reps_planned": reps, "meta": meta}));
    }
    let mut capped: Option<String> = None;
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
        let rep = key.0;
        let all: Vec<Value> = read_jsonl(&out_path);
        let spent = cpu_prior + controls_cpu + cms_cpu_sum(&all);
        if item["pair"] == 0 && item["position"] == 0 && !modes.contains_key(&rep) {
            // Per-repetition planning, from the receipts so far.
            let cost = |r: u64, arm: Option<&str>| -> (f64, usize) {
                let rows: Vec<&Value> = all
                    .iter()
                    .filter(|x| {
                        x["type"] == "attempt"
                            && x["status"] != "skipped"
                            && x["rep"] == r
                            && arm.is_none_or(|a| x["arm"] == a)
                    })
                    .collect();
                (
                    rows.iter()
                        .map(|x| x["total_cpu_ns"].as_u64().unwrap_or(0) as f64 / 1e9)
                        .sum(),
                    rows.len(),
                )
            };
            let n_targets = inst.targets.len();
            let full: Vec<f64> = (1..rep)
                .filter(|r| cost(*r, None).1 == 2 * n_targets)
                .map(|r| cost(r, None).0)
                .collect();
            let a_only: Vec<f64> = (1..rep)
                .filter(|r| cost(*r, Some("A")).1 == n_targets)
                .map(|r| cost(r, Some("A")).0)
                .collect();
            let mx = |v: &[f64]| {
                v.iter()
                    .cloned()
                    .fold(None::<f64>, |m, x| Some(m.map_or(x, |y| y.max(x))))
            };
            let (fc, ac) = (mx(&full), mx(&a_only));
            let remaining = cpu_cap - spent;
            let mode = if capped.is_some() {
                "none"
            } else {
                rep_mode(rep, remaining, guard, fc, ac)
            };
            modes.insert(rep, mode.to_string());
            emit(&json!({"type": "stage", "rep": rep, "mode": mode,
                         "remaining_cpu_s": (remaining * 1000.0).round() / 1000.0,
                         "dearest_full_rep_cpu_s": fc, "dearest_arm_a_cpu_s": ac,
                         "guard": guard}));
        }
        let mode = modes.get(&rep).cloned().unwrap_or_else(|| "full".into());
        if item["position"] == 0 && capped.is_none() {
            let arms = if mode == "armA" { 1.0 } else { 2.0 };
            if spent + arms * attempt_worst > cpu_cap {
                capped = Some(format!(
                    "cpu cap {cpu_cap} s: {spent:.1} s spent, a pair can cost {:.0} s",
                    arms * attempt_worst
                ));
            }
        }
        let skip_reason = if let Some(c) = &capped {
            Some(c.clone())
        } else if mode == "none" {
            Some("stage rule: the remaining allowance does not fit another repetition".to_string())
        } else if mode == "armA" && item["arm"] == "B" {
            Some("stage rule: arm A only in this repetition".to_string())
        } else {
            None
        };
        if let Some(reason) = skip_reason {
            let mut rec = item.clone();
            rec["type"] = json!("attempt");
            rec["seq"] = json!(seq);
            rec["status"] = json!("skipped");
            rec["reason"] = json!(reason);
            // Complete enough for the replay to check the slot against the manifest.
            let tt = inst
                .targets
                .iter()
                .find(|t| t.id == item["target_id"].as_str().unwrap())
                .unwrap();
            rec["target"] = json!([hx(tt.r.0), hx(tt.r.1)]);
            rec["phases"] = json!(item["arm"] == "B");
            rec["n"] = json!(inst.n);
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
        let seed = cms_seed(rep);
        let tag = format!("r{rep}_{}_{}", item["arm"].as_str().unwrap(), t.id);
        let (res, info) = run_cms_attempt(&inst, &cfg, &tag, t.r, item["arm"] == "B", fin, seed);
        emit(&cms_attempt_value(&item, &inst, &res, info, seq, seed));
        seq += 1;
        // Progress lines carry no timing, as in the native run.
        println!(
            "rep {} {:14} pair {:2} {} {} {}",
            item["rep"],
            item["form"].as_str().unwrap(),
            item["pair"],
            item["arm"].as_str().unwrap(),
            t.id,
            res.status
        );
        if res.status == "error" {
            eprintln!("instrument error on {tag}: the run stops (stop condition)");
            emit(&json!({"type": "end", "capped": capped, "stopped": "error"}));
            return 2;
        }
    }
    let total = cms_cpu_sum(&read_jsonl(&out_path));
    emit(&json!({"type": "end", "capped": capped,
                 "total_attempt_cpu_s": (total * 1000.0).round() / 1000.0}));
    0
}

// ---- controls: tiny known instances, export fidelity, positive control ----

/// A random CNF + XOR instance on `n` variables, with its ground truth by
/// brute force (no solver involved).
fn tiny_instance(rng: &mut SplitMix, n: u32) -> (Vec<Vec<i32>>, Vec<(Vec<u32>, bool)>, bool) {
    let mut clauses = Vec::new();
    for _ in 0..(4 * n + rng.below(n as u64) as u32) {
        let mut vs = BTreeSet::new();
        while vs.len() < 3 {
            vs.insert(1 + rng.below(n as u64) as i32);
        }
        clauses.push(
            vs.into_iter()
                .map(|v| if rng.below(2) == 1 { v } else { -v })
                .collect(),
        );
    }
    let mut xors = Vec::new();
    for _ in 0..(2 + rng.below(3)) {
        let k = 3 + rng.below(4) as usize;
        let mut vs = BTreeSet::new();
        while vs.len() < k {
            vs.insert(1 + rng.below(n as u64) as u32);
        }
        xors.push((vs.into_iter().collect::<Vec<u32>>(), rng.below(2) == 1));
    }
    let c = Cnf {
        nv: n,
        clauses,
        xors,
    };
    let sat = (0u32..(1 << n)).any(|m| {
        let model: Vec<bool> = (0..n).map(|i| (m >> i) & 1 == 1).collect();
        cnf_satisfied(&c, &model)
    });
    (c.clauses, c.xors, sat)
}

fn solver_from_lists(n: u32, clauses: &[Vec<i32>], xors: &[(Vec<u32>, bool)]) -> Solver {
    let mut s = Solver::new(n);
    for c in clauses {
        s.add_clause(c.clone());
    }
    for (v, r) in xors {
        s.add_xor(v, *r);
    }
    s
}

/// `cms-selftest`: the binary against brute force and the native solver on tiny
/// instances, before any frozen target is touched.
fn cms_selftest_cmd(a: &HashMap<String, String>) -> i32 {
    let cfg = match cms_config_from_args(a) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return 64;
        }
    };
    let (cms_sha, banner) = match cms_pinned(&cfg.bin) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("binary refused: {e}");
            return 3;
        }
    };
    let out_path = a.get("out").expect("--out controls.jsonl").clone();
    let count: usize = a.get("count").map(|v| v.parse().unwrap()).unwrap_or(100);
    let mut out = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&out_path)
        .expect("open controls");
    let mut emit = |v: &Value| {
        writeln!(out, "{}", serde_json::to_string(v).unwrap()).unwrap();
        out.flush().unwrap();
    };
    emit(
        &json!({"type": "header", "schema": CMS_CONTROLS_SCHEMA, "cms_binary_sha256": cms_sha,
                 "cms_banner": banner, "producer_sha256": sha_hex(THIS_SOURCE)}),
    );
    let mut rng = SplitMix(0x434d_5353_4c46_5453);
    let (mut agree, mut sat_n, mut unsat_n, mut bad, mut cpu_total) = (0, 0, 0, 0, 0u64);
    for i in 0..count {
        let n = 10 + rng.below(5) as u32;
        let (clauses, xors, truth) = tiny_instance(&mut rng, n);
        let text = to_dimacs_xor(&solver_from_lists(n, &clauses, &xors));
        let native = {
            let mut s = parse_dimacs_xor(&text).expect("crate parser");
            s.solve() == SolveResult::Sat
        };
        let path = cfg.scratch.join(format!("selftest_{i}.cnf"));
        fs::write(&path, &text).unwrap();
        let call =
            cms_call(&cfg, &path, &format!("selftest_{i}"), 100_000, 30.0, 1).expect("child");
        let _ = fs::remove_file(&path);
        let cpu = call.run.user_ns + call.run.sys_ns;
        cpu_total += cpu;
        let verdict = cms_verdict(&call.out, call.run.exit_code);
        let (cms_sat, model_ok) = match verdict {
            Ok(CmsVerdict::Sat) => {
                let ok = cms_model(&call.out, n)
                    .map(|m| {
                        parse_cnf_xor(&text)
                            .map(|c| cnf_satisfied(&c, &m))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false);
                (Some(true), ok)
            }
            Ok(CmsVerdict::Unsat) => (Some(false), true),
            _ => (None, false),
        };
        let pass = cms_sat == Some(truth) && native == truth && model_ok;
        agree += pass as u32;
        bad += !pass as u32;
        if truth {
            sat_n += 1
        } else {
            unsat_n += 1
        }
        emit(
            &json!({"type": "selftest", "i": i, "vars": n, "truth_sat": truth,
                     "native_sat": native, "cms_sat": cms_sat, "model_satisfies_file": model_ok,
                     "pass": pass, "cpu_ns": cpu, "dimacs_sha256": sha_hex(text.as_bytes())}),
        );
    }
    // The parity convention on hand-written files.
    for (name, text, want) in [
        ("parity_sat", "p cnf 3 3\nx 1 2 3 0\nx -1 2 0\n3 0\n", true),
        ("odd_cycle_unsat", "p cnf 3 3\nx 1 2 0\nx 2 3 0\nx 1 3 0\n", false),
        ("negation_flips_rhs_unsat", "p cnf 2 2\nx 1 2 0\nx -1 2 0\n", false),
        ("pigeonhole_3_2_unsat", "p cnf 6 9\n1 2 0\n3 4 0\n5 6 0\n-1 -3 0\n-1 -5 0\n-3 -5 0\n-2 -4 0\n-2 -6 0\n-4 -6 0\n", false),
    ] {
        let path = cfg.scratch.join(format!("selftest_{name}.cnf"));
        fs::write(&path, text).unwrap();
        let call = cms_call(&cfg, &path, &format!("selftest_{name}"), 100_000, 30.0, 1)
            .expect("child");
        let _ = fs::remove_file(&path);
        let cpu = call.run.user_ns + call.run.sys_ns;
        cpu_total += cpu;
        let got = match cms_verdict(&call.out, call.run.exit_code) {
            Ok(CmsVerdict::Sat) => Some(true),
            Ok(CmsVerdict::Unsat) => Some(false),
            _ => None,
        };
        let model_ok = !want
            || cms_model(&call.out, parse_cnf_xor(text).unwrap().nv)
                .map(|m| cnf_satisfied(&parse_cnf_xor(text).unwrap(), &m))
                .unwrap_or(false);
        let pass = got == Some(want) && model_ok;
        bad += !pass as u32;
        emit(&json!({"type": "selftest", "name": name, "truth_sat": want, "cms_sat": got,
                     "model_satisfies_file": model_ok, "pass": pass, "cpu_ns": cpu}));
    }
    println!(
        "selftest: {} random instances ({sat_n} SAT, {unsat_n} UNSAT) agree with brute force, \
         the native solver and the model check: {agree}; failures overall: {bad}; child CPU {:.3} s",
        count,
        cpu_total as f64 / 1e9
    );
    emit(
        &json!({"type": "selftest_summary", "random": count, "sat": sat_n, "unsat": unsat_n,
                 "failures": bad, "cpu_ns": 0, "child_cpu_ns": cpu_total}),
    );
    if bad == 0 && sat_n > 0 && unsat_n > 0 {
        0
    } else {
        2
    }
}

/// `cms-check`: export fidelity (the native solver on the DIMACS round trip
/// follows the original search exactly) and the positive control through the
/// real child-process path on cheap decomposable targets.
fn cms_check_cmd(a: &HashMap<String, String>) -> i32 {
    let inst = instance_from_args(a);
    let cfg = match cms_config_from_args(a) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return 64;
        }
    };
    if let Err(e) = cms_pinned(&cfg.bin) {
        eprintln!("binary refused: {e}");
        return 3;
    }
    let out_path = a.get("out").expect("--out controls.jsonl").clone();
    let fid_budget: u64 = a
        .get("fidelity-budget")
        .map(|v| v.parse().unwrap())
        .unwrap_or(20_000);
    let cases: Vec<(String, bool)> = a
        .get("fidelity")
        .cloned()
        .unwrap_or_else(|| "b3:A,b3:B,a0:A,a0:B".into())
        .split(',')
        .map(|s| {
            let (t, arm) = s.split_once(':').expect("target:arm");
            (t.to_string(), arm == "B")
        })
        .collect();
    let positive: Vec<(String, bool)> = a
        .get("positive")
        .cloned()
        .unwrap_or_else(|| "b3:A,b3:B".into())
        .split(',')
        .map(|s| {
            let (t, arm) = s.split_once(':').expect("target:arm");
            (t.to_string(), arm == "B")
        })
        .collect();
    let form = formulation("subgroup_xor").unwrap();
    let mut out = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&out_path)
        .expect("open controls");
    let mut emit = |v: &Value| {
        writeln!(out, "{}", serde_json::to_string(v).unwrap()).unwrap();
        out.flush().unwrap();
    };
    let mut failures = 0;
    for (tid, phases) in &cases {
        let t = inst.targets.iter().find(|t| &t.id == tid).expect("target");
        let seed = phase_seed(1);
        let (model, _vars) = build_model(&inst.ec.gf, &inst.basis, t.r, *phases, form);
        let original = model.load(Priority::Core);
        let text = to_dimacs_xor(&original);
        let reparsed = parse_dimacs_xor(&text).expect("round trip parse");
        let fixed_point = to_dimacs_xor(&reparsed) == text;
        let own = parse_cnf_xor(&text).expect("own parse");
        let counts_match = reparsed.n_xors() == own.xors.len()
            && own.nv == model.nv
            && own.xors.len() == original.n_xors()
            && own.clauses.len() == original.n_original_clauses();
        // Same search from both: priority, phases, budget.
        let run = |mut s: Solver| {
            s.set_branch_priority(&model.core);
            s.conflict_budget = fid_budget;
            s.scramble_saved_phases(seed);
            let c0 = cpu_ns();
            let r = s.solve();
            (
                format!("{r:?}"),
                s.conflicts(),
                s.stats.decisions,
                s.stats.propagations,
                s.stats.restarts,
                cpu_ns() - c0,
            )
        };
        let (ra, rb) = (
            run(model.load(Priority::None)),
            run(parse_dimacs_xor(&text).expect("parse")),
        );
        let same = ra.0 == rb.0 && ra.1 == rb.1 && ra.2 == rb.2 && ra.3 == rb.3 && ra.4 == rb.4;
        let pass = fixed_point && counts_match && same;
        failures += !pass as u32;
        emit(
            &json!({"type": "fidelity", "target_id": tid, "arm": if *phases {"B"} else {"A"},
                     "dimacs_sha256": sha_hex(text.as_bytes()), "dimacs_bytes": text.len(),
                     "vars": model.nv, "clauses": own.clauses.len(), "xors": own.xors.len(),
                     "reexport_is_identical": fixed_point, "counts_match": counts_match,
                     "conflict_budget": fid_budget,
                     "original": {"status": ra.0, "conflicts": ra.1, "decisions": ra.2, "propagations": ra.3, "restarts": ra.4},
                     "round_trip": {"status": rb.0, "conflicts": rb.1, "decisions": rb.2, "propagations": rb.3, "restarts": rb.4},
                     "identical_search": same, "pass": pass, "cpu_ns": ra.5 + rb.5}),
        );
        println!(
            "fidelity {tid}:{} fixed_point={fixed_point} counts={counts_match} identical_search={same} ({} conflicts)",
            if *phases { "B" } else { "A" },
            ra.1
        );
    }
    let mut first_conflicts: Option<(u64, u64)> = None;
    for (n, (tid, phases)) in positive.iter().enumerate() {
        let t = inst.targets.iter().find(|t| &t.id == tid).expect("target");
        let item = json!({"rep": 0, "form": "subgroup_xor", "pair": 0, "position": 0,
                          "arm": if *phases {"B"} else {"A"}, "target_id": t.id, "set": t.set});
        let tag = format!("control_{}_{}", if *phases { "B" } else { "A" }, t.id);
        let repeats = if n == 0 { 2 } else { 1 };
        for rep in 0..repeats {
            let (res, info) = run_cms_attempt(
                &inst,
                &cfg,
                &format!("{tag}_{rep}"),
                t.r,
                *phases,
                form,
                cms_seed(0),
            );
            let mut v = cms_attempt_value(&item, &inst, &res, info, n as u64, cms_seed(0));
            v["type"] = json!("positive_control");
            v["cpu_ns"] = v["total_cpu_ns"].clone();
            let ok = res.status == "verified" && res.found.is_some();
            failures += !ok as u32;
            if n == 0 {
                match first_conflicts {
                    None => first_conflicts = Some((res.conflicts, res.decisions)),
                    Some(p) => {
                        let det = p == (res.conflicts, res.decisions);
                        v["same_seed_rerun_identical"] = json!(det);
                        failures += !det as u32;
                    }
                }
            }
            println!(
                "positive control {tag}: {} ({} conflicts)",
                res.status, res.conflicts
            );
            emit(&v);
        }
    }
    if failures == 0 {
        0
    } else {
        2
    }
}

fn usage() {
    eprintln!(
        "usage: phase_paired_fixed_budget <gate|run|freeze|cms-selftest|cms-check|cms-run> [--key value ...]\n\
         cms-selftest --cms BIN --scratch DIR --out controls.jsonl [--count 60]\n\
         cms-check    --cms BIN --scratch DIR --out controls.jsonl [--fidelity b3:A,b3:B,a0:A,a0:B] [--positive b3:A,b3:B]\n\
         cms-run      --cms BIN --scratch DIR --out R.jsonl [--manifest P] [--reps 3] [--conflict-budget 3000000]\n\
                      [--maxtime 120] [--cpu-cap 3600] [--cpu-prior S] [--controls controls.jsonl] [--resume]\n\
         (the cms-* subcommands: PROTOCOL_ADDENDUM_2.md; the binary is hash-checked)\n\
         usage: phase_paired_fixed_budget <gate|run|freeze> [--key value ...]\n\
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
        Some("cms-selftest") => cms_selftest_cmd(&arg_map(&args[1..])),
        Some("cms-check") => cms_check_cmd(&arg_map(&args[1..])),
        Some("cms-run") => cms_run_cmd(&arg_map(&args[1..])),
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

    // ---- CryptoMiniSat follow-up (PROTOCOL_ADDENDUM_2.md) ----

    const CANNED_SAT: &str = "c CryptoMiniSat version 5.16.0\n\
        c restarts                 : 15          (2441.67   confls per restart)\n\
        c decisions                : 37998       (0.00      % random)\n\
        c decisions/conflicts      : 1.04\n\
        c propagations             : 2133K       (2310K     props/s)\n\
        c conflicts                : 36625       \n\
        c Conflicts in UIP         : 7\n\
        c restarts                 : 15          (2441.67   confls per restart)\n\
        c conflicts                : 36625       \n\
        c Total time (this thread) : 0.92        \n\
        s SATISFIABLE\n\
        v -1 2 -3 4\n\
        v 5 0\n";
    const CANNED_UNSAT: &str = "c conflicts                : 666390      \n\
        c Total time (this thread) : 25.34       \n\
        s UNSATISFIABLE\n";
    const CANNED_UNKNOWN: &str =
        "c Not finished running -- signal caught or some maximum reached\n\
        c propagations             : 1M          (1886K     props/s)\n\
        c conflicts                : 5002        \n\
        s INDETERMINATE\n";

    #[test]
    fn cms_output_parser_reads_canned_outputs() {
        let o = parse_cms_output(CANNED_SAT);
        assert_eq!(o.status.as_deref(), Some("SATISFIABLE"));
        assert_eq!(o.model, vec![-1, 2, -3, 4, 5]);
        assert!(o.model_closed && !o.malformed);
        assert_eq!(o.conflicts, vec![36625, 36625], "labels must match exactly");
        assert_eq!(
            (o.decisions, o.propagations, o.restarts),
            (Some(37998), Some(2_133_000), Some(15))
        );
        assert_eq!(o.total_time_s, Some(0.92));
        assert_eq!(cms_verdict(&o, Some(10)), Ok(CmsVerdict::Sat));
        assert_eq!(cms_conflicts(&o), Ok(36625));
        assert_eq!(cms_model(&o, 5), Ok(vec![false, true, false, true, true]));
        let u = parse_cms_output(CANNED_UNSAT);
        assert_eq!(cms_verdict(&u, Some(20)), Ok(CmsVerdict::Unsat));
        assert_eq!(cms_conflicts(&u), Ok(666390));
        let k = parse_cms_output(CANNED_UNKNOWN);
        assert_eq!(cms_verdict(&k, Some(15)), Ok(CmsVerdict::Unknown));
        assert_eq!(cms_conflicts(&k), Ok(5002));
        assert_eq!(k.propagations, Some(1_000_000));
    }

    #[test]
    fn cms_verdict_and_model_fail_closed() {
        let sat = parse_cms_output(CANNED_SAT);
        assert!(
            cms_verdict(&sat, Some(20)).is_err(),
            "SAT line with the UNSAT exit code"
        );
        assert!(cms_verdict(&sat, None).is_err(), "no exit code (a signal)");
        assert!(cms_verdict(&parse_cms_output(CANNED_UNKNOWN), Some(0)).is_err());
        assert!(cms_verdict(&parse_cms_output("c nothing\n"), Some(10)).is_err());
        assert!(cms_conflicts(&parse_cms_output("s UNSATISFIABLE\n")).is_err());
        assert!(cms_model(&sat, 6).is_err(), "variable 6 missing");
        assert!(cms_model(&sat, 4).is_err(), "literal 5 out of range");
        let repeated = parse_cms_output("s SATISFIABLE\nv 1 -1 0\n");
        assert!(cms_model(&repeated, 1).is_err());
        let unclosed = parse_cms_output("s SATISFIABLE\nv 1 2\n");
        assert!(cms_model(&unclosed, 2).is_err());
        let bad = parse_cms_output("s SATISFIABLE\nv 1 x 0\n");
        assert!(bad.malformed && cms_model(&bad, 1).is_err());
    }

    #[test]
    fn scaled_statistics_parse_with_suffixes() {
        assert_eq!(parse_scaled("42"), Some(42));
        assert_eq!(parse_scaled("222K"), Some(222_000));
        assert_eq!(parse_scaled("1.5M"), Some(1_500_000));
        assert_eq!(parse_scaled("2G"), Some(2_000_000_000));
        assert_eq!(parse_scaled("x"), None);
    }

    #[test]
    fn dimacs_xor_reader_follows_the_cryptominisat_parity_convention() {
        let c = parse_cnf_xor("p cnf 3 3\nx 1 2 3 0\nx -1 2 0\nx -1 -3 0\n").unwrap();
        assert_eq!(c.nv, 3);
        assert_eq!(
            c.xors,
            vec![
                (vec![1, 2, 3], true),
                (vec![1, 2], false),
                (vec![1, 3], true)
            ]
        );
        assert!(
            parse_cnf_xor("p cnf 2 1\nx 1 1 0\n").is_err(),
            "repeated variable"
        );
        assert!(parse_cnf_xor("1 2 0\n").is_err(), "no header");
        // x1^x2^x3 = 1, x1 = x2, x1 ^ x3 = 1: x = (0,0,1) works, (1,1,1) does not.
        assert!(cnf_satisfied(&c, &[false, false, true]));
        assert!(!cnf_satisfied(&c, &[true, true, true]));
        // The empty clause is never satisfied.
        let e = parse_cnf_xor("p cnf 1 1\n0\n").unwrap();
        assert!(!cnf_satisfied(&e, &[true]));
    }

    #[test]
    fn tiny_family_has_both_answers_and_the_native_solver_agrees_with_brute_force() {
        let mut rng = SplitMix(0x434d_5353_4c46_5453);
        let (mut sat, mut unsat) = (0, 0);
        for _ in 0..40 {
            let n = 10 + rng.below(5) as u32;
            let (clauses, xors, truth) = tiny_instance(&mut rng, n);
            let text = to_dimacs_xor(&solver_from_lists(n, &clauses, &xors));
            let mut s = parse_dimacs_xor(&text).unwrap();
            assert_eq!(
                s.solve() == SolveResult::Sat,
                truth,
                "native on the round trip"
            );
            // My reader sees the same constraints the crate wrote.
            let c = parse_cnf_xor(&text).unwrap();
            assert_eq!(c.nv, n);
            if truth {
                sat += 1;
            } else {
                unsat += 1;
            }
        }
        assert!(
            sat >= 3 && unsat >= 3,
            "family must exercise both answers ({sat}/{unsat})"
        );
    }

    #[test]
    fn append_clause_keeps_the_header_count() {
        let t = append_clause("p cnf 3 2\n1 2 0\nx 1 3 0\n", &[-1, -2]);
        assert!(t.starts_with("p cnf 3 3\n"));
        assert!(t.ends_with("-1 -2 0\n"));
        let c = parse_cnf_xor(&t).unwrap();
        assert_eq!((c.clauses.len(), c.xors.len()), (2, 1));
    }

    #[test]
    fn stage_rule_follows_the_preregistered_cases() {
        // Repetition 1 always starts.
        assert_eq!(rep_mode(1, 10.0, 1.1, None, None), "full");
        // Fits in full with the guard; arm A only; none.
        assert_eq!(rep_mode(2, 1100.0, 1.1, Some(1000.0), Some(400.0)), "full");
        assert_eq!(rep_mode(2, 1099.0, 1.1, Some(1000.0), Some(400.0)), "armA");
        assert_eq!(rep_mode(2, 441.0, 1.1, Some(1000.0), Some(400.0)), "armA");
        assert_eq!(rep_mode(2, 439.0, 1.1, Some(1000.0), Some(400.0)), "none");
        // No completed repetition to extrapolate from: do not start another.
        assert_eq!(rep_mode(2, 3000.0, 1.1, None, None), "none");
        assert_ne!(cms_seed(1), cms_seed(2));
        assert!(cms_seed(1) != 0 && cms_seed(2) != 0 && cms_seed(3) != 0);
    }

    /// The export of real arm instances parses back to the same text, and the
    /// native solver follows exactly the same search on the original and on the
    /// round trip (same clauses, rows and order), for both arms.
    #[test]
    fn export_round_trip_is_faithful_for_both_arms() {
        let i = inst();
        let form = formulation("subgroup_xor").unwrap();
        for phases in [false, true] {
            let t = i.targets.iter().find(|t| t.id == "b3").unwrap();
            let (model, _) = build_model(&i.ec.gf, &i.basis, t.r, phases, form);
            let original = model.load(Priority::Core);
            let text = to_dimacs_xor(&original);
            let reparsed = parse_dimacs_xor(&text).unwrap();
            assert_eq!(to_dimacs_xor(&reparsed), text, "re-export is a fixed point");
            let c = parse_cnf_xor(&text).unwrap();
            assert_eq!(c.nv, model.nv);
            assert_eq!(c.xors.len(), original.n_xors());
            assert_eq!(c.clauses.len(), original.n_original_clauses());
            let run = |mut s: Solver| {
                s.set_branch_priority(&model.core);
                s.conflict_budget = 4_000;
                s.scramble_saved_phases(phase_seed(1));
                let r = format!("{:?}", s.solve());
                (r, s.conflicts(), s.stats.decisions, s.stats.propagations)
            };
            assert_eq!(
                run(model.load(Priority::None)),
                run(parse_dimacs_xor(&text).unwrap()),
                "phases = {phases}"
            );
        }
    }

    /// With PHASE_PAIRED_CMS naming the pinned binary, the child-process path
    /// agrees with brute force on tiny instances (skipped otherwise).
    #[test]
    fn pinned_binary_agrees_with_brute_force_when_present() {
        let Ok(bin) = std::env::var("PHASE_PAIRED_CMS") else {
            return;
        };
        let scratch = std::env::temp_dir().join(format!("pp_cms_test_{}", std::process::id()));
        fs::create_dir_all(&scratch).unwrap();
        let cfg = CmsConfig {
            bin,
            conflicts: 100_000,
            maxtime_s: 30.0,
            scratch: scratch.clone(),
            keep_dimacs: false,
        };
        assert!(
            cms_pinned(&cfg.bin).is_ok(),
            "binary must be the pinned build"
        );
        let mut rng = SplitMix(99);
        for k in 0..12 {
            let n = 10 + rng.below(4) as u32;
            let (clauses, xors, truth) = tiny_instance(&mut rng, n);
            let text = to_dimacs_xor(&solver_from_lists(n, &clauses, &xors));
            let path = scratch.join(format!("t{k}.cnf"));
            fs::write(&path, &text).unwrap();
            let call = cms_call(&cfg, &path, &format!("t{k}"), 100_000, 30.0, 1).unwrap();
            let v = cms_verdict(&call.out, call.run.exit_code).unwrap();
            assert_eq!(v == CmsVerdict::Sat, truth, "instance {k}");
            if truth {
                let m = cms_model(&call.out, n).unwrap();
                assert!(cnf_satisfied(&parse_cnf_xor(&text).unwrap(), &m));
            }
        }
        let _ = fs::remove_dir_all(&scratch);
    }
}
