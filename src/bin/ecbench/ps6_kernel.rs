//! PS-6: the PS-2 group-law kernel, generalised from the program's fixed
//! curve to any binary curve `y² + xy = x³ + a·x² + b` over a one-word field.
//!
//! **Source.**  A copy, with the changes below, of
//! `examples/pdp_speedup/group.rs` and `examples/pdp_speedup/kernel.rs` at
//! commit `2a14b74c7b1db8e1b0217e112770ea7933625e03` (PS-2: scalar
//! group-law kernel).  Those files hard-code the program's curve
//! `y² + xy = x³ + x² + 1` (`a = b = 1`); the ledger's char2 ladder runs on
//! random binary curves, so the two constants become parameters.  Nothing
//! else in the arithmetic changed: the same row structure (the two row
//! points `U± = P₁ ± R` once per row, the pair denominators inverted a row
//! at a time by Montgomery's trick), the same four candidate third
//! abscissae per pair, the same exact fallback.
//!
//! **Changes from the PS-2 copy** (every one is a deviation named in
//! `research/pdp_oracle_speedup_20261011/ps6_charge/README.md`):
//!
//! * `a` replaces the constant `1` in `x₃ = λ² + λ + x₁ + x₂ + a` and in the
//!   doubling; `b` enters only through the 2-torsion point
//!   `T = (0, √b)` (the base point of abscissa 0) and through
//!   [`degenerate_target`], where the PS-1 translation `x(P + T) = 1/x`
//!   becomes `x(P + T) = √b / x` (derived in the README, checked
//!   exhaustively in the tests below against the baseline's own root
//!   sets).
//! * The base points are handed in (the run's factor base supplies every
//!   abscissa of `V` that lifts), not recomputed here.
//! * A pair with several candidates in `V` reports the **smallest** one,
//!   which is the root the baseline's `SubspaceOracle::decompose` reports
//!   (it scans `V` in increasing order), so the two return the same
//!   witness `[x₁, x₂, x₃]`, not merely the same verdict.
//! * The kernel counts its own work ([`Work`]): pairs visited, rows,
//!   batch-inverted elements, pairs sent to the exact path.
//!
//! Exactness conditions are PS-1's: the caller sends a
//! [`degenerate_target`] to the baseline, because there the baseline may
//! return extra roots with two non-liftable abscissae.

#![allow(dead_code)] // shared by two binaries and the tests; each uses part of it

use crypto_lib::cryptanalysis::semaev_decomp::Gf2;

/// A point of `y² + xy = x³ + a·x² + b` over the one-word field.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pt {
    Inf,
    Aff(u64, u64),
}

pub fn neg(p: Pt) -> Pt {
    match p {
        Pt::Inf => Pt::Inf,
        Pt::Aff(x, y) => Pt::Aff(x, y ^ x),
    }
}

/// Exact group law, every special case (`P + O`, `P + (−P)`, doubling, the
/// 2-torsion point `(0, √b)`), one inversion per call.
pub fn add(gf: &Gf2, a: u64, p: Pt, q: Pt) -> Pt {
    match (p, q) {
        (Pt::Inf, r) | (r, Pt::Inf) => r,
        (Pt::Aff(x1, y1), Pt::Aff(x2, y2)) => {
            if x1 == x2 {
                if y1 ^ y2 == x1 {
                    return Pt::Inf;
                }
                // y1 == y2 and x1 != 0: doubling.
                let lam = x1 ^ gf.mul(y1, gf.inv(x1));
                let x3 = gf.sqr(lam) ^ lam ^ a;
                let y3 = gf.sqr(x1) ^ gf.mul(lam ^ 1, x3);
                return Pt::Aff(x3, y3);
            }
            let lam = gf.mul(y1 ^ y2, gf.inv(x1 ^ x2));
            let x3 = gf.sqr(lam) ^ lam ^ x1 ^ x2 ^ a;
            let y3 = gf.mul(lam, x1 ^ x3) ^ x3 ^ y1;
            Pt::Aff(x3, y3)
        }
    }
}

