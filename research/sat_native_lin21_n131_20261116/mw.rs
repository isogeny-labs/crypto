//! Multi-word binary field F_2[z]/(f) with `[u64; W]` elements, binary curve
//! y^2 + xy = x^3 + a x^2 + b, and two reference deciders for "R = P_i + P_j + P_k over the
//! factor base of rational points with abscissa of degree < d":
//!   * `Ctx::scan`  : a generic-width port of the BASELINE arm (`Arm::Base`, level 4, no pretest)
//!                    of research/scan_floor_leaf_20261116/scan_leaf.rs (sha256 816d0831...), which
//!                    is itself a standalone port of the repository's `lin21-ht` leaf
//!                    (examples/ic_algebraic_floor/linear21.rs): batched t stage (R - P_i, 1/t),
//!                    half-trace elimination, root split, membership + group verification.
//!                    Differences from that file: field words are `[u64; W]`; the field polynomial
//!                    is a parameter; the reduction loops until no high bits remain; the pivot
//!                    table of `solve_f2` is indexed up to 64 W bits and reset by a used-list; no
//!                    operation counters. n odd (half-trace).
//!   * `Ctx::pairs` : plain pair enumeration in point form: for every unordered pair of base
//!                    points (both signs, i <= j) test whether x(R - P_i - P_j) has degree < d
//!                    (batched affine additions, x only).
//! Carry-less multiply: the pclmulqdq intrinsic when the build enables it
//! (`-C target-cpu=native` on this host), otherwise a portable shift-and-xor fallback.

use std::time::Instant;

pub type Fe<const W: usize> = [u64; W];

#[cfg(target_feature = "pclmulqdq")]
#[inline(always)]
fn clmul(a: u64, b: u64) -> (u64, u64) {
    // SAFETY: compiled only when the target has pclmulqdq.
    unsafe {
        use std::arch::x86_64::*;
        let z = _mm_clmulepi64_si128::<0>(_mm_set_epi64x(0, a as i64), _mm_set_epi64x(0, b as i64));
        let lo = _mm_cvtsi128_si64(z) as u64;
        let hi = _mm_cvtsi128_si64(_mm_srli_si128::<8>(z)) as u64;
        (lo, hi)
    }
}

#[cfg(not(target_feature = "pclmulqdq"))]
#[inline(always)]
fn clmul(a: u64, b: u64) -> (u64, u64) {
    clmul_portable(a, b)
}

pub fn clmul_portable(a: u64, b: u64) -> (u64, u64) {
    let (mut lo, mut hi) = (0u64, 0u64);
    let mut b = b;
    while b != 0 {
        let i = b.trailing_zeros();
        b &= b - 1;
        lo ^= a << i;
        if i != 0 {
            hi ^= a >> (64 - i);
        }
    }
    (lo, hi)
}

pub fn clmul_backend() -> &'static str {
    if cfg!(target_feature = "pclmulqdq") {
        "pclmulqdq"
    } else {
        "portable"
    }
}

/// Self-check of the active carry-less multiply against the portable one.
pub fn clmul_selfcheck() -> bool {
    let mut s = 0x1234_5678_9abc_def1u64;
    for _ in 0..2000 {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let a = s;
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        if clmul(a, s) != clmul_portable(a, s) {
            return false;
        }
    }
    true
}

