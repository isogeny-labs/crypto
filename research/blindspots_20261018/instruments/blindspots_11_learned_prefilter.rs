//! Blind-spot item 11: a learned necessary-condition prefilter on x_R for
//! subspace factor bases (binary curves, m = 2, 3 summands).
//!
//! Standard library only. Build (each card does this):
//!   rustc --edition 2021 -C opt-level=3 -C codegen-units=1 -C target-cpu=native \
//!         -o target/blindspots-11/<dir>/b11 examples/blindspots_11_learned_prefilter.rs
//!
//! Subcommands: selftest | plan | cell | verdict (see PROTOCOL.md).
//!
//! Objects. F = GF(2^n), n odd. E_{a,b}: y^2 + xy = x^3 + a x^2 + b with
//! a in {0,1}, b != 0. V an F_2-subspace of F of dimension l; the base is
//! B = { P in E(F) : x(P) in V }, closed under negation. The label of a target
//! R is "R lies in the m-fold sumset B + .. + B" (summands finite, repeats and
//! degenerate P - P + Q allowed), decided exactly.
//!
//! Arms: (a) no filter; (b) the trace filter: delta(P) = Tr(x(P) + a) is a
//! homomorphism E(F) -> F_2, so delta(R) must lie in the m-fold sumset of
//! D = { delta(P) : P in B }, refuting R otherwise; (c) learned classifiers
//! (logistic regression, gradient-boosted trees) over features of x_R; (d)
//! within-base label-permutation null; (e) planted-label positive controls.

use std::cell::Cell;
use std::time::Instant;

// ------------------------------------------------------------------ utilities

thread_local! {
    static POPCH: Cell<char> = Cell::new('U');
}

/// Print one flat JSON object with the target population tagged first.
macro_rules! emitj {
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        println!("{{\"pop\":\"{}\",{}", POPCH.with(|c| c.get()), &s[1..]);
    }};
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        let lim = u64::MAX - (u64::MAX % n);
        loop {
            let v = self.next();
            if v < lim {
                return v % n;
            }
        }
    }
}

fn mix(a: u64, b: u64) -> u64 {
    let mut r = Rng(a ^ b.wrapping_mul(0xD6E8_FEB8_6659_FD93));
    r.next()
}

struct Args {
    v: Vec<String>,
}
impl Args {
    fn flag(&self, name: &str) -> Option<String> {
        self.v
            .iter()
            .position(|a| a == name)
            .and_then(|i| self.v.get(i + 1))
            .cloned()
    }
    fn num<T: std::str::FromStr>(&self, name: &str, default: T) -> T {
        self.flag(name)
            .map(|s| s.parse().unwrap_or_else(|_| panic!("bad value for {name}")))
            .unwrap_or(default)
    }
}

fn rss_peak_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1).and_then(|v| v.parse().ok()))
        })
        .unwrap_or(0)
}

// ------------------------------------------------------------------- GF(2^n)

fn pmod(mut a: u64, b: u64) -> u64 {
    let db = 63 - b.leading_zeros();
    while a != 0 && 63 - a.leading_zeros() >= db {
        a ^= b << (63 - a.leading_zeros() - db);
    }
    a
}

fn is_irreducible(f: u64) -> bool {
    let n = 63 - f.leading_zeros();
    if n == 1 {
        return true;
    }
    for g in 2..(1u64 << (n / 2 + 1)) {
        if pmod(f, g) == 0 {
            return false;
        }
    }
    true
}

/// Same candidate order as the repository's `find_irreducible_sparse` (and as
/// the other blind-spot instruments): the least `low` among constant + up to
/// three further terms for which `z^n + low` is irreducible.
fn find_irreducible_sparse(n: u32) -> u64 {
    let mut cands: Vec<u64> = vec![1];
    for i in 1..n {
        cands.push(1 | (1 << i));
        for j in (i + 1)..n {
            cands.push(1 | (1 << i) | (1 << j));
            for k in (j + 1)..n {
                cands.push(1 | (1 << i) | (1 << j) | (1 << k));
            }
        }
    }
    cands.sort_unstable();
    let hi = 1u64 << n;
    for low in cands {
        if is_irreducible(hi | low) {
            return low;
        }
    }
    panic!("no sparse irreducible of degree {n}");
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
unsafe fn clmul_hw(a: u64, b: u64) -> u64 {
    use std::arch::x86_64::*;
    let x = _mm_set_epi64x(0, a as i64);
    let y = _mm_set_epi64x(0, b as i64);
    let r = _mm_clmulepi64_si128(x, y, 0x00);
    _mm_cvtsi128_si64(r) as u64
}

fn clmul_soft(a: u64, mut b: u64) -> u64 {
    let (mut r, mut s) = (0u64, a);
    while b != 0 {
        if b & 1 == 1 {
            r ^= s;
        }
        s <<= 1;
        b >>= 1;
    }
    r
}

#[derive(Default, Clone, Copy, Debug)]
struct OpSnap {
    mul: u64,
    sqr: u64,
    lin: u64,
}
impl OpSnap {
    fn sub(self, o: OpSnap) -> OpSnap {
        OpSnap {
            mul: self.mul - o.mul,
            sqr: self.sqr - o.sqr,
            lin: self.lin - o.lin,
        }
    }
}

struct Gf {
    n: u32,
    mask: u64,
    lowbits: Vec<u32>,
    trmask: u64,
    ht: Vec<u64>,
    hw: bool,
    c_mul: Cell<u64>,
    c_sqr: Cell<u64>,
    c_lin: Cell<u64>,
    nb_rows: Vec<u64>,
}

impl Gf {
    fn new(n: u32) -> Gf {
        assert!(n % 2 == 1 && (5..=31).contains(&n), "odd n in 5..=31");
        let low = find_irreducible_sparse(n);
        let lowbits: Vec<u32> = (0..n).filter(|j| (low >> j) & 1 == 1).collect();
        #[cfg(target_arch = "x86_64")]
        let hw = is_x86_feature_detected!("pclmulqdq");
        #[cfg(not(target_arch = "x86_64"))]
        let hw = false;
        let mut gf = Gf {
            n,
            mask: (1u64 << n) - 1,
            lowbits,
            trmask: 0,
            ht: vec![],
            hw,
            c_mul: Cell::new(0),
            c_sqr: Cell::new(0),
            c_lin: Cell::new(0),
            nb_rows: vec![],
        };
        // trace mask
        let mut tm = 0u64;
        for k in 0..n {
            let e = 1u64 << k;
            let (mut t, mut s) = (0u64, e);
            for _ in 0..n {
                t ^= s;
                s = gf.sqr(s);
            }
            assert!(t == 0 || t == 1);
            if t == 1 {
                tm |= e;
            }
        }
        gf.trmask = tm;
        // half-trace images of the basis: H(c) = sum_{j=0}^{(n-1)/2} c^(4^j)
        let mut ht = vec![0u64; n as usize];
        for k in 0..n {
            let mut s = 1u64 << k;
            let mut t = 0u64;
            for _ in 0..=((n - 1) / 2) {
                t ^= s;
                s = gf.sqr(gf.sqr(s));
            }
            ht[k as usize] = t;
        }
        gf.ht = ht;
        gf.build_normal_basis();
        gf.c_mul.set(0);
        gf.c_sqr.set(0);
        gf.c_lin.set(0);
        gf
    }
    fn snap(&self) -> OpSnap {
        OpSnap {
            mul: self.c_mul.get(),
            sqr: self.c_sqr.get(),
            lin: self.c_lin.get(),
        }
    }
    #[inline(always)]
    fn reduce(&self, mut r: u64) -> u64 {
        loop {
            let hi = r >> self.n;
            if hi == 0 {
                return r;
            }
            r &= self.mask;
            for &j in &self.lowbits {
                r ^= hi << j;
            }
        }
    }
    #[inline(always)]
    fn clmul(&self, a: u64, b: u64) -> u64 {
        #[cfg(target_arch = "x86_64")]
        {
            if self.hw {
                return unsafe { clmul_hw(a, b) };
            }
        }
        clmul_soft(a, b)
    }
    #[inline(always)]
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.c_mul.set(self.c_mul.get() + 1);
        self.reduce(self.clmul(a, b))
    }
    #[inline(always)]
    fn sqr(&self, a: u64) -> u64 {
        self.c_sqr.set(self.c_sqr.get() + 1);
        self.reduce(self.clmul(a, a))
    }
    fn mul_soft(&self, a: u64, b: u64) -> u64 {
        self.reduce(clmul_soft(a, b))
    }
    /// a^(2^n - 2) by the Itoh-Tsujii chain on n - 1.
    fn inv(&self, a: u64) -> u64 {
        assert!(a != 0, "inverse of zero");
        let e = self.n - 1;
        let bits = 32 - e.leading_zeros();
        let (mut b, mut k) = (a, 1u32);
        for i in (0..bits - 1).rev() {
            let mut t = b;
            for _ in 0..k {
                t = self.sqr(t);
            }
            b = self.mul(t, b);
            k *= 2;
            if (e >> i) & 1 == 1 {
                b = self.mul(self.sqr(b), a);
                k += 1;
            }
        }
        debug_assert_eq!(k, e);
        self.sqr(b)
    }
    #[inline(always)]
    fn tr(&self, a: u64) -> u64 {
        ((a & self.trmask).count_ones() & 1) as u64
    }
    /// A root z of z^2 + z = c + Tr(c) (so of z^2 + z = c when Tr(c) = 0).
    #[inline(always)]
    fn half_trace(&self, c: u64) -> u64 {
        let mut r = 0u64;
        let mut x = c;
        while x != 0 {
            let i = x.trailing_zeros();
            r ^= self.ht[i as usize];
            x &= x - 1;
        }
        r
    }
    fn sqrt(&self, a: u64) -> u64 {
        let mut t = a;
        for _ in 0..(self.n - 1) {
            t = self.sqr(t);
        }
        t
    }
    fn build_normal_basis(&mut self) {
        let n = self.n as usize;
        let mut rng = Rng(0xB11_0000 + n as u64);
        loop {
            let alpha = rng.next() & self.mask;
            if alpha == 0 {
                continue;
            }
            // columns: alpha^(2^i)
            let mut col = vec![0u64; n];
            let mut s = alpha;
            for c in col.iter_mut() {
                *c = s;
                s = self.sqr(s);
            }
            // Solve M b = x: M has columns col[i]. Build M as row masks:
            // row r bit i = bit r of col[i].
            let mut rows = vec![0u64; n];
            for r in 0..n {
                let mut m = 0u64;
                for (i, c) in col.iter().enumerate() {
                    if (c >> r) & 1 == 1 {
                        m |= 1 << i;
                    }
                }
                rows[r] = m;
            }
            // invert [rows | I]
            let mut aug: Vec<(u64, u64)> = (0..n).map(|r| (rows[r], 1u64 << r)).collect();
            let mut ok = true;
            for c in 0..n {
                let p = (c..n).find(|&r| (aug[r].0 >> c) & 1 == 1);
                match p {
                    None => {
                        ok = false;
                        break;
                    }
                    Some(p) => {
                        aug.swap(c, p);
                        let pr = aug[c];
                        for r in 0..n {
                            if r != c && (aug[r].0 >> c) & 1 == 1 {
                                aug[r].0 ^= pr.0;
                                aug[r].1 ^= pr.1;
                            }
                        }
                    }
                }
            }
            if !ok {
                continue;
            }
            // M^{-1} rows are aug[r].1 (row r of the inverse): b_r = parity(x & inv_row_r)
            self.nb_rows = (0..n).map(|r| aug[r].1).collect();
            return;
        }
    }
    fn nb_coords(&self, x: u64) -> u64 {
        let mut b = 0u64;
        for (r, m) in self.nb_rows.iter().enumerate() {
            b |= (((x & m).count_ones() & 1) as u64) << r;
        }
        b
    }
}

