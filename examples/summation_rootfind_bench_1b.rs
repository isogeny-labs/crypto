//! Phase 1b counted-operation benchmark for the binary Semaev `S₄`
//! quartic-in-subspace root find (`semaev_decomp::decompose`'s hot path).
//!
//! Protocol: workspace `docs/notes/summation-rootfind-20261124/PROTOCOL.md`
//! (Phase 1, frozen 2026-11-24T01:52Z) and `PROTOCOL_1B.md` (amendment,
//! frozen 2026-11-25T05:40Z). Class: component counted-op diagnostic,
//! `[pcc:unverified]`, L0. NOT a leaderboard / benchmarks row, NOT a rho
//! comparison.
//!
//! Phase 1b adds, on the Phase 1 instrument:
//!   * arm `closed` (B): the Dickson / half-trace closed form, a port of
//!     `binary_radical_root::RadicalQuartic::solve` to every width, falling
//!     back to the subspace gcd on its fallback classes (charged in full);
//!   * a three-word field (`Gf2W`, n = 131, the ECC2K-130 field);
//!   * planting that works for even n (Phase 1's lifter returned no point for
//!     even n > 20, so those cells were all random targets);
//!   * a squaring-kernel study (clmul / spread / pdep / table, one shared
//!     reduction, verified bit-exact) and the counted Itoh–Tsujii census.
//!
//! The Phase 1 arms (`scan`, `baseline`, `frob`), quartic, seeds and planting
//! choice are reproduced exactly so the Phase 1 op census replicates.
//!
//! ```text
//! SRB_OUT=/path/to/data cargo run --release --example summation_rootfind_bench_1b
//! ```

use std::cell::Cell;
use std::collections::BTreeSet;
use std::io::Write;

use crypto_lib::binary_ecc::f2m::IrreduciblePoly;
use crypto_lib::cryptanalysis::certicom131_symlp::gf2w::{self, Gf2W};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::semaev_decomp::{Gf2, Gf2_128};
use num_bigint::BigUint;

// ---------------------------------------------------------------------------
// Field words.
// ---------------------------------------------------------------------------

#[derive(Default, Clone, Copy, Debug)]
struct Ops {
    mul: u64,
    sqr: u64,
    inv: u64,
    lin: u64,
}

trait Word: Copy + PartialEq + Eq + Ord + std::hash::Hash + Default + std::fmt::Debug {
    fn xor(self, o: Self) -> Self;
    fn one() -> Self;
    fn zero() -> Self;
    fn from_small(x: u64) -> Self;
    /// Monomial `z^i`.
    fn mono(i: u32) -> Self;
    fn byte(self, j: usize) -> u8;
    fn parity_and(self, m: Self) -> u32;
    /// `x < 2^l` (x lies in V).
    fn below(self, l: u32) -> bool;
    fn bit(self, i: u32) -> bool;
    /// A uniform element of `F_{2^n}` from the seed stream. For u64/u128 this
    /// consumes exactly the two draws Phase 1 consumed.
    fn rand(seed: &mut u64, n: u32) -> Self;
    fn is_zero(self) -> bool {
        self == Self::zero()
    }
}

fn mask128(n: u32) -> u128 {
    if n >= 128 {
        u128::MAX
    } else {
        (1u128 << n) - 1
    }
}

impl Word for u64 {
    fn xor(self, o: Self) -> Self {
        self ^ o
    }
    fn one() -> Self {
        1
    }
    fn zero() -> Self {
        0
    }
    fn from_small(x: u64) -> Self {
        x
    }
    fn mono(i: u32) -> Self {
        1u64 << i
    }
    fn byte(self, j: usize) -> u8 {
        if j >= 8 {
            0
        } else {
            (self >> (8 * j)) as u8
        }
    }
    fn parity_and(self, m: Self) -> u32 {
        (self & m).count_ones() & 1
    }
    fn below(self, l: u32) -> bool {
        self >> l == 0
    }
    fn bit(self, i: u32) -> bool {
        i < 64 && (self >> i) & 1 == 1
    }
    fn rand(seed: &mut u64, n: u32) -> Self {
        let lo = splitmix64(seed) as u128;
        let hi = splitmix64(seed) as u128;
        (((hi << 64) | lo) & mask128(n)) as u64
    }
}

impl Word for u128 {
    fn xor(self, o: Self) -> Self {
        self ^ o
    }
    fn one() -> Self {
        1
    }
    fn zero() -> Self {
        0
    }
    fn from_small(x: u64) -> Self {
        x as u128
    }
    fn mono(i: u32) -> Self {
        1u128 << i
    }
    fn byte(self, j: usize) -> u8 {
        if j >= 16 {
            0
        } else {
            (self >> (8 * j)) as u8
        }
    }
    fn parity_and(self, m: Self) -> u32 {
        (self & m).count_ones() & 1
    }
    fn below(self, l: u32) -> bool {
        self >> l == 0
    }
    fn bit(self, i: u32) -> bool {
        i < 128 && (self >> i) & 1 == 1
    }
    fn rand(seed: &mut u64, n: u32) -> Self {
        let lo = splitmix64(seed) as u128;
        let hi = splitmix64(seed) as u128;
        ((hi << 64) | lo) & mask128(n)
    }
}

/// Three-word element (`Gf2W`'s `[u64; 3]`, low word first).
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct W192([u64; 3]);

impl Word for W192 {
    fn xor(self, o: Self) -> Self {
        W192([self.0[0] ^ o.0[0], self.0[1] ^ o.0[1], self.0[2] ^ o.0[2]])
    }
    fn one() -> Self {
        W192([1, 0, 0])
    }
    fn zero() -> Self {
        W192([0; 3])
    }
    fn from_small(x: u64) -> Self {
        W192([x, 0, 0])
    }
    fn mono(i: u32) -> Self {
        let mut w = [0u64; 3];
        w[(i / 64) as usize] = 1u64 << (i % 64);
        W192(w)
    }
    fn byte(self, j: usize) -> u8 {
        if j >= 24 {
            0
        } else {
            (self.0[j / 8] >> (8 * (j % 8))) as u8
        }
    }
    fn parity_and(self, m: Self) -> u32 {
        ((self.0[0] & m.0[0]).count_ones()
            ^ (self.0[1] & m.0[1]).count_ones()
            ^ (self.0[2] & m.0[2]).count_ones())
            & 1
    }
    fn below(self, l: u32) -> bool {
        debug_assert!(l < 64);
        self.0[0] >> l == 0 && self.0[1] == 0 && self.0[2] == 0
    }
    fn bit(self, i: u32) -> bool {
        i < 192 && (self.0[(i / 64) as usize] >> (i % 64)) & 1 == 1
    }
    fn rand(seed: &mut u64, n: u32) -> Self {
        let mut w = [splitmix64(seed), splitmix64(seed), splitmix64(seed)];
        for i in 0..3 {
            let lo = 64 * i as u32;
            if n <= lo {
                w[i] = 0;
            } else if n - lo < 64 {
                w[i] &= (1u64 << (n - lo)) - 1;
            }
        }
        W192(w)
    }
}

// ---------------------------------------------------------------------------
// Raw (uncounted) lab fields, one trait over the three widths.
// ---------------------------------------------------------------------------

trait Raw {
    type E: Word;
    fn mul(&self, a: Self::E, b: Self::E) -> Self::E;
    fn sqr(&self, a: Self::E) -> Self::E;
    fn inv(&self, a: Self::E) -> Self::E;
    fn sqr_k(&self, a: Self::E, k: u32) -> Self::E;
    fn n(&self) -> u32;
}

impl Raw for Gf2 {
    type E = u64;
    #[inline]
    fn mul(&self, a: u64, b: u64) -> u64 {
        Gf2::mul(self, a, b)
    }
    #[inline]
    fn sqr(&self, a: u64) -> u64 {
        Gf2::sqr(self, a)
    }
    #[inline]
    fn inv(&self, a: u64) -> u64 {
        Gf2::inv(self, a)
    }
    #[inline]
    fn sqr_k(&self, a: u64, k: u32) -> u64 {
        Gf2::sqr_k(self, a, k)
    }
    fn n(&self) -> u32 {
        self.n
    }
}

impl Raw for Gf2_128 {
    type E = u128;
    #[inline]
    fn mul(&self, a: u128, b: u128) -> u128 {
        Gf2_128::mul(self, a, b)
    }
    #[inline]
    fn sqr(&self, a: u128) -> u128 {
        Gf2_128::sqr(self, a)
    }
    #[inline]
    fn inv(&self, a: u128) -> u128 {
        Gf2_128::inv(self, a)
    }
    #[inline]
    fn sqr_k(&self, a: u128, k: u32) -> u128 {
        Gf2_128::sqr_k(self, a, k)
    }
    fn n(&self) -> u32 {
        self.n
    }
}

impl Raw for Gf2W {
    type E = W192;
    #[inline]
    fn mul(&self, a: W192, b: W192) -> W192 {
        W192(Gf2W::mul(self, &a.0, &b.0))
    }
    #[inline]
    fn sqr(&self, a: W192) -> W192 {
        W192(Gf2W::sqr(self, &a.0))
    }
    #[inline]
    fn inv(&self, a: W192) -> W192 {
        W192(Gf2W::inv(self, &a.0))
    }
    #[inline]
    fn sqr_k(&self, a: W192, k: u32) -> W192 {
        W192(Gf2W::sqr_k(self, &a.0, k))
    }
    fn n(&self) -> u32 {
        self.n
    }
}

