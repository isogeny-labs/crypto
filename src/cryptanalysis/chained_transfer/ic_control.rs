//! # Positive control for the index-calculus instrument: genus-3 IC on a full
//! Jacobian, with single and double large primes, against negation-map rho.
//!
//! Protocol: `research/ic_positive_control_20261112/PROTOCOL.md`.  Nothing here
//! is a result about the transfer; it asks whether the instrument can report a
//! win in the one regime where index calculus is known to beat rho
//! (`Õ(p^{4/3})` for `g = 3` with double large primes, against rho's `p^{3/2}`).
//!
//! ## What is counted
//!
//! * **Group operations** (`GroupOps`): every Cantor addition or doubling the
//!   solver performs, set-up, walk and the final `[t]G` check.
//! * **Native operations**, unpriced by the group count and priced at
//!   0, 0.1 and 1 GAE per operation by the caller (AGENTS.md §5a): modular
//!   multiplications (`mul`) and additions/subtractions (`add`) of the
//!   relation phase (factor-base set-up, smoothness tests, root finding) and
//!   of the linear algebra (graph reduction, peeling, Wiedemann).  The primary
//!   price uses `mul` only, the repository convention (`linalg_mulmods`);
//!   `add` is reported beside it as a sensitivity.
//!
//! ## Arms
//!
//! * [`Arm::Full`]: the factor base is every degree-1 place (up to sign);
//!   only fully smooth `u` are relations.
//! * [`Arm::SingleLp`]: a reduced base; relations with at most one place
//!   outside it, merged on the large prime.
//! * [`Arm::DoubleLp`]: a reduced base; relations with at most two places
//!   outside it, a graph on the large primes; cycles through the vertex "1"
//!   give full relations.
//!
//! All three feed the same pipeline: relations, cycle rows (the large-prime
//! columns eliminated along a breadth-first spanning tree), peeling of
//! columns of weight one, Wiedemann on `AᵀDA`, and a certificate `[t]G = T`
//! by an independent right-to-left multiplication.

use std::collections::HashMap;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use super::hyper64::{H64Curve, H64Div};
use crate::cryptanalysis::ic_boundary::{
    pivot_inversion_steps, rho_reference_walk, CountedGroup, GroupOps, RhoWalk,
};
use crate::cryptanalysis::residual_walk::{is_prime_u64, mix64};

// ── counters ─────────────────────────────────────────────────────────────

/// Native operations on residues: modular multiplications and additions or
/// subtractions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Nat {
    pub mul: u64,
    pub add: u64,
}

impl Nat {
    pub fn merge(&mut self, o: Nat) {
        self.mul += o.mul;
        self.add += o.add;
    }
}

// ── arithmetic modulo p (p < 2^31) ──────────────────────────────────────

#[inline]
fn pm(p: u64, a: u64, b: u64, n: &mut Nat) -> u64 {
    n.mul += 1;
    a * b % p
}
#[inline]
fn pa(p: u64, a: u64, b: u64, n: &mut Nat) -> u64 {
    n.add += 1;
    let s = a + b;
    if s >= p {
        s - p
    } else {
        s
    }
}
#[inline]
fn ps(p: u64, a: u64, b: u64, n: &mut Nat) -> u64 {
    n.add += 1;
    if a >= b {
        a - b
    } else {
        a + p - b
    }
}
fn ppow(p: u64, a: u64, mut e: u64, n: &mut Nat) -> u64 {
    let mut res = 1u64;
    let mut base = a % p;
    while e > 0 {
        if e & 1 == 1 {
            res = pm(p, res, base, n);
        }
        e >>= 1;
        if e > 0 {
            base = pm(p, base, base, n);
        }
    }
    res
}
fn pinv(p: u64, a: u64, n: &mut Nat) -> u64 {
    ppow(p, a, p - 2, n)
}

// ── polynomials modulo a monic u of degree 1..=3 over F_p ───────────────

/// Residues modulo a monic `u` of degree `d ≤ 3`, as `[c0, c1, c2]`.
struct ModU {
    p: u64,
    d: usize,
    /// `u` without its leading 1.
    u: [u64; 3],
}

impl ModU {
    fn mul(&self, a: &[u64; 3], b: &[u64; 3], n: &mut Nat) -> [u64; 3] {
        let (p, d) = (self.p, self.d);
        let mut t = [0u64; 5];
        for i in 0..d {
            for j in 0..d {
                let m = pm(p, a[i], b[j], n);
                t[i + j] = pa(p, t[i + j], m, n);
            }
        }
        for k in (d..(2 * d - 1)).rev() {
            let c = t[k];
            for j in 0..d {
                let m = pm(p, c, self.u[j], n);
                t[k - d + j] = ps(p, t[k - d + j], m, n);
            }
        }
        [t[0], t[1], t[2]]
    }

    /// `a · (x + aa)`.
    fn mul_xpa(&self, a: &[u64; 3], aa: u64, n: &mut Nat) -> [u64; 3] {
        let (p, d) = (self.p, self.d);
        let top = a[d - 1];
        let mut out = [0u64; 3];
        for j in 1..d {
            out[j] = a[j - 1];
        }
        for ((o, &uj), &aj) in out.iter_mut().zip(self.u.iter()).zip(a.iter()).take(d) {
            let m = pm(p, top, uj, n);
            *o = ps(p, *o, m, n);
            let m2 = pm(p, aa, aj, n);
            *o = pa(p, *o, m2, n);
        }
        out
    }

    /// `x + aa` modulo `u`.
    fn base(&self, aa: u64) -> [u64; 3] {
        let p = self.p;
        if self.d == 1 {
            // x ≡ −u0
            [(aa + p - self.u[0]) % p, 0, 0]
        } else {
            [aa, 1, 0]
        }
    }

    /// `(x + aa)^e mod u`, `e ≥ 1`.
    fn pow_xpa(&self, aa: u64, e: u64, n: &mut Nat) -> [u64; 3] {
        let mut res = self.base(aa);
        let bits = 64 - e.leading_zeros();
        for i in (0..bits - 1).rev() {
            res = self.mul(&res, &res, n);
            if (e >> i) & 1 == 1 {
                res = self.mul_xpa(&res, aa, n);
            }
        }
        res
    }
}

type Poly = Vec<u64>;

