//! Lift-dense factor-base subspaces (study `lift_dense_factor_base_20261027`).
//!
//! E_{a,b} : y^2 + xy = x^3 + a x^2 + b over F_q, q = 2^n. A nonzero x lifts
//! to E(F_q) iff Tr(x + a + b/x^2) = 0; otherwise it lifts to the twist.
//! For a subspace V the mean number of m-term decompositions of a random
//! target is proportional to |V ∩ L|^m (L = lifting abscissae), so the gain
//! of V over a random subspace of the same dimension is (2 rho)^m, rho the
//! lift density of V. This harness measures how large rho can be:
//!
//! - `kmax`: the largest all-lift subspace (V \ {0} ⊂ L) found, exactly for
//!   small n and under a fixed work budget otherwise;
//! - `rho*`: the best lift density found at dimensions above `kmax`;
//! - arms: curves, density-matched random sets (null), planted random sets
//!   (positive control for the search), and the explicit subfield
//!   construction at composite n (structural positive control);
//! - `yield`: relations per target for m = 2, counted by point arithmetic,
//!   for a dense V against random subspaces of the same dimension.
//!
//! Std only. `cargo run --release -- full <outdir>`.

use std::fmt::Write as _;
use std::io::Write as _;
use std::time::Instant;

const SEED: u64 = 0x11F7_2026_1027_C;

// ---------------------------------------------------------------- F_q ----

struct Fq {
    n: u32,
    q1: usize,
    exp: Vec<u32>,
    log: Vec<u32>,
    tr_mask: u32,
    half_trace: Vec<u32>,
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
    /// Builds log/exp tables from a generator of the full multiplicative
    /// group. Finding an element of order 2^n - 1 proves `poly` irreducible
    /// (a reducible quotient ring has fewer than 2^n - 1 units).
    fn new(n: u32, poly: u32) -> Fq {
        assert!((poly >> n) == 1 && n <= 24);
        let q1 = (1usize << n) - 1;
        let mut exp = vec![0u32; 2 * q1];
        let mut log = vec![0u32; q1 + 1];
        let mut found = false;
        'search: for g in 2u32..64 {
            let mut e = 1u32;
            for i in 0..q1 {
                if i > 0 && e == 1 {
                    continue 'search;
                }
                exp[i] = e;
                log[e as usize] = i as u32;
                e = clmul_reduce(e, g, n, poly);
            }
            if e != 1 {
                continue;
            }
            found = true;
            break;
        }
        assert!(
            found,
            "no generator: poly {poly:#x} not irreducible for n = {n}"
        );
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
    fn pow2k(&self, a: u32, k: u32) -> u32 {
        // a^(2^k)
        if a == 0 {
            return 0;
        }
        let l = self.log[a as usize] as u64;
        self.exp[((l << k) % self.q1 as u64) as usize]
    }
    fn sqrt(&self, a: u32) -> u32 {
        self.pow2k(a, self.n - 1)
    }
    #[inline]
    fn tr(&self, a: u32) -> u32 {
        (a & self.tr_mask).count_ones() & 1
    }
    /// Half-trace (odd n only): H(w)^2 + H(w) = w + Tr(w).
    fn ht(&self, a: u32) -> u32 {
        assert!(self.n % 2 == 1);
        let mut r = 0;
        for i in 0..self.n {
            if (a >> i) & 1 == 1 {
                r ^= self.half_trace[i as usize];
            }
        }
        r
    }
}

/// (n, irreducible polynomial), checked by `Fq::new`.
const FIELDS: &[(u32, u32)] = &[
    (9, (1 << 9) | (1 << 4) | 1),
    (11, (1 << 11) | (1 << 2) | 1),
    (12, (1 << 12) | (1 << 3) | 1),
    (13, (1 << 13) | (1 << 4) | (1 << 3) | (1 << 1) | 1),
    (15, (1 << 15) | (1 << 1) | 1),
    (16, (1 << 16) | (1 << 5) | (1 << 3) | (1 << 1) | 1),
    (17, (1 << 17) | (1 << 3) | 1),
    (18, (1 << 18) | (1 << 3) | 1),
    (19, (1 << 19) | (1 << 5) | (1 << 2) | (1 << 1) | 1),
    (20, (1 << 20) | (1 << 3) | 1),
    (21, (1 << 21) | (1 << 2) | 1),
    (22, (1 << 22) | (1 << 1) | 1),
    (23, (1 << 23) | (1 << 5) | 1),
];

// ---------------------------------------------------------------- rng ----

#[derive(Clone)]
struct Rng(u64);
impl Rng {
    fn new(tag: &str, n: u32) -> Rng {
        // Derived seed: SEED, n and the instance label (FNV-1a), then mixed.
        let mut h: u64 = 0xcbf29ce484222325 ^ SEED ^ ((n as u64) << 40);
        for b in tag.bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        let mut r = Rng(h);
        r.next();
        r
    }
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

// ------------------------------------------------------------ bitsets ----

#[derive(Clone)]
struct Bits(Vec<u64>);
impl Bits {
    fn new(n: u32) -> Bits {
        Bits(vec![0; ((1usize << n) + 63) / 64])
    }
    #[inline]
    fn get(&self, x: u32) -> bool {
        (self.0[(x >> 6) as usize] >> (x & 63)) & 1 == 1
    }
    #[inline]
    fn set(&mut self, x: u32) {
        self.0[(x >> 6) as usize] |= 1 << (x & 63);
    }
    #[inline]
    fn clear(&mut self, x: u32) {
        self.0[(x >> 6) as usize] &= !(1 << (x & 63));
    }
}

/// Lift set of E_{a,b}: bit x set iff x != 0 and Tr(x + a + b/x^2) = 0.
fn lift_set(f: &Fq, a: u32, b: u32) -> Bits {
    let mut s = Bits::new(f.n);
    for x in 1..=(f.q1 as u32) {
        let w = x ^ a ^ f.mul(b, f.inv(f.sq(x)));
        if f.tr(w) == 0 {
            s.set(x);
        }
    }
    s
}

fn members(n: u32, s: &Bits) -> Vec<u32> {
    (1..(1u32 << n)).filter(|&x| s.get(x)).collect()
}

// -------------------------------------------------- all-lift search ----

struct Search<'a> {
    _s: &'a Bits,
    levels: Vec<Bits>,
    ops: u64,
    budget: u64,
    best: usize,
    best_basis: Vec<u32>,
    cur: Vec<u32>,
    capped: bool,
    /// count mode: no bound pruning; counts[k] = number of k-dim subspaces
    /// (one reduced-echelon basis each) inside s, below the prefix
    count_mode: bool,
    counts: Vec<u64>,
}

