//! **Round 6, lead R6.1: direct `S₅` systems and the `ic` plug-in for the
//! chained `x`-frame descent.**
//!
//! `research/isogeny_fibers_20261010/round6/PROGRAM.md` §R6.1, following
//! round-4 lead 16.1 (`examples/r4_16_1_m_crossover.rs`).  Four commands,
//! all native:
//!
//! * `interpolate`: the direct symmetrised `S₅` (`A(w) + s·B(w)`, `w = u² + u`,
//!   `s = Σu`, orbit-coefficient unknowns over `F₂`) and the direct `x`-frame
//!   `S₅` (Semaev, symmetric in five abscissae, `b = 1`) by linear algebra
//!   over sampled 4-term relations, with the same code checked against the
//!   library's `S₄` terms (`symmetrised_terms(3)`, `plain_terms(3)`), then
//!   verified on fresh relations at other `n` and both `a`.
//! * `discrepancy`: the round-4 gap between the panel and `ic bench` hit
//!   rates (symmetrised `S₄`, `n = 23`, invariant `ℓ = 12` base).  The same
//!   oracle function runs under both drivers with one node budget; every
//!   call logs the reductions consumed and why it ended.
//! * `panel`: per-attempt `m = 4` cells over one random `V ∋ 1`, every
//!   verdict gated against exact enumeration: `x-chained` through the new
//!   `chained-x` plug-in, `sym-chained` (round-4 control), `sym-S5`
//!   (direct symmetrised) and `x-S5` (direct `x`-frame).
//! * `e2e`: end-to-end runs through the framework's `run_pipeline` (the code
//!   `ic bench` runs; same calibration, same target source, planted logarithm
//!   recovered and verified): `mitm` `m = 2` control, `chained-x` `m = 3, 4`,
//!   and the direct `S₅` oracles defined here.
//!
//! Unit: the engine's counted word XORs (`f4_word_ops_thread`), as in round
//! 4; the symbolic build of a system is not counted (lower bound) and is
//! timed separately for the direct `S₅` arms, whose target-independent part
//! is built once per base.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::env;
use std::fs;
use std::time::Instant;

use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::ic_boundary::{
    calibrate_binary_instance, calibrate_row_ops, koblitz_factor_base, koblitz_instance,
    lift_abscissae, BinaryGroup, BinaryInstance, ColumnFold, CountedGroup, FactorBase, GroupOps,
    OracleCounters,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::{
    ChainedXOracle, KoblitzSymmetrisedBase, MitmOracle, SymmetrisedOracle,
};
use crypto_lib::cryptanalysis::ic_framework::stages::{
    BooleanSystem, DecompositionOracle, FactorBaseBuilder, InstanceCtx, Params, SolverCost,
    SolverTotals, SystemShape, Targets,
};
use crypto_lib::cryptanalysis::ic_framework::{run_pipeline, PipelineSpec, RunReport};
use crypto_lib::cryptanalysis::koblitz_fast::FastPoint;
use crypto_lib::cryptanalysis::koblitz_groebner::{
    f4_word_ops_thread, solve_boolean_system_filtered, split_rule_default, FieldStructure,
    SolveOptions, SolveStats, SolverEngine, SymElement, MAX_VARS,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{FrobeniusFactorBase, KoblitzCurve};
use crypto_lib::cryptanalysis::koblitz_symmetrised::{
    build_symmetrised_factor_base, build_symmetrised_factor_base_from_basis,
    build_symmetrised_system, chained_symmetrised_groebner_decompose,
    lift_symmetrised_root_accepting, plain_subspace_factor_base, plain_terms,
    random_subspace_containing_one, symmetrised_terms, SymmetrisedFactorBase, SymmetrisedSystem,
};
use crypto_lib::cryptanalysis::pq_groebner_f2::F2BoolPoly;

const ENGINE_ENV: &[&str] = &[
    "IC_REDUCTION_CACHE",
    "KIC_F4_INHERIT",
    "SOLVER_SPLIT_RULE",
    "KIC_F4_MAX_DEGREE_ONLY",
    "KIC_F4_CLOSURE_ROUNDS",
    "KIC_F4_SOLVER_FULL_READBACK",
    "F4_F2_MAX_ROWS",
    "F4_F2_MAX_COLS",
    "KIC_LINEAR_ELIM",
    "KIC_FALLROW_GATE",
];

// ── Small helpers ──────────────────────────────────────────────────

fn fe(v: u64, n: u32) -> F2mElement {
    F2mElement::from_biguint(&BigUint::from(v), n)
}

fn bits(e: &F2mElement) -> u64 {
    e.raw_bits().first().copied().unwrap_or(0)
}

struct Args(HashMap<String, String>);

impl Args {
    fn parse(raw: &[String]) -> Self {
        let mut m = HashMap::new();
        let mut i = 0;
        while i < raw.len() {
            let k = raw[i].trim_start_matches("--").to_string();
            let v = raw.get(i + 1).cloned().unwrap_or_default();
            m.insert(k, v);
            i += 2;
        }
        Args(m)
    }
    fn get(&self, k: &str, d: &str) -> String {
        self.0.get(k).cloned().unwrap_or_else(|| d.to_string())
    }
    fn u(&self, k: &str, d: u64) -> u64 {
        self.0.get(k).map(|v| v.parse().expect(k)).unwrap_or(d)
    }
    fn f(&self, k: &str, d: f64) -> f64 {
        self.0.get(k).map(|v| v.parse().expect(k)).unwrap_or(d)
    }
}

fn rule_ell(n: u32, m: usize) -> usize {
    let fact: f64 = (1..=m).map(|k| k as f64).product();
    ((n as f64 + fact.log2()) / m as f64).ceil() as usize
}

fn write_json(path: &str, v: &Value) {
    fs::write(path, serde_json::to_string_pretty(v).unwrap()).expect("write json");
}

// ── Symmetric orbits and interpolation ─────────────────────────────

/// Monomials `(e_0, …, e_{k−1})` with `e_i ≤ maxe`, each tagged with the
/// index of its orbit (the sorted exponent vector) under `S_k`.
struct Orbits {
    orbit_keys: Vec<Vec<u32>>,
    monomials: Vec<(Vec<u32>, usize)>,
}

impl Orbits {
    fn new(k: usize, maxe: u32) -> Self {
        let mut keys: Vec<Vec<u32>> = Vec::new();
        let mut key_index: HashMap<Vec<u32>, usize> = HashMap::new();
        let mut monomials = Vec::new();
        let total = (maxe as u64 + 1).pow(k as u32);
        for code in 0..total {
            let mut e = Vec::with_capacity(k);
            let mut c = code;
            for _ in 0..k {
                e.push((c % (maxe as u64 + 1)) as u32);
                c /= maxe as u64 + 1;
            }
            let mut key = e.clone();
            key.sort_unstable_by(|a, b| b.cmp(a));
            let idx = *key_index.entry(key.clone()).or_insert_with(|| {
                keys.push(key);
                keys.len() - 1
            });
            monomials.push((e, idx));
        }
        Orbits {
            orbit_keys: keys,
            monomials,
        }
    }

    /// Orbit sums `m_λ(v)` for every orbit, counting field multiplications.
    fn eval(&self, gf: &Gf, v: &[u64], maxe: u32, mults: &mut u64) -> Vec<u64> {
        let pw: Vec<Vec<u64>> = v
            .iter()
            .map(|&x| {
                let mut p = vec![1u64];
                for _ in 0..maxe {
                    let last = *p.last().unwrap();
                    p.push(gf.mul(last, x));
                    *mults += 1;
                }
                p
            })
            .collect();
        let mut out = vec![0u64; self.orbit_keys.len()];
        for (e, idx) in &self.monomials {
            let mut t = 1u64;
            for (i, &ei) in e.iter().enumerate() {
                if ei > 0 {
                    t = gf.mul(t, pw[i][ei as usize]);
                    *mults += 1;
                }
            }
            out[*idx] ^= t;
        }
        out
    }
}

/// Field arithmetic on single words, borrowed from the instance.
struct Gf<'a>(&'a BinaryInstance);

impl Gf<'_> {
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.0.gf.mul(a, b)
    }
    fn inv(&self, a: u64) -> u64 {
        self.0.gf.inv(a)
    }
}

/// Incremental row echelon over `F₂` with `cols` columns.
struct Echelon {
    cols: usize,
    words: usize,
    rows: Vec<Vec<u64>>,
    pivots: Vec<usize>,
    word_xors: u64,
}

impl Echelon {
    fn new(cols: usize) -> Self {
        Echelon {
            cols,
            words: cols.div_ceil(64),
            rows: Vec::new(),
            pivots: Vec::new(),
            word_xors: 0,
        }
    }
    fn insert(&mut self, mut row: Vec<u64>) -> bool {
        for (r, &p) in self.rows.iter().zip(&self.pivots) {
            if (row[p / 64] >> (p % 64)) & 1 == 1 {
                for w in 0..self.words {
                    row[w] ^= r[w];
                }
                self.word_xors += self.words as u64;
            }
        }
        let Some(p) = (0..self.cols).find(|&c| (row[c / 64] >> (c % 64)) & 1 == 1) else {
            return false;
        };
        // Keep the basis fully reduced on its pivots.
        for r in self.rows.iter_mut() {
            if (r[p / 64] >> (p % 64)) & 1 == 1 {
                for w in 0..self.words {
                    r[w] ^= row[w];
                }
                self.word_xors += self.words as u64;
            }
        }
        self.rows.push(row);
        self.pivots.push(p);
        true
    }
    /// A basis of the null space of the stored rows.
    fn nullspace(&self) -> Vec<Vec<u8>> {
        let pivset: HashSet<usize> = self.pivots.iter().copied().collect();
        let mut out = Vec::new();
        for free in (0..self.cols).filter(|c| !pivset.contains(c)) {
            let mut v = vec![0u8; self.cols];
            v[free] = 1;
            for (r, &p) in self.rows.iter().zip(&self.pivots) {
                if (r[free / 64] >> (free % 64)) & 1 == 1 {
                    v[p] = 1;
                }
            }
            out.push(v);
        }
        out
    }
}

/// A frame of the summation polynomial being interpolated.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Frame {
    /// `A(w) + s·B(w)`, `k` invariants `w`, `A` of degree `2^{k−3}` in each
    /// `w`, `B` one less.
    Sym,
    /// Semaev's `S_k` in `x`, degree `2^{k−2}` in each abscissa.
    X,
}