#[inline(always)]
pub fn fe<const W: usize>(x: u64) -> Fe<W> {
    let mut o = [0u64; W];
    o[0] = x;
    o
}
#[inline(always)]
pub fn fx<const W: usize>(a: Fe<W>, b: Fe<W>) -> Fe<W> {
    let mut o = a;
    for w in 0..W {
        o[w] ^= b[w];
    }
    o
}
#[inline(always)]
pub fn fz<const W: usize>(a: &Fe<W>) -> bool {
    a.iter().all(|&x| x == 0)
}
#[inline(always)]
pub fn fshr<const W: usize>(a: Fe<W>, k: usize) -> Fe<W> {
    let (q, r) = (k / 64, k % 64);
    let mut o = [0u64; W];
    for w in 0..W - q.min(W) {
        o[w] = a[w + q] >> r;
        if r != 0 && w + q + 1 < W {
            o[w] |= a[w + q + 1] << (64 - r);
        }
    }
    o
}
#[inline(always)]
pub fn ftop<const W: usize>(a: &Fe<W>) -> usize {
    for w in (0..W).rev() {
        if a[w] != 0 {
            return w * 64 + 63 - a[w].leading_zeros() as usize;
        }
    }
    usize::MAX
}
#[inline(always)]
pub fn fsetbit<const W: usize>(a: &mut Fe<W>, i: usize, v: u64) {
    a[i / 64] |= v << (i % 64);
}
/// The value as an index if it has no bit at or above `d` (d <= 32).
#[inline(always)]
pub fn fsmall<const W: usize>(a: &Fe<W>, d: usize) -> Option<u64> {
    if a[0] >> d != 0 || a[1..].iter().any(|&x| x != 0) {
        None
    } else {
        Some(a[0])
    }
}
pub fn fhex<const W: usize>(a: &Fe<W>) -> String {
    let mut s = String::new();
    for w in (0..W).rev() {
        s.push_str(&format!("{:016x}", a[w]));
    }
    s
}
pub fn fparse<const W: usize>(s: &str) -> Fe<W> {
    let mut o = [0u64; W];
    let s = format!("{:0>width$}", s, width = 16 * W);
    assert!(s.len() == 16 * W, "hex too long");
    for w in 0..W {
        o[W - 1 - w] = u64::from_str_radix(&s[16 * w..16 * w + 16], 16).unwrap();
    }
    o
}

pub struct Gf<const W: usize> {
    pub n: usize,
    pub irr: Fe<W>,
    terms: Vec<usize>,
    fast1: bool,
    mask: u64,
}