/// An F₂-linear map applied a byte at a time through tables.
struct LinTab<E: Word> {
    tab: Vec<[E; 256]>,
}

impl<E: Word> LinTab<E> {
    fn from_images(images: &[E]) -> Self {
        let bytes = images.len().div_ceil(8);
        let mut tab = vec![[E::zero(); 256]; bytes];
        for (j, t) in tab.iter_mut().enumerate() {
            for v in 1..256usize {
                let low = v.trailing_zeros() as usize;
                let i = 8 * j + low;
                let prev = t[v & (v - 1)];
                t[v] = if i < images.len() {
                    prev.xor(images[i])
                } else {
                    prev
                };
            }
        }
        Self { tab }
    }
    #[inline]
    fn apply(&self, x: E) -> E {
        let mut r = E::zero();
        for (j, t) in self.tab.iter().enumerate() {
            let b = x.byte(j);
            if b != 0 {
                r = r.xor(t[b as usize]);
            }
        }
        r
    }
}

/// Linear-algebra extras of a field: √, trace, half-trace (odd n), and a
/// general Artin–Schreier solver (any n) used only for uncounted planting.
struct Extras<E: Word> {
    sqrt: LinTab<E>,
    trace_mask: E,
    half_trace: Option<LinTab<E>>,
    /// Echelon rows (pivot bit, vector, preimage) of `z ↦ z² + z` on its image.
    as_rows: Vec<(u32, E, E)>,
    /// Artin–Schreier solver as one byte table, any n (Phase 1c): the
    /// half-trace for odd n, the linear inverse of `z ↦ z² + z` on the trace-0
    /// hyperplane for even n (as the lab's `ArtinSchreier`).
    as_tab: Option<LinTab<E>>,
}

fn extras<R: Raw>(gf: &R) -> Extras<R::E> {
    let n = gf.n();
    let basis: Vec<R::E> = (0..n).map(R::E::mono).collect();
    let sqrt: Vec<R::E> = basis.iter().map(|&b| gf.sqr_k(b, n - 1)).collect();
    let tr = |x: R::E| {
        let (mut acc, mut p) = (R::E::zero(), x);
        for _ in 0..n {
            acc = acc.xor(p);
            p = gf.sqr(p);
        }
        acc
    };
    let mut trace_mask = R::E::zero();
    for (i, &b) in basis.iter().enumerate() {
        let t = tr(b);
        assert!(t == R::E::zero() || t == R::E::one(), "trace not in F2");
        if t == R::E::one() {
            trace_mask = trace_mask.xor(R::E::mono(i as u32));
        }
    }
    let half_trace = (n % 2 == 1).then(|| {
        let ht: Vec<R::E> = basis
            .iter()
            .map(|&b| {
                let (mut acc, mut p) = (R::E::zero(), b);
                for _ in 0..=(n - 1) / 2 {
                    acc = acc.xor(p);
                    p = gf.sqr_k(p, 2);
                }
                acc
            })
            .collect();
        LinTab::from_images(&ht)
    });
    // Gaussian elimination over F₂ on (φ(zⁱ), zⁱ), φ(z) = z² + z.
    let mut rows: Vec<(u32, R::E, R::E)> = Vec::new();
    for i in 0..n {
        let zi = R::E::mono(i);
        let mut v = gf.sqr(zi).xor(zi);
        let mut pre = zi;
        for &(p, rv, rp) in &rows {
            if v.bit(p) {
                v = v.xor(rv);
                pre = pre.xor(rp);
            }
        }
        if let Some(p) = (0..n).rev().find(|&b| v.bit(b)) {
            // keep rows fully reduced at their pivots
            for r in rows.iter_mut() {
                if r.1.bit(p) {
                    r.1 = r.1.xor(v);
                    r.2 = r.2.xor(pre);
                }
            }
            rows.push((p, v, pre));
        }
    }
    assert_eq!(rows.len() as u32, n - 1, "z²+z has rank n−1");
    let mut ex = Extras {
        sqrt: LinTab::from_images(&sqrt),
        trace_mask,
        half_trace,
        as_rows: rows,
        as_tab: None,
    };
    if n.is_multiple_of(2) {
        // a fixed trace-1 basis element t1; images M(eᵢ) solve eᵢ (+ t1)
        let t1 = (0..n)
            .map(R::E::mono)
            .find(|b| b.parity_and(ex.trace_mask) == 1)
            .expect("some basis element has trace 1");
        let imgs: Vec<R::E> = (0..n)
            .map(|i| {
                let e = R::E::mono(i);
                let c = if e.parity_and(ex.trace_mask) == 1 {
                    e.xor(t1)
                } else {
                    e
                };
                ex.as_solve_any(c).expect("trace-0 input")
            })
            .collect();
        ex.as_tab = Some(LinTab::from_images(&imgs));
    }
    ex
}

impl<E: Word> Extras<E> {
    /// A root of `u² + u = c`, or `None` when `Tr(c) = 1`.
    fn as_solve_any(&self, c: E) -> Option<E> {
        let mut v = c;
        let mut pre = E::zero();
        for &(p, rv, rp) in &self.as_rows {
            if v.bit(p) {
                v = v.xor(rv);
                pre = pre.xor(rp);
            }
        }
        v.is_zero().then_some(pre)
    }
}

// ---------------------------------------------------------------------------
// Counting instrument.
// ---------------------------------------------------------------------------

trait CField {
    type E: Word;
    fn mul(&self, a: Self::E, b: Self::E) -> Self::E;
    fn sqr(&self, a: Self::E) -> Self::E;
    fn inv(&self, a: Self::E) -> Self::E;
    fn sqrt(&self, a: Self::E) -> Self::E;
    fn trace(&self, a: Self::E) -> u32;
    fn half_trace(&self, a: Self::E) -> Self::E;
    /// A root of `z² + z = a` for trace-0 `a` (one table application), any n.
    fn as_solve(&self, a: Self::E) -> Self::E;
    fn ops(&self) -> Ops;
    fn reset(&self);
    #[inline]
    fn add(&self, a: Self::E, b: Self::E) -> Self::E {
        a.xor(b)
    }
}

struct Counted<'a, R: Raw> {
    gf: &'a R,
    ex: &'a Extras<R::E>,
    ops: Cell<Ops>,
}

impl<'a, R: Raw> Counted<'a, R> {
    fn new(gf: &'a R, ex: &'a Extras<R::E>) -> Self {
        Self {
            gf,
            ex,
            ops: Cell::new(Ops::default()),
        }
    }
    #[inline]
    fn bump(&self, f: impl FnOnce(&mut Ops)) {
        let mut o = self.ops.get();
        f(&mut o);
        self.ops.set(o);
    }
}

impl<R: Raw> CField for Counted<'_, R> {
    type E = R::E;
    #[inline]
    fn mul(&self, a: R::E, b: R::E) -> R::E {
        self.bump(|o| o.mul += 1);
        self.gf.mul(a, b)
    }
    #[inline]
    fn sqr(&self, a: R::E) -> R::E {
        self.bump(|o| o.sqr += 1);
        self.gf.sqr(a)
    }
    #[inline]
    fn inv(&self, a: R::E) -> R::E {
        self.bump(|o| o.inv += 1);
        self.gf.inv(a)
    }
    #[inline]
    fn sqrt(&self, a: R::E) -> R::E {
        self.bump(|o| o.lin += 1);
        self.ex.sqrt.apply(a)
    }
    #[inline]
    fn trace(&self, a: R::E) -> u32 {
        self.bump(|o| o.lin += 1);
        a.parity_and(self.ex.trace_mask)
    }
    #[inline]
    fn half_trace(&self, a: R::E) -> R::E {
        self.bump(|o| o.lin += 1);
        self.ex
            .half_trace
            .as_ref()
            .expect("half-trace needs odd n")
            .apply(a)
    }
    #[inline]
    fn as_solve(&self, a: R::E) -> R::E {
        self.bump(|o| o.lin += 1);
        match (&self.ex.half_trace, &self.ex.as_tab) {
            (Some(ht), _) => ht.apply(a),
            (None, Some(m)) => m.apply(a),
            _ => unreachable!("an Artin–Schreier table exists for every n"),
        }
    }
    fn ops(&self) -> Ops {
        self.ops.get()
    }
    fn reset(&self) {
        self.ops.set(Ops::default());
    }
}

// ---------------------------------------------------------------------------
// Dense small polynomials (unchanged from Phase 1).
// ---------------------------------------------------------------------------

const MAXD: usize = 7;

#[derive(Clone)]
struct P<E: Word> {
    c: [E; MAXD],
}
impl<E: Word> P<E> {
    fn zero() -> Self {
        Self {
            c: [E::zero(); MAXD],
        }
    }
    fn deg(&self) -> Option<usize> {
        (0..MAXD).rev().find(|&i| !self.c[i].is_zero())
    }
}

fn rem_monic<F: CField>(mut r: P<F::E>, f: &P<F::E>, df: usize, cf: &F) -> P<F::E> {
    let mut d = r.deg();
    while let Some(dd) = d {
        if dd < df {
            break;
        }
        let lead = r.c[dd];
        let shift = dd - df;
        for j in 0..df {
            let t = cf.mul(f.c[j], lead);
            r.c[shift + j] = cf.add(r.c[shift + j], t);
        }
        r.c[dd] = F::E::zero();
        d = r.deg();
    }
    r
}

