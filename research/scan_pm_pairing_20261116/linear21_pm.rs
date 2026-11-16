//! `lin21-ht` leaf with interchangeable t stages
//! (`research/scan_pm_pairing_20261116/`).
//!
//! The leaf (`leaf`, `split_root_and_verify`, `verify`, `solve_f2`) is the
//! half-trace variant of `examples/ic_algebraic_floor/linear21.rs` with two
//! additions: an operation counter behind a const generic (`K`), and a lazy
//! ordinate (`QSrc`) so that a t stage that does not compute `y(Q)` can still
//! verify.  The repository's own `Counts` increments are kept in every mode.
//!
//! t stages (`Q_i = R − P_i`, `t = x(Q_i)`, `1/t`):
//!
//! * `decide_base` — as the repository: `FastCurve::add_many` on a block of
//!   256 negated base points, then `Gf2::batch_inv` on the abscissae.
//! * `decide_pm` — ±P pairing with ONE shared inversion per pair.  The two
//!   base points above an abscissa `x` give `t₁ = x(R − P)`, `t₂ = x(R + P)`,
//!   the two roots of `e²t² + Πt + Π² + b` with `e = x_R + x`, `Π = x·x_R`.
//!   With `g = Π + √b` the single batched inverse of `e·g` gives `1/e` and
//!   `e/g`; then `t₂ = t₁ + Π/e²`, `1/t₁ = t₂·(e/g)²`, `1/t₂ = t₁·(e/g)²`.
//!   11 mul + 4 sqr per pair after the batch, against 16 mul + 2 sqr.
//! * `decide_lz` — the library's `add_many_lazy` (abscissa and slope, the
//!   eight-lane kernel where it runs), then `batch_inv` on the abscissae.
//! * `decide_lp` — the library's existing pairing
//!   `add_many_lazy_soa_paired` (one shared inverse of `x_R + x` per pair),
//!   then `batch_inv` on the abscissae.

use crypto_lib::cryptanalysis::koblitz_fast::{BatchScratch, FastCurve, FastPoint};
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;

pub const ST_T: usize = 0;
pub const ST_BUILD: usize = 1;
pub const ST_SOLVE: usize = 2;
pub const ST_SPLIT: usize = 3;
pub const ST_MEMBER: usize = 4;
pub const OP_MUL: usize = 0;
pub const OP_SQR: usize = 1;
pub const OP_INV: usize = 2;
pub const OP_TLOOK: usize = 3;
pub const OP_WOP: usize = 4;

/// Counted operations by stage and kind; filled only when `K`.
#[derive(Default, Clone, Copy)]
pub struct Ops {
    pub s: [[u64; 5]; 5],
    /// Leaves whose t came from the per-point fallback (PM/LP).
    pub fallback_leaves: u64,
    /// LP: blocks the library's paired kernel computed / declined.
    pub lp_kernel_blocks: u64,
    pub lp_declined_blocks: u64,
}

/// The repository's `Counts`, field for field.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Counts {
    pub trials: u64,
    pub consistent: u64,
    pub splits: u64,
    pub gauss_xors: u64,
    pub field_muls: u64,
    pub field_invs: u64,
    pub verify_adds: u64,
    pub truncated: bool,
}

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

/// Where a leaf's full point comes from when a verification needs it.
#[derive(Clone, Copy)]
pub enum QSrc {
    Full(FastPoint),
    /// `y = λ(x_R + x) + x + y_R`.
    Lazy { x: u64, lam: u64 },
    /// The pair's second member: `λ₂ = λ + x₁/e`.
    Lazy2 { x: u64, lam: u64, x1: u64, ie: u64 },
    /// A sum of `add_many_lazy` and its slope.
    Lib { sum: FastPoint, lam: u64 },
}