fn ptrim(a: &mut Poly) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

/// `a mod m` for any nonzero `m` (made monic by one inversion).
fn prem(p: u64, a: &Poly, m: &Poly, n: &mut Nat) -> Poly {
    let mut r = a.clone();
    ptrim(&mut r);
    let dm = m.len() - 1;
    let lead = m[dm];
    let il = if lead == 1 { 1 } else { pinv(p, lead, n) };
    while r.len() > dm {
        let dr = r.len() - 1;
        let c = if il == 1 { r[dr] } else { pm(p, r[dr], il, n) };
        for j in 0..dm {
            let t = pm(p, c, m[j], n);
            r[dr - dm + j] = ps(p, r[dr - dm + j], t, n);
        }
        r.pop();
        ptrim(&mut r);
    }
    r
}

/// `a / m` for monic `m`, exact or not (quotient only).
fn pdiv_monic(p: u64, a: &Poly, m: &Poly, n: &mut Nat) -> Poly {
    let mut r = a.clone();
    let dm = m.len() - 1;
    if r.len() <= dm {
        return Vec::new();
    }
    let mut q = vec![0u64; r.len() - dm];
    while r.len() > dm {
        let dr = r.len() - 1;
        let c = r[dr];
        q[dr - dm] = c;
        for j in 0..dm {
            let t = pm(p, c, m[j], n);
            r[dr - dm + j] = ps(p, r[dr - dm + j], t, n);
        }
        r.pop();
    }
    q
}

fn pgcd_monic(p: u64, a: &Poly, b: &Poly, n: &mut Nat) -> Poly {
    let (mut a, mut b) = (a.clone(), b.clone());
    ptrim(&mut a);
    ptrim(&mut b);
    while !b.is_empty() {
        let r = prem(p, &a, &b, n);
        a = b;
        b = r;
    }
    if let Some(&lead) = a.last() {
        if lead != 1 {
            let il = pinv(p, lead, n);
            for c in a.iter_mut() {
                *c = pm(p, *c, il, n);
            }
        }
    }
    a
}

/// `u | x^p − x` for monic `u` of degree 1..=3 (so `u` is squarefree and
/// splits completely over `F_p`).
fn splits_completely(p: u64, u: &Poly, n: &mut Nat) -> bool {
    let d = u.len() - 1;
    let mut uu = [0u64; 3];
    uu[..d].copy_from_slice(&u[..d]);
    let ctx = ModU { p, d, u: uu };
    let xp = ctx.pow_xpa(0, p, n);
    let x = ctx.base(0);
    xp[..d] == x[..d]
}

/// The roots of a monic squarefree `u` known to split completely
/// (Cantor–Zassenhaus).  `None` only if `u` does not split.
fn roots_of(p: u64, u: &Poly, rng: &mut StdRng, n: &mut Nat) -> Option<Vec<u64>> {
    let d = u.len() - 1;
    if d == 0 {
        return Some(Vec::new());
    }
    if d == 1 {
        return Some(vec![(p - u[0]) % p]);
    }
    for _ in 0..64 {
        let aa = rng.gen_range(0..p);
        let mut uu = [0u64; 3];
        uu[..d].copy_from_slice(&u[..d]);
        let ctx = ModU { p, d, u: uu };
        let h = ctx.pow_xpa(aa, (p - 1) / 2, n);
        let mut hm1: Poly = h[..d].to_vec();
        hm1[0] = ps(p, hm1[0], 1, n);
        ptrim(&mut hm1);
        let g = pgcd_monic(p, u, &hm1, n);
        let dg = g.len() - 1;
        if dg == 0 || dg == d {
            continue;
        }
        let q = pdiv_monic(p, u, &g, n);
        let mut out = roots_of(p, &g, rng, n)?;
        out.extend(roots_of(p, &q, rng, n)?);
        return Some(out);
    }
    None
}

// ── arithmetic modulo r (prime, < 2^50 for the fast path) ───────────────

#[inline]
fn mulmod(a: u64, b: u64, r: u64) -> u64 {
    if r < (1u64 << 50) {
        let q = ((a as f64) * (b as f64) / (r as f64)) as u64;
        let t = a.wrapping_mul(b).wrapping_sub(q.wrapping_mul(r)) as i64;
        let ri = r as i64;
        (if t < 0 {
            t + ri
        } else if t >= ri {
            t - ri
        } else {
            t
        }) as u64
    } else {
        ((a as u128 * b as u128) % r as u128) as u64
    }
}
#[inline]
fn addm(a: u64, b: u64, r: u64) -> u64 {
    let s = a + b;
    if s >= r {
        s - r
    } else {
        s
    }
}
#[inline]
fn subm(a: u64, b: u64, r: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + r - b
    }
}
fn powm(mut a: u64, mut e: u64, r: u64) -> u64 {
    let mut res = 1;
    while e > 0 {
        if e & 1 == 1 {
            res = mulmod(res, a, r);
        }
        a = mulmod(a, a, r);
        e >>= 1;
    }
    res
}
fn invm(a: u64, r: u64) -> u64 {
    powm(a, r - 2, r)
}

// ── instances ───────────────────────────────────────────────────────────

/// A genus-3 Jacobian with `#J = h·r`, `r` prime, a generator `G` of the
/// order-`r` subgroup and a target `T = [t]G`.
#[derive(Clone, Debug)]
pub struct Instance {
    pub curve: H64Curve,
    pub p: u64,
    pub order: u64,
    pub h: u64,
    pub r: u64,
    pub g: H64Div,
    pub t_secret: u64,
    pub target: H64Div,
}

