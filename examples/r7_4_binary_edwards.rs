//! Lead R7.4, item 2: a binary-Edwards check of Corollary 9.2 (R5.4).
//!
//! Corollary 9.2 says 2-torsion symmetrisation of a Weierstrass x-frame is
//! null in characteristic 2.  This example tests the analogous statements on
//! toy binary Edwards curves (Bernstein-Lange-Farashi Kashani)
//!
//! ```text
//!   d1 (x + y) + d2 (x^2 + y^2) = xy + xy (x + y) + x^2 y^2,   Tr(d2) = 1,
//! ```
//!
//! over F_{2^n}, n = 9 .. 12, with the complete affine addition law, neutral
//! element (0, 0) and negation (x, y) -> (y, x).
//!
//! Exact structural checks (`structure` block of the output, every curve):
//!
//! * the Edwards group is cyclic of order N, `-(x, y) = (y, x)` is index
//!   negation, the birational map to y^2 + xy = x^3 + (d1^2 + d2) x^2 +
//!   d1^4 (d1^2 + d1 + d2)^2 is a group homomorphism (all pairs, or a large
//!   sample), and the torsion points (1, 1) (order 2) and the order-4 points
//!   are listed;
//! * the fibres of w = x + y are exactly the orbits of <T, -1> with T = (1, 1)
//!   (so the Edwards w-frame is the T-symmetrised frame by construction);
//! * with b = d1^4 (d1^2 + d1 + d2) and x_W the Weierstrass abscissa:
//!   x_W(P + T) = sqrt(b) / x_W(P), x_W(2P) = u(P)^2 for u = x_W + sqrt(b) /
//!   x_W, and for u4 = u + b^(1/4) / u: u4 is invariant under translation by
//!   an order-4 point Q4, its fibres are the orbits of <Q4, -1>, and
//!   x_W(4P) = u4(P)^4 (the Z/4 analogue of Theorem 9).
//!
//! Pair-table (mitm) oracle, exact over every target of the prime-order
//! subgroup G (r = largest prime factor of N, r^2 not dividing N):
//!
//! * arms at equal base size |F|, every base a union of orbits of <M, -1>
//!   (negation-closed, like every x-frame base): `plain` (M = 1), `s2`
//!   (M = <T>, the fibres of w), `s4` (M = <Q4>, only when 4 | N), each swept
//!   over all of F (`full`) or over one point per M-coset (`quot`, the
//!   quotient oracle). F0 (a multiple of 8) is calibrated per curve so that
//!   the plain arm's decomposable fraction is near a target H in
//!   {0.05, 0.1, 0.25, 0.5};
//! * counted unit: table lookups W = |sweep| per target; U = number of
//!   distinct signed columns +-[h]P (nonzero), optionally + 1;
//! * four yields (`METRICS`): lambda = mean number of distinct relations
//!   modulo M per target (the primary, linear in the hit rate) or H = the
//!   fraction of targets with at least one relation, each divided by U W;
//!   the primary is lambda/c with U = c;
//! * the ratio is yield(arm) / yield(plain) with a bootstrap over bases
//!   (stratified by curve in pooled rows).
//!
//! The law for m = 3 (pair table, k = 2) predicts a ratio 1/mu for the full
//! sweep and 1 for the quotient sweep, in the small-hit-rate limit and for
//! many columns; U = c and U = c + 1 bracket the finite-size effect.
//! Curves with n = 13, 14 are a scale check beyond the n <= 12 scope.
//!
//! Standalone (no library code), single-threaded, JSON on stdout.
//!
//! ```sh
//! cargo run --release --example r7_4_binary_edwards -- 1000 10 > binary_edwards.json
//! ```
//! Arguments: bases per arm per cell (default 300), curves per n (default 6).

use std::collections::HashMap;

// ---------------------------------------------------------------- utilities

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut d = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            out.push(d);
            while n.is_multiple_of(d) {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        out.push(n);
    }
    out
}

// ---------------------------------------------------------------- GF(2^n)

struct Gf {
    n: u32,
    q: usize,
    exp: Vec<u32>,
    log: Vec<u32>,
    zsq: Vec<u32>,  // z -> z^2 + z
    zinv: Vec<u32>, // c -> some z with z^2 + z = c, or u32::MAX
}

