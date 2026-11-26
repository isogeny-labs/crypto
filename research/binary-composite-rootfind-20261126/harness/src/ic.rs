//! Index calculus with the subfield factor base `V = F_{2^{n'}}` ⊂ `F_{2^n}`.
//!
//! Arms (all share the factor base, the ordered relation-target stream,
//! the sign resolution / group-law replay of every relation, the linear
//! algebra and the final `[d]G = Q` check):
//!
//! * `F`  — m = 3 pairs-and-solve, last summand by the factored route
//!   `S₄ = S₃(u, x_R, X)·S₃(u', x_R, X)` (crypto
//!   `binary_radical_root::FactoredSubspaceOracle`, counted port) with the
//!   target-independent pair table of `u` (REPORT_1C "factored_b") and
//!   batched inversions per row.  `F0` is the same without the table
//!   (stage A recomputed per target), derived exactly from F's counts.
//! * `G`  — m = 3 pairs-and-solve, last summand by the subspace gcd
//!   `gcd(f, L_V mod f)` (crypto `SubspaceOracle`, counted port), batched
//!   leading-coefficient inversions per row.
//! * `M3` — m = 3 meet-in-the-middle: the same `u` table hashed, probed
//!   with `x(R ± P₃)` from one Artin–Schreier step per `P₃`.
//! * `M2` — m = 2 meet-in-the-middle: probe `x(R)` in the `u` table.
use crate::field::{batch_add, Curve, Ops, Pt, CF};
use crate::gen::powmod;
use crate::seeds::Sm64;
use std::collections::HashMap;

// ───────────────────────── subspace and factor base ─────────────────────────

pub struct Sub {
    pub basis: Vec<u64>,
    pub span: Vec<u64>,
    echelon: Vec<(u32, u64)>,
}

impl Sub {
    pub fn new(basis: &[u64]) -> Self {
        let l = basis.len();
        let mut span = Vec::with_capacity(1 << l);
        for idx in 0..(1u64 << l) {
            let mut v = 0;
            for (j, &e) in basis.iter().enumerate() {
                if (idx >> j) & 1 == 1 {
                    v ^= e;
                }
            }
            span.push(v);
        }
        let mut echelon: Vec<(u32, u64)> = vec![];
        for &v0 in basis {
            let mut v = v0;
            for &(p, e) in &echelon {
                if (v >> p) & 1 == 1 {
                    v ^= e;
                }
            }
            assert!(v != 0);
            let p = 63 - v.leading_zeros();
            for e in echelon.iter_mut() {
                if (e.1 >> p) & 1 == 1 {
                    e.1 ^= v;
                }
            }
            echelon.push((p, v));
            echelon.sort_unstable_by_key(|e| std::cmp::Reverse(e.0));
        }
        Sub { basis: basis.to_vec(), span, echelon }
    }
    #[inline(always)]
    pub fn contains(&self, cf: &CF, mut x: u64) -> bool {
        cf.lin();
        for &(p, e) in &self.echelon {
            if (x >> p) & 1 == 1 {
                x ^= e;
            }
        }
        x == 0
    }
}

pub struct Fb {
    /// Abscissae of rational points in V (0 = the 2-torsion point T).
    pub xs: Vec<u64>,
    pub pts: Vec<Pt>,
    /// x → index into xs.
    pub idx: HashMap<u64, usize>,
}

/// Factor base: every `x ∈ V` that lifts, one point per `±` class.
pub fn build_fb(c: &Curve, cf: &CF, sub: &Sub) -> Fb {
    let mut x2s: Vec<u64> = sub.span.iter().map(|&x| if x == 0 { 0 } else { cf.sqr(x) }).collect();
    let mut tmp = vec![];
    cf.batch_inv(&mut x2s, &mut tmp);
    let mut fb = Fb { xs: vec![], pts: vec![], idx: HashMap::new() };
    for (k, &x) in sub.span.iter().enumerate() {
        let p = if x == 0 { Some(c.t2()) } else { c.lift_with(cf, x, x2s[k]) };
        if let Some(p) = p {
            let (p, _) = c.canon(p);
            debug_assert!(c.on_curve(p));
            fb.idx.insert(x, fb.xs.len());
            fb.xs.push(x);
            fb.pts.push(p);
        }
    }
    fb
}

// ───────────────────────── Artin–Schreier quadratic ─────────────────────────