/// `#J` for a genus-3 curve: the first two `L`-polynomial coefficients from
/// `N_1, N_2`, then baby-step giant-step over the Hasse–Weil window for the
/// last one with a random divisor, every candidate checked against 32 random
/// divisors; exactly one must survive.  `n2` is `N_2` (the `O(p²)` count),
/// supplied so the caller can reuse or thread it.
pub fn jacobian_order_bsgs(curve: &H64Curve, n2: u64, seed: u64) -> Result<u64, String> {
    let g = 3usize;
    let p = curve.p() as i128;
    let n1 = curve.count_points(1) as i128;
    let s = [n1 - p - 1, n2 as i128 - p * p - 1];
    let mut c = [0i128; 3];
    c[0] = 1;
    for k in 1..g {
        let num: i128 = (1..=k).map(|i| s[i - 1] * c[k - i]).sum();
        if num % k as i128 != 0 {
            return Err("Newton integrality".into());
        }
        c[k] = num / k as i128;
    }
    let base: i128 =
        c.iter().sum::<i128>() + (1..=g).map(|j| p.pow(j as u32) * c[g - j]).sum::<i128>();
    let bound = (20.0 * (p as f64).powf(1.5)).ceil() as i128 + 1;
    let sp = (p as f64).sqrt();
    let lo = ((sp - 1.0).powi(6) * (1.0 - 1e-9)).floor().max(1.0) as i128;
    let hi = ((sp + 1.0).powi(6) * (1.0 + 1e-9)).ceil() as i128;
    let from = (base - bound).max(lo).max(1);
    let to = (base + bound).min(hi);
    if to <= from {
        return Err("empty window".into());
    }
    let width = (to - from) as u64;
    let mut st = seed ^ 0xA5A5_5A5A;
    let d0 = curve.random_divisor(&mut st);
    let m = ((width as f64).sqrt().ceil() as u64).max(1);
    let mut baby: HashMap<u64, u64> = HashMap::with_capacity(m as usize + 1);
    let mut cur = H64Div::IDENTITY;
    for j in 0..=m {
        baby.entry(curve.key_of(&cur)).or_insert(j);
        cur = curve.cantor(cur, d0);
    }
    // w = −from·D0
    let w = curve.negate(curve.mul_uncounted(d0, from as u64));
    let md0 = curve.mul_uncounted(d0, m);
    let nmd0 = curve.negate(md0);
    let mut q = w;
    let mut cands = Vec::new();
    for i in 0..=(width / m + 1) {
        if let Some(&j) = baby.get(&curve.key_of(&q)) {
            let sidx = i * m + j;
            if sidx <= width {
                let n = from as u64 + sidx;
                if curve.mul_uncounted(d0, n).is_identity() {
                    cands.push(n);
                }
            }
        }
        q = curve.cantor(q, nmd0);
    }
    cands.sort_unstable();
    cands.dedup();
    let tests: Vec<H64Div> = (0..32).map(|_| curve.random_divisor(&mut st)).collect();
    let surv: Vec<u64> = cands
        .into_iter()
        .filter(|&n| {
            tests
                .iter()
                .all(|t| curve.mul_uncounted(*t, n).is_identity())
        })
        .collect();
    match surv.as_slice() {
        [one] => Ok(*one),
        [] => Err("no candidate".into()),
        many => Err(format!("ambiguous: {} candidates", many.len())),
    }
}

/// Split `n = h·r` with `r` prime, `r > h`, `h ≤ max_h`, `h` the smallest
/// such cofactor.
pub fn split_order(n: u64, max_h: u64) -> Option<(u64, u64)> {
    for h in 1..=max_h {
        if n.is_multiple_of(h) {
            let r = n / h;
            if r > h && is_prime_u64(r) {
                return Some((h, r));
            }
        }
    }
    None
}

/// A random monic `y² = f(x)`, `deg f = 7`, from `seed`.
pub fn random_curve(p: u64, seed: u64) -> Option<H64Curve> {
    let mut st = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ p;
    let mut f = Vec::with_capacity(8);
    for _ in 0..7 {
        st = st.wrapping_add(0x9E37_79B9_7F4A_7C15);
        f.push(mix64(st) % p);
    }
    f.push(1);
    H64Curve::new(p, 3, &f).ok()
}

/// Build one instance at `p` from the curve seed `curve_seed` and a target
/// seed; `None` if the curve has no `#J = h·r` with `h ≤ max_h`.
pub fn make_instance(
    p: u64,
    curve_seed: u64,
    target_seed: u64,
    max_h: u64,
) -> Result<Instance, String> {
    let curve = random_curve(p, curve_seed).ok_or("f not squarefree")?;
    let n2 = curve.count_points(2);
    let order = jacobian_order_bsgs(&curve, n2, curve_seed)?;
    let (h, r) = split_order(order, max_h).ok_or_else(|| format!("#J = {order} has no h*r"))?;
    instance_from(curve, order, h, r, target_seed)
}

/// The generator and target for a curve of known `#J = h·r`.
pub fn instance_from(
    curve: H64Curve,
    order: u64,
    h: u64,
    r: u64,
    target_seed: u64,
) -> Result<Instance, String> {
    let p = curve.p();
    let mut st = target_seed ^ 0x1234_5678_9ABC_DEF0;
    let mut g = H64Div::IDENTITY;
    for _ in 0..64 {
        let d = curve.random_divisor(&mut st);
        let cand = curve.mul_uncounted(d, h);
        if !cand.is_identity() && curve.mul_uncounted(cand, r).is_identity() {
            g = cand;
            break;
        }
    }
    if g.is_identity() {
        return Err("no generator of order r".into());
    }
    st = mix64(st);
    let t_secret = 1 + mix64(st) % (r - 1);
    let target = curve.mul_rtl(g, t_secret);
    Ok(Instance {
        curve,
        p,
        order,
        h,
        r,
        g,
        t_secret,
        target,
    })
}

// ── the index-calculus arms ─────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    Full,
    SingleLp,
    DoubleLp,
}

