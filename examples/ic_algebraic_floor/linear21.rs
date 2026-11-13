//! Table-free `2 + 1` three-summand decision by linear algebra over `F₂`
//! (`research/groebner_free_matched_20261113/`).
//!
//! For a target `R` and every base point `P₃` (both signs), `Q = R − P₃` is
//! tested for `Q = ±P₁ ± P₂` with `x(P_i) ∈ V = span(1, z, …, z^{d−1})`.
//! With `s = x₁ + x₂`, `p = x₁x₂` and `u = x(Q)`, Semaev's third summation
//! polynomial on `y² + xy = x³ + ax² + b` is
//!
//! ```text
//! S₃(x₁, x₂, u) = p² + u·p + u²·s² + b,
//! ```
//!
//! which is `F₂`-affine in `(s, p)` because squaring is linear.  No pair
//! table and no Gröbner basis: the memory is the factor base itself.
//!
//! Two solvers of the same decision:
//!
//! * [`Variant::Full`] — the repository's existing linear solve
//!   (`examples/ic_m2_linear.rs`, `ic_m3_cycle.rs`): `n` equations in the
//!   `3d − 1` bits of `(s, p)`, `p ∈ span(1, …, z^{2d−2})`.
//! * [`Variant::HalfTrace`] — `p` eliminated first.  Put `p = u·t`; then
//!   `t² + t = s² + c` with `c = b/u²`, so `t = s + H(s) + H(c) + ε` with
//!   `H` the half-trace (`n` odd, `H(a)² + H(a) = a + Tr(a)`) and
//!   `ε ∈ F₂`, subject to `Tr(s) = Tr(c)`.  The decision is the linear
//!   system "the top `n − (2d − 1)` coordinates of
//!   `u·(L(s) + H(c) + ε)` vanish, and `Tr(s) = Tr(c)`", `L = id + H`:
//!   `n − 2d + 2` equations in `d + 1` unknown bits.
//!
//! Both then split `z² + s·z + p` over `F₂[z]_{<d}` (linear again) and
//! verify the summands in the group.

use crypto_lib::cryptanalysis::koblitz_fast::{BatchScratch, FastCurve, FastPoint};
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Full,
    HalfTrace,
}

#[derive(Default, Clone, Copy)]
pub struct Counts {
    /// Third summands tried (`Q = R − P₃` formed and tested).
    pub trials: u64,
    /// Trials whose linear system was consistent.
    pub consistent: u64,
    /// `(s, p)` candidates handed to the split.
    pub splits: u64,
    /// Word XORs of every `F₂` elimination.
    pub gauss_xors: u64,
    /// Field multiplications, squarings and inversions of the test itself
    /// (the group additions are counted by `trials` and `verify_adds`).
    pub field_muls: u64,
    pub field_invs: u64,
    pub verify_adds: u64,
    /// A kernel larger than the enumeration cap was truncated: a "no" is
    /// then not a refutation.
    pub truncated: bool,
}

/// An `F₂`-linear map on field elements, as byte tables.
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

pub struct Linear21<'a> {
    gf: &'a Gf2,
    fast: &'a FastCurve,
    n: u32,
    d: usize,
    b: u64,
    /// Base points and their negatives, in base order.
    points: Vec<FastPoint>,
    negated: Vec<FastPoint>,
    /// `2·x + sign` → base index, `usize::MAX` when absent; `x < 2^d`.
    index_of: Vec<usize>,
    /// `z^{2j}` in the field, `j < 2d − 1` (full variant).
    z2: Vec<u64>,
    /// `L(z^j) = z^j + H(z^j)`, `j < d`, and `Tr(z^j)` as a bit mask.
    ell: Vec<u64>,
    trace_mask: u64,
    half_trace: LinMap,
    sqrt: LinMap,
    /// `1/s²` for every `s ∈ V \ {0}` (index `s`), for the half-trace split.
    inv_s2: Vec<u64>,
    /// Kernel dimensions enumerated in full; beyond it the run is truncated.
    pub kernel_cap: usize,
}