/// Roots of `A·X² + B·X + C` (`A·B ≠ 0`, `ab_inv = 1/(A·B)`), one AS step
/// (crypto `binary_radical_root::as_roots`, counted).
#[inline(always)]
fn as_roots(c: &Curve, cf: &CF, a: u64, b: u64, cc: u64, ab_inv: u64) -> Option<[u64; 2]> {
    let inv_a = cf.mul(b, ab_inv);
    let inv_b = cf.mul(a, ab_inv);
    let w = cf.mul(cf.mul(a, cc), cf.sqr(inv_b));
    cf.lin();
    if c.asv.trace(w) != 0 {
        return None;
    }
    let k = cf.mul(b, inv_a);
    cf.lin();
    let x0 = cf.mul(k, c.asv.solve(w));
    Some([x0, x0 ^ k])
}

// ───────────────────────── the quartic S₄(x1, x2, X, r) ─────────────────────

/// Coefficients `[c0..c4]` of `S₄(x1, x2, X, r)` in `X` for the curve
/// constant `b`, as `Res_U(S₃(x1,x2,U), S₃(r,X,U))` with
/// `S₃(x1,x2,x3) = x1²x2² + x1²x3² + x2²x3² + x1x2x3 + b`.
/// Precomputed per target: `r2 = r², r4 = r⁴`.  13 m + 5 s.
#[inline(always)]
pub fn quartic(cf: &CF, b: u64, x1: u64, x2: u64, r: u64, r2: u64, r4: u64) -> [u64; 5] {
    let aa = cf.sqr(x1 ^ x2);
    let bb = cf.mul(x1, x2);
    let cc = cf.sqr(bb) ^ b;
    let p = cf.mul(aa, r2) ^ cc;
    let q0 = cf.mul(aa, b) ^ cf.mul(cc, r2);
    let b2 = cf.sqr(bb);
    let b2r2 = cf.mul(b2, r2);
    let br = cf.mul(bb, r);
    let c4 = cf.sqr(p) ^ b2r2;
    let c3 = cf.mul(br, p);
    let c2 = cf.mul(b2, b) ^ cf.mul(cf.mul(aa, cc), r2) ^ cf.mul(b2, r4);
    let c1 = cf.mul(br, q0);
    let c0 = cf.sqr(q0) ^ cf.mul(b2r2, b);
    [c0, c1, c2, c3, c4]
}

#[inline(always)]
fn horner(cf: &CF, q: &[u64], t: u64) -> u64 {
    let mut v = 0;
    for &c in q.iter().rev() {
        v = cf.mul(v, t) ^ c;
    }
    v
}

// ───────────────────────── the pair table (stage A) ─────────────────────────

/// Per row `i` (x1 = fb.xs[i]): `(j, u)` for every `u = x(P_i ± P_j)`, `j ≥ i`,
/// plus the pairs whose `u` is irrational (never for rational P_i, P_j;
/// kept for the guard) and a flag for the pair `(T, T)`.
pub struct PairTable {
    pub rows: Vec<Vec<(u32, u64)>>,
    pub irr: Vec<Vec<u32>>,
    pub nu: u64,
}

pub fn pair_table(c: &Curve, cf: &CF, fb: &Fb) -> PairTable {
    let f = fb.xs.len();
    let mut rows = Vec::with_capacity(f);
    let mut irr = Vec::with_capacity(f);
    let (mut inv, mut tmp) = (vec![], vec![]);
    let mut nu = 0;
    let mut ab: Vec<(u64, u64)> = vec![];
    for i in 0..f {
        let x1 = fb.xs[i];
        inv.clear();
        ab.clear();
        for j in i..f {
            let x2 = fb.xs[j];
            let a1 = cf.sqr(x1 ^ x2);
            let b1 = cf.mul(x1, x2);
            ab.push((a1, b1));
            inv.push(if a1 == 0 || b1 == 0 { 0 } else { cf.mul(a1, b1) });
        }
        cf.batch_inv(&mut inv, &mut tmp);
        let mut us = vec![];
        let mut ir = vec![];
        for j in i..f {
            let x2 = fb.xs[j];
            let (a1, b1) = ab[j - i];
            if a1 == 0 {
                // x1 = x2: P − P = O (no u); P + P has u = (x⁴ + b)/x².
                if x1 != 0 {
                    let x2s = cf.sqr(x1);
                    let u = cf.mul(cf.sqr(x2s) ^ c.b, cf.inv(x2s));
                    us.push((j as u32, u));
                }
                continue;
            }
            if b1 == 0 {
                // one of them is T: x(T ± P) = √b / x.
                us.push((j as u32, cf.mul(c.sqrt_b, cf.inv(x1 ^ x2))));
                continue;
            }
            let c1 = cf.sqr(b1) ^ c.b;
            match as_roots(c, cf, a1, b1, c1, inv[j - i]) {
                Some([u0, u1]) => {
                    us.push((j as u32, u0));
                    us.push((j as u32, u1));
                }
                None => ir.push(j as u32),
            }
        }
        nu += us.len() as u64;
        rows.push(us);
        irr.push(ir);
    }
    PairTable { rows, irr, nu }
}