impl Arm {
    pub fn name(self) -> &'static str {
        match self {
            Arm::Full => "A_full",
            Arm::SingleLp => "B_single_lp",
            Arm::DoubleLp => "C_double_lp",
        }
    }
    pub fn max_lp(self) -> usize {
        match self {
            Arm::Full => 0,
            Arm::SingleLp => 1,
            Arm::DoubleLp => 2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct IcParams {
    pub arm: Arm,
    /// Factor-base columns (ignored for [`Arm::Full`], which takes every
    /// place).
    pub nb: usize,
    pub seed: u64,
    /// Give up after this many trials.
    pub max_trials: u64,
    /// Check every accepted relation by group arithmetic (tests only).
    pub verify_relations: bool,
}

/// Factor-base size by the frozen rule `nb = ceil(c · p^α)`.
pub fn fb_size(arm: Arm, p: u64, c_b: f64, c_c: f64) -> usize {
    let pf = p as f64;
    match arm {
        Arm::Full => usize::MAX,
        Arm::SingleLp => (c_b * pf.powf(0.75)).ceil() as usize,
        Arm::DoubleLp => (c_c * pf.powf(2.0 / 3.0)).ceil() as usize,
    }
}

#[derive(Clone, Debug, Default)]
pub struct IcResult {
    pub arm: &'static str,
    pub p: u64,
    pub recovered: Option<u64>,
    pub certified: bool,
    pub exhausted: bool,
    /// Every Cantor addition and doubling.
    pub group: GroupOps,
    pub setup_ops: u64,
    pub walk_ops: u64,
    pub verify_ops: u64,
    /// Relation phase native operations (set-up, smoothness, roots).
    pub rel: Nat,
    /// Linear-algebra native operations (graph, peel, Wiedemann).
    pub la: Nat,
    pub la_graph: Nat,
    pub la_peel: Nat,
    pub la_wied: Nat,
    pub fb_cols: usize,
    pub trials: u64,
    pub splits: u64,
    pub relations: u64,
    pub rels_by_lp: [u64; 3],
    pub bad_relations: u64,
    pub cycles: i64,
    pub core_rows: usize,
    pub core_cols: usize,
    pub attempts: u32,
    pub wall_ns: u64,
}

impl IcResult {
    pub fn group_ops(&self) -> u64 {
        self.group.adds + self.group.doubles
    }
    /// Native operations, primary reading: multiplications only.
    pub fn native_mul(&self) -> u64 {
        self.rel.mul + self.la.mul
    }
    /// Sensitivity reading: multiplications plus additions/subtractions.
    pub fn native_all(&self) -> u64 {
        self.rel.mul + self.rel.add + self.la.mul + self.la.add
    }
    /// Total at `w` GAE per native multiplication.
    pub fn total(&self, w: f64) -> f64 {
        self.group_ops() as f64 + w * self.native_mul() as f64
    }
}

#[derive(Clone, Copy, Debug)]
struct Rel {
    fb: [(u32, i8); 3],
    nfb: u8,
    lp: [(u32, i8); 2],
    nlp: u8,
    /// Coefficient of `t`: `−h·b mod r`.
    tcoef: u64,
    /// `h·a mod r`.
    rhs: u64,
}

impl Rel {
    fn endpoints(&self) -> (u32, i8, u32, i8) {
        match self.nlp {
            0 => (0, 0, 0, 0),
            1 => (0, 0, self.lp[0].0, self.lp[0].1),
            _ => (self.lp[0].0, self.lp[0].1, self.lp[1].0, self.lp[1].1),
        }
    }
}

type Row = (Vec<(u32, u64)>, u64);

/// `sa·a + sb·b` as sorted sparse vectors; `neg*` flips the sign.
fn combine(
    a: &[(u32, u64)],
    neg_a: bool,
    b: &[(u32, u64)],
    neg_b: bool,
    r: u64,
    nat: &mut Nat,
) -> Vec<(u32, u64)> {
    let f = |v: u64, neg: bool| if neg && v != 0 { r - v } else { v };
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    nat.add += (a.len() + b.len()) as u64;
    while i < a.len() || j < b.len() {
        if j == b.len() || (i < a.len() && a[i].0 < b[j].0) {
            out.push((a[i].0, f(a[i].1, neg_a)));
            i += 1;
        } else if i == a.len() || b[j].0 < a[i].0 {
            out.push((b[j].0, f(b[j].1, neg_b)));
            j += 1;
        } else {
            let v = addm(f(a[i].1, neg_a), f(b[j].1, neg_b), r);
            if v != 0 {
                out.push((a[i].0, v));
            }
            i += 1;
            j += 1;
        }
    }
    out
}

struct Dsu {
    parent: Vec<u32>,
    edges: Vec<u32>,
    verts: Vec<u32>,
    ops: u64,
}

impl Dsu {
    fn new() -> Dsu {
        Dsu {
            parent: vec![0],
            edges: vec![0],
            verts: vec![1],
            ops: 0,
        }
    }
    fn add_vertex(&mut self) -> u32 {
        let id = self.parent.len() as u32;
        self.parent.push(id);
        self.edges.push(0);
        self.verts.push(1);
        id
    }
    fn find(&mut self, mut x: u32) -> u32 {
        while self.parent[x as usize] != x {
            let gp = self.parent[self.parent[x as usize] as usize];
            self.parent[x as usize] = gp;
            x = gp;
            self.ops += 1;
        }
        x
    }
    fn edge(&mut self, a: u32, b: u32) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            self.edges[ra as usize] += 1;
        } else {
            self.parent[rb as usize] = ra;
            self.edges[ra as usize] += self.edges[rb as usize] + 1;
            self.verts[ra as usize] += self.verts[rb as usize];
        }
        self.ops += 1;
    }
    /// Independent cycles in the component of the vertex "1".
    fn cycles0(&mut self) -> i64 {
        let r0 = self.find(0) as usize;
        self.edges[r0] as i64 - self.verts[r0] as i64 + 1
    }
}

