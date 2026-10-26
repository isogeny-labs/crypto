//! Twist obstruction for summation-polynomial relations, measured on toy
//! binary fields. Protocol: `../PROTOCOL.md`.
//!
//! Model: q = 2^n with n odd prime, F_{q^2} = F_q[s]/(s^2 + s + 1),
//! E_a : y^2 + xy = x^3 + a x^2 + b with a, b in F_q. Because n is odd,
//! E_{a+1} is the quadratic twist of E_a and phi(x, y) = (x, y + s x) is an
//! x-preserving isomorphism E_{a+1} -> E_a over F_{q^2}. The q-power
//! Frobenius sigma acts as s -> s + 1.
//!
//! Families (all exact, deterministic, seeded):
//!   census : every P in E(F_q) plus every Q in phi(E'(F_q)); is P + Q
//!            fixed by sigma? Prediction: iff Q is 2-torsion.
//!   decomp : every factor-base decomposition R = Q_1 + ... + Q_m with
//!            x(Q_i) in V = span(1, t, ..., t^{l-1}), found by point
//!            arithmetic, classified by the sigma-parity of each Q_i.
//!            Prediction: the off-parity sub-sum is O or T = (0, sqrt b),
//!            the off-parity solutions are exactly the predicted set, and
//!            the x-solution set is identical on E_a and on its twist.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::io::Write as _;
use std::time::Instant;

// ---------------------------------------------------------------- F_q ----

struct Fq {
    n: u32,
    q1: usize,
    exp: Vec<u32>,
    log: Vec<u32>,
    tr_mask: u32,
    half_trace: Vec<u32>, // H(e_i), i < n
    sqrt_basis: Vec<u32>, // sqrt(e_i), i < n
}

fn clmul_reduce(a: u32, b: u32, n: u32, poly: u32) -> u32 {
    let mut r: u64 = 0;
    let (a, mut b, mut i) = (a as u64, b, 0);
    while b != 0 {
        if b & 1 == 1 {
            r ^= a << i;
        }
        b >>= 1;
        i += 1;
    }
    for k in (n..2 * n).rev() {
        if (r >> k) & 1 == 1 {
            r ^= (poly as u64) << (k - n);
        }
    }
    r as u32
}

impl Fq {
    fn new(n: u32, poly: u32) -> Fq {
        assert!(n % 2 == 1 && (poly >> n) == 1);
        // n is prime here, so poly is irreducible iff x^(2^n) = x mod poly
        // and poly has no root in F_2.
        assert!(poly & 1 == 1 && (poly.count_ones() % 2 == 1), "root in F_2");
        let mut z = 2u32;
        for _ in 0..n {
            z = clmul_reduce(z, z, n, poly);
        }
        assert_eq!(z, 2, "x^(2^n) != x: poly not irreducible");
        let q1 = (1usize << n) - 1;
        let mut exp = vec![0u32; 2 * q1];
        let mut log = vec![0u32; q1 + 1];
        let mut g = 2u32;
        'search: loop {
            let mut e = 1u32;
            for i in 0..q1 {
                if i > 0 && e == 1 {
                    g += 1;
                    continue 'search;
                }
                exp[i] = e;
                log[e as usize] = i as u32;
                e = clmul_reduce(e, g, n, poly);
            }
            assert_eq!(e, 1);
            break;
        }
        for i in 0..q1 {
            exp[q1 + i] = exp[i];
        }
        let mut f = Fq {
            n,
            q1,
            exp,
            log,
            tr_mask: 0,
            half_trace: vec![],
            sqrt_basis: vec![],
        };
        for i in 0..n {
            let e = 1u32 << i;
            let (mut t, mut z) = (0u32, e);
            for _ in 0..n {
                t ^= z;
                z = f.sq(z);
            }
            assert!(t <= 1);
            f.tr_mask |= t << i;
            let (mut h, mut z) = (0u32, e);
            for _ in 0..=(n - 1) / 2 {
                h ^= z;
                z = f.sq(f.sq(z));
            }
            f.half_trace.push(h);
            let mut r = e;
            for _ in 0..n - 1 {
                r = f.sq(r);
            }
            f.sqrt_basis.push(r);
        }
        f
    }
    #[inline]
    fn mul(&self, a: u32, b: u32) -> u32 {
        if a == 0 || b == 0 {
            return 0;
        }
        self.exp[(self.log[a as usize] + self.log[b as usize]) as usize]
    }
    #[inline]
    fn sq(&self, a: u32) -> u32 {
        self.mul(a, a)
    }
    #[inline]
    fn inv(&self, a: u32) -> u32 {
        assert!(a != 0);
        self.exp[(self.q1 - self.log[a as usize] as usize) % self.q1]
    }
    #[inline]
    fn tr(&self, a: u32) -> u32 {
        (a & self.tr_mask).count_ones() & 1
    }
    fn linear(&self, table: &[u32], a: u32) -> u32 {
        let mut r = 0;
        for i in 0..self.n {
            if (a >> i) & 1 == 1 {
                r ^= table[i as usize];
            }
        }
        r
    }
    fn ht(&self, a: u32) -> u32 {
        self.linear(&self.half_trace, a)
    }
    fn sqrt(&self, a: u32) -> u32 {
        self.linear(&self.sqrt_basis, a)
    }
}