impl<const W: usize> Gf<W> {
    /// `low`: exponents of the field polynomial below n (including 0).
    pub fn new(n: usize, low: &[usize]) -> Self {
        assert!(W <= 4 && n < 64 * W && low.iter().all(|&t| t < 64 && t < n));
        let mut irr = [0u64; W];
        fsetbit(&mut irr, n, 1);
        for &t in low {
            irr[0] ^= 1u64 << t;
        }
        let mx = *low.iter().max().unwrap();
        let fast1 = W == 1 && 2 * mx <= n && n + mx < 64;
        Self { n, irr, terms: low.to_vec(), fast1, mask: if n < 64 { (1u64 << n) - 1 } else { 0 } }
    }
    pub fn from_irr_u64(n: usize, irr: u64) -> Self {
        let low: Vec<usize> = (0..n).filter(|&t| (irr >> t) & 1 == 1).collect();
        Self::new(n, &low)
    }
    /// z^(2^n) = z, n prime and f(0) = f(1) = 1: then f is irreducible.
    pub fn irreducible_prime_degree(&self) -> bool {
        let n = self.n;
        let prime = n > 1 && (2..n).take_while(|p| p * p <= n).all(|p| n % p != 0);
        let z = fe::<W>(2);
        prime && self.terms.contains(&0) && self.terms.len() % 2 == 0 && self.sqr_k(z, n) == z
    }
    #[inline(always)]
    fn fold(&self, hi: u64) -> u64 {
        let mut t = 0;
        for &e in &self.terms {
            t ^= hi << e;
        }
        t
    }
    #[inline(always)]
    fn reduce(&self, mut c: [u64; 8]) -> Fe<W> {
        if self.fast1 {
            // one word, two folds (the reduction of scan_leaf.rs)
            let lo = c[0] & self.mask;
            let hi = (c[0] >> self.n) | (c[1] << (64 - self.n));
            let t = lo ^ self.fold(hi);
            let mut o = [0u64; W];
            o[0] = (t & self.mask) ^ self.fold(t >> self.n);
            return o;
        }
        let (q, r) = (self.n / 64, self.n % 64);
        let tw = 2 * W;
        loop {
            let mut hi = [0u64; 8];
            let mut any = 0u64;
            for i in 0..tw - q {
                let mut v = c[i + q] >> r;
                if r != 0 && i + q + 1 < tw {
                    v |= c[i + q + 1] << (64 - r);
                }
                hi[i] = v;
                any |= v;
            }
            if any == 0 {
                break;
            }
            c[q] &= (1u64 << r) - 1;
            for x in c[q + 1..tw].iter_mut() {
                *x = 0;
            }
            for &t in &self.terms {
                for i in 0..tw {
                    c[i] ^= hi[i] << t;
                    if t != 0 && i > 0 {
                        c[i] ^= hi[i - 1] >> (64 - t);
                    }
                }
            }
        }
        let mut o = [0u64; W];
        o.copy_from_slice(&c[..W]);
        o
    }
    #[inline(always)]
    pub fn mul(&self, a: Fe<W>, b: Fe<W>) -> Fe<W> {
        let mut c = [0u64; 8];
        for i in 0..W {
            for j in 0..W {
                let (lo, hi) = clmul(a[i], b[j]);
                c[i + j] ^= lo;
                c[i + j + 1] ^= hi;
            }
        }
        self.reduce(c)
    }
    #[inline(always)]
    pub fn sqr(&self, a: Fe<W>) -> Fe<W> {
        let mut c = [0u64; 8];
        for i in 0..W {
            let (lo, hi) = clmul(a[i], a[i]);
            c[2 * i] = lo;
            c[2 * i + 1] = hi;
        }
        self.reduce(c)
    }
    pub fn sqr_k(&self, mut a: Fe<W>, k: usize) -> Fe<W> {
        for _ in 0..k {
            a = self.sqr(a);
        }
        a
    }
    /// Itoh-Tsujii: a^(2^(n-1)-1), then one squaring.
    pub fn inv(&self, a: Fe<W>) -> Fe<W> {
        debug_assert!(!fz(&a));
        let e = (self.n - 1) as u32;
        let bits = 32 - e.leading_zeros();
        let (mut r, mut k) = (a, 1usize);
        for i in (0..bits - 1).rev() {
            r = self.mul(self.sqr_k(r, k), r);
            k *= 2;
            if (e >> i) & 1 == 1 {
                r = self.mul(self.sqr(r), a);
                k += 1;
            }
        }
        self.sqr(r)
    }
    /// Montgomery's trick: 3(k-1) multiplications and one inversion. No zero.
    #[inline(always)]
    pub fn batch_inv(&self, xs: &mut [Fe<W>], acc: &mut Vec<Fe<W>>) {
        if xs.is_empty() {
            return;
        }
        acc.clear();
        let mut p = xs[0];
        acc.push(p);
        for &x in &xs[1..] {
            p = self.mul(p, x);
            acc.push(p);
        }
        let mut inv = self.inv(p);
        for i in (1..xs.len()).rev() {
            let xi = xs[i];
            xs[i] = self.mul(inv, acc[i - 1]);
            inv = self.mul(inv, xi);
        }
        xs[0] = inv;
    }
    pub fn half(&self, a: Fe<W>) -> Fe<W> {
        let (mut t, mut h) = (a, a);
        for _ in 0..(self.n - 1) / 2 {
            t = self.sqr(self.sqr(t));
            h = fx(h, t);
        }
        h
    }
    pub fn trace(&self, a: Fe<W>) -> u64 {
        let (mut t, mut tr) = (a, a);
        for _ in 1..self.n {
            t = self.sqr(t);
            tr = fx(tr, t);
        }
        assert!(tr[0] <= 1 && tr[1..].iter().all(|&x| x == 0));
        tr[0]
    }
}