/// The cycle rows of the relation graph: large-prime columns eliminated
/// along a breadth-first spanning tree of the component of the vertex "1".
fn build_rows(rels: &[Rel], nverts: usize, tcol: u32, r: u64, nat: &mut Nat) -> Vec<Row> {
    let neg = |e: i8| if e > 0 { 1u64 } else { r - 1 }; // coefficient as residue
    let frow = |rel: &Rel| -> Vec<(u32, u64)> {
        let mut v: Vec<(u32, u64)> = rel.fb[..rel.nfb as usize]
            .iter()
            .map(|&(c, e)| (c, neg(e)))
            .collect();
        v.push((tcol, rel.tcoef));
        v.sort_unstable_by_key(|x| x.0);
        v
    };
    // adjacency over non-loop edges
    let mut adj: Vec<Vec<u32>> = vec![Vec::new(); nverts];
    for (i, rel) in rels.iter().enumerate() {
        if rel.nlp > 0 {
            let (a, _, b, _) = rel.endpoints();
            adj[a as usize].push(i as u32);
            adj[b as usize].push(i as u32);
            nat.add += 2;
        }
    }
    let mut beta: Vec<Option<(Vec<(u32, u64)>, u64)>> = vec![None; nverts];
    beta[0] = Some((Vec::new(), 0));
    let mut is_tree = vec![false; rels.len()];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0u32);
    while let Some(u) = queue.pop_front() {
        let edges = adj[u as usize].clone();
        for ei in edges {
            let rel = &rels[ei as usize];
            let (a, ea, b, eb) = rel.endpoints();
            let (v, ev, eu) = if a == u { (b, eb, ea) } else { (a, ea, eb) };
            if beta[v as usize].is_some() {
                continue;
            }
            is_tree[ei as usize] = true;
            let (bu, cu) = beta[u as usize].clone().expect("parent reached");
            // ℓ_v = ev·(rhs − F − tcoef t) − ev·eu·ℓ_u
            let fv = frow(rel);
            let neg_f = ev > 0;
            let neg_u = (ev as i32) * (eu as i32) > 0;
            let terms = combine(&fv, neg_f, &bu, neg_u, r, nat);
            let mut c = if ev > 0 { rel.rhs } else { subm(0, rel.rhs, r) };
            let cu_term = if (ev as i32) * (eu as i32) > 0 {
                subm(0, cu, r)
            } else {
                cu
            };
            c = addm(c, cu_term, r);
            nat.add += 2;
            beta[v as usize] = Some((terms, c));
            queue.push_back(v);
        }
    }
    let mut rows = Vec::new();
    for (i, rel) in rels.iter().enumerate() {
        if is_tree[i] {
            continue;
        }
        let (a, ea, b, eb) = rel.endpoints();
        if rel.nlp > 0 && (beta[a as usize].is_none() || beta[b as usize].is_none()) {
            continue;
        }
        let mut terms = frow(rel);
        let mut rhs = rel.rhs;
        if rel.nlp > 0 {
            if a != 0 {
                let (ba, ca) = beta[a as usize].as_ref().expect("reached");
                terms = combine(&terms, false, ba, ea < 0, r, nat);
                rhs = if ea > 0 {
                    subm(rhs, *ca, r)
                } else {
                    addm(rhs, *ca, r)
                };
                nat.add += 1;
            }
            if b != 0 {
                let (bb, cb) = beta[b as usize].as_ref().expect("reached");
                terms = combine(&terms, false, bb, eb < 0, r, nat);
                rhs = if eb > 0 {
                    subm(rhs, *cb, r)
                } else {
                    addm(rhs, *cb, r)
                };
                nat.add += 1;
            }
        }
        rows.push((terms, rhs));
    }
    rows
}

/// Peel columns of weight one (and the rows holding them) from `rows`; the
/// column `tcol` is never peeled.  Returns the surviving rows and the number
/// of surviving columns, columns renumbered densely, `t` last.
fn peel(rows: &[Row], ncols: usize, tcol: u32, nat: &mut Nat) -> (Vec<Row>, usize, u32) {
    let mut colrows: Vec<Vec<u32>> = vec![Vec::new(); ncols];
    let mut w = vec![0u32; ncols];
    for (i, (t, _)) in rows.iter().enumerate() {
        for &(c, _) in t {
            colrows[c as usize].push(i as u32);
            w[c as usize] += 1;
            nat.add += 1;
        }
    }
    let mut alive = vec![true; rows.len()];
    let mut stack: Vec<u32> = (0..ncols as u32)
        .filter(|&c| w[c as usize] == 1 && c != tcol)
        .collect();
    while let Some(c) = stack.pop() {
        if w[c as usize] != 1 {
            continue;
        }
        let Some(&ri) = colrows[c as usize].iter().find(|&&i| alive[i as usize]) else {
            continue;
        };
        alive[ri as usize] = false;
        for &(cc, _) in &rows[ri as usize].0 {
            nat.add += 1;
            w[cc as usize] -= 1;
            if w[cc as usize] == 1 && cc != tcol {
                stack.push(cc);
            }
        }
    }
    let mut map = vec![u32::MAX; ncols];
    let mut n = 0u32;
    for c in 0..ncols {
        if c as u32 != tcol && w[c] > 0 {
            map[c] = n;
            n += 1;
        }
    }
    map[tcol as usize] = n;
    let new_t = n;
    let mut out = Vec::new();
    for (i, (t, rhs)) in rows.iter().enumerate() {
        if alive[i] {
            let mut nt: Vec<(u32, u64)> = t.iter().map(|&(c, v)| (map[c as usize], v)).collect();
            nt.sort_unstable_by_key(|x| x.0);
            out.push((nt, *rhs));
        }
    }
    (out, n as usize + 1, new_t)
}

/// `y = A x` (rows) with the operation count.
fn mat_vec(rows: &[Row], x: &[u64], y: &mut [u64], r: u64, nat: &mut Nat) {
    for (i, (t, _)) in rows.iter().enumerate() {
        let mut acc = 0u64;
        for &(c, v) in t {
            let xv = x[c as usize];
            if v == 1 {
                acc = addm(acc, xv, r);
            } else if v == r - 1 {
                acc = subm(acc, xv, r);
            } else {
                nat.mul += 1;
                acc = addm(acc, mulmod(v, xv, r), r);
            }
            nat.add += 1;
        }
        y[i] = acc;
    }
}

/// `z = Aᵀ y`.
fn mat_t_vec(rows: &[Row], y: &[u64], z: &mut [u64], r: u64, nat: &mut Nat) {
    z.iter_mut().for_each(|v| *v = 0);
    for (i, (t, _)) in rows.iter().enumerate() {
        let yv = y[i];
        if yv == 0 {
            continue;
        }
        for &(c, v) in t {
            let term = if v == 1 {
                yv
            } else if v == r - 1 {
                subm(0, yv, r)
            } else {
                nat.mul += 1;
                mulmod(v, yv, r)
            };
            z[c as usize] = addm(z[c as usize], term, r);
            nat.add += 1;
        }
    }
}

/// `out = AᵀDA v`.
fn m_apply(
    rows: &[Row],
    d: &[u64],
    v: &[u64],
    tmp: &mut [u64],
    out: &mut [u64],
    r: u64,
    nat: &mut Nat,
) {
    mat_vec(rows, v, tmp, r, nat);
    for (t, &di) in tmp.iter_mut().zip(d.iter()) {
        nat.mul += 1;
        *t = mulmod(*t, di, r);
    }
    mat_t_vec(rows, tmp, out, r, nat);
}