// ───────────────────────── stage B (factored), per target ───────────────────

/// All `{x1, x2, x3}` (sorted, deduplicated) with `x_i` in the factor base
/// and `S₄(x1, x2, x3, x_R) = 0`, by the factored route on the pair table.
pub fn solve_factored(c: &Curve, cf: &CF, sub: &Sub, fb: &Fb, pt: &PairTable, xr: u64, out: &mut Vec<[u64; 3]>) {
    out.clear();
    let (mut inv, mut tmp) = (vec![], vec![]);
    let mut ab: Vec<(u64, u64)> = vec![];
    let r2 = cf.sqr(xr);
    let r4 = cf.sqr(r2);
    let accept = |x1: u64, x2: u64, x: u64, out: &mut Vec<[u64; 3]>| {
        if sub.contains(cf, x) && fb.idx.contains_key(&x) {
            let q = quartic(cf, c.b, x1, x2, xr, r2, r4);
            if horner(cf, &q, x) == 0 {
                let mut t = [x1, x2, x];
                t.sort_unstable();
                out.push(t);
            }
        }
    };
    for (i, us) in pt.rows.iter().enumerate() {
        let x1 = fb.xs[i];
        inv.clear();
        ab.clear();
        for &(_, u) in us {
            let a2 = cf.sqr(u ^ xr);
            let b2 = cf.mul(u, xr);
            ab.push((a2, b2));
            inv.push(if a2 == 0 || b2 == 0 { 0 } else { cf.mul(a2, b2) });
        }
        cf.batch_inv(&mut inv, &mut tmp);
        for (k, &(j, u)) in us.iter().enumerate() {
            let x2 = fb.xs[j as usize];
            let (a2, b2) = ab[k];
            if a2 == 0 {
                if u != 0 {
                    let u2 = cf.sqr(u);
                    let x = cf.mul(cf.sqr(u2) ^ c.b, cf.inv(u2));
                    accept(x1, x2, x, out);
                }
            } else if b2 == 0 {
                let x = cf.mul(c.sqrt_b, cf.inv(u ^ xr));
                accept(x1, x2, x, out);
            } else {
                let c2 = cf.sqr(b2) ^ c.b;
                if let Some(xs) = as_roots(c, cf, a2, b2, c2, inv[k]) {
                    accept(x1, x2, xs[0], out);
                    accept(x1, x2, xs[1], out);
                }
            }
        }
        // P_i − P_i = O: then P3 = ±R, X = x_R.
        accept(x1, x1, xr, out);
    }
    // Guard (irrational u, never hit with a rational base): by evaluation.
    for (i, ir) in pt.irr.iter().enumerate() {
        for &j in ir {
            let (x1, x2) = (fb.xs[i], fb.xs[j as usize]);
            let gx = xr == x1 || xr == x2 || cf.mul(xr, x1) == c.sqrt_b || cf.mul(xr, x2) == c.sqrt_b;
            if gx {
                let q = quartic(cf, c.b, x1, x2, xr, r2, r4);
                for &t in &fb.xs {
                    if horner(cf, &q, t) == 0 {
                        let mut tt = [x1, x2, t];
                        tt.sort_unstable();
                        out.push(tt);
                    }
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
}

// ───────────────────────── the subspace gcd, per target ─────────────────────

/// Counted `subspace_poly_for_basis`.
pub fn subspace_poly(cf: &CF, basis: &[u64]) -> Vec<u64> {
    let mut a = vec![1u64];
    for &v in basis {
        let mut lv = 0;
        for (j, &aj) in a.iter().enumerate() {
            lv ^= cf.mul(aj, cf.sqr_k(v, j as u32));
        }
        assert!(lv != 0);
        let mut next = vec![0u64; a.len() + 1];
        for (j, &aj) in a.iter().enumerate() {
            next[j + 1] ^= cf.sqr(aj);
            next[j] ^= cf.mul(lv, aj);
        }
        a = next;
    }
    a
}

const MAXD: usize = 7;
type P7 = [u64; MAXD];
fn pdeg(p: &P7) -> Option<usize> {
    (0..MAXD).rev().find(|&i| p[i] != 0)
}
fn rem_monic(cf: &CF, mut r: P7, f: &P7, df: usize) -> P7 {
    while let Some(dd) = pdeg(&r) {
        if dd < df {
            break;
        }
        let lead = r[dd];
        let sh = dd - df;
        for j in 0..df {
            r[sh + j] ^= cf.mul(f[j], lead);
        }
        r[dd] = 0;
    }
    r
}
fn sqr_mod_monic(cf: &CF, a: &P7, f: &P7, df: usize) -> P7 {
    let mut s = [0u64; MAXD];
    for i in 0..=3 {
        if a[i] != 0 {
            s[2 * i] = cf.sqr(a[i]);
        }
    }
    rem_monic(cf, s, f, df)
}
fn prem(cf: &CF, a: &P7, b: &P7, db: usize) -> P7 {
    let lb = b[db];
    let mut r = *a;
    while let Some(dd) = pdeg(&r) {
        if dd < db {
            break;
        }
        let lead = r[dd];
        let sh = dd - db;
        for j in 0..MAXD {
            if r[j] != 0 {
                r[j] = cf.mul(lb, r[j]);
            }
        }
        for j in 0..=db {
            r[sh + j] ^= cf.mul(lead, b[j]);
        }
        r[dd] = 0;
    }
    r
}
fn poly_gcd(cf: &CF, mut a: P7, mut b: P7) -> P7 {
    loop {
        let db = match pdeg(&b) {
            None => return a,
            Some(d) => d,
        };
        if db == 0 {
            return [0; MAXD];
        }
        let r = prem(cf, &a, &b, db);
        a = b;
        b = r;
    }
}

/// Same contract as [`solve_factored`], by the subspace gcd on every pair.
pub fn solve_gcd(c: &Curve, cf: &CF, fb: &Fb, lv: &[u64], xr: u64, out: &mut Vec<[u64; 3]>) {
    out.clear();
    let f = fb.xs.len();
    let r2 = cf.sqr(xr);
    let r4 = cf.sqr(r2);
    let (mut leads, mut tmp) = (vec![], vec![]);
    let mut qs: Vec<[u64; 5]> = vec![];
    for i in 0..f {
        let x1 = fb.xs[i];
        qs.clear();
        leads.clear();
        for j in i..f {
            let q = quartic(cf, c.b, x1, fb.xs[j], xr, r2, r4);
            let d = (0..5).rev().find(|&k| q[k] != 0);
            leads.push(d.map(|d| q[d]).unwrap_or(0));
            qs.push(q);
        }
        cf.batch_inv(&mut leads, &mut tmp);
        for (k, q) in qs.iter().enumerate() {
            let x2 = fb.xs[i + k];
            let Some(df) = (0..5).rev().find(|&t| q[t] != 0) else {
                // zero quartic: every X is a root (never for x_R ≠ 0).
                for &t in &fb.xs {
                    let mut tt = [x1, x2, t];
                    tt.sort_unstable();
                    out.push(tt);
                }
                continue;
            };
            if df == 0 {
                continue;
            }
            let mut fm = [0u64; MAXD];
            for t in 0..df {
                fm[t] = cf.mul(q[t], leads[k]);
            }
            fm[df] = 1;
            // L_V(X) mod f
            let mut pow = [0u64; MAXD];
            pow[1] = 1;
            let mut pw = rem_monic(cf, pow, &fm, df);
            let mut acc = [0u64; MAXD];
            for (t, &at) in lv.iter().enumerate() {
                if at != 0 {
                    for j in 0..df {
                        if pw[j] != 0 {
                            acc[j] ^= cf.mul(pw[j], at);
                        }
                    }
                }
                if t + 1 < lv.len() {
                    pw = sqr_mod_monic(cf, &pw, &fm, df);
                }
            }
            let g = poly_gcd(cf, fm, acc);
            match pdeg(&g) {
                None | Some(0) => {}
                Some(1) => {
                    let x = cf.mul(g[0], cf.inv(g[1]));
                    if fb.idx.contains_key(&x) {
                        let mut tt = [x1, x2, x];
                        tt.sort_unstable();
                        out.push(tt);
                    }
                }
                Some(dg) => {
                    for &t in &fb.xs {
                        if horner(cf, &g[..=dg], t) == 0 {
                            let mut tt = [x1, x2, t];
                            tt.sort_unstable();
                            out.push(tt);
                        }
                    }
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
}

// ───────────────────────── hash table of u (MITM) ───────────────────────────

pub struct UHash {
    pub map: HashMap<u64, Vec<(u32, u32)>, BuildFix>,
}

#[derive(Clone, Default)]
pub struct BuildFix;
impl std::hash::BuildHasher for BuildFix {
    type Hasher = FixHasher;
    fn build_hasher(&self) -> FixHasher {
        FixHasher(0)
    }
}
pub struct FixHasher(u64);
impl std::hash::Hasher for FixHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0 ^ b as u64).wrapping_mul(0x100_0000_01b3);
        }
    }
    fn write_u64(&mut self, x: u64) {
        self.0 = (x ^ (x >> 29)).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    }
}

pub fn uhash(cf: &CF, pt: &PairTable) -> UHash {
    let mut map: HashMap<u64, Vec<(u32, u32)>, BuildFix> = HashMap::with_hasher(BuildFix);
    for (i, us) in pt.rows.iter().enumerate() {
        for &(j, u) in us {
            cf.hash();
            map.entry(u).or_default().push((i as u32, j));
        }
    }
    UHash { map }
}

/// M3 per target: for each `P₃`, `x(R ± P₃)` are the roots of
/// `S₃(x₃, x_R, U)`; probe both in the table.
pub fn solve_m3(c: &Curve, cf: &CF, fb: &Fb, uh: &UHash, xr: u64, out: &mut Vec<[u64; 3]>) {
    out.clear();
    let f = fb.xs.len();
    let (mut inv, mut tmp) = (Vec::with_capacity(f), vec![]);
    let mut ab = Vec::with_capacity(f);
    for k in 0..f {
        let x3 = fb.xs[k];
        let a = cf.sqr(x3 ^ xr);
        let b = cf.mul(x3, xr);
        ab.push((a, b));
        inv.push(if a == 0 || b == 0 { 0 } else { cf.mul(a, b) });
    }
    cf.batch_inv(&mut inv, &mut tmp);
    let probe = |u: u64, x3: u64, out: &mut Vec<[u64; 3]>| {
        cf.hash();
        if let Some(v) = uh.map.get(&u) {
            for &(i, j) in v {
                let mut t = [fb.xs[i as usize], fb.xs[j as usize], x3];
                t.sort_unstable();
                out.push(t);
            }
        }
    };
    for k in 0..f {
        let x3 = fb.xs[k];
        let (a, b) = ab[k];
        if a == 0 {
            // x3 = x_R: R − P3 = O (then P1 + P2 = O) or R + P3 = 2P3.
            let x2s = cf.sqr(x3);
            if x3 != 0 {
                let u = cf.mul(cf.sqr(x2s) ^ c.b, cf.inv(x2s));
                probe(u, x3, out);
            }
            continue;
        }
        if b == 0 {
            let u = cf.mul(c.sqrt_b, cf.inv(x3 ^ xr));
            probe(u, x3, out);
            continue;
        }
        let cc = cf.sqr(b) ^ c.b;
        if let Some([u0, u1]) = as_roots(c, cf, a, b, cc, inv[k]) {
            probe(u0, x3, out);
            probe(u1, x3, out);
        }
    }
    out.sort_unstable();
    out.dedup();
}

/// M2 per target: probe `x(R)`.
pub fn solve_m2(cf: &CF, fb: &Fb, uh: &UHash, xr: u64, out: &mut Vec<[u64; 2]>) {
    out.clear();
    cf.hash();
    if let Some(v) = uh.map.get(&xr) {
        for &(i, j) in v {
            out.push([fb.xs[i as usize], fb.xs[j as usize]]);
        }
    }
    out.sort_unstable();
    out.dedup();
}

// ───────────────────────── relation-target stream ───────────────────────────

/// `R_{k+1} = R_k + J_{h(x(R_k))}` on `W` lanes, batched; `R = [a]G + [b]Q`.
pub struct Targets {
    /// Amendment A1: arithmetic-progression lanes `R ← R + S` (never cycle
    /// before ℓ steps) instead of the frozen r-adding walk.
    ap: Option<(Pt, u64, u64)>,
    lanes: Vec<(Pt, u64, u64)>,
    jumps: Vec<(Pt, u64, u64)>,
    buf: std::collections::VecDeque<(Pt, u64, u64)>,
    scr: (Vec<u64>, Vec<u64>),
    pub produced: u64,
}

pub const TGT_LANES: usize = 8;
pub const TGT_JUMPS: usize = 16;

fn jidx(x: u64, r: usize) -> usize {
    ((x.wrapping_mul(0xC2B2_AE3D_27D4_EB4F) >> 40) as usize) % r
}

impl Targets {
    pub fn new(c: &Curve, cf: &CF, g: Pt, q: Pt, rng: &mut Sm64) -> Self {
        let ell = c.ell;
        let (a0, b0) = (rng.below(ell), rng.below(ell));
        let (as_, bs) = (rng.below(ell), rng.below(ell));
        let start = c.joint(cf, a0, g, b0, q);
        let stride = c.joint(cf, as_, g, bs, q);
        let mut lanes = vec![(start, a0, b0)];
        for k in 1..TGT_LANES {
            let (p, a, b) = lanes[k - 1];
            lanes.push((c.add(cf, p, stride), (a + as_) % ell, (b + bs) % ell));
        }
        let mut jumps = vec![];
        for _ in 0..TGT_JUMPS {
            let (a, b) = (rng.bits(12) % ell, rng.bits(12) % ell);
            jumps.push((c.joint(cf, a, g, b, q), a, b));
        }
        Targets { ap: None, lanes, jumps, buf: Default::default(), scr: (vec![], vec![]), produced: 0 }
    }
    /// Amendment A1 stream: 8 lanes with independent random starts
    /// `[a_k]G + [b_k]Q` and one common random stride `S = [a_s]G + [b_s]Q`.
    pub fn new_ap(c: &Curve, cf: &CF, g: Pt, q: Pt, rng: &mut Sm64) -> Self {
        let ell = c.ell;
        let (as_, bs) = (rng.below(ell), rng.below(ell));
        let stride = c.joint(cf, as_, g, bs, q);
        let mut lanes = vec![];
        for _ in 0..TGT_LANES {
            let (a, b) = (rng.below(ell), rng.below(ell));
            lanes.push((c.joint(cf, a, g, b, q), a, b));
        }
        Targets { ap: Some((stride, as_, bs)), lanes, jumps: vec![], buf: Default::default(), scr: (vec![], vec![]), produced: 0 }
    }
    pub fn next(&mut self, c: &Curve, cf: &CF) -> (Pt, u64, u64) {
        if self.buf.is_empty() {
            for &l in &self.lanes {
                self.buf.push_back(l);
            }
            let ps: Vec<Pt> = self.lanes.iter().map(|l| l.0).collect();
            if let Some((st, sa, sb)) = self.ap {
                let qs: Vec<Pt> = vec![st; ps.len()];
                let mut out = vec![];
                batch_add(c, cf, &ps, &qs, &mut out, &mut self.scr);
                for k in 0..self.lanes.len() {
                    let (_, a, b) = self.lanes[k];
                    self.lanes[k] = (out[k], (a + sa) % c.ell, (b + sb) % c.ell);
                }
                self.produced += 1;
                return self.buf.pop_front().unwrap();
            }
            let js: Vec<usize> = ps
                .iter()
                .map(|p| match p {
                    Pt::A(x, _) => jidx(*x, TGT_JUMPS),
                    Pt::Inf => 0,
                })
                .collect();
            let qs: Vec<Pt> = js.iter().map(|&j| self.jumps[j].0).collect();
            let mut out = vec![];
            batch_add(c, cf, &ps, &qs, &mut out, &mut self.scr);
            for k in 0..self.lanes.len() {
                let (_, a, b) = self.lanes[k];
                let j = &self.jumps[js[k]];
                self.lanes[k] = (out[k], (a + j.1) % c.ell, (b + j.2) % c.ell);
            }
        }
        self.produced += 1;
        self.buf.pop_front().unwrap()
    }
}

// ───────────────────────── relations ────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Row {
    pub cols: Vec<(usize, i64)>,
    pub a: u64,
    pub b: u64,
}

/// Signs `s, s₂, s₃` with `P₁ + s₂P₂ (+ s₃P₃) = s·R`, by the group law
/// (this is the relation's replay check).  `None` if no sign works.
pub fn resolve(c: &Curve, cf: &CF, fb: &Fb, xs: &[u64], r: Pt, ra: u64, rb: u64) -> Option<Row> {
    let pts: Vec<Pt> = xs.iter().map(|x| fb.pts[fb.idx[x]]).collect();
    let nr = c.neg(r);
    let mut found = None;
    if xs.len() == 2 {
        for s2 in [1i64, -1] {
            let p2 = if s2 == 1 { pts[1] } else { c.neg(pts[1]) };
            let sum = c.add(cf, pts[0], p2);
            if sum == r {
                found = Some((1i64, vec![1, s2]));
                break;
            }
            if sum == nr {
                found = Some((-1, vec![1, s2]));
                break;
            }
        }
    } else {
        'o: for s2 in [1i64, -1] {
            let p2 = if s2 == 1 { pts[1] } else { c.neg(pts[1]) };
            let u = c.add(cf, pts[0], p2);
            for s3 in [1i64, -1] {
                let p3 = if s3 == 1 { pts[2] } else { c.neg(pts[2]) };
                let sum = c.add(cf, u, p3);
                if sum == r {
                    found = Some((1, vec![1, s2, s3]));
                    break 'o;
                }
                if sum == nr {
                    found = Some((-1, vec![1, s2, s3]));
                    break 'o;
                }
            }
        }
    }
    let (s, sg) = found?;
    let ell = c.ell;
    let mut cols: Vec<(usize, i64)> = vec![];
    for (k, x) in xs.iter().enumerate() {
        if *x == 0 {
            continue; // T: projects to O in the ℓ-part.
        }
        let col = fb.idx[x];
        if let Some(e) = cols.iter_mut().find(|e| e.0 == col) {
            e.1 += sg[k];
        } else {
            cols.push((col, sg[k]));
        }
    }
    cols.retain(|e| e.1 != 0);
    let (a, b) = if s == 1 { (ra, rb) } else { ((ell - ra) % ell, (ell - rb) % ell) };
    Some(Row { cols, a, b })
}

// ───────────────────────── linear algebra (Wiedemann mod ℓ) ────────────────

fn inv_mod(cf: &CF, a: u64, m: u64) -> u64 {
    cf.zmul(2 * (64 - m.leading_zeros()) as u64);
    powmod(a, m - 2, m)
}

/// A non-zero `c` with `Σ c_j row_j = 0` (over the columns), or `None`.
/// Wiedemann on the square `N × N` matrix `Mᵀ` padded with zero rows.
fn kernel(cf: &CF, rows: &[Row], ncols: usize, ell: u64, rng: &mut Sm64) -> Option<Vec<u64>> {
    let n = rows.len().max(ncols);
    let mm = |a: u64, b: u64| ((a as u128 * b as u128) % ell as u128) as u64;
    // B v : out[col] = Σ_j coeff_{j,col} v[j]
    let matvec = |v: &[u64]| {
        let mut out = vec![0u64; n];
        for (j, r) in rows.iter().enumerate() {
            if v[j] == 0 {
                continue;
            }
            for &(col, e) in &r.cols {
                cf.zadd(1);
                let t = if e == 1 {
                    v[j]
                } else if e == -1 {
                    (ell - v[j]) % ell
                } else {
                    cf.zmul(1);
                    let ee = ((e % ell as i64) + ell as i64) as u64 % ell;
                    mm(ee, v[j])
                };
                out[col] = (out[col] + t) % ell;
            }
        }
        out
    };
    let u: Vec<u64> = (0..n).map(|_| rng.below(ell)).collect();
    let v: Vec<u64> = (0..n).map(|_| rng.below(ell)).collect();
    let dot = |x: &[u64]| {
        cf.zmul(n as u64);
        cf.zadd(n as u64);
        x.iter().zip(&u).fold(0u64, |acc, (&a, &b)| (acc + mm(a, b)) % ell)
    };
    let len = 2 * n + 2;
    let mut seq = Vec::with_capacity(len);
    let mut w = v.clone();
    for _ in 0..len {
        seq.push(dot(&w));
        w = matvec(&w);
    }
    // Berlekamp–Massey mod ℓ.
    let mut cpoly = vec![1u64];
    let mut bpoly = vec![1u64];
    let (mut l, mut mshift, mut bd) = (0usize, 1usize, 1u64);
    for i in 0..len {
        let mut d = seq[i];
        for k in 1..=l.min(cpoly.len() - 1) {
            d = (d + mm(cpoly[k], seq[i - k])) % ell;
        }
        cf.zmul(l as u64);
        if d == 0 {
            mshift += 1;
            continue;
        }
        let coef = mm(d, inv_mod(cf, bd, ell));
        let t = cpoly.clone();
        if cpoly.len() < bpoly.len() + mshift {
            cpoly.resize(bpoly.len() + mshift, 0);
        }
        for (k, &bk) in bpoly.iter().enumerate() {
            cpoly[k + mshift] = (cpoly[k + mshift] + ell - mm(coef, bk)) % ell;
        }
        cf.zmul(bpoly.len() as u64);
        if 2 * l <= i {
            l = i + 1 - l;
            bpoly = t;
            bd = d;
            mshift = 1;
        } else {
            mshift += 1;
        }
    }
    cpoly.resize(l + 1, 0);
    let kstar = (0..=l).rev().find(|&k| cpoly[k] != 0)?;
    let e = l - kstar;
    if e == 0 {
        return None;
    }
    // y = Σ_{k=0}^{k*} c_k B^{k*−k} v  (Horner)
    let mut y: Vec<u64> = v.iter().map(|&x| mm(cpoly[0], x)).collect();
    cf.zmul(n as u64);
    for k in 1..=kstar {
        let by = matvec(&y);
        cf.zmul(n as u64);
        cf.zadd(n as u64);
        y = by.iter().zip(&v).map(|(&a, &b)| (a + mm(cpoly[k], b)) % ell).collect();
    }
    for _ in 0..=e + 1 {
        if y.iter().all(|&t| t == 0) {
            return None;
        }
        let z = matvec(&y);
        if z.iter().all(|&t| t == 0) {
            y.truncate(rows.len());
            return Some(y);
        }
        y = z;
    }
    None
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LaStats {
    pub calls: u32,
    pub attempts: u32,
}

/// The logarithm from the relations: `d = −Σ c_j a_j / Σ c_j b_j`.
pub fn solve_log(cf: &CF, rows: &[Row], ell: u64, rng: &mut Sm64, st: &mut LaStats) -> Option<u64> {
    st.calls += 1;
    // compress columns
    let mut map: HashMap<usize, usize> = HashMap::new();
    let rr: Vec<Row> = rows
        .iter()
        .map(|r| Row {
            cols: r
                .cols
                .iter()
                .map(|&(c, e)| {
                    let k = map.len();
                    (*map.entry(c).or_insert(k), e)
                })
                .collect(),
            a: r.a,
            b: r.b,
        })
        .collect();
    let ncols = map.len();
    let mm = |a: u64, b: u64| ((a as u128 * b as u128) % ell as u128) as u64;
    for _ in 0..4 {
        st.attempts += 1;
        let Some(c) = kernel(cf, &rr, ncols, ell, rng) else { continue };
        // check the dependency exactly
        let mut acc = vec![0i128; ncols];
        for (j, r) in rr.iter().enumerate() {
            for &(col, e) in &r.cols {
                acc[col] = (acc[col] + e as i128 * c[j] as i128).rem_euclid(ell as i128);
            }
        }
        cf.zadd(rr.iter().map(|r| r.cols.len() as u64).sum());
        if acc.iter().any(|&t| t != 0) {
            continue;
        }
        let (mut num, mut den) = (0u64, 0u64);
        for (j, r) in rr.iter().enumerate() {
            num = (num + mm(c[j], r.a)) % ell;
            den = (den + mm(c[j], r.b)) % ell;
        }
        cf.zmul(2 * rr.len() as u64);
        if den == 0 {
            continue;
        }
        let d = mm((ell - num) % ell, inv_mod(cf, den, ell));
        return Some(d);
    }
    None
}

// ───────────────────────── one IC solve ─────────────────────────────────────

pub struct IcOut {
    pub arm: &'static str,
    pub d: Option<u64>,
    pub verified: bool,
    pub targets: u64,
    pub relations: usize,
    pub columns: usize,
    pub fb: usize,
    pub rows_unresolved: u64,
    pub la: LaStats,
    pub phases: Vec<(&'static str, Ops)>,
    pub wall_ns: u128,
    pub extra: serde_json::Value,
}

pub fn ncols_of(rows: &[Row]) -> usize {
    let mut s: std::collections::HashSet<usize> = Default::default();
    for r in rows {
        for c in &r.cols {
            s.insert(c.0);
        }
    }
    s.len()
}