// ------------------------------------------------------------ F_{q^2} ----

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct F2 {
    u: u32,
    v: u32,
} // u + v s

const ZERO: F2 = F2 { u: 0, v: 0 };

fn f2(u: u32) -> F2 {
    F2 { u, v: 0 }
}
fn add2(a: F2, b: F2) -> F2 {
    F2 {
        u: a.u ^ b.u,
        v: a.v ^ b.v,
    }
}
fn mul2(f: &Fq, a: F2, b: F2) -> F2 {
    let uu = f.mul(a.u, b.u);
    let vv = f.mul(a.v, b.v);
    let m = f.mul(a.u ^ a.v, b.u ^ b.v);
    F2 {
        u: uu ^ vv,
        v: m ^ uu,
    }
}
fn inv2(f: &Fq, a: F2) -> F2 {
    let nrm = f.sq(a.u) ^ f.mul(a.u, a.v) ^ f.sq(a.v);
    let ni = f.inv(nrm);
    F2 {
        u: f.mul(a.u ^ a.v, ni),
        v: f.mul(a.v, ni),
    }
}
fn sigma2(a: F2) -> F2 {
    F2 {
        u: a.u ^ a.v,
        v: a.v,
    }
}

// -------------------------------------------------------------- curve ----

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pt {
    x: F2,
    y: F2,
    inf: bool,
}
const O: Pt = Pt {
    x: ZERO,
    y: ZERO,
    inf: true,
};

struct Curve {
    a: u32,
    b: u32,
}

