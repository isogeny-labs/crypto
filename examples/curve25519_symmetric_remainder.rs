#![recursion_limit = "256"]

//! Paired direct-remainder / explicit-quotient systems for
//! `research/curve25519_symmetric_remainder_20261125/`.
//!
//! This is a stage diagnostic, not an ECDLP algorithm.  The two arms share
//! the same cubic root polynomial, binary exponentiation schedule,
//! intermediate residues, symmetrised S4 equation, and targets.  The direct
//! arm eliminates each modular quotient; the quotient arm retains it so every
//! membership-gadget equation is quadratic.
//!
//! ```text
//! cargo run --release --example curve25519_symmetric_remainder -- selftest
//! RAYON_NUM_THREADS=1 cargo run --release --example \
//!   curve25519_symmetric_remainder -- cell --arm direct --d 19 \
//!   --kind random --index 0 --cap 300 --dmax 24
//! ```

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::time::{Duration, Instant};

use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Ordering, Poly};
use crypto_lib::cryptanalysis::ic_framework::subgroup_semaev::{
    primitive_root_of_unity, semaev_eval, symmetrise,
};

const P: u64 = 1_048_573;
const A: u64 = 63_700;
const B: u64 = 872_826;
const R: u64 = 131_203;
const GX: u64 = 349_837;
const GY: u64 = 122;
const LO: u64 = 349_833;
const SEED: u64 = 20_261_125;

#[inline]
fn addm(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}

#[inline]
fn subm(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b % p) % p
}

#[inline]
fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1;
    while e != 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}

fn invm(a: u64, p: u64) -> u64 {
    powm(a, p - 2, p)
}

