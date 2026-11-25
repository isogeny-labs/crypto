#![recursion_limit = "256"]

//! Pair-trace quotient and quadratized 2+1 S3 split from
//! `research/curve25519_pair_trace_20261125/`.
//!
//! This is a stage diagnostic, not an ECDLP algorithm. D is the predecessor's
//! symmetric-S4/direct-remainder baseline. R uses three raw roots and two
//! quadratized S3 links. P quotients the swap of the first pair through its
//! canonical scale and ratio trace, while keeping every input equation
//! quadratic.
//!
//! ```text
//! cargo run --release --example curve25519_pair_trace -- selftest
//! RAYON_NUM_THREADS=1 cargo run --release --example \
//!   curve25519_pair_trace -- cell --arm P --d 7 \
//!   --kind random --index 0 --cap 300 --dmax 24
//! ```

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::time::{Duration, Instant};

use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Ordering, Poly};
use crypto_lib::cryptanalysis::ic_framework::subgroup_semaev::{
    primitive_root_of_unity, symmetrise,
};

const P: u64 = 1_048_573;
const A: u64 = 63_700;
const B: u64 = 872_826;
const R: u64 = 131_203;
const GX: u64 = 349_837;
const GY: u64 = 122;
const LO: u64 = 349_833;
const FACTOR_G: u64 = 3;
const SEED: u64 = 2_026_112_503;

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

#[derive(Clone, Debug, PartialEq, Eq)]
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
    Raw,
    Pair,
}