fn sqr_mod_monic<F: CField>(a: &P<F::E>, f: &P<F::E>, df: usize, cf: &F) -> P<F::E> {
    let mut s = P::<F::E>::zero();
    for i in 0..=3 {
        if !a.c[i].is_zero() {
            s.c[2 * i] = cf.sqr(a.c[i]);
        }
    }
    rem_monic(s, f, df, cf)
}

fn poly_gcd<F: CField>(mut a: P<F::E>, mut b: P<F::E>, cf: &F) -> P<F::E> {
    loop {
        let db = match b.deg() {
            None => return a,
            Some(d) => d,
        };
        if db == 0 {
            return P::zero();
        }
        let r = prem(&a, &b, db, cf);
        a = b;
        b = r;
    }
}

fn prem<F: CField>(a: &P<F::E>, b: &P<F::E>, db: usize, cf: &F) -> P<F::E> {
    let lb = b.c[db];
    let mut r = a.clone();
    let mut d = r.deg();
    while let Some(dd) = d {
        if dd < db {
            break;
        }
        let lead = r.c[dd];
        let shift = dd - db;
        for j in 0..MAXD {
            r.c[j] = cf.mul(lb, r.c[j]);
        }
        for j in 0..=db {
            let t = cf.mul(lead, b.c[j]);
            r.c[shift + j] = cf.add(r.c[shift + j], t);
        }
        r.c[dd] = F::E::zero();
        d = r.deg();
    }
    r
}

fn subspace_poly_raw<R: Raw>(l: u32, gf: &R) -> Vec<R::E> {
    let mut coeffs: Vec<R::E> = vec![R::E::one()];
    for i in 0..l {
        let beta = R::E::from_small(1u64 << i);
        let mut lb = R::E::zero();
        let mut bp = beta;
        for &aj in coeffs.iter() {
            lb = lb.xor(gf.mul(aj, bp));
            bp = gf.sqr(bp);
        }
        let old = coeffs.clone();
        let m = old.len();
        let mut nw = vec![R::E::zero(); m + 1];
        for j in 0..=m {
            let sq = if j >= 1 {
                gf.sqr(old[j - 1])
            } else {
                R::E::zero()
            };
            let lin = if j < m {
                gf.mul(lb, old[j])
            } else {
                R::E::zero()
            };
            nw[j] = sq.xor(lin);
        }
        coeffs = nw;
    }
    coeffs
}

fn quartic_coeffs<F: CField>(x1: F::E, x2: F::E, xr: F::E, cf: &F) -> [F::E; 5] {
    let s = cf.add(x1, x2);
    let p = cf.mul(x1, x2);
    let s2 = cf.sqr(s);
    let p2 = cf.sqr(p);
    let s4 = cf.sqr(s2);
    let p4 = cf.sqr(p2);
    let p3 = cf.mul(p2, p);
    let p_s2 = cf.mul(p, s2);
    let xr2 = cf.sqr(xr);
    let xr3 = cf.mul(xr2, xr);
    let xr4 = cf.sqr(xr2);
    let p2_xr2 = cf.mul(p2, xr2);

    let c0 = cf.add(cf.add(cf.add(xr4, s4), cf.mul(p4, xr4)), p2_xr2);
    let c1 = cf.add(cf.add(cf.mul(p3, xr3), cf.mul(p_s2, xr)), cf.mul(p, xr3));
    let c2 = cf.add(
        cf.add(cf.add(cf.mul(cf.mul(p2, s2), xr2), cf.mul(p2, xr4)), p2),
        cf.mul(s2, xr2),
    );
    let c3 = cf.add(cf.add(cf.mul(p3, xr), cf.mul(p_s2, xr3)), cf.mul(p, xr));
    let c4 = cf.add(cf.add(cf.add(F::E::one(), p4), cf.mul(s4, xr4)), p2_xr2);
    [c0, c1, c2, c3, c4]
}

// ---------------------------------------------------------------------------
// Arms.
// ---------------------------------------------------------------------------

fn arm_scan<F: CField>(coeffs: &[F::E; 5], l: u32, cf: &F) -> Vec<F::E> {
    let mut roots = Vec::new();
    let span = 1u64 << l;
    for i in 0..span {
        let v = F::E::from_small(i);
        let mut acc = coeffs[4];
        for k in (0..4).rev() {
            acc = cf.add(cf.mul(acc, v), coeffs[k]);
        }
        if acc.is_zero() {
            roots.push(v);
        }
    }
    roots.sort();
    roots
}

/// Monic-ise `f` (one inversion, `df` mults). Same op sequence as Phase 1.
fn monicise<F: CField>(coeffs: &[F::E; 5], cf: &F) -> Option<(P<F::E>, usize)> {
    let mut f = P::<F::E>::zero();
    for k in 0..5 {
        f.c[k] = coeffs[k];
    }
    let df = f.deg().unwrap_or(0);
    if df == 0 {
        return None;
    }
    let linv = cf.inv(f.c[df]);
    for k in 0..df {
        f.c[k] = cf.mul(f.c[k], linv);
    }
    f.c[df] = F::E::one();
    Some((f, df))
}

/// The subspace gcd on an already-monic `f`.
fn baseline_core<F: CField>(f: P<F::E>, df: usize, lv: &[F::E], cf: &F) -> P<F::E> {
    let mut t = P::<F::E>::zero();
    t.c[1] = F::E::one();
    let mut pow = rem_monic(t, &f, df, cf);
    let mut acc = P::<F::E>::zero();
    for (i, &ai) in lv.iter().enumerate() {
        if !ai.is_zero() {
            for j in 0..df {
                acc.c[j] = cf.add(acc.c[j], cf.mul(pow.c[j], ai));
            }
        }
        if i + 1 < lv.len() {
            pow = sqr_mod_monic(&pow, &f, df, cf);
        }
    }
    poly_gcd(f, acc, cf)
}

fn arm_baseline<F: CField>(coeffs: &[F::E; 5], lv: &[F::E], cf: &F) -> P<F::E> {
    match monicise(coeffs, cf) {
        None => P::zero(),
        Some((f, df)) => baseline_core(f, df, lv, cf),
    }
}

/// Arm C (Phase 1), unchanged.
fn arm_baseline_frob<F: CField>(coeffs: &[F::E; 5], lv: &[F::E], cf: &F) -> P<F::E> {
    let mut f = P::<F::E>::zero();
    for k in 0..5 {
        f.c[k] = coeffs[k];
    }
    let df = f.deg().unwrap_or(0);
    if df != 4 {
        return arm_baseline(coeffs, lv, cf);
    }
    let linv = cf.inv(f.c[4]);
    for k in 0..4 {
        f.c[k] = cf.mul(f.c[k], linv);
    }
    f.c[4] = F::E::one();
    let r4 = [f.c[0], f.c[1], f.c[2], f.c[3]];
    let mut r6 = [F::E::zero(); 4];
    r6[2] = cf.add(r6[2], f.c[0]);
    r6[3] = cf.add(r6[3], f.c[1]);
    for j in 0..4 {
        r6[j] = cf.add(r6[j], cf.mul(f.c[2], r4[j]));
    }
    let mut xr4 = [F::E::zero(); 4];
    xr4[1] = cf.add(xr4[1], f.c[0]);
    xr4[2] = cf.add(xr4[2], f.c[1]);
    xr4[3] = cf.add(xr4[3], f.c[2]);
    for j in 0..4 {
        xr4[j] = cf.add(xr4[j], cf.mul(f.c[3], r4[j]));
    }
    for j in 0..4 {
        r6[j] = cf.add(r6[j], cf.mul(f.c[3], xr4[j]));
    }
    let frob = |p: &[F::E; 4]| -> [F::E; 4] {
        let s0 = cf.sqr(p[0]);
        let s1 = cf.sqr(p[1]);
        let s2 = cf.sqr(p[2]);
        let s3 = cf.sqr(p[3]);
        let mut o = [F::E::zero(); 4];
        o[0] = cf.add(o[0], s0);
        o[2] = cf.add(o[2], s1);
        for j in 0..4 {
            o[j] = cf.add(o[j], cf.mul(s2, r4[j]));
        }
        for j in 0..4 {
            o[j] = cf.add(o[j], cf.mul(s3, r6[j]));
        }
        o
    };
    let mut pow = [F::E::zero(); 4];
    pow[1] = F::E::one();
    let mut acc = [F::E::zero(); 4];
    for (i, &ai) in lv.iter().enumerate() {
        if !ai.is_zero() {
            for j in 0..4 {
                acc[j] = cf.add(acc[j], cf.mul(pow[j], ai));
            }
        }
        if i + 1 < lv.len() {
            pow = frob(&pow);
        }
    }
    let mut accp = P::<F::E>::zero();
    for j in 0..4 {
        accp.c[j] = acc[j];
    }
    poly_gcd(f, accp, cf)
}

/// Why arm B handed a quartic to the gcd (codes recorded per instance).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BClass {
    Closed = 0,
    F3Zero = 1,
    ShiftRoot = 2,
    AlphaZero = 3,
    Bijective = 4,
    NoExponent = 5,
    DegLow = 6,
}

/// Dickson exponents for arm B: bits of `3⁻¹ mod (2ⁿ−1)` and, when it exists,
/// `3⁻¹ mod (2ⁿ+1)/3`, most significant first.
struct Dickson {
    e_minus: Vec<bool>,
    e_plus: Option<Vec<bool>>,
}