#[inline(always)]
fn sign_bit(p: FastPoint) -> usize {
    usize::from(p.y > (p.x ^ p.y))
}

/// Solve `Σ u_k cols[k] = rhs` over `F₂`; a particular solution and a
/// kernel basis (as combinations of the columns), or `None`.
fn solve_f2(cols: &[u64], rhs: u64, xors: &mut u64, kernel: &mut Vec<u64>) -> Option<u64> {
    let mut piv: [(u64, u64); 64] = [(0, 0); 64];
    kernel.clear();
    for (k, &col) in cols.iter().enumerate() {
        let (mut v, mut comb) = (col, 1u64 << k);
        while v != 0 {
            let top = 63 - v.leading_zeros() as usize;
            if piv[top].0 == 0 {
                piv[top] = (v, comb);
                break;
            }
            v ^= piv[top].0;
            comb ^= piv[top].1;
            *xors += 1;
        }
        if v == 0 {
            kernel.push(comb);
        }
    }
    let (mut v, mut comb) = (rhs, 0u64);
    while v != 0 {
        let top = 63 - v.leading_zeros() as usize;
        if piv[top].0 == 0 {
            return None;
        }
        v ^= piv[top].0;
        comb ^= piv[top].1;
        *xors += 1;
    }
    Some(comb)
}

/// Carry-less product of two polynomials of degree below 32.
#[inline(always)]
fn clmul(mut a: u64, b: u64) -> u64 {
    let mut acc = 0;
    let mut i = 0;
    while a != 0 {
        if a & 1 == 1 {
            acc ^= b << i;
        }
        a >>= 1;
        i += 1;
    }
    acc
}

impl<'a> Linear21<'a> {
    pub fn new(
        gf: &'a Gf2,
        fast: &'a FastCurve,
        n: u32,
        d: usize,
        b: u64,
        base: Vec<FastPoint>,
    ) -> Self {
        assert!(n % 2 == 1, "the half-trace needs odd n");
        assert!(2 * d - 1 <= n as usize, "p must need no reduction");
        assert!(3 * d <= 64, "one word per combination");
        let mut index_of = vec![usize::MAX; 2usize << d];
        for (i, p) in base.iter().enumerate() {
            assert!(!p.infinity && p.x < (1u64 << d));
            index_of[2 * p.x as usize + sign_bit(*p)] = i;
        }
        let negated = base.iter().map(|&p| fast.neg(p)).collect();
        let zpow = |k: u32| -> u64 {
            let mut t = 1u64;
            for _ in 0..k {
                t = gf.mul(t, 2);
            }
            t
        };
        let half = |a: u64| -> u64 {
            let (mut t, mut h) = (a, a);
            for _ in 0..(n - 1) / 2 {
                t = gf.sqr(gf.sqr(t));
                h ^= t;
            }
            h
        };
        let trace = |a: u64| -> u64 {
            let (mut t, mut tr) = (a, a);
            for _ in 1..n {
                t = gf.sqr(t);
                tr ^= t;
            }
            assert!(tr <= 1);
            tr
        };
        let z2 = (0..2 * d as u32 - 1).map(|j| zpow(2 * j)).collect();
        let ell = (0..d as u32)
            .map(|j| (1u64 << j) ^ half(1u64 << j))
            .collect();
        let trace_mask = (0..d as u32).fold(0u64, |m, j| m | (trace(1u64 << j) << j));
        let half_trace = LinMap::new(n, |bit| half(1u64 << bit));
        let sqrt = LinMap::new(n, |bit| gf.sqr_k(1u64 << bit, n - 1));
        let mut inv_s2: Vec<u64> = (0..1u64 << d).collect();
        inv_s2[0] = 1;
        gf.batch_inv(&mut inv_s2, &mut Vec::new());
        for v in inv_s2.iter_mut() {
            *v = gf.sqr(*v);
        }
        Self {
            gf,
            fast,
            n,
            d,
            b,
            points: base,
            negated,
            index_of,
            z2,
            ell,
            trace_mask,
            half_trace,
            sqrt,
            inv_s2,
            kernel_cap: 16,
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

    /// Split `z² + s·z + p` over `V` and verify `Q = P_j + P_k` over the
    /// signed base; the two base indices on success.
    fn split_and_verify(
        &self,
        q: FastPoint,
        s: u64,
        p: u64,
        c: &mut Counts,
        scratch: &mut Vec<u64>,
    ) -> Option<(usize, usize)> {
        let d = self.d;
        c.splits += 1;
        let x1 = if s == 0 {
            // A double root: x₁ = √p.
            self.sqrt.apply(p)
        } else {
            let mut cols = [0u64; 32];
            for (i, col) in cols.iter_mut().enumerate().take(d) {
                *col = (1u64 << (2 * i)) ^ clmul(s, 1u64 << i);
            }
            solve_f2(&cols[..d], p, &mut c.gauss_xors, scratch)?
        };
        let x2 = x1 ^ s;
        if x1 == 0 || x2 == 0 || x1 >> d != 0 || x2 >> d != 0 {
            return None;
        }
        self.verify(q, x1, x2, c)
    }

    /// Either sign of either summand against `Q` in the group.
    fn verify(&self, q: FastPoint, x1: u64, x2: u64, c: &mut Counts) -> Option<(usize, usize)> {
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
                c.verify_adds += 1;
                if self.fast.add(self.points[j], self.points[k]) == q {
                    return Some((j, k));
                }
            }
        }
        None
    }