struct Poly5 {
    frame: Frame,
    k: usize,
    a_orbits: Orbits,
    a_max: u32,
    b_orbits: Option<Orbits>,
    b_max: u32,
    /// Coefficient per orbit (A first, then B).
    coeffs: Vec<u8>,
}

impl Poly5 {
    fn unknowns(&self) -> usize {
        self.a_orbits.orbit_keys.len() + self.b_orbits.as_ref().map_or(0, |b| b.orbit_keys.len())
    }

    fn row_values(&self, gf: &Gf, v: &[u64], s: u64, mults: &mut u64) -> Vec<u64> {
        let mut vals = self.a_orbits.eval(gf, v, self.a_max, mults);
        if let Some(b) = &self.b_orbits {
            for t in b.eval(gf, v, self.b_max, mults) {
                vals.push(gf.mul(s, t));
                *mults += 1;
            }
        }
        vals
    }

    fn evaluate(&self, gf: &Gf, v: &[u64], s: u64) -> u64 {
        let mut m = 0;
        let vals = self.row_values(gf, v, s, &mut m);
        vals.iter()
            .zip(&self.coeffs)
            .filter(|(_, &c)| c == 1)
            .fold(0, |acc, (&x, _)| acc ^ x)
    }

    /// Monomials with coefficient 1, as `(exponents, s-exponent)`.
    fn terms(&self) -> Vec<(Vec<u32>, u32)> {
        let na = self.a_orbits.orbit_keys.len();
        let mut out = Vec::new();
        for (e, idx) in &self.a_orbits.monomials {
            if self.coeffs[*idx] == 1 {
                out.push((e.clone(), 0));
            }
        }
        if let Some(b) = &self.b_orbits {
            for (e, idx) in &b.monomials {
                if self.coeffs[na + *idx] == 1 {
                    out.push((e.clone(), 1));
                }
            }
        }
        out
    }
}

/// One random relation `P_1 + … + P_{k−1} = R` on the instance's curve,
/// as the `k` frame values and `s`; `None` when a frame value is undefined.
fn sample_relation(
    inst: &BinaryInstance,
    gf: &Gf,
    frame: Frame,
    k: usize,
    rng: &mut StdRng,
) -> Option<(Vec<u64>, u64)> {
    let fc = &inst.fast;
    let n = inst.n;
    let mut pts = Vec::new();
    while pts.len() < k - 1 {
        let x = rng.gen_range(1..(1u64 << n));
        let lifts = inst.points_with_x(x);
        if lifts.is_empty() {
            continue;
        }
        pts.push(lifts[rng.gen_range(0..lifts.len())]);
    }
    let r = pts.iter().fold(FastPoint::INFINITY, |s, &p| fc.add(s, p));
    if r.infinity {
        return None;
    }
    pts.push(r);
    frame_values(gf, frame, &pts.iter().map(|p| p.x).collect::<Vec<_>>())
}

/// `(w_i, s)` in the `u`-frame, or `(x_i, 0)`.
fn frame_values(gf: &Gf, frame: Frame, xs: &[u64]) -> Option<(Vec<u64>, u64)> {
    match frame {
        Frame::X => Some((xs.to_vec(), 0)),
        Frame::Sym => {
            let mut ws = Vec::new();
            let mut s = 0;
            for &x in xs {
                if x == 1 {
                    return None;
                }
                let u = gf.inv(x ^ 1);
                ws.push(gf.mul(u, u) ^ u);
                s ^= u;
            }
            Some((ws, s))
        }
    }
}

struct InterpStats {
    samples: u64,
    rows: u64,
    rank: usize,
    nullity: usize,
    mults: u64,
    word_xors: u64,
    wall_s: f64,
}

/// Interpolate the frame's `S_k` on `inst` by sampled relations.
fn interpolate(inst: &BinaryInstance, frame: Frame, k: usize, seed: u64) -> (Poly5, InterpStats) {
    let gf = Gf(inst);
    let (a_max, b_max) = match frame {
        Frame::Sym => (1u32 << (k - 3), (1u32 << (k - 3)) - 1),
        Frame::X => (1u32 << (k - 2), 0),
    };
    let a_orbits = Orbits::new(k, a_max);
    let b_orbits = (frame == Frame::Sym).then(|| Orbits::new(k, b_max));
    let mut poly = Poly5 {
        frame,
        k,
        a_orbits,
        a_max,
        b_orbits,
        b_max,
        coeffs: Vec::new(),
    };
    let cols = poly.unknowns();
    let mut ech = Echelon::new(cols);
    let mut rng = StdRng::seed_from_u64(seed);
    let started = Instant::now();
    let (mut samples, mut rows, mut mults, mut stale) = (0u64, 0u64, 0u64, 0u32);
    let n = inst.n as usize;
    while stale < 8 {
        let Some((v, s)) = sample_relation(inst, &gf, frame, k, &mut rng) else {
            continue;
        };
        samples += 1;
        let vals = poly.row_values(&gf, &v, s, &mut mults);
        let mut grew = false;
        for bit in 0..n {
            let mut row = vec![0u64; cols.div_ceil(64)];
            for (c, &x) in vals.iter().enumerate() {
                if (x >> bit) & 1 == 1 {
                    row[c / 64] |= 1 << (c % 64);
                }
            }
            rows += 1;
            grew |= ech.insert(row);
        }
        if grew {
            stale = 0;
        } else {
            stale += 1;
        }
    }
    let null = ech.nullspace();
    let stats = InterpStats {
        samples,
        rows,
        rank: ech.rows.len(),
        nullity: null.len(),
        mults,
        word_xors: ech.word_xors,
        wall_s: started.elapsed().as_secs_f64(),
    };
    poly.coeffs = null.into_iter().next().unwrap_or_else(|| vec![0; cols]);
    (poly, stats)
}

/// Fresh relations must all vanish; random tuples should mostly not.
fn verify(inst: &BinaryInstance, poly: &Poly5, trials: usize, seed: u64) -> Value {
    let gf = Gf(inst);
    let mut rng = StdRng::seed_from_u64(seed);
    let (mut rel_ok, mut rel_n) = (0usize, 0usize);
    while rel_n < trials {
        if let Some((v, s)) = sample_relation(inst, &gf, poly.frame, poly.k, &mut rng) {
            rel_n += 1;
            rel_ok += (poly.evaluate(&gf, &v, s) == 0) as usize;
        }
    }
    let (mut rnd_zero, mut rnd_n) = (0usize, 0usize);
    while rnd_n < trials {
        let xs: Vec<u64> = (0..poly.k)
            .map(|_| rng.gen_range(2..(1u64 << inst.n)))
            .collect();
        if let Some((v, s)) = frame_values(&gf, poly.frame, &xs) {
            rnd_n += 1;
            rnd_zero += (poly.evaluate(&gf, &v, s) == 0) as usize;
        }
    }
    json!({
        "n": inst.n, "a": inst.a, "relations": rel_n, "relations_vanishing": rel_ok,
        "random_tuples": rnd_n, "random_vanishing": rnd_zero,
        "pass": rel_ok == rel_n && rnd_zero * 4 < rnd_n,
    })
}

/// The library's `S₄` terms as a coefficient set over the same orbits, so
/// the interpolation code is checked against a known answer.
fn library_terms(frame: Frame) -> HashSet<(Vec<u32>, u32)> {
    match frame {
        Frame::Sym => symmetrised_terms(3)
            .unwrap()
            .into_iter()
            .map(|t| (t[..4].to_vec(), t[4]))
            .collect(),
        Frame::X => plain_terms(3)
            .unwrap()
            .into_iter()
            .map(|t| (t, 0))
            .collect(),
    }
}

fn cmd_interpolate(args: &Args) {
    let n0 = args.u("n", 31) as u32;
    let a0 = args.u("a", 1) as u8;
    let seed = args.u("seed", 0x5_1A7E);
    let out = args.get("json", "interpolation.json");
    let inst = koblitz_instance(a0, n0).expect("interpolation instance");
    let mut results = Vec::new();
    for (frame, k) in [
        (Frame::Sym, 4usize),
        (Frame::X, 4),
        (Frame::Sym, 5),
        (Frame::X, 5),
    ] {
        let (poly, st) = interpolate(&inst, frame, k, seed ^ k as u64);
        let terms = poly.terms();
        let n_terms = terms.len();
        let support = poly.coeffs.iter().filter(|&&c| c == 1).count();
        let library_match = (k == 4).then(|| {
            let got: HashSet<(Vec<u32>, u32)> = terms.iter().cloned().collect();
            got == library_terms(frame)
        });
        let mut checks = Vec::new();
        for (n, a) in [
            (13u32, 0u8),
            (13, 1),
            (17, 1),
            (19, 0),
            (23, 0),
            (29, 1),
            (37, 0),
        ] {
            if let Some(i2) = koblitz_instance(a, n) {
                checks.push(verify(&i2, &poly, 200, seed ^ ((n as u64) << 8) ^ a as u64));
            }
        }
        let max_bool_degree = terms
            .iter()
            .map(|(e, se)| e[..k - 1].iter().map(|x| x.count_ones()).sum::<u32>() + se)
            .max()
            .unwrap_or(0);
        eprintln!(
            "{frame:?} S{k}: unknowns {} rank {} nullity {} support {support} orbits, {n_terms} monomials, samples {} mults {} XORs {} wall {:.2}s lib-match {:?}",
            poly.unknowns(),
            st.rank,
            st.nullity,
            st.samples,
            st.mults,
            st.word_xors,
            st.wall_s,
            library_match
        );
        let a_n = poly.a_orbits.orbit_keys.len();
        results.push(json!({
            "frame": format!("{frame:?}"), "k": k,
            "unknowns": poly.unknowns(),
            "a_orbits": a_n,
            "b_orbits": poly.b_orbits.as_ref().map(|b| b.orbit_keys.len()),
            "degree_bound_a": poly.a_max, "degree_bound_b": poly.b_max,
            "rank": st.rank, "nullity": st.nullity,
            "support_orbits": support,
            "support_a": poly.coeffs[..a_n].iter().filter(|&&c| c == 1).count(),
            "monomials": n_terms,
            "boolean_degree_in_unknown_summands": max_bool_degree,
            "samples": st.samples, "bit_rows": st.rows,
            "field_mults": st.mults, "word_xors": st.word_xors,
            "wall_s_contended": st.wall_s,
            "matches_library_s4": library_match,
            "orbits_with_coefficient_1": poly.coeffs.iter().enumerate()
                .filter(|(_, &c)| c == 1)
                .map(|(i, _)| if i < a_n {
                    json!({"part": "A", "exponents": poly.a_orbits.orbit_keys[i]})
                } else {
                    json!({"part": "B", "exponents": poly.b_orbits.as_ref().unwrap().orbit_keys[i - a_n]})
                })
                .collect::<Vec<_>>(),
            "verification": checks,
        }));
    }
    write_json(
        &out,
        &json!({
            "experiment": "R6.1 interpolation of S4 (control) and S5 in the u-frame (A + sB) and the x-frame",
            "instance": inst.name, "n": n0, "a": a0, "seed": seed,
            "results": results,
        }),
    );
}