fn inv3_bits(m: &BigUint) -> Option<Vec<bool>> {
    let three = BigUint::from(3u32);
    let r = (m % &three).to_u32_digits().first().copied().unwrap_or(0);
    let e = match r {
        1 => (m * 2u32 + 1u32) / &three,
        2 => (m + 1u32) / &three,
        _ => return None,
    };
    debug_assert_eq!((&e * 3u32) % m, BigUint::from(1u32) % m);
    let bits = e.bits();
    Some((0..bits).rev().map(|i| e.bit(i)).collect())
}

fn dickson_for(n: u32) -> Dickson {
    let one = BigUint::from(1u32);
    let p = &one << n;
    let qm = &p - &one;
    let np = (&p + &one) / 3u32;
    Dickson {
        e_minus: inv3_bits(&qm).expect("odd n: 3 ∤ 2ⁿ−1"),
        e_plus: inv3_bits(&np),
    }
}

/// `D_e(a)` by the Lucas ladder of `RadicalQuartic::dickson`.
fn dickson<F: CField>(a: F::E, e: &[bool], cf: &F) -> F::E {
    let (mut lo, mut hi) = (F::E::zero(), a);
    for &bit in e {
        let mid = cf.add(cf.mul(lo, hi), a);
        if bit {
            lo = mid;
            hi = cf.sqr(hi);
        } else {
            hi = mid;
            lo = cf.sqr(lo);
        }
    }
    lo
}

/// Arm B: the closed form of `RadicalQuartic::solve`, inversions inline,
/// on the counting field; on a fallback class the subspace gcd runs on the
/// same monic `f` and is charged in full. Returns (roots in V, class).
fn arm_closed<F: CField>(
    coeffs: &[F::E; 5],
    lv: &[F::E],
    l: u32,
    dk: &Dickson,
    cf: &F,
    groots: &dyn Fn(&P<F::E>) -> Vec<F::E>,
) -> (Vec<F::E>, BClass, Vec<F::E>) {
    let (f, df) = match monicise(coeffs, cf) {
        None => return (Vec::new(), BClass::DegLow, Vec::new()),
        Some(x) => x,
    };
    let fallback = |f: P<F::E>, why: BClass| -> (Vec<F::E>, BClass, Vec<F::E>) {
        let g = baseline_core(f, df, lv, cf);
        (groots(&g), why, Vec::new())
    };
    if df != 4 {
        return fallback(f, BClass::DegLow);
    }
    let (f0, f1, f2, f3) = (f.c[0], f.c[1], f.c[2], f.c[3]);
    if f3.is_zero() {
        return fallback(f, BClass::F3Zero);
    }
    let s = cf.sqrt(cf.mul(f1, cf.inv(f3)));
    let s2 = cf.sqr(s);
    let b2 = cf.add(cf.mul(f3, s), f2);
    let b0 = cf.add(
        cf.add(
            cf.add(
                cf.add(cf.sqr(s2), cf.mul(f3, cf.mul(s2, s))),
                cf.mul(f2, s2),
            ),
            cf.mul(f1, s),
        ),
        f0,
    );
    if b0.is_zero() {
        return fallback(f, BClass::ShiftRoot);
    }
    let g = cf.inv(b0);
    let (alpha, beta, gamma) = (cf.mul(b2, g), cf.mul(f3, g), g);
    if alpha.is_zero() {
        return fallback(f, BClass::AlphaZero);
    }
    let ra = cf.sqrt(alpha);
    let a32 = cf.mul(alpha, ra);
    let p = cf.inv(cf.mul(beta, a32));
    let (inv_a32, inv_beta) = (cf.mul(beta, p), cf.mul(a32, p));
    let (a, inv_a) = (cf.mul(beta, inv_a32), cf.mul(a32, inv_beta));
    // cubic_root
    let e = if cf.trace(inv_a) == 0 {
        &dk.e_minus
    } else {
        match &dk.e_plus {
            Some(e) => e,
            None => return fallback(f, BClass::NoExponent),
        }
    };
    let t = dickson(a, e, cf);
    if cf.add(cf.mul(cf.sqr(t), t), t) != a {
        return fallback(f, BClass::Bijective);
    }
    let d = cf.mul(ra, t);
    let id = cf.inv(d);
    // affine_roots
    let mut zs: Vec<F::E> = Vec::with_capacity(4);
    let c = cf.mul(beta, id);
    let w = cf.mul(gamma, cf.sqr(cf.mul(d, inv_beta)));
    if cf.trace(w) == 0 {
        let v0 = cf.mul(c, cf.half_trace(w));
        let id2 = cf.sqr(id);
        for v in [v0, cf.add(v0, c)] {
            let q = cf.mul(v, id2);
            if cf.trace(q) == 0 {
                let z = cf.mul(d, cf.half_trace(q));
                zs.push(z);
                zs.push(cf.add(z, d));
            }
        }
    }
    let mut roots = Vec::new();
    let mut all = Vec::new(); // every root in F_{2^n} (validation only; free)
    for z in zs {
        let x = cf.add(s, cf.inv(z));
        all.push(x);
        if x.below(l) {
            roots.push(x);
        }
    }
    roots.sort();
    roots.dedup();
    (roots, BClass::Closed, all)
}

/// Number of distinct roots of `f` in `F_{2^n}`: `deg gcd(f, X^{2^n} + X)`,
/// on an uncounted scratch instrument (validation only).
fn roots_in_field<R: Raw>(coeffs: &[R::E; 5], gf: &R, ex: &Extras<R::E>) -> usize {
    let cf = Counted::new(gf, ex);
    let (f, df) = match monicise(coeffs, &cf) {
        None => return 0,
        Some(x) => x,
    };
    let mut t = P::<R::E>::zero();
    t.c[1] = R::E::one();
    let x_mod = rem_monic(t, &f, df, &cf);
    let mut pow = x_mod.clone();
    for _ in 0..gf.n() {
        pow = sqr_mod_monic(&pow, &f, df, &cf);
    }
    for j in 0..MAXD {
        pow.c[j] = pow.c[j].xor(x_mod.c[j]);
    }
    if pow.deg().is_none() {
        return df; // f divides X^{2^n} + X: splits completely
    }
    poly_gcd(f, pow, &cf).deg().unwrap_or(0)
}

/// Outcome of arm D on one quartic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DClass {
    /// `S₃(x₁,x₂,·)` split and every `S₃(u,x_R,·)` was solved by AS steps.
    Solved = 0,
    /// `S₃(x₁,x₂,·)` has no rational root and the guard did not fire.
    Rejected = 1,
    /// `x₁ = x₂`, `x₁x₂ = 0`, `u = x_R` or `u·x_R = 0`: gcd.
    Degenerate = 2,
    /// `x_R ∈ {x₁, x₂, 1/x₁, 1/x₂}` with no rational u: gcd.
    Guard = 3,
}

/// Roots of `A X² + B X + C` (`A·B ≠ 0`) by one Artin–Schreier step, as
/// `binary_radical_root::as_roots` (one inversion of `A·B`, charged here).
fn as_roots_c<F: CField>(a: F::E, b: F::E, c: F::E, cf: &F) -> Option<[F::E; 2]> {
    let ab_inv = cf.inv(cf.mul(a, b));
    let (inv_a, inv_b) = (cf.mul(b, ab_inv), cf.mul(a, ab_inv));
    let w = cf.mul(cf.mul(a, c), cf.sqr(inv_b));
    if cf.trace(w) != 0 {
        return None;
    }
    let k = cf.mul(b, inv_a);
    let x0 = cf.mul(k, cf.as_solve(w));
    Some([x0, cf.add(x0, k)])
}

/// Arm D: the factored route of `binary_radical_root::FactoredOracle`
/// (b = 1), every width and parity. Returns (roots in V, class, ops after
/// stage A, every candidate root in F when solved).
#[allow(clippy::too_many_arguments)]
fn arm_factored<F: CField>(
    coeffs: &[F::E; 5],
    x1: F::E,
    x2: F::E,
    xr: F::E,
    inv_xr: F::E,
    lv: &[F::E],
    l: u32,
    cf: &F,
    groots: &dyn Fn(&P<F::E>) -> Vec<F::E>,
) -> (Vec<F::E>, DClass, Ops, Vec<F::E>) {
    let by_gcd = |why: DClass, stage_a: Ops| -> (Vec<F::E>, DClass, Ops, Vec<F::E>) {
        match monicise(coeffs, cf) {
            None => (Vec::new(), why, stage_a, Vec::new()),
            Some((f, df)) => {
                let g = baseline_core(f, df, lv, cf);
                (groots(&g), why, stage_a, Vec::new())
            }
        }
    };
    // Stage A: u, u' = roots of S₃(x₁, x₂, ·) = (x₁+x₂)² X² + x₁x₂ X + (x₁x₂)² + 1.
    let a1 = cf.sqr(cf.add(x1, x2));
    let b1 = cf.mul(x1, x2);
    if a1.is_zero() || b1.is_zero() {
        let o = cf.ops();
        return by_gcd(DClass::Degenerate, o);
    }
    let c1 = cf.add(cf.sqr(b1), F::E::one());
    let us = as_roots_c(a1, b1, c1, cf);
    let stage_a = cf.ops();
    let us = match us {
        Some(us) => us,
        None => {
            // guard: x_R ∈ {x₁, x₂, 1/x₁, 1/x₂} (compares only; 1/x_R per target)
            if xr == x1 || xr == x2 || inv_xr == x1 || inv_xr == x2 {
                return by_gcd(DClass::Guard, stage_a);
            }
            return (Vec::new(), DClass::Rejected, stage_a, Vec::new());
        }
    };
    // Stage B: X = roots of S₃(u, x_R, ·) for each rational u.
    let mut all = Vec::with_capacity(4);
    for u in us {
        let a2 = cf.sqr(cf.add(u, xr));
        let b2 = cf.mul(u, xr);
        if a2.is_zero() || b2.is_zero() {
            return by_gcd(DClass::Degenerate, stage_a);
        }
        let c2 = cf.add(cf.sqr(b2), F::E::one());
        if let Some(xs) = as_roots_c(a2, b2, c2, cf) {
            all.extend(xs);
        }
    }
    let mut roots: Vec<F::E> = all.iter().copied().filter(|x| x.below(l)).collect();
    roots.sort();
    roots.dedup();
    (roots, DClass::Solved, stage_a, all)
}