/// Berlekamp–Massey; returns `c[0..=L]` with `c[0] = 1`,
/// `Σ_i c[i]·s[n−i] = 0` for `n ≥ L`.
fn berlekamp_massey(s: &[u64], r: u64, nat: &mut Nat) -> Vec<u64> {
    let mut c = vec![1u64];
    let mut b = vec![1u64];
    let mut l = 0usize;
    let mut m = 1usize;
    let mut binv = 1u64;
    for n in 0..s.len() {
        let mut d = s[n];
        for i in 1..=l.min(c.len() - 1) {
            nat.mul += 1;
            nat.add += 1;
            d = addm(d, mulmod(c[i], s[n - i], r), r);
        }
        if d == 0 {
            m += 1;
            continue;
        }
        let coef = {
            nat.mul += 1;
            mulmod(d, binv, r)
        };
        let t = c.clone();
        if c.len() < b.len() + m {
            c.resize(b.len() + m, 0);
        }
        for (i, &bv) in b.iter().enumerate() {
            nat.mul += 1;
            nat.add += 1;
            c[i + m] = subm(c[i + m], mulmod(coef, bv, r), r);
        }
        if 2 * l <= n {
            l = n + 1 - l;
            b = t;
            nat.mul += pivot_inversion_steps(r);
            binv = invm(d, r);
            m = 1;
        } else {
            m += 1;
        }
    }
    c.truncate(l + 1);
    c.resize(l + 1, 0);
    c
}

/// Solve `A x = c` (rows × cols, full column rank) by Wiedemann on
/// `AᵀDA x = AᵀD c`.  `None` if the recurrence is degenerate or the residual
/// is nonzero.
fn wiedemann(
    rows: &[Row],
    ncols: usize,
    r: u64,
    rng: &mut StdRng,
    dense_u: bool,
    nat: &mut Nat,
) -> Option<Vec<u64>> {
    let nr = rows.len();
    let d: Vec<u64> = (0..nr).map(|_| rng.gen_range(1..r)).collect();
    // b' = Aᵀ D c
    let mut tmp = vec![0u64; nr];
    for ((t, row), &di) in tmp.iter_mut().zip(rows.iter()).zip(d.iter()) {
        nat.mul += 1;
        *t = mulmod(row.1, di, r);
    }
    let mut bp = vec![0u64; ncols];
    mat_t_vec(rows, &tmp, &mut bp, r, nat);
    // projection u
    let mut proj: Vec<(usize, u64)> = Vec::new();
    if dense_u || ncols <= 16 {
        proj.extend((0..ncols).map(|i| (i, rng.gen_range(1..r))));
    } else {
        for _ in 0..16 {
            proj.push((rng.gen_range(0..ncols), rng.gen_range(1..r)));
        }
    }
    let len = 2 * ncols + 2;
    let mut seq = Vec::with_capacity(len);
    let mut v = bp.clone();
    let mut w = vec![0u64; ncols];
    for _ in 0..len {
        let mut s = 0u64;
        for &(i, uv) in &proj {
            nat.mul += 1;
            nat.add += 1;
            s = addm(s, mulmod(uv, v[i], r), r);
        }
        seq.push(s);
        m_apply(rows, &d, &v, &mut tmp, &mut w, r, nat);
        std::mem::swap(&mut v, &mut w);
    }
    let c = berlekamp_massey(&seq, r, nat);
    let l = c.len() - 1;
    if l == 0 || c[l] == 0 {
        return None;
    }
    // x = −c_L⁻¹ Σ_{j=0}^{L−1} c_{L−1−j}... via Horner on M:
    // P(M) b' = 0 with P(z) = z^L + c_1 z^{L−1} + … + c_L.
    // x = −c_L⁻¹ (M^{L−1} b' + c_1 M^{L−2} b' + … + c_{L−1} b').
    let mut acc = vec![0u64; ncols];
    let mut v = bp.clone();
    for j in 0..l {
        // term M^j b' has coefficient c_{L−1−j}
        let coef = c[l - 1 - j];
        for (a, &vi) in acc.iter_mut().zip(v.iter()) {
            nat.mul += 1;
            nat.add += 1;
            *a = addm(*a, mulmod(coef, vi, r), r);
        }
        if j + 1 < l {
            m_apply(rows, &d, &v, &mut tmp, &mut w, r, nat);
            std::mem::swap(&mut v, &mut w);
        }
    }
    nat.mul += pivot_inversion_steps(r);
    let sc = subm(0, invm(c[l], r), r);
    for a in acc.iter_mut() {
        nat.mul += 1;
        *a = mulmod(*a, sc, r);
    }
    // residual A x = c
    let mut ax = vec![0u64; nr];
    mat_vec(rows, &acc, &mut ax, r, nat);
    if ax.iter().zip(rows.iter()).all(|(a, row)| *a == row.1) {
        Some(acc)
    } else {
        None
    }
}