// --------------------------------------------------------------------- curve

#[derive(Clone, Copy)]
struct Curve {
    a: u64,
    b: u64,
}

type Pt = Option<(u64, u64)>;

fn on_curve(gf: &Gf, cv: &Curve, p: (u64, u64)) -> bool {
    let (x, y) = p;
    let lhs = gf.sqr(y) ^ gf.mul(x, y);
    let x2 = gf.sqr(x);
    let rhs = gf.mul(x2, x) ^ gf.mul(cv.a, x2) ^ cv.b;
    lhs == rhs
}

fn neg(p: Pt) -> Pt {
    p.map(|(x, y)| (x, y ^ x))
}

fn add(gf: &Gf, cv: &Curve, p: Pt, q: Pt) -> Pt {
    let (p, q) = match (p, q) {
        (None, q) => return q,
        (p, None) => return p,
        (Some(p), Some(q)) => (p, q),
    };
    let (x1, y1) = p;
    let (x2, y2) = q;
    if x1 == x2 {
        if y1 != y2 {
            return None; // q = -p
        }
        if x1 == 0 {
            return None; // 2T = O
        }
        let lam = x1 ^ gf.mul(y1, gf.inv(x1));
        let x3 = gf.sqr(lam) ^ lam ^ cv.a;
        let y3 = gf.sqr(x1) ^ gf.mul(lam ^ 1, x3);
        return Some((x3, y3));
    }
    let lam = gf.mul(y1 ^ y2, gf.inv(x1 ^ x2));
    let x3 = gf.sqr(lam) ^ lam ^ cv.a ^ x1 ^ x2;
    let y3 = gf.mul(lam, x1 ^ x3) ^ x3 ^ y1;
    Some((x3, y3))
}

fn scalar_mul(gf: &Gf, cv: &Curve, p: Pt, mut k: u64) -> Pt {
    let (mut r, mut q) = (None, p);
    while k > 0 {
        if k & 1 == 1 {
            r = add(gf, cv, r, q);
        }
        q = add(gf, cv, q, q);
        k >>= 1;
    }
    r
}

/// A point with abscissa x, if one exists (sign fixed by the half-trace).
fn point_at(gf: &Gf, cv: &Curve, x: u64) -> Option<(u64, u64)> {
    if x == 0 {
        return Some((0, gf.sqrt(cv.b)));
    }
    let xi = gf.inv(x);
    let c = x ^ cv.a ^ gf.mul(cv.b, gf.sqr(xi));
    if gf.tr(c) == 1 {
        return None;
    }
    let z = gf.half_trace(c);
    Some((x, gf.mul(x, z)))
}

/// delta(P) = Tr(x(P) + a).
fn delta(gf: &Gf, cv: &Curve, x: u64) -> u64 {
    gf.tr(x ^ cv.a)
}

// ---------------------------------------------------------------------- base

struct Base {
    ech: Vec<(u32, u64)>, // (pivot bit, vector), descending pivot
    pts: Vec<(u64, u64)>, // one point per x-class with a point, x in V
    dset: [bool; 2],     // values of delta over B
    tv0: bool,           // Tr vanishes on V
}

fn ech_insert(ech: &mut Vec<(u32, u64)>, v: u64) -> bool {
    let mut x = v;
    for &(p, e) in ech.iter() {
        if (x >> p) & 1 == 1 {
            x ^= e;
        }
    }
    if x == 0 {
        return false;
    }
    let p = 63 - x.leading_zeros();
    ech.push((p, x));
    ech.sort_by(|a, b| b.0.cmp(&a.0));
    true
}

#[inline(always)]
fn ech_reduce(ech: &[(u32, u64)], mut x: u64) -> u64 {
    for &(p, e) in ech {
        if (x >> p) & 1 == 1 {
            x ^= e;
        }
    }
    x
}

fn span_all(basis: &[u64]) -> Vec<u64> {
    let l = basis.len();
    let mut out = Vec::with_capacity(1 << l);
    let mut cur = 0u64;
    out.push(0);
    for i in 1u64..(1u64 << l) {
        cur ^= basis[i.trailing_zeros() as usize];
        out.push(cur);
    }
    out
}

fn build_base(gf: &Gf, cv: &Curve, kind: char, ell: usize, rng: &mut Rng) -> Base {
    let mut ech: Vec<(u32, u64)> = vec![];
    let mut basis: Vec<u64> = vec![];
    match kind {
        'P' => {
            // geometric: 1, beta, beta^2, ..
            loop {
                let beta = rng.next() & gf.mask;
                if beta <= 1 {
                    continue;
                }
                ech.clear();
                basis.clear();
                let mut s = 1u64;
                let mut ok = true;
                for _ in 0..ell {
                    if !ech_insert(&mut ech, s) {
                        ok = false;
                        break;
                    }
                    basis.push(s);
                    s = gf.mul(s, beta);
                }
                if ok {
                    break;
                }
            }
        }
        _ => {
            while basis.len() < ell {
                let v = rng.next() & gf.mask;
                if kind == 'T' && gf.tr(v) == 1 {
                    continue;
                }
                if ech_insert(&mut ech, v) {
                    basis.push(v);
                }
            }
            if kind == 'G' && basis.iter().all(|&v| gf.tr(v) == 0) {
                // force V not inside the trace-zero hyperplane: swap the last
                // vector for one of trace 1
                loop {
                    let v = rng.next() & gf.mask;
                    if gf.tr(v) == 0 {
                        continue;
                    }
                    let mut e2: Vec<(u32, u64)> = vec![];
                    let mut ok = true;
                    for &u in basis[..ell - 1].iter().chain(std::iter::once(&v)) {
                        if !ech_insert(&mut e2, u) {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        basis[ell - 1] = v;
                        ech = e2;
                        break;
                    }
                }
            }
        }
    }
    let tv0 = basis.iter().all(|&v| gf.tr(v) == 0);
    let all = span_all(&basis);
    // points by batch inversion of the non-zero abscissae
    let nz: Vec<u64> = all.iter().copied().filter(|&x| x != 0).collect();
    let inv = batch_inv(gf, &nz);
    let mut pts = vec![];
    let mut dset = [false; 2];
    pts.push((0u64, gf.sqrt(cv.b)));
    dset[delta(gf, cv, 0) as usize] = true;
    for (k, &x) in nz.iter().enumerate() {
        let xi = inv[k];
        let c = x ^ cv.a ^ gf.mul(cv.b, gf.sqr(xi));
        if gf.tr(c) == 0 {
            let z = gf.half_trace(c);
            pts.push((x, gf.mul(x, z)));
            dset[delta(gf, cv, x) as usize] = true;
        }
    }
    Base {
        ech,
        pts,
        dset,
        tv0,
    }
}

fn batch_inv(gf: &Gf, d: &[u64]) -> Vec<u64> {
    let k = d.len();
    if k == 0 {
        return vec![];
    }
    let mut pre = Vec::with_capacity(k);
    let mut acc = 1u64;
    for &v in d {
        acc = gf.mul(acc, v);
        pre.push(acc);
    }
    let mut inv_all = gf.inv(acc);
    let mut out = vec![0u64; k];
    for i in (0..k).rev() {
        let prev = if i == 0 { 1 } else { pre[i - 1] };
        out[i] = if i == 0 { inv_all } else { gf.mul(inv_all, prev) };
        inv_all = gf.mul(inv_all, d[i]);
    }
    out
}

/// The m-fold sumset of the delta-values of B, as a 2-bit mask.
fn delta_sumset(b: &Base, m: usize) -> [bool; 2] {
    let mut cur = [true, false]; // {0}
    for _ in 0..m {
        let mut nx = [false; 2];
        for s in 0..2 {
            if cur[s] {
                for d in 0..2 {
                    if b.dset[d] {
                        nx[s ^ d] = true;
                    }
                }
            }
        }
        cur = nx;
    }
    cur
}

// ---------------------------------------------------------------- decomposer

struct Decomposer<'a> {
    gf: &'a Gf,
    cv: Curve,
    base: &'a Base,
    m: usize,
    tset: Vec<u64>, // bitset over F (m = 3 only)
}

#[inline(always)]
fn bit_get(bs: &[u64], x: u64) -> bool {
    (bs[(x >> 6) as usize] >> (x & 63)) & 1 == 1
}
#[inline(always)]
fn bit_set(bs: &mut [u64], x: u64) {
    bs[(x >> 6) as usize] |= 1 << (x & 63);
}