impl<'a> Search<'a> {
    fn new(n: u32, s: &'a Bits, budget: u64) -> Search<'a> {
        Search {
            _s: s,
            levels: (0..24).map(|_| Bits::new(n)).collect(),
            ops: 0,
            budget,
            best: 0,
            best_basis: vec![],
            cur: vec![],
            capped: false,
            count_mode: false,
            counts: vec![0; 25],
        }
    }

    fn record(&mut self) {
        if self.cur.len() > self.best {
            self.best = self.cur.len();
            self.best_basis = self.cur.clone();
        }
    }

    /// C_{U+v} from C_U (listed in `cand`, marked in level `lv`).
    fn child(&mut self, cand: &[u32], lv: usize, v: u32) -> Vec<u32> {
        self.ops += cand.len() as u64;
        let mut out = Vec::new();
        for &w in cand {
            if self.levels[lv].get(w ^ v) {
                out.push(w);
            }
        }
        for &w in &out {
            self.levels[lv + 1].set(w);
        }
        out
    }

    fn release(&mut self, c: &[u32], lv: usize) {
        for &w in c {
            self.levels[lv].clear(w);
        }
    }

    /// Exhaustive DFS below the current prefix. Children are restricted to
    /// reduced-echelon tails (v > last, v zero on the tail's pivot bits),
    /// which reaches every subspace containing the prefix.
    fn dfs(&mut self, cand: &[u32], lv: usize, last: u32, pivmask: u32) {
        self.record();
        self.counts[self.cur.len()] += 1;
        if self.ops > self.budget {
            self.capped = true;
            return;
        }
        // Bound: extending U (dim k) by e dimensions puts 2^k (2^e - 1)
        // elements into C_U, so e <= log2(|C| / 2^k + 1).
        let k = self.cur.len();
        let mut room = 0usize;
        while ((1u128 << k) * ((1u128 << (room + 1)) - 1)) <= cand.len() as u128 {
            room += 1;
        }
        if !self.count_mode && k + room <= self.best {
            return;
        }
        for i in 0..cand.len() {
            let v = cand[i];
            if v <= last || v & pivmask != 0 {
                continue;
            }
            let c = self.child(cand, lv, v);
            self.cur.push(v);
            let top = 1u32 << (31 - v.leading_zeros());
            self.dfs(&c, lv + 1, v, pivmask | top);
            self.cur.pop();
            self.release(&c, lv + 1);
            if self.capped {
                return;
            }
        }
    }

    /// Random prefix while |C| > thresh, then the exhaustive tail.
    fn restart(&mut self, all: &[u32], thresh: usize, rng: &mut Rng) {
        let mut stack: Vec<Vec<u32>> = vec![];
        let mut cand: Vec<u32> = all.to_vec();
        for &w in &cand {
            self.levels[0].set(w);
        }
        let mut lv = 0;
        while cand.len() > thresh {
            let v = cand[rng.below(cand.len() as u64) as usize];
            let c = self.child(&cand, lv, v);
            self.cur.push(v);
            stack.push(std::mem::replace(&mut cand, c));
            lv += 1;
            self.record();
        }
        self.dfs(&cand.clone(), lv, 0, 0);
        // unwind: levels[i] holds C at depth i
        self.release(&cand, lv);
        while let Some(prev) = stack.pop() {
            lv -= 1;
            self.release(&prev, lv);
            self.cur.pop();
        }
        debug_assert!(self.cur.is_empty() && lv == 0);
    }
}

/// Every element of span(basis) (nonzero) is in s.
fn all_lift(s: &Bits, basis: &[u32]) -> bool {
    let k = basis.len();
    let mut x = 0u32;
    for g in 1u64..(1u64 << k) {
        x ^= basis[g.trailing_zeros() as usize];
        if !s.get(x) {
            return false;
        }
    }
    true
}

fn rank(vs: &[u32]) -> usize {
    let mut piv: Vec<u32> = vec![];
    for &v in vs {
        let mut x = v;
        for &p in &piv {
            x = x.min(x ^ p);
        }
        if x != 0 {
            piv.push(x);
            piv.sort_unstable_by(|a, b| b.cmp(a));
        }
    }
    piv.len()
}

/// Count of nonzero elements of span(basis) in s (Gray code).
fn count_in(s: &Bits, basis: &[u32]) -> u64 {
    let k = basis.len();
    let mut x = 0u32;
    let mut c = 0u64;
    for g in 1u64..(1u64 << k) {
        x ^= basis[g.trailing_zeros() as usize];
        c += s.get(x) as u64;
    }
    c
}

fn random_basis(n: u32, l: usize, rng: &mut Rng) -> Vec<u32> {
    loop {
        let b: Vec<u32> = (0..l)
            .map(|_| 1 + rng.below((1u64 << n) - 1) as u32)
            .collect();
        if rank(&b) == l {
            return b;
        }
    }
}

/// Hill climb on bases of dimension l, starting from `start` (an all-lift
/// basis) extended by random vectors. Moves replace one basis vector by a
/// random vector; ties are accepted.
fn hill_climb(n: u32, s: &Bits, start: &[u32], l: usize, iters: usize, rng: &mut Rng) -> u64 {
    let mut b: Vec<u32> = start.iter().copied().take(l).collect();
    while b.len() < l {
        let v = 1 + rng.below((1u64 << n) - 1) as u32;
        b.push(v);
        if rank(&b) < b.len() {
            b.pop();
        }
    }
    let mut cur = count_in(s, &b);
    for _ in 0..iters {
        let i = rng.below(l as u64) as usize;
        let old = b[i];
        b[i] = 1 + rng.below((1u64 << n) - 1) as u32;
        if rank(&b) < l {
            b[i] = old;
            continue;
        }
        let c = count_in(s, &b);
        if c >= cur {
            cur = c;
        } else {
            b[i] = old;
        }
    }
    cur
}

// ------------------------------------------------- structured control ----

fn divisors(n: u32) -> Vec<u32> {
    (2..n).filter(|d| n % d == 0).collect()
}

/// Subfield construction: V = u F_{2^d} with u = sqrt(b)/w and
/// Tr_{n/d}(w) = 0 makes Tr(sqrt(b)/x) vanish on V, so the lift condition
/// is affine on V; its zero set is a subspace when Tr(a) = 0. Returns the
/// best all-lift dimension found over `tries` choices of w (each verified
/// element by element), or 0.
fn structured(f: &Fq, s: &Bits, a: u32, b: u32, tries: usize, rng: &mut Rng) -> (u32, usize) {
    let n = f.n;
    let c = f.sqrt(b);
    let mut best = (0u32, 0usize);
    for d in divisors(n) {
        // basis of F_{2^d}: powers of a generator g^((2^n-1)/(2^d-1))
        let step = f.q1 / ((1usize << d) - 1);
        let gen = f.exp[step];
        let mut sub: Vec<u32> = vec![];
        let mut e = 1u32;
        for _ in 0..(1usize << d) - 1 {
            sub.push(e);
            if rank(&sub) < sub.len() {
                sub.pop();
            }
            if sub.len() == d as usize {
                break;
            }
            e = f.mul(e, gen);
        }
        assert_eq!(sub.len(), d as usize);
        let trnd = |z: u32| -> u32 {
            let mut t = 0;
            for j in 0..n / d {
                t ^= f.pow2k(z, d * j);
            }
            t
        };
        for _ in 0..tries {
            // w = z^(2^d) + z has Tr_{n/d}(w) = 0
            let z = 1 + rng.below(f.q1 as u64) as u32;
            let w = f.pow2k(z, d) ^ z;
            if w == 0 {
                continue;
            }
            assert_eq!(trnd(w), 0);
            let u = f.mul(c, f.inv(w));
            let vb: Vec<u32> = sub.iter().map(|&y| f.mul(u, y)).collect();
            // lift functional on V: g(x) = Tr(x) + Tr(a) (+ Tr(c/x) = 0)
            let lin: Vec<u32> = vb.iter().map(|&x| f.tr(x)).collect();
            let ta = f.tr(a);
            let cand: Vec<u32> = if ta == 1 {
                vec![]
            } else if lin.iter().all(|&t| t == 0) {
                vb.clone()
            } else {
                // kernel of the functional
                let p = lin.iter().position(|&t| t == 1).unwrap();
                (0..vb.len())
                    .filter(|&i| i != p)
                    .map(|i| if lin[i] == 1 { vb[i] ^ vb[p] } else { vb[i] })
                    .collect()
            };
            if !cand.is_empty()
                && rank(&cand) == cand.len()
                && all_lift(s, &cand)
                && cand.len() > best.1
            {
                best = (d, cand.len());
            }
        }
    }
    best
}

/// log2 of the expected number of k-dim subspaces inside a random set of
/// density 1/2: log2 [n choose k]_2 - (2^k - 1).
fn null_log2_count(n: u32, k: u32) -> f64 {
    let mut g = 0f64;
    for i in 0..k {
        let num = (2f64).powi((n - i) as i32) - 1.0;
        let den = (2f64).powi((k - i) as i32) - 1.0;
        g += (num / den).log2();
    }
    g - ((2f64).powi(k as i32) - 1.0)
}

fn null_kmax(n: u32) -> u32 {
    (1..n)
        .filter(|&k| null_log2_count(n, k) >= 0.0)
        .max()
        .unwrap_or(1)
}

// --------------------------------------------------------- curves (m=2) ----

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pt {
    x: u32,
    y: u32,
    inf: bool,
}
const O: Pt = Pt {
    x: 0,
    y: 0,
    inf: true,
};

struct Curve {
    a: u32,
    b: u32,
}
impl Curve {
    fn on(&self, f: &Fq, p: &Pt) -> bool {
        p.inf || {
            let x2 = f.sq(p.x);
            f.sq(p.y) ^ f.mul(p.x, p.y) == f.mul(x2, p.x) ^ f.mul(self.a, x2) ^ self.b
        }
    }
    fn neg(&self, p: &Pt) -> Pt {
        if p.inf {
            *p
        } else {
            Pt {
                x: p.x,
                y: p.y ^ p.x,
                inf: false,
            }
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
            if p.y == q.y ^ q.x {
                return O;
            }
            let lam = p.x ^ f.mul(p.y, f.inv(p.x));
            let x3 = f.sq(lam) ^ lam ^ self.a;
            let y3 = f.sq(p.x) ^ f.mul(lam ^ 1, x3);
            return Pt {
                x: x3,
                y: y3,
                inf: false,
            };
        }
        let lam = f.mul(p.y ^ q.y, f.inv(p.x ^ q.x));
        let x3 = f.sq(lam) ^ lam ^ p.x ^ q.x ^ self.a;
        let y3 = f.mul(lam, p.x ^ x3) ^ x3 ^ p.y;
        Pt {
            x: x3,
            y: y3,
            inf: false,
        }
    }
    /// A lift of x != 0 to E(F_q), if x lifts.
    fn lift(&self, f: &Fq, x: u32) -> Option<Pt> {
        let w = x ^ self.a ^ f.mul(self.b, f.inv(f.sq(x)));
        if f.tr(w) != 0 {
            return None;
        }
        let p = Pt {
            x,
            y: f.mul(x, f.ht(w)),
            inf: false,
        };
        assert!(self.on(f, &p));
        Some(p)
    }
}

/// Mean number of unordered pairs {P, Q} of factor-base points (x in V)
/// with P + Q = R, over `targets` uniform R, and the counting prediction
/// k(2k+1) / #E from |V ∩ L| = k.
fn yield_m2(
    f: &Fq,
    cv: &Curve,
    s: &Bits,
    basis: &[u32],
    targets: usize,
    order: u64,
    rng: &mut Rng,
) -> (f64, f64, u64) {
    let n = f.n;
    let mut inv = Bits::new(n);
    let mut fb: Vec<Pt> = vec![];
    let mut x = 0u32;
    for g in 1u64..(1u64 << basis.len()) {
        x ^= basis[g.trailing_zeros() as usize];
        inv.set(x);
        if s.get(x) {
            let p = cv.lift(f, x).unwrap();
            fb.push(p);
            fb.push(cv.neg(&p));
        }
    }
    let k = (fb.len() / 2) as u64;
    let mut total = 0u64;
    for _ in 0..targets {
        // uniform-ish target: random lifting x, random sign
        let r = loop {
            let x = 1 + rng.below(f.q1 as u64) as u32;
            if let Some(p) = cv.lift(f, x) {
                break if rng.next() & 1 == 1 { cv.neg(&p) } else { p };
            }
        };
        // ordered (P, Q) with P in FB, Q = R - P in FB; unordered = (ordered + diag) / 2
        let (mut ord, mut diag) = (0u64, 0u64);
        for p in &fb {
            let q = cv.add(f, &r, &cv.neg(p));
            if !q.inf && q.x != 0 && inv.get(q.x) {
                ord += 1;
                if q == *p {
                    diag += 1;
                }
            }
        }
        total += (ord + diag) / 2;
    }
    // every multiset {P, Q} of the 2k points sums to one R: C(2k,2) + 2k = k(2k+1)
    let pred = (k * (2 * k + 1)) as f64 / order as f64;
    (total as f64 / targets as f64, pred, k)
}

// ------------------------------------------- composite-degree lines ----

/// Solver for z^2 + z = w over F_2^n (any n): echelon rows of the F_2-linear
/// map z -> z^2 + z with tracked preimages.
struct AsSolver {
    rows: Vec<(u32, u32)>, // (image, preimage), images in echelon form (distinct top bits)
}
impl AsSolver {
    fn new(f: &Fq) -> AsSolver {
        let mut rows: Vec<(u32, u32)> = vec![];
        for i in 0..f.n {
            let z = 1u32 << i;
            let (mut im, mut pre) = (f.sq(z) ^ z, z);
            for &(r, q) in &rows {
                if im ^ r < im {
                    im ^= r;
                    pre ^= q;
                }
            }
            if im != 0 {
                rows.push((im, pre));
                rows.sort_unstable_by(|a, b| b.0.cmp(&a.0));
            }
        }
        assert_eq!(rows.len() as u32, f.n - 1, "kernel of z^2+z must be F_2");
        AsSolver { rows }
    }
    fn solve(&self, w: u32) -> Option<u32> {
        let (mut x, mut z) = (w, 0u32);
        for &(r, q) in &self.rows {
            if x ^ r < x {
                x ^= r;
                z ^= q;
            }
        }
        if x == 0 {
            Some(z)
        } else {
            None
        }
    }
}

/// A lift of x != 0 to E(F_q) for any n, if x lifts.
fn lift_any(f: &Fq, asv: &AsSolver, cv: &Curve, x: u32) -> Option<Pt> {
    let w = x ^ cv.a ^ f.mul(cv.b, f.inv(f.sq(x)));
    let z = asv.solve(w)?;
    let p = Pt {
        x,
        y: f.mul(x, z),
        inf: false,
    };
    assert!(cv.on(f, &p));
    Some(p)
}

fn subfield_basis(f: &Fq, d: u32) -> Vec<u32> {
    let gen = f.exp[f.q1 / ((1usize << d) - 1)];
    let mut sub: Vec<u32> = vec![];
    let mut e = 1u32;
    while sub.len() < d as usize {
        sub.push(e);
        if rank(&sub) < sub.len() {
            sub.pop();
        }
        e = f.mul(e, gen);
    }
    for &z in &sub {
        assert_eq!(f.pow2k(z, d), z);
    }
    sub
}

fn tr_rel(f: &Fq, z: u32, d: u32) -> u32 {
    let mut t = 0;
    for j in 0..f.n / d {
        t ^= f.pow2k(z, d * j);
    }
    t
}

/// Ordered m-tuples of factor-base points summing to R, averaged over
/// uniform targets; exact prediction |FB|^m / #E.
fn yield_ordered(
    f: &Fq,
    cv: &Curve,
    fb: &[Pt],
    inv: &Bits,
    m: usize,
    targets: &[Pt],
) -> (f64, f64) {
    if fb.is_empty() {
        return (0.0, 0.0);
    }
    let mut total = 0u64;
    let mut total2 = 0u64;
    let mut idx = vec![0usize; m - 1];
    for r in targets {
        let mut here = 0u64;
        // enumerate (P_1..P_{m-1}); P_m = R - sum must be in FB
        loop {
            let mut acc = *r;
            for &i in &idx {
                acc = cv.add(f, &acc, &cv.neg(&fb[i]));
            }
            if !acc.inf && acc.x != 0 && inv.get(acc.x) {
                here += 1;
            }
            // next index tuple
            let mut j = 0;
            while j < idx.len() {
                idx[j] += 1;
                if idx[j] < fb.len() {
                    break;
                }
                idx[j] = 0;
                j += 1;
            }
            if j == idx.len() {
                break;
            }
        }
        total += here;
        total2 += here * here;
    }
    let t = targets.len() as f64;
    (total as f64 / t, total2 as f64 / t)
}

/// For E over F_{2^N}, N = d n: the standard line F_{2^d} against a twisted
/// line u F_{2^d} with Tr_{N/d}(u) = Tr_{N/d}(sqrt(b)/u) = 0. Exact lift
/// counts on both lines (every element), and the m = n relation yield.
fn run_lines(n_field: u32, poly: u32, d: u32, quick: bool) -> Vec<String> {
    let f = Fq::new(n_field, poly);
    let asv = AsSolver::new(&f);
    let m = (n_field / d) as usize;
    let sub = subfield_basis(&f, d);
    let mut rng = Rng::new(&format!("lines_d{d}"), n_field);
    // a with Tr(a) = 0 and with Tr(a) = 1
    let pick_a = |rng: &mut Rng, t: u32| loop {
        let a = rng.below(f.q1 as u64 + 1) as u32;
        if f.tr(a) == t {
            break a;
        }
    };
    let mut rows = vec![];
    for ci in 0..8 {
        let ta = (ci / 4) as u32; // four curves with Tr(a) = 0, four with Tr(a) = 1
        let a = pick_a(&mut rng, ta);
        let b = 1 + rng.below(f.q1 as u64) as u32;
        let cv = Curve { a, b };
        let c = f.sqrt(b);
        // exact count of u with Tr_{N/d}(u) = Tr_{N/d}(sqrt(b)/u) = 0: Gray
        // code over ker Tr_{N/d}, spanned by images z^(2^d) + z.
        let mut kb: Vec<u32> = vec![];
        while kb.len() < (n_field - d) as usize {
            let z = 1 + rng.below(f.q1 as u64) as u32;
            kb.push(f.pow2k(z, d) ^ z);
            if rank(&kb) < kb.len() {
                kb.pop();
            }
        }
        let (mut u_count, mut u_found, mut uu) = (0u64, None, 0u32);
        for g in 1u64..(1u64 << kb.len()) {
            uu ^= kb[g.trailing_zeros() as usize];
            debug_assert_eq!(tr_rel(&f, uu, d), 0);
            if tr_rel(&f, f.mul(c, f.inv(uu)), d) == 0 {
                u_count += 1;
                if u_found.is_none() {
                    u_found = Some(uu);
                }
            }
        }
        let tries = u_count;
        // For n = 2, ker Tr_{N/d} = F_{2^d}, so a twisted u exists only if
        // sqrt(b) is in F_{2^d}: recorded, not an error.
        let mut lines: Vec<(&str, Vec<u32>)> = vec![("standard", sub.clone())];
        if let Some(u) = u_found {
            lines.push(("twisted", sub.iter().map(|&y| f.mul(u, y)).collect()));
        } else {
            rows.push(format!(
                "{{\"kind\":\"line\",\"N\":{n_field},\"d\":{d},\"m\":{m},\"curve\":{ci},\"a\":{a},\"b\":{b},\"Tr_a\":{ta},\"line\":\"twisted\",\"u\":null,\"u_count\":{tries}}}"
            ));
        }
        let u = u_found.unwrap_or(1);
        // common targets: uniform points of E(F_q)
        let nt = if quick {
            100
        } else if m >= 4 {
            300
        } else {
            4000
        };
        let targets: Vec<Pt> = (0..nt)
            .map(|_| loop {
                let x = 1 + rng.below(f.q1 as u64) as u32;
                if let Some(p) = lift_any(&f, &asv, &cv, x) {
                    break if rng.next() & 1 == 1 { cv.neg(&p) } else { p };
                }
            })
            .collect();
        // #E = 2 (O and the x = 0 point) + 2 * #{x != 0 lifting}
        let mut liftn = 0u64;
        for x in 1..=(f.q1 as u32) {
            let w = x ^ a ^ f.mul(b, f.inv(f.sq(x)));
            liftn += (f.tr(w) == 0) as u64;
        }
        let order = 2 + 2 * liftn;
        for (name, basis) in &lines {
            let mut inv = Bits::new(n_field);
            let mut fb: Vec<Pt> = vec![];
            let mut x = 0u32;
            for g in 1u64..(1u64 << basis.len()) {
                x ^= basis[g.trailing_zeros() as usize];
                inv.set(x);
                if let Some(p) = lift_any(&f, &asv, &cv, x) {
                    fb.push(p);
                    fb.push(cv.neg(&p));
                }
            }
            let lifting = fb.len() / 2;
            let (y, y2) = yield_ordered(&f, &cv, &fb, &inv, m, &targets);
            let pred = (fb.len() as f64).powi(m as i32) / order as f64;
            // Amendment 3: the sampler never draws O or T = (0, sqrt b), so
            // the matching expectation drops the ordered tuples summing to
            // them: (|FB|^m - C_O - C_T) / (#E - 2), C_O and C_T exact.
            let t_pt = Pt {
                x: 0,
                y: f.sqrt(b),
                inf: false,
            };
            assert!(cv.on(&f, &t_pt));
            let c_o = yield_ordered(&f, &cv, &fb, &inv, m, &[O]).0 as u64;
            let c_t = yield_ordered(&f, &cv, &fb, &inv, m, &[t_pt]).0 as u64;
            let pred_excl =
                ((fb.len() as f64).powi(m as i32) - (c_o + c_t) as f64) / (order - 2) as f64;
            rows.push(format!(
                "{{\"kind\":\"line\",\"N\":{n_field},\"d\":{d},\"m\":{m},\"curve\":{ci},\"a\":{a},\"b\":{b},\"Tr_a\":{ta},\"line\":\"{name}\",\"u\":{},\"u_count\":{tries},\"line_nonzero\":{},\"lifting\":{lifting},\"fb_points\":{},\"order\":{order},\"targets\":{nt},\"yield_ordered_mean\":{y:.6},\"yield_ordered_m2\":{y2:.6},\"yield_pred\":{pred:.6},\"c_O\":{c_o},\"c_T\":{c_t},\"yield_pred_excl\":{pred_excl:.6}}}",
                if *name == "twisted" { u } else { 1 },
                (1u64 << d) - 1,
                fb.len()
            ));
        }
    }
    rows
}

/// Exact number of u with Tr_{N/d}(u) = Tr_{N/d}(sqrt(b)/u) = 0 (u != 0).
fn u_count_exact(f: &Fq, d: u32, b: u32) -> u64 {
    let c = f.sqrt(b);
    let mut cnt = 0u64;
    for u in 1..=(f.q1 as u32) {
        if tr_rel(f, u, d) == 0 && tr_rel(f, f.mul(c, f.inv(u)), d) == 0 {
            cnt += 1;
        }
    }
    cnt
}

/// Amendment 3 census: u-count distribution over b, exhaustive (every b in
/// F_q^*) or on `sample` uniform b. With q = 2^d and n = N/d the corrected
/// theorem allows only (q - 1) * r with
/// n = 2: r in {0, q - 1} ... written as count in {0, q - 1};
/// n = 3: r in {0, 1, 2, q + 1} (zeros of a binary quadratic form on P^1);
/// n = 4: any r (points of a plane cubic), count divisible by q - 1.
fn u_census(n_field: u32, poly: u32, d: u32, sample: Option<u64>) -> String {
    let f = Fq::new(n_field, poly);
    let q1 = (1u64 << d) - 1;
    let n = n_field / d;
    let mut hist: std::collections::BTreeMap<u64, u64> = Default::default();
    let mut bad = 0u64;
    let mut rng = Rng::new(&format!("ucensus_d{d}"), n_field);
    // ker Tr_{N/d}, listed once (the u-count only ranges over it)
    let ker: Vec<u32> = (1..=(f.q1 as u32))
        .filter(|&u| tr_rel(&f, u, d) == 0)
        .collect();
    let bs: Vec<u32> = match sample {
        None => (1..=(f.q1 as u32)).collect(),
        Some(k) => (0..k).map(|_| 1 + rng.below(f.q1 as u64) as u32).collect(),
    };
    for &b in &bs {
        let cb = f.sqrt(b);
        let c = ker
            .iter()
            .filter(|&&u| tr_rel(&f, f.mul(cb, f.inv(u)), d) == 0)
            .count() as u64;
        *hist.entry(c).or_default() += 1;
        let ok = match n {
            2 => c == 0 || c == q1,
            3 => c % q1 == 0 && [0, 1, 2, q1 + 2].contains(&(c / q1)),
            _ => c % q1 == 0,
        };
        bad += (!ok) as u64;
    }
    let h: Vec<String> = hist.iter().map(|(k, v)| format!("\"{k}\":{v}")).collect();
    format!(
        "{{\"kind\":\"u_census\",\"N\":{n_field},\"d\":{d},\"n\":{n},\"exhaustive\":{},\"b_count\":{},\"violations\":{bad},\"hist\":{{{}}}}}",
        sample.is_none(),
        bs.len(),
        h.join(",")
    )
}

/// (N, d) cells for the composite-degree line study.
const LINE_CELLS: &[(u32, u32)] = &[
    (12, 6),
    (16, 8),
    (20, 10),
    (18, 9),
    (15, 5),
    (21, 7),
    (18, 6),
    (16, 4),
    (20, 5),
];

// -------------------------------------------------------------- driver ----

struct Inst {
    arm: &'static str,
    label: String,
    a: u32,
    b: u32,
    s: Bits,
}

fn json_list<T: std::fmt::Display>(v: &[T]) -> String {
    let mut s = String::from("[");
    for (i, x) in v.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        let _ = write!(s, "{x}");
    }
    s.push(']');
    s
}