impl Curve {
    fn on_curve(&self, f: &Fq, p: &Pt) -> bool {
        if p.inf {
            return true;
        }
        let lhs = add2(mul2(f, p.y, p.y), mul2(f, p.x, p.y));
        let x2 = mul2(f, p.x, p.x);
        let rhs = add2(add2(mul2(f, x2, p.x), mul2(f, f2(self.a), x2)), f2(self.b));
        lhs == rhs
    }
    fn neg(&self, p: &Pt) -> Pt {
        if p.inf {
            return *p;
        }
        Pt {
            x: p.x,
            y: add2(p.y, p.x),
            inf: false,
        }
    }
    fn add(&self, f: &Fq, p: &Pt, q: &Pt) -> Pt {
        if p.inf {
            return *q;
        }
        if q.inf {
            return *p;
        }
        if p.x == q.x {
            if p.y == add2(q.y, q.x) {
                return O;
            }
            // doubling, x != 0
            let lam = add2(p.x, mul2(f, p.y, inv2(f, p.x)));
            let x3 = add2(add2(mul2(f, lam, lam), lam), f2(self.a));
            let y3 = add2(mul2(f, p.x, p.x), mul2(f, add2(lam, f2(1)), x3));
            return Pt {
                x: x3,
                y: y3,
                inf: false,
            };
        }
        let lam = mul2(f, add2(p.y, q.y), inv2(f, add2(p.x, q.x)));
        let x3 = add2(
            add2(add2(add2(mul2(f, lam, lam), lam), p.x), q.x),
            f2(self.a),
        );
        let y3 = add2(add2(mul2(f, lam, add2(p.x, x3)), x3), p.y);
        Pt {
            x: x3,
            y: y3,
            inf: false,
        }
    }
    fn sub(&self, f: &Fq, p: &Pt, q: &Pt) -> Pt {
        self.add(f, p, &self.neg(q))
    }
    fn sigma(&self, p: &Pt) -> Pt {
        if p.inf {
            return *p;
        }
        Pt {
            x: sigma2(p.x),
            y: sigma2(p.y),
            inf: false,
        }
    }
    fn two_torsion(&self, f: &Fq) -> Pt {
        Pt {
            x: ZERO,
            y: f2(f.sqrt(self.b)),
            inf: false,
        }
    }
    /// Trace type of x != 0 on this curve: 0 if x lifts to E(F_q), 1 if it
    /// lifts to the sigma = -1 part (the embedded twist).
    fn trace_type(&self, f: &Fq, x: u32) -> u32 {
        let w = x ^ self.a ^ f.mul(self.b, f.inv(f.sq(x)));
        f.tr(w)
    }
    /// One lift of x in F_q to E(F_{q^2}).
    fn lift(&self, f: &Fq, x: u32) -> Pt {
        if x == 0 {
            return self.two_torsion(f);
        }
        let w = x ^ self.a ^ f.mul(self.b, f.inv(f.sq(x)));
        let t = f.tr(w);
        let h = f.ht(w);
        let p = Pt {
            x: f2(x),
            y: F2 {
                u: f.mul(x, h),
                v: f.mul(x, t),
            },
            inf: false,
        };
        debug_assert!(self.on_curve(f, &p));
        p
    }
    /// +1: sigma P = P, -1: sigma P = -P, 0: both (2-torsion or O).
    fn parity(&self, p: &Pt) -> i32 {
        let s = self.sigma(p);
        let plus = s == *p;
        let minus = s == self.neg(p);
        match (plus, minus) {
            (true, true) => 0,
            (true, false) => 1,
            (false, true) => -1,
            _ => panic!("point with x in F_q has no sigma parity"),
        }
    }
}

// ---------------------------------------------------------------- rng ----

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, m: u64) -> u64 {
        self.next() % m
    }
}

// ------------------------------------------------------------- census ----