/// Run one index-calculus arm on an instance.
pub fn run_ic(inst: &Instance, prm: IcParams) -> IcResult {
    let start = std::time::Instant::now();
    let curve = &inst.curve;
    let (p, r, h) = (inst.p, inst.r, inst.h);
    let mut rng = StdRng::seed_from_u64(prm.seed);
    let mut res = IcResult {
        arm: prm.arm.name(),
        p,
        ..Default::default()
    };
    let mut ops = GroupOps::default();
    let f = curve.f_coeffs();

    // ── factor base ──
    let want = if prm.arm == Arm::Full {
        usize::MAX
    } else {
        prm.nb
    };
    let mut col = vec![u32::MAX; p as usize];
    let mut ncols = 0usize;
    {
        let half = (p - 1) / 2;
        for x in 0..p {
            if ncols >= want {
                break;
            }
            let mut fx = 0u64;
            for &cf in f.iter().rev() {
                fx = pm(p, fx, x, &mut res.rel);
                fx = pa(p, fx, cf, &mut res.rel);
            }
            if fx == 0 {
                continue;
            }
            if ppow(p, fx, half, &mut res.rel) == 1 {
                col[x as usize] = ncols as u32;
                ncols += 1;
            }
        }
    }
    res.fb_cols = ncols;
    let tcol = ncols as u32;
    let nunk = ncols + 1;

    // ── walk set-up: 16 jumps α G + β T, short coefficients ──
    let short = r.min(1 << 12);
    let mut jumps: Vec<(H64Div, u64, u64)> = Vec::new();
    for _ in 0..16 {
        let a = rng.gen_range(1..short);
        let b = rng.gen_range(1..short);
        let ag = curve.mul(&mut ops, inst.g, a);
        let bt = curve.mul(&mut ops, inst.target, b);
        jumps.push((curve.add(&mut ops, ag, bt), a, b));
    }
    res.setup_ops = ops.adds + ops.doubles;

    let mut rels: Vec<Rel> = Vec::new();
    let mut vid: HashMap<u32, u32> = HashMap::new();
    let mut dsu = Dsu::new();
    let margin = |n: usize| 8 + n / 100;
    let mut cur = H64Div::IDENTITY;
    let (mut ca, mut cb) = (0u64, 0u64);
    let mut since_restart = u64::MAX;
    let mut next_check = nunk as u64;
    let mut solved = false;

    'outer: while !solved {
        // ── collect until the next check ──
        while (rels.len() as u64) < next_check {
            if res.trials >= prm.max_trials {
                res.exhausted = true;
                break 'outer;
            }
            if since_restart >= 4096 {
                let a = rng.gen_range(1..r);
                let b = rng.gen_range(1..r);
                let ag = curve.mul(&mut ops, inst.g, a);
                let bt = curve.mul(&mut ops, inst.target, b);
                cur = curve.add(&mut ops, ag, bt);
                ca = a;
                cb = b;
                since_restart = 0;
            } else {
                let j = (mix64(curve.key_of(&cur)) >> 24) as usize % 16;
                let (m, aj, bj) = jumps[j];
                cur = curve.add(&mut ops, cur, m);
                ca = addm(ca, aj, r);
                cb = addm(cb, bj, r);
                since_restart += 1;
            }
            res.trials += 1;
            if cur.deg != 3 {
                continue;
            }
            let u: Poly = cur.u_coeffs();
            if !splits_completely(p, &u, &mut res.rel) {
                continue;
            }
            res.splits += 1;
            let Some(roots) = roots_of(p, &u, &mut rng, &mut res.rel) else {
                continue;
            };
            let vc = cur.v_coeffs();
            let mut rel = Rel {
                fb: [(0, 0); 3],
                nfb: 0,
                lp: [(0, 0); 2],
                nlp: 0,
                tcoef: 0,
                rhs: 0,
            };
            let mut lps: Vec<(u32, i8)> = Vec::new();
            let mut places: Vec<(u64, u64)> = Vec::new();
            for &x in &roots {
                let mut y = 0u64;
                for &cf in vc.iter().rev() {
                    y = pm(p, y, x, &mut res.rel);
                    y = pa(p, y, cf, &mut res.rel);
                }
                places.push((x, y));
                if y == 0 {
                    continue; // 2-torsion place: its log is 0 in the r-part
                }
                let eps: i8 = if y > (p - 1) / 2 { -1 } else { 1 };
                if col[x as usize] != u32::MAX {
                    rel.fb[rel.nfb as usize] = (col[x as usize], eps);
                    rel.nfb += 1;
                } else {
                    lps.push((x as u32, eps));
                }
            }
            if lps.len() > prm.arm.max_lp() {
                continue;
            }
            if rel.nfb == 0 && lps.is_empty() {
                continue;
            }
            res.rel.mul += 2;
            rel.rhs = mulmod(h % r, ca, r);
            rel.tcoef = subm(0, mulmod(h % r, cb, r), r);
            if prm.verify_relations {
                // Σ ε hP_i must equal h R, tested on the divisor classes.
                let mut sum = H64Div::IDENTITY;
                for &(x, y) in &places {
                    let pt = curve.point(x, y).expect("a root of u lies on the curve");
                    sum = curve.cantor(sum, pt);
                }
                if sum != cur {
                    res.bad_relations += 1;
                }
            }
            for (x, e) in lps {
                let id = *vid.entry(x).or_insert_with(|| dsu.add_vertex());
                rel.lp[rel.nlp as usize] = (id, e);
                rel.nlp += 1;
            }
            let (a, _, b, _) = rel.endpoints();
            dsu.edge(a, b);
            res.rels_by_lp[rel.nlp as usize] += 1;
            res.relations += 1;
            rels.push(rel);
        }

        // ── is a solve worth trying? ──
        res.cycles = dsu.cycles0();
        let nver = dsu.parent.len();
        if prm.arm != Arm::Full {
            let need = (nunk + margin(nunk)) as i64;
            if res.cycles < need {
                next_check = rels.len() as u64 + (need - res.cycles).max(1) as u64;
                continue;
            }
        } else {
            let mut scratch = Nat::default();
            let rows = build_rows(&rels, nver, tcol, r, &mut scratch);
            let (core, cc, _) = peel(&rows, nunk, tcol, &mut scratch);
            if core.len() < cc + margin(cc) {
                next_check = rels.len() as u64 + (nunk as u64 / 32).max(8);
                continue;
            }
        }

        // ── counted attempt ──
        res.attempts += 1;
        let mut g_nat = Nat::default();
        let rows = build_rows(&rels, nver, tcol, r, &mut g_nat);
        let mut p_nat = Nat::default();
        let (core, cc, ct) = peel(&rows, nunk, tcol, &mut p_nat);
        res.la_graph.merge(g_nat);
        res.la_peel.merge(p_nat);
        res.la.merge(g_nat);
        res.la.merge(p_nat);
        res.core_rows = core.len();
        res.core_cols = cc;
        if core.len() < cc + margin(cc) {
            next_check = rels.len() as u64 + (nunk as u64 / 20).max(8);
            continue;
        }
        let mut w_nat = Nat::default();
        let dense = res.attempts > 1;
        let sol = wiedemann(&core, cc, r, &mut rng, dense, &mut w_nat);
        res.la_wied.merge(w_nat);
        res.la.merge(w_nat);
        if let Some(x) = sol {
            let t = x[ct as usize];
            let v0 = ops.adds + ops.doubles;
            let chk = curve.mul(&mut ops, inst.g, t);
            res.verify_ops = ops.adds + ops.doubles - v0;
            let ok = chk == inst.target && curve.mul_rtl(inst.g, t) == inst.target;
            if ok {
                res.recovered = Some(t);
                res.certified = true;
                solved = true;
                continue;
            }
        }
        next_check = rels.len() as u64 + (nunk as u64 / 20).max(16);
    }
    res.la.add += dsu.ops;
    res.group = ops;
    res.walk_ops = (ops.adds + ops.doubles) - res.setup_ops - res.verify_ops;
    res.wall_ns = start.elapsed().as_nanos() as u64;
    res
}