// ── Direct S5 systems ──────────────────────────────────────────────

/// `c · Y` for a constant `c`: the `F₂`-linear map of multiplication by
/// `c`, applied to the coordinates.
fn cmul(gf: &Gf, c: u64, y: &SymElement, n: u32, nv: usize) -> SymElement {
    let mut out = SymElement::zero(n, nv);
    for j in 0..n as usize {
        if y.coords[j].is_zero() {
            continue;
        }
        let col = gf.mul(c, 1u64 << j);
        for k in 0..n as usize {
            if (col >> k) & 1 == 1 {
                out.coords[k] = out.coords[k].add(&y.coords[j]);
            }
        }
    }
    out
}

fn sym_pow_table(
    v: &SymElement,
    maxe: u32,
    st: &FieldStructure,
    n: u32,
    nv: usize,
) -> Vec<SymElement> {
    let mut p = vec![SymElement::constant(&F2mElement::one(n), n, nv), v.clone()];
    for e in 2..=maxe as usize {
        let next = if e % 2 == 0 {
            p[e / 2].square(st)
        } else {
            p[e - 1].mul(v, st)
        };
        p.push(next);
    }
    p
}

fn is_one_const(e: usize) -> bool {
    e == 0
}

/// The target-independent part of a direct `S₅` system: for each power
/// `k` of the target's frame value, `Σ_e c_e Π_i v_i^{e_i}` over the four
/// unknown summands, Weil-restricted.  Built once per base.
struct S5Pre {
    n_vars: usize,
    /// `A_k` (sym) or `X_k` (x), indexed by the target's exponent.
    a: Vec<SymElement>,
    /// `B_k` and `s₀·B_k` (sym only).
    b: Vec<SymElement>,
    s0b: Vec<SymElement>,
    build_s: f64,
    products: u64,
    complete: bool,
    max_terms_per_coordinate: usize,
}

fn build_s5_pre(
    poly: &Poly5,
    summand_vars: &[SymElement],
    s0: Option<&SymElement>,
    st: &FieldStructure,
    n: u32,
    nv: usize,
    wall_cap_s: f64,
) -> S5Pre {
    let started = Instant::now();
    let maxe = poly.a_max;
    let tables: Vec<Vec<SymElement>> = summand_vars
        .iter()
        .map(|v| sym_pow_table(v, maxe, st, n, nv))
        .collect();
    let mut products = 0u64;
    let pair = |i: usize, j: usize, products: &mut u64| -> Vec<Vec<SymElement>> {
        (0..=maxe as usize)
            .map(|e1| {
                (0..=maxe as usize)
                    .map(|e2| {
                        if is_one_const(e1) {
                            tables[j][e2].clone()
                        } else if is_one_const(e2) {
                            tables[i][e1].clone()
                        } else {
                            *products += 1;
                            tables[i][e1].mul(&tables[j][e2], st)
                        }
                    })
                    .collect()
            })
            .collect()
    };
    let q12 = pair(0, 1, &mut products);
    let q34 = pair(2, 3, &mut products);
    let mut complete = true;
    // coefficient maps: (e1,e2,e3,e4,k, s) -> 1
    let terms = poly.terms();
    let kmax = maxe as usize;
    let build = |s_exp: u32, top: usize, products: &mut u64, complete: &mut bool| {
        let mut out: Vec<SymElement> = Vec::new();
        for k in 0..=top {
            let mut acc = SymElement::zero(n, nv);
            let mut by_pair: BTreeMap<(usize, usize), Vec<(usize, usize)>> = BTreeMap::new();
            for (e, se) in &terms {
                if *se == s_exp && e[4] as usize == k {
                    by_pair
                        .entry((e[0] as usize, e[1] as usize))
                        .or_default()
                        .push((e[2] as usize, e[3] as usize));
                }
            }
            for ((e1, e2), rest) in by_pair {
                if started.elapsed().as_secs_f64() > wall_cap_s {
                    *complete = false;
                    break;
                }
                let mut inner = SymElement::zero(n, nv);
                for (e3, e4) in rest {
                    inner = inner.add(&q34[e3][e4]);
                }
                let prod = if e1 == 0 && e2 == 0 {
                    inner
                } else {
                    *products += 1;
                    q12[e1][e2].mul(&inner, st)
                };
                acc = acc.add(&prod);
            }
            out.push(acc);
        }
        out
    };
    let a = build(0, kmax, &mut products, &mut complete);
    let (b, s0b) = if let Some(s0) = s0 {
        let b = build(1, poly.b_max as usize, &mut products, &mut complete);
        let s0b: Vec<SymElement> = b
            .iter()
            .map(|bk| {
                products += 1;
                s0.mul(bk, st)
            })
            .collect();
        (b, s0b)
    } else {
        (Vec::new(), Vec::new())
    };
    let max_terms = a
        .iter()
        .chain(&b)
        .chain(&s0b)
        .flat_map(|e| e.coords.iter().map(|c| c.terms.len()))
        .max()
        .unwrap_or(0);
    S5Pre {
        n_vars: nv,
        a,
        b,
        s0b,
        build_s: started.elapsed().as_secs_f64(),
        products,
        complete,
        max_terms_per_coordinate: max_terms,
    }
}

impl S5Pre {
    /// The Weil-restricted equations for a target with frame value `v_r`
    /// (and `u_R` in the `u`-frame).
    fn equations(&self, gf: &Gf, n: u32, v_r: u64, u_r: Option<u64>) -> Vec<F2BoolPoly> {
        let mut acc = SymElement::zero(n, self.n_vars);
        let mut pw = 1u64;
        for ak in &self.a {
            acc = acc.add(&cmul(gf, pw, ak, n, self.n_vars));
            pw = gf.mul(pw, v_r);
        }
        if let Some(u_r) = u_r {
            let mut pw = 1u64;
            for (bk, s0bk) in self.b.iter().zip(&self.s0b) {
                acc = acc.add(&cmul(gf, pw, s0bk, n, self.n_vars));
                acc = acc.add(&cmul(gf, gf.mul(pw, u_r), bk, n, self.n_vars));
                pw = gf.mul(pw, v_r);
            }
        }
        acc.coords
    }
}

/// Per-call record kept by every driver here (the discrepancy protocol).
#[derive(Clone, Debug, Default)]
struct Call {
    outcome: &'static str,
    reductions: u64,
    splits: u64,
    word_xors: u64,
    wall_ns: u64,
    build_ns: u64,
}

impl Call {
    fn json(&self) -> Value {
        json!({"outcome": self.outcome, "reductions": self.reductions, "splits": self.splits,
               "word_xors": self.word_xors, "wall_ms_contended": self.wall_ns as f64 / 1e6,
               "build_ms_contended": self.build_ns as f64 / 1e6})
    }
}

fn solve_opts(engine: SolverEngine, budget: usize) -> SolveOptions {
    SolveOptions {
        engine,
        max_solutions: usize::MAX,
        node_budget: budget,
        split_rule: split_rule_default(),
    }
}

fn outcome_of(found: bool, stats: &SolveStats) -> &'static str {
    if found {
        "found"
    } else if stats.exhausted {
        "budget"
    } else {
        "refuted"
    }
}

/// The direct symmetrised `S₅` on `F_u`.
struct SymS5 {
    kc: KoblitzCurve,
    fb_u: SymmetrisedFactorBase,
    pre: S5Pre,
    st: FieldStructure,
    ell_w: usize,
}

impl SymS5 {
    fn new(
        kc: &KoblitzCurve,
        fb_u: &SymmetrisedFactorBase,
        poly: &Poly5,
        cap_s: f64,
    ) -> Option<Self> {
        let n = kc.n;
        let ell_w = fb_u.ell - 1;
        let nv = 4 * ell_w + 1;
        if nv > MAX_VARS {
            return None;
        }
        let st = FieldStructure::new(n, &kc.curve.irreducible);
        let ws: Vec<SymElement> = (0..4)
            .map(|i| SymElement::from_subspace_vars(&fb_u.w_basis, i * ell_w, n, nv))
            .collect();
        let mut s0 = SymElement::zero(n, nv);
        for i in 0..4 {
            s0 = s0.add(&SymElement::from_subspace_vars(
                &fb_u.v_basis[1..],
                i * ell_w,
                n,
                nv,
            ));
        }
        let mut eps = SymElement::zero(n, nv);
        eps.coords[0] = F2BoolPoly::from_monos(
            vec![crypto_lib::cryptanalysis::pq_groebner_f2::F2BoolMono::var(
                (4 * ell_w) as u32,
            )],
            nv,
        );
        s0 = s0.add(&eps);
        let pre = build_s5_pre(poly, &ws, Some(&s0), &st, n, nv, cap_s);
        Some(SymS5 {
            kc: kc.clone(),
            fb_u: fb_u.clone(),
            pre,
            st,
            ell_w,
        })
    }