fn sqrtm(a: u64, p: u64) -> Option<u64> {
    if a == 0 {
        return Some(0);
    }
    if powm(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(powm(a, (p + 1) / 4, p));
    }
    let (mut q, mut s) = (p - 1, 0u32);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while powm(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut c = powm(z, q, p);
    let mut x = powm(a, q.div_ceil(2), p);
    let mut t = powm(a, q, p);
    let mut m = s;
    while t != 1 {
        let mut i = 0u32;
        let mut tt = t;
        while tt != 1 {
            tt = mulm(tt, tt, p);
            i += 1;
        }
        let b = powm(c, 1u64 << (m - i - 1), p);
        x = mulm(x, b, p);
        c = mulm(b, b, p);
        t = mulm(t, c, p);
        m = i;
    }
    Some(x)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point(Option<(u64, u64)>);

#[derive(Clone, Copy)]
struct Curve {
    p: u64,
    a: u64,
    b: u64,
}

impl Curve {
    fn rhs(self, x: u64) -> u64 {
        addm(
            addm(
                mulm(mulm(x, x, self.p), x, self.p),
                mulm(self.a, x, self.p),
                self.p,
            ),
            self.b,
            self.p,
        )
    }

    fn on_curve(self, q: Point) -> bool {
        match q.0 {
            None => true,
            Some((x, y)) => mulm(y, y, self.p) == self.rhs(x),
        }
    }

    fn neg(self, q: Point) -> Point {
        Point(q.0.map(|(x, y)| (x, (self.p - y) % self.p)))
    }

    fn add(self, s: Point, t: Point) -> Point {
        let (Some((x1, y1)), Some((x2, y2))) = (s.0, t.0) else {
            return if s.0.is_none() { t } else { s };
        };
        let lam = if x1 == x2 {
            if addm(y1, y2, self.p) == 0 {
                return Point(None);
            }
            mulm(
                addm(mulm(3, mulm(x1, x1, self.p), self.p), self.a, self.p),
                invm(mulm(2, y1, self.p), self.p),
                self.p,
            )
        } else {
            mulm(
                subm(y2, y1, self.p),
                invm(subm(x2, x1, self.p), self.p),
                self.p,
            )
        };
        let x3 = subm(subm(mulm(lam, lam, self.p), x1, self.p), x2, self.p);
        let y3 = subm(mulm(lam, subm(x1, x3, self.p), self.p), y1, self.p);
        Point(Some((x3, y3)))
    }

    fn mul(self, mut k: u64, mut q: Point) -> Point {
        let mut acc = Point(None);
        while k != 0 {
            if k & 1 == 1 {
                acc = self.add(acc, q);
            }
            q = self.add(q, q);
            k >>= 1;
        }
        acc
    }
}

#[derive(Clone, Copy)]
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

thread_local! {
    static BUILD_OPS: Cell<u64> = const { Cell::new(0) };
}

fn build_reset() {
    BUILD_OPS.with(|c| c.set(0));
}

fn build_bump(n: u64) {
    BUILD_OPS.with(|c| c.set(c.get() + n));
}

fn build_ops() -> u64 {
    BUILD_OPS.with(Cell::get)
}

#[derive(Clone, Debug)]
struct Expr {
    n: usize,
    terms: BTreeMap<Vec<u32>, u64>,
}

impl Expr {
    fn zero(n: usize) -> Self {
        Self {
            n,
            terms: BTreeMap::new(),
        }
    }

    fn constant(n: usize, c: u64, p: u64) -> Self {
        let mut out = Self::zero(n);
        if c % p != 0 {
            out.terms.insert(vec![0; n], c % p);
        }
        out
    }

    fn variable(n: usize, i: usize) -> Self {
        let mut e = vec![0; n];
        e[i] = 1;
        let mut out = Self::zero(n);
        out.terms.insert(e, 1);
        out
    }

    fn add(&self, other: &Self, p: u64) -> Self {
        debug_assert_eq!(self.n, other.n);
        let mut out = self.clone();
        for (e, &c) in &other.terms {
            build_bump(1);
            let v = out.terms.entry(e.clone()).or_insert(0);
            *v = addm(*v, c, p);
        }
        out.terms.retain(|_, c| *c != 0);
        out
    }

    fn scale(&self, k: u64, p: u64) -> Self {
        let mut out = Self::zero(self.n);
        for (e, &c) in &self.terms {
            build_bump(1);
            let v = mulm(c, k, p);
            if v != 0 {
                out.terms.insert(e.clone(), v);
            }
        }
        out
    }

    fn sub(&self, other: &Self, p: u64) -> Self {
        self.add(&other.scale(p - 1, p), p)
    }

    fn mul(&self, other: &Self, p: u64) -> Self {
        let mut out = Self::zero(self.n);
        for (ea, &ca) in &self.terms {
            for (eb, &cb) in &other.terms {
                build_bump(2);
                let e: Vec<u32> = ea.iter().zip(eb).map(|(a, b)| a + b).collect();
                let v = out.terms.entry(e).or_insert(0);
                *v = addm(*v, mulm(ca, cb, p), p);
            }
        }
        out.terms.retain(|_, c| *c != 0);
        out
    }

    fn pow(&self, mut k: u32, p: u64) -> Self {
        let mut a = self.clone();
        let mut r = Self::constant(self.n, 1, p);
        while k != 0 {
            if k & 1 == 1 {
                r = r.mul(&a, p);
            }
            k >>= 1;
            if k != 0 {
                a = a.mul(&a, p);
            }
        }
        r
    }

    fn degree(&self) -> u32 {
        self.terms.keys().map(|e| e.iter().sum()).max().unwrap_or(0)
    }

    fn eval(&self, x: &[u64], p: u64) -> u64 {
        self.terms.iter().fold(0, |acc, (e, &c)| {
            let v = e.iter().enumerate().fold(c, |v, (i, &k)| {
                if k == 0 {
                    v
                } else {
                    mulm(v, powm(x[i], u64::from(k), p), p)
                }
            });
            addm(acc, v, p)
        })
    }

    fn to_poly(&self, p: u64) -> Poly {
        let terms: Vec<(Vec<u32>, u64)> = self.terms.iter().map(|(e, &c)| (e.clone(), c)).collect();
        f4_fp::normalise(&terms, p, Ordering::Grevlex)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    Direct,
    Quotient,
}

impl Arm {
    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "direct" | "D" => Ok(Self::Direct),
            "quotient" | "Q" => Ok(Self::Quotient),
            _ => Err(format!("unknown arm {s:?}; use direct or quotient")),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Quotient => "quotient",
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Square,
    TimesX,
}

fn schedule(d: u64) -> Vec<Op> {
    assert!(d >= 2);
    let top = 63 - d.leading_zeros();
    let mut out = Vec::new();
    for bit in (0..top).rev() {
        out.push(Op::Square);
        if d >> bit & 1 == 1 {
            out.push(Op::TimesX);
        }
    }
    out
}

#[derive(Clone, Copy)]
struct Config {
    curve: Curve,
    lo: u64,
    g: u64,
}

fn relation_expr(cfg: Config, n: usize, xt: u64, seed: u64) -> Result<Expr, String> {
    let p = cfg.curve.p;
    let e1 = Expr::variable(n, 0);
    let e2 = Expr::variable(n, 1);
    let e3 = Expr::variable(n, 2);
    let e1x = Expr::constant(n, 3 * cfg.lo % p, p).add(&e1.scale(cfg.g, p), p);
    let e2x = Expr::constant(n, 3 * mulm(cfg.lo, cfg.lo, p) % p, p)
        .add(&e1.scale(mulm(2 * cfg.lo % p, cfg.g, p), p), p)
        .add(&e2.scale(mulm(cfg.g, cfg.g, p), p), p);
    let e3x = Expr::constant(n, mulm(mulm(cfg.lo, cfg.lo, p), cfg.lo, p), p)
        .add(&e1.scale(mulm(mulm(cfg.lo, cfg.lo, p), cfg.g, p), p), p)
        .add(&e2.scale(mulm(cfg.lo, mulm(cfg.g, cfg.g, p), p), p), p)
        .add(&e3.scale(mulm(mulm(cfg.g, cfg.g, p), cfg.g, p), p), p);
    let ex = [e1x, e2x, e3x];
    let terms = symmetrise(3, xt, cfg.curve.a, cfg.curve.b, p, seed)?;
    let mut out = Expr::zero(n);
    for (powers, c) in terms {
        let mut t = Expr::constant(n, c, p);
        for (i, &k) in powers.iter().enumerate() {
            if k != 0 {
                t = t.mul(&ex[i].pow(k, p), p);
            }
        }
        out = out.add(&t, p);
    }
    Ok(out)
}

fn convolution(a: &[Expr; 3], b: &[Expr; 3], p: u64) -> [Expr; 5] {
    let n = a[0].n;
    let mut c: [Expr; 5] = std::array::from_fn(|_| Expr::zero(n));
    for i in 0..3 {
        for j in 0..3 {
            c[i + j] = c[i + j].add(&a[i].mul(&b[j], p), p);
        }
    }
    c
}

fn direct_remainder(a: &[Expr; 3], b: &[Expr; 3], e: &[Expr; 3], p: u64) -> [Expr; 3] {
    let c = convolution(a, b, p);
    let e1e3 = e[0].mul(&e[2], p);
    let e1e2 = e[0].mul(&e[1], p);
    let e1sq = e[0].mul(&e[0], p);
    let r0 = c[0].add(&c[3].mul(&e[2], p), p).add(&c[4].mul(&e1e3, p), p);
    let r1 = c[1]
        .sub(&c[3].mul(&e[1], p), p)
        .add(&c[4].mul(&e[2].sub(&e1e2, p), p), p);
    let r2 = c[2]
        .add(&c[3].mul(&e[0], p), p)
        .add(&c[4].mul(&e1sq.sub(&e[1], p), p), p);
    [r0, r1, r2]
}

struct Built {
    exprs: Vec<Expr>,
    polys: Vec<Poly>,
    n_vars: usize,
    input_terms: usize,
    input_degree: u32,
    membership_build_ops: u64,
    build_ms: f64,
}

fn build_system(cfg: Config, arm: Arm, d: u64, xt: u64, seed: u64) -> Result<Built, String> {
    let start = Instant::now();
    build_reset();
    let ops = schedule(d);
    let residues = ops.len();
    let n = 3
        + 3 * residues
        + if arm == Arm::Quotient {
            2 * residues
        } else {
            0
        };
    let p = cfg.curve.p;
    let e: [Expr; 3] = std::array::from_fn(|i| Expr::variable(n, i));
    let x = [Expr::zero(n), Expr::constant(n, 1, p), Expr::zero(n)];
    let mut current = x.clone();
    let mut equations = vec![relation_expr(cfg, n, xt, seed)?];
    for (step, op) in ops.iter().enumerate() {
        let other = match op {
            Op::Square => current.clone(),
            Op::TimesX => x.clone(),
        };
        let next: [Expr; 3] = std::array::from_fn(|k| Expr::variable(n, 3 + 3 * step + k));
        match arm {
            Arm::Direct => {
                let reduced = direct_remainder(&current, &other, &e, p);
                for k in 0..3 {
                    equations.push(next[k].sub(&reduced[k], p));
                }
            }
            Arm::Quotient => {
                let c = convolution(&current, &other, p);
                let h0 = Expr::variable(n, 3 + 3 * residues + 2 * step);
                let h1 = Expr::variable(n, 3 + 3 * residues + 2 * step + 1);
                equations.push(c[4].sub(&h1, p));
                equations.push(c[3].sub(&h0, p).add(&h1.mul(&e[0], p), p));
                equations.push(
                    c[2].add(&h0.mul(&e[0], p), p)
                        .sub(&h1.mul(&e[1], p), p)
                        .sub(&next[2], p),
                );
                equations.push(
                    c[1].sub(&h0.mul(&e[1], p), p)
                        .add(&h1.mul(&e[2], p), p)
                        .sub(&next[1], p),
                );
                equations.push(c[0].add(&h0.mul(&e[2], p), p).sub(&next[0], p));
            }
        }
        current = next;
    }
    equations.push(current[0].sub(&Expr::constant(n, 1, p), p));
    equations.push(current[1].clone());
    equations.push(current[2].clone());
    let membership_build_ops = build_ops();
    let polys: Vec<Poly> = equations
        .iter()
        .map(|f| f.to_poly(p))
        .filter(|f| !f.is_empty())
        .collect();
    let input_terms = polys.iter().map(Vec::len).sum();
    let input_degree = equations.iter().map(Expr::degree).max().unwrap_or(0);
    Ok(Built {
        exprs: equations,
        polys,
        n_vars: n,
        input_terms,
        input_degree,
        membership_build_ops,
        build_ms: start.elapsed().as_secs_f64() * 1000.0,
    })
}

fn elementary3(w: [u64; 3], p: u64) -> [u64; 3] {
    [
        addm(addm(w[0], w[1], p), w[2], p),
        addm(
            addm(mulm(w[0], w[1], p), mulm(w[0], w[2], p), p),
            mulm(w[1], w[2], p),
            p,
        ),
        mulm(mulm(w[0], w[1], p), w[2], p),
    ]
}

fn numeric_product(a: [u64; 3], b: [u64; 3], e: [u64; 3], p: u64) -> ([u64; 3], [u64; 2]) {
    let mut c = [0u64; 5];
    for i in 0..3 {
        for j in 0..3 {
            c[i + j] = addm(c[i + j], mulm(a[i], b[j], p), p);
        }
    }
    let h1 = c[4];
    let h0 = addm(c[3], mulm(h1, e[0], p), p);
    let r2 = subm(addm(c[2], mulm(h0, e[0], p), p), mulm(h1, e[1], p), p);
    let r1 = addm(subm(c[1], mulm(h0, e[1], p), p), mulm(h1, e[2], p), p);
    let r0 = addm(c[0], mulm(h0, e[2], p), p);
    ([r0, r1, r2], [h0, h1])
}

fn planted_values(arm: Arm, d: u64, w: [u64; 3], p: u64) -> Vec<u64> {
    let ops = schedule(d);
    let e = elementary3(w, p);
    let mut residues = Vec::with_capacity(ops.len());
    let mut quotients = Vec::with_capacity(ops.len());
    let x = [0, 1, 0];
    let mut current = x;
    for op in &ops {
        let other = if matches!(op, Op::Square) { current } else { x };
        let (next, h) = numeric_product(current, other, e, p);
        residues.push(next);
        quotients.push(h);
        current = next;
    }
    assert_eq!(current, [1, 0, 0], "roots should divide X^d - 1");
    let mut out = e.to_vec();
    for r in residues {
        out.extend_from_slice(&r);
    }
    if arm == Arm::Quotient {
        for h in quotients {
            out.extend_from_slice(&h);
        }
    }
    out
}

fn roots_of_unity(p: u64, d: u64) -> Result<Vec<u64>, String> {
    let z =
        primitive_root_of_unity(p, d).ok_or_else(|| format!("no primitive {d}-th root mod {p}"))?;
    let mut w = 1;
    let mut out = Vec::with_capacity(d as usize);
    for _ in 0..d {
        out.push(w);
        w = mulm(w, z, p);
    }
    Ok(out)
}

#[derive(Clone, Debug)]
struct Target {
    scalar: u64,
    point: Point,
    roots: Option<[u64; 3]>,
    root_indices: Option<[usize; 3]>,
    signs: Option<[bool; 3]>,
}

fn dlog_table(curve: Curve, generator: Point, r: u64) -> HashMap<Point, u64> {
    let mut table = HashMap::with_capacity(r as usize);
    let mut q = Point(None);
    for k in 0..r {
        table.insert(q, k);
        q = curve.add(q, generator);
    }
    table
}

fn planted_targets(cfg: Config, d: u64) -> Result<Vec<Target>, String> {
    let roots = roots_of_unity(cfg.curve.p, d)?;
    let generator = Point(Some((GX, GY)));
    let logs = dlog_table(cfg.curve, generator, R);
    let mut base = Vec::new();
    for (i, &w) in roots.iter().enumerate() {
        let x = addm(cfg.lo, mulm(cfg.g, w, cfg.curve.p), cfg.curve.p);
        if let Some(y) = sqrtm(cfg.curve.rhs(x), cfg.curve.p) {
            base.push((i, w, Point(Some((x, y)))));
        }
    }
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for i in 0..base.len() {
        for j in i + 1..base.len() {
            for k in j + 1..base.len() {
                for mask in 0..8u8 {
                    let chosen = [base[i], base[j], base[k]];
                    let mut sum = Point(None);
                    let mut signs = [false; 3];
                    for h in 0..3 {
                        signs[h] = mask >> h & 1 == 1;
                        let q = if signs[h] {
                            cfg.curve.neg(chosen[h].2)
                        } else {
                            chosen[h].2
                        };
                        sum = cfg.curve.add(sum, q);
                    }
                    let Some(&scalar) = logs.get(&sum) else {
                        continue;
                    };
                    if scalar == 0 || !seen.insert(scalar) {
                        continue;
                    }
                    out.push(Target {
                        scalar,
                        point: sum,
                        roots: Some([chosen[0].1, chosen[1].1, chosen[2].1]),
                        root_indices: Some([chosen[0].0, chosen[1].0, chosen[2].0]),
                        signs: Some(signs),
                    });
                }
            }
        }
    }
    Ok(out)
}

fn random_target(cfg: Config, d: u64, index: u64, seed: u64) -> Target {
    let mut rng = Rng(seed ^ d.rotate_left(17) ^ index.wrapping_mul(0xd1b5_4a32_d192_ed03));
    let scalar = 1 + rng.next() % (R - 1);
    let point = cfg.curve.mul(scalar, Point(Some((GX, GY))));
    Target {
        scalar,
        point,
        roots: None,
        root_indices: None,
        signs: None,
    }
}

fn target_for(cfg: Config, d: u64, kind: &str, index: usize, seed: u64) -> Result<Target, String> {
    match kind {
        "random" => Ok(random_target(cfg, d, index as u64, seed)),
        "planted" => planted_targets(cfg, d)?
            .into_iter()
            .nth(index)
            .ok_or_else(|| {
                format!(
                    "d={d} has fewer than {} distinct planted subgroup targets",
                    index + 1
                )
            }),
        _ => Err(format!(
            "unknown target kind {kind:?}; use random or planted"
        )),
    }
}

fn peak_rss_kib() -> Option<u64> {
    let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
    // SAFETY: successful getrusage initialises the complete output object.
    if unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) } != 0 {
        return None;
    }
    // SAFETY: checked successful initialisation above. Linux reports KiB.
    u64::try_from(unsafe { usage.assume_init() }.ru_maxrss).ok()
}

fn arg(args: &[String], name: &str, default: &str) -> String {
    args.iter()
        .position(|x| x == name)
        .map(|i| {
            args.get(i + 1)
                .unwrap_or_else(|| panic!("missing value after {name}"))
                .clone()
        })
        .unwrap_or_else(|| default.to_string())
}

fn run_cell(args: &[String]) -> Result<(), String> {
    if std::env::var("RAYON_NUM_THREADS").ok().as_deref() != Some("1") {
        return Err("cell requires RAYON_NUM_THREADS=1".into());
    }
    let arm = Arm::parse(&arg(args, "--arm", ""))?;
    let d: u64 = arg(args, "--d", "19").parse().map_err(|_| "bad --d")?;
    if ![7, 9, 19, 21, 27, 57].contains(&d) {
        return Err(format!("d={d} is not a frozen cell"));
    }
    let kind = arg(args, "--kind", "random");
    let index: usize = arg(args, "--index", "0")
        .parse()
        .map_err(|_| "bad --index")?;
    let cap: u64 = arg(args, "--cap", "300").parse().map_err(|_| "bad --cap")?;
    let dmax: u32 = arg(args, "--dmax", "24")
        .parse()
        .map_err(|_| "bad --dmax")?;
    let seed: u64 = arg(args, "--seed", &SEED.to_string())
        .parse()
        .map_err(|_| "bad --seed")?;
    let cfg = Config {
        curve: Curve { p: P, a: A, b: B },
        lo: LO,
        g: 1,
    };
    let target = target_for(cfg, d, &kind, index, seed)?;
    let (xt, yt) = target.point.0.ok_or("target is identity")?;
    let built = build_system(cfg, arm, d, xt, seed ^ d ^ index as u64)?;
    let (planted_equations_zero, planted_group_verified) = if let Some(w) = target.roots {
        let values = planted_values(arm, d, w, P);
        let equations_zero = built.exprs.iter().all(|f| f.eval(&values, P) == 0);
        let group_verified = cfg.curve.mul(target.scalar, Point(Some((GX, GY)))) == target.point;
        (Some(equations_zero), Some(group_verified))
    } else {
        (None, None)
    };
    if planted_equations_zero == Some(false) || planted_group_verified == Some(false) {
        return Err("planted correctness check failed before F4".into());
    }
    let opts = F4Options::new(Ordering::Grevlex, dmax).with_budget(Duration::from_secs(cap));
    let report = f4_fp::f4(&built.polys, built.n_vars, P, &opts);
    let censored = report.timed_out || report.pairs_above_bound != 0;
    println!(
        "{}",
        serde_json::json!({
            "schema": "curve25519.symmetric-remainder-row/v1",
            "study_id": "EXP-C25519-SYMREM-001",
            "revision": option_env!("GIT_COMMIT_HASH"),
            "arm": arm.name(), "d": d, "kind": kind, "index": index, "seed": seed,
            "p": P, "r": R, "m": 3, "target_scalar": target.scalar,
            "target_x": xt, "target_y": yt,
            "root_indices": target.root_indices, "roots": target.roots, "sign_negated": target.signs,
            "schedule_multiplications": schedule(d).len(),
            "n_vars": built.n_vars, "n_equations": built.polys.len(),
            "input_terms": built.input_terms, "input_degree": built.input_degree,
            "membership_build_coeff_ops": built.membership_build_ops,
            "build_ms": built.build_ms,
            "planted_equations_zero": planted_equations_zero,
            "planted_group_verified": planted_group_verified,
            "inconsistent": report.inconsistent,
            "field_ops": report.field_ops,
            "degree_reached": report.degree_reached,
            "solving_degree": report.solving_degree,
            "solving_degree_max": report.solving_degree_max,
            "max_rows": report.max_rows, "max_cols": report.max_cols,
            "max_cols_to_solution": report.max_cols_to_solution,
            "steps": report.steps, "steps_to_solution": report.steps_to_solution,
            "timed_out": report.timed_out, "pairs_above_bound": report.pairs_above_bound,
            "censored": censored, "f4_ms": report.ms, "peak_rss_kib": peak_rss_kib(),
            "label": "stage diagnostic; [pcc:unverified]"
        })
    );
    Ok(())
}

fn all_root_triples(roots: &[u64]) -> Vec<[u64; 3]> {
    let mut out = Vec::new();
    for i in 0..roots.len() {
        for j in i + 1..roots.len() {
            for k in j + 1..roots.len() {
                out.push([roots[i], roots[j], roots[k]]);
            }
        }
    }
    out
}

fn tiny_instance() -> Result<Config, String> {
    let p = 71;
    let d = 7;
    let roots = roots_of_unity(p, d)?;
    for a in 0..p {
        for b in 1..p {
            if addm(
                mulm(4, mulm(mulm(a, a, p), a, p), p),
                mulm(27, mulm(b, b, p), p),
                p,
            ) == 0
            {
                continue;
            }
            let curve = Curve { p, a, b };
            for lo in 0..p {
                let liftable = roots
                    .iter()
                    .filter(|&&w| sqrtm(curve.rhs(addm(lo, w, p)), p).is_some())
                    .count();
                if liftable >= 4 {
                    return Ok(Config { curve, lo, g: 1 });
                }
            }
        }
    }
    Err("no tiny exhaustive instance found".into())
}

fn tiny_truth(cfg: Config, roots: &[u64]) -> (BTreeSet<u64>, BTreeSet<u64>) {
    let mut target_x = BTreeSet::new();
    for x in 0..cfg.curve.p {
        if sqrtm(cfg.curve.rhs(x), cfg.curve.p).is_some() {
            target_x.insert(x);
        }
    }
    let mut truth = BTreeSet::new();
    for triple in all_root_triples(roots) {
        let mut lifts: [Option<Point>; 3] = [None, None, None];
        for i in 0..3 {
            let x = addm(cfg.lo, mulm(cfg.g, triple[i], cfg.curve.p), cfg.curve.p);
            lifts[i] = sqrtm(cfg.curve.rhs(x), cfg.curve.p).map(|y| Point(Some((x, y))));
        }
        if lifts.iter().any(Option::is_none) {
            continue;
        }
        for mask in 0..8u8 {
            let mut sum = Point(None);
            for i in 0..3 {
                let q = lifts[i].unwrap();
                sum = cfg.curve.add(
                    sum,
                    if mask >> i & 1 == 1 {
                        cfg.curve.neg(q)
                    } else {
                        q
                    },
                );
            }
            if let Some((x, _)) = sum.0 {
                truth.insert(x);
            }
        }
    }
    (target_x, truth)
}

fn run_selftest() -> Result<(), String> {
    let main = Config {
        curve: Curve { p: P, a: A, b: B },
        lo: LO,
        g: 1,
    };
    let generator = Point(Some((GX, GY)));
    if !main.curve.on_curve(generator) || main.curve.mul(R, generator) != Point(None) {
        return Err("committed k20 generator fixture failed".into());
    }
    let mut main_triples = 0usize;
    let mut planted_counts = BTreeMap::new();
    for d in [7, 9, 19] {
        let roots = roots_of_unity(P, d)?;
        let triples = all_root_triples(&roots);
        for w in triples {
            for arm in [Arm::Direct, Arm::Quotient] {
                let built = build_system(main, arm, d, GX, SEED ^ d)?;
                let values = planted_values(arm, d, w, P);
                if !built.exprs[1..].iter().all(|f| f.eval(&values, P) == 0) {
                    return Err(format!(
                        "membership equation failed for arm={} d={d} roots={w:?}",
                        arm.name()
                    ));
                }
            }
            main_triples += 1;
        }
        let planted = planted_targets(main, d)?;
        if planted.len() < 4 {
            return Err(format!(
                "d={d} has only {} distinct planted subgroup targets",
                planted.len()
            ));
        }
        for target in planted.iter().take(4) {
            let xt = target.point.0.unwrap().0;
            for arm in [Arm::Direct, Arm::Quotient] {
                let built = build_system(main, arm, d, xt, SEED ^ d)?;
                let values = planted_values(arm, d, target.roots.unwrap(), P);
                if !built.exprs.iter().all(|f| f.eval(&values, P) == 0) {
                    return Err(format!(
                        "full planted equation failed for arm={} d={d}",
                        arm.name()
                    ));
                }
            }
        }
        planted_counts.insert(d, planted.len());
    }

    let tiny = tiny_instance()?;
    let d = 7;
    let roots = roots_of_unity(tiny.curve.p, d)?;
    let triples = all_root_triples(&roots);
    let (targets, truth) = tiny_truth(tiny, &roots);
    let mut positives = 0usize;
    let mut negatives = 0usize;
    for xt in &targets {
        let mut verdicts = Vec::new();
        for arm in [Arm::Direct, Arm::Quotient] {
            let built = build_system(tiny, arm, d, *xt, SEED ^ *xt)?;
            let consistent = triples.iter().any(|&w| {
                let values = planted_values(arm, d, w, tiny.curve.p);
                built
                    .exprs
                    .iter()
                    .all(|f| f.eval(&values, tiny.curve.p) == 0)
            });
            verdicts.push(consistent);
        }
        let expected = truth.contains(xt);
        if verdicts != [expected, expected] {
            return Err(format!(
                "tiny exhaustive mismatch xt={xt}: expected={expected} arms={verdicts:?}"
            ));
        }
        if expected {
            positives += 1
        } else {
            negatives += 1
        }
    }
    if positives == 0 || negatives == 0 {
        return Err("tiny exhaustive panel lacks a positive or negative control".into());
    }
    println!(
        "{}",
        serde_json::json!({
            "schema": "curve25519.symmetric-remainder-selftest/v1",
            "main_curve": {"p": P, "r": R, "generator_ok": true},
            "main_distinct_root_triples_checked_per_arm": main_triples,
            "planted_subgroup_targets_available": planted_counts,
            "planted_checked_per_d_per_arm": 4,
            "tiny": {"p": tiny.curve.p, "a": tiny.curve.a, "b": tiny.curve.b,
                "lo": tiny.lo, "d": d, "target_abscissae": targets.len(),
                "positive": positives, "negative": negatives},
            "direct_quotient_truth_agreement": true,
            "status": "pass"
        })
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.get(1).map(String::as_str) {
        Some("selftest") => run_selftest(),
        Some("cell") => run_cell(&args[2..]),
        _ => Err("usage: curve25519_symmetric_remainder selftest | cell --arm direct|quotient --d N --kind random|planted --index I [--cap 300 --dmax 24 --seed 20261125]".into()),
    };
    if let Err(e) = result {
        eprintln!("curve25519_symmetric_remainder: {e}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotient_identity_and_direct_remainder_agree() {
        let p = 1013;
        let mut rng = Rng(17);
        for _ in 0..100 {
            let a = [rng.next() % p, rng.next() % p, rng.next() % p];
            let b = [rng.next() % p, rng.next() % p, rng.next() % p];
            let e = [rng.next() % p, rng.next() % p, rng.next() % p];
            let (r, h) = numeric_product(a, b, e, p);
            let mut c = [0u64; 5];
            for i in 0..3 {
                for j in 0..3 {
                    c[i + j] = addm(c[i + j], mulm(a[i], b[j], p), p);
                }
            }
            assert_eq!(c[4], h[1]);
            assert_eq!(c[3], subm(h[0], mulm(h[1], e[0], p), p));
            assert_eq!(
                r[2],
                subm(addm(c[2], mulm(h[0], e[0], p), p), mulm(h[1], e[1], p), p)
            );
            assert_eq!(
                r[1],
                addm(subm(c[1], mulm(h[0], e[1], p), p), mulm(h[1], e[2], p), p)
            );
            assert_eq!(r[0], addm(c[0], mulm(h[0], e[2], p), p));
        }
    }

    #[test]
    fn all_frozen_orders_end_at_one() {
        for d in [7, 9, 19, 21, 27, 57] {
            let roots = roots_of_unity(P, d).unwrap();
            let w = [roots[0], roots[1], roots[2]];
            for arm in [Arm::Direct, Arm::Quotient] {
                let values = planted_values(arm, d, w, P);
                assert_eq!(
                    values.len(),
                    3 + 3 * schedule(d).len()
                        + if arm == Arm::Quotient {
                            2 * schedule(d).len()
                        } else {
                            0
                        }
                );
            }
        }
    }

    #[test]
    fn committed_generator_has_order_r() {
        let curve = Curve { p: P, a: A, b: B };
        let g = Point(Some((GX, GY)));
        assert!(curve.on_curve(g));
        assert_eq!(curve.mul(R, g), Point(None));
    }
}