impl Arm {
    const ALL: [Self; 3] = [Self::Direct, Self::Raw, Self::Pair];

    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "direct" | "D" => Ok(Self::Direct),
            "raw" | "R" => Ok(Self::Raw),
            "pair" | "P" => Ok(Self::Pair),
            _ => Err(format!("unknown arm {s:?}; use D, R, or P")),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Direct => "D",
            Self::Raw => "R",
            Self::Pair => "P",
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

#[derive(Clone, Debug)]
struct ScalarLayout {
    root: usize,
    powers: BTreeMap<u64, usize>,
}

fn scalar_plan(d: u64) -> (Vec<(u64, u64, u64)>, (u64, u64)) {
    match d {
        7 => (vec![(2, 1, 1), (3, 2, 1), (6, 3, 3)], (6, 1)),
        9 => (vec![(2, 1, 1), (4, 2, 2), (8, 4, 4)], (8, 1)),
        19 => (
            vec![(2, 1, 1), (4, 2, 2), (8, 4, 4), (16, 8, 8), (18, 16, 2)],
            (18, 1),
        ),
        _ => panic!("no frozen scalar chain for d={d}"),
    }
}

fn alloc_scalar(next: &mut usize, d: u64) -> ScalarLayout {
    let root = *next;
    *next += 1;
    let mut powers = BTreeMap::new();
    for (out, _, _) in scalar_plan(d).0 {
        powers.insert(out, *next);
        *next += 1;
    }
    ScalarLayout { root, powers }
}

fn scalar_ref(layout: &ScalarLayout, exponent: u64, n: usize) -> Expr {
    if exponent == 1 {
        Expr::variable(n, layout.root)
    } else {
        Expr::variable(n, layout.powers[&exponent])
    }
}

fn scalar_equations(layout: &ScalarLayout, d: u64, n: usize, p: u64) -> Vec<Expr> {
    let (steps, final_pair) = scalar_plan(d);
    let mut equations = Vec::with_capacity(steps.len() + 1);
    for (out, left, right) in steps {
        equations.push(scalar_ref(layout, out, n).sub(
            &scalar_ref(layout, left, n).mul(&scalar_ref(layout, right, n), p),
            p,
        ));
    }
    equations.push(
        scalar_ref(layout, final_pair.0, n)
            .mul(&scalar_ref(layout, final_pair.1, n), p)
            .sub(&Expr::constant(n, 1, p), p),
    );
    equations
}

#[derive(Clone, Copy, Debug)]
enum TraceRef {
    One,
    C,
    Var(usize),
}

#[derive(Clone, Debug)]
enum TraceStep {
    Double {
        hi: TraceRef,
        lo: TraceRef,
        even: usize,
        odd: Option<usize>,
        cross: Option<usize>,
    },
    Add {
        even: TraceRef,
        odd: TraceRef,
        out: usize,
    },
}

#[derive(Clone, Debug)]
struct TraceLayout {
    c: usize,
    steps: Vec<TraceStep>,
    hi: TraceRef,
    lo: TraceRef,
}

fn alloc_trace(next: &mut usize, d: u64) -> TraceLayout {
    let c = *next;
    *next += 1;
    let n = (d - 1) / 2;
    let bits = format!("{n:b}");
    let mut k = 1u64;
    let mut hi = TraceRef::C;
    let mut lo = TraceRef::One;
    let mut steps = Vec::new();
    for bit in bits.bytes().skip(1) {
        let even = *next;
        *next += 1;
        let (odd, cross) = if k == 1 {
            (None, None)
        } else {
            let odd = *next;
            let cross = *next + 1;
            *next += 2;
            (Some(odd), Some(cross))
        };
        steps.push(TraceStep::Double {
            hi,
            lo,
            even,
            odd,
            cross,
        });
        let even_ref = TraceRef::Var(even);
        let odd_ref = odd.map_or(TraceRef::C, TraceRef::Var);
        k *= 2;
        if bit == b'1' {
            let out = *next;
            *next += 1;
            steps.push(TraceStep::Add {
                even: even_ref,
                odd: odd_ref,
                out,
            });
            hi = TraceRef::Var(out);
            lo = even_ref;
            k += 1;
        } else {
            hi = even_ref;
            lo = odd_ref;
        }
    }
    assert_eq!(k, n);
    TraceLayout { c, steps, hi, lo }
}

fn trace_ref(reference: TraceRef, layout: &TraceLayout, n: usize, p: u64) -> Expr {
    match reference {
        TraceRef::One => Expr::constant(n, 1, p),
        TraceRef::C => Expr::variable(n, layout.c),
        TraceRef::Var(index) => Expr::variable(n, index),
    }
}

fn trace_equations(layout: &TraceLayout, n: usize, p: u64) -> Vec<Expr> {
    let c = Expr::variable(n, layout.c);
    let mut equations = Vec::new();
    for step in &layout.steps {
        match *step {
            TraceStep::Double {
                hi,
                lo,
                even,
                odd,
                cross,
            } => {
                let hi = trace_ref(hi, layout, n, p);
                let lo = trace_ref(lo, layout, n, p);
                equations
                    .push(Expr::variable(n, even).sub(&hi.mul(&hi, p).sub(&lo.mul(&lo, p), p), p));
                if let (Some(odd), Some(cross)) = (odd, cross) {
                    let cross_expr = Expr::variable(n, cross);
                    equations.push(cross_expr.sub(&c.mul(&lo, p), p));
                    equations.push(
                        Expr::variable(n, odd)
                            .sub(&lo.mul(&hi.scale(2, p).sub(&cross_expr, p), p), p),
                    );
                }
            }
            TraceStep::Add { even, odd, out } => equations.push(
                Expr::variable(n, out).sub(
                    &c.mul(&trace_ref(even, layout, n, p), p)
                        .sub(&trace_ref(odd, layout, n, p), p),
                    p,
                ),
            ),
        }
    }
    equations.push(c.sub(&Expr::constant(n, 2, p), p).mul(
        &trace_ref(layout.hi, layout, n, p).add(&trace_ref(layout.lo, layout, n, p), p),
        p,
    ));
    equations
}

#[derive(Clone, Debug)]
struct RawLayout {
    w1: ScalarLayout,
    w2: ScalarLayout,
    pi: usize,
    delta: usize,
    beta: usize,
    u: usize,
    u2: usize,
    w3: ScalarLayout,
    pi2: usize,
    n: usize,
}

fn raw_layout(d: u64) -> RawLayout {
    let mut next = 0;
    let w1 = alloc_scalar(&mut next, d);
    let w2 = alloc_scalar(&mut next, d);
    let pi = next;
    let delta = next + 1;
    let beta = next + 2;
    let u = next + 3;
    let u2 = next + 4;
    next += 5;
    let w3 = alloc_scalar(&mut next, d);
    let pi2 = next;
    next += 1;
    RawLayout {
        w1,
        w2,
        pi,
        delta,
        beta,
        u,
        u2,
        w3,
        pi2,
        n: next,
    }
}

#[derive(Clone, Debug)]
struct PairLayout {
    z: ScalarLayout,
    trace: TraceLayout,
    a: usize,
    pi: usize,
    delta: usize,
    beta: usize,
    u: usize,
    u2: usize,
    w: ScalarLayout,
    pi2: usize,
    n: usize,
}

fn pair_layout(d: u64) -> PairLayout {
    let mut next = 0;
    let z = alloc_scalar(&mut next, d);
    let trace = alloc_trace(&mut next, d);
    let a = next;
    let pi = next + 1;
    let delta = next + 2;
    let beta = next + 3;
    let u = next + 4;
    let u2 = next + 5;
    next += 6;
    let w = alloc_scalar(&mut next, d);
    let pi2 = next;
    next += 1;
    PairLayout {
        z,
        trace,
        a,
        pi,
        delta,
        beta,
        u,
        u2,
        w,
        pi2,
        n: next,
    }
}

fn x_from_root(cfg: Config, root: &Expr, p: u64) -> Expr {
    Expr::constant(root.n, cfg.lo, p).add(&root.scale(cfg.g, p), p)
}

fn add_quadratized_links(
    equations: &mut Vec<Expr>,
    cfg: Config,
    sigma: Expr,
    pi_index: usize,
    delta_index: usize,
    beta_index: usize,
    u_index: usize,
    u2_index: usize,
    x3: Expr,
    pi2_index: usize,
    xt: u64,
) {
    let n = sigma.n;
    let p = cfg.curve.p;
    let pi = Expr::variable(n, pi_index);
    let delta = Expr::variable(n, delta_index);
    let beta = Expr::variable(n, beta_index);
    let u = Expr::variable(n, u_index);
    let u2 = Expr::variable(n, u2_index);
    equations.push(delta.sub(&sigma.mul(&sigma, p).sub(&pi.scale(4, p), p), p));
    equations.push(
        beta.sub(
            &sigma
                .mul(&pi.add(&Expr::constant(n, cfg.curve.a, p), p), p)
                .add(&Expr::constant(n, 2 * cfg.curve.b % p, p), p),
            p,
        ),
    );
    equations.push(u2.sub(&u.mul(&u, p), p));
    let gamma = pi
        .sub(&Expr::constant(n, cfg.curve.a, p), p)
        .pow(2, p)
        .sub(&sigma.scale(4 * cfg.curve.b % p, p), p);
    equations.push(
        delta
            .mul(&u2, p)
            .sub(&beta.mul(&u, p).scale(2, p), p)
            .add(&gamma, p),
    );

    let sigma2 = u.add(&x3, p);
    let pi2 = Expr::variable(n, pi2_index);
    equations.push(pi2.sub(&u.mul(&x3, p), p));
    let xt2 = mulm(xt, xt, p);
    let second = sigma2
        .mul(&sigma2, p)
        .sub(&pi2.scale(4, p), p)
        .scale(xt2, p)
        .sub(
            &sigma2
                .mul(&pi2.add(&Expr::constant(n, cfg.curve.a, p), p), p)
                .add(&Expr::constant(n, 2 * cfg.curve.b % p, p), p)
                .scale(2 * xt % p, p),
            p,
        )
        .add(
            &pi2.sub(&Expr::constant(n, cfg.curve.a, p), p)
                .pow(2, p)
                .sub(&sigma2.scale(4 * cfg.curve.b % p, p), p),
            p,
        );
    equations.push(second);
}

fn finish_built(start: Instant, equations: Vec<Expr>, n: usize, p: u64) -> Built {
    let membership_build_ops = build_ops();
    let polys: Vec<Poly> = equations
        .iter()
        .map(|f| f.to_poly(p))
        .filter(|f| !f.is_empty())
        .collect();
    let input_terms = polys.iter().map(Vec::len).sum();
    let input_degree = equations.iter().map(Expr::degree).max().unwrap_or(0);
    Built {
        exprs: equations,
        polys,
        n_vars: n,
        input_terms,
        input_degree,
        membership_build_ops,
        build_ms: start.elapsed().as_secs_f64() * 1000.0,
    }
}

fn build_direct(cfg: Config, d: u64, xt: u64, seed: u64) -> Result<Built, String> {
    let start = Instant::now();
    build_reset();
    let ops = schedule(d);
    let n = 3 + 3 * ops.len();
    let p = cfg.curve.p;
    let e: [Expr; 3] = std::array::from_fn(|i| Expr::variable(n, i));
    let x = [Expr::zero(n), Expr::constant(n, 1, p), Expr::zero(n)];
    let mut current = x.clone();
    let mut equations = vec![relation_expr(cfg, n, xt, seed)?];
    for (step, op) in ops.iter().enumerate() {
        let other = if matches!(op, Op::Square) {
            current.clone()
        } else {
            x.clone()
        };
        let next: [Expr; 3] = std::array::from_fn(|k| Expr::variable(n, 3 + 3 * step + k));
        let reduced = direct_remainder(&current, &other, &e, p);
        for k in 0..3 {
            equations.push(next[k].sub(&reduced[k], p));
        }
        current = next;
    }
    equations.push(current[0].sub(&Expr::constant(n, 1, p), p));
    equations.push(current[1].clone());
    equations.push(current[2].clone());
    Ok(finish_built(start, equations, n, p))
}

fn build_raw(cfg: Config, d: u64, xt: u64) -> Built {
    let start = Instant::now();
    build_reset();
    let layout = raw_layout(d);
    let n = layout.n;
    let p = cfg.curve.p;
    let mut equations = scalar_equations(&layout.w1, d, n, p);
    equations.extend(scalar_equations(&layout.w2, d, n, p));
    let x1 = x_from_root(cfg, &Expr::variable(n, layout.w1.root), p);
    let x2 = x_from_root(cfg, &Expr::variable(n, layout.w2.root), p);
    let sigma = x1.add(&x2, p);
    equations.push(Expr::variable(n, layout.pi).sub(&x1.mul(&x2, p), p));
    equations.extend(scalar_equations(&layout.w3, d, n, p));
    let x3 = x_from_root(cfg, &Expr::variable(n, layout.w3.root), p);
    add_quadratized_links(
        &mut equations,
        cfg,
        sigma,
        layout.pi,
        layout.delta,
        layout.beta,
        layout.u,
        layout.u2,
        x3,
        layout.pi2,
        xt,
    );
    finish_built(start, equations, n, p)
}

fn build_pair(cfg: Config, d: u64, xt: u64) -> Built {
    let start = Instant::now();
    build_reset();
    let layout = pair_layout(d);
    let n = layout.n;
    let p = cfg.curve.p;
    let z = Expr::variable(n, layout.z.root);
    let c = Expr::variable(n, layout.trace.c);
    let a = Expr::variable(n, layout.a);
    let mut equations = scalar_equations(&layout.z, d, n, p);
    equations.extend(trace_equations(&layout.trace, n, p));
    equations.push(a.sub(&z.mul(&c, p), p));
    let sigma = Expr::constant(n, 2 * cfg.lo % p, p).add(&a.scale(cfg.g, p), p);
    let pi_value = Expr::constant(n, mulm(cfg.lo, cfg.lo, p), p)
        .add(&a.scale(mulm(cfg.lo, cfg.g, p), p), p)
        .add(&z.mul(&z, p).scale(mulm(cfg.g, cfg.g, p), p), p);
    equations.push(Expr::variable(n, layout.pi).sub(&pi_value, p));
    equations.extend(scalar_equations(&layout.w, d, n, p));
    let x3 = x_from_root(cfg, &Expr::variable(n, layout.w.root), p);
    add_quadratized_links(
        &mut equations,
        cfg,
        sigma,
        layout.pi,
        layout.delta,
        layout.beta,
        layout.u,
        layout.u2,
        x3,
        layout.pi2,
        xt,
    );
    finish_built(start, equations, n, p)
}

fn build_system(cfg: Config, arm: Arm, d: u64, xt: u64, seed: u64) -> Result<Built, String> {
    match arm {
        Arm::Direct => build_direct(cfg, d, xt, seed),
        Arm::Raw => Ok(build_raw(cfg, d, xt)),
        Arm::Pair => Ok(build_pair(cfg, d, xt)),
    }
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

fn fill_scalar(values: &mut [u64], layout: &ScalarLayout, d: u64, root: u64, p: u64) {
    values[layout.root] = root;
    for (&exponent, &index) in &layout.powers {
        values[index] = powm(root, exponent, p);
    }
    assert_eq!(powm(root, d, p), 1);
}

fn trace_numeric(k: u64, c: u64, p: u64) -> u64 {
    match k {
        0 => 1,
        1 => c,
        _ => {
            let mut lo = 1;
            let mut hi = c;
            for _ in 2..=k {
                let next = subm(mulm(c, hi, p), lo, p);
                lo = hi;
                hi = next;
            }
            hi
        }
    }
}

fn trace_ref_value(reference: TraceRef, layout: &TraceLayout, values: &[u64]) -> u64 {
    match reference {
        TraceRef::One => 1,
        TraceRef::C => values[layout.c],
        TraceRef::Var(index) => values[index],
    }
}

fn fill_trace(values: &mut [u64], layout: &TraceLayout, d: u64, c: u64, p: u64) {
    values[layout.c] = c;
    for step in &layout.steps {
        match *step {
            TraceStep::Double {
                hi,
                lo,
                even,
                odd,
                cross,
            } => {
                let hi = trace_ref_value(hi, layout, values);
                let lo = trace_ref_value(lo, layout, values);
                values[even] = subm(mulm(hi, hi, p), mulm(lo, lo, p), p);
                if let (Some(odd), Some(cross)) = (odd, cross) {
                    values[cross] = mulm(c, lo, p);
                    values[odd] = mulm(lo, subm(2 * hi % p, values[cross], p), p);
                }
            }
            TraceStep::Add { even, odd, out } => {
                values[out] = subm(
                    mulm(c, trace_ref_value(even, layout, values), p),
                    trace_ref_value(odd, layout, values),
                    p,
                );
            }
        }
    }
    let n = (d - 1) / 2;
    assert_eq!(
        trace_ref_value(layout.hi, layout, values),
        trace_numeric(n, c, p)
    );
    assert_eq!(
        trace_ref_value(layout.lo, layout, values),
        trace_numeric(n - 1, c, p)
    );
}

fn signed_root_point(cfg: Config, w: u64, negated: bool) -> Result<Point, String> {
    let x = addm(cfg.lo, mulm(cfg.g, w, cfg.curve.p), cfg.curve.p);
    let y = sqrtm(cfg.curve.rhs(x), cfg.curve.p)
        .ok_or_else(|| format!("factor-base abscissa {x} does not lift"))?;
    let point = Point(Some((x, y)));
    Ok(if negated { cfg.curve.neg(point) } else { point })
}

fn pair_intermediate(cfg: Config, w: [u64; 3], signs: [bool; 3]) -> Result<u64, String> {
    cfg.curve
        .add(
            signed_root_point(cfg, w[0], signs[0])?,
            signed_root_point(cfg, w[1], signs[1])?,
        )
        .0
        .map(|(x, _)| x)
        .ok_or_else(|| "chosen first pair sums to infinity".to_string())
}

#[allow(clippy::too_many_arguments)]
fn fill_link_values(
    values: &mut [u64],
    cfg: Config,
    sigma: u64,
    pi: u64,
    pi_index: usize,
    delta_index: usize,
    beta_index: usize,
    u_index: usize,
    u2_index: usize,
    w3: u64,
    pi2_index: usize,
    u: u64,
) {
    let p = cfg.curve.p;
    values[pi_index] = pi;
    values[delta_index] = subm(mulm(sigma, sigma, p), 4 * pi % p, p);
    values[beta_index] = addm(
        mulm(sigma, addm(pi, cfg.curve.a, p), p),
        2 * cfg.curve.b % p,
        p,
    );
    values[u_index] = u;
    values[u2_index] = mulm(u, u, p);
    let x3 = addm(cfg.lo, mulm(cfg.g, w3, p), p);
    values[pi2_index] = mulm(u, x3, p);
}

fn planted_values(
    arm: Arm,
    d: u64,
    w: [u64; 3],
    signs: [bool; 3],
    cfg: Config,
) -> Result<Vec<u64>, String> {
    let p = cfg.curve.p;
    if arm == Arm::Direct {
        let ops = schedule(d);
        let e = elementary3(w, p);
        let x = [0, 1, 0];
        let mut current = x;
        let mut out = e.to_vec();
        for op in &ops {
            let other = if matches!(op, Op::Square) { current } else { x };
            let (next, _) = numeric_product(current, other, e, p);
            out.extend_from_slice(&next);
            current = next;
        }
        assert_eq!(current, [1, 0, 0], "roots should divide X^d - 1");
        return Ok(out);
    }

    let u = pair_intermediate(cfg, w, signs)?;
    match arm {
        Arm::Direct => unreachable!(),
        Arm::Raw => {
            let layout = raw_layout(d);
            let mut values = vec![0; layout.n];
            fill_scalar(&mut values, &layout.w1, d, w[0], p);
            fill_scalar(&mut values, &layout.w2, d, w[1], p);
            fill_scalar(&mut values, &layout.w3, d, w[2], p);
            let x1 = addm(cfg.lo, mulm(cfg.g, w[0], p), p);
            let x2 = addm(cfg.lo, mulm(cfg.g, w[1], p), p);
            fill_link_values(
                &mut values,
                cfg,
                addm(x1, x2, p),
                mulm(x1, x2, p),
                layout.pi,
                layout.delta,
                layout.beta,
                layout.u,
                layout.u2,
                w[2],
                layout.pi2,
                u,
            );
            Ok(values)
        }
        Arm::Pair => {
            let layout = pair_layout(d);
            let mut values = vec![0; layout.n];
            let product = mulm(w[0], w[1], p);
            let z = powm(product, d.div_ceil(2), p);
            let q = mulm(w[0], invm(z, p), p);
            let c = addm(q, invm(q, p), p);
            fill_scalar(&mut values, &layout.z, d, z, p);
            fill_trace(&mut values, &layout.trace, d, c, p);
            fill_scalar(&mut values, &layout.w, d, w[2], p);
            values[layout.a] = mulm(z, c, p);
            let sigma = addm(2 * cfg.lo % p, mulm(cfg.g, values[layout.a], p), p);
            let pi = addm(
                addm(
                    mulm(cfg.lo, cfg.lo, p),
                    mulm(mulm(cfg.lo, cfg.g, p), values[layout.a], p),
                    p,
                ),
                mulm(mulm(cfg.g, cfg.g, p), mulm(z, z, p), p),
                p,
            );
            fill_link_values(
                &mut values,
                cfg,
                sigma,
                pi,
                layout.pi,
                layout.delta,
                layout.beta,
                layout.u,
                layout.u2,
                w[2],
                layout.pi2,
                u,
            );
            Ok(values)
        }
    }
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

fn planted_targets_with_logs(
    cfg: Config,
    d: u64,
    logs: &HashMap<Point, u64>,
) -> Result<Vec<Target>, String> {
    let roots = roots_of_unity(cfg.curve.p, d)?;
    let mut base = Vec::new();
    for (i, &w) in roots.iter().enumerate() {
        let x = addm(cfg.lo, mulm(cfg.g, w, cfg.curve.p), cfg.curve.p);
        if let Some(y) = sqrtm(cfg.curve.rhs(x), cfg.curve.p) {
            base.push((i, w, Point(Some((x, y)))));
        }
    }
    let mut seen_x = HashSet::new();
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
                    let target_x = sum.0.unwrap().0;
                    if scalar == 0 || !seen_x.insert(target_x) {
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

fn planted_targets(cfg: Config, d: u64) -> Result<Vec<Target>, String> {
    let logs = dlog_table(cfg.curve, Point(Some((GX, GY))), R);
    planted_targets_with_logs(cfg, d, &logs)
}

fn run_search_g() -> Result<(), String> {
    let curve = Curve { p: P, a: A, b: B };
    let logs = dlog_table(curve, Point(Some((GX, GY))), R);
    for g in 1..P {
        let cfg = Config { curve, lo: LO, g };
        let d7 = planted_targets_with_logs(cfg, 7, &logs)?;
        if d7.len() < 4 {
            continue;
        }
        let d9 = planted_targets_with_logs(cfg, 9, &logs)?;
        if d9.len() < 4 {
            continue;
        }
        let d19 = planted_targets_with_logs(cfg, 19, &logs)?;
        if d19.len() < 4 {
            continue;
        }
        println!(
            "{}",
            serde_json::json!({
                "schema": "curve25519.quotient-layout-g-search/v1",
                "selection": "first positive integer g with at least four distinct planted subgroup target abscissae at every qualification order",
                "g": g,
                "planted_distinct_target_x": {"7": d7.len(), "9": d9.len(), "19": d19.len()}
            })
        );
        return Ok(());
    }
    Err("no g satisfies the planted-target preflight".into())
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
        g: FACTOR_G,
    };
    let target = target_for(cfg, d, &kind, index, seed)?;
    let (xt, yt) = target.point.0.ok_or("target is identity")?;
    let built = build_system(cfg, arm, d, xt, seed ^ d ^ index as u64)?;
    let (planted_equations_zero, planted_group_verified) = if let Some(w) = target.roots {
        let values = planted_values(arm, d, w, target.signs.unwrap(), cfg)?;
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
            "schema": "curve25519.pair-trace-row/v1",
            "study_id": "EXP-C25519-PAIRTRACE-001",
            "revision": option_env!("GIT_COMMIT_HASH"),
            "arm": arm.name(), "d": d, "kind": kind, "index": index, "seed": seed,
            "p": P, "r": R, "m": 3, "target_scalar": target.scalar,
            "target_x": xt, "target_y": yt,
            "root_indices": target.root_indices, "roots": target.roots, "sign_negated": target.signs,
            "direct_schedule_multiplications": if arm == Arm::Direct { Some(schedule(d).len()) } else { None },
            "all_input_equations_quadratic": if arm == Arm::Direct { None } else { Some(built.input_degree <= 2) },
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
        g: FACTOR_G,
    };
    let generator = Point(Some((GX, GY)));
    if !main.curve.on_curve(generator) || main.curve.mul(R, generator) != Point(None) {
        return Err("committed k20 generator fixture failed".into());
    }
    let mut planted_counts = BTreeMap::new();
    let mut pair_counts = BTreeMap::new();
    let mut trace_counts = BTreeMap::new();
    for d in [7, 9, 19] {
        let roots = roots_of_unity(P, d)?;
        let mut pairs = HashSet::new();
        for i in 0..roots.len() {
            for j in i..roots.len() {
                let product = mulm(roots[i], roots[j], P);
                let z = powm(product, d.div_ceil(2), P);
                let q = mulm(roots[i], invm(z, P), P);
                pairs.insert((z, addm(q, invm(q, P), P)));
            }
        }
        let expected_pairs = d as usize * (d as usize + 1) / 2;
        if pairs.len() != expected_pairs {
            return Err(format!(
                "pair-trace map d={d}: got {} images, expected {expected_pairs}",
                pairs.len()
            ));
        }
        pair_counts.insert(d, pairs.len());

        let expected_traces: BTreeSet<u64> =
            roots.iter().map(|&q| addm(q, invm(q, P), P)).collect();
        let n = (d - 1) / 2;
        let observed_traces: BTreeSet<u64> = (0..P)
            .filter(|&c| {
                mulm(
                    subm(c, 2, P),
                    addm(trace_numeric(n, c, P), trace_numeric(n - 1, c, P), P),
                    P,
                ) == 0
            })
            .collect();
        if observed_traces != expected_traces {
            return Err(format!("trace polynomial root set mismatch at d={d}"));
        }
        let layout = pair_layout(d);
        for &c in &expected_traces {
            let mut values = vec![0; layout.n];
            fill_trace(&mut values, &layout.trace, d, c, P);
            if !trace_equations(&layout.trace, layout.n, P)
                .iter()
                .all(|f| f.eval(&values, P) == 0)
            {
                return Err(format!("trace circuit assignment failed at d={d}, c={c}"));
            }
        }
        trace_counts.insert(d, expected_traces.len());

        let planted = planted_targets(main, d)?;
        if planted.len() < 4 {
            return Err(format!(
                "d={d} has only {} distinct planted subgroup targets",
                planted.len()
            ));
        }
        for target in planted.iter().take(4) {
            let xt = target.point.0.unwrap().0;
            for arm in Arm::ALL {
                let built = build_system(main, arm, d, xt, SEED ^ d)?;
                let values =
                    planted_values(arm, d, target.roots.unwrap(), target.signs.unwrap(), main)?;
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

    let roots7 = roots_of_unity(P, 7)?;
    let mut main_d7_signed_relations = 0usize;
    for w in all_root_triples(&roots7) {
        for mask in 0..8u8 {
            let signs = std::array::from_fn(|i| mask >> i & 1 == 1);
            let points: Result<Vec<Point>, String> = (0..3)
                .map(|i| signed_root_point(main, w[i], signs[i]))
                .collect();
            let Ok(points) = points else { continue };
            let sum = points
                .into_iter()
                .fold(Point(None), |acc, point| main.curve.add(acc, point));
            let Some((xt, _)) = sum.0 else { continue };
            for arm in Arm::ALL {
                let built = build_system(main, arm, 7, xt, SEED ^ xt)?;
                let values = planted_values(arm, 7, w, signs, main)?;
                if !built.exprs.iter().all(|f| f.eval(&values, P) == 0) {
                    return Err(format!(
                        "main d=7 signed relation failed arm={} roots={w:?} mask={mask}",
                        arm.name()
                    ));
                }
            }
            main_d7_signed_relations += 1;
        }
    }
    if main_d7_signed_relations == 0 {
        return Err("main d=7 panel has no signed relation".into());
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
        for arm in Arm::ALL {
            let built = build_system(tiny, arm, d, *xt, SEED ^ *xt)?;
            let consistent = triples.iter().any(|&w| {
                (0..8u8).any(|mask| {
                    let signs = std::array::from_fn(|i| mask >> i & 1 == 1);
                    let Ok(values) = planted_values(arm, d, w, signs, tiny) else {
                        return false;
                    };
                    built
                        .exprs
                        .iter()
                        .all(|f| f.eval(&values, tiny.curve.p) == 0)
                })
            });
            verdicts.push(consistent);
        }
        let expected = truth.contains(xt);
        if verdicts != vec![expected; Arm::ALL.len()] {
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
            "schema": "curve25519.pair-trace-selftest/v1",
            "main_curve": {"p": P, "r": R, "generator_ok": true},
            "main_d7_signed_relations_checked_per_arm": main_d7_signed_relations,
            "unordered_pair_trace_images": pair_counts,
            "trace_polynomial_roots": trace_counts,
            "planted_subgroup_targets_available": planted_counts,
            "planted_checked_per_d_per_arm": 4,
            "all_split_input_equations_quadratic": true,
            "tiny": {"p": tiny.curve.p, "a": tiny.curve.a, "b": tiny.curve.b,
                "lo": tiny.lo, "d": d, "target_abscissae": targets.len(),
                "positive": positives, "negative": negatives},
            "all_arm_truth_agreement": true,
            "status": "pass"
        })
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.get(1).map(String::as_str) {
        Some("selftest") => run_selftest(),
        Some("search-g") => run_search_g(),
        Some("cell") => run_cell(&args[2..]),
        _ => Err("usage: curve25519_pair_trace selftest | search-g | cell --arm D|R|P --d 7|9|19 --kind random|planted --index I [--cap 300 --dmax 24 --seed 2026112503]".into()),
    };
    if let Err(e) = result {
        eprintln!("curve25519_pair_trace: {e}");
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
    fn frozen_system_shapes_are_exact() {
        let cfg = Config {
            curve: Curve { p: P, a: A, b: B },
            lo: LO,
            g: FACTOR_G,
        };
        for (d, raw_n, pair_n) in [(7, 18, 18), (9, 18, 20), (19, 24, 28)] {
            let raw = build_system(cfg, Arm::Raw, d, GX, SEED).unwrap();
            let pair = build_system(cfg, Arm::Pair, d, GX, SEED).unwrap();
            assert_eq!(raw.n_vars, raw_n);
            assert_eq!(pair.n_vars, pair_n);
            assert_eq!(raw.exprs.len(), raw_n + 1);
            assert_eq!(pair.exprs.len(), pair_n + 1);
            assert!(raw.input_degree <= 2);
            assert!(pair.input_degree <= 2);
        }
        let direct = build_system(cfg, Arm::Direct, 19, GX, SEED).unwrap();
        assert_eq!(
            (
                direct.n_vars,
                direct.exprs.len(),
                direct.input_terms,
                direct.input_degree
            ),
            (21, 22, 104, 4)
        );
    }

    #[test]
    fn pair_trace_is_a_bijection_on_unordered_pairs() {
        for d in [7, 9, 19] {
            let roots = roots_of_unity(P, d).unwrap();
            let mut images = HashSet::new();
            for i in 0..roots.len() {
                for j in i..roots.len() {
                    let z = powm(mulm(roots[i], roots[j], P), d.div_ceil(2), P);
                    let q = mulm(roots[i], invm(z, P), P);
                    images.insert((z, addm(q, invm(q, P), P)));
                }
            }
            assert_eq!(images.len(), d as usize * (d as usize + 1) / 2);
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