impl<'a> Decomposer<'a> {
    fn new(gf: &'a Gf, cv: Curve, base: &'a Base, m: usize) -> Decomposer<'a> {
        assert!(m == 2 || m == 3);
        let mut tset = vec![];
        if m == 3 {
            let words = ((1u64 << gf.n) as usize + 63) / 64;
            tset = vec![0u64; words];
            let pts = &base.pts;
            let mm = pts.len();
            // doubling x(2P) = x^2 + b / x^2 for x != 0
            for &(x, _) in pts.iter() {
                if x != 0 {
                    let x2 = gf.sqr(x);
                    let v = x2 ^ gf.mul(cv.b, gf.inv(x2));
                    bit_set(&mut tset, v);
                }
            }
            for i in 0..mm {
                let (xi, yi) = pts[i];
                let rest = &pts[i + 1..];
                let d: Vec<u64> = rest.iter().map(|&(xj, _)| xi ^ xj).collect();
                let dinv = batch_inv(gf, &d);
                for (k, &(xj, yj)) in rest.iter().enumerate() {
                    let l1 = gf.mul(yi ^ yj, dinv[k]);
                    let xp = gf.sqr(l1) ^ l1 ^ cv.a ^ xi ^ xj;
                    let w = gf.mul(xj, dinv[k]);
                    let xm = xp ^ gf.sqr(w) ^ w;
                    bit_set(&mut tset, xp);
                    bit_set(&mut tset, xm);
                }
            }
        }
        Decomposer {
            gf,
            cv,
            base,
            m,
            tset,
        }
    }
    #[inline(always)]
    fn test(&self, x: u64) -> bool {
        if self.m == 2 {
            self.gf.c_lin.set(self.gf.c_lin.get() + self.base.ech.len() as u64);
            ech_reduce(&self.base.ech, x) == 0
        } else {
            self.gf.c_lin.set(self.gf.c_lin.get() + 1);
            bit_get(&self.tset, x)
        }
    }
    /// Exact: is R = (xr, yr) in the m-fold sumset of B?
    fn label(&self, xr: u64, yr: u64) -> bool {
        let gf = self.gf;
        let cv = self.cv;
        let pts = &self.base.pts;
        for chunk in pts.chunks(256) {
            let mut d: Vec<u64> = Vec::with_capacity(chunk.len());
            let mut special = false;
            for &(xi, _) in chunk {
                let v = xr ^ xi;
                if v == 0 {
                    special = true;
                    d.push(1);
                } else {
                    d.push(v);
                }
            }
            if special {
                // R = +-P_i for some i.
                if self.m == 3 {
                    return true; // R = P_i + P_j - P_j .. O is a pair sum
                }
                if xr != 0 {
                    let x2 = gf.sqr(xr);
                    let v = x2 ^ gf.mul(cv.b, gf.inv(x2));
                    if self.test(v) {
                        return true;
                    }
                }
            }
            let dinv = batch_inv(gf, &d);
            for (k, &(xi, yi)) in chunk.iter().enumerate() {
                if xr == xi {
                    continue;
                }
                let l1 = gf.mul(yr ^ yi, dinv[k]);
                let x1 = gf.sqr(l1) ^ l1 ^ cv.a ^ xr ^ xi;
                let w = gf.mul(xi, dinv[k]);
                let x2 = x1 ^ gf.sqr(w) ^ w;
                if self.test(x1) || self.test(x2) {
                    return true;
                }
            }
        }
        false
    }
}

// ----------------------------------------------------------------- features

const KPOW: [u32; 16] = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31];

struct Feat {
    d: usize,
    groups: Vec<u8>, // group id per column
}

const GROUP_NAMES: [&str; 7] = ["polybits", "normalbits", "powertraces", "hilbert90", "shiftedV", "halftrace", "context"];

fn feat_layout(n: usize) -> Feat {
    let mut groups = vec![];
    groups.extend(std::iter::repeat(0u8).take(n));
    groups.extend(std::iter::repeat(1u8).take(n));
    groups.extend(std::iter::repeat(2u8).take(32));
    groups.extend(std::iter::repeat(3u8).take(6));
    groups.extend(std::iter::repeat(4u8).take(7));
    groups.extend(std::iter::repeat(5u8).take(2 * n));
    groups.extend(std::iter::repeat(6u8).take(3));
    Feat { d: groups.len(), groups }
}

fn wt_mod_v(ech: &[(u32, u64)], x: u64) -> f32 {
    ech_reduce(ech, x).count_ones() as f32
}

/// Features of the target abscissa x_R relative to the curve and the base.
fn features(gf: &Gf, cv: &Curve, base: &Base, m: usize, xr: u64, out: &mut Vec<f32>) {
    let n = gf.n as usize;
    let one = |b: u64| -> f32 { (b & 1) as f32 };
    // polynomial-basis bits
    for i in 0..n {
        out.push(one(xr >> i));
    }
    // normal-basis bits
    gf.c_lin.set(gf.c_lin.get() + n as u64);
    let nb = gf.nb_coords(xr);
    for i in 0..n {
        out.push(one(nb >> i));
    }
    // absolute traces of x^k and x^-k, k odd <= 31
    let xi = gf.inv(xr);
    for &base_v in &[xr, xi] {
        let sq = gf.sqr(base_v);
        let mut p = base_v;
        for (idx, _k) in KPOW.iter().enumerate() {
            if idx > 0 {
                p = gf.mul(p, sq);
            }
            out.push(gf.tr(p) as f32);
        }
    }
    gf.c_lin.set(gf.c_lin.get() + 32);
    // Hilbert 90 indicators: Tr(.) = 0 means the element is z^2 + z
    let xi2 = gf.sqr(xi);
    let bx = gf.mul(cv.b, xi2);
    let c = xr ^ cv.a ^ bx;
    out.push(1.0 - gf.tr(xr) as f32);
    out.push(1.0 - gf.tr(xr ^ cv.a) as f32);
    out.push(1.0 - gf.tr(xi) as f32);
    out.push(1.0 - gf.tr(bx) as f32);
    out.push(1.0 - gf.tr(xr ^ cv.a ^ xi) as f32);
    out.push(1.0 - gf.tr(c) as f32);
    gf.c_lin.set(gf.c_lin.get() + 6);
    // shifted subspaces
    let ech = &base.ech;
    for s in [0u64, 1, cv.a, cv.a ^ 1] {
        out.push((ech_reduce(ech, xr ^ s) == 0) as u8 as f32);
    }
    out.push(wt_mod_v(ech, xr));
    out.push(wt_mod_v(ech, xr ^ 1));
    out.push(wt_mod_v(ech, xr ^ cv.a));
    gf.c_lin.set(gf.c_lin.get() + 7 * ech.len() as u64);
    // half-trace bits of x and of c
    gf.c_lin.set(gf.c_lin.get() + 2 * n as u64);
    let h1 = gf.half_trace(xr);
    let h2 = gf.half_trace(c);
    for i in 0..n {
        out.push(one(h1 >> i));
    }
    for i in 0..n {
        out.push(one(h2 >> i));
    }
    // context: the trace filter's own verdict, V inside the trace hyperplane, Tr(a)
    let ss = delta_sumset(base, m);
    let dr = gf.tr(xr ^ cv.a) as usize;
    out.push(ss[dr] as u8 as f32); // 1 = compatible (not refuted by arm b)
    out.push(base.tv0 as u8 as f32);
    out.push(gf.tr(cv.a) as f32);
}

// ------------------------------------------------------------------- models

fn auc(scores: &[f64], labels: &[u8]) -> f64 {
    // Mann-Whitney U with average ranks; positive class = 1
    let n = scores.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| scores[a].partial_cmp(&scores[b]).unwrap());
    let (mut npos, mut nneg) = (0f64, 0f64);
    let mut rank_sum = 0f64;
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j + 1 < n && scores[idx[j + 1]] == scores[idx[i]] {
            j += 1;
        }
        let avg = (i + j) as f64 / 2.0 + 1.0;
        for k in i..=j {
            if labels[idx[k]] == 1 {
                rank_sum += avg;
                npos += 1.0;
            } else {
                nneg += 1.0;
            }
        }
        i = j + 1;
    }
    if npos == 0.0 || nneg == 0.0 {
        return f64::NAN;
    }
    (rank_sum - npos * (npos + 1.0) / 2.0) / (npos * nneg)
}

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