fn eval_raw<R: Raw>(coeffs: &[R::E; 5], x: R::E, gf: &R) -> R::E {
    let mut acc = coeffs[4];
    for k in (0..4).rev() {
        acc = gf.mul(acc, x).xor(coeffs[k]);
    }
    acc
}

// ---------------------------------------------------------------------------
// Oracles (uncounted).
// ---------------------------------------------------------------------------

fn brute_roots<R: Raw>(coeffs: &[R::E; 5], l: u32, gf: &R) -> Vec<R::E> {
    let mut roots = Vec::new();
    for i in 0..(1u64 << l) {
        let v = R::E::from_small(i);
        let mut acc = coeffs[4];
        for k in (0..4).rev() {
            acc = gf.mul(acc, v).xor(coeffs[k]);
        }
        if acc.is_zero() {
            roots.push(v);
        }
    }
    roots.sort();
    roots
}

fn groots_raw<R: Raw>(g: &P<R::E>, l: u32, gf: &R) -> Vec<R::E> {
    let dg = match g.deg() {
        None => return Vec::new(),
        Some(d) => d,
    };
    let mut roots = Vec::new();
    for i in 0..(1u64 << l) {
        let v = R::E::from_small(i);
        let mut acc = g.c[dg];
        for k in (0..dg).rev() {
            acc = gf.mul(acc, v).xor(g.c[k]);
        }
        if acc.is_zero() {
            roots.push(v);
        }
    }
    roots.sort();
    roots
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

// ---------------------------------------------------------------------------
// Planting on y² + xy = x³ + x² + 1 (a = b = 1), uncounted. Odd n uses the
// half-trace root exactly as `koblitz_fast_arith::artin_schreier_root` (so
// odd-n instances are Phase 1's); even n uses the general solver (Phase 1
// had none there).
// ---------------------------------------------------------------------------

type Pt<E> = Option<(E, E)>;

fn lift<R: Raw>(x: R::E, gf: &R, ex: &Extras<R::E>) -> Option<(R::E, R::E)> {
    let n = gf.n();
    if x.is_zero() {
        return Some((x, R::E::one())); // y² = b = 1
    }
    let xi = gf.inv(x);
    let rhs = x.xor(R::E::one()).xor(gf.sqr(xi)); // x + a + b/x²
    let u = if n % 2 == 1 {
        if rhs.parity_and(ex.trace_mask) == 1 {
            return None;
        }
        // H(c) = Σ_{k=0}^{(n−1)/2} c^{4^k}, as the lab's lifter computes it
        let (mut t, mut a) = (rhs, rhs);
        for _ in 0..(n - 1) / 2 {
            a = gf.sqr(gf.sqr(a));
            t = t.xor(a);
        }
        t
    } else {
        ex.as_solve_any(rhs)?
    };
    debug_assert_eq!(gf.sqr(u).xor(u), rhs);
    Some((x, gf.mul(x, u)))
}

fn add<R: Raw>(p: Pt<R::E>, q: Pt<R::E>, gf: &R) -> Pt<R::E> {
    let (x1, y1) = match p {
        None => return q,
        Some(v) => v,
    };
    let (x2, y2) = match q {
        None => return p,
        Some(v) => v,
    };
    let one = R::E::one(); // a = 1
    if x1 == x2 {
        if y2 == x1.xor(y1) {
            return None; // q = −p
        }
        if x1.is_zero() {
            return None; // 2-torsion doubles to infinity
        }
        let lam = x1.xor(gf.mul(y1, gf.inv(x1)));
        let x3 = gf.sqr(lam).xor(lam).xor(one);
        let y3 = gf.sqr(x1).xor(gf.mul(lam.xor(one), x3));
        return Some((x3, y3));
    }
    let lam = gf.mul(y1.xor(y2), gf.inv(x1.xor(x2)));
    let x3 = gf.sqr(lam).xor(lam).xor(x1).xor(x2).xor(one);
    let y3 = gf.mul(lam, x1.xor(x3)).xor(x3).xor(y1);
    Some((x3, y3))
}

fn on_curve<R: Raw>(p: (R::E, R::E), gf: &R) -> bool {
    let (x, y) = p;
    let x2 = gf.sqr(x);
    gf.sqr(y).xor(gf.mul(x, y)) == gf.mul(x2, x).xor(x2).xor(R::E::one())
}

// ---------------------------------------------------------------------------
// Cell runner.
// ---------------------------------------------------------------------------

struct Row {
    arm: &'static str,
    o: Ops,
}

struct Inst {
    rows: Vec<Row>,
    nroots: usize,
    planted: bool,
    plant_ok: bool,
    bclass: i32,
    dclass: i32,
    nroots_field: i32,
}

/// Planting mode. `Phase1`: draw x₁, x₂, x₃ once, as Phase 1 did (a draw
/// plants only when all three lift, about 1/8 of the time). `Resample`
/// (amendment 1b.1): redraw the triple from V until all three lift and the sum
/// is finite, up to 256 tries, so planted instances really decompose.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Plant {
    Phase1,
    Resample,
}

fn run_cell<R: Raw>(
    gf: &R,
    ex: &Extras<R::E>,
    l: u32,
    ninst: usize,
    seed0: u64,
    mode: Plant,
) -> Vec<Inst> {
    let validate = std::env::var("SRB_VALIDATE").is_ok();
    let cf = Counted::new(gf, ex);
    let n = gf.n();
    let lv = subspace_poly_raw(l, gf);
    let span = 1u64 << l;
    let dk = (n % 2 == 1).then(|| dickson_for(n));
    let mut seed = seed0;
    let mut out = Vec::with_capacity(ninst);
    let groots = |g: &P<R::E>| groots_raw(g, l, gf);

    for inst in 0..ninst {
        let mut x1 = R::E::from_small(splitmix64(&mut seed) % span);
        let mut x2 = R::E::from_small(splitmix64(&mut seed) % span);
        let planted = inst % 2 == 0;
        let mut plant_ok = false;
        let xr = if planted {
            let x3 = R::E::from_small(splitmix64(&mut seed) % span);
            let try_plant = |x1: R::E, x2: R::E, x3: R::E| -> Option<R::E> {
                let p1 = lift(x1, gf, ex)?;
                let p2 = lift(x2, gf, ex)?;
                let p3 = lift(x3, gf, ex)?;
                debug_assert!(on_curve(p1, gf) && on_curve(p2, gf) && on_curve(p3, gf));
                add(add(Some(p1), Some(p2), gf), Some(p3), gf).map(|p| p.0)
            };
            let mut r = try_plant(x1, x2, x3);
            if mode == Plant::Resample {
                let mut tries = 0;
                while r.is_none() && tries < 256 {
                    x1 = R::E::from_small(splitmix64(&mut seed) % span);
                    x2 = R::E::from_small(splitmix64(&mut seed) % span);
                    let x3 = R::E::from_small(splitmix64(&mut seed) % span);
                    r = try_plant(x1, x2, x3);
                    tries += 1;
                }
            }
            match r {
                Some(xr) => {
                    plant_ok = true;
                    xr
                }
                None => R::E::rand(&mut seed, n),
            }
        } else {
            R::E::rand(&mut seed, n)
        };

        let coeffs = quartic_coeffs(x1, x2, xr, &cf);

        cf.reset();
        let r_scan = arm_scan(&coeffs, l, &cf);
        let o_scan = cf.ops();
        cf.reset();
        let g = arm_baseline(&coeffs, &lv, &cf);
        let o_base = cf.ops();
        cf.reset();
        let gc = arm_baseline_frob(&coeffs, &lv, &cf);
        let o_frob = cf.ops();
        let closed = dk.as_ref().map(|dk| {
            cf.reset();
            let (roots, why, all) = arm_closed(&coeffs, &lv, l, dk, &cf, &groots);
            (roots, why, cf.ops(), all)
        });

        let inv_xr = gf.inv(xr); // per target, amortised over the row
        cf.reset();
        let (d_roots, dclass, d_a, d_all) =
            arm_factored(&coeffs, x1, x2, xr, inv_xr, &lv, l, &cf, &groots);
        let o_fac = cf.ops();
        let o_fac_b = Ops {
            mul: o_fac.mul - d_a.mul,
            sqr: o_fac.sqr - d_a.sqr,
            inv: o_fac.inv - d_a.inv,
            lin: o_fac.lin - d_a.lin,
        };

        let truth = brute_roots(&coeffs, l, gf);
        let tset: BTreeSet<_> = truth.iter().copied().collect();
        let chk = |name: &str, s: BTreeSet<R::E>| {
            assert_eq!(s, tset, "{name} mismatch n={n} l={l} inst={inst}");
        };
        chk("SCAN", r_scan.into_iter().collect());
        chk("BASELINE", groots_raw(&g, l, gf).into_iter().collect());
        chk("FROB", groots_raw(&gc, l, gf).into_iter().collect());
        let mut rows = vec![
            Row {
                arm: "scan",
                o: o_scan,
            },
            Row {
                arm: "baseline",
                o: o_base,
            },
            Row {
                arm: "frob",
                o: o_frob,
            },
        ];
        let mut bclass = -1;
        let mut nroots_field = -1i32;
        if validate {
            let nf = roots_in_field(&coeffs, gf, ex);
            nroots_field = nf as i32;
            if let Some((_, why, _, all)) = &closed {
                if *why == BClass::Closed {
                    // soundness: every closed-form root is a root of f
                    for &x in all {
                        assert!(
                            eval_raw(&coeffs, x, gf).is_zero(),
                            "CLOSED unsound root n={n} l={l} inst={inst}"
                        );
                    }
                    // completeness: as many distinct roots as f has in F_{2^n}
                    let d: BTreeSet<_> = all.iter().copied().collect();
                    assert_eq!(d.len(), nf, "CLOSED incomplete n={n} l={l} inst={inst}");
                }
            }
        }
        if let Some((roots, why, o, _)) = closed {
            chk("CLOSED", roots.into_iter().collect());
            rows.push(Row { arm: "closed", o });
            bclass = why as i32;
        }
        chk("FACTORED", d_roots.into_iter().collect());
        if validate {
            match dclass {
                DClass::Solved => {
                    for &x in &d_all {
                        assert!(
                            eval_raw(&coeffs, x, gf).is_zero(),
                            "FACTORED unsound root n={n} l={l} inst={inst}"
                        );
                    }
                    let d: BTreeSet<_> = d_all.iter().copied().collect();
                    assert_eq!(
                        d.len() as i32,
                        nroots_field,
                        "FACTORED incomplete n={n} l={l} inst={inst}"
                    );
                }
                DClass::Rejected => assert_eq!(
                    nroots_field, 0,
                    "FACTORED rejected a quartic with roots n={n} l={l} inst={inst}"
                ),
                _ => {}
            }
        }
        rows.push(Row {
            arm: "factored",
            o: o_fac,
        });
        rows.push(Row {
            arm: "factored_b",
            o: o_fac_b,
        });
        out.push(Inst {
            rows,
            nroots: truth.len(),
            planted,
            plant_ok,
            bclass,
            dclass: dclass as i32,
            nroots_field,
        });
    }
    out
}

