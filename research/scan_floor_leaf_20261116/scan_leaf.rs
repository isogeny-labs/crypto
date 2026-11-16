//! Scan-floor leaf pilot: standalone port of the `lin21-ht` 2+1 leaf
//! (`examples/ic_algebraic_floor/linear21.rs`) with two candidate arms,
//! B ("dual leaf") and C ("y-free t").  See PROTOCOL.md.
//!
//! Build: rustc -O -C target-cpu=native scan_leaf.rs
//! Everything that produces or processes a number is in this file.

use std::fmt::Write as _;
use std::hint::black_box;
use std::time::Instant;

// ───────────────────────── field F_2[z]/(f), one word ─────────────────────

#[inline(always)]
fn clmul(a: u64, b: u64) -> u128 {
    // SAFETY: built with target-cpu=native on a pclmulqdq host; `main` checks.
    unsafe {
        use std::arch::x86_64::*;
        let z = _mm_clmulepi64_si128::<0>(_mm_set_epi64x(0, a as i64), _mm_set_epi64x(0, b as i64));
        let lo = _mm_cvtsi128_si64(z) as u64;
        let hi = _mm_cvtsi128_si64(_mm_srli_si128::<8>(z)) as u64;
        ((hi as u128) << 64) | lo as u128
    }
}

fn pmod(mut c: u128, f: u64, n: u32) -> u64 {
    for i in (n..127).rev() {
        if (c >> i) & 1 == 1 {
            c ^= (f as u128) << (i - n);
        }
    }
    c as u64
}

fn pgcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        while a != 0 && (63 - a.leading_zeros()) >= (63 - b.leading_zeros()) {
            a ^= b << ((63 - a.leading_zeros()) - (63 - b.leading_zeros()));
        }
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn is_irreducible(f: u64, n: u32) -> bool {
    let sqk = |k: u32| -> u64 {
        let mut t = 2u64;
        for _ in 0..k {
            t = pmod(clmul(t, t), f, n);
        }
        t
    };
    if sqk(n) != 2 {
        return false;
    }
    let mut m = n;
    let mut p = 2;
    while p <= m {
        if m % p == 0 {
            if pgcd(f, sqk(n / p) ^ 2) != 1 {
                return false;
            }
            while m % p == 0 {
                m /= p;
            }
        }
        p += 1;
    }
    true
}

struct Gf {
    n: u32,
    irr: u64,
    mask: u64,
    terms: [u32; 8],
    nt: usize,
}