    /// Indices into `fb_u.points` (with `T` appended when through `R + T`).
    fn decompose(
        &self,
        inst: &BinaryInstance,
        target: &BinaryPoint,
        engine: SolverEngine,
        budget: usize,
    ) -> (Option<(Vec<usize>, bool)>, Call) {
        let gf = Gf(inst);
        let mut call = Call::default();
        let BinaryPoint::Affine { x, .. } = target else {
            call.outcome = "unbuildable";
            return (None, call);
        };
        let x_r = bits(x);
        if x_r == 1 {
            call.outcome = "unbuildable";
            return (None, call);
        }
        let u_r = gf.inv(x_r ^ 1);
        let w_r = gf.mul(u_r, u_r) ^ u_r;
        let t0 = Instant::now();
        let eqs = self.pre.equations(&gf, inst.n, w_r, Some(u_r));
        call.build_ns = t0.elapsed().as_nanos() as u64;
        let sys = SymmetrisedSystem {
            equations: Vec::new(),
            n_vars: self.pre.n_vars,
            ell_w: self.ell_w,
            m: 4,
            u_r: fe(u_r, inst.n),
        };
        let before = f4_word_ops_thread();
        let t1 = Instant::now();
        let mut found = None;
        let (_, stats) = solve_boolean_system_filtered(
            &eqs,
            self.pre.n_vars,
            &solve_opts(engine, budget),
            |root| match lift_symmetrised_root_accepting(
                &self.kc,
                &self.fb_u,
                &sys,
                root,
                target,
                |_| true,
            ) {
                Some(r) => {
                    found = Some(r);
                    true
                }
                None => false,
            },
        );
        call.word_xors = f4_word_ops_thread().wrapping_sub(before);
        call.wall_ns = t1.elapsed().as_nanos() as u64;
        call.reductions = stats.reductions as u64;
        call.splits = stats.splits as u64;
        call.outcome = outcome_of(found.is_some(), &stats);
        let _ = &self.st;
        (found, call)
    }
}

/// The direct `x`-frame `S₅` on `F_x`.
struct XS5 {
    basis: Vec<F2mElement>,
    pre: S5Pre,
    by_x: HashMap<u64, Vec<usize>>,
    points: Vec<FastPoint>,
}

impl XS5 {
    fn new(
        kc: &KoblitzCurve,
        inst: &BinaryInstance,
        basis: &[F2mElement],
        points: &[BinaryPoint],
        poly: &Poly5,
        cap_s: f64,
    ) -> Option<Self> {
        let n = kc.n;
        let ell = basis.len();
        let nv = 4 * ell;
        if nv > MAX_VARS {
            return None;
        }
        let st = FieldStructure::new(n, &kc.curve.irreducible);
        let xs: Vec<SymElement> = (0..4)
            .map(|i| SymElement::from_subspace_vars(basis, i * ell, n, nv))
            .collect();
        let pre = build_s5_pre(poly, &xs, None, &st, n, nv, cap_s);
        let fpts: Vec<FastPoint> = points.iter().map(|p| inst.fast.lift(p)).collect();
        let mut by_x: HashMap<u64, Vec<usize>> = HashMap::new();
        for (i, p) in fpts.iter().enumerate() {
            by_x.entry(p.x).or_default().push(i);
        }
        Some(XS5 {
            basis: basis.to_vec(),
            pre,
            by_x,
            points: fpts,
        })
    }

    fn decompose(
        &self,
        inst: &BinaryInstance,
        target: FastPoint,
        engine: SolverEngine,
        budget: usize,
    ) -> (Option<Vec<usize>>, Call) {
        let gf = Gf(inst);
        let mut call = Call::default();
        if !self.pre.complete {
            call.outcome = "unbuildable";
            return (None, call);
        }
        let t0 = Instant::now();
        let eqs = self.pre.equations(&gf, inst.n, target.x, None);
        call.build_ns = t0.elapsed().as_nanos() as u64;
        let ell = self.basis.len();
        let basis_w: Vec<u64> = self.basis.iter().map(bits).collect();
        let fc = &inst.fast;
        let before = f4_word_ops_thread();
        let t1 = Instant::now();
        let mut found = None;
        let (_, stats) = solve_boolean_system_filtered(
            &eqs,
            self.pre.n_vars,
            &solve_opts(engine, budget),
            |root| {
                let xs: Vec<u64> = (0..4)
                    .map(|i| {
                        (0..ell).fold(0u64, |acc, t| {
                            if (root >> (i * ell + t)) & 1 == 1 {
                                acc ^ basis_w[t]
                            } else {
                                acc
                            }
                        })
                    })
                    .collect();
                let lists: Option<Vec<&Vec<usize>>> = xs.iter().map(|x| self.by_x.get(x)).collect();
                let Some(lists) = lists else { return false };
                let mut idx = [0usize; 4];
                let total: usize = lists.iter().map(|l| l.len()).product();
                for code in 0..total {
                    let mut c = code;
                    let mut sum = FastPoint::INFINITY;
                    for (i, l) in lists.iter().enumerate() {
                        idx[i] = l[c % l.len()];
                        c /= l.len();
                        sum = fc.add(sum, self.points[idx[i]]);
                    }
                    if sum == target {
                        found = Some(idx.to_vec());
                        return true;
                    }
                }
                false
            },
        );
        call.word_xors = f4_word_ops_thread().wrapping_sub(before);
        call.wall_ns = t1.elapsed().as_nanos() as u64;
        call.reductions = stats.reductions as u64;
        call.splits = stats.splits as u64;
        call.outcome = outcome_of(found.is_some(), &stats);
        (found, call)
    }
}

// ── Exact enumeration (the gate) ───────────────────────────────────

struct Exact {
    points: Vec<FastPoint>,
    pairs: HashSet<u64>,
}

impl Exact {
    fn new(inst: &BinaryInstance, pts: &[FastPoint]) -> Self {
        let fc = &inst.fast;
        let mut pairs = HashSet::new();
        for i in 0..pts.len() {
            for j in i..pts.len() {
                pairs.insert(fc.add(pts[i], pts[j]).pack());
            }
        }
        Exact {
            points: pts.to_vec(),
            pairs,
        }
    }
    fn decomposes(&self, inst: &BinaryInstance, t: FastPoint, m: usize) -> bool {
        let fc = &inst.fast;
        match m {
            3 => self
                .points
                .iter()
                .any(|&p| self.pairs.contains(&fc.add(t, fc.neg(p)).pack())),
            4 => {
                for i in 0..self.points.len() {
                    let a = fc.add(t, fc.neg(self.points[i]));
                    for j in i..self.points.len() {
                        if self
                            .pairs
                            .contains(&fc.add(a, fc.neg(self.points[j])).pack())
                        {
                            return true;
                        }
                    }
                }
                false
            }
            _ => unreachable!(),
        }
    }
}

// ── Framework glue ─────────────────────────────────────────────────

fn ctx_for<'a>(
    inst: &BinaryInstance,
    g: &'a BinaryGroup<'a>,
    target: FastPoint,
) -> InstanceCtx<'a, BinaryGroup<'a>> {
    InstanceCtx {
        group: g,
        generator: inst.generator,
        target,
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: Some(inst.n),
    }
}

/// A singleton-orbit view of a point set (no Frobenius fold: the subspace
/// is not invariant), so the framework's projected column fold applies.
fn singleton_view(
    kc: &KoblitzCurve,
    points: &[BinaryPoint],
    basis: &[F2mElement],
) -> FrobeniusFactorBase {
    let mut fb = plain_subspace_factor_base_like(kc, points, basis);
    fb.subspace_basis = basis.to_vec();
    fb
}

fn plain_subspace_factor_base_like(
    kc: &KoblitzCurve,
    points: &[BinaryPoint],
    basis: &[F2mElement],
) -> FrobeniusFactorBase {
    // Group ±P by abscissa, every point its own Frobenius orbit.
    let mut by_x: BTreeMap<Vec<u64>, Vec<usize>> = BTreeMap::new();
    for (i, p) in points.iter().enumerate() {
        if let BinaryPoint::Affine { x, .. } = p {
            by_x.entry(x.raw_bits().to_vec()).or_default().push(i);
        }
    }
    let mut ordered = Vec::new();
    let mut signed_orbits = Vec::new();
    let mut signed_orbit_of = Vec::new();
    for (_, idxs) in by_x {
        let o = signed_orbits.len();
        let mut orbit = Vec::new();
        for (k, &i) in idxs.iter().enumerate() {
            orbit.push(ordered.len());
            signed_orbit_of.push((o, 0u32, k == 1));
            ordered.push(points[i].clone());
        }
        signed_orbits.push(orbit);
    }
    let orbits: Vec<Vec<usize>> = (0..ordered.len()).map(|i| vec![i]).collect();
    let orbit_of: Vec<(usize, u32)> = (0..ordered.len()).map(|i| (i, 0)).collect();
    // The struct has a private derived-data cache, so start from a real
    // subspace base over the same basis and replace every public field.
    let mut fb = plain_subspace_factor_base(kc, basis).expect("independent basis");
    fb.points = ordered;
    fb.orbits = orbits;
    fb.orbit_of = orbit_of;
    fb.signed_orbits = signed_orbits;
    fb.signed_orbit_of = signed_orbit_of;
    fb.subspace = Vec::new();
    fb
}

/// A fixed point set as a framework base, columns folded by cofactor
/// projection (so `P`, `−P`, `P + T`, `−P + T` share one where they can).
struct FixedBase {
    fb: FactorBase<FastPoint>,
}

impl FixedBase {
    fn new(
        inst: &BinaryInstance,
        points: &[BinaryPoint],
        basis: &[F2mElement],
        desc: &str,
        subspace: bool,
    ) -> Self {
        let view = singleton_view(inst.koblitz.as_ref().unwrap(), points, basis);
        let mut fb = koblitz_factor_base(
            inst,
            &view,
            ColumnFold::ProjectedSignedFrobeniusOrbit,
            desc.into(),
        )
        .expect("framework base");
        fb.dimension = Some(basis.len() as u32);
        fb.subspace_basis = subspace.then(|| basis.iter().map(bits).collect());
        FixedBase { fb }
    }
    fn clone_fb(&self) -> FactorBase<FastPoint> {
        FactorBase {
            points: self.fb.points.clone(),
            neg_index: self.fb.neg_index.clone(),
            col_of: self.fb.col_of.clone(),
            coef_of: self.fb.coef_of.clone(),
            columns: self.fb.columns,
            abscissae: self.fb.abscissae,
            dimension: self.fb.dimension,
            description: self.fb.description.clone(),
            abscissa_list: self.fb.abscissa_list.clone(),
            x_index: self.fb.x_index.clone(),
            point_index: self.fb.point_index.clone(),
            cost: self.fb.cost.clone(),
            subspace_basis: self.fb.subspace_basis.clone(),
        }
    }
}