    /// The roots of `z² + s·z + p` by the half-trace: `x₁ = s·y` with
    /// `y² + y = p/s²`, so no elimination.  Same verification.
    fn split_root_and_verify(
        &self,
        q: FastPoint,
        s: u64,
        p: u64,
        c: &mut Counts,
    ) -> Option<(usize, usize)> {
        let (d, gf) = (self.d, self.gf);
        c.splits += 1;
        let x1 = if s == 0 {
            self.sqrt.apply(p)
        } else {
            let w = gf.mul(p, self.inv_s2[s as usize]);
            let y = self.half_trace.apply(w);
            c.field_muls += 3;
            if gf.sqr(y) ^ y != w {
                return None; // Tr(p/s²) = 1: no root in the field
            }
            gf.mul(s, y)
        };
        let x2 = x1 ^ s;
        if x1 == 0 || x2 == 0 || x1 >> d != 0 || x2 >> d != 0 {
            return None;
        }
        self.verify(q, x1, x2, c)
    }

    fn pair_full(&self, q: FastPoint, c: &mut Counts, ws: &mut Scratch) -> Option<(usize, usize)> {
        let d = self.d;
        let u = q.x;
        let u2 = self.gf.sqr(u);
        c.field_muls += 1;
        ws.cols.clear();
        let mut t = u2;
        for _ in 0..d {
            ws.cols.push(t);
            t = self.times_z(self.times_z(t));
        }
        let mut t = u;
        for j in 0..2 * d - 1 {
            ws.cols.push(self.z2[j] ^ t);
            t = self.times_z(t);
        }
        let part = solve_f2(&ws.cols, self.b, &mut c.gauss_xors, &mut ws.kernel)?;
        c.consistent += 1;
        let dim = ws.kernel.len();
        if dim > self.kernel_cap {
            c.truncated = true;
        }
        let dim = dim.min(self.kernel_cap);
        let dmask = (1u64 << d) - 1;
        for sel in 0u64..(1u64 << dim) {
            let mut comb = part;
            for i in 0..dim {
                if (sel >> i) & 1 == 1 {
                    comb ^= ws.kernel[i];
                }
            }
            if let Some(w) = self.split_and_verify(q, comb & dmask, comb >> d, c, &mut ws.inner) {
                return Some(w);
            }
        }
        None
    }