impl Gf {
    fn new(n: u32) -> Gf {
        let q = 1usize << n;
        for poly in ((1u64 << n) | 1..(1u64 << (n + 1))).step_by(2) {
            let mut exp = vec![0u32; 2 * (q - 1)];
            let mut v = 1u64;
            let mut ok = true;
            for (i, slot) in exp.iter_mut().enumerate().take(q - 1) {
                if i > 0 && v == 1 {
                    ok = false;
                    break;
                }
                *slot = v as u32;
                v <<= 1;
                if (v >> n) & 1 == 1 {
                    v ^= poly;
                }
            }
            if !ok || v != 1 {
                continue;
            }
            for i in 0..(q - 1) {
                exp[q - 1 + i] = exp[i];
            }
            let mut log = vec![0u32; q];
            for (i, &e) in exp.iter().enumerate().take(q - 1) {
                log[e as usize] = i as u32;
            }
            let mut f = Gf {
                n,
                q,
                exp,
                log,
                zsq: vec![0; q],
                zinv: vec![u32::MAX; q],
            };
            for z in 0..q as u32 {
                f.zsq[z as usize] = f.mul(z, z) ^ z;
            }
            for z in 0..q as u32 {
                let c = f.zsq[z as usize] as usize;
                if f.zinv[c] == u32::MAX {
                    f.zinv[c] = z;
                }
            }
            return f;
        }
        panic!("no primitive polynomial");
    }
    fn mul(&self, a: u32, b: u32) -> u32 {
        if a == 0 || b == 0 {
            0
        } else {
            self.exp[(self.log[a as usize] + self.log[b as usize]) as usize]
        }
    }
    fn sq(&self, a: u32) -> u32 {
        self.mul(a, a)
    }
    fn inv(&self, a: u32) -> u32 {
        assert!(a != 0, "inverse of zero");
        self.exp[(self.q - 1) - self.log[a as usize] as usize]
    }
    fn div(&self, a: u32, b: u32) -> u32 {
        self.mul(a, self.inv(b))
    }
    /// a^(1/2^k)
    fn root2k(&self, a: u32, k: u32) -> u32 {
        if a == 0 {
            return 0;
        }
        let m = (self.q - 1) as u64;
        // multiply the exponent by 2^{-k} mod (2^n - 1), 2^{-k} = 2^{n-k}
        let e = (self.log[a as usize] as u64 * (1u64 << (self.n - k))) % m;
        self.exp[e as usize]
    }
    fn tr(&self, a: u32) -> u32 {
        let mut t = a;
        let mut s = a;
        for _ in 1..self.n {
            s = self.sq(s);
            t ^= s;
        }
        t & 1
    }
    /// some z with z^2 + z = c (requires Tr(c) = 0)
    fn solve_zsq(&self, c: u32) -> Option<u32> {
        let z = self.zinv[c as usize];
        (z != u32::MAX).then_some(z)
    }
}

// ---------------------------------------------------------------- binary Edwards curve

type Pt = (u32, u32);

struct Ed<'a> {
    f: &'a Gf,
    d1: u32,
    d2: u32,
}

impl Ed<'_> {
    fn on_curve(&self, p: Pt) -> bool {
        let f = self.f;
        let (x, y) = p;
        let lhs = f.mul(self.d1, x ^ y) ^ f.mul(self.d2, f.sq(x) ^ f.sq(y));
        let xy = f.mul(x, y);
        let rhs = xy ^ f.mul(xy, x ^ y) ^ f.sq(xy);
        lhs == rhs
    }
    fn neg(&self, p: Pt) -> Pt {
        (p.1, p.0)
    }
    /// Complete affine addition law (BLF, Theorem 3.? in the paper); returns
    /// None if a denominator vanishes (never happens when Tr(d2) = 1).
    fn add(&self, p: Pt, q: Pt) -> Option<Pt> {
        let f = self.f;
        let (x1, y1) = p;
        let (x2, y2) = q;
        let a1 = x1 ^ f.sq(x1);
        let b1 = y1 ^ f.sq(y1);
        let s2 = x2 ^ y2;
        let s1 = x1 ^ y1;
        let den_x = self.d1 ^ f.mul(a1, s2);
        let den_y = self.d1 ^ f.mul(b1, s2);
        if den_x == 0 || den_y == 0 {
            return None;
        }
        let num_x = f.mul(self.d1, x1 ^ x2)
            ^ f.mul(self.d2, f.mul(s1, s2))
            ^ f.mul(a1, f.mul(x2, y1 ^ y2 ^ 1) ^ f.mul(y1, y2));
        let num_y = f.mul(self.d1, y1 ^ y2)
            ^ f.mul(self.d2, f.mul(s1, s2))
            ^ f.mul(b1, f.mul(y2, x1 ^ x2 ^ 1) ^ f.mul(x1, x2));
        Some((f.div(num_x, den_x), f.div(num_y, den_y)))
    }
    fn mul(&self, mut k: u64, p: Pt) -> Option<Pt> {
        let mut r: Pt = (0, 0);
        let mut q = p;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(r, q)?;
            }
            q = self.add(q, q)?;
            k >>= 1;
        }
        Some(r)
    }
    fn points(&self) -> Vec<Pt> {
        let f = self.f;
        let mut v = Vec::new();
        for x in 0..f.q as u32 {
            let a = self.d2 ^ x ^ f.sq(x);
            let b = self.d1 ^ x ^ f.sq(x);
            let c = f.mul(self.d1, x) ^ f.mul(self.d2, f.sq(x));
            if a == 0 {
                continue; // cannot happen when Tr(d2) = 1
            }
            if b == 0 {
                v.push((x, f.sqrt_div(c, a)));
            } else {
                let cc = f.div(f.mul(a, c), f.sq(b));
                if f.tr(cc) == 0 {
                    let z = f.solve_zsq(cc).unwrap();
                    let k = f.div(b, a);
                    v.push((x, f.mul(k, z)));
                    v.push((x, f.mul(k, z ^ 1)));
                }
            }
        }
        v
    }
}

impl Gf {
    fn sqrt_div(&self, c: u32, a: u32) -> u32 {
        self.root2k(self.div(c, a), 1)
    }
}

// ---------------------------------------------------------------- Weierstrass image

/// y^2 + xy = x^3 + a2 x^2 + b over F_{2^n}
struct Wei<'a> {
    f: &'a Gf,
    a2: u32,
    b: u32,
}

type WPt = Option<(u32, u32)>;