fn run_n(n: u32, poly: u32, quick: bool) -> Vec<String> {
    let t0 = Instant::now();
    let f = Fq::new(n, poly);
    let mut rng = Rng::new("curves", n);
    let mut insts: Vec<Inst> = vec![];
    insts.push(Inst {
        arm: "curve",
        label: "koblitz_a0_b1".into(),
        a: 0,
        b: 1,
        s: lift_set(&f, 0, 1),
    });
    insts.push(Inst {
        arm: "curve",
        label: "koblitz_a1_b1".into(),
        a: 1,
        b: 1,
        s: lift_set(&f, 1, 1),
    });
    let extra = if n <= 11 && !quick { 24 } else { 4 };
    for i in 0..extra {
        let a = (i % 2) as u32;
        let b = 2 + rng.below(f.q1 as u64 - 1) as u32;
        insts.push(Inst {
            arm: "curve",
            label: format!("random_{i}"),
            a,
            b,
            s: lift_set(&f, a, b),
        });
    }
    // density-matched nulls: null_i is a uniformly random set with exactly
    // the lift-set size of curve i (paired), so subspace counts compare at
    // equal density.
    let ncurves = insts.len();
    let base = members(n, &insts[0].s);
    let q1 = f.q1 as u32;
    let make_null = |tag: &str, size: usize| -> Bits {
        let mut r = Rng::new(tag, n);
        let mut perm: Vec<u32> = (1..=q1).collect();
        for i in (1..perm.len()).rev() {
            let j = r.below(i as u64 + 1) as usize;
            perm.swap(i, j);
        }
        let mut s = Bits::new(n);
        for &x in &perm[..size] {
            s.set(x);
        }
        s
    };
    for i in 0..ncurves {
        let tag = format!("null_{i}");
        let size = members(n, &insts[i].s).len();
        let s = make_null(&tag, size);
        insts.push(Inst {
            arm: "null",
            label: tag,
            a: 0,
            b: 0,
            s,
        });
    }
    let kp = (null_kmax(n) + 3).min(n - 2);
    for i in 0..3 {
        let tag = format!("planted_{i}");
        let mut s = make_null(&tag, base.len());
        let mut r = Rng::new(&format!("{tag}_plant"), n);
        let pb = random_basis(n, kp as usize, &mut r);
        let mut x = 0u32;
        let mut added = 0usize;
        for g in 1u64..(1u64 << kp) {
            x ^= pb[g.trailing_zeros() as usize];
            if !s.get(x) {
                s.set(x);
                added += 1;
            }
        }
        // restore the density by clearing random members outside the plant
        let mut pset = Bits::new(n);
        let mut x = 0u32;
        for g in 1u64..(1u64 << kp) {
            x ^= pb[g.trailing_zeros() as usize];
            pset.set(x);
        }
        while added > 0 {
            let y = 1 + r.below(q1 as u64) as u32;
            if s.get(y) && !pset.get(y) {
                s.clear(y);
                added -= 1;
            }
        }
        insts.push(Inst {
            arm: "planted",
            label: tag,
            a: 0,
            b: 0,
            s,
        });
    }

    let exact = n <= 13 && !quick;
    let budget: u64 = if quick { 1 << 24 } else { 1 << 31 };
    let thresh = 2048usize;
    let ls_extra = 6usize;
    let iters = if quick { 200 } else { 1500 };
    let rows: Vec<String> = std::thread::scope(|sc| {
        let hs: Vec<_> = insts
            .iter()
            .map(|inst| {
                let f = &f;
                sc.spawn(move || {
                    let ti = Instant::now();
                    let all = members(n, &inst.s);
                    let density = all.len() as f64 / f.q1 as f64;
                    let mut rng = Rng::new(&format!("search_{}", inst.label), n);
                    // exact (n <= 13): unpruned exhaustive DFS from the root;
                    // counts[k] = number of k-dim all-lift subspaces, and the
                    // largest k with a nonzero count is the exact kmax.
                    let (k_exact, exact_capped, exact_ops, counts) = if exact {
                        let mut sc = Search::new(n, &inst.s, 1 << 44);
                        sc.count_mode = true;
                        sc.restart(&all, usize::MAX, &mut rng);
                        let c: Vec<u64> = sc.counts.iter().copied().take_while(|&c| c > 0).collect();
                        (Some(c.len() - 1), sc.capped, sc.ops, c)
                    } else {
                        (None, false, 0, vec![])
                    };
                    // budgeted
                    let mut sb = Search::new(n, &inst.s, budget);
                    let mut restarts = 0u64;
                    let mut budget_exhaustive = false;
                    while sb.ops < budget {
                        let before = sb.ops;
                        sb.restart(&all, thresh, &mut rng);
                        restarts += 1;
                        // a root-level exhaustive pass that finished is exact
                        if (all.len() <= thresh && !sb.capped) || sb.ops == before {
                            budget_exhaustive = !sb.capped;
                            break;
                        }
                        sb.capped = false;
                    }
                    let kb = sb.best;
                    assert!(all_lift(&inst.s, &sb.best_basis) && rank(&sb.best_basis) == kb);
                    let (sd, sk) = if inst.arm == "curve" {
                        structured(f, &inst.s, inst.a, inst.b, 200, &mut rng)
                    } else {
                        (0, 0)
                    };
                    // density above kmax
                    let mut ls = vec![];
                    let mut rho = vec![];
                    let mut rho_rand = vec![];
                    for l in kb + 1..=(kb + ls_extra).min(16).min(n as usize - 1) {
                        let best = hill_climb(n, &inst.s, &sb.best_basis, l, iters, &mut rng);
                        let mut rr = 0f64;
                        for _ in 0..32 {
                            let b = random_basis(n, l, &mut rng);
                            rr += count_in(&inst.s, &b) as f64;
                        }
                        let tot = ((1u64 << l) - 1) as f64;
                        ls.push(l);
                        rho.push(format!("{:.6}", best as f64 / tot));
                        rho_rand.push(format!("{:.6}", rr / 32.0 / tot));
                    }
                    let mut row = String::new();
                    let _ = write!(
                        row,
                        "{{\"kind\":\"kmax\",\"n\":{n},\"arm\":\"{}\",\"label\":\"{}\",\"a\":{},\"b\":{},\"density\":{:.6},\"null_kmax_pred\":{},\"planted_k\":{},\"k_exact\":{},\"subspace_counts\":{},\"exact_capped\":{},\"exact_ops\":{},\"k_budget\":{},\"budget_ops\":{},\"restarts\":{},\"budget_exhaustive\":{},\"best_basis\":{},\"struct_d\":{},\"struct_k\":{},\"ls\":{},\"rho_star\":{},\"rho_rand\":{},\"secs\":{:.2}}}",
                        inst.arm,
                        inst.label,
                        inst.a,
                        inst.b,
                        density,
                        null_kmax(n),
                        if inst.arm == "planted" { kp.to_string() } else { "null".into() },
                        k_exact.map(|k| k.to_string()).unwrap_or("null".into()),
                        json_list(&counts),
                        exact_capped,
                        exact_ops,
                        kb,
                        sb.ops,
                        restarts,
                        budget_exhaustive,
                        json_list(&sb.best_basis),
                        sd,
                        sk,
                        json_list(&ls),
                        json_list(&rho),
                        json_list(&rho_rand),
                        ti.elapsed().as_secs_f64()
                    );
                    (row, sb.best_basis.clone())
                })
            })
            .collect();
        let res: Vec<(String, Vec<u32>)> = hs.into_iter().map(|h| h.join().unwrap()).collect();
        let mut rows: Vec<String> = res.iter().map(|r| r.0.clone()).collect();
        // m = 2 yield check, odd n, koblitz_a0 curve: dense V (the all-lift
        // basis extended by 2 hill-climbed... here: the all-lift basis itself)
        // against random subspaces of the same dimension.
        if n % 2 == 1 && n >= 11 {
            let inst = &insts[0];
            let cv = Curve {
                a: inst.a,
                b: inst.b,
            };
            let order = 2 + 2 * members(n, &inst.s).len() as u64;
            let dense = &res[0].1;
            let l = dense.len();
            let mut ry = Rng::new("yield", n);
            let targets = if quick { 500 } else { 20000 };
            let (md, pd, kd) = yield_m2(&f, &cv, &inst.s, dense, targets, order, &mut ry);
            let mut rnd = vec![];
            for j in 0..4 {
                let b = random_basis(n, l, &mut ry);
                let (mr, pr, kr) = yield_m2(&f, &cv, &inst.s, &b, targets, order, &mut ry);
                rnd.push(format!(
                    "{{\"j\":{j},\"k\":{kr},\"mean\":{mr:.6},\"pred\":{pr:.6}}}"
                ));
            }
            rows.push(format!(
                "{{\"kind\":\"yield_m2\",\"n\":{n},\"label\":\"{}\",\"order\":{order},\"l\":{l},\"targets\":{targets},\"dense\":{{\"k\":{kd},\"mean\":{md:.6},\"pred\":{pd:.6}}},\"random\":{}}}",
                inst.label,
                json_list(&rnd)
            ));
        }
        rows
    });
    eprintln!("n = {n}: {:.1} s", t0.elapsed().as_secs_f64());
    rows
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("quick");
    let out = args.get(2).cloned().unwrap_or_else(|| "out".into());
    let quick = mode != "full" && mode != "lines";
    let lines_only = mode == "lines";
    // optional third argument: largest field degree to run (for timing tests)
    let maxn: u32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(99);
    std::fs::create_dir_all(&out).unwrap();
    if mode == "ucount" {
        // ucount <N> <d> <b>: exact u-count for one curve
        let nf: u32 = args[2].parse().unwrap();
        let d: u32 = args[3].parse().unwrap();
        let b: u32 = args[4].parse().unwrap();
        let poly = FIELDS.iter().find(|x| x.0 == nf).unwrap().1;
        let f = Fq::new(nf, poly);
        println!("N={nf} d={d} b={b} u_count={}", u_count_exact(&f, d, b));
        return;
    }
    if mode == "census" {
        std::fs::create_dir_all(&out).unwrap();
        let mut fh = std::fs::File::create(format!("{out}/u_census.jsonl")).unwrap();
        for &(nf, d, sample) in &[
            (12u32, 6u32, None),
            (15, 5, None),
            (18, 6, None),
            (18, 9, None),
            (16, 4, None),
            (21, 7, Some(4096u64)),
            (20, 5, Some(4096)),
        ] {
            let poly = FIELDS.iter().find(|x| x.0 == nf).unwrap().1;
            let r = u_census(nf, poly, d, sample);
            eprintln!("{r}");
            writeln!(fh, "{r}").unwrap();
        }
        return;
    }
    let mut fh = std::fs::File::create(format!("{out}/rows.jsonl")).unwrap();
    let t0 = Instant::now();
    for &(n, poly) in FIELDS {
        if (quick && n > 17) || n > maxn || lines_only {
            continue;
        }
        for r in run_n(n, poly, quick) {
            writeln!(fh, "{r}").unwrap();
        }
        fh.flush().unwrap();
    }
    for &(nf, d) in LINE_CELLS {
        if (quick && nf > 16) || nf > maxn {
            continue;
        }
        let poly = FIELDS.iter().find(|x| x.0 == nf).unwrap().1;
        let t = Instant::now();
        for r in run_lines(nf, poly, d, quick) {
            writeln!(fh, "{r}").unwrap();
        }
        fh.flush().unwrap();
        eprintln!(
            "lines N = {nf}, d = {d}: {:.1} s",
            t.elapsed().as_secs_f64()
        );
    }
    let mut run = std::fs::File::create(format!("{out}/run.json")).unwrap();
    writeln!(
        run,
        "{{\"mode\":\"{mode}\",\"seed\":\"{SEED:#x}\",\"secs\":{:.1}}}",
        t0.elapsed().as_secs_f64()
    )
    .unwrap();
}