impl<'a> FactorBaseBuilder<BinaryGroup<'a>> for FixedBase {
    fn name(&self) -> &str {
        "r6-fixed-subspace"
    }
    fn describe(&self, _p: &Params) -> String {
        self.fb.description.clone()
    }
    fn build(
        &self,
        _ctx: &InstanceCtx<BinaryGroup<'a>>,
        _params: &Params,
        _ops: &mut GroupOps,
    ) -> Result<FactorBase<FastPoint>, String> {
        Ok(self.clone_fb())
    }
}

/// A traced, deadline-guarded wrapper: records every target the runner
/// passes, why the call ended and what it consumed.
struct Traced<'o, O> {
    inner: O,
    deadline: Option<Instant>,
    calls: Vec<(FastPoint, Call)>,
    truncated: bool,
    _p: std::marker::PhantomData<&'o ()>,
}

/// Direct `S₅` oracles as framework oracles.
enum S5Kind {
    Sym(Box<SymS5>),
    X(Box<XS5>),
}

struct S5Oracle<'i> {
    inst: &'i BinaryInstance,
    kind: S5Kind,
    engine: SolverEngine,
    budget: usize,
    totals: SolverTotals,
    shape: SystemShape,
    last: Call,
}

impl<'i> S5Oracle<'i> {
    fn new(inst: &'i BinaryInstance, kind: S5Kind, budget: usize) -> Self {
        let (eqs, nv) = match &kind {
            S5Kind::Sym(s) => (s.pre.a[0].coords.clone(), s.pre.n_vars),
            S5Kind::X(x) => (x.pre.a[0].coords.clone(), x.pre.n_vars),
        };
        let shape = BooleanSystem {
            equations: eqs,
            n_vars: nv,
        }
        .shape();
        S5Oracle {
            inst,
            kind,
            engine: SolverEngine::InheritedF4 { max_degree: 3 },
            budget,
            totals: SolverTotals {
                solver: "direct S5, inherited-f4 (cap 3)".into(),
                ..Default::default()
            },
            shape,
            last: Call::default(),
        }
    }
}

impl<'a> DecompositionOracle<BinaryGroup<'a>> for S5Oracle<'_> {
    fn name(&self) -> &str {
        match self.kind {
            S5Kind::Sym(_) => "r6-sym-s5",
            S5Kind::X(_) => "r6-x-s5",
        }
    }
    fn summands(&self) -> u32 {
        4
    }
    fn describe(&self, _p: &Params) -> String {
        format!("direct {} S5, node budget {}", self.name(), self.budget)
    }
    fn decompose(
        &mut self,
        ctx: &InstanceCtx<BinaryGroup<'a>>,
        fb: &FactorBase<FastPoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: FastPoint,
    ) -> Option<Vec<usize>> {
        let (xs, call): (Option<Vec<u64>>, Call) = match &self.kind {
            S5Kind::Sym(s) => {
                let t = self.inst.fast.lower(point);
                let (r, c) = s.decompose(self.inst, &t, self.engine, self.budget);
                (
                    r.map(|(idx, _)| {
                        idx.iter()
                            .map(|&i| self.inst.fast.lift(&s.fb_u.points[i]).x)
                            .collect()
                    }),
                    c,
                )
            }
            S5Kind::X(x) => {
                let (r, c) = x.decompose(self.inst, point, self.engine, self.budget);
                (r.map(|idx| idx.iter().map(|&i| x.points[i].x).collect()), c)
            }
        };
        let cost = SolverCost {
            ops: call.word_xors,
            op_unit: "word XORs (elimination, specialisation and linear elimination only)".into(),
            wall_ns: call.wall_ns,
            peak_bytes: 0,
            degree_reached: None,
            solving_degree: None,
            timed_out: call.outcome == "budget",
            extra: BTreeMap::new(),
        };
        self.totals
            .absorb(&self.shape, Some(&cost), call.outcome == "budget");
        self.last = call;
        let xs = xs?;
        let r = lift_abscissae(ctx.group, fb, ops, &xs, point);
        if r.is_none() {
            counters.lift_failures += 1;
            self.last.outcome = "lift_failure";
        }
        r
    }
    fn last_system(&self) -> Option<SystemShape> {
        Some(self.shape.clone())
    }
    fn solver_totals(&self) -> Option<SolverTotals> {
        Some(self.totals.clone())
    }
}

/// What the last call of a wrapped oracle did, read without changing it.
trait LastCall {
    fn last_call(&self, xors: u64, budget_delta: u64, result: bool) -> Call;
}

impl LastCall for S5Oracle<'_> {
    fn last_call(&self, _x: u64, _b: u64, _r: bool) -> Call {
        self.last.clone()
    }
}

impl LastCall for ChainedXOracle<'_> {
    fn last_call(&self, _x: u64, _b: u64, _r: bool) -> Call {
        match self.calls.last() {
            Some(c) => Call {
                outcome: c.outcome,
                reductions: c.reductions,
                splits: c.splits,
                word_xors: c.word_xors,
                wall_ns: c.wall_ns,
                build_ns: 0,
            },
            None => Call::default(),
        }
    }
}

/// The library's symmetrised oracle exposes no reduction count; the call
/// is classified from its result and its budget counter, and the
/// reductions are filled in by the replay through the panel path.
impl LastCall for SymmetrisedOracle<'_> {
    fn last_call(&self, xors: u64, budget_delta: u64, result: bool) -> Call {
        Call {
            outcome: if result {
                "found"
            } else if budget_delta > 0 {
                "budget"
            } else {
                "refuted"
            },
            word_xors: xors,
            ..Default::default()
        }
    }
}

impl LastCall for MitmOracle {
    fn last_call(&self, _x: u64, _b: u64, result: bool) -> Call {
        Call {
            outcome: if result { "found" } else { "refuted" },
            ..Default::default()
        }
    }
}

impl<'a, O: DecompositionOracle<BinaryGroup<'a>> + LastCall> DecompositionOracle<BinaryGroup<'a>>
    for Traced<'_, O>
{
    fn name(&self) -> &str {
        self.inner.name()
    }
    fn summands(&self) -> u32 {
        self.inner.summands()
    }
    fn describe(&self, p: &Params) -> String {
        self.inner.describe(p)
    }
    fn prepare(
        &mut self,
        ctx: &InstanceCtx<BinaryGroup<'a>>,
        fb: &FactorBase<FastPoint>,
        params: &Params,
        ops: &mut GroupOps,
    ) -> Result<(), String> {
        self.inner.prepare(ctx, fb, params, ops)
    }
    fn setup_native(&self) -> BTreeMap<String, u64> {
        self.inner.setup_native()
    }
    fn decompose(
        &mut self,
        ctx: &InstanceCtx<BinaryGroup<'a>>,
        fb: &FactorBase<FastPoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: FastPoint,
    ) -> Option<Vec<usize>> {
        if self.deadline.is_some_and(|d| Instant::now() > d) {
            self.truncated = true;
            return None;
        }
        let b0 = self.inner.solver_totals().map_or(0, |t| t.budget_exceeded);
        let x0 = f4_word_ops_thread();
        let r = self.inner.decompose(ctx, fb, ops, counters, point);
        let xors = f4_word_ops_thread().wrapping_sub(x0);
        let b1 = self.inner.solver_totals().map_or(0, |t| t.budget_exceeded);
        let call = self.inner.last_call(xors, b1 - b0, r.is_some());
        self.calls.push((point, call));
        r
    }
    fn last_system(&self) -> Option<SystemShape> {
        self.inner.last_system()
    }
    fn solver_totals(&self) -> Option<SolverTotals> {
        self.inner.solver_totals()
    }
}

fn traced<'o, O>(inner: O, wall_s: f64) -> Traced<'o, O> {
    Traced {
        inner,
        deadline: (wall_s > 0.0)
            .then(|| Instant::now() + std::time::Duration::from_secs_f64(wall_s)),
        calls: Vec::new(),
        truncated: false,
        _p: std::marker::PhantomData,
    }
}

fn spec(name: &str, oracle: &str, oracle_spec: &str, max_trials: u64, seed: u64) -> PipelineSpec {
    let (_, op) = Params::parse_spec(oracle_spec).unwrap();
    PipelineSpec {
        factor_base: name.into(),
        factor_base_params: Params::default(),
        oracle: oracle.into(),
        oracle_params: op,
        solver: None,
        solver_params: Params::default(),
        targets: Targets::Random,
        linalg: String::new(),
        max_trials,
        seed,
        solver_budget_seconds: 0,
    }
}

fn calibration(inst: &BinaryInstance) -> crypto_lib::cryptanalysis::ic_boundary::Calibration {
    let mut calib = calibrate_binary_instance(inst);
    calibrate_row_ops(inst.r, &mut calib);
    let _ = calib.pin("koblitz", &inst.name);
    calib
}

fn report_json(rep: &RunReport, calls: &[(FastPoint, Call)], truncated: bool) -> Value {
    let mut outcomes: BTreeMap<&str, u64> = BTreeMap::new();
    for (_, c) in calls {
        *outcomes.entry(c.outcome).or_insert(0) += 1;
    }
    let reductions: u64 = calls.iter().map(|(_, c)| c.reductions).sum();
    json!({
        "label": rep.label, "instance": rep.instance, "r": rep.r,
        "columns": rep.factor_base.columns,
        "targets_tried": rep.decomposition.targets_tried,
        "relations_found": rep.decomposition.relations_found,
        "hit_rate": rep.decomposition.hit_rate,
        "solver_budget_exceeded": rep.decomposition.solver.as_ref().map(|s| s.budget_exceeded),
        "solver_calls": rep.decomposition.solver.as_ref().map(|s| s.calls),
        "solver_ops": rep.decomposition.solver.as_ref().map(|s| s.ops),
        "verified": rep.verified, "exhausted": rep.exhausted,
        "total_gae": rep.total_gae, "S": rep.s,
        "wall_s_contended": rep.wall_seconds,
        "wall_truncated": truncated,
        "call_outcomes": outcomes, "reductions_total": reductions,
        "full_report": serde_json::to_value(rep).unwrap_or(Value::Null),
    })
}

// ── The discrepancy protocol ───────────────────────────────────────