struct Mat<'a> {
    x: &'a [f32],
    d: usize,
}
impl<'a> Mat<'a> {
    fn row(&self, i: usize) -> &[f32] {
        &self.x[i * self.d..(i + 1) * self.d]
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Model {
    Lr,
    Gbt,
}

struct Lr {
    w: Vec<f64>,
    b: f64,
    mu: Vec<f64>,
    isd: Vec<f64>,
}

fn lr_fit(m: &Mat, y: &[u8], fit: &[usize], seed: u64) -> Lr {
    let d = m.d;
    let nf = fit.len() as f64;
    let mut mu = vec![0f64; d];
    let mut var = vec![0f64; d];
    for &i in fit {
        for (j, &v) in m.row(i).iter().enumerate() {
            mu[j] += v as f64;
        }
    }
    for v in mu.iter_mut() {
        *v /= nf;
    }
    for &i in fit {
        for (j, &v) in m.row(i).iter().enumerate() {
            let t = v as f64 - mu[j];
            var[j] += t * t;
        }
    }
    let isd: Vec<f64> = var
        .iter()
        .map(|&s| {
            let sd = (s / nf).sqrt();
            1.0 / sd.max(0.05)
        })
        .collect();
    let mut w = vec![0f64; d];
    let mut b = 0f64;
    let (mut mw, mut vw) = (vec![0f64; d], vec![0f64; d]);
    let (mut mb, mut vb) = (0f64, 0f64);
    let (b1, b2, eps, lr, l2) = (0.9f64, 0.999f64, 1e-8, 0.02f64, 1e-4);
    let mut order: Vec<usize> = fit.to_vec();
    let mut rng = Rng(seed);
    let bs = 128usize;
    let mut t = 0f64;
    let mut gw = vec![0f64; d];
    let mut z = vec![0f64; d];
    for _epoch in 0..30 {
        for i in (1..order.len()).rev() {
            let j = rng.below(i as u64 + 1) as usize;
            order.swap(i, j);
        }
        for batch in order.chunks(bs) {
            gw.iter_mut().for_each(|g| *g = 0.0);
            let mut gb = 0f64;
            for &i in batch {
                let r = m.row(i);
                let mut s = b;
                for j in 0..d {
                    let v = ((r[j] as f64 - mu[j]) * isd[j]).clamp(-8.0, 8.0);
                    z[j] = v;
                    s += w[j] * v;
                }
                let e = sigmoid(s) - y[i] as f64;
                for j in 0..d {
                    gw[j] += e * z[j];
                }
                gb += e;
            }
            let k = batch.len() as f64;
            t += 1.0;
            let c1 = 1.0 - b1.powf(t);
            let c2 = 1.0 - b2.powf(t);
            for j in 0..d {
                let g = gw[j] / k + l2 * w[j];
                mw[j] = b1 * mw[j] + (1.0 - b1) * g;
                vw[j] = b2 * vw[j] + (1.0 - b2) * g * g;
                w[j] -= lr * (mw[j] / c1) / ((vw[j] / c2).sqrt() + eps);
            }
            let g = gb / k;
            mb = b1 * mb + (1.0 - b1) * g;
            vb = b2 * vb + (1.0 - b2) * g * g;
            b -= lr * (mb / c1) / ((vb / c2).sqrt() + eps);
        }
    }
    Lr { w, b, mu, isd }
}

fn lr_score(l: &Lr, r: &[f32]) -> f64 {
    let mut s = l.b;
    for j in 0..r.len() {
        s += l.w[j] * ((r[j] as f64 - l.mu[j]) * l.isd[j]).clamp(-8.0, 8.0);
    }
    s
}

const GBT_ROUNDS: usize = 100;
const GBT_DEPTH: usize = 3;
const GBT_ETA: f64 = 0.1;
const GBT_LAMBDA: f64 = 1.0;
const MAXBIN: usize = 16;

struct Tree {
    // internal nodes in level order: (feature, threshold, always_left)
    nodes: Vec<(usize, f32, bool)>,
    leaves: Vec<f64>,
}
struct Gbt {
    base: f64,
    trees: Vec<Tree>,
    splits_by_feature: Vec<u32>,
}

fn make_cuts(m: &Mat, fit: &[usize], j: usize) -> Vec<f32> {
    let mut v: Vec<f32> = fit.iter().map(|&i| m.row(i)[j]).collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut u: Vec<f32> = v.clone();
    u.dedup();
    if u.len() <= MAXBIN {
        return u[1..].to_vec();
    }
    let mut cuts = vec![];
    for q in 1..MAXBIN {
        let c = v[v.len() * q / MAXBIN];
        if cuts.last().map_or(true, |&l| c > l) && c > u[0] {
            cuts.push(c);
        }
    }
    cuts
}

fn bin_of(cuts: &[f32], v: f32) -> u8 {
    cuts.iter().take_while(|&&c| c <= v).count() as u8
}

fn gbt_fit(m: &Mat, y: &[u8], fit: &[usize]) -> Gbt {
    let d = m.d;
    let nfit = fit.len();
    let cuts: Vec<Vec<f32>> = (0..d).map(|j| make_cuts(m, fit, j)).collect();
    let mut bins = vec![0u8; nfit * d];
    for (k, &i) in fit.iter().enumerate() {
        let r = m.row(i);
        for j in 0..d {
            bins[k * d + j] = bin_of(&cuts[j], r[j]);
        }
    }
    let pos = fit.iter().filter(|&&i| y[i] == 1).count() as f64;
    let p0 = (pos / nfit as f64).clamp(1e-6, 1.0 - 1e-6);
    let base = (p0 / (1.0 - p0)).ln();
    let mut margin = vec![base; nfit];
    let mut trees = Vec::with_capacity(GBT_ROUNDS);
    let mut splits_by_feature = vec![0u32; d];
    let nb = MAXBIN + 1;
    for _round in 0..GBT_ROUNDS {
        let mut g = vec![0f64; nfit];
        let mut h = vec![0f64; nfit];
        for k in 0..nfit {
            let p = sigmoid(margin[k]);
            g[k] = p - y[fit[k]] as f64;
            h[k] = (p * (1.0 - p)).max(1e-9);
        }
        let mut node_of = vec![0usize; nfit];
        let mut nodes: Vec<(usize, f32, bool)> = vec![];
        let mut node_t: Vec<usize> = vec![];
        for level in 0..GBT_DEPTH {
            let nn = 1usize << level;
            let mut hist = vec![0f64; nn * d * nb * 2];
            for k in 0..nfit {
                let nid = node_of[k];
                let row = &bins[k * d..(k + 1) * d];
                let base_i = nid * d * nb;
                for j in 0..d {
                    let o = (base_i + j * nb + row[j] as usize) * 2;
                    hist[o] += g[k];
                    hist[o + 1] += h[k];
                }
            }
            let mut choice: Vec<(usize, usize, bool)> = vec![(0, 0, true); nn];
            for nid in 0..nn {
                let (mut gt, mut ht) = (0f64, 0f64);
                for bi in 0..nb {
                    gt += hist[(nid * d * nb + bi) * 2];
                    ht += hist[(nid * d * nb + bi) * 2 + 1];
                }
                let parent = gt * gt / (ht + GBT_LAMBDA);
                let mut best = (1e-9f64, 0usize, 0usize, false);
                for j in 0..d {
                    let k = cuts[j].len();
                    let (mut gl, mut hl) = (0f64, 0f64);
                    for t in 0..k {
                        gl += hist[(nid * d * nb + j * nb + t) * 2];
                        hl += hist[(nid * d * nb + j * nb + t) * 2 + 1];
                        let (gr, hr) = (gt - gl, ht - hl);
                        if hl < 2.0 || hr < 2.0 {
                            continue;
                        }
                        let gain = gl * gl / (hl + GBT_LAMBDA) + gr * gr / (hr + GBT_LAMBDA) - parent;
                        if gain > best.0 {
                            best = (gain, j, t, true);
                        }
                    }
                }
                if best.3 {
                    choice[nid] = (best.1, best.2, false);
                    splits_by_feature[best.1] += 1;
                }
            }
            for nid in 0..nn {
                let (j, t, al) = choice[nid];
                let thr = if al { 0.0 } else { cuts[j][t] };
                nodes.push((j, thr, al));
                node_t.push(t);
            }
            for k in 0..nfit {
                let nid = node_of[k];
                let (j, t, al) = choice[nid];
                let right = !al && (bins[k * d + j] as usize) > t;
                node_of[k] = 2 * nid + right as usize;
            }
        }
        let nl = 1usize << GBT_DEPTH;
        let (mut gl, mut hl) = (vec![0f64; nl], vec![0f64; nl]);
        for k in 0..nfit {
            gl[node_of[k]] += g[k];
            hl[node_of[k]] += h[k];
        }
        let leaves: Vec<f64> = (0..nl)
            .map(|i| -gl[i] / (hl[i] + GBT_LAMBDA) * GBT_ETA)
            .collect();
        for k in 0..nfit {
            margin[k] += leaves[node_of[k]];
        }
        trees.push(Tree { nodes, leaves });
    }
    Gbt {
        base,
        trees,
        splits_by_feature,
    }
}

fn gbt_score(gb: &Gbt, r: &[f32]) -> f64 {
    let mut s = gb.base;
    for t in &gb.trees {
        let mut nid = 0usize;
        let mut off = 0usize;
        for level in 0..GBT_DEPTH {
            let (j, thr, al) = t.nodes[off + nid];
            let right = !al && r[j] >= thr;
            nid = 2 * nid + right as usize;
            off += 1 << level;
        }
        s += t.leaves[nid];
    }
    s
}

// ---------------------------------------------------------------- pipeline

const N_BASES: usize = 10;
const BASE_KIND: [char; N_BASES] = ['T', 'T', 'T', 'T', 'G', 'G', 'G', 'P', 'P', 'P'];
const BASE_FOLD: [usize; N_BASES] = [0, 1, 2, 3, 0, 2, 4, 1, 3, 4];
const N_FOLDS: usize = 5;

struct FoldOut {
    score: Vec<f64>,
    thr_cal: [f64; N_FOLDS],
    min_pos_test: [f64; N_FOLDS],
    gbt_group_splits: [u64; 7],
}

fn run_folds(
    model: Model,
    x: &[f32],
    d: usize,
    groups: &[u8],
    y: &[u8],
    base_of: &[usize],
    seed: u64,
) -> FoldOut {
    let n = y.len();
    let m = Mat { x, d };
    let mut score = vec![0f64; n];
    let mut thr_cal = [f64::NEG_INFINITY; N_FOLDS];
    let mut min_pos_test = [f64::INFINITY; N_FOLDS];
    let mut gsp = [0u64; 7];
    for f in 0..N_FOLDS {
        let test: Vec<usize> = (0..n).filter(|&i| BASE_FOLD[base_of[i]] == f).collect();
        let mut fit = vec![];
        let mut cal = vec![];
        for i in 0..n {
            if BASE_FOLD[base_of[i]] == f {
                continue;
            }
            if mix(seed ^ 0xCA11, i as u64) % 4 == 0 {
                cal.push(i);
            } else {
                fit.push(i);
            }
        }
        let scorer: Box<dyn Fn(&[f32]) -> f64> = match model {
            Model::Lr => {
                let l = lr_fit(&m, y, &fit, mix(seed, 1000 + f as u64));
                Box::new(move |r| lr_score(&l, r))
            }
            Model::Gbt => {
                let g = gbt_fit(&m, y, &fit);
                for (j, &c) in g.splits_by_feature.iter().enumerate() {
                    gsp[groups[j] as usize] += c as u64;
                }
                Box::new(move |r| gbt_score(&g, r))
            }
        };
        let mut thr = f64::INFINITY;
        for &i in &cal {
            if y[i] == 1 {
                thr = thr.min(scorer(m.row(i)));
            }
        }
        if thr == f64::INFINITY {
            thr = f64::NEG_INFINITY;
        }
        thr_cal[f] = thr;
        for &i in &test {
            let s = scorer(m.row(i));
            score[i] = s;
            if y[i] == 1 {
                min_pos_test[f] = min_pos_test[f].min(s);
            }
        }
    }
    FoldOut {
        score,
        thr_cal,
        min_pos_test,
        gbt_group_splits: gsp,
    }
}

struct GStat {
    neg: usize,
    pos: usize,
    ref_neg: usize, // calibrated threshold
    fr: usize,
    roc0_neg: usize,
    union_neg: usize,
    auc: f64,
    rc: Vec<u8>, // per negative: refuted by c (calibrated)
    rb: Vec<u8>, // per negative: refuted by b
}

fn group_stats(
    out: &FoldOut,
    y: &[u8],
    base_of: &[usize],
    sel: &dyn Fn(char) -> bool,
    b_ref: &[bool],
) -> GStat {
    let n = y.len();
    let mut st = GStat {
        neg: 0,
        pos: 0,
        ref_neg: 0,
        fr: 0,
        roc0_neg: 0,
        union_neg: 0,
        auc: f64::NAN,
        rc: vec![],
        rb: vec![],
    };
    let mut aucs = vec![];
    for f in 0..N_FOLDS {
        let mut sc = vec![];
        let mut lb = vec![];
        for i in 0..n {
            if BASE_FOLD[base_of[i]] != f || !sel(BASE_KIND[base_of[i]]) {
                continue;
            }
            let s = out.score[i];
            sc.push(s);
            lb.push(y[i]);
            let refuted = s < out.thr_cal[f];
            if y[i] == 1 {
                st.pos += 1;
                if refuted {
                    st.fr += 1;
                }
            } else {
                st.neg += 1;
                if refuted {
                    st.ref_neg += 1;
                }
                if s < out.min_pos_test[f] {
                    st.roc0_neg += 1;
                }
                if refuted || b_ref[i] {
                    st.union_neg += 1;
                }
                st.rc.push(refuted as u8);
                st.rb.push(b_ref[i] as u8);
            }
        }
        let a = auc(&sc, &lb);
        if a.is_finite() {
            aucs.push(a);
        }
    }
    if !aucs.is_empty() {
        st.auc = aucs.iter().sum::<f64>() / aucs.len() as f64;
    }
    st
}

fn bootstrap_delta(rc: &[u8], rb: &[u8], credit: bool, seed: u64) -> (f64, f64, f64) {
    let n = rc.len();
    if n == 0 {
        return (f64::NAN, f64::NAN, f64::NAN);
    }
    let diff: Vec<f64> = (0..n)
        .map(|i| (if credit { rc[i] as f64 } else { 0.0 }) - rb[i] as f64)
        .collect();
    let mean = diff.iter().sum::<f64>() / n as f64;
    let mut rng = Rng(seed);
    let mut ms = vec![];
    for _ in 0..1000 {
        let mut s = 0f64;
        for _ in 0..n {
            s += diff[rng.below(n as u64) as usize];
        }
        ms.push(s / n as f64);
    }
    ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (mean, ms[25], ms[974])
}

fn jnum(x: f64) -> String {
    if x.is_finite() {
        format!("{:.6}", x)
    } else {
        "null".to_string()
    }
}

struct CellParams {
    n: u32,
    m: usize,
    ell: usize, // 0 = auto
    per_base: usize,
    perms: usize,
    pilot: usize,
    pop: char, // 'U' uniform over E(F); 'S' uniform over 2E(F) (delta = 0), the subgroup-target population
}

fn lambda_formula(n: u32, m: usize, ell: usize) -> f64 {
    let fact = (1..=m).product::<usize>() as f64;
    (2f64).powi((m * ell) as i32) / (fact * (2f64).powi(n as i32))
}

fn formula_ell(n: u32, m: usize) -> usize {
    let target = std::f64::consts::LN_2;
    let mut best = (f64::INFINITY, 3usize);
    for ell in 3..(n as usize - 1) {
        let r = (lambda_formula(n, m, ell) / target).ln().abs();
        if r < best.0 {
            best = (r, ell);
        }
    }
    best.1
}

/// The selftest draws a in {0, 1}; the measured cells fix a = 0 (see `make_cell_curve`).
fn make_curve(gf: &Gf, seed: u64) -> Curve {
    let mut rng = Rng(seed);
    let a = rng.next() & 1;
    make_curve_a(gf, &mut rng, a)
}

/// Measured cells use the Koblitz-form family y^2 + xy = x^3 + b (a = 0), b random non-zero.
fn make_cell_curve(gf: &Gf, seed: u64) -> Curve {
    let mut rng = Rng(seed);
    make_curve_a(gf, &mut rng, 0)
}

fn make_curve_a(gf: &Gf, rng: &mut Rng, a: u64) -> Curve {
    loop {
        let b = rng.next() & gf.mask;
        if b != 0 {
            return Curve { a, b };
        }
    }
}

fn random_target(gf: &Gf, cv: &Curve, rng: &mut Rng, pop: char) -> (u64, u64) {
    loop {
        let x = rng.next() & gf.mask;
        if x == 0 {
            continue;
        }
        if pop == 'S' && gf.tr(x ^ cv.a) == 1 {
            continue;
        }
        if let Some((x, y)) = point_at(gf, cv, x) {
            let y = if rng.next() & 1 == 1 { y ^ x } else { y };
            return (x, y);
        }
    }
}

fn pos_fraction(gf: &Gf, cv: &Curve, kind: char, ell: usize, m: usize, count: usize, seed: u64, pop: char) -> f64 {
    let mut rng = Rng(seed);
    let base = build_base(gf, cv, kind, ell, &mut rng);
    let dec = Decomposer::new(gf, *cv, &base, m);
    let mut pos = 0;
    for _ in 0..count {
        let (x, y) = random_target(gf, cv, &mut rng, pop);
        if dec.label(x, y) {
            pos += 1;
        }
    }
    pos as f64 / count as f64
}

fn run_cell(p: &CellParams) {
    let t0 = Instant::now();
    let gf = Gf::new(p.n);
    let n = p.n as usize;
    POPCH.with(|c| c.set(p.pop));
    let cell_seed = 20261018u64 ^ ((p.n as u64) << 40) ^ ((p.m as u64) << 32) ^ if p.pop == 'S' { 1 << 24 } else { 0 };
    let curve_seed = 20261018u64 ^ ((p.n as u64) << 40) ^ ((p.m as u64) << 32);
    let cv = make_cell_curve(&gf, mix(curve_seed, 1));
    // ---- choose ell
    let ell0 = formula_ell(p.n, p.m);
    let mut ell = p.ell;
    if ell == 0 {
        let mut best = (f64::INFINITY, ell0);
        let cands: Vec<usize> = [ell0.saturating_sub(1), ell0, ell0 + 1]
            .iter()
            .copied()
            .filter(|&l| l >= 3 && l + 2 < n)
            .collect();
        for &l in &cands {
            let half = p.pilot / 2;
            let f1 = pos_fraction(&gf, &cv, 'G', l, p.m, half, mix(cell_seed, 7000 + l as u64), p.pop);
            let f2 = pos_fraction(&gf, &cv, 'G', l, p.m, half, mix(cell_seed, 8000 + l as u64), p.pop);
            let f = (f1 + f2) / 2.0;
            emitj!(
                "{{\"kind\":\"pilot\",\"n\":{},\"m\":{},\"l\":{},\"formula_lambda\":{},\"pos_frac\":{}}}",
                p.n,
                p.m,
                l,
                jnum(lambda_formula(p.n, p.m, l)),
                jnum(f)
            );
            let dist = (f - 0.5).abs();
            if dist < best.0 - 1e-12 {
                best = (dist, l);
            }
        }
        ell = best.1;
    }
    emitj!(
        "{{\"kind\":\"cell\",\"n\":{},\"m\":{},\"l\":{},\"a\":{},\"b\":{},\"seed\":{},\"per_base\":{},\"perms\":{},\"formula_l\":{},\"hw_clmul\":{}}}",
        p.n, p.m, ell, cv.a, cv.b, cell_seed, p.per_base, p.perms, ell0, gf.hw
    );
    // ---- bases, targets, labels, features
    let lay = feat_layout(n);
    let d = lay.d;
    let total = N_BASES * p.per_base;
    let mut xs: Vec<f32> = Vec::with_capacity(total * d);
    let mut y: Vec<u8> = Vec::with_capacity(total);
    let mut base_of: Vec<usize> = Vec::with_capacity(total);
    let mut b_ref: Vec<bool> = Vec::with_capacity(total);
    let mut tr_x: Vec<u8> = vec![]; // Tr(x_R)
    let mut tr_xi: Vec<u8> = vec![]; // Tr(1/x_R)
    let mut tr_x3: Vec<u8> = vec![]; // Tr(x_R^3)
    let mut tr_x5: Vec<u8> = vec![]; // Tr(x_R^5)
    let mut false_b = 0usize;
    let (mut dec_neg_ops, mut dec_neg_lin, mut dec_neg_cnt) = (0u64, 0u64, 0u64);
    let (mut dec_pos_ops, mut dec_pos_lin, mut dec_pos_cnt) = (0u64, 0u64, 0u64);
    let mut feat_ops = OpSnap::default();
    for bi in 0..N_BASES {
        let kind = BASE_KIND[bi];
        let mut brng = Rng(mix(cell_seed, 100 + bi as u64));
        let base = build_base(&gf, &cv, kind, ell, &mut brng);
        let ss = delta_sumset(&base, p.m);
        let dec = Decomposer::new(&gf, cv, &base, p.m);
        let mut trng = Rng(mix(cell_seed, 500 + bi as u64));
        let mut npos = 0usize;
        let mut nb_ref = 0usize;
        for _ in 0..p.per_base {
            let (xr, yr) = random_target(&gf, &cv, &mut trng, p.pop);
            let s0 = gf.snap();
            let lab = dec.label(xr, yr);
            let s1 = gf.snap().sub(s0);
            if lab {
                dec_pos_ops += s1.mul + s1.sqr;
                dec_pos_lin += s1.lin;
                dec_pos_cnt += 1;
                npos += 1;
            } else {
                dec_neg_ops += s1.mul + s1.sqr;
                dec_neg_lin += s1.lin;
                dec_neg_cnt += 1;
            }
            let dr = gf.tr(xr ^ cv.a) as usize;
            let refuted_b = !ss[dr];
            if refuted_b {
                nb_ref += 1;
                if lab {
                    false_b += 1;
                }
            }
            let s2 = gf.snap();
            features(&gf, &cv, &base, p.m, xr, &mut xs);
            feat_ops = OpSnap {
                mul: feat_ops.mul + gf.snap().sub(s2).mul,
                sqr: feat_ops.sqr + gf.snap().sub(s2).sqr,
                lin: feat_ops.lin + gf.snap().sub(s2).lin,
            };
            y.push(lab as u8);
            base_of.push(bi);
            b_ref.push(refuted_b);
            tr_x.push(gf.tr(xr) as u8);
            let x2 = gf.sqr(xr);
            let x3 = gf.mul(x2, xr);
            let x5 = gf.mul(gf.sqr(x2), xr);
            tr_xi.push(gf.tr(gf.inv(xr)) as u8);
            tr_x3.push(gf.tr(x3) as u8);
            tr_x5.push(gf.tr(x5) as u8);
        }
        emitj!(
            "{{\"kind\":\"base\",\"n\":{},\"m\":{},\"l\":{},\"idx\":{},\"type\":\"{}\",\"classes\":{},\"tv0\":{},\"dset0\":{},\"dset1\":{},\"sumset0\":{},\"sumset1\":{},\"targets\":{},\"pos\":{},\"pos_frac\":{},\"b_refutes\":{}}}",
            p.n, p.m, ell, bi, kind, base.pts.len(), base.tv0, base.dset[0], base.dset[1], ss[0], ss[1],
            p.per_base, npos, jnum(npos as f64 / p.per_base as f64), nb_ref
        );
    }
    assert_eq!(xs.len(), total * d);
    let t_label = t0.elapsed().as_secs_f64();
    emitj!(
        "{{\"kind\":\"label_summary\",\"n\":{},\"m\":{},\"l\":{},\"targets\":{},\"pos\":{},\"false_refutations_b\":{},\"seconds\":{}}}",
        p.n, p.m, ell, total, y.iter().map(|&v| v as usize).sum::<usize>(), false_b, jnum(t_label)
    );
    // ---- cost lines (counted work)
    let tn = total as f64;
    let dn = dec_neg_cnt.max(1) as f64;
    let dp = dec_pos_cnt.max(1) as f64;
    emitj!(
        "{{\"kind\":\"cost\",\"n\":{},\"m\":{},\"l\":{},\"feat_mul\":{},\"feat_sqr\":{},\"feat_lin\":{},\"lr_model_ops\":{},\"gbt_model_ops\":{},\"dec_neg_field_ops\":{},\"dec_neg_lin\":{},\"dec_pos_field_ops\":{},\"dec_pos_lin\":{},\"neg_targets\":{},\"pos_targets\":{}}}",
        p.n, p.m, ell,
        jnum(feat_ops.mul as f64 / tn), jnum(feat_ops.sqr as f64 / tn), jnum(feat_ops.lin as f64 / tn),
        d * 2, GBT_ROUNDS * (GBT_DEPTH + 1),
        jnum(dec_neg_ops as f64 / dn), jnum(dec_neg_lin as f64 / dn),
        jnum(dec_pos_ops as f64 / dp), jnum(dec_pos_lin as f64 / dp),
        dec_neg_cnt, dec_pos_cnt
    );
    // ---- the learning arms
    let selectors: [(&str, Box<dyn Fn(char) -> bool>); 4] = [
        ("ALL", Box::new(|_| true)),
        ("T", Box::new(|k| k == 'T')),
        ("G", Box::new(|k| k == 'G')),
        ("P", Box::new(|k| k == 'P')),
    ];
    let models = [(Model::Lr, "lr"), (Model::Gbt, "gbt")];
    for &(model, mname) in &models {
        let tm = Instant::now();
        let real = run_folds(model, &xs, d, &lay.groups, &y, &base_of, mix(cell_seed, 9000));
        // null: within-base label permutations
        let mut null: Vec<Vec<(f64, f64)>> = vec![vec![]; selectors.len()]; // per selector: (auc, roc0 rate)
        for pi in 0..p.perms {
            let mut yp = y.clone();
            let mut prng = Rng(mix(cell_seed, 20000 + pi as u64));
            for bi in 0..N_BASES {
                let idx: Vec<usize> = (0..total).filter(|&i| base_of[i] == bi).collect();
                let mut labs: Vec<u8> = idx.iter().map(|&i| y[i]).collect();
                for i in (1..labs.len()).rev() {
                    let j = prng.below(i as u64 + 1) as usize;
                    labs.swap(i, j);
                }
                for (k, &i) in idx.iter().enumerate() {
                    yp[i] = labs[k];
                }
            }
            let out = run_folds(model, &xs, d, &lay.groups, &yp, &base_of, mix(cell_seed, 30000 + pi as u64));
            for (si, (_, sel)) in selectors.iter().enumerate() {
                let st = group_stats(&out, &yp, &base_of, sel.as_ref(), &b_ref);
                let r0 = if st.neg > 0 { st.roc0_neg as f64 / st.neg as f64 } else { f64::NAN };
                null[si].push((st.auc, r0));
            }
        }
        for (si, (sname, sel)) in selectors.iter().enumerate() {
            let st = group_stats(&real, &y, &base_of, sel.as_ref(), &b_ref);
            let neg = st.neg.max(1) as f64;
            let rate_b = st.rb.iter().map(|&v| v as f64).sum::<f64>() / neg;
            let credit = st.fr == 0;
            let rate_c = if credit { st.ref_neg as f64 / neg } else { 0.0 };
            let (dl, lo, hi) = bootstrap_delta(&st.rc, &st.rb, credit, mix(cell_seed, 40000 + si as u64 * 7 + mname.len() as u64));
            let rate_u = st.union_neg as f64 / neg;
            let roc0 = st.roc0_neg as f64 / neg;
            let na: Vec<f64> = null[si].iter().map(|v| v.0).filter(|v| v.is_finite()).collect();
            let nr: Vec<f64> = null[si].iter().map(|v| v.1).filter(|v| v.is_finite()).collect();
            let amax = na.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let rmax = nr.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let amean = if na.is_empty() { f64::NAN } else { na.iter().sum::<f64>() / na.len() as f64 };
            let rmean = if nr.is_empty() { f64::NAN } else { nr.iter().sum::<f64>() / nr.len() as f64 };
            emitj!(
                "{{\"kind\":\"summary\",\"n\":{},\"m\":{},\"l\":{},\"model\":\"{}\",\"type\":\"{}\",\"neg\":{},\"pos\":{},\"rate_b\":{},\"rate_c\":{},\"fr\":{},\"delta\":{},\"delta_lo\":{},\"delta_hi\":{},\"rate_union\":{},\"roc0\":{},\"auc\":{},\"null_auc_max\":{},\"null_auc_mean\":{},\"null_roc0_max\":{},\"null_roc0_mean\":{},\"perms\":{}}}",
                p.n, p.m, ell, mname, sname, st.neg, st.pos, jnum(rate_b), jnum(rate_c), st.fr,
                jnum(dl), jnum(lo), jnum(hi), jnum(rate_u), jnum(roc0), jnum(st.auc),
                jnum(amax), jnum(amean), jnum(rmax), jnum(rmean), p.perms
            );
        }
        if model == Model::Gbt {
            let g = real.gbt_group_splits;
            let tot: u64 = g.iter().sum();
            let mut s = String::new();
            for (i, name) in GROUP_NAMES.iter().enumerate() {
                s.push_str(&format!(",\"splits_{}\":{}", name, g[i]));
            }
            emitj!(
                "{{\"kind\":\"gbt_splits\",\"n\":{},\"m\":{},\"l\":{},\"total\":{}{}}}",
                p.n, p.m, ell, tot, s
            );
        }
        // ---- positive controls with this model
        let mut yc1 = vec![0u8; total]; // planted: Tr(x_R) = 0
        let mut yc2 = vec![0u8; total]; // planted interaction: if Tr(1/x) = 0 then Tr(x^3) = 0 else Tr(x^5) = 0
        for i in 0..total {
            // planted label: Tr(x_R) = 0 on U; on S Tr(x_R) = Tr(a) is constant, so Tr(1/x_R) = 0
            yc1[i] = (if p.pop == 'U' { tr_x[i] == 0 } else { tr_xi[i] == 0 }) as u8;
            yc2[i] = (if tr_xi[i] == 0 { tr_x3[i] == 0 } else { tr_x5[i] == 0 }) as u8;
        }
        for (cname, yc) in [("planted_trace", &yc1), ("planted_selector", &yc2)] {
            let out = run_folds(model, &xs, d, &lay.groups, yc, &base_of, mix(cell_seed, 50000));
            let st = group_stats(&out, yc, &base_of, &|_| true, &b_ref);
            let neg = st.neg.max(1) as f64;
            emitj!(
                "{{\"kind\":\"control\",\"n\":{},\"m\":{},\"l\":{},\"model\":\"{}\",\"name\":\"{}\",\"neg\":{},\"pos\":{},\"auc\":{},\"fr\":{},\"rate_cal\":{},\"roc0\":{}}}",
                p.n, p.m, ell, mname, cname, st.neg, st.pos, jnum(st.auc), st.fr,
                jnum(st.ref_neg as f64 / neg), jnum(st.roc0_neg as f64 / neg)
            );
        }
        emitj!(
            "{{\"kind\":\"timing\",\"n\":{},\"m\":{},\"model\":\"{}\",\"seconds\":{}}}",
            p.n, p.m, mname, jnum(tm.elapsed().as_secs_f64())
        );
    }
    emitj!(
        "{{\"kind\":\"done\",\"n\":{},\"m\":{},\"l\":{},\"seconds\":{},\"rss_peak_kb\":{}}}",
        p.n, p.m, ell, jnum(t0.elapsed().as_secs_f64()), rss_peak_kb()
    );
}

// ------------------------------------------------------------------ selftest

fn ok(name: &str, cond: bool, fails: &mut usize) {
    println!("{} {}", if cond { "PASS" } else { "FAIL" }, name);
    if !cond {
        *fails += 1;
    }
}

fn selftest() -> usize {
    let mut fails = 0usize;
    // 1. field arithmetic
    for &n in &[11u32, 17, 19, 23, 29] {
        let gf = Gf::new(n);
        let mut rng = Rng(n as u64);
        let mut good = true;
        for _ in 0..2000 {
            let a = rng.next() & gf.mask;
            let b = rng.next() & gf.mask;
            let c = rng.next() & gf.mask;
            good &= gf.mul(a, b) == gf.mul_soft(a, b);
            good &= gf.mul(a, b) == gf.mul(b, a);
            good &= gf.mul(gf.mul(a, b), c) == gf.mul(a, gf.mul(b, c));
            good &= gf.mul(a, b ^ c) == gf.mul(a, b) ^ gf.mul(a, c);
            good &= gf.sqr(a) == gf.mul(a, a);
            if a != 0 {
                good &= gf.mul(a, gf.inv(a)) == 1;
            }
            good &= gf.tr(a) == gf.tr(gf.sqr(a));
            good &= gf.tr(a ^ b) == gf.tr(a) ^ gf.tr(b);
            let h = gf.half_trace(a);
            good &= gf.sqr(h) ^ h == a ^ gf.tr(a);
        }
        ok(&format!("field n={n}: mul/inv/trace/half-trace identities, clmul == soft"), good, &mut fails);
        // normal basis round trip
        let mut good = true;
        for _ in 0..200 {
            let x = rng.next() & gf.mask;
            let nb = gf.nb_coords(x);
            let mut s = 0u64;
            // alpha is recovered from coords of the basis: reconstruct via solving is
            // implicit; check linearity and bijectivity on random pairs instead
            let y = rng.next() & gf.mask;
            good &= gf.nb_coords(x ^ y) == gf.nb_coords(x) ^ gf.nb_coords(y);
            s ^= nb;
            let _ = s;
        }
        // bijectivity: images of the basis vectors are independent
        let mut ech = vec![];
        let mut rank = 0;
        for i in 0..n {
            if ech_insert(&mut ech, gf.nb_coords(1u64 << i)) {
                rank += 1;
            }
        }
        good &= rank == n;
        ok(&format!("field n={n}: normal-basis coordinate map is a linear bijection"), good, &mut fails);
    }
    // 2. curve and group law, n = 11
    let gf = Gf::new(11);
    for seed in 0..3u64 {
        let cv = make_curve(&gf, mix(0xC0DE, seed));
        let mut all_pts: Vec<(u64, u64)> = vec![];
        for x in 0..(1u64 << 11) {
            if let Some((x, y)) = point_at(&gf, &cv, x) {
                all_pts.push((x, y));
                if x != 0 {
                    all_pts.push((x, y ^ x));
                }
            }
        }
        let order = all_pts.len() as u64 + 1;
        let mut good = true;
        let mut rng = Rng(77 + seed);
        for _ in 0..300 {
            let p = all_pts[rng.below(all_pts.len() as u64) as usize];
            let q = all_pts[rng.below(all_pts.len() as u64) as usize];
            let r = all_pts[rng.below(all_pts.len() as u64) as usize];
            good &= on_curve(&gf, &cv, p);
            let s = add(&gf, &cv, Some(p), Some(q));
            if let Some(sp) = s {
                good &= on_curve(&gf, &cv, sp);
            }
            good &= add(&gf, &cv, add(&gf, &cv, Some(p), Some(q)), Some(r))
                == add(&gf, &cv, Some(p), add(&gf, &cv, Some(q), Some(r)));
            good &= add(&gf, &cv, Some(p), neg(Some(p))).is_none();
            good &= scalar_mul(&gf, &cv, Some(p), order).is_none();
        }
        ok(&format!("curve a={} b={}: {} points, group-law identities, N*P = O", cv.a, cv.b, order), good, &mut fails);
        // Hasse
        let t = (1i64 << 11) + 1 - order as i64;
        ok(&format!("curve a={}: Hasse bound |t| = {} <= 2*sqrt(2^11)", cv.a, t.abs()), (t.abs() as f64) <= 2.0 * (2048f64).sqrt(), &mut fails);
        // the trace homomorphism delta(P+Q) = delta(P) + delta(Q)
        let mut good = true;
        for _ in 0..500 {
            let p = all_pts[rng.below(all_pts.len() as u64) as usize];
            let q = all_pts[rng.below(all_pts.len() as u64) as usize];
            if let Some((xs, _)) = add(&gf, &cv, Some(p), Some(q)) {
                good &= delta(&gf, &cv, xs) == delta(&gf, &cv, p.0) ^ delta(&gf, &cv, q.0);
            }
        }
        ok(&format!("curve a={}: delta(P) = Tr(x + a) is additive", cv.a), good, &mut fails);
        // the population S (delta = 0) is exactly 2E(F) minus O
        let doubles: std::collections::HashSet<Pt> = all_pts.iter().map(|&q| add(&gf, &cv, Some(q), Some(q))).collect();
        let mut good = true;
        for &(x, y) in &all_pts {
            let in2e = doubles.contains(&Some((x, y)));
            good &= in2e == (delta(&gf, &cv, x) == 0);
        }
        let mut srng = Rng(99);
        for _ in 0..200 {
            let (x, y) = random_target(&gf, &cv, &mut srng, 'S');
            good &= doubles.contains(&Some((x, y)));
        }
        ok(&format!("curve a={}: {{delta = 0}} equals 2E(F), and the S-population sampler stays inside it", cv.a), good, &mut fails);
    }
    // 3. exact decomposer against brute force over every point of E(F_{2^11}) and E(F_{2^13})
    for &(n, ell, m) in &[(11u32, 4usize, 2usize), (11, 5, 2), (11, 3, 3), (11, 4, 3), (13, 5, 2), (13, 4, 3)] {
        let gf = Gf::new(n);
        for seed in 0..2u64 {
            for &kind in &['G', 'T', 'P'] {
                let cv = make_curve(&gf, mix(0xD00D + n as u64, seed));
                let mut rng = Rng(mix(0xABC, seed + ell as u64));
                let base = build_base(&gf, &cv, kind, ell, &mut rng);
                // full B (both signs)
                let mut bfull: Vec<Pt> = vec![];
                for &(x, y) in &base.pts {
                    bfull.push(Some((x, y)));
                    if x != 0 {
                        bfull.push(Some((x, y ^ x)));
                    }
                }
                // m-fold sumset by repeated addition
                let mut cur: std::collections::HashSet<Pt> = bfull.iter().copied().collect();
                for _ in 1..m {
                    let mut nx = std::collections::HashSet::new();
                    for &p in &cur {
                        for &q in &bfull {
                            nx.insert(add(&gf, &cv, p, q));
                        }
                    }
                    cur = nx;
                }
                let dec = Decomposer::new(&gf, cv, &base, m);
                let ss = delta_sumset(&base, m);
                let (mut agree, mut tot, mut npos) = (true, 0, 0);
                let mut false_b = 0;
                for x in 1..(1u64 << n) {
                    if let Some((x, y)) = point_at(&gf, &cv, x) {
                        for &yy in &[y, y ^ x] {
                            let want = cur.contains(&Some((x, yy)));
                            let got = dec.label(x, yy);
                            agree &= want == got;
                            tot += 1;
                            npos += want as usize;
                            let refuted_b = !ss[gf.tr(x ^ cv.a) as usize];
                            if refuted_b && want {
                                false_b += 1;
                            }
                        }
                    }
                }
                ok(
                    &format!("decomposer n={n} l={ell} m={m} base={kind} a={}: exact vs brute force on {tot} points ({npos} decomposable), arm b false refutations {false_b}", cv.a),
                    agree && false_b == 0 && npos > 0 && npos < tot,
                    &mut fails,
                );
            }
        }
    }
    // 4. learner sanity on synthetic features from the real feature code (n = 13)
    {
        let gf = Gf::new(13);
        let cv = make_curve(&gf, 0x5EED);
        let lay = feat_layout(13);
        let mut rng = Rng(5);
        let base = build_base(&gf, &cv, 'G', 5, &mut rng);
        let total = N_BASES * 400;
        let mut xs: Vec<f32> = vec![];
        let (mut tx, mut ti, mut t3, mut t5) = (vec![], vec![], vec![], vec![]);
        let mut base_of = vec![];
        for bi in 0..N_BASES {
            for _ in 0..400 {
                let (x, _) = random_target(&gf, &cv, &mut rng, 'U');
                features(&gf, &cv, &base, 2, x, &mut xs);
                tx.push(gf.tr(x) as u8);
                ti.push(gf.tr(gf.inv(x)) as u8);
                let x2 = gf.sqr(x);
                let x3 = gf.mul(x2, x);
                let x5 = gf.mul(gf.sqr(x2), x);
                t3.push(gf.tr(x3) as u8);
                t5.push(gf.tr(x5) as u8);
                base_of.push(bi);
            }
        }
        let d = lay.d;
        ok("feature vector length matches layout", xs.len() == total * d, &mut fails);
        let yp: Vec<u8> = tx.iter().map(|&t| (t == 0) as u8).collect();
        let yx: Vec<u8> = (0..total).map(|i| (if ti[i] == 0 { t3[i] == 0 } else { t5[i] == 0 }) as u8).collect();
        let mut yn = yp.clone();
        for i in (1..yn.len()).rev() {
            let j = rng.below(i as u64 + 1) as usize;
            yn.swap(i, j);
        }
        for &(model, name) in &[(Model::Lr, "lr"), (Model::Gbt, "gbt")] {
            let sel = |_: char| true;
            let none = vec![false; total];
            let o = run_folds(model, &xs, d, &lay.groups, &yp, &base_of, 1);
            let st = group_stats(&o, &yp, &base_of, &sel, &none);
            ok(&format!("positive control {name}: planted Tr(x) recovered (auc {:.4}, fr {}, refuted {:.3})", st.auc, st.fr, st.ref_neg as f64 / st.neg as f64),
               st.auc >= 0.99 && st.roc0_neg as f64 / st.neg as f64 >= 0.9, &mut fails);
            let o = run_folds(model, &xs, d, &lay.groups, &yn, &base_of, 2);
            let st = group_stats(&o, &yn, &base_of, &sel, &none);
            ok(&format!("negative control {name}: permuted labels give auc {:.4} within [0.45, 0.55]", st.auc), st.auc > 0.45 && st.auc < 0.55, &mut fails);
            let o = run_folds(model, &xs, d, &lay.groups, &yx, &base_of, 3);
            let st = group_stats(&o, &yx, &base_of, &sel, &none);
            if model == Model::Gbt {
                ok(&format!("interaction control gbt: planted selector interaction recovered (auc {:.4})", st.auc), st.auc >= 0.95, &mut fails);
            } else {
                println!("INFO interaction control lr (descriptive, linear model on a selector label): auc {:.4}", st.auc);
            }
        }
    }
    println!("selftest failures: {fails}");
    fails
}

// ---------------------------------------------------------------------- plan

fn plan(n: u32, m: usize) {
    let l0 = formula_ell(n, m);
    for ell in 3..(n as usize - 1) {
        let lam = lambda_formula(n, m, ell);
        println!(
            "{{\"kind\":\"plan\",\"n\":{},\"m\":{},\"l\":{},\"formula_lambda\":{},\"formula_pos_frac\":{},\"chosen_center\":{}}}",
            n, m, ell, jnum(lam), jnum(1.0 - (-lam).exp()), ell == l0
        );
    }
}

// ------------------------------------------------------------------- verdict

fn parse_flat(line: &str) -> Vec<(String, String)> {
    let t = line.trim().trim_start_matches('{').trim_end_matches('}');
    let mut out = vec![];
    let mut key = String::new();
    let mut val = String::new();
    let mut in_str = false;
    let mut state = 0; // 0 reading key, 1 reading value
    let mut chars = t.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '"' {
            in_str = !in_str;
            continue;
        }
        if !in_str && c == ':' && state == 0 {
            state = 1;
            continue;
        }
        if !in_str && c == ',' && state == 1 {
            out.push((key.clone(), val.clone()));
            key.clear();
            val.clear();
            state = 0;
            continue;
        }
        if state == 0 {
            key.push(c)
        } else {
            val.push(c)
        }
    }
    if !key.is_empty() {
        out.push((key, val));
    }
    out
}

fn kv<'a>(r: &'a [(String, String)], k: &str) -> &'a str {
    r.iter().find(|(a, _)| a == k).map(|(_, v)| v.as_str()).unwrap_or("null")
}
fn kf(r: &[(String, String)], k: &str) -> f64 {
    kv(r, k).parse::<f64>().unwrap_or(f64::NAN)
}