fn emit_cell(w: &mut dyn Write, word: &str, n: u32, irr: &str, l: u32, insts: &[Inst]) {
    let arms: Vec<&str> = insts[0].rows.iter().map(|r| r.arm).collect();
    for (k, arm) in arms.iter().enumerate() {
        let col =
            |f: fn(&Ops) -> u64| -> Vec<u64> { insts.iter().map(|i| f(&i.rows[k].o)).collect() };
        let j = serde_json::json!({
            "word": word, "n": n, "irr": irr, "l": l, "arm": arm,
            "ninst": insts.len(),
            "mul": col(|o| o.mul), "sqr": col(|o| o.sqr),
            "inv": col(|o| o.inv), "lin": col(|o| o.lin),
            "nroots": insts.iter().map(|i| i.nroots).collect::<Vec<_>>(),
            "planted": insts.iter().map(|i| i.planted as u8).collect::<Vec<_>>(),
            "plant_ok": insts.iter().map(|i| i.plant_ok as u8).collect::<Vec<_>>(),
            "bclass": insts.iter().map(|i| i.bclass).collect::<Vec<_>>(),
            "nroots_field": insts.iter().map(|i| i.nroots_field).collect::<Vec<_>>(),
            "dclass": insts.iter().map(|i| i.dclass).collect::<Vec<_>>(),
        });
        writeln!(w, "{}", j).unwrap();
    }
}

// ---------------------------------------------------------------------------
// Per-op weights of the lab fields (latency chains, median of 5 runs).
// ---------------------------------------------------------------------------

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn time_chain<E: Word>(reps: u64, x0: E, mut step: impl FnMut(E) -> E) -> f64 {
    use std::hint::black_box;
    use std::time::Instant;
    let t = Instant::now();
    let mut x = x0;
    for _ in 0..reps {
        x = step(black_box(x));
    }
    black_box(x);
    t.elapsed().as_nanos() as f64 / reps as f64
}

struct Weights {
    mul: (f64, Vec<f64>),
    sqr: (f64, Vec<f64>),
    inv: (f64, Vec<f64>),
    lin: (f64, Vec<f64>),
}

fn lab_weights<R: Raw>(gf: &R, ex: &Extras<R::E>, reps: u64) -> Weights {
    let mut seed = 0x0bad_5eed_u64;
    let a = R::E::rand(&mut seed, gf.n()).xor(R::E::one());
    let b = R::E::rand(&mut seed, gf.n());
    let one = R::E::one();
    let run = |f: &mut dyn FnMut() -> f64| {
        let v: Vec<f64> = (0..5).map(|_| f()).collect();
        (median(v.clone()), v)
    };
    let mul = run(&mut || time_chain(reps, a, |x| gf.mul(x, b)));
    let sqr = run(&mut || time_chain(reps, a, |x| gf.sqr(x).xor(one)));
    let inv = run(&mut || time_chain(reps / 20, a, |x| gf.inv(x).xor(one)));
    let lin = run(&mut || time_chain(reps, a, |x| ex.sqrt.apply(x).xor(one)));
    Weights { mul, sqr, inv, lin }
}

/// Counted Itoh–Tsujii inversion (the chain shape of `Gf2_128::inv` and
/// `Gf2W::inv`): returns (mul, sqr) and checks the result.
fn it_census<R: Raw>(gf: &R, ex: &Extras<R::E>) -> (u64, u64) {
    let cf = Counted::new(gf, ex);
    let mut seed = 0x17_u64;
    let a = R::E::rand(&mut seed, gf.n()).xor(R::E::one());
    let e = gf.n() - 1;
    let mut c = a;
    let mut k = 1u32;
    let bits = 32 - e.leading_zeros();
    for i in (0..bits - 1).rev() {
        let mut t = c;
        for _ in 0..k {
            t = cf.sqr(t);
        }
        c = cf.mul(c, t);
        k *= 2;
        if (e >> i) & 1 == 1 {
            c = cf.mul(cf.sqr(c), a);
            k += 1;
        }
    }
    let inv = cf.sqr(c);
    assert_eq!(
        inv,
        gf.inv(a),
        "counted Itoh–Tsujii disagrees with the lab inv"
    );
    let o = cf.ops();
    (o.mul, o.sqr)
}

// ---------------------------------------------------------------------------
// Multi-squaring Itoh–Tsujii inversion (Phase 1c): the lab's chain, with
// every x^{2^k}, k ≥ 2, applied as one precomputed F₂-linear byte table.
// ---------------------------------------------------------------------------

struct MsInv<E: Word> {
    n: u32,
    tabs: Vec<(u32, LinTab<E>)>,
}

impl<E: Word> MsInv<E> {
    fn new<R: Raw<E = E>>(gf: &R) -> Self {
        let n = gf.n();
        let e = n - 1;
        let bits = 32 - e.leading_zeros();
        let mut ks = Vec::new();
        let mut k = 1u32;
        for i in (0..bits - 1).rev() {
            if k >= 2 && !ks.contains(&k) {
                ks.push(k);
            }
            k *= 2;
            if (e >> i) & 1 == 1 {
                k += 1;
            }
        }
        let tabs = ks
            .into_iter()
            .map(|k| {
                let imgs: Vec<E> = (0..n).map(|i| gf.sqr_k(E::mono(i), k)).collect();
                (k, LinTab::from_images(&imgs))
            })
            .collect();
        Self { n, tabs }
    }
    fn bytes(&self) -> usize {
        self.tabs
            .iter()
            .map(|(_, t)| t.tab.len() * 256 * std::mem::size_of::<E>())
            .sum()
    }
    #[inline]
    fn msq(&self, x: E, k: u32) -> E {
        self.tabs
            .iter()
            .find(|(kk, _)| *kk == k)
            .expect("table for k")
            .1
            .apply(x)
    }
    /// (inverse, mul, sqr, lin) — `0 ↦ 0`.
    fn inv_counted<R: Raw<E = E>>(&self, gf: &R, a: E) -> (E, u64, u64, u64) {
        if a.is_zero() {
            return (a, 0, 0, 0);
        }
        let (mut m, mut q, mut li) = (0u64, 0u64, 0u64);
        let e = self.n - 1;
        let bits = 32 - e.leading_zeros();
        let mut c = a;
        let mut k = 1u32;
        for i in (0..bits - 1).rev() {
            let t = if k == 1 {
                q += 1;
                gf.sqr(c)
            } else {
                li += 1;
                self.msq(c, k)
            };
            c = gf.mul(c, t);
            m += 1;
            k *= 2;
            if (e >> i) & 1 == 1 {
                c = gf.mul(gf.sqr(c), a);
                m += 1;
                q += 1;
                k += 1;
            }
        }
        q += 1;
        (gf.sqr(c), m, q, li)
    }
}