// ── rho ─────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default)]
pub struct RhoOut {
    pub p: u64,
    pub recovered: Option<u64>,
    pub certified: bool,
    pub group: GroupOps,
    pub walk_ops: u64,
    pub setup_and_other_ops: u64,
    /// Canonicalisations (the walk's per-step native work).
    pub native: u64,
    pub steps: u64,
    pub wall_ns: u64,
}

impl RhoOut {
    pub fn group_ops(&self) -> u64 {
        self.group.adds + self.group.doubles
    }
    pub fn total(&self, w: f64) -> f64 {
        self.group_ops() as f64 + w * self.native as f64
    }
}

/// Negation-map rho on the order-`r` subgroup, `rho_reference_walk` with
/// `RhoWalk::negation()`; certified by an independent `[k]G` check.
pub fn run_rho(inst: &Instance, seed: u64) -> RhoOut {
    let max_steps = (60.0 * (inst.r as f64).sqrt()) as u64 + 10_000;
    let rr = rho_reference_walk(
        &inst.curve,
        inst.g,
        inst.target,
        inst.r,
        seed,
        max_steps,
        RhoWalk::negation(),
    );
    let certified = rr
        .recovered
        .map(|d| inst.curve.mul_rtl(inst.g, d) == inst.target)
        .unwrap_or(false);
    let walk = rr.counters.get("walk_operations").copied().unwrap_or(0);
    let total = rr.group_ops.adds + rr.group_ops.doubles;
    RhoOut {
        p: inst.p,
        recovered: rr.recovered,
        certified,
        group: rr.group_ops,
        walk_ops: walk,
        setup_and_other_ops: total.saturating_sub(walk),
        native: rr
            .counters
            .get("canonicalisations_uncharged")
            .copied()
            .unwrap_or(0)
            .max(rr.steps),
        steps: rr.steps,
        wall_ns: rr.wall_ns,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_instance(p: u64) -> Instance {
        for cs in 1..400u64 {
            if let Ok(i) = make_instance(p, cs, 7, 64) {
                if i.r > 1000 {
                    return i;
                }
            }
        }
        panic!("no instance at p = {p}");
    }

    #[test]
    fn mulmod_matches_u128() {
        let mut s = 12345u64;
        for &r in &[1_000_003u64, (1u64 << 45) + 59, (1u64 << 49) - 3] {
            for _ in 0..20000 {
                s = mix64(s);
                let a = s % r;
                s = mix64(s);
                let b = s % r;
                assert_eq!(
                    mulmod(a, b, r),
                    ((a as u128 * b as u128) % r as u128) as u64
                );
            }
        }
    }

    #[test]
    fn roots_and_smoothness() {
        let p = 10007u64;
        let mut rng = StdRng::seed_from_u64(3);
        let mut n = Nat::default();
        // (x−3)(x−100)(x−9000)
        let mut u: Poly = vec![1];
        for rt in [3u64, 100, 9000] {
            let mut nu = vec![0u64; u.len() + 1];
            for (i, &c) in u.iter().enumerate() {
                nu[i + 1] = (nu[i + 1] + c) % p;
                nu[i] = (nu[i] + c * (p - rt)) % p;
            }
            u = nu;
        }
        assert!(splits_completely(p, &u, &mut n));
        let mut got = roots_of(p, &u, &mut rng, &mut n).unwrap();
        got.sort_unstable();
        assert_eq!(got, vec![3, 100, 9000]);
        // x² + 1 has no root mod 10007 (10007 ≡ 3 mod 4)
        assert!(!splits_completely(p, &vec![1, 0, 1], &mut n));
    }

    #[test]
    fn order_is_exact_and_arms_certify() {
        let inst = small_instance(101);
        // #J kills random divisors, r·G = 0, [t]G = T
        let mut st = 99u64;
        for _ in 0..8 {
            let d = inst.curve.random_divisor(&mut st);
            assert!(inst.curve.mul_uncounted(d, inst.order).is_identity());
        }
        assert!(inst.curve.mul_uncounted(inst.g, inst.r).is_identity());
        assert_eq!(inst.curve.mul_rtl(inst.g, inst.t_secret), inst.target);
        for (arm, nb) in [(Arm::Full, 0), (Arm::SingleLp, 14), (Arm::DoubleLp, 9)] {
            let out = run_ic(
                &inst,
                IcParams {
                    arm,
                    nb,
                    seed: 11,
                    max_trials: 3_000_000,
                    verify_relations: true,
                },
            );
            assert_eq!(out.bad_relations, 0, "{arm:?} produced an invalid relation");
            assert!(out.certified, "{arm:?} did not certify: {out:?}");
            assert_eq!(out.recovered.unwrap() % inst.r, inst.t_secret % inst.r);
            if arm == Arm::DoubleLp {
                assert!(out.rels_by_lp[2] > 0, "no double-large-prime relation");
            }
            if arm == Arm::SingleLp {
                assert!(out.rels_by_lp[1] > 0, "no single-large-prime relation");
            }
        }
    }

    #[test]
    fn rho_matches_ic_on_a_small_instance() {
        let inst = small_instance(101);
        let rho = run_rho(&inst, 5);
        assert!(rho.certified);
        assert_eq!(rho.recovered.unwrap() % inst.r, inst.t_secret % inst.r);
        let ic = run_ic(
            &inst,
            IcParams {
                arm: Arm::DoubleLp,
                nb: 9,
                seed: 5,
                max_trials: 3_000_000,
                verify_relations: false,
            },
        );
        assert_eq!(ic.recovered, rho.recovered.map(|d| d % inst.r));
    }
}