/// F_2-linear map as byte tables.
pub struct LinMap<const W: usize> {
    t: Vec<[Fe<W>; 256]>,
}

impl<const W: usize> LinMap<W> {
    pub fn new(n: usize, image_of_bit: impl Fn(usize) -> Fe<W>) -> Self {
        let bytes = n.div_ceil(8);
        let mut t = vec![[[0u64; W]; 256]; bytes];
        for (j, row) in t.iter_mut().enumerate() {
            for v in 1usize..256 {
                let bit = 8 * j + v.trailing_zeros() as usize;
                let img = if bit < n { image_of_bit(bit) } else { [0u64; W] };
                row[v] = fx(row[v & (v - 1)], img);
            }
        }
        Self { t }
    }
    #[inline(always)]
    pub fn apply(&self, x: Fe<W>) -> Fe<W> {
        let mut acc = [0u64; W];
        for (j, row) in self.t.iter().enumerate() {
            let byte = (x[j / 8] >> (8 * (j % 8))) & 0xff;
            let e = &row[byte as usize];
            for w in 0..W {
                acc[w] ^= e[w];
            }
        }
        acc
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pt<const W: usize> {
    pub x: Fe<W>,
    pub y: Fe<W>,
    pub inf: bool,
}

#[inline(always)]
fn sign_bit<const W: usize>(p: &Pt<W>) -> usize {
    let o = fx(p.x, p.y);
    for w in (0..W).rev() {
        if p.y[w] != o[w] {
            return usize::from(p.y[w] > o[w]);
        }
    }
    0
}

pub struct Scratch<const W: usize> {
    cols: Vec<Fe<W>>,
    kernel: Vec<u64>,
    dens: Vec<Fe<W>>,
    acc: Vec<Fe<W>>,
    qs: Vec<Pt<W>>,
    uinv: Vec<Fe<W>>,
    piv: Vec<(Fe<W>, u64)>,
    used: Vec<usize>,
}

impl<const W: usize> Scratch<W> {
    pub fn new() -> Self {
        Self {
            cols: Vec::new(),
            kernel: Vec::new(),
            dens: Vec::new(),
            acc: Vec::new(),
            qs: Vec::new(),
            uinv: Vec::new(),
            piv: vec![([0u64; W], 0); 64 * W],
            used: Vec::new(),
        }
    }
}

pub struct Ctx<const W: usize> {
    pub gf: Gf<W>,
    pub n: usize,
    pub d: usize,
    pub a: Fe<W>,
    pub b: Fe<W>,
    pub sqrt_b: Fe<W>,
    pub points: Vec<Pt<W>>,
    pub negated: Vec<Pt<W>>,
    pub index_of: Vec<usize>,
    ell: Vec<Fe<W>>,
    trace_mask: u64,
    pub half_trace: LinMap<W>,
    sqrt: LinMap<W>,
    inv_s2: Vec<Fe<W>>,
    kernel_cap: usize,
    pub setup_ns: u64,
}

#[derive(Default, Clone, Copy)]
pub struct ScanStat {
    pub leaves: u64,
    pub consistent: u64,
    pub splits: u64,
    pub in_range: u64,
    pub truncated: bool,
}

impl<const W: usize> Ctx<W> {
    pub fn new(gf: Gf<W>, d: usize, a: u64, b: u64) -> Self {
        let n = gf.n;
        assert!(n % 2 == 1 && 2 * d - 1 <= n && d <= 20);
        let t0 = Instant::now();
        let (a, b) = (fe::<W>(a), fe::<W>(b));
        let sqrt_b = gf.sqr_k(b, n - 1);
        let half_trace = LinMap::new(n, |bit| gf.half(Self::unit(bit)));
        let sqrt = LinMap::new(n, |bit| gf.sqr_k(Self::unit(bit), n - 1));
        // Factor base: every rational point with x in V, both signs, x ascending.
        let mut points = Vec::new();
        points.push(Pt { x: [0u64; W], y: sqrt_b, inf: false });
        let mut ixs: Vec<Fe<W>> = (1..1u64 << d).map(fe::<W>).collect();
        gf.batch_inv(&mut ixs, &mut Vec::new());
        for x in 1u64..(1u64 << d) {
            let xe = fe::<W>(x);
            let rhs = fx(fx(xe, a), gf.mul(b, gf.sqr(ixs[x as usize - 1])));
            let w = half_trace.apply(rhs);
            if fx(gf.sqr(w), w) != rhs {
                continue;
            }
            let y = gf.mul(xe, w);
            points.push(Pt { x: xe, y, inf: false });
            points.push(Pt { x: xe, y: fx(xe, y), inf: false });
        }
        let mut index_of = vec![usize::MAX; 2usize << d];
        for (i, p) in points.iter().enumerate() {
            index_of[2 * p.x[0] as usize + sign_bit(p)] = i;
        }
        let negated: Vec<Pt<W>> = points.iter().map(|p| Pt { x: p.x, y: fx(p.x, p.y), inf: false }).collect();
        let ell: Vec<Fe<W>> = (0..d).map(|j| fx(Self::unit(j), gf.half(Self::unit(j)))).collect();
        let trace_mask = (0..d).fold(0u64, |m, j| m | (gf.trace(Self::unit(j)) << j));
        let mut inv_s2: Vec<Fe<W>> = (0..1u64 << d).map(fe::<W>).collect();
        inv_s2[0] = fe(1);
        gf.batch_inv(&mut inv_s2, &mut Vec::new());
        for v in inv_s2.iter_mut() {
            *v = gf.sqr(*v);
        }
        let setup_ns = t0.elapsed().as_nanos() as u64;
        Self { gf, n, d, a, b, sqrt_b, points, negated, index_of, ell, trace_mask, half_trace, sqrt, inv_s2, kernel_cap: 16, setup_ns }
    }

    fn unit(j: usize) -> Fe<W> {
        let mut e = [0u64; W];
        e[j / 64] = 1u64 << (j % 64);
        e
    }

    /// Number of nonzero abscissae of degree < d that lift (the panel's `A`).
    pub fn admissible(&self) -> usize {
        (self.points.len() - 1) / 2
    }

    pub fn on_curve(&self, p: &Pt<W>) -> bool {
        let f = &self.gf;
        let x2 = f.sqr(p.x);
        p.inf || fx(f.sqr(p.y), f.mul(p.x, p.y)) == fx(fx(f.mul(x2, p.x), f.mul(self.a, x2)), self.b)
    }

    /// A rational point with abscissa x (x != 0), if any.
    pub fn lift_x(&self, x: Fe<W>) -> Option<Pt<W>> {
        let f = &self.gf;
        if fz(&x) {
            return Some(Pt { x, y: self.sqrt_b, inf: false });
        }
        let ix = f.inv(x);
        let rhs = fx(fx(x, self.a), f.mul(self.b, f.sqr(ix)));
        let w = self.half_trace.apply(rhs);
        if fx(f.sqr(w), w) != rhs {
            return None;
        }
        Some(Pt { x, y: f.mul(x, w), inf: false })
    }

    pub fn inf() -> Pt<W> {
        Pt { x: [0u64; W], y: [0u64; W], inf: true }
    }

    pub fn neg(&self, p: Pt<W>) -> Pt<W> {
        if p.inf {
            p
        } else {
            Pt { x: p.x, y: fx(p.x, p.y), inf: false }
        }
    }

    pub fn double(&self, p: Pt<W>) -> Pt<W> {
        if p.inf || fz(&p.x) {
            return Self::inf();
        }
        let f = &self.gf;
        let l = fx(p.x, f.mul(p.y, f.inv(p.x)));
        let x3 = fx(fx(f.sqr(l), l), self.a);
        let y3 = fx(f.sqr(p.x), f.mul(fx(l, fe(1)), x3));
        Pt { x: x3, y: y3, inf: false }
    }

    #[inline(always)]
    fn add_with_lambda(&self, p: Pt<W>, q: Pt<W>, lambda: Fe<W>) -> Pt<W> {
        let f = &self.gf;
        let x3 = fx(fx(fx(fx(f.sqr(lambda), lambda), p.x), q.x), self.a);
        let y3 = fx(fx(f.mul(lambda, fx(p.x, x3)), x3), p.y);
        Pt { x: x3, y: y3, inf: false }
    }

    pub fn add(&self, p: Pt<W>, q: Pt<W>) -> Pt<W> {
        if p.inf {
            return q;
        }
        if q.inf {
            return p;
        }
        if p.x == q.x {
            if fx(p.y, q.y) == p.x {
                return Self::inf();
            }
            return self.double(p);
        }
        let lambda = self.gf.mul(fx(p.y, q.y), self.gf.inv(fx(p.x, q.x)));
        self.add_with_lambda(p, q, lambda)
    }

    /// Port of `add_many`: `P + Q_j`, one inversion for the slice.
    #[inline(always)]
    fn add_many(&self, p: Pt<W>, qs: &[Pt<W>], out: &mut Vec<Pt<W>>, dens: &mut Vec<Fe<W>>, acc: &mut Vec<Fe<W>>) {
        let f = &self.gf;
        dens.clear();
        dens.extend(qs.iter().map(|q| if p.inf || q.inf || q.x == p.x { fe(1) } else { fx(p.x, q.x) }));
        f.batch_inv(dens, acc);
        let mut double_p: Option<Pt<W>> = None;
        for (q, &inv) in qs.iter().zip(dens.iter()) {
            if p.inf || q.inf || q.x == p.x {
                let sum = if p.inf {
                    *q
                } else if q.inf {
                    p
                } else if fx(p.y, q.y) == p.x {
                    Self::inf()
                } else {
                    *double_p.get_or_insert_with(|| self.double(p))
                };
                out.push(sum);
                continue;
            }
            let lambda = f.mul(fx(p.y, q.y), inv);
            out.push(self.add_with_lambda(p, *q, lambda));
        }
    }

    /// Either sign of either summand against `Q` in the group (port of `verify`).
    #[inline(always)]
    fn verify(&self, q: Pt<W>, x1: u64, x2: u64) -> Option<(usize, usize)> {
        for s1 in 0..2 {
            let j = self.index_of[2 * x1 as usize + s1];
            if j == usize::MAX {
                continue;
            }
            for s2 in 0..2 {
                let k = self.index_of[2 * x2 as usize + s2];
                if k == usize::MAX {
                    continue;
                }
                if self.add(self.points[j], self.points[k]) == q {
                    return Some((j, k));
                }
            }
        }
        None
    }

    /// Port of `solve_f2`: particular solution and kernel basis as column combinations.
    #[inline(always)]
    fn solve_f2(cols: &[Fe<W>], rhs: Fe<W>, piv: &mut [(Fe<W>, u64)], used: &mut Vec<usize>, kernel: &mut Vec<u64>) -> Option<u64> {
        for &u in used.iter() {
            piv[u] = ([0u64; W], 0);
        }
        used.clear();
        kernel.clear();
        for (k, &col) in cols.iter().enumerate() {
            let (mut v, mut comb) = (col, 1u64 << k);
            loop {
                let top = ftop(&v);
                if top == usize::MAX {
                    kernel.push(comb);
                    break;
                }
                if fz(&piv[top].0) {
                    piv[top] = (v, comb);
                    used.push(top);
                    break;
                }
                v = fx(v, piv[top].0);
                comb ^= piv[top].1;
            }
        }
        let (mut v, mut comb) = (rhs, 0u64);
        loop {
            let top = ftop(&v);
            if top == usize::MAX {
                return Some(comb);
            }
            if fz(&piv[top].0) {
                return None;
            }
            v = fx(v, piv[top].0);
            comb ^= piv[top].1;
        }
    }

    /// Port of the baseline leaf (`pair_half_trace` + `split_root_and_verify` up to the range check).
    #[inline(always)]
    fn leaf<F: FnMut(u64, u64)>(&self, u: Fe<W>, ui: Fe<W>, ws_cols: &mut Vec<Fe<W>>, piv: &mut [(Fe<W>, u64)], used: &mut Vec<usize>, kernel: &mut Vec<u64>, st: &mut ScanStat, mut found: F) {
        let (d, n) = (self.d, self.n);
        let gf = &self.gf;
        let cc = gf.mul(self.b, gf.sqr(ui));
        let hc = self.half_trace.apply(cc);
        let tr_c = fx(fx(gf.sqr(hc), hc), cc);
        debug_assert!(tr_c[0] <= 1);
        let shift = 2 * d - 1;
        let tbit = n - shift;
        ws_cols.clear();
        for j in 0..d {
            let mut top = fshr(gf.mul(u, self.ell[j]), shift);
            fsetbit(&mut top, tbit, (self.trace_mask >> j) & 1);
            ws_cols.push(top);
        }
        ws_cols.push(fshr(u, shift));
        let mut rhs = fshr(gf.mul(u, hc), shift);
        fsetbit(&mut rhs, tbit, tr_c[0] & 1);
        let Some(part) = Self::solve_f2(ws_cols, rhs, piv, used, kernel) else {
            return;
        };
        st.consistent += 1;
        let dim = kernel.len();
        if dim > self.kernel_cap {
            st.truncated = true;
        }
        let dim = dim.min(self.kernel_cap);
        let dmask = (1u64 << d) - 1;
        for sel in 0u64..(1u64 << dim) {
            let mut comb = part;
            for i in 0..dim {
                if (sel >> i) & 1 == 1 {
                    comb ^= kernel[i];
                }
            }
            let s = comb & dmask;
            let eps = (comb >> d) & 1;
            let se = fe::<W>(s);
            let ls = fx(se, self.half_trace.apply(se));
            let p = gf.mul(u, fx(fx(ls, hc), fe(eps)));
            st.splits += 1;
            let x1 = if s == 0 {
                self.sqrt.apply(p)
            } else {
                let w = gf.mul(p, self.inv_s2[s as usize]);
                let y = self.half_trace.apply(w);
                if fx(gf.sqr(y), y) != w {
                    continue;
                }
                gf.mul(se, y)
            };
            let x2 = fx(x1, se);
            let (Some(a1), Some(a2)) = (fsmall(&x1, d), fsmall(&x2, d)) else {
                continue;
            };
            if a1 == 0 || a2 == 0 {
                continue;
            }
            st.in_range += 1;
            found(a1, a2);
        }
    }

    /// Port of the baseline scan (`decide`, all witnesses).
    pub fn scan(&self, r: Pt<W>, ws: &mut Scratch<W>, out: &mut Vec<[usize; 3]>) -> ScanStat {
        const BLOCK: usize = 256;
        let mut st = ScanStat::default();
        let mut lo = 0;
        while lo < self.points.len() {
            let hi = (lo + BLOCK).min(self.points.len());
            ws.qs.clear();
            self.add_many(r, &self.negated[lo..hi], &mut ws.qs, &mut ws.dens, &mut ws.acc);
            ws.uinv.clear();
            ws.uinv.extend(ws.qs.iter().map(|q| if q.inf || fz(&q.x) { fe(1) } else { q.x }));
            self.gf.batch_inv(&mut ws.uinv, &mut ws.acc);
            for off in 0..hi - lo {
                let q = ws.qs[off];
                if q.inf || fz(&q.x) {
                    continue;
                }
                st.leaves += 1;
                let ui = ws.uinv[off];
                let i = lo + off;
                self.leaf(q.x, ui, &mut ws.cols, &mut ws.piv, &mut ws.used, &mut ws.kernel, &mut st, |x1, x2| {
                    if let Some((j, k)) = self.verify(q, x1, x2) {
                        let mut t = [i, j, k];
                        t.sort_unstable();
                        out.push(t);
                    }
                });
            }
            lo = hi;
        }
        st
    }

    /// Plain pair enumeration in point form. Returns (pairs tested, hits).
    pub fn pairs(&self, r: Pt<W>, ws: &mut Scratch<W>, out: &mut Vec<[usize; 3]>) -> u64 {
        const BLOCK: usize = 256;
        let f = &self.gf;
        let np = self.points.len();
        let mut tested = 0u64;
        let mut qs: Vec<Pt<W>> = Vec::with_capacity(np);
        let mut lo = 0;
        while lo < np {
            let hi = (lo + BLOCK).min(np);
            self.add_many(r, &self.negated[lo..hi], &mut qs, &mut ws.dens, &mut ws.acc);
            lo = hi;
        }
        for i in 0..np {
            let q = qs[i];
            let mut lo = i;
            while lo < np {
                let hi = (lo + BLOCK).min(np);
                tested += (hi - lo) as u64;
                if q.inf {
                    lo = hi;
                    continue;
                }
                ws.dens.clear();
                ws.dens.extend(self.negated[lo..hi].iter().map(|p| if p.x == q.x { fe(1) } else { fx(p.x, q.x) }));
                f.batch_inv(&mut ws.dens, &mut ws.acc);
                for (off, &inv) in ws.dens.iter().enumerate() {
                    let m = self.negated[lo + off];
                    let t = if m.x == q.x {
                        // q = +-P_j: T is the point at infinity or 2 q
                        let t = self.add(q, m);
                        if t.inf {
                            continue;
                        }
                        t
                    } else {
                        let lambda = f.mul(fx(q.y, m.y), inv);
                        let x3 = fx(fx(fx(fx(f.sqr(lambda), lambda), q.x), m.x), self.a);
                        if fsmall(&x3, self.d).is_none() {
                            continue;
                        }
                        self.add_with_lambda(q, m, lambda)
                    };
                    if let Some(x) = fsmall(&t.x, self.d) {
                        let k = self.index_of[2 * x as usize + sign_bit(&t)];
                        assert!(k != usize::MAX, "rational point with abscissa in V missing from the base");
                        let mut tr = [i, lo + off, k];
                        tr.sort_unstable();
                        out.push(tr);
                    }
                }
                lo = hi;
            }
        }
        tested
    }

    /// P_i + P_j + P_k == R in the group.
    pub fn triple_ok(&self, t: [usize; 3], r: Pt<W>) -> bool {
        self.add(self.add(self.points[t[0]], self.points[t[1]]), self.points[t[2]]) == r
    }

    /// (all three abscissae lift, some signed sum has abscissa x_r)
    pub fn lift_check(&self, xs: [Fe<W>; 3], x_r: Fe<W>) -> (bool, bool) {
        let p: Vec<Option<Pt<W>>> = xs.iter().map(|&x| self.lift_x(x)).collect();
        if p.iter().any(|q| q.is_none()) {
            return (false, false);
        }
        let (p1, p2, p3) = (p[0].unwrap(), p[1].unwrap(), p[2].unwrap());
        for s2 in 0..2 {
            for s3 in 0..2 {
                let a = if s2 == 0 { p2 } else { self.neg(p2) };
                let b = if s3 == 0 { p3 } else { self.neg(p3) };
                let s = self.add(self.add(p1, a), b);
                if !s.inf && s.x == x_r {
                    return (true, true);
                }
            }
        }
        (true, false)
    }
}