/// Verify against the lab `inv` and time the multi-squaring inversion.
fn msinv_study<R: Raw>(gf: &R, reps: u64) -> serde_json::Value {
    let ms = MsInv::new(gf);
    let mut seed = 0x1a2b_u64 ^ gf.n() as u64;
    for _ in 0..100_000 {
        let a = R::E::rand(&mut seed, gf.n());
        assert_eq!(ms.inv_counted(gf, a).0, gf.inv(a), "ms-inv n={}", gf.n());
    }
    let a0 = R::E::rand(&mut seed, gf.n()).xor(R::E::one());
    let (_, m, q, li) = ms.inv_counted(gf, a0);
    let one = R::E::one();
    let runs: Vec<f64> = (0..5)
        .map(|_| time_chain(reps / 20, a0, |x| ms.inv_counted(gf, x).0.xor(one)))
        .collect();
    serde_json::json!({
        "verified_inputs": 100_000, "msinv_ns": median(runs.clone()), "msinv_runs": runs,
        "mul": m, "sqr": q, "lin": li,
        "ks": ms.tabs.iter().map(|(k, _)| *k).collect::<Vec<_>>(),
        "table_bytes": ms.bytes(),
    })
}

// ---------------------------------------------------------------------------
// Squaring kernels (C4). Elements as up to three u64 limbs; one shared
// reduction (the shift-and-xor fold of `Gf2W::reduce`) under every kernel,
// so the kernels differ only in how the unreduced square/product is formed.
// ---------------------------------------------------------------------------

#[cfg(target_arch = "x86_64")]
mod kern {
    use std::arch::x86_64::*;

    pub struct K {
        pub n: u32,
        pub limbs: usize,
        low_terms: Vec<u32>,
        mask: [u64; 3],
        table: [u16; 256],
    }

    #[inline(always)]
    fn shr6(r: &[u64; 6], s: u32) -> [u64; 6] {
        let (w, b) = ((s / 64) as usize, s % 64);
        let mut o = [0u64; 6];
        for i in 0..6 {
            let j = i + w;
            if j < 6 {
                o[i] = r[j] >> b;
                if b != 0 && j + 1 < 6 {
                    o[i] |= r[j + 1] << (64 - b);
                }
            }
        }
        o
    }
    #[inline(always)]
    fn shl6(r: &[u64; 6], s: u32) -> [u64; 6] {
        let (w, b) = ((s / 64) as usize, s % 64);
        let mut o = [0u64; 6];
        for i in (0..6).rev() {
            if i >= w {
                let j = i - w;
                o[i] = r[j] << b;
                if b != 0 && j >= 1 {
                    o[i] |= r[j - 1] >> (64 - b);
                }
            }
        }
        o
    }

    impl K {
        pub fn new(n: u32, low_terms: &[u32]) -> Self {
            let mut mask = [0u64; 3];
            for i in 0..n {
                mask[(i / 64) as usize] |= 1u64 << (i % 64);
            }
            let mut table = [0u16; 256];
            for (v, t) in table.iter_mut().enumerate() {
                let mut s = 0u16;
                for b in 0..8 {
                    if (v >> b) & 1 == 1 {
                        s |= 1 << (2 * b);
                    }
                }
                *t = s;
            }
            let mut lt = low_terms.to_vec();
            lt.sort_unstable();
            K {
                n,
                limbs: n.div_ceil(64) as usize,
                low_terms: lt,
                mask,
                table,
            }
        }

        #[inline(always)]
        pub fn reduce(&self, mut r: [u64; 6]) -> [u64; 3] {
            loop {
                let top = shr6(&r, self.n);
                if top.iter().all(|&w| w == 0) {
                    return [r[0], r[1], r[2]];
                }
                for k in 0..3 {
                    r[k] &= self.mask[k];
                }
                for k in 3..6 {
                    r[k] = 0;
                }
                for &t in &self.low_terms {
                    let s = shl6(&top, t);
                    for k in 0..6 {
                        r[k] ^= s[k];
                    }
                }
            }
        }

        #[inline]
        #[target_feature(enable = "pclmulqdq,sse4.1")]
        unsafe fn clmul(a: u64, b: u64) -> (u64, u64) {
            let p =
                _mm_clmulepi64_si128(_mm_set_epi64x(0, a as i64), _mm_set_epi64x(0, b as i64), 0);
            (
                _mm_extract_epi64(p, 0) as u64,
                _mm_extract_epi64(p, 1) as u64,
            )
        }

        #[target_feature(enable = "pclmulqdq,sse4.1")]
        pub unsafe fn mul(&self, a: &[u64; 3], b: &[u64; 3]) -> [u64; 3] {
            let mut r = [0u64; 6];
            for i in 0..self.limbs {
                for j in 0..self.limbs {
                    let (lo, hi) = unsafe { Self::clmul(a[i], b[j]) };
                    r[i + j] ^= lo;
                    r[i + j + 1] ^= hi;
                }
            }
            self.reduce(r)
        }

        /// a² by `limbs` carry-less squarings (cross terms vanish in char 2).
        #[target_feature(enable = "pclmulqdq,sse4.1")]
        pub unsafe fn sqr_clmul(&self, a: &[u64; 3]) -> [u64; 3] {
            let mut r = [0u64; 6];
            for i in 0..self.limbs {
                let (lo, hi) = unsafe { Self::clmul(a[i], a[i]) };
                r[2 * i] = lo;
                r[2 * i + 1] = hi;
            }
            self.reduce(r)
        }

        #[inline(always)]
        fn spread32(x: u64) -> u64 {
            let mut x = x & 0xffff_ffff;
            x = (x | (x << 16)) & 0x0000_ffff_0000_ffff;
            x = (x | (x << 8)) & 0x00ff_00ff_00ff_00ff;
            x = (x | (x << 4)) & 0x0f0f_0f0f_0f0f_0f0f;
            x = (x | (x << 2)) & 0x3333_3333_3333_3333;
            x = (x | (x << 1)) & 0x5555_5555_5555_5555;
            x
        }

        pub fn sqr_spread(&self, a: &[u64; 3]) -> [u64; 3] {
            let mut r = [0u64; 6];
            for i in 0..self.limbs {
                r[2 * i] = Self::spread32(a[i]);
                r[2 * i + 1] = Self::spread32(a[i] >> 32);
            }
            self.reduce(r)
        }

        #[target_feature(enable = "bmi2")]
        pub unsafe fn sqr_pdep(&self, a: &[u64; 3]) -> [u64; 3] {
            let mut r = [0u64; 6];
            for i in 0..self.limbs {
                r[2 * i] = _pdep_u64(a[i], 0x5555_5555_5555_5555);
                r[2 * i + 1] = _pdep_u64(a[i] >> 32, 0x5555_5555_5555_5555);
            }
            self.reduce(r)
        }

        /// Unreduced parts (no reduction), folded 6 -> 3 words by XOR so a
        /// latency chain can be formed: the spread/multiply cost alone.
        #[target_feature(enable = "pclmulqdq,sse4.1")]
        pub unsafe fn part_mul(&self, a: &[u64; 3], b: &[u64; 3]) -> [u64; 3] {
            let mut r = [0u64; 6];
            for i in 0..self.limbs {
                for j in 0..self.limbs {
                    let (lo, hi) = unsafe { Self::clmul(a[i], b[j]) };
                    r[i + j] ^= lo;
                    r[i + j + 1] ^= hi;
                }
            }
            [r[0] ^ r[3], r[1] ^ r[4], r[2] ^ r[5]]
        }
        #[target_feature(enable = "pclmulqdq,sse4.1")]
        pub unsafe fn part_sqr_clmul(&self, a: &[u64; 3]) -> [u64; 3] {
            let mut r = [0u64; 6];
            for i in 0..self.limbs {
                let (lo, hi) = unsafe { Self::clmul(a[i], a[i]) };
                r[2 * i] = lo;
                r[2 * i + 1] = hi;
            }
            [r[0] ^ r[3], r[1] ^ r[4], r[2] ^ r[5]]
        }
        pub fn part_spread(&self, a: &[u64; 3]) -> [u64; 3] {
            let mut r = [0u64; 6];
            for i in 0..self.limbs {
                r[2 * i] = Self::spread32(a[i]);
                r[2 * i + 1] = Self::spread32(a[i] >> 32);
            }
            [r[0] ^ r[3], r[1] ^ r[4], r[2] ^ r[5]]
        }
        #[target_feature(enable = "bmi2")]
        pub unsafe fn part_pdep(&self, a: &[u64; 3]) -> [u64; 3] {
            let mut r = [0u64; 6];
            for i in 0..self.limbs {
                r[2 * i] = _pdep_u64(a[i], 0x5555_5555_5555_5555);
                r[2 * i + 1] = _pdep_u64(a[i] >> 32, 0x5555_5555_5555_5555);
            }
            [r[0] ^ r[3], r[1] ^ r[4], r[2] ^ r[5]]
        }

        pub fn sqr_table(&self, a: &[u64; 3]) -> [u64; 3] {
            let mut r = [0u64; 6];
            for i in 0..self.limbs {
                let mut lo = 0u64;
                let mut hi = 0u64;
                for b in 0..4 {
                    lo |= (self.table[((a[i] >> (8 * b)) & 0xff) as usize] as u64) << (16 * b);
                    hi |= (self.table[((a[i] >> (32 + 8 * b)) & 0xff) as usize] as u64) << (16 * b);
                }
                r[2 * i] = lo;
                r[2 * i + 1] = hi;
            }
            self.reduce(r)
        }
    }
}