/// The panel path of round 4, instrumented: the body of
/// `symmetrised_groebner_decompose` with its `SolveStats` kept.
fn panel_sym(
    kc: &KoblitzCurve,
    fb: &SymmetrisedFactorBase,
    st: &FieldStructure,
    target: &BinaryPoint,
    m: usize,
    budget: usize,
) -> (bool, Call) {
    let mut call = Call::default();
    let Some(sys) = build_symmetrised_system(kc, fb, target, m, st) else {
        call.outcome = "unbuildable";
        return (false, call);
    };
    let before = f4_word_ops_thread();
    let t0 = Instant::now();
    let mut found = false;
    let (_, stats) = solve_boolean_system_filtered(
        &sys.equations,
        sys.n_vars,
        &solve_opts(SolverEngine::InheritedF4 { max_degree: 3 }, budget),
        |root| {
            let ok =
                lift_symmetrised_root_accepting(kc, fb, &sys, root, target, |_| true).is_some();
            found |= ok;
            ok
        },
    );
    call.word_xors = f4_word_ops_thread().wrapping_sub(before);
    call.wall_ns = t0.elapsed().as_nanos() as u64;
    call.reductions = stats.reductions as u64;
    call.splits = stats.splits as u64;
    call.outcome = outcome_of(found, &stats);
    (found, call)
}

fn cmd_discrepancy(args: &Args) {
    let n = args.u("n", 23) as u32;
    let a = args.u("a", 0) as u8;
    let budget = args.u("node-budget", 4096) as usize;
    let panel_targets = args.u("panel-targets", 24) as usize;
    let max_trials = args.u("max-trials", 300);
    let seed = args.u("seed", 123_212_651_130);
    let panel_seed = args.u("panel-seed", 0x16_1C_0550);
    let out = args.get("json", "discrepancy.json");
    let inst = koblitz_instance(a, n).expect("instance");
    let kc = inst.koblitz.clone().unwrap();
    let st = FieldStructure::new(n, &kc.curve.irreducible);
    let fb_lib = build_symmetrised_factor_base(&kc, &[0, 1]).expect("divisor 0;1");
    // The panel rebuilt F_u from the basis; check it is the same base.
    let fb_u = build_symmetrised_factor_base_from_basis(&kc, fb_lib.v_basis.clone()).unwrap();
    assert_eq!(fb_u.points.len(), fb_lib.points.len());
    eprintln!("n={n} a={a} ell={} |F_u|={}", fb_u.ell, fb_u.points.len());

    // (1) The round-4 panel targets, reproduced from its seed rule, then more.
    let mut rng = StdRng::seed_from_u64(panel_seed ^ ((n as u64) << 32) ^ a as u64);
    let g = kc.generator().clone();
    let r = inst.r;
    let mut ptargets: Vec<BinaryPoint> = Vec::new();
    while ptargets.len() < panel_targets {
        let p = kc.mul(&g, &BigUint::from(rng.gen_range(1..r)));
        if let BinaryPoint::Affine { x, .. } = &p {
            if *x != F2mElement::one(n) {
                ptargets.push(p);
            }
        }
    }
    // Framework oracle on the same targets.
    let group = BinaryGroup(&inst.fast);
    let ctx = ctx_for(&inst, &group, inst.generator);
    let base = KoblitzSymmetrisedBase { instance: &inst };
    let (_, bparams) = Params::parse_spec("koblitz-symmetrised:divisor=0;1").unwrap();
    let mut ops = GroupOps::default();
    let fb = base.build(&ctx, &bparams, &mut ops).unwrap();
    let (_, oparams) =
        Params::parse_spec(&format!("symmetrised:m=3,divisor=0;1,node_budget={budget}")).unwrap();
    let mut lib_oracle = traced(SymmetrisedOracle::new(3, &inst), 0.0);
    lib_oracle.prepare(&ctx, &fb, &oparams, &mut ops).unwrap();
    let mut counters = OracleCounters::default();
    let mut panel_rows = Vec::new();
    let (mut p_found, mut f_found, mut agree, mut xor_equal) = (0, 0, 0, 0);
    for (i, t) in ptargets.iter().enumerate() {
        let (pf, pc) = panel_sym(&kc, &fb_u, &st, t, 3, budget);
        let ft = inst.fast.lift(t);
        let fr = lib_oracle.decompose(&ctx, &fb, &mut ops, &mut counters, ft);
        let fc = lib_oracle.calls.last().unwrap().1.clone();
        p_found += pf as u64;
        f_found += fr.is_some() as u64;
        agree += (pc.outcome == fc.outcome) as u64;
        xor_equal += (pc.word_xors == fc.word_xors) as u64;
        panel_rows.push(json!({"target": i, "panel": pc.json(), "framework": fc.json()}));
        if i < 24 || i % 24 == 23 {
            eprintln!(
                "  panel target {i}: panel {} ({} red, {} XOR)  framework {} ({} XOR)",
                pc.outcome, pc.reductions, pc.word_xors, fc.outcome, fc.word_xors
            );
        }
    }
    eprintln!(
        "panel targets: panel found {p_found}/{panel_targets}, framework found {f_found}/{panel_targets}, outcomes agree {agree}, XOR counts equal {xor_equal}"
    );

    // (2) The pipeline `ic bench` runs (same base, oracle, calibration and
    // target source), every target logged, then replayed through the panel.
    let calib = calibration(&inst);
    let planted = 1 + seed % (r - 1);
    let mut ops2 = GroupOps::default();
    let q = group.mul(&mut ops2, inst.generator, planted);
    let ctx2 = ctx_for(&inst, &group, q);
    let pspec = PipelineSpec {
        factor_base: "koblitz-symmetrised".into(),
        factor_base_params: bparams.clone(),
        oracle: "symmetrised".into(),
        oracle_params: oparams.clone(),
        solver: None,
        solver_params: Params::default(),
        targets: Targets::Random,
        linalg: String::new(),
        max_trials,
        seed,
        solver_budget_seconds: 0,
    };
    let mut pipe_oracle = traced(SymmetrisedOracle::new(3, &inst), 0.0);
    let started = Instant::now();
    let rep = run_pipeline(
        &ctx2,
        &pspec,
        &base,
        &mut pipe_oracle,
        planted,
        &calib,
        None,
    )
    .expect("pipeline");
    eprintln!(
        "pipeline: {} trials, {} relations (hit {:.3}), budget-exceeded {:?}, verified {}, S {:.3e}, {:.0}s",
        rep.decomposition.targets_tried,
        rep.decomposition.relations_found,
        rep.decomposition.hit_rate,
        rep.decomposition.solver.as_ref().map(|s| s.budget_exceeded),
        rep.verified,
        rep.s,
        started.elapsed().as_secs_f64()
    );
    let mut pipe_rows = Vec::new();
    let (mut rp_found, mut rp_agree, mut rp_xor) = (0u64, 0u64, 0u64);
    let mut red_by_outcome: BTreeMap<&str, Vec<u64>> = BTreeMap::new();
    for (i, (fp, fcall)) in pipe_oracle.calls.iter().enumerate() {
        let fp = *fp;
        let t = inst.fast.lower(fp);
        let (pf, pc) = panel_sym(&kc, &fb_u, &st, &t, 3, budget);
        rp_found += pf as u64;
        rp_agree += (pc.outcome == fcall.outcome) as u64;
        rp_xor += (pc.word_xors == fcall.word_xors) as u64;
        red_by_outcome
            .entry(pc.outcome)
            .or_default()
            .push(pc.reductions);
        pipe_rows.push(
            json!({"call": i, "framework": fcall.json(), "panel_replay": pc.json(),
                              "in_subgroup": inst.fast.mul_u64(fp, r).infinity}),
        );
    }
    let n_calls = pipe_oracle.calls.len() as u64;
    eprintln!(
        "pipeline targets replayed in the panel path: found {rp_found}/{n_calls}, outcomes agree {rp_agree}, XOR equal {rp_xor}"
    );
    let summary: BTreeMap<&str, Value> = red_by_outcome
        .iter()
        .map(|(k, v)| {
            (
                *k,
                json!({"count": v.len(), "reductions_min": v.iter().min(), "reductions_max": v.iter().max(),
                       "reductions_mean": v.iter().sum::<u64>() as f64 / v.len().max(1) as f64}),
            )
        })
        .collect();
    write_json(
        &out,
        &json!({
            "experiment": "R6.1 discrepancy protocol: panel vs ic-pipeline hit rate, symmetrised S4, invariant divisor 0;1",
            "n": n, "a": a, "ell": fb_u.ell, "fu_points": fb_u.points.len(), "node_budget": budget,
            "engine": "InheritedF4 { max_degree: 3 }, default split rule",
            "panel_seed": panel_seed, "pipeline_seed": seed, "max_trials": max_trials,
            "panel_targets": {"count": panel_targets, "panel_found": p_found, "framework_found": f_found,
                              "outcomes_agree": agree, "word_xors_equal": xor_equal, "rows": panel_rows},
            "pipeline": {"report": report_json(&rep, &[], false), "calls": n_calls,
                         "replay_found": rp_found, "replay_outcomes_agree": rp_agree,
                         "replay_word_xors_equal": rp_xor, "replay_reductions_by_outcome": summary,
                         "rows": pipe_rows},
        }),
    );
}

// ── The m = 4 panel ────────────────────────────────────────────────

fn interp_or_load(arg: &str) -> (Poly5, Poly5, Value) {
    // Interpolate both S5 once, on n = 29, a = 1 (field-size independent
    // F₂ coefficients; the interpolate command verifies that).
    let _ = arg;
    let inst = koblitz_instance(1, 29).expect("n29");
    let t0 = Instant::now();
    let (sym, ss) = interpolate(&inst, Frame::Sym, 5, 0x5_1A7E ^ 5);
    let (xp, xs) = interpolate(&inst, Frame::X, 5, 0x5_1A7E ^ 5);
    assert_eq!(ss.nullity, 1, "symmetrised S5 not unique");
    assert_eq!(xs.nullity, 1, "x-frame S5 not unique");
    let info = json!({
        "sym": {"samples": ss.samples, "field_mults": ss.mults, "word_xors": ss.word_xors, "wall_s": ss.wall_s},
        "x": {"samples": xs.samples, "field_mults": xs.mults, "word_xors": xs.word_xors, "wall_s": xs.wall_s},
        "wall_s_contended": t0.elapsed().as_secs_f64(),
    });
    (sym, xp, info)
}