    fn pair_half_trace(
        &self,
        q: FastPoint,
        ui: u64,
        c: &mut Counts,
        ws: &mut Scratch,
    ) -> Option<(usize, usize)> {
        let (d, n) = (self.d, self.n as usize);
        let gf = self.gf;
        let u = q.x;
        let cc = gf.mul(self.b, gf.sqr(ui));
        c.field_muls += 2;
        let hc = self.half_trace.apply(cc);
        // Tr(c) = H(c)² + H(c) + c.
        let tr_c = gf.sqr(hc) ^ hc ^ cc;
        c.field_muls += 1;
        debug_assert!(tr_c <= 1);
        // Rows: the coordinates z^{2d−1} … z^{n−1} of the product, then the
        // trace condition in the next bit.
        let shift = 2 * d - 1;
        let tbit = n - shift;
        ws.cols.clear();
        for j in 0..d {
            let top = gf.mul(u, self.ell[j]) >> shift;
            ws.cols.push(top | (((self.trace_mask >> j) & 1) << tbit));
        }
        ws.cols.push(u >> shift); // ε
        c.field_muls += d as u64 + 1;
        let rhs = (gf.mul(u, hc) >> shift) | (tr_c << tbit);
        let part = solve_f2(&ws.cols, rhs, &mut c.gauss_xors, &mut ws.kernel)?;
        c.consistent += 1;
        let dim = ws.kernel.len();
        if dim > self.kernel_cap {
            c.truncated = true;
        }
        let dim = dim.min(self.kernel_cap);
        let dmask = (1u64 << d) - 1;
        for sel in 0u64..(1u64 << dim) {
            let mut comb = part;
            for i in 0..dim {
                if (sel >> i) & 1 == 1 {
                    comb ^= ws.kernel[i];
                }
            }
            let s = comb & dmask;
            let eps = (comb >> d) & 1;
            // L(s) = s + H(s).
            let ls = s ^ self.half_trace.apply(s);
            let p = gf.mul(u, ls ^ hc ^ eps);
            c.field_muls += 1;
            debug_assert!(p >> shift == 0, "p must lie in span(1..z^(2d-2))");
            if let Some(w) = self.split_root_and_verify(q, s, p, c) {
                return Some(w);
            }
        }
        None
    }

    /// Does `R = P_i + P_j + P_k` over the signed base?  Stops at the first
    /// verified witness.  `expired` is polled every block of 256 trials.
    pub fn decide(
        &self,
        r: FastPoint,
        variant: Variant,
        c: &mut Counts,
        mut expired: impl FnMut() -> bool,
    ) -> Result<Option<[usize; 3]>, ()> {
        const BLOCK: usize = 256;
        let mut ws = Scratch::default();
        let mut batch = BatchScratch::default();
        let mut qs: Vec<FastPoint> = Vec::with_capacity(BLOCK);
        let mut uinv: Vec<u64> = Vec::with_capacity(BLOCK);
        let mut tmp: Vec<u64> = Vec::new();
        let mut lo = 0;
        while lo < self.points.len() {
            if expired() {
                return Err(());
            }
            let hi = (lo + BLOCK).min(self.points.len());
            // Q_i = R − P_i for the block, one inversion (Montgomery).
            qs.clear();
            self.fast
                .add_many(r, &self.negated[lo..hi], &mut qs, &mut batch);
            if variant == Variant::HalfTrace {
                // 1/x(Q_i) for the block, one more inversion.
                uinv.clear();
                uinv.extend(
                    qs.iter()
                        .map(|q| if q.infinity || q.x == 0 { 1 } else { q.x }),
                );
                self.gf.batch_inv(&mut uinv, &mut tmp);
                c.field_invs += 1;
                c.field_muls += 3 * (hi - lo) as u64;
            }
            for (off, &q) in qs.iter().enumerate() {
                c.trials += 1;
                if q.infinity || q.x == 0 {
                    continue;
                }
                let hit = match variant {
                    Variant::Full => self.pair_full(q, c, &mut ws),
                    Variant::HalfTrace => self.pair_half_trace(q, uinv[off], c, &mut ws),
                };
                if let Some((j, k)) = hit {
                    return Ok(Some([lo + off, j, k]));
                }
            }
            lo = hi;
        }
        Ok(None)
    }
}

#[derive(Default)]
struct Scratch {
    cols: Vec<u64>,
    kernel: Vec<u64>,
    inner: Vec<u64>,
}