fn verdict(dir: &str) {
    let mut rows: Vec<Vec<(String, String)>> = vec![];
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .expect("read raw dir")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map_or(false, |x| x == "jsonl"))
        .collect();
    files.sort();
    for f in &files {
        let text = std::fs::read_to_string(f).unwrap();
        for l in text.lines() {
            if l.starts_with("{\"pop\"") {
                rows.push(parse_flat(l));
            }
        }
    }
    println!("verdict input: {} jsonl files, {} rows", files.len(), rows.len());
    let mut cells: Vec<(String, u32, usize)> = rows
        .iter()
        .filter(|r| kv(r, "kind") == "done")
        .map(|r| (kv(r, "pop").to_string(), kf(r, "n") as u32, kf(r, "m") as usize))
        .collect();
    cells.sort();
    cells.dedup();
    println!("cell-populations present: {}", cells.len());
    // per population: pass table, any valid control, trace-applicable rows
    type Key = (String, String); // (model, type)
    let mut passes: std::collections::BTreeMap<String, std::collections::BTreeMap<Key, Vec<(u32, usize)>>> = Default::default();
    let mut any_valid: std::collections::BTreeMap<String, bool> = Default::default();
    let mut t_rows: std::collections::BTreeMap<String, (u32, u32)> = Default::default(); // (rows, rows where c >= b - 0.05)
    let mut false_b_total = 0.0;
    for (pop, n, m) in &cells {
        let (n, m) = (*n, *m);
        let sel = |r: &Vec<(String, String)>, kind: &str| {
            kv(r, "kind") == kind && kv(r, "pop") == pop && kf(r, "n") as u32 == n && kf(r, "m") as usize == m
        };
        if let Some(ls) = rows.iter().find(|r| sel(r, "label_summary")) {
            false_b_total += kf(ls, "false_refutations_b");
            println!(
                "cell pop={pop} n={n} m={m} l={}: targets {} positives {} false refutations of arm b {}",
                kv(ls, "l"), kv(ls, "targets"), kv(ls, "pos"), kv(ls, "false_refutations_b")
            );
        }
        for model in ["lr", "gbt"] {
            let ctl = rows.iter().find(|r| sel(r, "control") && kv(r, "model") == model && kv(r, "name") == "planted_trace");
            let ctl_ok = ctl.map_or(false, |c| kf(c, "auc") >= 0.99 && kf(c, "roc0") >= 0.9);
            println!("  control planted_trace {model}: {}", if ctl_ok { "PASS" } else { "FAIL" });
            if ctl_ok {
                any_valid.insert(pop.clone(), true);
            }
            for ty in ["ALL", "T", "G", "P"] {
                if let Some(s) = rows.iter().find(|r| sel(r, "summary") && kv(r, "model") == model && kv(r, "type") == ty) {
                    let fr = kf(s, "fr");
                    let delta = kf(s, "delta");
                    let lo = kf(s, "delta_lo");
                    let roc0 = kf(s, "roc0");
                    let auc = kf(s, "auc");
                    let null_ok = roc0 > kf(s, "null_roc0_max") && auc > kf(s, "null_auc_max");
                    let pass = ctl_ok && fr == 0.0 && delta >= 0.05 && lo > 0.0 && null_ok;
                    println!(
                        "  {model:>3} {ty:>3}: neg {} pos {} rate_b {:.4} rate_c {:.4} fr {} delta {:.4} [{:.4},{:.4}] union {:.4} roc0 {:.4} auc {:.4} null_auc_max {:.4} null_roc0_max {:.4} -> {}",
                        kv(s, "neg"), kv(s, "pos"), kf(s, "rate_b"), kf(s, "rate_c"), fr, delta, lo, kf(s, "delta_hi"),
                        kf(s, "rate_union"), roc0, auc, kf(s, "null_auc_max"), kf(s, "null_roc0_max"),
                        if pass { "CELL PASS" } else { "no" }
                    );
                    if pass {
                        passes.entry(pop.clone()).or_default().entry((model.to_string(), ty.to_string())).or_default().push((n, m));
                    }
                    if ty == "T" && kf(s, "rate_b") > 0.0 {
                        let e = t_rows.entry(pop.clone()).or_insert((0, 0));
                        e.0 += 1;
                        if kf(s, "rate_c") >= kf(s, "rate_b") - 0.05 {
                            e.1 += 1;
                        }
                    }
                }
            }
        }
        // net work at three prices of the unpriced word operations (gbt, all bases)
        if let (Some(c), Some(s)) = (
            rows.iter().find(|r| sel(r, "cost")),
            rows.iter().find(|r| sel(r, "summary") && kv(r, "model") == "gbt" && kv(r, "type") == "ALL"),
        ) {
            let neg = kf(s, "neg");
            let pos = kf(s, "pos");
            let negfrac = neg / (neg + pos);
            for w in [0.0, 0.1, 1.0] {
                let filt = kf(c, "feat_mul") + kf(c, "feat_sqr") + w * (kf(c, "feat_lin") + kf(c, "gbt_model_ops"));
                let dneg = kf(c, "dec_neg_field_ops") + w * kf(c, "dec_neg_lin");
                let dpos = kf(c, "dec_pos_field_ops") + w * kf(c, "dec_pos_lin");
                let mean_dec = negfrac * dneg + (1.0 - negfrac) * dpos;
                let saved = negfrac * kf(s, "rate_c") * dneg;
                println!(
                    "  cost pop={pop} n={n} m={m} w={w}: gbt filter {:.1} per target, decomposer {:.1} per target, saved by refutation {:.1} -> net {:+.4} of decomposer work",
                    filt, mean_dec, saved, (saved - filt) / mean_dec
                );
            }
        }
    }
    println!("false refutations of arm b over all cell-populations: {false_b_total}");
    let mut per_pop: Vec<(String, u8, String)> = vec![]; // (pop, rank, text); rank 3 best
    let pops: Vec<String> = {
        let mut v: Vec<String> = cells.iter().map(|c| c.0.clone()).collect();
        v.dedup();
        v
    };
    for pop in &pops {
        let mut winners: Vec<String> = vec![];
        let mut single: Vec<String> = vec![];
        if let Some(pm) = passes.get(pop) {
            for ((model, ty), v) in pm {
                let mut cs = v.clone();
                cs.sort();
                cs.dedup();
                if cs.len() >= 2 {
                    winners.push(format!("{model}/{ty} in {} cells", cs.len()));
                } else {
                    single.push(format!("{model}/{ty} in {:?}", cs));
                }
            }
        }
        let tr = t_rows.get(pop).copied().unwrap_or((0, 0));
        let (rank, text) = if !any_valid.get(pop).copied().unwrap_or(false) {
            (1, "INCONCLUSIVE (primitive): the planted-label positive control failed in every cell and model, so the instrument cannot see an effect".to_string())
        } else if !winners.is_empty() {
            (4, format!("ADVANCES (primitive): the learned filter beats the trace filter by >= 5 points at zero false refutations, null excluded, in >= 2 cells: {}", winners.join("; ")))
        } else if !single.is_empty() {
            (2, format!("INCONCLUSIVE (primitive): the frozen rule is met in exactly one cell per arm, which the protocol does not accept as replicated: {}", single.join("; ")))
        } else if tr.0 > 0 && tr.1 == 0 {
            (0, "REGRESSES (primitive): on the trace-applicable bases the learned filter alone refutes more than 5 points fewer targets than the trace filter in every cell, and gains nowhere".to_string())
        } else {
            (3, "MATCHES (primitive): no cell and base type gives the learned filter a refutation rate 5 points above the trace filter at zero false refutations with the null band excluded".to_string())
        };
        println!("VERDICT population {pop}: {text}");
        per_pop.push((pop.clone(), rank, text));
    }
    // overall: ADVANCES if any population advances, else INCONCLUSIVE if any, else MATCHES if any, else REGRESSES
    let order = |r: u8| match r { 4 => 5, 2 => 4, 3 => 3, 1 => 2, _ => 1 };
    if let Some(best) = per_pop.iter().max_by_key(|p| order(p.1)) {
        println!("VERDICT OVERALL (population {}): {}", best.0, best.2);
    }
}

fn main() {
    let args = Args {
        v: std::env::args().skip(1).collect(),
    };
    match args.v.first().map(|s| s.as_str()) {
        Some("selftest") => {
            let f = selftest();
            if f > 0 {
                std::process::exit(1);
            }
        }
        Some("plan") => plan(args.num("--n", 17u32), args.num("--m", 2usize)),
        Some("cell") => {
            let p = CellParams {
                n: args.num("--n", 17u32),
                m: args.num("--m", 2usize),
                ell: args.num("--l", 0usize),
                per_base: args.num("--per-base", 2000usize),
                perms: args.num("--perms", 10usize),
                pilot: args.num("--pilot", 400usize),
                pop: args.flag("--pop").map_or('U', |s| s.chars().next().unwrap()),
            };
            assert!(p.pop == 'U' || p.pop == 'S', "--pop U|S");
            run_cell(&p);
        }
        Some("verdict") => verdict(&args.flag("--dir").expect("--dir")),
        _ => {
            eprintln!("usage: b11 selftest | plan --n N --m M | cell --n N --m M [--l L] [--per-base K] [--perms P] [--pilot Q] | verdict --dir DIR");
            std::process::exit(2);
        }
    }
}