fn cmd_panel(args: &Args) {
    for var in ENGINE_ENV {
        assert!(env::var(var).is_err(), "the run needs {var} unset");
    }
    let n = args.u("n", 17) as u32;
    let a = args.u("a", 1) as u8;
    let m = 4usize;
    let targets = args.u("targets", 24) as usize;
    let budget = args.u("node-budget", 4096) as usize;
    let seed = args.u("seed", 0x16_1C_0550);
    let cell_wall = args.f("cell-wall-s", 180.0);
    let pre_cap = args.f("pre-cap-s", 300.0);
    let arms_arg = args.get("arms", "x-chained,sym-chained,sym-s5,x-s5");
    let arms: Vec<&str> = arms_arg.split(',').collect();
    let ell = args.u("ell", rule_ell(n, m) as u64) as usize;
    let out = args.get("json", "panel.json");
    let inst = koblitz_instance(a, n).expect("instance");
    let kc = inst.koblitz.clone().unwrap();
    let fc = &inst.fast;
    let r = inst.r;
    let (sym_poly, x_poly, interp_info) = interp_or_load("");
    let mut rng = StdRng::seed_from_u64(seed ^ ((n as u64) << 32) ^ a as u64);
    let g = kc.generator().clone();
    let mut tlist: Vec<BinaryPoint> = Vec::new();
    while tlist.len() < targets {
        let p = kc.mul(&g, &BigUint::from(rng.gen_range(1..r)));
        if let BinaryPoint::Affine { x, .. } = &p {
            if *x != F2mElement::one(n) {
                tlist.push(p);
            }
        }
    }
    let ft: Vec<FastPoint> = tlist.iter().map(|t| fc.lift(t)).collect();
    let mut vrng =
        StdRng::seed_from_u64(seed ^ ((n as u64) << 32) ^ ((m as u64) << 8) ^ ((a as u64) << 4));
    let v = random_subspace_containing_one(&kc, ell, &mut vrng);
    let fb_x = plain_subspace_factor_base(&kc, &v).expect("F_x");
    let fb_u = build_symmetrised_factor_base_from_basis(&kc, v.clone()).expect("F_u");
    let t_fast = fc.lift(&fb_u.two_torsion);
    let fx_pts: Vec<FastPoint> = fb_x.points.iter().map(|p| fc.lift(p)).collect();
    let fu_pts: Vec<FastPoint> = fb_u.points.iter().map(|p| fc.lift(p)).collect();
    let ex_x = Exact::new(&inst, &fx_pts);
    let ex_u = Exact::new(&inst, &fu_pts);
    let truth_x: Vec<bool> = ft.iter().map(|&t| ex_x.decomposes(&inst, t, m)).collect();
    let truth_u: Vec<bool> = ft
        .iter()
        .map(|&t| ex_u.decomposes(&inst, t, m) || ex_u.decomposes(&inst, fc.add(t, t_fast), m))
        .collect();
    let base_x = FixedBase::new(&inst, &fb_x.points, &v, "F_x random V", true);
    let base_u = FixedBase::new(&inst, &fb_u.points, &v, "F_u random V", false);
    eprintln!(
        "n={n} a={a} r={r} ell={ell} |F_x|={} |F_u|={} cols_x={} cols_u={} truth x {} u {}",
        fx_pts.len(),
        fu_pts.len(),
        base_x.fb.columns,
        base_u.fb.columns,
        truth_x.iter().filter(|&&b| b).count(),
        truth_u.iter().filter(|&&b| b).count()
    );
    let group = BinaryGroup(&inst.fast);
    let ctx = ctx_for(&inst, &group, inst.generator);
    let engine = SolverEngine::InheritedF4 { max_degree: 3 };
    let mut cells = Vec::new();
    for arm in arms {
        let on_u = arm.starts_with("sym");
        let truth = if on_u { &truth_u } else { &truth_x };
        let vars = match arm {
            "x-chained" => 4 * ell + 2 * n as usize,
            "sym-chained" => 4 * (ell - 1) + 1 + 2 * n as usize,
            "sym-s5" => 4 * (ell - 1) + 1,
            "x-s5" => 4 * ell,
            _ => panic!("arm {arm}"),
        };
        let mut pre_info = Value::Null;
        let mut sym_s5 = None;
        let mut x_s5 = None;
        let mut chained = None;
        let mut refused: Option<String> = None;
        if vars > MAX_VARS {
            refused = Some(format!("{vars} unknowns > {MAX_VARS}"));
        } else {
            match arm {
                "sym-s5" => {
                    let s = SymS5::new(&kc, &fb_u, &sym_poly, pre_cap).unwrap();
                    pre_info = json!({"build_s_contended": s.pre.build_s, "products": s.pre.products,
                                      "complete": s.pre.complete, "max_terms_per_coordinate": s.pre.max_terms_per_coordinate});
                    if !s.pre.complete {
                        refused = Some(format!("target-independent build exceeded {pre_cap} s"));
                    }
                    sym_s5 = Some(s);
                }
                "x-s5" => {
                    let s = XS5::new(&kc, &inst, &v, &fb_x.points, &x_poly, pre_cap).unwrap();
                    pre_info = json!({"build_s_contended": s.pre.build_s, "products": s.pre.products,
                                      "complete": s.pre.complete, "max_terms_per_coordinate": s.pre.max_terms_per_coordinate});
                    if !s.pre.complete {
                        refused = Some(format!("target-independent build exceeded {pre_cap} s"));
                    }
                    x_s5 = Some(s);
                }
                "x-chained" => {
                    let mut o = ChainedXOracle::new(4, &inst);
                    let (_, p) =
                        Params::parse_spec(&format!("chained-x:m=4,node_budget={budget},trace=1"))
                            .unwrap();
                    let mut ops = GroupOps::default();
                    match o.prepare(&ctx, &base_x.fb, &p, &mut ops) {
                        Ok(()) => chained = Some(o),
                        Err(e) => refused = Some(e),
                    }
                }
                _ => {}
            }
        }
        eprintln!("  {arm}: precompute {pre_info}");
        let mut rows = Vec::new();
        let (mut found, mut refuted, mut bud, mut gate, mut xors, mut reds, mut wall, mut build) =
            (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
        let mut attempts = 0u64;
        let mut truncated = false;
        let started = Instant::now();
        if refused.is_none() {
            for (ti, t) in tlist.iter().enumerate() {
                if started.elapsed().as_secs_f64() > cell_wall {
                    truncated = true;
                    break;
                }
                let tf = ft[ti];
                let (rel_sum, call): (Option<FastPoint>, Call) = match arm {
                    "x-chained" => {
                        let o = chained.as_mut().unwrap();
                        let mut ops = GroupOps::default();
                        let mut counters = OracleCounters::default();
                        let r = o.decompose(&ctx, &base_x.fb, &mut ops, &mut counters, tf);
                        let c = o.last_call(0, 0, false);
                        (
                            r.map(|idx| {
                                idx.iter().fold(FastPoint::INFINITY, |s, &i| {
                                    fc.add(s, base_x.fb.points[i])
                                })
                            }),
                            c,
                        )
                    }
                    "sym-chained" => {
                        let before = f4_word_ops_thread();
                        let t0 = Instant::now();
                        let oc = chained_symmetrised_groebner_decompose(
                            &kc,
                            &fb_u,
                            &FieldStructure::new(n, &kc.curve.irreducible),
                            t,
                            engine,
                            budget,
                        );
                        let mut c = Call {
                            word_xors: f4_word_ops_thread().wrapping_sub(before),
                            wall_ns: t0.elapsed().as_nanos() as u64,
                            ..Default::default()
                        };
                        match oc {
                            None => {
                                c.outcome = "unbuildable";
                                (None, c)
                            }
                            Some(oc) => {
                                c.splits = oc.effort;
                                c.outcome = if oc.relation.is_some() {
                                    "found"
                                } else if oc.complete {
                                    "refuted"
                                } else {
                                    "budget"
                                };
                                (
                                    oc.relation.map(|idx| {
                                        idx.iter()
                                            .fold(FastPoint::INFINITY, |s, &i| fc.add(s, fu_pts[i]))
                                    }),
                                    c,
                                )
                            }
                        }
                    }
                    "sym-s5" => {
                        let (r, c) = sym_s5.as_ref().unwrap().decompose(&inst, t, engine, budget);
                        (
                            r.map(|(idx, _)| {
                                idx.iter()
                                    .fold(FastPoint::INFINITY, |s, &i| fc.add(s, fu_pts[i]))
                            }),
                            c,
                        )
                    }
                    "x-s5" => {
                        let (r, c) = x_s5.as_ref().unwrap().decompose(&inst, tf, engine, budget);
                        (
                            r.map(|idx| {
                                idx.iter()
                                    .fold(FastPoint::INFINITY, |s, &i| fc.add(s, fx_pts[i]))
                            }),
                            c,
                        )
                    }
                    _ => unreachable!(),
                };
                attempts += 1;
                xors += call.word_xors;
                reds += call.reductions;
                wall += call.wall_ns;
                build += call.build_ns;
                match call.outcome {
                    "found" => {
                        let s = rel_sum.unwrap();
                        let ok = s == tf || (on_u && s == fc.add(tf, t_fast));
                        if !ok || !truth[ti] {
                            gate += 1;
                        }
                        found += 1;
                    }
                    "refuted" => {
                        if truth[ti] {
                            gate += 1;
                        }
                        refuted += 1;
                    }
                    "budget" => bud += 1,
                    _ => {}
                }
                eprintln!(
                    "    {arm} target {ti}: {} red {} XOR {} {:.1}s",
                    call.outcome,
                    call.reductions,
                    call.word_xors,
                    call.wall_ns as f64 / 1e9
                );
                let mut row = call.json();
                row["target"] = json!(ti);
                row["truth"] = json!(truth[ti]);
                rows.push(row);
            }
        }
        let solved_frac = if attempts > 0 {
            (found + refuted) as f64 / attempts as f64
        } else {
            0.0
        };
        let feasible = refused.is_none() && found > 0 && solved_frac >= 0.5 && gate == 0;
        eprintln!(
            "  n={n} a={a} m=4 ell={ell} {arm:11} vars {vars:2} found {found:2} refuted {refuted:2} budget {bud:2} gate {gate} XOR/rel {:.3e} red/att {:.0} wall {:.1}s build {:.1}s{}{}",
            if found > 0 { xors as f64 / found as f64 } else { f64::INFINITY },
            if attempts > 0 { reds as f64 / attempts as f64 } else { 0.0 },
            wall as f64 / 1e9,
            build as f64 / 1e9,
            refused.as_ref().map(|r| format!(" REFUSED: {r}")).unwrap_or_default(),
            if truncated { " WALL-TRUNCATED" } else { "" }
        );
        cells.push(json!({
            "arm": arm, "m": m, "ell": ell, "n_vars": vars,
            "frame": if on_u { "u" } else { "x" },
            "base_points": if on_u { fu_pts.len() } else { fx_pts.len() },
            "columns_projected": if on_u { base_u.fb.columns } else { base_x.fb.columns },
            "refused": refused, "precompute": pre_info,
            "attempts": attempts, "found": found, "refuted": refuted, "budget_exhausted": bud,
            "truth_decomposable": truth.iter().take(attempts as usize).filter(|&&b| b).count(),
            "gate_failures": gate, "solved_fraction": solved_frac, "feasible": feasible,
            "word_xors": xors, "xor_per_relation": if found > 0 { xors as f64 / found as f64 } else { f64::INFINITY },
            "reductions": reds,
            "solve_wall_s_contended": wall as f64 / 1e9, "build_wall_s_contended": build as f64 / 1e9,
            "wall_truncated": truncated, "per_attempt": rows,
        }));
        write_json(
            &out,
            &json!({
                "experiment": "R6.1 m = 4 panel: chained x (plug-in), chained sym (control), direct sym S5, direct x S5",
                "n": n, "a": a, "r": r, "h": inst.cofactor, "ell": ell, "node_budget": budget, "targets": targets,
                "seed": seed, "v_basis": v.iter().map(bits).collect::<Vec<_>>(),
                "engine": "InheritedF4 { max_degree: 3 }, default split rule",
                "feasible_rule": "found >= 1, (found + refuted)/attempts >= 0.5, zero gate failures",
                "interpolation": interp_info,
                "fx_points": fx_pts.len(), "fu_points": fu_pts.len(),
                "cols_x": base_x.fb.columns, "cols_u": base_u.fb.columns,
                "cells": cells,
            }),
        );
    }
    write_json(
        &out,
        &json!({
            "experiment": "R6.1 m = 4 panel: chained x (plug-in), chained sym (control), direct sym S5, direct x S5",
            "n": n, "a": a, "r": r, "h": inst.cofactor, "ell": ell, "node_budget": budget, "targets": targets,
            "seed": seed, "v_basis": v.iter().map(bits).collect::<Vec<_>>(),
            "engine": "InheritedF4 { max_degree: 3 }, default split rule",
            "feasible_rule": "found >= 1, (found + refuted)/attempts >= 0.5, zero gate failures",
            "interpolation": interp_info,
            "fx_points": fx_pts.len(), "fu_points": fu_pts.len(),
            "cols_x": base_x.fb.columns, "cols_u": base_u.fb.columns,
            "cells": cells,
        }),
    );
}

// ── End to end ─────────────────────────────────────────────────────

fn cmd_e2e(args: &Args) {
    for var in ENGINE_ENV {
        assert!(env::var(var).is_err(), "the run needs {var} unset");
    }
    let n = args.u("n", 17) as u32;
    let a = args.u("a", 1) as u8;
    let budget = args.u("node-budget", 4096) as usize;
    let seed = args.u("seed", 123_212_651_130);
    let vseed = args.u("v-seed", 0x16_1C_0550);
    let wall = args.f("arm-wall-s", 600.0);
    let max_trials = args.u("max-trials", 20000);
    let arms_arg = args.get("arms", "mitm-m2,chained-m3,chained-m4,sym-s5");
    let out = args.get("json", "e2e.json");
    let inst = koblitz_instance(a, n).expect("instance");
    let kc = inst.koblitz.clone().unwrap();
    let r = inst.r;
    let calib = calibration(&inst);
    let group = BinaryGroup(&inst.fast);
    let planted = 1 + seed % (r - 1);
    let mut ops = GroupOps::default();
    let q = group.mul(&mut ops, inst.generator, planted);
    let ctx = ctx_for(&inst, &group, q);
    let needs_s5 = arms_arg.contains("s5");
    let polys = needs_s5.then(|| interp_or_load(""));
    let mut rows = Vec::new();
    for arm in arms_arg.split(',') {
        let m: usize = match arm {
            "mitm-m2" | "chained-m2" => 2,
            "chained-m3" => 3,
            _ => 4,
        };
        let ell = args.u(&format!("ell-{m}"), rule_ell(n, m) as u64) as usize;
        let mut vrng = StdRng::seed_from_u64(
            vseed ^ ((n as u64) << 32) ^ ((m as u64) << 8) ^ ((a as u64) << 4),
        );
        let v = random_subspace_containing_one(&kc, ell, &mut vrng);
        let fb_x = plain_subspace_factor_base(&kc, &v).unwrap();
        let started = Instant::now();
        let (rep, calls, truncated, extra) = match arm {
            "mitm-m2" => {
                let base = FixedBase::new(&inst, &fb_x.points, &v, "F_x random V", true);
                let mut o = traced(MitmOracle::new(2), wall);
                let rep = run_pipeline(
                    &ctx,
                    &spec("r6-fixed-subspace", "mitm", "mitm:m=2", max_trials, seed),
                    &base,
                    &mut o,
                    planted,
                    &calib,
                    None,
                );
                (rep, o.calls, o.truncated, Value::Null)
            }
            "chained-m2" | "chained-m3" | "chained-m4" => {
                let base = FixedBase::new(&inst, &fb_x.points, &v, "F_x random V", true);
                let mut o = traced(ChainedXOracle::new(m as u32, &inst), wall);
                let sp = spec(
                    "r6-fixed-subspace",
                    "chained-x",
                    &format!("chained-x:m={m},node_budget={budget},trace=1"),
                    max_trials,
                    seed,
                );
                let rep = run_pipeline(&ctx, &sp, &base, &mut o, planted, &calib, None);
                (rep, o.calls, o.truncated, Value::Null)
            }
            "sym-s5" | "x-s5" => {
                let (sym_poly, x_poly, info) = polys.as_ref().unwrap();
                let (base, kind, pre) = if arm == "sym-s5" {
                    let fb_u = build_symmetrised_factor_base_from_basis(&kc, v.clone()).unwrap();
                    let s = SymS5::new(&kc, &fb_u, sym_poly, wall).unwrap();
                    let pre = json!({"build_s_contended": s.pre.build_s, "products": s.pre.products, "complete": s.pre.complete});
                    (
                        FixedBase::new(&inst, &fb_u.points, &v, "F_u random V", false),
                        S5Kind::Sym(Box::new(s)),
                        pre,
                    )
                } else {
                    let s = XS5::new(&kc, &inst, &v, &fb_x.points, x_poly, wall).unwrap();
                    let pre = json!({"build_s_contended": s.pre.build_s, "products": s.pre.products, "complete": s.pre.complete});
                    (
                        FixedBase::new(&inst, &fb_x.points, &v, "F_x random V", true),
                        S5Kind::X(Box::new(s)),
                        pre,
                    )
                };
                let mut o = traced(S5Oracle::new(&inst, kind, budget), wall);
                let rep = run_pipeline(
                    &ctx,
                    &spec(
                        "r6-fixed-subspace",
                        o.inner.name(),
                        "x:m=4",
                        max_trials,
                        seed,
                    ),
                    &base,
                    &mut o,
                    planted,
                    &calib,
                    None,
                );
                (
                    rep,
                    o.calls,
                    o.truncated,
                    json!({"precompute": pre, "interpolation": info}),
                )
            }
            other => panic!("arm {other}"),
        };
        match rep {
            Ok(rep) => {
                eprintln!(
                    "n={n} a={a} {arm:10} ell={ell} cols {} trials {} rels {} hit {:.3} budget {:?} verified {} S {:.4e} wall {:.0}s{}",
                    rep.factor_base.columns,
                    rep.decomposition.targets_tried,
                    rep.decomposition.relations_found,
                    rep.decomposition.hit_rate,
                    rep.decomposition.solver.as_ref().map(|s| s.budget_exceeded),
                    rep.verified,
                    rep.s,
                    started.elapsed().as_secs_f64(),
                    if truncated { " WALL-TRUNCATED" } else { "" }
                );
                let mut j = report_json(&rep, &calls, truncated);
                j["arm"] = json!(arm);
                j["m"] = json!(m);
                j["ell"] = json!(ell);
                j["extra"] = extra;
                j["per_call"] = calls
                    .iter()
                    .map(|(_, c)| c.json())
                    .collect::<Vec<_>>()
                    .into();
                rows.push(j);
            }
            Err(e) => {
                eprintln!("n={n} a={a} {arm}: refused: {e}");
                rows.push(json!({"arm": arm, "m": m, "ell": ell, "refused": e}));
            }
        }
        write_json(
            &out,
            &json!({
                "experiment": "R6.1 end-to-end through ic_framework::run_pipeline (the ic bench pipeline)",
                "n": n, "a": a, "r": r, "h": inst.cofactor, "instance": inst.name,
                "node_budget": budget, "seed": seed, "v_seed": vseed, "planted": planted,
                "arm_wall_s": wall, "max_trials": max_trials,
                "rho_floor_S_derived": (std::f64::consts::PI / 4.0).sqrt(),
                "rows": rows,
            }),
        );
    }
}

fn main() {
    let raw: Vec<String> = env::args().skip(1).collect();
    let cmd = raw.first().cloned().unwrap_or_default();
    let args = Args::parse(&raw[1.min(raw.len())..]);
    match cmd.as_str() {
        "interpolate" => cmd_interpolate(&args),
        "discrepancy" => cmd_discrepancy(&args),
        "panel" => cmd_panel(&args),
        "e2e" => cmd_e2e(&args),
        _ => panic!("usage: r6_1_direct_s5 interpolate|discrepancy|panel|e2e [--key value …]"),
    }
}