/// `√b`, the ordinate of the rational 2-torsion point `T = (0, √b)`.
pub fn sqrt_b(gf: &Gf2, b: u64) -> u64 {
    gf.sqr_k(b, gf.n - 1)
}

/// The kernel's base: the rational points of `V` that the run's factor base
/// holds (any one of the two signs each) plus the 2-torsion point
/// `T = (0, √b)`, which the baseline's span contains and the factor base,
/// built from the non-zero abscissae, does not.  Sorted by abscissa.
pub fn base_with_torsion(gf: &Gf2, b: u64, mut points: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    points.push((0, sqrt_b(gf, b)));
    points.sort_unstable_by_key(|&(x, _)| x);
    points
}

/// A target for which the group-law formulation can disagree with the
/// baseline: `x_R = 0` (R is 2-torsion), `x_R ∈ V`, or `√b / x_R ∈ V`
/// (translation by `T` sends `x ↦ √b / x`).  `V = {x < 2^l}`.
pub fn degenerate_target(gf: &Gf2, b: u64, l: u32, xr: u64) -> bool {
    xr == 0 || xr < (1u64 << l) || gf.mul(sqrt_b(gf, b), gf.inv(xr)) < (1u64 << l)
}

/// The four candidate third abscissae for the pair `(P₁, P₂)`:
/// `x(P₁+P₂+R), x(P₁+P₂−R), x(P₁−P₂+R), x(P₁−P₂−R)`, `None` where the sum
/// is the point at infinity.  Exact reference, slow.
pub fn thirds(gf: &Gf2, a: u64, p1: (u64, u64), p2: (u64, u64), r: (u64, u64)) -> [Option<u64>; 4] {
    let (p1, p2) = (Pt::Aff(p1.0, p1.1), Pt::Aff(p2.0, p2.1));
    let r = Pt::Aff(r.0, r.1);
    let mut out = [None; 4];
    for (k, z) in [add(gf, a, p1, p2), add(gf, a, p1, neg(p2))]
        .into_iter()
        .enumerate()
    {
        for (m, rr) in [r, neg(r)].into_iter().enumerate() {
            if let Pt::Aff(x, _) = add(gf, a, z, rr) {
                out[2 * k + m] = Some(x);
            }
        }
    }
    out
}

/// What one kernel call did, in the kernel's own native units.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Work {
    /// Pairs `(P₁, P₂)` with `P₁ ≤ P₂` visited.
    pub pairs: u64,
    /// Rows (base points `P₁`) started: two exact additions and two batched
    /// inversions' fixed cost each.
    pub rows: u64,
    /// Elements passed through the batch inversions (`2` per pair of a row).
    pub batch_elems: u64,
    /// Pairs with a zero denominator, sent to the exact reference.
    pub exact_pairs: u64,
}

impl Work {
    pub fn absorb(&mut self, o: Work) {
        self.pairs += o.pairs;
        self.rows += o.rows;
        self.batch_elems += o.batch_elems;
        self.exact_pairs += o.exact_pairs;
    }
}

/// The scalar row kernel.  Pairs-and-solve by the group law with the two
/// row points `U± = P₁ ± R` hoisted out of the pair loop and the pair
/// denominators inverted a row at a time.
///
/// For a base pair `(P₁, P₂)` the four candidate third abscissae are
/// `x(U₊ ± P₂)` and `x(U₋ ± P₂)`; with `λ = (y_U + y₂)/(x_U + x₂)`:
///
/// ```text
///   x(U + P₂) = λ² + λ + (x_U + x₂) + a
///   x(U − P₂) = λ̃² + λ̃ + (x_U + x₂) + a,   λ̃ = λ + x₂/(x_U + x₂)
/// ```
///
/// so a pair costs two batched inversions, four products and four
/// squarings, and the test `X₃ ∈ V` is `X₃ < 2^ℓ`.
pub struct Kernel<'a> {
    gf: &'a Gf2,
    a: u64,
    l: u32,
    /// Rational base points `(x, y)`, `x` ascending, `x = 0` included.
    base: Vec<(u64, u64)>,
    den: Vec<u64>,
    scratch: Vec<u64>,
}