fn census(f: &Fq, c: &Curve) -> String {
    let t0 = Instant::now();
    let q = 1u64 << f.n;
    let mut plus = vec![O, c.two_torsion(f)];
    let mut minus = vec![O, c.two_torsion(f)];
    for x in 1..q as u32 {
        let p = c.lift(f, x);
        assert!(c.on_curve(f, &p));
        let pr = c.parity(&p);
        let tt = c.trace_type(f, x);
        assert_eq!(pr, if tt == 0 { 1 } else { -1 }, "trace/sigma mismatch");
        if pr == 1 {
            plus.push(p);
            plus.push(c.neg(&p));
        } else {
            minus.push(p);
            minus.push(c.neg(&p));
        }
    }
    // Independent count of #E'(F_q) on the twist model over F_q, and phi.
    let tw = Curve { a: c.a ^ 1, b: c.b };
    let mut n_twist = 2u64; // O and (0, sqrt b)
    let mut phi_ok = 0u64;
    for x in 1..q as u32 {
        let w = x ^ tw.a ^ f.mul(tw.b, f.inv(f.sq(x)));
        if f.tr(w) == 0 {
            n_twist += 2;
            let yp = f.mul(x, f.ht(w));
            for y in [yp, yp ^ x] {
                // phi(x, y) = (x, y + s x)
                let img = Pt {
                    x: f2(x),
                    y: F2 { u: y, v: x },
                    inf: false,
                };
                if c.on_curve(f, &img) && c.parity(&img) == -1 {
                    phi_ok += 1;
                }
            }
        }
    }
    let t2 = c.two_torsion(f);
    let mut cnt = [[0u64; 2]; 2]; // [q_is_2torsion][sum_is_rational]
    for p in &plus {
        for qq in &minus {
            let s = c.add(f, p, qq);
            let rational = c.sigma(&s) == s;
            let tor = qq.inf || *qq == t2;
            cnt[tor as usize][rational as usize] += 1;
        }
    }
    format!(
        "{{\"family\":\"census\",\"n\":{},\"a\":{},\"b\":{},\"E_Fq\":{},\"sigma_minus\":{},\"twist_count_Fq\":{},\"phi_image_ok\":{},\"pairs\":{},\"nontor_rational\":{},\"nontor_irrational\":{},\"tor_rational\":{},\"tor_irrational\":{},\"seconds\":{:.3}}}",
        f.n, c.a, c.b, plus.len(), minus.len(), n_twist, phi_ok,
        (plus.len() * minus.len()), cnt[0][1], cnt[0][0], cnt[1][1], cnt[1][0],
        t0.elapsed().as_secs_f64()
    )
}

// ------------------------------------------------------------- decomp ----

type Key = Vec<u32>;

struct Ctx<'a> {
    f: &'a Fq,
    c: &'a Curve,
    l: u32,
    lifts: Vec<Pt>, // lift of every x in V
}