fn to3<E: Word>(x: E) -> [u64; 3] {
    let mut o = [0u64; 3];
    for j in 0..24 {
        o[j / 8] |= (x.byte(j) as u64) << (8 * (j % 8));
    }
    o
}

/// Verify every kernel bit-exact against the lab field, then time them
/// (latency chains, median of 5). Returns JSON.
#[cfg(target_arch = "x86_64")]
fn kernel_study<R: Raw>(gf: &R, low_terms: &[u32], reps: u64) -> serde_json::Value {
    let n = gf.n();
    if !(std::arch::is_x86_feature_detected!("pclmulqdq")
        && std::arch::is_x86_feature_detected!("bmi2")
        && std::arch::is_x86_feature_detected!("sse4.1"))
    {
        return serde_json::json!({"skipped": "no pclmulqdq/bmi2"});
    }
    let k = kern::K::new(n, low_terms);
    let mut seed = 0xc0ffee_u64 ^ n as u64;
    for _ in 0..100_000 {
        let a = R::E::rand(&mut seed, n);
        let b = R::E::rand(&mut seed, n);
        let (a3, b3) = (to3(a), to3(b));
        let want_m = to3(gf.mul(a, b));
        let want_s = to3(gf.sqr(a));
        unsafe {
            assert_eq!(k.mul(&a3, &b3), want_m, "kernel mul n={n}");
            assert_eq!(k.sqr_clmul(&a3), want_s, "kernel sqr_clmul n={n}");
            assert_eq!(k.sqr_pdep(&a3), want_s, "kernel sqr_pdep n={n}");
        }
        assert_eq!(k.sqr_spread(&a3), want_s, "kernel sqr_spread n={n}");
        assert_eq!(k.sqr_table(&a3), want_s, "kernel sqr_table n={n}");
    }
    let a0 = to3(R::E::rand(&mut seed, n));
    let b0 = to3(R::E::rand(&mut seed, n));
    let t = |f: &dyn Fn([u64; 3]) -> [u64; 3]| -> (f64, Vec<f64>) {
        let v: Vec<f64> = (0..5)
            .map(|_| {
                use std::hint::black_box;
                let st = std::time::Instant::now();
                let mut x = a0;
                for _ in 0..reps {
                    x = f(black_box(x));
                    x[0] ^= 1;
                }
                black_box(x);
                st.elapsed().as_nanos() as f64 / reps as f64
            })
            .collect();
        (median(v.clone()), v)
    };
    let mul = t(&|x| unsafe { k.mul(&x, &b0) });
    let clm = t(&|x| unsafe { k.sqr_clmul(&x) });
    let spr = t(&|x| k.sqr_spread(&x));
    let pdp = t(&|x| unsafe { k.sqr_pdep(&x) });
    let tab = t(&|x| k.sqr_table(&x));
    let p_mul = t(&|x| unsafe { k.part_mul(&x, &b0) });
    let p_clm = t(&|x| unsafe { k.part_sqr_clmul(&x) });
    let p_spr = t(&|x| k.part_spread(&x));
    let p_pdp = t(&|x| unsafe { k.part_pdep(&x) });
    serde_json::json!({
        "part_mul_ns": p_mul.0, "part_sqr_clmul_ns": p_clm.0,
        "part_spread_ns": p_spr.0, "part_pdep_ns": p_pdp.0,
        "verified_inputs": 100_000,
        "mul_ns": mul.0, "mul_runs": mul.1,
        "sqr_clmul_ns": clm.0, "sqr_clmul_runs": clm.1,
        "sqr_spread_ns": spr.0, "sqr_spread_runs": spr.1,
        "sqr_pdep_ns": pdp.0, "sqr_pdep_runs": pdp.1,
        "sqr_table_ns": tab.0, "sqr_table_runs": tab.1,
    })
}

#[cfg(not(target_arch = "x86_64"))]
fn kernel_study<R: Raw>(_gf: &R, _low: &[u32], _reps: u64) -> serde_json::Value {
    serde_json::json!({"skipped": "not x86_64"})
}

// ---------------------------------------------------------------------------

fn field_block<R: Raw>(
    sink: &mut dyn Write,
    word: &str,
    gf: &R,
    irr: &IrreduciblePoly,
    ls: &[u32],
    ninst: usize,
    master: u64,
    cell_idx: &mut u64,
    reps: u64,
    mode: Plant,
    measure: bool,
) {
    let n = gf.n();
    let ex = extras(gf);
    let irr_str = format!("{:?}", irr.low_terms);
    if measure {
        let w = lab_weights(gf, &ex, reps);
        let (it_mul, it_sqr) = it_census(gf, &ex);
        let kern = kernel_study(gf, &irr.low_terms, reps);
        let msinv = msinv_study(gf, reps);
        writeln!(
            sink,
            "{}",
            serde_json::json!({"rec":"fieldcost","word":word,"n":n,"irr":irr_str,
                "mul_ns":w.mul.0,"sqr_ns":w.sqr.0,"inv_ns":w.inv.0,"lin_ns":w.lin.0,
                "mul_runs":w.mul.1,"sqr_runs":w.sqr.1,"inv_runs":w.inv.1,"lin_runs":w.lin.1,
                "sqr_per_mul":w.sqr.0/w.mul.0,"inv_per_mul":w.inv.0/w.mul.0,
                "it_mul":it_mul,"it_sqr":it_sqr,"kernels":kern,"msinv":msinv})
        )
        .unwrap();
    }
    for &l in ls {
        let seed = master ^ cell_idx.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        *cell_idx += 1;
        let insts = run_cell(gf, &ex, l, ninst, seed, mode);
        emit_cell(sink, word, n, &irr_str, l, &insts);
        let ok = insts.iter().filter(|i| i.plant_ok).count();
        let pl = insts.iter().filter(|i| i.planted).count();
        eprintln!("done {word} n={n} l={l} planted_ok={ok}/{pl}");
    }
}

fn main() {
    let out_dir = std::env::var("SRB_OUT").ok();
    let ninst: usize = std::env::var("SRB_NINST")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2000);
    let reps: u64 = std::env::var("SRB_REPS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4_000_000);
    let master: u64 = 0x5ead_1ca1_c0de_2024;
    let ls: &[u32] = &[6, 8, 10, 12];
    // SRB_PLANT=phase1 reproduces Phase 1's instances (replication gate);
    // the default is the resampled planting of amendment 1b.1.
    let mode = match std::env::var("SRB_PLANT").as_deref() {
        Ok("phase1") => Plant::Phase1,
        _ => Plant::Resample,
    };
    let validate = std::env::var("SRB_VALIDATE").is_ok();
    let tag = match (mode == Plant::Phase1, validate) {
        (true, false) => "phase1mode",
        (true, true) => "phase1mode_validate",
        (false, false) => "1c",
        (false, true) => "1c_validate",
    };

    let mut sink: Box<dyn Write> = match &out_dir {
        Some(d) => {
            std::fs::create_dir_all(d).unwrap();
            Box::new(std::io::BufWriter::new(
                std::fs::File::create(format!("{d}/counts_{tag}.jsonl")).unwrap(),
            ))
        }
        None => Box::new(std::io::BufWriter::new(std::io::stdout())),
    };

    // Same grid order as Phase 1 (so its cell seeds replicate), then n = 131.
    let mut cell_idx: u64 = 0;
    for &n in &[31u32, 41, 53, 24, 32] {
        let irr = find_irreducible_sparse(n).expect("sparse irreducible");
        let gf = Gf2::new(&irr);
        field_block(
            &mut *sink,
            "single",
            &gf,
            &irr,
            ls,
            ninst,
            master,
            &mut cell_idx,
            reps,
            mode,
            true,
        );
    }
    for irr in [IrreduciblePoly::deg_113(), IrreduciblePoly::deg_127()] {
        let gf = Gf2_128::new(&irr);
        field_block(
            &mut *sink,
            "wide",
            &gf,
            &irr,
            ls,
            ninst,
            master,
            &mut cell_idx,
            reps,
            mode,
            true,
        );
    }
    let irr = gf2w::low_weight_irreducible(131);
    let gf = Gf2W::new(&irr);
    field_block(
        &mut *sink,
        "three",
        &gf,
        &irr,
        ls,
        ninst,
        master,
        &mut cell_idx,
        reps,
        mode,
        true,
    );
    // Phase 1c: l = 14 at two and three words, after every earlier cell so
    // their seeds do not move (PROTOCOL_1C §3). Skipped in Phase 1 mode.
    if mode == Plant::Resample && std::env::var("SRB_NO_L14").is_err() {
        for irr in [IrreduciblePoly::deg_113(), IrreduciblePoly::deg_127()] {
            let gf = Gf2_128::new(&irr);
            field_block(
                &mut *sink,
                "wide",
                &gf,
                &irr,
                &[14],
                ninst,
                master,
                &mut cell_idx,
                reps,
                mode,
                false,
            );
        }
        let irr = gf2w::low_weight_irreducible(131);
        let gf = Gf2W::new(&irr);
        field_block(
            &mut *sink,
            "three",
            &gf,
            &irr,
            &[14],
            ninst,
            master,
            &mut cell_idx,
            reps,
            mode,
            false,
        );
    }
    sink.flush().unwrap();
}