impl Gf {
    fn new(n: u32) -> Self {
        let hi = 1u64 << n;
        let mut low = 1u64;
        while !is_irreducible(hi | low, n) {
            low += 2;
        }
        let mut terms = [0u32; 8];
        let mut nt = 0;
        for e in 0..n {
            if (low >> e) & 1 == 1 {
                terms[nt] = e;
                nt += 1;
            }
        }
        assert!(2 * terms[nt - 1] <= n && n + terms[nt - 1] < 64, "two folds must suffice");
        Self { n, irr: hi | low, mask: hi - 1, terms, nt }
    }
    #[inline(always)]
    fn fold(&self, hi: u64) -> u64 {
        let mut t = 0;
        for i in 0..self.nt {
            t ^= hi << self.terms[i];
        }
        t
    }
    #[inline(always)]
    fn reduce(&self, c: u128) -> u64 {
        let lo = (c as u64) & self.mask;
        let hi = (c >> self.n) as u64;
        let t = lo ^ self.fold(hi);
        (t & self.mask) ^ self.fold(t >> self.n)
    }
    #[inline(always)]
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.reduce(clmul(a, b))
    }
    #[inline(always)]
    fn sqr(&self, a: u64) -> u64 {
        self.reduce(clmul(a, a))
    }
    fn sqr_k(&self, mut a: u64, k: u32) -> u64 {
        for _ in 0..k {
            a = self.sqr(a);
        }
        a
    }
    /// Itoh–Tsujii: a^(2^(n-1)-1), then one squaring.
    #[inline]
    fn inv(&self, a: u64) -> u64 {
        debug_assert!(a != 0);
        let e = self.n - 1;
        let bits = 32 - e.leading_zeros();
        let (mut r, mut k) = (a, 1u32);
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
    /// Montgomery's trick: 3(k-1) multiplications and one inversion.  No zero.
    #[inline(always)]
    fn batch_inv(&self, xs: &mut [u64], acc: &mut Vec<u64>) {
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
}

// ───────────────────────── counters ───────────────────────────────────────

const ST_T: usize = 0; // t stage: t = x(R - P1) and 1/t for every leaf
const ST_E: usize = 1; // half-trace elimination: system build + solve (and B's tests)
const ST_S: usize = 2; // root split
const ST_M: usize = 3; // membership checks and group verification
const STAGES: [&str; 4] = ["t_stage", "ht_elimination", "root_split", "membership"];
const MUL: usize = 0;
const SQR: usize = 1;
const INV: usize = 2;
const LK: usize = 3;
const WOP: usize = 4;
const CATS: [&str; 5] = ["mul", "sqr", "inv", "lookup", "wop"];

#[derive(Clone, Copy, Default)]
struct Cnt {
    c: [[u64; 5]; 4],
    trials: u64,
    consistent: u64,
    splits: u64,
    verify_adds: u64,
    pretest_pass: u64,
    truncated: bool,
}

impl Cnt {
    fn add(&mut self, o: &Cnt) {
        for s in 0..4 {
            for k in 0..5 {
                self.c[s][k] += o.c[s][k];
            }
        }
        self.trials += o.trials;
        self.consistent += o.consistent;
        self.splits += o.splits;
        self.verify_adds += o.verify_adds;
        self.pretest_pass += o.pretest_pass;
        self.truncated |= o.truncated;
    }
    fn priced_stage(&self, s: usize, w: &[f64; 5]) -> f64 {
        (0..5).map(|k| self.c[s][k] as f64 * w[k]).sum()
    }
    fn priced(&self, w: &[f64; 5]) -> f64 {
        (0..4).map(|s| self.priced_stage(s, w)).sum()
    }
}

macro_rules! ct {
    ($C:ident, $cnt:expr, $st:expr, $cat:expr, $k:expr) => {
        if $C {
            $cnt.c[$st][$cat] += $k as u64;
        }
    };
}

fn prices(n: u32) -> [(&'static str, [f64; 5]); 3] {
    let n = n as f64;
    [
        ("hw", [8.0, 8.0, 8.0 * (n + 8.0), 1.0, 1.0]),
        ("clmul_min", [3.0, 3.0, 3.0 * (n + 8.0), 1.0, 1.0]),
        ("bit_serial", [2.0 * n, n, 2.0 * n * (n + 8.0), 1.0, 1.0]),
    ]
}

// ───────────────────────── linear maps as byte tables ─────────────────────

struct LinMap {
    t: Vec<[u64; 256]>,
}

impl LinMap {
    fn new(n: u32, image_of_bit: impl Fn(u32) -> u64) -> Self {
        let bytes = n.div_ceil(8) as usize;
        let mut t = vec![[0u64; 256]; bytes];
        for (j, row) in t.iter_mut().enumerate() {
            for v in 1usize..256 {
                let low = v.trailing_zeros();
                let bit = 8 * j as u32 + low;
                let img = if bit < n { image_of_bit(bit) } else { 0 };
                row[v] = row[v & (v - 1)] ^ img;
            }
        }
        Self { t }
    }
    #[inline(always)]
    fn apply(&self, mut x: u64) -> u64 {
        let mut acc = 0;
        for row in &self.t {
            acc ^= row[(x & 0xff) as usize];
            x >>= 8;
        }
        acc
    }
}

// ───────────────────────── F_2 elimination ────────────────────────────────

/// Port of `solve_f2`: particular solution and kernel basis as column combinations.
#[inline(always)]
fn solve_f2<const C: bool>(cols: &[u64], rhs: u64, cnt: &mut Cnt, st: usize, kernel: &mut Vec<u64>) -> Option<u64> {
    let mut piv: [(u64, u64); 64] = [(0, 0); 64];
    kernel.clear();
    for (k, &col) in cols.iter().enumerate() {
        let (mut v, mut comb) = (col, 1u64 << k);
        ct!(C, cnt, st, WOP, 1);
        while v != 0 {
            let top = 63 - v.leading_zeros() as usize;
            ct!(C, cnt, st, WOP, 2);
            if piv[top].0 == 0 {
                piv[top] = (v, comb);
                ct!(C, cnt, st, WOP, 1);
                break;
            }
            v ^= piv[top].0;
            comb ^= piv[top].1;
            ct!(C, cnt, st, WOP, 2);
        }
        if v == 0 {
            kernel.push(comb);
            ct!(C, cnt, st, WOP, 1);
        }
    }
    let (mut v, mut comb) = (rhs, 0u64);
    while v != 0 {
        let top = 63 - v.leading_zeros() as usize;
        ct!(C, cnt, st, WOP, 2);
        if piv[top].0 == 0 {
            return None;
        }
        v ^= piv[top].0;
        comb ^= piv[top].1;
        ct!(C, cnt, st, WOP, 2);
    }
    Some(comb)
}

/// Is `target` in the span of `vecs`?  No combination tracking.
#[inline(always)]
fn in_span<const C: bool>(vecs: &[u64], target: u64, cnt: &mut Cnt, st: usize) -> bool {
    let mut piv = [0u64; 64];
    for &col in vecs {
        let mut v = col;
        while v != 0 {
            let top = 63 - v.leading_zeros() as usize;
            ct!(C, cnt, st, WOP, 2);
            if piv[top] == 0 {
                piv[top] = v;
                ct!(C, cnt, st, WOP, 1);
                break;
            }
            v ^= piv[top];
            ct!(C, cnt, st, WOP, 1);
        }
    }
    let mut v = target;
    while v != 0 {
        let top = 63 - v.leading_zeros() as usize;
        ct!(C, cnt, st, WOP, 2);
        if piv[top] == 0 {
            return false;
        }
        v ^= piv[top];
        ct!(C, cnt, st, WOP, 1);
    }
    true
}

/// Basis of {m : parity(m & row) = 0 for every row}, m of `ncols` bits.
fn nullspace(rows: &[u64], ncols: u32) -> Vec<u64> {
    let mut red: Vec<(u32, u64)> = Vec::new(); // (pivot bit, row), fully reduced
    for &r0 in rows {
        let mut r = r0;
        for &(pb, pr) in &red {
            if (r >> pb) & 1 == 1 {
                r ^= pr;
            }
        }
        if r == 0 {
            continue;
        }
        let pb = r.trailing_zeros();
        for e in red.iter_mut() {
            if (e.1 >> pb) & 1 == 1 {
                e.1 ^= r;
            }
        }
        red.push((pb, r));
    }
    let mut out = Vec::new();
    for f in 0..ncols {
        if red.iter().any(|&(pb, _)| pb == f) {
            continue;
        }
        let mut m = 1u64 << f;
        for &(pb, pr) in &red {
            if (pr >> f) & 1 == 1 {
                m |= 1u64 << pb;
            }
        }
        out.push(m);
    }
    for &m in &out {
        for &r in rows {
            assert!((m & r).count_ones() % 2 == 0);
        }
    }
    out
}

// ───────────────────────── curve y² + xy = x³ + a x² + b ──────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pt {
    x: u64,
    y: u64,
    inf: bool,
}
const INF: Pt = Pt { x: 0, y: 0, inf: true };

#[inline(always)]
fn sign_bit(p: Pt) -> usize {
    usize::from(p.y > (p.x ^ p.y))
}

#[derive(Default)]
struct Scratch {
    cols: Vec<u64>,
    kernel: Vec<u64>,
    rows: Vec<u64>,
    dens: Vec<u64>,
    acc: Vec<u64>,
    qs: Vec<Pt>,
    uinv: Vec<u64>,
    ts: Vec<u64>,
    uis: Vec<u64>,
    pp: Vec<u64>,
    rec: Option<Vec<(u64, u64)>>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Arm {
    Base,
    B1,
    B2,
}

struct Ctx {
    gf: Gf,
    n: u32,
    d: usize,
    a: u64,
    b: u64,
    sqrt_b: u64,
    points: Vec<Pt>,
    negated: Vec<Pt>,
    index_of: Vec<usize>,
    ell: Vec<u64>,
    trace_mask: u64,
    half_trace: LinMap,
    sqrt: LinMap,
    inv_s2: Vec<u64>,
    kernel_cap: usize,
    // B1: dual tables over a basis of V-perp.
    tmap: LinMap,
    t2map: LinMap,
    vperp_dim: usize,
    // C: per-abscissa table sqrt(b)/x and the pairing of the base by abscissa.
    sb_ix: Vec<u64>,
    pairs: Vec<(u64, u32, u32)>,
    singles: Vec<u32>,
    setup_base_ns: u64,
    setup_b1_ns: u64,
    setup_c_ns: u64,
}

impl Ctx {
    fn half(gf: &Gf, a: u64) -> u64 {
        let (mut t, mut h) = (a, a);
        for _ in 0..(gf.n - 1) / 2 {
            t = gf.sqr(gf.sqr(t));
            h ^= t;
        }
        h
    }
    fn trace(gf: &Gf, a: u64) -> u64 {
        let (mut t, mut tr) = (a, a);
        for _ in 1..gf.n {
            t = gf.sqr(t);
            tr ^= t;
        }
        assert!(tr <= 1);
        tr
    }

    fn new(n: u32, d: usize) -> Self {
        assert!(n % 2 == 1 && 2 * d - 1 <= n as usize && 3 * d <= 64);
        let t0 = Instant::now();
        let gf = Gf::new(n);
        let (a, b) = (1u64, 1u64);
        let sqrt_b = gf.sqr_k(b, n - 1);
        // Factor base: every rational point with x in V, both signs, x ascending.
        let mut points = Vec::new();
        for x in 0u64..(1u64 << d) {
            if x == 0 {
                points.push(Pt { x: 0, y: sqrt_b, inf: false });
                continue;
            }
            let ix = gf.inv(x);
            let rhs = x ^ a ^ gf.mul(b, gf.sqr(ix));
            let w = Self::half(&gf, rhs);
            if gf.sqr(w) ^ w != rhs {
                continue;
            }
            let y = gf.mul(x, w);
            points.push(Pt { x, y, inf: false });
            points.push(Pt { x, y: x ^ y, inf: false });
        }
        let mut index_of = vec![usize::MAX; 2usize << d];
        for (i, p) in points.iter().enumerate() {
            index_of[2 * p.x as usize + sign_bit(*p)] = i;
        }
        let negated: Vec<Pt> = points.iter().map(|p| Pt { x: p.x, y: p.x ^ p.y, inf: false }).collect();
        let ell: Vec<u64> = (0..d as u32).map(|j| (1u64 << j) ^ Self::half(&gf, 1u64 << j)).collect();
        let trace_mask = (0..d as u32).fold(0u64, |m, j| m | (Self::trace(&gf, 1u64 << j) << j));
        let half_trace = LinMap::new(n, |bit| Self::half(&gf, 1u64 << bit));
        let sqrt = LinMap::new(n, |bit| gf.sqr_k(1u64 << bit, n - 1));
        let mut inv_s2: Vec<u64> = (0..1u64 << d).collect();
        inv_s2[0] = 1;
        gf.batch_inv(&mut inv_s2, &mut Vec::new());
        for v in inv_s2.iter_mut() {
            *v = gf.sqr(*v);
        }
        let setup_base_ns = t0.elapsed().as_nanos() as u64;

        // B1: V-perp under the trace form, G(m) = m² + m on its basis.
        let t0 = Instant::now();
        let zp: Vec<u64> = {
            let mut v = vec![1u64];
            for _ in 1..2 * n {
                let w = v[v.len() - 1] << 1;
                v.push(if (w >> n) & 1 == 1 { w ^ gf.irr } else { w });
            }
            v
        };
        let trz: Vec<u64> = zp.iter().map(|&e| Self::trace(&gf, e)).collect();
        let rows: Vec<u64> = (0..d).map(|k| (0..n as usize).fold(0u64, |m, j| m | (trz[j + k] << j))).collect();
        let mb = nullspace(&rows, n);
        assert_eq!(mb.len(), n as usize - d);
        let gb: Vec<u64> = mb.iter().map(|&m| gf.sqr(m) ^ m).collect();
        let msb: Vec<u64> = mb.iter().map(|&m| gf.mul(m, sqrt_b)).collect();
        let tmap = LinMap::new(n, |bit| {
            gb.iter().enumerate().fold(0u64, |acc, (i, &g)| acc | (Self::trace(&gf, gf.mul(g, 1u64 << bit)) << i))
        });
        let t2map = LinMap::new(n, |bit| {
            msb.iter().enumerate().fold(0u64, |acc, (i, &m)| acc | (Self::trace(&gf, gf.mul(m, 1u64 << bit)) << i))
        });
        let setup_b1_ns = t0.elapsed().as_nanos() as u64;

        // C: sqrt(b)/x over V, and the base paired by abscissa.
        let t0 = Instant::now();
        let mut sb_ix: Vec<u64> = (0..1u64 << d).collect();
        sb_ix[0] = 1;
        gf.batch_inv(&mut sb_ix, &mut Vec::new());
        for v in sb_ix.iter_mut() {
            *v = gf.mul(*v, sqrt_b);
        }
        let (mut pairs, mut singles) = (Vec::new(), Vec::new());
        let mut i = 0;
        while i < points.len() {
            if i + 1 < points.len() && points[i + 1].x == points[i].x && points[i].x != 0 {
                pairs.push((points[i].x, i as u32, i as u32 + 1));
                i += 2;
            } else {
                singles.push(i as u32);
                i += 1;
            }
        }
        let setup_c_ns = t0.elapsed().as_nanos() as u64;
        Self {
            vperp_dim: mb.len(),
            gf,
            n,
            d,
            a,
            b,
            sqrt_b,
            points,
            negated,
            index_of,
            ell,
            trace_mask,
            half_trace,
            sqrt,
            inv_s2,
            kernel_cap: 16,
            tmap,
            t2map,
            sb_ix,
            pairs,
            singles,
            setup_base_ns,
            setup_b1_ns,
            setup_c_ns,
        }
    }

    #[inline(always)]
    fn times_z(&self, v: u64) -> u64 {
        let w = v << 1;
        if (w >> self.n) & 1 == 1 {
            w ^ self.gf.irr
        } else {
            w
        }
    }

    fn double(&self, p: Pt) -> Pt {
        if p.inf || p.x == 0 {
            return INF;
        }
        let f = &self.gf;
        let l = p.x ^ f.mul(p.y, f.inv(p.x));
        let x3 = f.sqr(l) ^ l ^ self.a;
        let y3 = f.sqr(p.x) ^ f.mul(l ^ 1, x3);
        Pt { x: x3, y: y3, inf: false }
    }

    #[inline(always)]
    fn add_with_lambda(&self, p: Pt, q: Pt, lambda: u64) -> Pt {
        let f = &self.gf;
        let x3 = f.sqr(lambda) ^ lambda ^ p.x ^ q.x ^ self.a;
        let y3 = f.mul(lambda, p.x ^ x3) ^ x3 ^ p.y;
        Pt { x: x3, y: y3, inf: false }
    }

    /// `P + Q`; one inversion, two multiplications, one squaring when generic.
    fn add(&self, p: Pt, q: Pt) -> Pt {
        if p.inf {
            return q;
        }
        if q.inf {
            return p;
        }
        if p.x == q.x {
            if p.y ^ q.y == p.x {
                return INF;
            }
            return self.double(p);
        }
        let lambda = self.gf.mul(p.y ^ q.y, self.gf.inv(p.x ^ q.x));
        self.add_with_lambda(p, q, lambda)
    }

    /// Port of `FastCurve::add_many`: `P + Q_j`, one inversion for the slice.
    #[inline(always)]
    fn add_many<const C: bool>(&self, p: Pt, qs: &[Pt], out: &mut Vec<Pt>, dens: &mut Vec<u64>, acc: &mut Vec<u64>, cnt: &mut Cnt) {
        let f = &self.gf;
        dens.clear();
        dens.extend(qs.iter().map(|q| if p.inf || q.inf || q.x == p.x { 1 } else { p.x ^ q.x }));
        ct!(C, cnt, ST_T, WOP, 2 * qs.len());
        f.batch_inv(dens, acc);
        ct!(C, cnt, ST_T, MUL, 3 * (qs.len() - 1));
        ct!(C, cnt, ST_T, INV, 1);
        let mut double_p: Option<Pt> = None;
        for (q, &inv) in qs.iter().zip(dens.iter()) {
            if p.inf || q.inf || q.x == p.x {
                let sum = if p.inf {
                    *q
                } else if q.inf {
                    p
                } else if p.y ^ q.y == p.x {
                    INF
                } else {
                    ct!(C, cnt, ST_T, INV, 1);
                    ct!(C, cnt, ST_T, MUL, 2);
                    ct!(C, cnt, ST_T, SQR, 2);
                    *double_p.get_or_insert_with(|| self.double(p))
                };
                out.push(sum);
                continue;
            }
            let lambda = f.mul(p.y ^ q.y, inv);
            out.push(self.add_with_lambda(p, *q, lambda));
            ct!(C, cnt, ST_T, MUL, 2);
            ct!(C, cnt, ST_T, SQR, 1);
            ct!(C, cnt, ST_T, WOP, 9);
        }
    }

    /// Either sign of either summand against `Q` in the group (port of `verify`).
    #[inline(always)]
    fn verify<const C: bool>(&self, q: Pt, x1: u64, x2: u64, cnt: &mut Cnt) -> Option<(usize, usize)> {
        for s1 in 0..2 {
            let j = self.index_of[2 * x1 as usize + s1];
            ct!(C, cnt, ST_M, LK, 1);
            if j == usize::MAX {
                continue;
            }
            for s2 in 0..2 {
                let k = self.index_of[2 * x2 as usize + s2];
                ct!(C, cnt, ST_M, LK, 1);
                if k == usize::MAX {
                    continue;
                }
                if C {
                    cnt.verify_adds += 1;
                }
                ct!(C, cnt, ST_M, INV, 1);
                ct!(C, cnt, ST_M, MUL, 2);
                ct!(C, cnt, ST_M, SQR, 1);
                ct!(C, cnt, ST_M, WOP, 10);
                if self.add(self.points[j], self.points[k]) == q {
                    return Some((j, k));
                }
            }
        }
        None
    }

    /// B1: the dual (left-kernel) verdict of one leaf from `1/t` alone.
    #[inline(always)]
    fn dual_test<const C: bool>(&self, ui: u64, cnt: &mut Cnt, rows: &mut Vec<u64>) -> bool {
        let nb = self.half_trace.t.len();
        rows.clear();
        let mut a = ui;
        for _ in 0..2 * self.d - 1 {
            rows.push(self.tmap.apply(a));
            a = self.times_z(a);
        }
        let tau = self.t2map.apply(ui);
        ct!(C, cnt, ST_E, LK, 2 * self.d * nb);
        ct!(C, cnt, ST_E, WOP, 2 * self.d * 2 * nb + 3 * (2 * self.d - 1));
        in_span::<C>(rows, tau, cnt, ST_E)
    }

    /// Port of `pair_half_trace` + `split_root_and_verify` up to the range
    /// check; `found(x1, x2)` receives every split that lands in `V`.
    /// `level`: 2 = system build only, 3 = + solve, 4 = all.  `pretest`: B2.
    #[inline(always)]
    fn leaf<const C: bool, F: FnMut(u64, u64, &mut Cnt)>(
        &self,
        u: u64,
        ui: u64,
        level: u8,
        pretest: bool,
        cnt: &mut Cnt,
        cols: &mut Vec<u64>,
        kernel: &mut Vec<u64>,
        sink: &mut u64,
        mut found: F,
    ) {
        let (d, n) = (self.d, self.n as usize);
        let gf = &self.gf;
        let nb = self.half_trace.t.len();
        let cc = gf.mul(self.b, gf.sqr(ui));
        let hc = self.half_trace.apply(cc);
        let tr_c = gf.sqr(hc) ^ hc ^ cc;
        debug_assert!(tr_c <= 1);
        let shift = 2 * d - 1;
        let tbit = n - shift;
        cols.clear();
        for j in 0..d {
            let top = gf.mul(u, self.ell[j]) >> shift;
            cols.push(top | (((self.trace_mask >> j) & 1) << tbit));
        }
        cols.push(u >> shift);
        let rhs = (gf.mul(u, hc) >> shift) | (tr_c << tbit);
        ct!(C, cnt, ST_E, MUL, d + 2);
        ct!(C, cnt, ST_E, SQR, 2);
        ct!(C, cnt, ST_E, LK, nb);
        ct!(C, cnt, ST_E, WOP, 2 * nb + 2 + 5 * d + 1 + 3);
        if level == 2 {
            *sink ^= rhs ^ cols[d] ^ cols[0];
            black_box(&cols);
            return;
        }
        if pretest {
            if !in_span::<C>(cols, rhs, cnt, ST_E) {
                return;
            }
            if C {
                cnt.pretest_pass += 1;
            }
        }
        let Some(part) = solve_f2::<C>(cols, rhs, cnt, ST_E, kernel) else {
            return;
        };
        if C {
            cnt.consistent += 1;
        }
        if level == 3 {
            *sink ^= part;
            return;
        }
        let dim = kernel.len();
        if dim > self.kernel_cap {
            cnt.truncated = true;
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
            let ls = s ^ self.half_trace.apply(s);
            let p = gf.mul(u, ls ^ hc ^ eps);
            ct!(C, cnt, ST_S, MUL, 1);
            ct!(C, cnt, ST_S, LK, nb);
            ct!(C, cnt, ST_S, WOP, 2 * nb + 6 + 2 * dim);
            debug_assert!(p >> shift == 0);
            if C {
                cnt.splits += 1;
            }
            let x1 = if s == 0 {
                ct!(C, cnt, ST_S, LK, nb);
                ct!(C, cnt, ST_S, WOP, 2 * nb);
                self.sqrt.apply(p)
            } else {
                let w = gf.mul(p, self.inv_s2[s as usize]);
                let y = self.half_trace.apply(w);
                ct!(C, cnt, ST_S, MUL, 1);
                ct!(C, cnt, ST_S, SQR, 1);
                ct!(C, cnt, ST_S, LK, nb + 1);
                ct!(C, cnt, ST_S, WOP, 2 * nb + 2);
                if gf.sqr(y) ^ y != w {
                    continue;
                }
                ct!(C, cnt, ST_S, MUL, 1);
                gf.mul(s, y)
            };
            let x2 = x1 ^ s;
            ct!(C, cnt, ST_M, WOP, 7);
            if x1 == 0 || x2 == 0 || x1 >> d != 0 || x2 >> d != 0 {
                continue;
            }
            found(x1, x2, cnt);
        }
    }

    /// The baseline scan (port of `decide`, all witnesses) and the B arms,
    /// which share its t stage.  `level` 1 = t stage only.
    #[inline(always)]
    fn scan<const C: bool>(&self, r: Pt, level: u8, arm: Arm, cnt: &mut Cnt, ws: &mut Scratch, out: &mut Vec<[usize; 3]>) -> u64 {
        const BLOCK: usize = 256;
        let mut sink = 0u64;
        let mut lo = 0;
        while lo < self.points.len() {
            let hi = (lo + BLOCK).min(self.points.len());
            ws.qs.clear();
            self.add_many::<C>(r, &self.negated[lo..hi], &mut ws.qs, &mut ws.dens, &mut ws.acc, cnt);
            ws.uinv.clear();
            ws.uinv.extend(ws.qs.iter().map(|q| if q.inf || q.x == 0 { 1 } else { q.x }));
            self.gf.batch_inv(&mut ws.uinv, &mut ws.acc);
            ct!(C, cnt, ST_T, WOP, 2 * (hi - lo));
            ct!(C, cnt, ST_T, MUL, 3 * (hi - lo - 1));
            ct!(C, cnt, ST_T, INV, 1);
            if level == 1 {
                black_box(&ws.qs);
                for (q, &ui) in ws.qs.iter().zip(ws.uinv.iter()) {
                    sink ^= q.x ^ ui;
                }
                lo = hi;
                continue;
            }
            for off in 0..hi - lo {
                let q = ws.qs[off];
                if C {
                    cnt.trials += 1;
                }
                if q.inf || q.x == 0 {
                    continue;
                }
                let ui = ws.uinv[off];
                if C {
                    if let Some(v) = ws.rec.as_mut() {
                        v.push((q.x, ui));
                    }
                }
                if arm == Arm::B1 {
                    if !self.dual_test::<C>(ui, cnt, &mut ws.rows) {
                        continue;
                    }
                    if C {
                        cnt.pretest_pass += 1;
                    }
                }
                let i = lo + off;
                self.leaf::<C, _>(q.x, ui, level, arm == Arm::B2, cnt, &mut ws.cols, &mut ws.kernel, &mut sink, |x1, x2, cnt| {
                    if let Some((j, k)) = self.verify::<C>(q, x1, x2, cnt) {
                        let mut t = [i, j, k];
                        t.sort_unstable();
                        out.push(t);
                    }
                });
            }
            lo = hi;
        }
        sink
    }

    /// One leaf through the baseline path (C's degenerate abscissae).
    #[inline(never)]
    fn fallback_leaf<const C: bool>(&self, r: Pt, i: usize, level: u8, cnt: &mut Cnt, ws: &mut Scratch, out: &mut Vec<[usize; 3]>, sink: &mut u64) {
        let q = self.add(r, self.negated[i]);
        ct!(C, cnt, ST_T, INV, 1);
        ct!(C, cnt, ST_T, MUL, 2);
        ct!(C, cnt, ST_T, SQR, 1);
        ct!(C, cnt, ST_T, WOP, 10);
        if C {
            cnt.trials += 1;
        }
        if q.inf || q.x == 0 {
            return;
        }
        let ui = self.gf.inv(q.x);
        ct!(C, cnt, ST_T, INV, 1);
        if C {
            if let Some(v) = ws.rec.as_mut() {
                v.push((q.x, ui));
            }
        }
        if level == 1 {
            *sink ^= q.x ^ ui;
            return;
        }
        self.leaf::<C, _>(q.x, ui, level, false, cnt, &mut ws.cols, &mut ws.kernel, sink, |x1, x2, cnt| {
            if let Some((j, k)) = self.verify::<C>(q, x1, x2, cnt) {
                let mut t = [i, j, k];
                t.sort_unstable();
                out.push(t);
            }
        });
    }

    /// Arm C: `t` and `1/t` for both signs of every base abscissa from the
    /// quadratic `S3(x, t, xR) = 0`; no y, no group addition.
    #[inline(always)]
    fn scan_c<const C: bool>(&self, r: Pt, level: u8, cnt: &mut Cnt, ws: &mut Scratch, out: &mut Vec<[usize; 3]>) -> u64 {
        const PBLOCK: usize = 128;
        let gf = &self.gf;
        let nb = self.half_trace.t.len();
        let mut sink = 0u64;
        if r.inf || r.x == 0 {
            for i in 0..self.points.len() {
                self.fallback_leaf::<C>(r, i, level, cnt, ws, out, &mut sink);
            }
            return sink;
        }
        for idx in 0..self.singles.len() {
            let i = self.singles[idx] as usize;
            self.fallback_leaf::<C>(r, i, level, cnt, ws, out, &mut sink);
        }
        let xr = r.x;
        let a0 = xr ^ gf.mul(self.sqrt_b, gf.inv(xr));
        ct!(C, cnt, ST_T, INV, 1);
        ct!(C, cnt, ST_T, MUL, 1);
        let mut lo = 0;
        while lo < self.pairs.len() {
            let hi = (lo + PBLOCK).min(self.pairs.len());
            ws.dens.clear();
            ws.pp.clear();
            for &(x, _, _) in &self.pairs[lo..hi] {
                let e = x ^ xr;
                let p = gf.mul(x, xr);
                let dd = gf.mul(e, p ^ self.sqrt_b);
                ws.pp.push(p);
                ws.dens.push(if dd == 0 { 1 } else { dd });
            }
            ct!(C, cnt, ST_T, MUL, 2 * (hi - lo));
            ct!(C, cnt, ST_T, WOP, 4 * (hi - lo));
            gf.batch_inv(&mut ws.dens, &mut ws.acc);
            ct!(C, cnt, ST_T, MUL, 3 * (hi - lo - 1));
            ct!(C, cnt, ST_T, INV, 1);
            ws.ts.clear();
            ws.uis.clear();
            for off in 0..hi - lo {
                let (x, _, _) = self.pairs[lo + off];
                let e = x ^ xr;
                let p = ws.pp[off];
                let qq = p ^ self.sqrt_b;
                ct!(C, cnt, ST_T, WOP, 3);
                if e == 0 || qq == 0 {
                    // Degenerate: x = xR or x·xR = sqrt(b); marked, done below.
                    ws.ts.push(0);
                    ws.ts.push(0);
                    ws.uis.push(0);
                    ws.uis.push(0);
                    continue;
                }
                let id2 = gf.sqr(ws.dens[off]);
                let g = gf.mul(p, id2);
                let k = gf.mul(g, gf.sqr(qq));
                let kp = gf.mul(g, gf.sqr(e));
                let aa = x ^ self.sb_ix[x as usize] ^ a0;
                let w = self.half_trace.apply(aa) ^ aa;
                let t1 = gf.mul(k, w);
                let v = gf.mul(kp, w);
                ws.ts.push(t1);
                ws.uis.push(v ^ kp);
                ws.ts.push(t1 ^ k);
                ws.uis.push(v);
                ct!(C, cnt, ST_T, MUL, 5);
                ct!(C, cnt, ST_T, SQR, 3);
                ct!(C, cnt, ST_T, LK, nb + 1);
                ct!(C, cnt, ST_T, WOP, 2 * nb + 5);
            }
            if level == 1 {
                black_box(&ws.ts);
                for (&t, &ui) in ws.ts.iter().zip(ws.uis.iter()) {
                    sink ^= t ^ ui;
                }
            }
            for off in 0..hi - lo {
                let (_, ip, im) = self.pairs[lo + off];
                let (ip, im) = (ip as usize, im as usize);
                if ws.ts[2 * off] == 0 {
                    self.fallback_leaf::<C>(r, ip, level, cnt, ws, out, &mut sink);
                    self.fallback_leaf::<C>(r, im, level, cnt, ws, out, &mut sink);
                    continue;
                }
                if level == 1 {
                    continue;
                }
                for which in 0..2 {
                    let (u, ui) = (ws.ts[2 * off + which], ws.uis[2 * off + which]);
                    if C {
                        cnt.trials += 1;
                        if let Some(v) = ws.rec.as_mut() {
                            v.push((u, ui));
                        }
                    }
                    self.leaf::<C, _>(u, ui, level, false, cnt, &mut ws.cols, &mut ws.kernel, &mut sink, |x1, x2, cnt| {
                        // Rare: recover Q = R ∓ P and its base index by a group addition.
                        let qa = self.add(r, self.negated[ip]);
                        ct!(C, cnt, ST_M, INV, 1);
                        ct!(C, cnt, ST_M, MUL, 2);
                        ct!(C, cnt, ST_M, SQR, 1);
                        ct!(C, cnt, ST_M, WOP, 11);
                        let (q, i) = if !qa.inf && qa.x == u {
                            (qa, ip)
                        } else {
                            ct!(C, cnt, ST_M, INV, 1);
                            ct!(C, cnt, ST_M, MUL, 2);
                            ct!(C, cnt, ST_M, SQR, 1);
                            ct!(C, cnt, ST_M, WOP, 10);
                            (self.add(r, self.negated[im]), im)
                        };
                        assert!(!q.inf && q.x == u, "root of S3 is not x(R -+ P)");
                        if let Some((j, k)) = self.verify::<C>(q, x1, x2, cnt) {
                            let mut t = [i, j, k];
                            t.sort_unstable();
                            out.push(t);
                        }
                    });
                }
            }
            lo = hi;
        }
        sink
    }

    fn on_curve(&self, p: Pt) -> bool {
        let f = &self.gf;
        let x2 = f.sqr(p.x);
        f.sqr(p.y) ^ f.mul(p.x, p.y) == f.mul(x2, p.x) ^ f.mul(self.a, x2) ^ self.b
    }

    fn random_point(&self, rng: &mut Rng) -> Pt {
        let f = &self.gf;
        loop {
            let x = rng.next() & f.mask;
            if x == 0 {
                continue;
            }
            let rhs = x ^ self.a ^ f.mul(self.b, f.sqr(f.inv(x)));
            let w = self.half_trace.apply(rhs);
            if f.sqr(w) ^ w != rhs {
                continue;
            }
            let p = Pt { x, y: f.mul(x, w) ^ if rng.next() & 1 == 1 { x } else { 0 }, inf: false };
            assert!(self.on_curve(p));
            return p;
        }
    }
}

// ───────────────────────── rng, statistics ────────────────────────────────

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    fn below(&mut self, m: u64) -> u64 {
        self.next() % m
    }
}

/// Ratio of sums `num/den` over targets, with a paired bootstrap 95% interval.
fn ratio_ci(num: &[u64], den: &[u64], rng: &mut Rng) -> (f64, f64, f64) {
    let sum = |v: &[u64]| v.iter().map(|&x| x as f64).sum::<f64>();
    let point = sum(num) / sum(den);
    let m = num.len();
    let mut bs: Vec<f64> = (0..2000)
        .map(|_| {
            let (mut a, mut b) = (0f64, 0f64);
            for _ in 0..m {
                let i = rng.below(m as u64) as usize;
                a += num[i] as f64;
                b += den[i] as f64;
            }
            a / b
        })
        .collect();
    bs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (point, bs[49], bs[1949])
}

// ───────────────────────── exhaustive truth (pair enumeration) ────────────

struct PairTable {
    v: Vec<(u64, u64, u32, u32)>,
}

impl PairTable {
    fn new(cx: &Ctx) -> Self {
        let mut v = Vec::new();
        for j in 0..cx.points.len() {
            if cx.points[j].x == 0 {
                continue;
            }
            for k in j..cx.points.len() {
                if cx.points[k].x == 0 {
                    continue;
                }
                let s = cx.add(cx.points[j], cx.points[k]);
                if !s.inf {
                    v.push((s.x, s.y, j as u32, k as u32));
                }
            }
        }
        v.sort_unstable();
        Self { v }
    }
    fn truth(&self, cx: &Ctx, r: Pt) -> Vec<[usize; 3]> {
        let mut out = Vec::new();
        for i in 0..cx.points.len() {
            let q = cx.add(r, cx.negated[i]);
            if q.inf || q.x == 0 {
                continue;
            }
            let mut at = self.v.partition_point(|e| (e.0, e.1) < (q.x, q.y));
            while at < self.v.len() && (self.v[at].0, self.v[at].1) == (q.x, q.y) {
                let mut t = [i, self.v[at].2 as usize, self.v[at].3 as usize];
                t.sort_unstable();
                out.push(t);
                at += 1;
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }
}

fn canon(mut v: Vec<[usize; 3]>) -> Vec<[usize; 3]> {
    v.sort_unstable();
    v.dedup();
    v
}

// ───────────────────────── the experiment ─────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TArm {
    A1,
    A2,
    B1,
    B2,
    C,
    L1a,
    L1b,
    L2,
    L3,
    C1,
}

impl TArm {
    fn name(self) -> &'static str {
        match self {
            TArm::A1 => "A1",
            TArm::A2 => "A2",
            TArm::B1 => "B1",
            TArm::B2 => "B2",
            TArm::C => "C",
            TArm::L1a => "L1a",
            TArm::L1b => "L1b",
            TArm::L2 => "L2",
            TArm::L3 => "L3",
            TArm::C1 => "C1",
        }
    }
}

#[inline(never)]
fn run_timed(cx: &Ctx, arm: TArm, r: Pt, ws: &mut Scratch, out: &mut Vec<[usize; 3]>) -> (u64, usize, u64) {
    let mut cnt = Cnt::default();
    out.clear();
    let r = black_box(r);
    let t = Instant::now();
    let sink = match arm {
        TArm::A1 | TArm::A2 => cx.scan::<false>(r, 4, Arm::Base, &mut cnt, ws, out),
        TArm::B1 => cx.scan::<false>(r, 4, Arm::B1, &mut cnt, ws, out),
        TArm::B2 => cx.scan::<false>(r, 4, Arm::B2, &mut cnt, ws, out),
        TArm::C => cx.scan_c::<false>(r, 4, &mut cnt, ws, out),
        TArm::L1a | TArm::L1b => cx.scan::<false>(r, 1, Arm::Base, &mut cnt, ws, out),
        TArm::L2 => cx.scan::<false>(r, 2, Arm::Base, &mut cnt, ws, out),
        TArm::L3 => cx.scan::<false>(r, 3, Arm::Base, &mut cnt, ws, out),
        TArm::C1 => cx.scan_c::<false>(r, 1, &mut cnt, ws, out),
    };
    let ns = t.elapsed().as_nanos() as u64;
    black_box(sink);
    (ns, out.len(), sink)
}

fn jf(v: f64) -> String {
    if v.is_finite() {
        format!("{v:.6}")
    } else {
        "null".into()
    }
}

struct CellOut {
    n: u32,
    json: String,
    // for tables and decisions
    base: Cnt,
    arms: Vec<(&'static str, Cnt)>,
    n_no: usize,
    wall: Vec<(&'static str, (f64, f64, f64))>,
    aa: (f64, f64, f64),
    tstage_aa: (f64, f64, f64),
    tstage_c: Option<(f64, f64, f64)>,
    wall_stage_share: [f64; 3],
    gates_ok: Vec<(&'static str, bool)>,
    gate_text: String,
    ns_per_leaf: f64,
}

fn run_cell(n: u32, d: usize, n_no: usize, n_planted: usize, seed: u64, full: bool) -> CellOut {
    let cx = Ctx::new(n, d);
    let mut rng = Rng(seed ^ (n as u64).wrapping_mul(0xc2b2_ae3d_27d4_eb4f) ^ (d as u64).wrapping_mul(0x1656_67b1_9e37_79f9));
    let mut ws = Scratch::default();
    let mut out: Vec<[usize; 3]> = Vec::new();
    eprintln!(
        "cell n={n} l={d}: irr=0x{:x} |F|={} pairs={} singles={} vperp={}",
        cx.gf.irr,
        cx.points.len(),
        cx.pairs.len(),
        cx.singles.len(),
        cx.vperp_dim
    );

    // Targets: class 0 = no, 1 = natural yes, 2 = planted.
    let mut targets: Vec<(Pt, u8, [usize; 3])> = Vec::new();
    let mut base_sets: Vec<Vec<[usize; 3]>> = Vec::new();
    let (mut no, mut drawn) = (0usize, 0usize);
    while no < n_no {
        let r = cx.double(cx.random_point(&mut rng));
        if r.inf {
            continue;
        }
        drawn += 1;
        let mut c = Cnt::default();
        out.clear();
        cx.scan::<true>(r, 4, Arm::Base, &mut c, &mut ws, &mut out);
        assert!(!c.truncated);
        let set = canon(out.clone());
        if set.is_empty() {
            no += 1;
            targets.push((r, 0, [0; 3]));
        } else {
            targets.push((r, 1, [0; 3]));
        }
        base_sets.push(set);
    }
    let natural_yes = drawn - n_no;
    let np = cx.points.len() as u64;
    let mut planted_done = 0;
    while planted_done < n_planted {
        let mut t = [rng.below(np) as usize, rng.below(np) as usize, rng.below(np) as usize];
        let r = cx.add(cx.add(cx.points[t[0]], cx.points[t[1]]), cx.points[t[2]]);
        if r.inf || r.x == 0 || t.iter().any(|&i| cx.points[i].x == 0) {
            continue;
        }
        t.sort_unstable();
        let mut c = Cnt::default();
        out.clear();
        cx.scan::<true>(r, 4, Arm::Base, &mut c, &mut ws, &mut out);
        targets.push((r, 2, t));
        base_sets.push(canon(out.clone()));
        planted_done += 1;
    }

    // Counting pass and correctness gates.
    let table = if n <= 35 { Some(PairTable::new(&cx)) } else { None };
    let mut base_cnt = Cnt::default();
    let mut arm_cnt = [Cnt::default(); 3]; // B1, B2, C over the no class
    let (mut set_mismatch, mut truth_mismatch, mut truth_checked) = ([0usize; 3], 0usize, 0usize);
    let (mut planted_found, mut leaf_mismatch_b1, mut leaf_mismatch_b2, mut leaves_checked) = (0usize, 0u64, 0u64, 0u64);
    let (mut t_multiset_mismatch, mut total_decomps) = (0usize, 0usize);
    for (ti, &(r, class, planted)) in targets.iter().enumerate() {
        let mut c = Cnt::default();
        out.clear();
        ws.rec = Some(Vec::new());
        cx.scan::<true>(r, 4, Arm::Base, &mut c, &mut ws, &mut out);
        let mut rec_base = ws.rec.take().unwrap();
        assert_eq!(canon(out.clone()), base_sets[ti], "baseline is not deterministic");
        total_decomps += base_sets[ti].len();
        if class == 0 {
            base_cnt.add(&c);
        }
        if class == 2 && base_sets[ti].contains(&planted) {
            planted_found += 1;
        }
        if let Some(tb) = &table {
            truth_checked += 1;
            if tb.truth(&cx, r) != base_sets[ti] {
                truth_mismatch += 1;
            }
        }
        if !full {
            continue;
        }
        for (ai, arm) in [Arm::B1, Arm::B2].into_iter().enumerate() {
            let mut c = Cnt::default();
            out.clear();
            cx.scan::<true>(r, 4, arm, &mut c, &mut ws, &mut out);
            if canon(out.clone()) != base_sets[ti] {
                set_mismatch[ai] += 1;
            }
            if class == 0 {
                arm_cnt[ai].add(&c);
            }
        }
        let mut c = Cnt::default();
        out.clear();
        ws.rec = Some(Vec::new());
        cx.scan_c::<true>(r, 4, &mut c, &mut ws, &mut out);
        let mut rec_c = ws.rec.take().unwrap();
        if canon(out.clone()) != base_sets[ti] {
            set_mismatch[2] += 1;
        }
        if class == 0 {
            arm_cnt[2].add(&c);
        }
        rec_base.sort_unstable();
        rec_c.sort_unstable();
        if rec_base != rec_c {
            t_multiset_mismatch += 1;
        }
        // Per-leaf verdicts: baseline "consistent" against both B tests.
        for &(u, ui) in &rec_base {
            let mut c0 = Cnt::default();
            let mut sink = 0u64;
            cx.leaf::<true, _>(u, ui, 3, false, &mut c0, &mut ws.cols, &mut ws.kernel, &mut sink, |_, _, _| {});
            let base_ok = c0.consistent == 1;
            let b2_ok = {
                // `leaf` left the system's columns in ws.cols; rebuild its rhs.
                let gf = &cx.gf;
                let cc = gf.mul(cx.b, gf.sqr(ui));
                let hc = cx.half_trace.apply(cc);
                let tr_c = gf.sqr(hc) ^ hc ^ cc;
                let shift = 2 * d - 1;
                let rhs = (gf.mul(u, hc) >> shift) | (tr_c << (n as usize - shift));
                in_span::<false>(&ws.cols, rhs, &mut c0, ST_E)
            };
            let b1_ok = cx.dual_test::<false>(ui, &mut c0, &mut ws.rows);
            leaves_checked += 1;
            leaf_mismatch_b1 += u64::from(b1_ok != base_ok);
            leaf_mismatch_b2 += u64::from(b2_ok != base_ok);
        }
    }

    // Timed pass.
    let arms: Vec<TArm> = if full {
        vec![TArm::A1, TArm::A2, TArm::B1, TArm::B2, TArm::C, TArm::L1a, TArm::L1b, TArm::L2, TArm::L3, TArm::C1]
    } else {
        vec![TArm::A1, TArm::A2, TArm::L1a, TArm::L1b, TArm::L2, TArm::L3]
    };
    let na = arms.len();
    let mut best = vec![vec![u64::MAX; targets.len()]; na];
    let mut timed_mismatch = 0usize;
    for (ai, &arm) in arms.iter().enumerate() {
        run_timed(&cx, arm, targets[ai % targets.len()].0, &mut ws, &mut out); // warm-up
    }
    for _rep in 0..3 {
        for (ti, &(r, _, _)) in targets.iter().enumerate() {
            for s in 0..na {
                let ai = (s + ti) % na;
                let (ns, nw, _) = run_timed(&cx, arms[ai], r, &mut ws, &mut out);
                best[ai][ti] = best[ai][ti].min(ns);
                if matches!(arms[ai], TArm::A1 | TArm::A2 | TArm::B1 | TArm::B2 | TArm::C) {
                    if nw == 0 && !base_sets[ti].is_empty() || canon(out.clone()) != base_sets[ti] {
                        timed_mismatch += 1;
                    }
                }
            }
        }
    }
    let no_idx: Vec<usize> = (0..targets.len()).filter(|&i| targets[i].1 == 0).collect();
    let col = |a: TArm| -> Vec<u64> {
        let ai = arms.iter().position(|&x| x == a).unwrap();
        no_idx.iter().map(|&i| best[ai][i]).collect()
    };
    let mut brng = Rng(seed ^ 0xb007 ^ n as u64);
    let a1 = col(TArm::A1);
    let aa = ratio_ci(&a1, &col(TArm::A2), &mut brng);
    let l1a = col(TArm::L1a);
    let tstage_aa = ratio_ci(&l1a, &col(TArm::L1b), &mut brng);
    let mut wall: Vec<(&'static str, (f64, f64, f64))> = Vec::new();
    let mut tstage_c = None;
    if full {
        for a in [TArm::B1, TArm::B2, TArm::C] {
            wall.push((a.name(), ratio_ci(&a1, &col(a), &mut brng)));
        }
        tstage_c = Some(ratio_ci(&l1a, &col(TArm::C1), &mut brng));
    }
    let sum = |v: &[u64]| v.iter().map(|&x| x as f64).sum::<f64>();
    let (s_a, s_l1, s_l2, s_l3) = (sum(&a1), sum(&l1a), sum(&col(TArm::L2)), sum(&col(TArm::L3)));
    let wall_stage_share = [s_l1 / s_a, (s_l3 - s_l1) / s_a, (s_a - s_l3) / s_a];
    let ns_per_leaf = s_a / base_cnt.trials as f64;

    let mut gates_ok = vec![
        ("truth_n_le_35", truth_mismatch == 0),
        ("planted_found", planted_found == n_planted),
        ("timed_sets_equal", timed_mismatch == 0),
    ];
    if full {
        gates_ok.push(("B1_sets", set_mismatch[0] == 0));
        gates_ok.push(("B2_sets", set_mismatch[1] == 0));
        gates_ok.push(("C_sets", set_mismatch[2] == 0));
        gates_ok.push(("B1_leaf_verdicts", leaf_mismatch_b1 == 0));
        gates_ok.push(("B2_leaf_verdicts", leaf_mismatch_b2 == 0));
        gates_ok.push(("C_t_multiset", t_multiset_mismatch == 0));
    }
    let gate_text = format!(
        "targets {} (no {}, natural yes {}, planted {}); decompositions in baseline sets {}; exhaustive truth checked on {} targets, mismatches {}; planted found {}/{}; set mismatches B1/B2/C {}/{}/{}; leaves checked {} with verdict mismatches B1 {} B2 {}; t-multiset mismatches {}; timed-build set mismatches {}",
        targets.len(), n_no, natural_yes, n_planted, total_decomps, truth_checked, truth_mismatch, planted_found, n_planted,
        set_mismatch[0], set_mismatch[1], set_mismatch[2], leaves_checked, leaf_mismatch_b1, leaf_mismatch_b2, t_multiset_mismatch, timed_mismatch
    );
    eprintln!("  gates: {gate_text}");

    // JSON for the cell.
    let mut j = String::new();
    let cnt_json = |c: &Cnt| -> String {
        let mut s = String::from("{");
        for st in 0..4 {
            write!(s, "\"{}\":{{", STAGES[st]).unwrap();
            for k in 0..5 {
                write!(s, "\"{}\":{}{}", CATS[k], jf(c.c[st][k] as f64 / n_no as f64), if k < 4 { "," } else { "" }).unwrap();
            }
            s.push_str("},");
        }
        write!(
            s,
            "\"leaves\":{},\"consistent\":{},\"splits\":{},\"verify_adds\":{},\"pretest_pass\":{}",
            jf(c.trials as f64 / n_no as f64),
            jf(c.consistent as f64 / n_no as f64),
            jf(c.splits as f64 / n_no as f64),
            jf(c.verify_adds as f64 / n_no as f64),
            jf(c.pretest_pass as f64 / n_no as f64)
        )
        .unwrap();
        for (pn, w) in prices(n) {
            write!(s, ",\"priced_{pn}\":{}", jf(c.priced(&w) / n_no as f64)).unwrap();
            write!(s, ",\"priced_{pn}_t_stage\":{}", jf(c.priced_stage(ST_T, &w) / n_no as f64)).unwrap();
        }
        s.push('}');
        s
    };
    let ci_json = |c: (f64, f64, f64)| format!("{{\"ratio\":{},\"lo95\":{},\"hi95\":{}}}", jf(c.0), jf(c.1), jf(c.2));
    write!(
        j,
        "{{\"n\":{n},\"l\":{d},\"irr\":\"0x{:x}\",\"a\":1,\"b\":1,\"mode\":\"{}\",\"base_points\":{},\"pairs\":{},\"singles\":{},\"vperp_dim\":{},\"setup_ns\":{{\"base\":{},\"B1_tables\":{},\"C_tables\":{}}},",
        cx.gf.irr,
        if full { "full" } else { "breakdown" },
        cx.points.len(),
        cx.pairs.len(),
        cx.singles.len(),
        cx.vperp_dim,
        cx.setup_base_ns,
        cx.setup_b1_ns,
        cx.setup_c_ns
    )
    .unwrap();
    write!(j, "\"targets\":{{\"no\":{n_no},\"natural_yes\":{natural_yes},\"planted\":{n_planted}}},\"gates\":{{").unwrap();
    for (i, (g, ok)) in gates_ok.iter().enumerate() {
        write!(j, "{}\"{g}\":{ok}", if i > 0 { "," } else { "" }).unwrap();
    }
    write!(j, "}},\"gate_detail\":\"{gate_text}\",").unwrap();
    write!(j, "\"counted_per_target_no_class\":{{\"base\":{}", cnt_json(&base_cnt)).unwrap();
    let mut arms_out: Vec<(&'static str, Cnt)> = Vec::new();
    if full {
        for (i, nm) in ["B1", "B2", "C"].into_iter().enumerate() {
            write!(j, ",\"{nm}\":{}", cnt_json(&arm_cnt[i])).unwrap();
            arms_out.push((nm, arm_cnt[i]));
        }
    }
    j.push_str("},\"wall_ns_per_target_no_class\":{");
    for (i, &a) in arms.iter().enumerate() {
        write!(j, "{}\"{}\":{}", if i > 0 { "," } else { "" }, a.name(), jf(sum(&col(a)) / n_no as f64)).unwrap();
    }
    write!(j, "}},\"wall_ratio_A1_over_arm\":{{\"A2_AA\":{}", ci_json(aa)).unwrap();
    for (nm, c) in &wall {
        write!(j, ",\"{nm}\":{}", ci_json(*c)).unwrap();
    }
    write!(j, "}},\"t_stage_wall_ratio_L1a_over\":{{\"L1b_AA\":{}", ci_json(tstage_aa)).unwrap();
    if let Some(c) = tstage_c {
        write!(j, ",\"C1\":{}", ci_json(c)).unwrap();
    }
    write!(
        j,
        "}},\"baseline_wall_stage_share\":{{\"t_stage\":{},\"ht_elimination\":{},\"split_and_membership\":{},\"system_build_part\":{}}},\"baseline_ns_per_leaf\":{}}}",
        jf(wall_stage_share[0]),
        jf(wall_stage_share[1]),
        jf(wall_stage_share[2]),
        jf((s_l2 - s_l1) / s_a),
        jf(ns_per_leaf)
    )
    .unwrap();

    CellOut {
        n,
        json: j,
        base: base_cnt,
        arms: arms_out,
        n_no,
        wall,
        aa,
        tstage_aa,
        tstage_c,
        wall_stage_share,
        gates_ok,
        gate_text,
        ns_per_leaf,
    }
}

fn main() {
    assert!(std::is_x86_feature_detected!("pclmulqdq"), "this binary needs pclmulqdq");
    let args: Vec<String> = std::env::args().collect();
    let flag = |name: &str| args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned());
    let mode = flag("--mode").unwrap_or_else(|| "full".into());
    let full = mode == "full";
    let cells: Vec<(u32, usize)> = flag("--cells")
        .unwrap_or_else(|| "23:7,29:9,35:11,41:13,47:15".into())
        .split(',')
        .map(|s| {
            let p: Vec<&str> = s.split(':').collect();
            (p[0].parse().unwrap(), p[1].parse().unwrap())
        })
        .collect();
    let n_no: usize = flag("--no").map(|v| v.parse().unwrap()).unwrap_or(128);
    let n_planted: usize = flag("--planted").map(|v| v.parse().unwrap()).unwrap_or(16);
    let seed: u64 = flag("--seed").map(|v| v.parse().unwrap()).unwrap_or(20261116);
    let outdir = flag("--out").unwrap_or_else(|| ".".into());
    let stem = flag("--stem").unwrap_or_else(|| if full { "results".into() } else { "breakdown".into() });
    std::fs::create_dir_all(&outdir).unwrap();

    let t0 = Instant::now();
    let outs: Vec<CellOut> = cells.iter().map(|&(n, d)| run_cell(n, d, n_no, n_planted, seed, full)).collect();

    // ── tables ──
    let mut md = String::new();
    writeln!(md, "# Scan-floor leaf pilot: {mode} run (generated by scan_leaf.rs; every line [pcc:unverified])\n").unwrap();
    writeln!(md, "Baseline = standalone port of lin21-ht (the repository arm did not build). Seed {seed}, {n_no} non-decomposable + {n_planted} planted targets per cell, single thread, shared container.\n").unwrap();
    writeln!(md, "## 1. Baseline stage breakdown (per leaf, 'no' class)\n").unwrap();
    writeln!(md, "| n | l | leaves/target | ns/leaf | hw ops/leaf | t stage | ht elimination | root split | membership | wall: t stage | wall: ht elimination | wall: split+membership |").unwrap();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    for (o, &(_, d)) in outs.iter().zip(&cells) {
        let w = prices(o.n)[0].1;
        let tot = o.base.priced(&w);
        let sh = |s: usize| 100.0 * o.base.priced_stage(s, &w) / tot;
        writeln!(
            md,
            "| {} | {} | {:.0} | {:.1} | {:.1} | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% |",
            o.n,
            d,
            o.base.trials as f64 / o.n_no as f64,
            o.ns_per_leaf,
            tot / o.base.trials as f64,
            sh(0),
            sh(1),
            sh(2),
            sh(3),
            100.0 * o.wall_stage_share[0],
            100.0 * o.wall_stage_share[1],
            100.0 * o.wall_stage_share[2]
        )
        .unwrap();
    }
    writeln!(md, "\nCounted operations per leaf of the baseline by category (all stages):\n").unwrap();
    writeln!(md, "| n | mul | sqr | inv | lookup | wop | consistent leaves | splits |").unwrap();
    writeln!(md, "|---|---|---|---|---|---|---|---|").unwrap();
    for o in &outs {
        let l = o.base.trials as f64;
        let cat = |k: usize| (0..4).map(|s| o.base.c[s][k]).sum::<u64>() as f64 / l;
        writeln!(
            md,
            "| {} | {:.2} | {:.2} | {:.4} | {:.2} | {:.1} | {:.4} | {:.4} |",
            o.n,
            cat(0),
            cat(1),
            cat(2),
            cat(3),
            cat(4),
            o.base.consistent as f64 / l,
            o.base.splits as f64 / l
        )
        .unwrap();
    }
    writeln!(md, "\nAmdahl ceiling at each n (hw price): removing a stage entirely gives at most 1/(1 - share).\n").unwrap();

    let mut decisions = String::new();
    if full {
        writeln!(md, "## 2. Counted operations per target, baseline / arm ('no' class)\n").unwrap();
        writeln!(md, "| n | arm | mul | sqr | inv | lookup | wop | hw total | base/arm hw | base/arm clmul-min | base/arm bit-serial | pretest pass rate |").unwrap();
        writeln!(md, "|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
        for o in &outs {
            let pr = prices(o.n);
            let mut rows: Vec<(&str, &Cnt)> = vec![("base", &o.base)];
            for (nm, c) in &o.arms {
                rows.push((nm, c));
            }
            for (nm, c) in rows {
                let t = o.n_no as f64;
                let cat = |k: usize| (0..4).map(|s| c.c[s][k]).sum::<u64>() as f64 / t;
                writeln!(
                    md,
                    "| {} | {} | {:.0} | {:.0} | {:.1} | {:.0} | {:.0} | {:.0} | {:.3} | {:.3} | {:.3} | {:.4} |",
                    o.n,
                    nm,
                    cat(0),
                    cat(1),
                    cat(2),
                    cat(3),
                    cat(4),
                    c.priced(&pr[0].1) / t,
                    o.base.priced(&pr[0].1) / c.priced(&pr[0].1),
                    o.base.priced(&pr[1].1) / c.priced(&pr[1].1),
                    o.base.priced(&pr[2].1) / c.priced(&pr[2].1),
                    c.pretest_pass as f64 / c.trials as f64
                )
                .unwrap();
            }
        }
        writeln!(md, "\n## 3. Wall time, baseline A1 / arm (sum over the 'no' class, min of 3 repetitions, paired bootstrap 95%)\n").unwrap();
        writeln!(md, "| n | A/A (A1/A2) | B1 | B2 | C (whole leaf) |").unwrap();
        writeln!(md, "|---|---|---|---|---|").unwrap();
        let f = |c: (f64, f64, f64)| format!("{:.3} [{:.3}, {:.3}]", c.0, c.1, c.2);
        for o in &outs {
            writeln!(md, "| {} | {} | {} | {} | {} |", o.n, f(o.aa), f(o.wall[0].1), f(o.wall[1].1), f(o.wall[2].1)).unwrap();
        }
        writeln!(md, "\n## 4. C on the t stage\n").unwrap();
        writeln!(md, "| n | t-stage hw ops/leaf base | t-stage hw ops/leaf C | counted base/C (hw) | (clmul-min) | (bit-serial) | wall A/A (L1a/L1b) | wall L1a/C1 | whole-leaf counted base/C (hw) |").unwrap();
        writeln!(md, "|---|---|---|---|---|---|---|---|---|").unwrap();
        for o in &outs {
            let pr = prices(o.n);
            let c = &o.arms[2].1;
            writeln!(
                md,
                "| {} | {:.1} | {:.1} | {:.3} | {:.3} | {:.3} | {} | {} | {:.3} |",
                o.n,
                o.base.priced_stage(ST_T, &pr[0].1) / o.base.trials as f64,
                c.priced_stage(ST_T, &pr[0].1) / c.trials as f64,
                o.base.priced_stage(ST_T, &pr[0].1) / c.priced_stage(ST_T, &pr[0].1),
                o.base.priced_stage(ST_T, &pr[1].1) / c.priced_stage(ST_T, &pr[1].1),
                o.base.priced_stage(ST_T, &pr[2].1) / c.priced_stage(ST_T, &pr[2].1),
                f(o.tstage_aa),
                f(o.tstage_c.unwrap()),
                o.base.priced(&pr[0].1) / c.priced(&pr[0].1)
            )
            .unwrap();
        }

        // ── decisions per PROTOCOL.md ──
        let last3: Vec<&CellOut> = outs.iter().rev().take(3).collect();
        let largest = outs.last().unwrap();
        let hw = |o: &CellOut| prices(o.n)[0].1;
        let mut decide = |label: &str, arm: usize, thr: f64, tstage: bool| {
            let ratio = |o: &CellOut| {
                let w = hw(o);
                if tstage {
                    o.base.priced_stage(ST_T, &w) / o.arms[arm].1.priced_stage(ST_T, &w)
                } else {
                    o.base.priced(&w) / o.arms[arm].1.priced(&w)
                }
            };
            let gate_names: &[&str] = match arm {
                0 => &["B1_sets", "B1_leaf_verdicts"],
                1 => &["B2_sets", "B2_leaf_verdicts"],
                _ => &["C_sets", "C_t_multiset"],
            };
            let gates = outs.iter().all(|o| {
                o.gates_ok.iter().all(|(g, ok)| *ok || !(gate_names.contains(g) || matches!(*g, "truth_n_le_35" | "planted_found" | "timed_sets_equal")))
            });
            let counted_ok = last3.iter().all(|o| ratio(o) >= thr);
            let wall_ok = last3.iter().all(|o| {
                if tstage {
                    o.tstage_c.unwrap().1 > o.tstage_aa.2
                } else {
                    o.wall[arm].1 .1 > o.aa.2
                }
            });
            let verdict = if !gates {
                "VOID (a correctness gate failed)"
            } else if counted_ok && wall_ok {
                "holds"
            } else if ratio(largest) < 1.1 {
                "fails"
            } else {
                "inconclusive"
            };
            let rs: Vec<String> = last3.iter().rev().map(|o| format!("n={}: {:.3}", o.n, ratio(o))).collect();
            writeln!(
                decisions,
                "- **{label}**: {verdict}. hw-priced counted ratio baseline/arm at the three largest n: {} (threshold {thr}; fail line 1.1 at n={}); counted threshold met at all three: {counted_ok}; wall outside the A/A band at all three: {wall_ok}; gates pass: {gates}. [pcc:unverified]",
                rs.join(", "),
                largest.n
            )
            .unwrap();
        };
        decide("B (B1, dual leaf as proposed; primary)", 0, 1.5, false);
        decide("B2 (decision-only elimination on the baseline's system; secondary)", 1, 1.5, false);
        decide("C (y-free t; t stage)", 2, 1.3, true);
        writeln!(md, "\n## 5. Decisions under the frozen rules (PROTOCOL.md)\n\n{decisions}").unwrap();
    }
    writeln!(md, "\n## Gates\n").unwrap();
    for o in &outs {
        writeln!(md, "- n={}: {} — {}", o.n, if o.gates_ok.iter().all(|g| g.1) { "all pass" } else { "FAILURE" }, o.gate_text).unwrap();
    }
    writeln!(md, "\nRun time {:.1} s.", t0.elapsed().as_secs_f64()).unwrap();

    let mut js = String::new();
    write!(
        js,
        "{{\"schema\":\"scan_floor_leaf.pilot/v1\",\"standing\":\"pcc:unverified\",\"mode\":\"{mode}\",\"baseline\":\"standalone port of lin21-ht (repository arm did not build)\",\"seed\":{seed},\"no_targets\":{n_no},\"planted_targets\":{n_planted},\"prices\":{{\"hw\":\"mul 8, sqr 8, inv 8(n+8), lookup 1, wop 1\",\"clmul_min\":\"mul 3, sqr 3, inv 3(n+8), lookup 1, wop 1\",\"bit_serial\":\"mul 2n, sqr n, inv 2n(n+8), lookup 1, wop 1\"}},\"decisions\":\"{}\",\"cells\":[",
        decisions.replace('\n', " ").replace('"', "'")
    )
    .unwrap();
    for (i, o) in outs.iter().enumerate() {
        if i > 0 {
            js.push(',');
        }
        js.push_str(&o.json);
    }
    js.push_str("]}\n");
    let stem_md = if full { "tables".to_string() } else { format!("{stem}_tables") };
    let (jp, mp) = (format!("{outdir}/{stem}.json"), format!("{outdir}/{}.md", if stem == "results" { stem_md } else { format!("{stem}_tables") }));
    std::fs::write(&jp, js).unwrap();
    std::fs::write(&mp, &md).unwrap();
    print!("{md}");
    eprintln!("wrote {jp} and {mp}");
}