impl Wei<'_> {
    fn on_curve(&self, p: WPt) -> bool {
        match p {
            None => true,
            Some((x, y)) => {
                let f = self.f;
                f.sq(y) ^ f.mul(x, y) == f.mul(f.sq(x), x) ^ f.mul(self.a2, f.sq(x)) ^ self.b
            }
        }
    }
    fn add(&self, p: WPt, q: WPt) -> WPt {
        let f = self.f;
        match (p, q) {
            (None, _) => q,
            (_, None) => p,
            (Some((x1, y1)), Some((x2, y2))) => {
                if x1 == x2 {
                    if y1 ^ y2 == x1 || x1 == 0 {
                        return None;
                    }
                    let l = x1 ^ f.div(y1, x1);
                    let x3 = f.sq(l) ^ l ^ self.a2;
                    let y3 = f.sq(x1) ^ f.mul(l ^ 1, x3);
                    return Some((x3, y3));
                }
                let l = f.div(y1 ^ y2, x1 ^ x2);
                let x3 = f.sq(l) ^ l ^ x1 ^ x2 ^ self.a2;
                let y3 = f.mul(l, x1 ^ x3) ^ x3 ^ y1;
                Some((x3, y3))
            }
        }
    }
}

// ---------------------------------------------------------------- statistics

/// metric (num, den): num 0 = H, 2 = lambda; den 1 = U W with U = columns + 1,
/// den 3 = U W with U = columns
fn pooled_yield(cells: &[&[[f64; 4]]], idx: &[Vec<usize>], metric: (usize, usize)) -> f64 {
    let mut h = 0.0;
    let mut uw = 0.0;
    for (c, ix) in cells.iter().zip(idx) {
        for &i in ix {
            h += c[i][metric.0];
            uw += c[i][metric.1];
        }
    }
    h / uw
}

/// ratio of pooled yields arm/base with a stratified bootstrap over bases
fn ratio_ci(
    arm: &[&[[f64; 4]]],
    base: &[&[[f64; 4]]],
    rng: &mut Rng,
    boots: usize,
    metric: (usize, usize),
) -> (f64, f64, f64) {
    let full = |c: &[&[[f64; 4]]]| -> Vec<Vec<usize>> {
        c.iter().map(|v| (0..v.len()).collect()).collect()
    };
    let point = pooled_yield(arm, &full(arm), metric) / pooled_yield(base, &full(base), metric);
    let mut rs = Vec::with_capacity(boots);
    for _ in 0..boots {
        let ia: Vec<Vec<usize>> = arm
            .iter()
            .map(|v| (0..v.len()).map(|_| rng.below(v.len())).collect())
            .collect();
        let ib: Vec<Vec<usize>> = base
            .iter()
            .map(|v| (0..v.len()).map(|_| rng.below(v.len())).collect())
            .collect();
        let r = pooled_yield(arm, &ia, metric) / pooled_yield(base, &ib, metric);
        if r.is_finite() {
            rs.push(r);
        }
    }
    rs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if rs.is_empty() {
        return (point, f64::NAN, f64::NAN);
    }
    let lo = rs[(rs.len() as f64 * 0.025) as usize];
    let hi = rs[((rs.len() as f64 * 0.975) as usize).min(rs.len() - 1)];
    (point, lo, hi)
}

// ---------------------------------------------------------------- curve context

struct Ctx {
    n: u32,
    d1: u32,
    d2: u32,
    big_n: usize,
    r: usize,
    h: usize,
    /// orbits of <M,-1> as index lists, M of order mu (1, 2, 4), restricted
    /// to full-size orbits whose projection to G is nonzero
    orbits: [Vec<Vec<u32>>; 3],
}

const MUS: [usize; 3] = [1, 2, 4];

/// per-base measurement
struct BaseStat {
    h_full: f64,
    h_quot: f64,
    lam_full: f64,
    lam_quot: f64,
    u: f64,
    u0: f64,
    size: usize,
    lemma7_ok: bool,
}

fn measure_base(ctx: &Ctx, mu_i: usize, orbit_ids: &[usize]) -> BaseStat {
    let nn = ctx.big_n;
    let mu = MUS[mu_i];
    let mut f_idx: Vec<u32> = Vec::new();
    for &o in orbit_ids {
        f_idx.extend(ctx.orbits[mu_i][o].iter().copied());
    }
    // pair table
    let mut t2 = vec![false; nn];
    for i in 0..f_idx.len() {
        for j in i..f_idx.len() {
            t2[(f_idx[i] as usize + f_idx[j] as usize) % nn] = true;
        }
    }
    // columns: signed projections [h]P, nonzero
    let mut cols = std::collections::HashSet::new();
    for &i in &f_idx {
        let j = (ctx.h * i as usize) % nn / ctx.h; // in Z_r
        if j != 0 {
            cols.insert(j.min(ctx.r - j));
        }
    }
    // transversal: one representative per M-coset (cosets of <N/mu>)
    let step = nn / mu;
    let mut seen = std::collections::HashSet::new();
    let mut quot: Vec<u32> = Vec::new();
    for &i in &f_idx {
        if seen.insert(i as usize % step) {
            quot.push(i);
        }
    }
    let mut hf = 0usize;
    let mut hq = 0usize;
    let mut sum_f = 0usize;
    let mut sum_q = 0usize;
    let mut ok = true;
    for t in 1..ctx.r {
        let rr = t * ctx.h;
        let cnt_f = f_idx
            .iter()
            .filter(|&&p| t2[(rr + nn - p as usize) % nn])
            .count();
        let cnt_q = quot
            .iter()
            .filter(|&&p| t2[(rr + nn - p as usize) % nn])
            .count();
        sum_f += cnt_f;
        sum_q += cnt_q;
        if cnt_f > 0 {
            hf += 1;
        }
        if cnt_q > 0 {
            hq += 1;
        }
        if cnt_f != mu * cnt_q {
            ok = false;
        }
    }
    let denom = (ctx.r - 1) as f64;
    BaseStat {
        h_full: hf as f64 / denom,
        h_quot: hq as f64 / denom,
        lam_full: sum_f as f64 / denom,
        lam_quot: sum_q as f64 / denom,
        u: cols.len() as f64 + 1.0,
        u0: cols.len() as f64,
        size: f_idx.len(),
        lemma7_ok: ok,
    }
}