impl<'a> Ctx<'a> {
    fn new(f: &'a Fq, c: &'a Curve, l: u32) -> Ctx<'a> {
        let lifts = (0..1u32 << l).map(|x| c.lift(f, x)).collect();
        Ctx { f, c, l, lifts }
    }
    fn in_v(&self, p: &Pt) -> Option<u32> {
        if p.inf || p.x.v != 0 || p.x.u >> self.l != 0 {
            None
        } else {
            Some(p.x.u)
        }
    }
    fn signs(&self, x: u32) -> Vec<Pt> {
        let p = self.lifts[x as usize];
        if x == 0 {
            vec![p]
        } else {
            vec![p, self.c.neg(&p)]
        }
    }
    /// All x-tuples (sorted) in V^m with a signed lift summing to r, each
    /// with one witness (signed points, in key order).
    fn decompose(&self, r: &Pt, m: usize) -> BTreeMap<Key, Vec<Pt>> {
        let (f, c) = (self.f, self.c);
        let big = 1u32 << self.l;
        let mut out: BTreeMap<Key, Vec<Pt>> = BTreeMap::new();
        match m {
            2 => {
                for x1 in 0..big {
                    for q1 in self.signs(x1) {
                        let q2 = c.sub(f, r, &q1);
                        if let Some(x2) = self.in_v(&q2) {
                            if x2 >= x1 {
                                out.entry(vec![x1, x2]).or_insert(vec![q1, q2]);
                            }
                        }
                    }
                }
            }
            3 => {
                for x1 in 0..big {
                    let s1 = self.signs(x1);
                    for x2 in x1..big {
                        for q1 in &s1 {
                            for q2 in self.signs(x2) {
                                let s = c.add(f, q1, &q2);
                                let q3 = c.sub(f, r, &s);
                                if let Some(x3) = self.in_v(&q3) {
                                    if x3 >= x2 {
                                        out.entry(vec![x1, x2, x3]).or_insert(vec![*q1, q2, q3]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
        out
    }
    /// The off-parity solutions the theorem predicts (m = 3), built from the
    /// target alone: (x_R, x, x) when x_R in V, and (x(R+T), x, x(P_x+T))
    /// when x(R+T) in V; x ranges over off-parity x in V.
    fn predicted_off(&self, r: &Pt, tau: i32, m: usize) -> BTreeSet<Key> {
        let mut pred = BTreeSet::new();
        if m != 3 {
            return pred;
        }
        let (f, c) = (self.f, self.c);
        let t2 = c.two_torsion(f);
        let off: Vec<u32> = (1..1u32 << self.l)
            .filter(|&x| c.parity(&self.lifts[x as usize]) == -tau)
            .collect();
        if let Some(xr) = self.in_v(r) {
            for &x in &off {
                let mut k = vec![xr, x, x];
                k.sort();
                pred.insert(k);
            }
        }
        if let Some(xt) = self.in_v(&c.add(f, r, &t2)) {
            for &x in &off {
                if let Some(y) = self.in_v(&c.add(f, &self.lifts[x as usize], &t2)) {
                    let mut k = vec![xt, x, y];
                    k.sort();
                    pred.insert(k);
                }
            }
        }
        pred
    }
    fn s3(&self, x1: u32, x2: u32, x3: u32) -> u32 {
        let f = self.f;
        let (s1, s2, s3) = (f.sq(x1), f.sq(x2), f.sq(x3));
        f.mul(s1, s2) ^ f.mul(s1, s3) ^ f.mul(s2, s3) ^ f.mul(f.mul(x1, x2), x3) ^ self.c.b
    }
    /// S_4 = Res_X(S_3(x1, x2, X), S_3(x3, x4, X)) for two quadratics in X.
    fn s4(&self, x1: u32, x2: u32, x3: u32, x4: u32) -> u32 {
        let f = self.f;
        let quad = |u: u32, v: u32| {
            (
                f.sq(u) ^ f.sq(v),
                f.mul(u, v),
                f.mul(f.sq(u), f.sq(v)) ^ self.c.b,
            )
        };
        let (a2, a1, a0) = quad(x1, x2);
        let (b2, b1, b0) = quad(x3, x4);
        f.sq(f.mul(a2, b0) ^ f.mul(a0, b2))
            ^ f.mul(f.mul(a2, b1) ^ f.mul(a1, b2), f.mul(a1, b0) ^ f.mul(a0, b1))
    }
    fn poly_roots(&self, xr: u32, m: usize) -> BTreeSet<Key> {
        let big = 1u32 << self.l;
        let mut out = BTreeSet::new();
        for x1 in 0..big {
            for x2 in x1..big {
                if m == 2 {
                    if self.s3(x1, x2, xr) == 0 {
                        out.insert(vec![x1, x2]);
                    }
                } else {
                    for x3 in x2..big {
                        if self.s4(x1, x2, x3, xr) == 0 {
                            out.insert(vec![x1, x2, x3]);
                        }
                    }
                }
            }
        }
        out
    }
}

#[derive(Default)]
struct Tally {
    solutions: u64,
    pure: u64,
    pure_with_2tor: u64,
    off_o: u64,
    off_t: u64,
    violation: u64,
    witness_fail: u64,
    obs_off: u64,
    pred_off: u64,
    obs_minus_pred: u64,
    pred_minus_obs: u64,
}

fn classify(ctx: &Ctx, r: &Pt, sols: &BTreeMap<Key, Vec<Pt>>, m: usize) -> Tally {
    let (f, c) = (ctx.f, ctx.c);
    let tau = c.parity(r);
    assert!(tau != 0);
    let t2 = c.two_torsion(f);
    let mut t = Tally::default();
    let mut obs_off = BTreeSet::new();
    for (k, w) in sols {
        t.solutions += 1;
        let mut sum = O;
        for q in w {
            sum = c.add(f, &sum, q);
        }
        if sum != *r {
            t.witness_fail += 1;
        }
        let mut b = O;
        let (mut n_off, mut n_tor) = (0, 0);
        for q in w {
            match c.parity(q) {
                0 => n_tor += 1,
                p if p == -tau => {
                    n_off += 1;
                    b = c.add(f, &b, q);
                }
                _ => {}
            }
        }
        if n_off == 0 {
            if n_tor == 0 {
                t.pure += 1
            } else {
                t.pure_with_2tor += 1
            }
        } else {
            obs_off.insert(k.clone());
            if b.inf {
                t.off_o += 1
            } else if b == t2 {
                t.off_t += 1
            } else {
                t.violation += 1
            }
        }
    }
    let pred = ctx.predicted_off(r, tau, m);
    t.obs_off = obs_off.len() as u64;
    t.pred_off = pred.len() as u64;
    t.obs_minus_pred = obs_off.difference(&pred).count() as u64;
    t.pred_minus_obs = pred.difference(&obs_off).count() as u64;
    t
}

fn tally_json(t: &Tally) -> String {
    format!(
        "\"solutions\":{},\"pure\":{},\"pure_with_2tor\":{},\"off_O\":{},\"off_T\":{},\"violation\":{},\"witness_fail\":{},\"obs_off\":{},\"pred_off\":{},\"obs_minus_pred\":{},\"pred_minus_obs\":{}",
        t.solutions, t.pure, t.pure_with_2tor, t.off_o, t.off_t, t.violation, t.witness_fail,
        t.obs_off, t.pred_off, t.obs_minus_pred, t.pred_minus_obs
    )
}

fn set_diff_json(a: &BTreeSet<Key>, b: &BTreeSet<Key>) -> String {
    let mut s = String::from("[");
    for (i, k) in a.difference(b).take(8).enumerate() {
        if i > 0 {
            s.push(',');
        }
        let _ = write!(s, "{:?}", k);
    }
    s.push(']');
    s
}

struct Plan {
    n: u32,
    poly: u32,
    l2: u32,
    l3: u32,
    targets: usize,
    planted: usize,
    poly_check: usize,
}

fn random_target(ctx: &Ctx, rng: &mut Rng, want: i32) -> Pt {
    let (f, c) = (ctx.f, ctx.c);
    loop {
        let x = 1 + rng.below((1u64 << f.n) - 1) as u32;
        let p = c.lift(f, x);
        if c.parity(&p) == want {
            return if rng.next() & 1 == 1 { c.neg(&p) } else { p };
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn run_decomp_target(
    out: &mut Vec<String>,
    f: &Fq,
    curve_id: &str,
    ca: &Curve,
    l: u32,
    m: usize,
    arm: &str,
    r: Pt,
    poly: bool,
) {
    // Arm on E_a, and the same x(R) on the twist model E_{a+1}.
    let cb = Curve {
        a: ca.a ^ 1,
        b: ca.b,
    };
    let ctx_a = Ctx::new(f, ca, l);
    let ctx_b = Ctx::new(f, &cb, l);
    let rb = cb.lift(f, r.x.u);
    assert!(cb.parity(&rb) == -ca.parity(&r));
    let t0 = Instant::now();
    let sa = ctx_a.decompose(&r, m);
    let sec_a = t0.elapsed().as_secs_f64();
    let sb = ctx_b.decompose(&rb, m);
    let ta = classify(&ctx_a, &r, &sa, m);
    let tb = classify(&ctx_b, &rb, &sb, m);
    let ka: BTreeSet<Key> = sa.keys().cloned().collect();
    let kb: BTreeSet<Key> = sb.keys().cloned().collect();
    let v_nonzero = (1u32 << l) - 1;
    let p_e = (1..1u32 << l)
        .filter(|&x| ca.parity(&ctx_a.lifts[x as usize]) == 1)
        .count();
    let mut poly_part = String::from("\"poly_checked\":false");
    if poly {
        let pr = ctx_a.poly_roots(r.x.u, m);
        poly_part = format!(
            "\"poly_checked\":true,\"poly_roots\":{},\"poly_minus_points\":{},\"points_minus_poly\":{},\"poly_minus_points_sample\":{},\"points_minus_poly_sample\":{}",
            pr.len(),
            pr.difference(&ka).count(),
            ka.difference(&pr).count(),
            set_diff_json(&pr, &ka),
            set_diff_json(&ka, &pr)
        );
    }
    out.push(format!(
        "{{\"family\":\"decomp\",\"n\":{},\"l\":{},\"m\":{},\"curve\":\"{}\",\"a\":{},\"b\":{},\"arm\":\"{}\",\"target_x\":{},\"v_nonzero\":{},\"v_E_type\":{},\"curve_model\":{{{}}},\"twist_model\":{{{}}},\"xsets_equal\":{},\"xset_a_minus_b\":{},\"xset_b_minus_a\":{},{},\"seconds_one_model\":{:.4}}}",
        f.n, l, m, curve_id, ca.a, ca.b, arm, r.x.u, v_nonzero, p_e,
        tally_json(&ta), tally_json(&tb), ka == kb,
        ka.difference(&kb).count(), kb.difference(&ka).count(), poly_part, sec_a
    ));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("dry");
    let out_dir = args.get(2).cloned().unwrap_or_else(|| ".".into());
    let seed: u64 = 0x7715_7_2026_1026;
    let mut rng = Rng(seed);

    // (n, irreducible poly, l for m=2, l for m=3, targets, planted, poly-checked)
    let plans: Vec<Plan> = match mode {
        "dry" => vec![Plan {
            n: 7,
            poly: 0x83,
            l2: 5,
            l3: 4,
            targets: 4,
            planted: 2,
            poly_check: 2,
        }],
        "probe" => vec![Plan {
            n: 19,
            poly: 0x80027,
            l2: 11,
            l3: 9,
            targets: 1,
            planted: 1,
            poly_check: 1,
        }],
        "full" => vec![
            Plan {
                n: 11,
                poly: 0x805,
                l2: 7,
                l3: 6,
                targets: 60,
                planted: 10,
                poly_check: 10,
            },
            Plan {
                n: 13,
                poly: 0x201B,
                l2: 8,
                l3: 7,
                targets: 60,
                planted: 10,
                poly_check: 6,
            },
            Plan {
                n: 17,
                poly: 0x20009,
                l2: 10,
                l3: 8,
                targets: 40,
                planted: 8,
                poly_check: 3,
            },
            Plan {
                n: 19,
                poly: 0x80027,
                l2: 11,
                l3: 9,
                targets: 30,
                planted: 6,
                poly_check: 2,
            },
        ],
        _ => panic!("mode: dry | probe | full"),
    };
    let census_ns: Vec<(u32, u32)> = match mode {
        "dry" => vec![(7, 0x83)],
        "probe" => vec![(13, 0x201B)],
        _ => vec![(7, 0x83), (11, 0x805), (13, 0x201B)],
    };
    let random_curves = match mode {
        "dry" => 2,
        "probe" => 0,
        _ => 4,
    };

    let mut rows: Vec<String> = Vec::new();
    let t_all = Instant::now();

    for &(n, poly) in &census_ns {
        let f = Fq::new(n, poly);
        let mut curves = vec![Curve { a: 0, b: 1 }];
        for _ in 0..random_curves {
            curves.push(Curve {
                a: rng.below(1 << n) as u32,
                b: 1 + rng.below((1 << n) - 1) as u32,
            });
        }
        for c in &curves {
            let row = census(&f, c);
            eprintln!("{}", row);
            rows.push(row);
        }
    }

    for p in &plans {
        let f = Fq::new(p.n, p.poly);
        let mut curves: Vec<(String, Curve)> = vec![("koblitz_a0_b1".into(), Curve { a: 0, b: 1 })];
        for i in 0..random_curves {
            curves.push((
                format!("rand{}", i),
                Curve {
                    a: rng.below(1 << p.n) as u32,
                    b: 1 + rng.below((1 << p.n) - 1) as u32,
                },
            ));
        }
        for (id, c) in &curves {
            for (m, l) in [(2usize, p.l2), (3usize, p.l3)] {
                let ctx = Ctx::new(&f, c, l);
                // Arm A: random rational targets.
                // m = 2 is cheap: 4x the targets, every one polynomial-checked.
                let n_targets = if m == 2 { 4 * p.targets } else { p.targets };
                for i in 0..n_targets {
                    let r = random_target(&ctx, &mut rng, 1);
                    let poly = m == 2 || i < p.poly_check;
                    run_decomp_target(&mut rows, &f, id, c, l, m, "A_random", r, poly);
                }
                // Arm C1 (m = 3): planted x(R) in V, so (x_R, x, x) mixed
                // solutions must appear for every off-parity x in V.
                if m == 3 {
                    let mut done = 0;
                    while done < p.planted {
                        let x = 1 + rng.below((1u64 << l) - 1) as u32;
                        let r = ctx.lifts[x as usize];
                        if c.parity(&r) != 1 {
                            continue;
                        }
                        run_decomp_target(
                            &mut rows,
                            &f,
                            id,
                            c,
                            l,
                            m,
                            "C1_planted_xR_in_V",
                            r,
                            false,
                        );
                        done += 1;
                    }
                }
            }
            eprintln!(
                "n={} curve={} done, {} rows, {:.1}s",
                p.n,
                id,
                rows.len(),
                t_all.elapsed().as_secs_f64()
            );
        }
        // Arm C2 (m = 3): constructed curves with sqrt(b) = v1 v2, v1, v2
        // off-parity in V, and targets with x(R + T) in V.
        let l = p.l3;
        let mut made = 0;
        while made < p.planted {
            let v1 = 1 + rng.below((1u64 << l) - 1) as u32;
            let v2 = 1 + rng.below((1u64 << l) - 1) as u32;
            let b = f.sq(f.mul(v1, v2));
            let mut c = Curve {
                a: rng.below(1 << p.n) as u32,
                b,
            };
            let (t1, t2) = (c.trace_type(&f, v1), c.trace_type(&f, v2));
            if t1 != t2 {
                continue;
            }
            if t1 == 0 {
                c.a ^= 1;
            }
            let ctx = Ctx::new(&f, &c, l);
            let mut tries = 0;
            let r = loop {
                tries += 1;
                let v3 = 1 + rng.below((1u64 << l) - 1) as u32;
                let xr = f.mul(f.mul(v1, v2), f.inv(v3));
                let r = c.lift(&f, xr);
                if c.parity(&r) == 1 || tries > 64 {
                    break r;
                }
            };
            if c.parity(&r) != 1 {
                continue;
            }
            let _ = ctx;
            run_decomp_target(
                &mut rows,
                &f,
                &format!("c2_{}", made),
                &c,
                l,
                3,
                "C2_planted_T_pairs",
                r,
                false,
            );
            made += 1;
        }
    }

    std::fs::create_dir_all(&out_dir).unwrap();
    let mut fh = std::fs::File::create(format!("{}/rows.jsonl", out_dir)).unwrap();
    for r in &rows {
        writeln!(fh, "{}", r).unwrap();
    }
    let mut meta = std::fs::File::create(format!("{}/run.json", out_dir)).unwrap();
    writeln!(
        meta,
        "{{\"mode\":\"{}\",\"seed\":\"0x{:x}\",\"rows\":{},\"seconds\":{:.2}}}",
        mode,
        seed,
        rows.len(),
        t_all.elapsed().as_secs_f64()
    )
    .unwrap();
    eprintln!(
        "wrote {} rows in {:.1}s",
        rows.len(),
        t_all.elapsed().as_secs_f64()
    );
}