impl<'a> Kernel<'a> {
    pub fn new(gf: &'a Gf2, a: u64, l: u32, base: Vec<(u64, u64)>) -> Self {
        debug_assert!(base.windows(2).all(|w| w[0].0 < w[1].0));
        let m = base.len();
        Self {
            gf,
            a,
            l,
            base,
            den: vec![0; 2 * m],
            scratch: Vec::with_capacity(2 * m),
        }
    }

    pub fn base_len(&self) -> usize {
        self.base.len()
    }

    /// Pairs a refutation visits: `|B|(|B|+1)/2`.
    pub fn pairs_total(&self) -> u64 {
        let m = self.base.len() as u64;
        m * (m + 1) / 2
    }

    /// The first witness `[x₁, x₂, x₃]` of `R` in the baseline's order
    /// (smallest `(x₁, x₂)` pair, then smallest `x₃`), or `None`.
    pub fn first(&mut self, r: (u64, u64)) -> (Option<[u64; 3]>, Work) {
        let mut out = Vec::new();
        let w = self.run(r, false, &mut out);
        (out.first().copied(), w)
    }

    /// Every witness `[x₁, x₂, x₃]` with `x₁ ≤ x₂` the visited pair (a
    /// triple appears once per pair that completes it).
    pub fn all(&mut self, r: (u64, u64)) -> (Vec<[u64; 3]>, Work) {
        let mut out = Vec::new();
        let w = self.run(r, true, &mut out);
        (out, w)
    }