fn pick_orbits(ctx: &Ctx, mu_i: usize, f0: usize, rng: &mut Rng) -> Vec<usize> {
    let per = 2 * MUS[mu_i];
    let k = f0 / per;
    let total = ctx.orbits[mu_i].len();
    let mut chosen: Vec<usize> = Vec::new();
    while chosen.len() < k {
        let o = rng.below(total);
        if !chosen.contains(&o) {
            chosen.push(o);
        }
    }
    chosen
}

// ---------------------------------------------------------------- main experiment

struct CurveReport {
    json: String,
    ctx: Ctx,
}

fn analyse_curve(f: &Gf, d1: u32, d2: u32, rng: &mut Rng) -> Option<CurveReport> {
    let ed = Ed { f, d1, d2 };
    let pts = ed.points();
    let nn = pts.len();
    if !nn.is_multiple_of(2) {
        return None;
    }
    // every point on the curve, completeness
    if !pts.iter().all(|&p| ed.on_curve(p)) {
        return None;
    }
    let fac = prime_factors(nn as u64);
    let r = *fac.iter().max().unwrap() as usize;
    if r < 100 || !is_prime(r as u64) || (nn / r).is_multiple_of(r) {
        return None;
    }
    let h = nn / r;
    // generator of full group (cyclic check)
    let mut gen: Option<Pt> = None;
    for _ in 0..300 {
        let p = pts[rng.below(pts.len())];
        let mut ok = ed.mul(nn as u64, p)? == (0, 0);
        for &pf in &fac {
            if ed.mul(nn as u64 / pf, p)? == (0, 0) {
                ok = false;
            }
        }
        if ok {
            gen = Some(p);
            break;
        }
    }
    let gen = gen?; // not cyclic
                    // index table
    let mut by_idx: Vec<Pt> = Vec::with_capacity(nn);
    let mut idx_of: HashMap<Pt, u32> = HashMap::new();
    let mut cur: Pt = (0, 0);
    for k in 0..nn {
        by_idx.push(cur);
        idx_of.insert(cur, k as u32);
        cur = ed.add(cur, gen)?;
    }
    if cur != (0, 0) || idx_of.len() != nn {
        return None;
    }
    // ---- structure checks
    let mut bad_neg = 0usize;
    for k in 0..nn {
        if ed.neg(by_idx[k]) != by_idx[(nn - k) % nn] {
            bad_neg += 1;
        }
    }
    // group law sample (additive in index)
    let mut bad_add = 0usize;
    for _ in 0..4000 {
        let (i, j) = (rng.below(nn), rng.below(nn));
        if ed.add(by_idx[i], by_idx[j])? != by_idx[(i + j) % nn] {
            bad_add += 1;
        }
    }
    let t_idx = nn / 2;
    let has_z4 = nn.is_multiple_of(4);
    let q4_idx = if has_z4 { nn / 4 } else { 0 };
    let t_pt = by_idx[t_idx];
    let q4_pt = by_idx[q4_idx];
    let v2n = nn.trailing_zeros();
    // Weierstrass image
    let c0 = f.mul(d1, f.sq(d1) ^ d1 ^ d2); // d1 (d1^2 + d1 + d2)
    let a2 = f.sq(d1) ^ d2;
    let bw = f.mul(f.sq(f.sq(d1)), f.sq(f.sq(d1) ^ d1 ^ d2)); // d1^4 (d1^2+d1+d2)^2
    let w = Wei { f, a2, b: bw };
    let sb = f.root2k(bw, 1);
    let sb4 = f.root2k(bw, 2);
    let mut img: Vec<WPt> = vec![None; nn];
    let mut img_special = 0usize;
    let mut img_off_curve = 0usize;
    for k in 0..nn {
        let (x, y) = by_idx[k];
        let ww = x ^ y;
        let den = f.mul(x, y) ^ f.mul(d1, ww);
        if den == 0 {
            img_special += 1;
            img[k] = if k == t_idx { Some((0, sb)) } else { None };
            continue;
        }
        let u = f.div(f.mul(c0, ww), den);
        let v = f.mul(c0, f.div(x, den) ^ d1 ^ 1);
        let p = Some((u, v));
        if !w.on_curve(p) {
            img_off_curve += 1;
        }
        img[k] = p;
    }
    let mut bad_hom = 0usize;
    for _ in 0..4000 {
        let (i, j) = (rng.below(nn), rng.below(nn));
        if w.add(img[i], img[j]) != img[(i + j) % nn] {
            bad_hom += 1;
        }
    }
    // fibres of w
    let mut w_fib: HashMap<u32, Vec<usize>> = HashMap::new();
    for (k, &(x, y)) in by_idx.iter().enumerate() {
        w_fib.entry(x ^ y).or_default().push(k);
    }
    let orbit_of = |k: usize, mu: usize| -> Vec<usize> {
        let step = nn / mu;
        let mut o = Vec::new();
        for s in 0..mu {
            o.push((k + s * step) % nn);
            o.push((nn - k + s * step) % nn);
        }
        o.sort_unstable();
        o.dedup();
        o
    };
    let mut bad_w_fibre = 0usize;
    for fib in w_fib.values() {
        let mut fs = fib.clone();
        fs.sort_unstable();
        if fs != orbit_of(fs[0], 2) {
            bad_w_fibre += 1;
        }
    }
    let n_w_fibres = w_fib.len();
    // Theorem 9 analogues in the Weierstrass image
    let xw = |k: usize| -> Option<u32> { img[k].map(|(x, _)| x) };
    let mut bad_translate = 0usize; // x_W(P+T) = sqrt(b)/x_W(P)
    let mut bad_dbl = 0usize; // x_W(2P) = u(P)^2
    let mut bad_u_inv = 0usize; // u(P+T) = u(P) is implied; check anyway
    let mut bad_u4_dbl = 0usize; // x_W(4P) = u4(P)^4
    let mut bad_u4_q4 = 0usize; // u4(P + Q4) = u4(P)
    let mut bad_u4_fibre = 0usize;
    let mut u4_fib: HashMap<u32, Vec<usize>> = HashMap::new();
    let mut tested = 0usize;
    let u_of = |k: usize| -> Option<u32> {
        let x = xw(k)?;
        if x == 0 {
            return None;
        }
        Some(x ^ f.div(sb, x))
    };
    let u4_of = |k: usize| -> Option<u32> {
        let u = u_of(k)?;
        if u == 0 {
            return None;
        }
        Some(u ^ f.div(sb4, u))
    };
    for k in 0..nn {
        let Some(x) = xw(k) else { continue };
        if x == 0 {
            continue;
        }
        tested += 1;
        if xw((k + t_idx) % nn) != Some(f.div(sb, x)) {
            bad_translate += 1;
        }
        let u = x ^ f.div(sb, x);
        match xw((2 * k) % nn) {
            Some(x2) if x2 == f.sq(u) => {}
            _ => bad_dbl += 1,
        }
        if u_of((k + t_idx) % nn) != Some(u) {
            bad_u_inv += 1;
        }
        if let Some(u4) = u4_of(k).filter(|_| has_z4) {
            u4_fib.entry(u4).or_default().push(k);
            if u4_of((k + q4_idx) % nn) != Some(u4) {
                bad_u4_q4 += 1;
            }
            let u44 = f.sq(f.sq(u4));
            match xw((4 * k) % nn) {
                Some(x4) if x4 == u44 => {}
                _ => bad_u4_dbl += 1,
            }
        }
    }
    for fib in u4_fib.values() {
        let mut fs = fib.clone();
        fs.sort_unstable();
        // the fibre of u4 over the points where u4 is defined must be a union
        // of full <Q4,-1> orbits; here it must equal one orbit (undefined
        // points are 2-power torsion only)
        if fs != orbit_of(fs[0], 4) {
            bad_u4_fibre += 1;
        }
    }
    // order-4 points: coordinates
    let tors = format!(
        "{{\"T\":[{},{}],\"Q4\":[{},{}],\"neg_Q4\":[{},{}]}}",
        t_pt.0,
        t_pt.1,
        q4_pt.0,
        q4_pt.1,
        by_idx[(nn - q4_idx) % nn].0,
        by_idx[(nn - q4_idx) % nn].1
    );
    let t_is_11 = t_pt == (1, 1);
    let w_q4_is_1 = has_z4 && (q4_pt.0 ^ q4_pt.1) == 1;
    // ---- orbit pools
    let mut orbits: [Vec<Vec<u32>>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for (mi, &mu) in MUS.iter().enumerate() {
        if !nn.is_multiple_of(mu) {
            continue;
        }
        let mut seen = vec![false; nn];
        for k in 1..nn {
            if seen[k] {
                continue;
            }
            let o = orbit_of(k, mu);
            for &e in &o {
                seen[e] = true;
            }
            if o.len() != 2 * mu {
                continue;
            }
            if o.iter().any(|&e| (h * e).is_multiple_of(nn)) {
                continue;
            }
            orbits[mi].push(o.iter().map(|&e| e as u32).collect());
        }
    }
    let ctx = Ctx {
        n: f.n,
        d1,
        d2,
        big_n: nn,
        r,
        h,
        orbits,
    };
    let json = format!(
        "{{\"n\":{},\"d1\":{d1},\"d2\":{d2},\"tr_d1\":{},\"tr_d2\":{},\"N\":{nn},\"r\":{r},\"h\":{h},\"v2N\":{v2n},\
\"cyclic\":true,\"T_is_1_1\":{t_is_11},\"has_z4\":{has_z4},\"w_of_Q4_is_1\":{w_q4_is_1},\"torsion\":{tors},\
\"checks\":{{\"bad_neg_is_swap\":{bad_neg},\"bad_group_law_sample\":{bad_add},\
\"weierstrass_special_points\":{img_special},\"weierstrass_off_curve\":{img_off_curve},\"bad_hom_sample\":{bad_hom},\
\"w_fibres\":{n_w_fibres},\"bad_w_fibre_is_T_orbit\":{bad_w_fibre},\
\"points_tested_weierstrass\":{tested},\"bad_xW_translate_T\":{bad_translate},\"bad_xW_2P_eq_u2\":{bad_dbl},\
\"bad_u_T_invariant\":{bad_u_inv},\"bad_u4_Q4_invariant\":{bad_u4_q4},\"bad_xW_4P_eq_u4_4\":{bad_u4_dbl},\
\"u4_fibres\":{},\"bad_u4_fibre_is_Q4_orbit\":{bad_u4_fibre}}},\
\"orbit_pool\":[{},{},{}]}}",
        f.n,
        f.tr(d1),
        f.tr(d2),
        u4_fib.len(),
        ctx.orbits[0].len(),
        ctx.orbits[1].len(),
        ctx.orbits[2].len()
    );
    Some(CurveReport { json, ctx })
}

/// (name, (numerator column, denominator column)); the primary metric is lambda/c
const METRICS: [(&str, (usize, usize)); 4] = [
    ("lambda/c", (2, 3)),
    ("lambda/c+1", (2, 1)),
    ("H/c", (0, 3)),
    ("H/c+1", (0, 1)),
];

const ARMS: [&str; 5] = ["plain", "s2_full", "s2_quot", "s4_full", "s4_quot"];

struct Cell {
    json: String,
    /// per arm: (H, U*W) per base
    data: [Vec<[f64; 4]>; 5],
    lemma7_bad: usize,
    lemma7_n: usize,
    f0: usize,
    h_plain: f64,
}

fn run_cell(ctx: &Ctx, f0: usize, bases: usize, rng: &mut Rng) -> Cell {
    let mut data: [Vec<[f64; 4]>; 5] = Default::default();
    let mut bad = 0usize;
    let mut cnt = 0usize;
    let mut size_dev = 0usize;
    for _ in 0..bases {
        for mi in 0..3 {
            if ctx.orbits[mi].is_empty() {
                continue;
            }
            let ids = pick_orbits(ctx, mi, f0, rng);
            let st = measure_base(ctx, mi, &ids);
            if st.size != f0 {
                size_dev += 1;
            }
            let mu = MUS[mi] as f64;
            if mi == 0 {
                data[0].push([
                    st.h_full,
                    st.u * st.size as f64,
                    st.lam_full,
                    st.u0 * st.size as f64,
                ]);
            } else {
                data[2 * mi - 1].push([
                    st.h_full,
                    st.u * st.size as f64,
                    st.lam_quot,
                    st.u0 * st.size as f64,
                ]);
                data[2 * mi].push([
                    st.h_quot,
                    st.u * st.size as f64 / mu,
                    st.lam_quot,
                    st.u0 * st.size as f64 / mu,
                ]);
                cnt += 1;
                if !st.lemma7_ok || (st.h_full - st.h_quot).abs() > 0.0 {
                    bad += 1;
                }
            }
        }
    }
    let mean = |v: &[[f64; 4]]| v.iter().map(|x| x[0]).sum::<f64>() / v.len() as f64;
    let cols: Vec<String> = (0..5)
        .filter(|&a| !data[a].is_empty())
        .map(|a| {
            format!(
                "\"{}\":{{\"mean_H\":{:.6},\"mean_lambda\":{:.6},\"mean_UW\":{:.3},\"yield_H\":{:.6e},\"yield_lambda\":{:.6e}}}",
                ARMS[a],
                mean(&data[a]),
                data[a].iter().map(|x| x[2]).sum::<f64>() / bases as f64,
                data[a].iter().map(|x| x[1]).sum::<f64>() / bases as f64,
                data[a].iter().map(|x| x[0]).sum::<f64>() / data[a].iter().map(|x| x[1]).sum::<f64>(),
                data[a].iter().map(|x| x[2]).sum::<f64>() / data[a].iter().map(|x| x[1]).sum::<f64>()
            )
        })
        .collect();
    let json = format!(
        "{{\"n\":{},\"d1\":{},\"d2\":{},\"N\":{},\"r\":{},\"h\":{},\"F0\":{f0},\"bases\":{bases},\
\"size_mismatch\":{size_dev},\"lemma7_checks\":{cnt},\"lemma7_violations\":{bad},\"arms\":{{{}}}}}",
        ctx.n,
        ctx.d1,
        ctx.d2,
        ctx.big_n,
        ctx.r,
        ctx.h,
        cols.join(",")
    );
    let h_plain = mean(&data[0]);
    Cell {
        json,
        data,
        lemma7_bad: bad,
        lemma7_n: cnt,
        f0,
        h_plain,
    }
}

/// pilot: pick F0 (multiple of 8) whose plain-arm mean H is closest to target
fn calibrate(ctx: &Ctx, target: f64, rng: &mut Rng) -> usize {
    let mut best = (f64::MAX, 8usize);
    let mut f0 = 8usize;
    let cap = (ctx.big_n / 6).min(128);
    while f0 <= cap {
        let z4_ok = ctx.orbits[2].is_empty() || ctx.orbits[2].len() >= f0 / 8 + 2;
        if z4_ok && ctx.orbits[1].len() >= f0 / 4 + 2 && ctx.orbits[0].len() >= f0 / 2 + 2 {
            let mut hs = 0.0;
            let pilot = 40;
            for _ in 0..pilot {
                let ids = pick_orbits(ctx, 0, f0, rng);
                hs += measure_base(ctx, 0, &ids).h_full;
            }
            let hm = hs / pilot as f64;
            let dev = (hm - target).abs();
            if dev < best.0 {
                best = (dev, f0);
            }
        }
        f0 += 8;
    }
    best.1
}

fn main() {
    let mut rng = Rng(0x0074_2026_1031);
    let bases: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(300);
    let per_n: usize = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(6);
    let targets = [0.05f64, 0.1, 0.25, 0.5];
    let mut structure: Vec<String> = Vec::new();
    let mut cells_json: Vec<String> = Vec::new();
    // collected cells for pooled ratios, keyed by (family, target index)
    struct Keep {
        n: u32,
        v2: u32,
        trd1: u32,
        tgt: usize,
        cell: Cell,
    }
    let mut keep: Vec<Keep> = Vec::new();
    let mut lemma7_bad = 0usize;
    let mut lemma7_n = 0usize;
    for n in [9u32, 10, 11, 12, 13, 14] {
        let f = Gf::new(n);
        let want = if n <= 12 { per_n } else { per_n.min(3) };
        let mut got = 0usize;
        let mut tried = 0usize;
        let mut seen: Vec<(u32, u32)> = Vec::new();
        while got < want && tried < 20000 {
            tried += 1;
            let d1 = 1 + (rng.next() as u32) % (f.q as u32 - 1);
            let d2 = (rng.next() as u32) % f.q as u32;
            if f.tr(d2) != 1 || seen.contains(&(d1, d2)) {
                continue;
            }
            let Some(rep) = analyse_curve(&f, d1, d2, &mut rng) else {
                continue;
            };
            seen.push((d1, d2));
            got += 1;
            structure.push(rep.json.clone());
            let v2 = rep.ctx.big_n.trailing_zeros();
            let trd1 = f.tr(d1);
            for (ti, &tg) in targets.iter().enumerate() {
                let f0 = calibrate(&rep.ctx, tg, &mut rng);
                let cell = run_cell(&rep.ctx, f0, bases, &mut rng);
                lemma7_bad += cell.lemma7_bad;
                lemma7_n += cell.lemma7_n;
                eprintln!(
                    "n={n} d1={d1} d2={d2} N={} r={} h={} v2={v2} Trd1={trd1} target={tg} F0={f0} H_plain={:.3}",
                    rep.ctx.big_n, rep.ctx.r, rep.ctx.h, cell.h_plain
                );
                cells_json.push(format!(
                    "{{\"target_H_plain\":{tg},\"v2N\":{v2},\"tr_d1\":{trd1},\"cell\":{}}}",
                    cell.json
                ));
                keep.push(Keep {
                    n,
                    v2,
                    trd1,
                    tgt: ti,
                    cell,
                });
            }
        }
        eprintln!("n={n}: {got} curves after {tried} draws");
    }
    // torsion tally over random curves: complete Edwards vs Weierstrass
    let mut tally: Vec<String> = Vec::new();
    for n in [9u32, 10, 11, 12] {
        let f = Gf::new(n);
        let mut ed_cnt = [[0usize; 2]; 2]; // [tr(d1)][4 | N]
        let mut wei_cnt = [[0usize; 2]; 2]; // [tr(a)][4 | N]
        let mut ed_total = 0usize;
        let mut ed_incomplete_formula_fail = 0usize;
        let draws = 400usize;
        for _ in 0..draws {
            let d1 = 1 + (rng.next() as u32) % (f.q as u32 - 1);
            let d2 = loop {
                let d2 = (rng.next() as u32) % f.q as u32;
                if f.tr(d2) == 1 {
                    break d2;
                }
            };
            let ed = Ed { f: &f, d1, d2 };
            let nn = ed.points().len();
            ed_cnt[f.tr(d1) as usize][usize::from(nn.is_multiple_of(4))] += 1;
            ed_total += 1;
            let _ = &mut ed_incomplete_formula_fail;
            // Weierstrass y^2 + xy = x^3 + a x^2 + b, b != 0, a random
            let a = (rng.next() as u32) % f.q as u32;
            let b = 1 + (rng.next() as u32) % (f.q as u32 - 1);
            let mut cnt = 1usize; // point at infinity
            for x in 0..f.q as u32 {
                let rhs = f.mul(f.sq(x), x) ^ f.mul(a, f.sq(x)) ^ b;
                if x == 0 {
                    cnt += 1;
                } else {
                    let c = f.div(rhs, f.sq(x));
                    if f.tr(c) == 0 {
                        cnt += 2;
                    }
                }
            }
            wei_cnt[f.tr(a) as usize][usize::from(cnt.is_multiple_of(4))] += 1;
        }
        tally.push(format!(
            "{{\"n\":{n},\"edwards_complete_draws\":{ed_total},\"edwards_trd1_0_N_mod4_nonzero\":{},\"edwards_trd1_0_4_divides_N\":{},\"edwards_trd1_1_N_mod4_nonzero\":{},\"edwards_trd1_1_4_divides_N\":{},\"weierstrass_tra_0_not_4_divides\":{},\"weierstrass_tra_0_4_divides\":{},\"weierstrass_tra_1_not_4_divides\":{},\"weierstrass_tra_1_4_divides\":{}}}",
            ed_cnt[0][0], ed_cnt[0][1], ed_cnt[1][0], ed_cnt[1][1], wei_cnt[0][0], wei_cnt[0][1], wei_cnt[1][0], wei_cnt[1][1]
        ));
    }
    // pooled ratios
    let mut ratio_rows: Vec<String> = Vec::new();
    let mut brng = Rng(0xB007_5742);
    type Pred = Box<dyn Fn(&Keep) -> bool>;
    let mut groups: Vec<(String, Pred)> = Vec::new();
    groups.push(("n<=12".to_string(), Box::new(|k: &Keep| k.n <= 12)));
    for n in [9u32, 10, 11, 12, 13, 14] {
        groups.push((format!("n={n}"), Box::new(move |k: &Keep| k.n == n)));
    }
    groups.push((
        "v2N=1".to_string(),
        Box::new(|k: &Keep| k.v2 == 1 && k.n <= 12),
    ));
    groups.push((
        "v2N=2".to_string(),
        Box::new(|k: &Keep| k.v2 == 2 && k.n <= 12),
    ));
    groups.push((
        "v2N>=3".to_string(),
        Box::new(|k: &Keep| k.v2 >= 3 && k.n <= 12),
    ));
    groups.push((
        "Tr(d1)=0".to_string(),
        Box::new(|k: &Keep| k.trd1 == 0 && k.n <= 12),
    ));
    groups.push((
        "Tr(d1)=1".to_string(),
        Box::new(|k: &Keep| k.trd1 == 1 && k.n <= 12),
    ));
    let mut falsifier_hits: Vec<String> = Vec::new();
    for (mname, metric) in METRICS {
        for (ti, &tg) in targets.iter().enumerate() {
            for (gname, pred) in &groups {
                let all_sel: Vec<&Keep> = keep.iter().filter(|k| k.tgt == ti && pred(k)).collect();
                for (a, arm_name) in ARMS.iter().enumerate().skip(1) {
                    let sel: Vec<&Keep> = all_sel
                        .iter()
                        .copied()
                        .filter(|k| !k.cell.data[a].is_empty())
                        .collect();
                    if sel.is_empty() {
                        continue;
                    }
                    let base: Vec<&[[f64; 4]]> =
                        sel.iter().map(|k| k.cell.data[0].as_slice()).collect();
                    let arm: Vec<&[[f64; 4]]> =
                        sel.iter().map(|k| k.cell.data[a].as_slice()).collect();
                    let (pt, lo, hi) = ratio_ci(&arm, &base, &mut brng, 2000, metric);
                    let is_quot = a == 2 || a == 4;
                    let excl = !(lo <= 1.0 && 1.0 <= hi);
                    if is_quot && excl && !gname.starts_with("n=") {
                        falsifier_hits.push(format!("{gname}/{}/{mname}/target{tg}", arm_name));
                    }
                    // law prediction (small-hit-rate limit, m = 3): full sweep 1/mu, quotient 1
                    let mu = if a <= 2 { 2.0 } else { 4.0 };
                    let pred = if is_quot { 1.0 } else { 1.0 / mu };
                    let hp = sel.iter().map(|k| k.cell.h_plain).sum::<f64>() / sel.len() as f64;
                    ratio_rows.push(format!(
                        "{{\"metric\":\"{mname}\",\"group\":\"{gname}\",\"target_H_plain\":{tg},\"mean_realised_H_plain\":{hp:.4},\"curves\":{},\"arm\":\"{}\",\"ratio\":{pt:.4},\"ci_lo\":{lo:.4},\"ci_hi\":{hi:.4},\"ci_contains_1\":{},\"law_prediction\":{pred:.4},\"ci_contains_law\":{}}}",
                        sel.len(),
                        arm_name,
                        !excl,
                        lo <= pred && pred <= hi
                    ));
                }
            }
        }
    }
    // per-curve ratios (quot arms)
    let mut curve_rows: Vec<String> = Vec::new();
    for k in &keep {
        let base = [k.cell.data[0].as_slice()];
        for (a, arm_name) in ARMS.iter().enumerate().skip(1) {
            if k.cell.data[a].is_empty() {
                continue;
            }
            let arm = [k.cell.data[a].as_slice()];
            for (mname, metric) in METRICS {
                let (pt, lo, hi) = ratio_ci(&arm, &base, &mut brng, 1000, metric);
                curve_rows.push(format!(
                    "{{\"metric\":\"{mname}\",\"n\":{},\"v2N\":{},\"tr_d1\":{},\"target_H_plain\":{},\"F0\":{},\"H_plain\":{:.4},\"arm\":\"{}\",\"ratio\":{pt:.4},\"ci_lo\":{lo:.4},\"ci_hi\":{hi:.4}}}",
                    k.n, k.v2, k.trd1, targets[k.tgt], k.cell.f0, k.cell.h_plain, arm_name
                ));
            }
        }
    }
    println!(
        "{{\"experiment\":\"r7_4_binary_edwards\",\"kind\":\"measured (exact structure over all points; oracle exact over all targets of G; bootstrap over bases)\",\
\"bases_per_arm_per_cell\":{bases},\"lemma7_checks\":{lemma7_n},\"lemma7_violations\":{lemma7_bad},\
\"quotient_arm_ci_excludes_1\":[{}],\n\"torsion_tally\":[\n{}\n],\n\"structure\":[\n{}\n],\n\"cells\":[\n{}\n],\n\"pooled_ratios\":[\n{}\n],\n\"per_curve_ratios\":[\n{}\n]}}",
        falsifier_hits
            .iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<_>>()
            .join(","),
        tally.join(",\n"),
        structure.join(",\n"),
        cells_json.join(",\n"),
        ratio_rows.join(",\n"),
        curve_rows.join(",\n")
    );
}