pub struct Lin<'a> {
    gf: &'a Gf2,
    fast: &'a FastCurve,
    n: u32,
    d: usize,
    b: u64,
    sb: u64,
    bytes: u64,
    pub points: Vec<FastPoint>,
    negated: Vec<FastPoint>,
    index_of: Vec<usize>,
    ell: Vec<u64>,
    trace_mask: u64,
    half_trace: LinMap,
    sqrt: LinMap,
    inv_s2: Vec<u64>,
    pub kernel_cap: usize,
    /// Base index `i` starts a pair: `points[i + 1] = −points[i]`.
    pair_start: Vec<bool>,
    /// Pair index of a pair start.
    pair_of: Vec<u32>,
    /// Per pair: the abscissa and the ordinates of `−P_i`, `−P_{i+1}`.
    pair_x: Vec<u64>,
    pair_ya: Vec<u64>,
    pair_yb: Vec<u64>,
    pub pairs: usize,
    pub singles: usize,
}

#[inline(always)]
pub fn sign_bit(p: FastPoint) -> usize {
    usize::from(p.y > (p.x ^ p.y))
}

#[inline(always)]
fn solve_f2<const K: bool>(
    cols: &[u64],
    rhs: u64,
    xors: &mut u64,
    wops: &mut u64,
    kernel: &mut Vec<u64>,
) -> Option<u64> {
    let mut piv: [(u64, u64); 64] = [(0, 0); 64];
    kernel.clear();
    for (k, &col) in cols.iter().enumerate() {
        let (mut v, mut comb) = (col, 1u64 << k);
        while v != 0 {
            let top = 63 - v.leading_zeros() as usize;
            if K {
                *wops += 4;
            }
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
        if K {
            *wops += 4;
        }
        if piv[top].0 == 0 {
            return None;
        }
        v ^= piv[top].0;
        comb ^= piv[top].1;
        *xors += 1;
    }
    Some(comb)
}

#[derive(Default)]
struct Scratch {
    cols: Vec<u64>,
    kernel: Vec<u64>,
}

#[derive(Default)]
struct PmBuf {
    v: Vec<u64>,
    g: Vec<u64>,
    pp: Vec<u64>,
    t: Vec<u64>,
    ti: Vec<u64>,
    lam: Vec<u64>,
    ie: Vec<u64>,
}

const BLOCK: usize = 256;

impl<'a> Lin<'a> {
    pub fn new(
        gf: &'a Gf2,
        fast: &'a FastCurve,
        n: u32,
        d: usize,
        b: u64,
        base: Vec<FastPoint>,
    ) -> Self {
        assert!(n % 2 == 1 && 2 * d - 1 <= n as usize && 3 * d <= 64);
        let mut index_of = vec![usize::MAX; 2usize << d];
        for (i, p) in base.iter().enumerate() {
            assert!(!p.infinity && p.x < (1u64 << d));
            index_of[2 * p.x as usize + sign_bit(*p)] = i;
        }
        let negated: Vec<FastPoint> = base.iter().map(|&p| fast.neg(p)).collect();
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
        let sb = sqrt.apply(b);
        assert_eq!(gf.sqr(sb), b);
        let len = base.len();
        let mut pair_start = vec![false; len];
        let mut pair_of = vec![u32::MAX; len];
        let (mut pair_x, mut pair_ya, mut pair_yb) = (Vec::new(), Vec::new(), Vec::new());
        let mut i = 0;
        let mut singles = 0;
        while i < len {
            if i + 1 < len && base[i + 1] == negated[i] && base[i + 1] != base[i] {
                pair_start[i] = true;
                pair_of[i] = pair_x.len() as u32;
                pair_x.push(base[i].x);
                pair_ya.push(negated[i].y);
                pair_yb.push(negated[i + 1].y);
                i += 2;
            } else {
                singles += 1;
                i += 1;
            }
        }
        let pairs = pair_x.len();
        Self {
            gf,
            fast,
            n,
            d,
            b,
            sb,
            bytes: n.div_ceil(8) as u64,
            points: base,
            negated,
            index_of,
            ell,
            trace_mask,
            half_trace,
            sqrt,
            inv_s2,
            kernel_cap: 16,
            pair_start,
            pair_of,
            pair_x,
            pair_ya,
            pair_yb,
            pairs,
            singles,
        }
    }

    #[inline(always)]
    fn resolve<const K: bool>(&self, r: FastPoint, qs: QSrc, o: &mut Ops) -> FastPoint {
        let gf = self.gf;
        match qs {
            QSrc::Full(q) => q,
            QSrc::Lazy { x, lam } => {
                if K {
                    o.s[ST_MEMBER][OP_MUL] += 1;
                }
                FastPoint::affine(x, gf.mul(lam, r.x ^ x) ^ x ^ r.y)
            }
            QSrc::Lazy2 { x, lam, x1, ie } => {
                if K {
                    o.s[ST_MEMBER][OP_MUL] += 2;
                }
                let lam2 = lam ^ gf.mul(x1, ie);
                FastPoint::affine(x, gf.mul(lam2, r.x ^ x) ^ x ^ r.y)
            }
            QSrc::Lib { sum, lam } => {
                if K {
                    o.s[ST_MEMBER][OP_MUL] += 1;
                }
                self.fast.finish_lazy(r, sum, lam)
            }
        }
    }

    fn verify<const K: bool>(
        &self,
        q: FastPoint,
        x1: u64,
        x2: u64,
        c: &mut Counts,
        o: &mut Ops,
    ) -> Option<(usize, usize)> {
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
                if K {
                    o.s[ST_MEMBER][OP_INV] += 1;
                    o.s[ST_MEMBER][OP_MUL] += 2;
                    o.s[ST_MEMBER][OP_SQR] += 1;
                }
                if self.fast.add(self.points[j], self.points[k]) == q {
                    return Some((j, k));
                }
            }
        }
        None
    }

    #[allow(clippy::too_many_arguments)]
    fn split_root_and_verify<const K: bool>(
        &self,
        r: FastPoint,
        qs: QSrc,
        q: &mut Option<FastPoint>,
        s: u64,
        p: u64,
        c: &mut Counts,
        o: &mut Ops,
    ) -> Option<(usize, usize)> {
        let (d, gf) = (self.d, self.gf);
        c.splits += 1;
        let x1 = if s == 0 {
            if K {
                o.s[ST_SPLIT][OP_TLOOK] += self.bytes;
            }
            self.sqrt.apply(p)
        } else {
            let w = gf.mul(p, self.inv_s2[s as usize]);
            let y = self.half_trace.apply(w);
            c.field_muls += 3;
            if K {
                o.s[ST_SPLIT][OP_MUL] += 1;
                o.s[ST_SPLIT][OP_SQR] += 1;
                o.s[ST_SPLIT][OP_TLOOK] += self.bytes;
            }
            if gf.sqr(y) ^ y != w {
                return None;
            }
            if K {
                o.s[ST_SPLIT][OP_MUL] += 1;
            }
            gf.mul(s, y)
        };
        let x2 = x1 ^ s;
        if x1 == 0 || x2 == 0 || x1 >> d != 0 || x2 >> d != 0 {
            return None;
        }
        if q.is_none() {
            *q = Some(self.resolve::<K>(r, qs, o));
        }
        self.verify::<K>(q.unwrap(), x1, x2, c, o)
    }

    /// `pair_half_trace` of the repository arm for leaf `i`.
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    fn leaf<const K: bool, const ALL: bool>(
        &self,
        r: FastPoint,
        i: usize,
        u: u64,
        ui: u64,
        qs: QSrc,
        c: &mut Counts,
        o: &mut Ops,
        ws: &mut Scratch,
        all: &mut Vec<[u32; 3]>,
    ) -> Option<(usize, usize)> {
        let (d, n) = (self.d, self.n as usize);
        let gf = self.gf;
        let cc = gf.mul(self.b, gf.sqr(ui));
        c.field_muls += 2;
        let hc = self.half_trace.apply(cc);
        let tr_c = gf.sqr(hc) ^ hc ^ cc;
        c.field_muls += 1;
        debug_assert!(tr_c <= 1);
        let shift = 2 * d - 1;
        let tbit = n - shift;
        ws.cols.clear();
        for j in 0..d {
            let top = gf.mul(u, self.ell[j]) >> shift;
            ws.cols.push(top | (((self.trace_mask >> j) & 1) << tbit));
        }
        ws.cols.push(u >> shift);
        c.field_muls += d as u64 + 1;
        let rhs = (gf.mul(u, hc) >> shift) | (tr_c << tbit);
        if K {
            o.s[ST_BUILD][OP_MUL] += d as u64 + 2;
            o.s[ST_BUILD][OP_SQR] += 2;
            o.s[ST_BUILD][OP_TLOOK] += self.bytes;
            o.s[ST_BUILD][OP_WOP] += 3 * (d as u64 + 1);
        }
        let part = solve_f2::<K>(
            &ws.cols,
            rhs,
            &mut c.gauss_xors,
            &mut o.s[ST_SOLVE][OP_WOP],
            &mut ws.kernel,
        )?;
        c.consistent += 1;
        let dim = ws.kernel.len();
        if dim > self.kernel_cap {
            c.truncated = true;
        }
        let dim = dim.min(self.kernel_cap);
        let dmask = (1u64 << d) - 1;
        let mut q: Option<FastPoint> = None;
        for sel in 0u64..(1u64 << dim) {
            let mut comb = part;
            for k in 0..dim {
                if (sel >> k) & 1 == 1 {
                    comb ^= ws.kernel[k];
                }
            }
            let s = comb & dmask;
            let eps = (comb >> d) & 1;
            let ls = s ^ self.half_trace.apply(s);
            let p = gf.mul(u, ls ^ hc ^ eps);
            c.field_muls += 1;
            if K {
                o.s[ST_SPLIT][OP_MUL] += 1;
                o.s[ST_SPLIT][OP_TLOOK] += self.bytes;
                o.s[ST_SPLIT][OP_WOP] += dim as u64 + 4;
            }
            debug_assert!(p >> shift == 0);
            if let Some((j, k)) = self.split_root_and_verify::<K>(r, qs, &mut q, s, p, c, o) {
                if ALL {
                    let mut t = [i as u32, j as u32, k as u32];
                    t.sort_unstable();
                    all.push(t);
                } else {
                    return Some((j, k));
                }
            }
        }
        None
    }

    /// One leaf by a single addition and a single inversion (singletons and
    /// degenerate pairs of the paired t stages).
    #[inline(never)]
    #[allow(clippy::too_many_arguments)]
    fn single_leaf<const K: bool, const ALL: bool, const TONLY: bool>(
        &self,
        r: FastPoint,
        i: usize,
        c: &mut Counts,
        o: &mut Ops,
        ws: &mut Scratch,
        all: &mut Vec<[u32; 3]>,
        sink: &mut u64,
    ) -> Option<(usize, usize)> {
        let q = self.fast.add(r, self.negated[i]);
        c.trials += 1;
        if K {
            o.fallback_leaves += 1;
            o.s[ST_T][OP_INV] += 1;
            o.s[ST_T][OP_MUL] += 2;
            o.s[ST_T][OP_SQR] += 1;
        }
        if q.infinity || q.x == 0 {
            return None;
        }
        let ui = self.gf.inv(q.x);
        if K {
            o.s[ST_T][OP_INV] += 1;
        }
        if TONLY {
            *sink ^= q.x ^ ui;
            return None;
        }
        self.leaf::<K, ALL>(r, i, q.x, ui, QSrc::Full(q), c, o, ws, all)
    }

    /// The repository's t stage and scan order.
    pub fn decide_base<const K: bool, const ALL: bool, const TONLY: bool>(
        &self,
        r: FastPoint,
        c: &mut Counts,
        o: &mut Ops,
        all: &mut Vec<[u32; 3]>,
        sink: &mut u64,
    ) -> Option<[usize; 3]> {
        let mut ws = Scratch::default();
        let mut batch = BatchScratch::default();
        let mut qs: Vec<FastPoint> = Vec::with_capacity(BLOCK);
        let mut uinv: Vec<u64> = Vec::with_capacity(BLOCK);
        let mut tmp: Vec<u64> = Vec::new();
        let mut lo = 0;
        while lo < self.points.len() {
            let hi = (lo + BLOCK).min(self.points.len());
            qs.clear();
            self.fast
                .add_many(r, &self.negated[lo..hi], &mut qs, &mut batch);
            uinv.clear();
            uinv.extend(
                qs.iter()
                    .map(|q| if q.infinity || q.x == 0 { 1 } else { q.x }),
            );
            self.gf.batch_inv(&mut uinv, &mut tmp);
            c.field_invs += 1;
            c.field_muls += 3 * (hi - lo) as u64;
            if K {
                let m = (hi - lo) as u64;
                let deg = self.negated[lo..hi].iter().filter(|q| q.x == r.x).count() as u64;
                let nz = m - deg;
                // add_many: batch_inv (3 per non-zero + 9, one inversion),
                // slope, abscissa, ordinate; then batch_inv of the abscissae.
                o.s[ST_T][OP_MUL] += (3 * nz + 9) + 2 * nz + (3 * m + 9);
                o.s[ST_T][OP_SQR] += nz;
                o.s[ST_T][OP_INV] += 2 + deg;
            }
            if TONLY {
                for (q, &ui) in qs.iter().zip(&uinv) {
                    *sink ^= q.x ^ ui;
                }
                lo = hi;
                continue;
            }
            for (off, &q) in qs.iter().enumerate() {
                c.trials += 1;
                if q.infinity || q.x == 0 {
                    continue;
                }
                if let Some((j, k)) = self.leaf::<K, ALL>(
                    r,
                    lo + off,
                    q.x,
                    uinv[off],
                    QSrc::Full(q),
                    c,
                    o,
                    &mut ws,
                    all,
                ) {
                    return Some([lo + off, j, k]);
                }
            }
            lo = hi;
        }
        None
    }

    /// # Safety
    ///
    /// The field must fold (which implies `pclmulqdq`).
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "pclmulqdq")]
    #[inline(never)]
    unsafe fn pm_t_clmul(&self, gf: &Gf2, r: FastPoint, p0: usize, np: usize, w: &mut PmBuf, tmp: &mut Vec<u64>) {
        // SAFETY: the caller's contract.
        unsafe { std::hint::assert_unchecked(gf.folds()) };
        self.pm_t_inline(gf, r, p0, np, w, tmp);
    }

    /// The paired t stage for pairs `p0 .. p0 + np`: `t`, `1/t` for both
    /// members, the first member's slope and `1/e`.  A pair with `e = 0` or
    /// `g = 0` comes back with `v = 0` and is redone point by point.
    #[inline(always)]
    fn pm_t_inline(&self, gf: &Gf2, r: FastPoint, p0: usize, np: usize, w: &mut PmBuf, tmp: &mut Vec<u64>) {
        let (xr, yr, a, sb) = (r.x, r.y, self.fast.a, self.sb);
        let xs = &self.pair_x[p0..p0 + np];
        let ya = &self.pair_ya[p0..p0 + np];
        w.v.clear();
        w.g.clear();
        w.pp.clear();
        w.v.resize(np, 0);
        w.g.resize(np, 0);
        w.pp.resize(np, 0);
        for (((&x, v), g), pp) in xs
            .iter()
            .zip(w.v.iter_mut())
            .zip(w.g.iter_mut())
            .zip(w.pp.iter_mut())
        {
            let pi = gf.mul(x, xr);
            let gg = pi ^ sb;
            *pp = pi;
            *g = gg;
            *v = gf.mul(xr ^ x, gg);
        }
        gf.batch_inv(&mut w.v, tmp);
        w.t.clear();
        w.ti.clear();
        w.lam.clear();
        w.ie.clear();
        w.t.resize(2 * np, 0);
        w.ti.resize(2 * np, 0);
        w.lam.resize(np, 0);
        w.ie.resize(np, 0);
        let (tc, _) = w.t.as_chunks_mut::<2>();
        let (tic, _) = w.ti.as_chunks_mut::<2>();
        for k in 0..np {
            let vinv = w.v[k];
            let x = xs[k];
            let e = xr ^ x;
            let ie = gf.mul(vinv, w.g[k]);
            let lam = gf.mul(yr ^ ya[k], ie);
            let t1 = gf.sqr(lam) ^ lam ^ e ^ a;
            let t2 = t1 ^ gf.mul(w.pp[k], gf.sqr(ie));
            let m = gf.mul(gf.sqr(e), vinv);
            let kk = gf.sqr(m);
            tc[k] = [t1, t2];
            tic[k] = [gf.mul(t2, kk), gf.mul(t1, kk)];
            w.lam[k] = lam;
            w.ie[k] = ie;
        }
    }

    /// ±P pairing with one shared inversion per pair; same scan order.
    pub fn decide_pm<const K: bool, const ALL: bool, const TONLY: bool>(
        &self,
        r: FastPoint,
        c: &mut Counts,
        o: &mut Ops,
        all: &mut Vec<[u32; 3]>,
        sink: &mut u64,
    ) -> Option<[usize; 3]> {
        let mut ws = Scratch::default();
        let mut w = PmBuf::default();
        let mut tmp: Vec<u64> = Vec::new();
        let len = self.points.len();
        let folds = self.gf.folds();
        let mut i = 0;
        while i < len {
            if r.infinity || !self.pair_start[i] {
                if let Some((j, k)) =
                    self.single_leaf::<K, ALL, TONLY>(r, i, c, o, &mut ws, all, sink)
                {
                    return Some([i, j, k]);
                }
                i += 1;
                continue;
            }
            let p0 = self.pair_of[i] as usize;
            let (mut j, mut np) = (i, 0usize);
            while np < BLOCK / 2 && j < len && self.pair_start[j] {
                j += 2;
                np += 1;
            }
            #[cfg(target_arch = "x86_64")]
            if folds {
                // SAFETY: the field folds, as just checked.
                unsafe { self.pm_t_clmul(self.gf, r, p0, np, &mut w, &mut tmp) };
            } else {
                self.pm_t_inline(self.gf, r, p0, np, &mut w, &mut tmp);
            }
            #[cfg(not(target_arch = "x86_64"))]
            {
                let _ = folds;
                self.pm_t_inline(self.gf, r, p0, np, &mut w, &mut tmp);
            }
            c.field_invs += 1;
            c.field_muls += 11 * np as u64;
            if K {
                let nz = w.v.iter().filter(|&&v| v != 0).count() as u64;
                let m = np as u64;
                // Π and e·g for every pair; batch_inv; eight products and
                // four squarings per pair that has an inverse.
                o.s[ST_T][OP_MUL] += 2 * m + (3 * nz + 9) + 6 * nz;
                o.s[ST_T][OP_SQR] += 4 * nz;
                o.s[ST_T][OP_INV] += 1;
            }
            for k in 0..np {
                let ia = i + 2 * k;
                if w.v[k] == 0 {
                    for ib in [ia, ia + 1] {
                        if let Some((j, kk)) =
                            self.single_leaf::<K, ALL, TONLY>(r, ib, c, o, &mut ws, all, sink)
                        {
                            return Some([ib, j, kk]);
                        }
                    }
                    continue;
                }
                let (t1, t2) = (w.t[2 * k], w.t[2 * k + 1]);
                let (u1, u2) = (w.ti[2 * k], w.ti[2 * k + 1]);
                c.trials += 1;
                if TONLY {
                    *sink ^= t1 ^ t2 ^ u1 ^ u2;
                    continue;
                }
                let (lam, ie) = (w.lam[k], w.ie[k]);
                if let Some((j, kk)) = self.leaf::<K, ALL>(
                    r,
                    ia,
                    t1,
                    u1,
                    QSrc::Lazy { x: t1, lam },
                    c,
                    o,
                    &mut ws,
                    all,
                ) {
                    return Some([ia, j, kk]);
                }
                c.trials += 1;
                let x1 = self.pair_x[p0 + k];
                if let Some((j, kk)) = self.leaf::<K, ALL>(
                    r,
                    ia + 1,
                    t2,
                    u2,
                    QSrc::Lazy2 { x: t2, lam, x1, ie },
                    c,
                    o,
                    &mut ws,
                    all,
                ) {
                    return Some([ia + 1, j, kk]);
                }
            }
            i = j;
        }
        None
    }

    /// The library's lazy batched addition, then `batch_inv` on the abscissae.
    pub fn decide_lz<const ALL: bool, const TONLY: bool>(
        &self,
        r: FastPoint,
        c: &mut Counts,
        all: &mut Vec<[u32; 3]>,
        sink: &mut u64,
    ) -> Option<[usize; 3]> {
        let mut o = Ops::default();
        let mut ws = Scratch::default();
        let mut batch = BatchScratch::default();
        let mut qs: Vec<FastPoint> = Vec::with_capacity(BLOCK);
        let mut lambdas: Vec<u64> = Vec::with_capacity(BLOCK);
        let mut uinv: Vec<u64> = Vec::with_capacity(BLOCK);
        let mut tmp: Vec<u64> = Vec::new();
        let mut lo = 0;
        while lo < self.points.len() {
            let hi = (lo + BLOCK).min(self.points.len());
            if let Some(w) = self.lz_block::<ALL, TONLY>(
                r, lo, hi, c, &mut o, &mut ws, &mut batch, &mut qs, &mut lambdas, &mut uinv,
                &mut tmp, all, sink,
            ) {
                return Some(w);
            }
            lo = hi;
        }
        None
    }

    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    fn lz_block<const ALL: bool, const TONLY: bool>(
        &self,
        r: FastPoint,
        lo: usize,
        hi: usize,
        c: &mut Counts,
        o: &mut Ops,
        ws: &mut Scratch,
        batch: &mut BatchScratch,
        qs: &mut Vec<FastPoint>,
        lambdas: &mut Vec<u64>,
        uinv: &mut Vec<u64>,
        tmp: &mut Vec<u64>,
        all: &mut Vec<[u32; 3]>,
        sink: &mut u64,
    ) -> Option<[usize; 3]> {
        qs.clear();
        lambdas.clear();
        self.fast
            .add_many_lazy(r, &self.negated[lo..hi], qs, lambdas, batch);
        uinv.clear();
        uinv.extend(
            qs.iter()
                .map(|q| if q.infinity || q.x == 0 { 1 } else { q.x }),
        );
        self.gf.batch_inv(uinv, tmp);
        c.field_invs += 1;
        c.field_muls += 3 * (hi - lo) as u64;
        if TONLY {
            for (q, &ui) in qs.iter().zip(uinv.iter()) {
                *sink ^= q.x ^ ui;
            }
            return None;
        }
        for off in 0..qs.len() {
            let q = qs[off];
            c.trials += 1;
            if q.infinity || q.x == 0 {
                continue;
            }
            if let Some((j, k)) = self.leaf::<false, ALL>(
                r,
                lo + off,
                q.x,
                uinv[off],
                QSrc::Lib { sum: q, lam: lambdas[off] },
                c,
                o,
                ws,
                all,
            ) {
                return Some([lo + off, j, k]);
            }
        }
        None
    }

    /// The library's existing ±P pairing (`add_many_lazy_soa_paired`), then
    /// `batch_inv` on the abscissae.  Where the kernel declines a block, the
    /// block goes through `add_many_lazy`.
    pub fn decide_lp<const ALL: bool, const TONLY: bool>(
        &self,
        r: FastPoint,
        c: &mut Counts,
        o: &mut Ops,
        all: &mut Vec<[u32; 3]>,
        sink: &mut u64,
    ) -> Option<[usize; 3]> {
        let mut ws = Scratch::default();
        let mut batch = BatchScratch::default();
        let mut qs: Vec<FastPoint> = Vec::with_capacity(BLOCK);
        let mut lambdas: Vec<u64> = Vec::with_capacity(BLOCK);
        let mut x3: Vec<u64> = Vec::with_capacity(BLOCK);
        let mut uinv: Vec<u64> = Vec::with_capacity(BLOCK);
        let mut tmp: Vec<u64> = Vec::new();
        let len = self.points.len();
        let mut i = 0;
        while i < len {
            if r.infinity || !self.pair_start[i] {
                if let Some(w) = self.lz_block::<ALL, TONLY>(
                    r, i, i + 1, c, o, &mut ws, &mut batch, &mut qs, &mut lambdas, &mut uinv,
                    &mut tmp, all, sink,
                ) {
                    return Some(w);
                }
                i += 1;
                continue;
            }
            let p0 = self.pair_of[i] as usize;
            let (mut j, mut np) = (i, 0usize);
            while np < BLOCK / 2 && j < len && self.pair_start[j] {
                j += 2;
                np += 1;
            }
            x3.clear();
            lambdas.clear();
            let ran = self.fast.add_many_lazy_soa_paired(
                r,
                &self.pair_x[p0..p0 + np],
                &self.pair_ya[p0..p0 + np],
                &self.pair_yb[p0..p0 + np],
                &mut x3,
                &mut lambdas,
                &mut batch,
                2 * np as u64,
            );
            if !ran {
                o.lp_declined_blocks += 1;
                if let Some(w) = self.lz_block::<ALL, TONLY>(
                    r, i, j, c, o, &mut ws, &mut batch, &mut qs, &mut lambdas, &mut uinv,
                    &mut tmp, all, sink,
                ) {
                    return Some(w);
                }
                i = j;
                continue;
            }
            o.lp_kernel_blocks += 1;
            uinv.clear();
            uinv.extend(x3.iter().map(|&x| if x == 0 { 1 } else { x }));
            self.gf.batch_inv(&mut uinv, &mut tmp);
            c.field_invs += 1;
            c.field_muls += 6 * np as u64;
            if TONLY {
                for (&x, &ui) in x3.iter().zip(uinv.iter()) {
                    *sink ^= x ^ ui;
                }
                i = j;
                continue;
            }
            for off in 0..2 * np {
                let x = x3[off];
                c.trials += 1;
                if x == 0 {
                    continue;
                }
                if let Some((jj, k)) = self.leaf::<false, ALL>(
                    r,
                    i + off,
                    x,
                    uinv[off],
                    QSrc::Lib { sum: FastPoint::affine(x, 0), lam: lambdas[off] },
                    c,
                    o,
                    &mut ws,
                    all,
                ) {
                    return Some([i + off, jj, k]);
                }
            }
            i = j;
        }
        None
    }

    /// Modelled t-stage operations of `decide_lz` / `decide_lp` on a target
    /// with no degenerate sum: Montgomery's trick at 3 products per element
    /// plus 9 and one inversion per batch, one slope product and one squaring
    /// per sum.  The library kernels are not instrumented; this is a model.
    pub fn model_t(&self, paired: bool) -> [u64; 5] {
        let mut t = [0u64; 5];
        let len = self.points.len();
        let batchinv = |t: &mut [u64; 5], m: u64| {
            t[OP_MUL] += 3 * m + 9;
            t[OP_INV] += 1;
        };
        if !paired {
            let mut lo = 0;
            while lo < len {
                let m = ((lo + BLOCK).min(len) - lo) as u64;
                batchinv(&mut t, m);
                t[OP_MUL] += m;
                t[OP_SQR] += m;
                batchinv(&mut t, m);
                lo += BLOCK;
            }
            return t;
        }
        let mut i = 0;
        while i < len {
            if !self.pair_start[i] {
                batchinv(&mut t, 1);
                t[OP_MUL] += 1;
                t[OP_SQR] += 1;
                batchinv(&mut t, 1);
                i += 1;
                continue;
            }
            let (mut j, mut np) = (i, 0u64);
            while np < (BLOCK / 2) as u64 && j < len && self.pair_start[j] {
                j += 2;
                np += 1;
            }
            batchinv(&mut t, np);
            t[OP_MUL] += 2 * np;
            t[OP_SQR] += 2 * np;
            batchinv(&mut t, 2 * np);
            i = j;
        }
        t
    }

    /// Every sorted index triple `{i, j, k}` with `P_i + P_j + P_k = R` that
    /// the scan's conventions admit (`R − P_i` finite with non-zero abscissa;
    /// the pair's abscissae non-zero), by enumerating all `(i, j)` and looking
    /// the third point up.  Uses no linear algebra.
    pub fn exhaustive(&self, r: FastPoint) -> Vec<[u32; 3]> {
        let mut out = Vec::new();
        let mut batch = BatchScratch::default();
        let mut rest: Vec<FastPoint> = Vec::new();
        let lim = 1u64 << self.d;
        for i in 0..self.points.len() {
            let q = self.fast.add(r, self.negated[i]);
            if q.infinity || q.x == 0 {
                continue;
            }
            rest.clear();
            self.fast.add_many(q, &self.negated, &mut rest, &mut batch);
            for (j, s) in rest.iter().enumerate() {
                if self.points[j].x == 0 || s.infinity || s.x == 0 || s.x >= lim {
                    continue;
                }
                let k = self.index_of[2 * s.x as usize + sign_bit(*s)];
                if k == usize::MAX || k < j || self.points[k] != *s {
                    continue;
                }
                let mut t = [i as u32, j as u32, k as u32];
                t.sort_unstable();
                out.push(t);
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }
}