    fn run(&mut self, r: (u64, u64), all: bool, out: &mut Vec<[u64; 3]>) -> Work {
        #[cfg(target_arch = "x86_64")]
        if self.gf.folds() && std::arch::is_x86_feature_detected!("pclmulqdq") {
            // SAFETY: pclmulqdq was just detected.
            return unsafe { self.run_pclmul(r, all, out) };
        }
        self.run_body(r, all, out)
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn run_pclmul(&mut self, r: (u64, u64), all: bool, out: &mut Vec<[u64; 3]>) -> Work {
        self.run_body(r, all, out)
    }

    #[inline(always)]
    fn run_body(&mut self, r: (u64, u64), all: bool, out: &mut Vec<[u64; 3]>) -> Work {
        let gf = self.gf;
        let a = self.a;
        let span = 1u64 << self.l;
        let m = self.base.len();
        let rp = Pt::Aff(r.0, r.1);
        let nr = neg(rp);
        let mut work = Work::default();
        let mut found = false;
        for i1 in 0..m {
            let p1 = Pt::Aff(self.base[i1].0, self.base[i1].1);
            let (Pt::Aff(xu, yu), Pt::Aff(xv, yv)) = (add(gf, a, p1, rp), add(gf, a, p1, nr))
            else {
                // P₁ = ±R: only when x_R ∈ V, which the caller excludes.
                work.rows += 1;
                for i2 in i1..m {
                    work.pairs += 1;
                    work.exact_pairs += 1;
                    self.exact_pair(i1, i2, r, all, out, &mut found);
                    if found && !all {
                        return work;
                    }
                }
                continue;
            };
            work.rows += 1;
            let cnt = m - i1;
            work.batch_elems += 2 * cnt as u64;
            for k in 0..cnt {
                let x2 = self.base[i1 + k].0;
                self.den[k] = xu ^ x2;
                self.den[cnt + k] = xv ^ x2;
            }
            gf.batch_inv(&mut self.den[..2 * cnt], &mut self.scratch);
            for k in 0..cnt {
                let (x2, y2) = self.base[i1 + k];
                work.pairs += 1;
                let (j, jj) = (self.den[k], self.den[cnt + k]);
                if j == 0 || jj == 0 {
                    work.exact_pairs += 1;
                    self.exact_pair(i1, i1 + k, r, all, out, &mut found);
                    if found && !all {
                        return work;
                    }
                    continue;
                }
                let d = xu ^ x2;
                let lam = gf.mul(yu ^ y2, j);
                let t1 = gf.sqr(lam) ^ lam ^ d ^ a;
                let lam2 = lam ^ gf.mul(x2, j);
                let t2 = gf.sqr(lam2) ^ lam2 ^ d ^ a;
                let dd = xv ^ x2;
                let mu = gf.mul(yv ^ y2, jj);
                let t3 = gf.sqr(mu) ^ mu ^ dd ^ a;
                let mu2 = mu ^ gf.mul(x2, jj);
                let t4 = gf.sqr(mu2) ^ mu2 ^ dd ^ a;
                if (t1 < span) | (t2 < span) | (t3 < span) | (t4 < span) {
                    found = true;
                    let x1 = self.base[i1].0;
                    if !all {
                        let t = t1.min(t2).min(t3).min(t4);
                        out.push([x1, x2, t]);
                        return work;
                    }
                    for t in [t1, t2, t3, t4] {
                        if t < span {
                            out.push([x1, x2, t]);
                        }
                    }
                }
            }
        }
        work
    }

    #[cold]
    fn exact_pair(
        &self,
        i1: usize,
        i2: usize,
        r: (u64, u64),
        all: bool,
        out: &mut Vec<[u64; 3]>,
        found: &mut bool,
    ) {
        let span = 1u64 << self.l;
        let ts = thirds(self.gf, self.a, self.base[i1], self.base[i2], r);
        let x1 = self.base[i1].0;
        let x2 = self.base[i2].0;
        if all {
            for t in ts.into_iter().flatten() {
                if t < span {
                    *found = true;
                    out.push([x1, x2, t]);
                }
            }
        } else if let Some(t) = ts.into_iter().flatten().filter(|&t| t < span).min() {
            *found = true;
            out.push([x1, x2, t]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::ic_boundary::{random_binary_instance, BinaryInstance};
    use crypto_lib::cryptanalysis::koblitz_fast::FastPoint;
    use crypto_lib::cryptanalysis::semaev_decomp::SubspaceOracle;

    /// The rational base points of `V` in increasing abscissa, `x = 0`
    /// included, from the instance's own lifting.
    pub(crate) fn base_of(inst: &BinaryInstance, l: u32) -> Vec<(u64, u64)> {
        (0..(1u64 << l))
            .filter_map(|x| {
                inst.points_with_x(x)
                    .first()
                    .map(|p: &FastPoint| (p.x, p.y))
            })
            .collect()
    }

    /// Every abscissa that carries a rational point of the curve, from a
    /// full scan of the field.
    fn rational_points(inst: &BinaryInstance) -> Vec<FastPoint> {
        (0..(1u64 << inst.n))
            .flat_map(|x| inst.points_with_x(x))
            .collect()
    }

    #[test]
    fn x3_formula_matches_the_group_law_for_any_a() {
        for (n, seed) in [(8u32, 1u64), (9, 2), (10, 3)] {
            let Some(inst) = random_binary_instance(n, seed, 8) else {
                continue;
            };
            let gf = &inst.gf;
            let pts = rational_points(&inst);
            for w in pts.windows(2).take(300) {
                let (p, q) = (Pt::Aff(w[0].x, w[0].y), Pt::Aff(w[1].x, w[1].y));
                let got = add(gf, inst.a, p, q);
                let want = inst.fast.add(w[0], w[1]);
                match got {
                    Pt::Inf => assert!(want.infinity),
                    Pt::Aff(x, y) => assert_eq!((x, y), (want.x, want.y)),
                }
            }
        }
    }

    /// The group-law kernel returns the baseline's witness, not merely its
    /// verdict, on every rational target `x_R ≠ 0` that is not degenerate;
    /// on a degenerate one the kernel is not used (the plug-in defers to the
    /// baseline), so only the non-degenerate targets are compared.  Random
    /// curves with both values of `a`, every `l`.
    #[test]
    fn kernel_returns_the_baselines_witness_exhaustively() {
        let mut targets = 0u64;
        let mut degenerate = 0u64;
        let mut hits = 0u64;
        let mut cases = 0;
        for n in 6u32..=10 {
            for seed in 0u64..6 {
                let Some(inst) = random_binary_instance(n, 1000 * seed + n as u64, 8) else {
                    continue;
                };
                cases += 1;
                let gf = &inst.gf;
                let all_pts = rational_points(&inst);
                for l in 2..=(n - 1).min(7) {
                    let basis: Vec<u64> = (0..l).map(|i| 1u64 << i).collect();
                    let baseline = SubspaceOracle::new(&basis, inst.b, gf);
                    let base = base_of(&inst, l);
                    let mut kernel = Kernel::new(gf, inst.a, l, base);
                    for p in all_pts.iter().filter(|p| p.x != 0) {
                        let xr = p.x;
                        if degenerate_target(gf, inst.b, l, xr) {
                            degenerate += 1;
                            continue;
                        }
                        targets += 1;
                        let (want, _pairs) = baseline.decompose(xr, gf);
                        let (got, _w) = kernel.first((p.x, p.y));
                        assert_eq!(
                            got, want,
                            "n={n} l={l} a={} b={:#x} xr={xr:#x}",
                            inst.a, inst.b
                        );
                        hits += u64::from(want.is_some());
                    }
                }
            }
        }
        eprintln!(
            "exhaustive: {cases} curves, {targets} non-degenerate targets ({hits} with a witness), {degenerate} degenerate skipped"
        );
        assert!(cases >= 10 && targets > 10_000 && hits > 100);
    }

    /// The set of every kernel root (over all pairs) equals the baseline's
    /// enumeration by pairs, for the non-degenerate targets of a few fields.
    #[test]
    fn all_roots_agree_with_a_pairwise_enumeration() {
        let Some(inst) = random_binary_instance(9, 4242, 8) else {
            return;
        };
        let gf = &inst.gf;
        let l = 3;
        let basis: Vec<u64> = (0..l).map(|i| 1u64 << i).collect();
        let baseline = SubspaceOracle::new(&basis, inst.b, gf);
        let base = base_of(&inst, l);
        let mut kernel = Kernel::new(gf, inst.a, l, base.clone());
        for p in rational_points(&inst).iter().filter(|p| p.x != 0).take(200) {
            if degenerate_target(gf, inst.b, l, p.x) {
                continue;
            }
            let (roots, _) = kernel.all((p.x, p.y));
            // Reference: exact thirds for every pair.
            let mut reference: Vec<[u64; 3]> = Vec::new();
            for i in 0..base.len() {
                for j in i..base.len() {
                    for t in thirds(gf, inst.a, base[i], base[j], (p.x, p.y))
                        .into_iter()
                        .flatten()
                    {
                        if t < (1u64 << l) {
                            reference.push([base[i].0, base[j].0, t]);
                        }
                    }
                }
            }
            let mut got = roots.clone();
            got.sort_unstable();
            reference.sort_unstable();
            assert_eq!(got, reference);
            // The baseline's first witness is among them.
            if let (Some(w), _) = baseline.decompose(p.x, gf) {
                assert!(roots.contains(&w));
            }
        }
    }

    #[test]
    fn translation_by_the_two_torsion_point_is_sqrt_b_over_x() {
        for (n, seed) in [(8u32, 11u64), (9, 12), (10, 13), (11, 14)] {
            let Some(inst) = random_binary_instance(n, seed, 8) else {
                continue;
            };
            let gf = &inst.gf;
            let w = sqrt_b(gf, inst.b);
            let t = Pt::Aff(0, w);
            assert_eq!(add(gf, inst.a, t, t), Pt::Inf);
            for p in rational_points(&inst).iter().filter(|p| p.x != 0) {
                let s = add(gf, inst.a, Pt::Aff(p.x, p.y), t);
                let Pt::Aff(x, _) = s else {
                    panic!("P + T is finite for P ≠ T")
                };
                assert_eq!(x, gf.mul(w, gf.inv(p.x)));
            }
        }
    }
}
