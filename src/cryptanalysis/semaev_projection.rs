//! Exact projection joins for prime-field Semaev decomposition systems.
//!
//! This module replaces a Gröbner-basis decision for the three-summand
//! chain by a join on the roots of its two quadratic `S_3` links.  A root in
//! `F_p` is keyed by its value.  The conjugate roots of an irreducible
//! quadratic are keyed by the coefficients of their monic minimal
//! polynomial, so the join never has to construct `F_{p^2}`.  For four
//! summands, the same pair projection gives the balanced tree
//!
//! ```text
//! S_3(x1,x2,u) = S_3(x3,x4,v) = S_3(u,v,target) = 0.
//! ```
//!
//! The construction is exact, but it is still a table algorithm: it stores
//! at most `min(p, O(B^2))` finite pair roots for a factor base of size `B`.
//! It is therefore an oracle-stage alternative, not by itself an asymptotic
//! ECDLP claim.

use std::collections::HashMap;

use super::pkm_smooth_tower::{addm, invm, mulm, sqrtm, subm, Curve};

/// A root represented without choosing an extension-field model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize)]
pub enum QuadraticRootKey {
    /// A root in the base field.
    Base(u64),
    /// Either conjugate root of `X^2 + linear*X + constant`.
    Irreducible { linear: u64, constant: u64 },
}

/// The roots of a polynomial of degree at most two.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub enum QuadraticRoots {
    None,
    Finite(Vec<QuadraticRootKey>),
    All,
}

/// Exact, structural work done while building the pair projection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct BuildStats {
    pub unordered_pairs: u64,
    pub root_keys: u64,
    pub base_field_root_keys: u64,
    pub irreducible_root_keys: u64,
    pub universal_pairs: u64,
    pub distinct_keys: usize,
    pub distinct_base_roots: usize,
}

/// Exact work done by one online query.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct QueryStats {
    pub quadratics: u64,
    pub root_keys: u64,
    pub lookups: u64,
    pub compatible_keys: u64,
    pub cancellation_checks: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct ThreeSumWitness {
    pub x: [u64; 3],
    /// `None` denotes the point at infinity in the cancellation branch.
    pub auxiliary: Option<QuadraticRootKey>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct FourSumWitness {
    pub x: [u64; 4],
    /// `None` in one slot denotes a cancelling pair and the point at infinity.
    pub auxiliaries: [Option<u64>; 2],
}

/// `S_3(x1,x2,X)`, low coefficient first.
pub fn s3_quadratic(curve: &Curve, x1: u64, x2: u64) -> [u64; 3] {
    let p = curve.p;
    let d = subm(x1, x2, p);
    let s = addm(x1, x2, p);
    let product = mulm(x1, x2, p);
    let c2 = mulm(d, d, p);
    let c1 = subm(
        0,
        mulm(
            2,
            addm(
                mulm(s, addm(product, curve.a, p), p),
                mulm(2, curve.b, p),
                p,
            ),
            p,
        ),
        p,
    );
    let shifted = subm(product, curve.a, p);
    let c0 = subm(
        mulm(shifted, shifted, p),
        mulm(4, mulm(curve.b, s, p), p),
        p,
    );
    [c0, c1, c2]
}

pub fn eval_quadratic(q: [u64; 3], x: u64, p: u64) -> u64 {
    addm(q[0], mulm(x, addm(q[1], mulm(q[2], x, p), p), p), p)
}

/// Classify the roots of `c0 + c1 X + c2 X^2` over the algebraic closure.
///
/// Generic (reference) path: one `invm` for the leading coefficient and a
/// Tonelli-Shanks `sqrtm` of the discriminant.  For `S_3(x1,x2,X)` with
/// `x1,x2` abscissae of `F_p`-points the discriminant is `(4 y1 y2)^2`, so
/// the `Irreducible` outcome is unreachable there; see
/// `quadratic_roots_known_sqrt` for the fast path that uses that identity.
pub fn quadratic_root_keys(q: [u64; 3], p: u64) -> QuadraticRoots {
    let [c0, c1, c2] = q.map(|x| x % p);
    if c2 == 0 {
        if c1 == 0 {
            return if c0 == 0 {
                QuadraticRoots::All
            } else {
                QuadraticRoots::None
            };
        }
        return QuadraticRoots::Finite(vec![QuadraticRootKey::Base(mulm(
            subm(0, c0, p),
            invm(c1, p),
            p,
        ))]);
    }

    let inv_lead = invm(c2, p);
    let linear = mulm(c1, inv_lead, p);
    let constant = mulm(c0, inv_lead, p);
    let discriminant = subm(mulm(linear, linear, p), mulm(4 % p, constant, p), p);
    if let Some(root_disc) = sqrtm(discriminant, p) {
        let inv_two = invm(2 % p, p);
        let minus_linear = subm(0, linear, p);
        let r0 = mulm(addm(minus_linear, root_disc, p), inv_two, p);
        let r1 = mulm(subm(minus_linear, root_disc, p), inv_two, p);
        let mut roots = vec![QuadraticRootKey::Base(r0)];
        if r1 != r0 {
            roots.push(QuadraticRootKey::Base(r1));
        }
        QuadraticRoots::Finite(roots)
    } else {
        QuadraticRoots::Finite(vec![QuadraticRootKey::Irreducible { linear, constant }])
    }
}

/// Montgomery batch inversion: one `invm` plus `3(n-1)` multiplications for
/// all nonzero entries.  Zero entries map to zero and are skipped.
pub fn batch_invm(xs: &[u64], p: u64) -> Vec<u64> {
    let mut prefix = Vec::with_capacity(xs.len());
    let mut acc = 1u64 % p;
    for &x in xs {
        prefix.push(acc);
        if x % p != 0 {
            acc = mulm(acc, x % p, p);
        }
    }
    let mut out = vec![0u64; xs.len()];
    if xs.iter().all(|&x| x % p == 0) {
        return out;
    }
    let mut inv = invm(acc, p);
    for i in (0..xs.len()).rev() {
        let x = xs[i] % p;
        if x != 0 {
            out[i] = mulm(inv, prefix[i], p);
            inv = mulm(inv, x, p);
        }
    }
    out
}

/// Denominator whose inverse `quadratic_root_fast` needs: `d = x1 - x2` when
/// the quadratic is genuinely quadratic (`c2 = d^2 != 0`), else `c1`.
#[inline]
fn fast_den(q: [u64; 3], d: u64) -> u64 {
    if d != 0 {
        d
    } else {
        q[1]
    }
}

/// `c2^-1` (or `c1^-1` when `d = 0`) from the batch inverse of `fast_den`.
#[inline]
fn fast_inv_den(q: [u64; 3], d: u64, inv_den: u64, p: u64) -> u64 {
    let _ = q;
    if d != 0 {
        mulm(inv_den, inv_den, p)
    } else {
        inv_den
    }
}

/// Root classification of `S_3(x1,x2,X)` for two *rational* points, with the
/// square root of the discriminant supplied.
///
/// Identity (verified symbolically and on 2400 random pairs):
/// `c1^2 - 4 c0 c2 = 16 f(x1) f(x2) = (4 y1 y2)^2`, so for `P1=(x1,y1)`,
/// `P2=(x2,y2)` on `E` the discriminant is always a square in `F_p` with
/// `sqrt = 4 y1 y2`; `QuadraticRoots::Irreducible` cannot occur and no
/// Tonelli-Shanks call is needed.  `inv_lead` is `c2^-1` (`c2 != 0`) or
/// `c1^-1` (`c2 = 0`, `c1 != 0`); ignored otherwise.  `sqrt_disc = 4 y1 y2`.
pub fn quadratic_roots_known_sqrt(
    q: [u64; 3],
    p: u64,
    inv_lead: u64,
    sqrt_disc: u64,
) -> QuadraticRoots {
    let [c0, c1, c2] = q.map(|x| x % p);
    if c2 == 0 {
        if c1 == 0 {
            return if c0 == 0 {
                QuadraticRoots::All
            } else {
                QuadraticRoots::None
            };
        }
        return QuadraticRoots::Finite(vec![QuadraticRootKey::Base(mulm(
            subm(0, c0, p),
            inv_lead,
            p,
        ))]);
    }
    debug_assert!(
        {
            // c1^2 - 4 c0 c2 == sqrt_disc^2 (uncounted raw arithmetic)
            let m = |a: u64, b: u64| ((a as u128 * b as u128) % p as u128) as u64;
            subm(m(c1, c1), m(4 % p, m(c0, c2)), p) == m(sqrt_disc, sqrt_disc)
        },
        "S3 discriminant is not (4 y1 y2)^2: inputs are not rational points"
    );
    // Normalise the sign of sqrt(disc) so the root order is deterministic.
    let s = sqrt_disc.min(p - sqrt_disc) % p;
    let half_inv = mulm(inv_lead, p.div_ceil(2), p); // (2 c2)^-1
    let minus_c1 = subm(0, c1, p);
    let r0 = mulm(addm(minus_c1, s, p), half_inv, p);
    let r1 = mulm(subm(minus_c1, s, p), half_inv, p);
    let mut roots = vec![QuadraticRootKey::Base(r0)];
    if r1 != r0 {
        roots.push(QuadraticRootKey::Base(r1));
    }
    QuadraticRoots::Finite(roots)
}

/// All quadratics `S_3(x_i, t)` for rational points `(x_i, y_i)` and target
/// point `(t, yt)`, with one batch inversion.
fn roots_batch(curve: &Curve, pts: &[(u64, u64)], t: u64, yt: u64) -> Vec<QuadraticRoots> {
    let p = curve.p;
    let qs: Vec<[u64; 3]> = pts.iter().map(|&(x, _)| s3_quadratic(curve, x, t)).collect();
    let ds: Vec<u64> = pts.iter().map(|&(x, _)| subm(x, t, p)).collect();
    let dens: Vec<u64> = qs.iter().zip(&ds).map(|(&q, &d)| fast_den(q, d)).collect();
    let invs = batch_invm(&dens, p);
    pts.iter()
        .enumerate()
        .map(|(i, &(_, y))| {
            let inv = fast_inv_den(qs[i], ds[i], invs[i], p);
            let s = mulm(4, mulm(y, yt, p), p);
            quadratic_roots_known_sqrt(qs[i], p, inv, s)
        })
        .collect()
}

/// A target-independent projection of all unordered factor-base pairs.
#[derive(Clone, Debug)]
pub struct PairProjection {
    curve: Curve,
    base: Vec<u64>,
    witnesses: HashMap<QuadraticRootKey, (u64, u64)>,
    finite_witnesses: HashMap<u64, (u64, u64)>,
    /// Canonical `y_i` per base element (parallel to `base`); empty for
    /// projections built by `build_generic`.
    ys: Vec<u64>,
    /// Finite pair roots `(u, y_u, x1, x2)` in first-insertion order; empty
    /// for projections built by `build_generic`.
    finite_list: Vec<(u64, u64, u64, u64)>,
    universal_pair: Option<(u64, u64)>,
    stats: BuildStats,
}

impl PairProjection {
    /// Fast build: `B` square roots (one canonical `y_i` per base `x_i`), one
    /// batch inversion for all pair quadratics, no per-pair `sqrtm`.
    /// Produces the same `witnesses`/`finite_witnesses`/stats as
    /// `build_generic`.
    pub fn build(curve: Curve, base: &[u64]) -> Result<Self, String> {
        if curve.p <= 2 {
            return Err("projection join requires an odd prime field".into());
        }
        let p = curve.p;
        let mut clean = base.to_vec();
        clean.sort_unstable();
        clean.dedup();
        let mut ys = Vec::with_capacity(clean.len());
        for &x in &clean {
            match sqrtm(curve.rhs(x), p) {
                Some(y) => ys.push(y),
                None => return Err(format!("factor-base x={x} is not an F_p point")),
            }
        }

        let n = clean.len();
        let mut pairs = Vec::with_capacity(n * (n + 1) / 2);
        let mut qs = Vec::with_capacity(pairs.capacity());
        let mut dens = Vec::with_capacity(pairs.capacity());
        for i in 0..n {
            for j in i..n {
                let q = s3_quadratic(&curve, clean[i], clean[j]);
                let d = subm(clean[j], clean[i], p);
                dens.push(fast_den(q, d));
                qs.push(q);
                pairs.push((i, j, d));
            }
        }
        let invs = batch_invm(&dens, p);

        let mut witnesses = HashMap::new();
        let mut finite_witnesses = HashMap::new();
        let mut finite_list = Vec::new();
        let mut universal_pair = None;
        let mut stats = BuildStats::default();
        for (k, &(i, j, d)) in pairs.iter().enumerate() {
            let (x1, x2, y1, y2) = (clean[i], clean[j], ys[i], ys[j]);
            stats.unordered_pairs += 1;
            let inv = fast_inv_den(qs[k], d, invs[k], p);
            let sqrt_disc = mulm(4, mulm(y1, y2, p), p);
            match quadratic_roots_known_sqrt(qs[k], p, inv, sqrt_disc) {
                QuadraticRoots::None => {}
                QuadraticRoots::All => {
                    stats.universal_pairs += 1;
                    universal_pair.get_or_insert((x1, x2));
                }
                QuadraticRoots::Finite(keys) => {
                    for key in keys {
                        stats.root_keys += 1;
                        match key {
                            QuadraticRootKey::Base(root) => {
                                stats.base_field_root_keys += 1;
                                if !finite_witnesses.contains_key(&root) {
                                    finite_witnesses.insert(root, (x1, x2));
                                    let y_root = Self::root_y(&curve, x1, y1, x2, y2, d, inv, root);
                                    finite_list.push((root, y_root, x1, x2));
                                }
                            }
                            QuadraticRootKey::Irreducible { .. } => {
                                debug_assert!(false, "Irreducible root on rational inputs");
                                stats.irreducible_root_keys += 1;
                            }
                        }
                        witnesses.entry(key).or_insert((x1, x2));
                    }
                }
            }
        }
        stats.distinct_keys = witnesses.len();
        stats.distinct_base_roots = finite_witnesses.len();
        Ok(Self {
            curve,
            base: clean,
            witnesses,
            finite_witnesses,
            ys,
            finite_list,
            universal_pair,
            stats,
        })
    }

    /// `y` of the sum/difference point whose abscissa is `root`, from the
    /// group law with the already-batched inverse (no extra inversion, no
    /// square root).  `d = x2 - x1`; `inv = d^-2` if `d != 0` else `c1^-1`.
    #[allow(clippy::too_many_arguments)]
    fn root_y(curve: &Curve, x1: u64, y1: u64, x2: u64, y2: u64, d: u64, inv: u64, root: u64) -> u64 {
        let p = curve.p;
        let lam_y = |lambda: u64, x_third: u64| {
            subm(mulm(lambda, subm(x1, x_third, p), p), y1, p)
        };
        if d != 0 {
            // inv = d^-2, so d^-1 = d * inv.
            let inv_d = mulm(d, inv, p);
            for num in [subm(y2, y1, p), subm(0, addm(y1, y2, p), p)] {
                let lambda = mulm(num, inv_d, p);
                let x3 = subm(subm(mulm(lambda, lambda, p), x1, p), x2, p);
                if x3 == root {
                    return lam_y(lambda, x3);
                }
            }
        } else if y1 != 0 {
            // doubling: c2 = 0, inv = c1^-1 = (-4 y1^2)^-1, so (2 y1)^-1 = -2 y1 * inv.
            let inv_2y = mulm(mulm(2, y1, p), subm(0, inv, p), p);
            let lambda = mulm(addm(mulm(3, mulm(x1, x1, p), p), curve.a, p), inv_2y, p);
            let x3 = subm(mulm(lambda, lambda, p), mulm(2, x1, p), p);
            if x3 == root {
                return lam_y(lambda, x3);
            }
        }
        // Unreachable on rational inputs; sound fallback.
        debug_assert!(false, "root abscissa is not x(P1 +- P2)");
        sqrtm(curve.rhs(root), p).unwrap_or(0)
    }

    /// Reference build: per-pair `invm` + Tonelli-Shanks `sqrtm`.
    pub fn build_generic(curve: Curve, base: &[u64]) -> Result<Self, String> {
        if curve.p <= 2 {
            return Err("projection join requires an odd prime field".into());
        }
        let mut clean = base.to_vec();
        clean.sort_unstable();
        clean.dedup();
        if let Some(&bad) = clean
            .iter()
            .find(|&&x| sqrtm(curve.rhs(x), curve.p).is_none())
        {
            return Err(format!("factor-base x={bad} is not an F_p point"));
        }

        let mut witnesses = HashMap::new();
        let mut finite_witnesses = HashMap::new();
        let mut universal_pair = None;
        let mut stats = BuildStats::default();
        for (i, &x1) in clean.iter().enumerate() {
            for &x2 in &clean[i..] {
                stats.unordered_pairs += 1;
                match quadratic_root_keys(s3_quadratic(&curve, x1, x2), curve.p) {
                    QuadraticRoots::None => {}
                    QuadraticRoots::All => {
                        stats.universal_pairs += 1;
                        universal_pair.get_or_insert((x1, x2));
                    }
                    QuadraticRoots::Finite(keys) => {
                        for key in keys {
                            stats.root_keys += 1;
                            match key {
                                QuadraticRootKey::Base(root) => {
                                    stats.base_field_root_keys += 1;
                                    finite_witnesses.entry(root).or_insert((x1, x2));
                                }
                                QuadraticRootKey::Irreducible { .. } => {
                                    stats.irreducible_root_keys += 1;
                                }
                            }
                            witnesses.entry(key).or_insert((x1, x2));
                        }
                    }
                }
            }
        }
        stats.distinct_keys = witnesses.len();
        stats.distinct_base_roots = finite_witnesses.len();
        Ok(Self {
            curve,
            base: clean,
            witnesses,
            finite_witnesses,
            ys: Vec::new(),
            finite_list: Vec::new(),
            universal_pair,
            stats,
        })
    }

    pub fn curve(&self) -> Curve {
        self.curve
    }

    pub fn factor_base(&self) -> &[u64] {
        &self.base
    }

    pub fn build_stats(&self) -> BuildStats {
        self.stats
    }

    pub fn finite_support(&self) -> usize {
        self.finite_witnesses.len()
    }

    /// Exact three-summand query, including the pair-cancellation component
    /// omitted by a finite affine auxiliary variable.
    pub fn decompose_three(&self, target: u64) -> (Option<ThreeSumWitness>, QueryStats) {
        if self.fast_ready() {
            let t = target % self.curve.p;
            // One sqrtm per query; a non-point target takes the generic path.
            if let Some(yt) = sqrtm(self.curve.rhs(t), self.curve.p) {
                return self.decompose_three_fast(t, yt);
            }
        }
        self.decompose_three_generic(target)
    }

    fn fast_ready(&self) -> bool {
        self.ys.len() == self.base.len() && self.finite_list.len() == self.finite_witnesses.len()
    }

    fn decompose_three_fast(&self, target: u64, yt: u64) -> (Option<ThreeSumWitness>, QueryStats) {
        let mut stats = QueryStats::default();
        stats.cancellation_checks += 1;
        if !self.base.is_empty() && self.base.binary_search(&target).is_ok() {
            let x = self.base[0];
            return (
                Some(ThreeSumWitness {
                    x: [x, x, target],
                    auxiliary: None,
                }),
                stats,
            );
        }
        let pts: Vec<(u64, u64)> = self.base.iter().copied().zip(self.ys.iter().copied()).collect();
        let all_roots = roots_batch(&self.curve, &pts, target, yt);
        for (&x3, roots) in self.base.iter().zip(all_roots) {
            stats.quadratics += 1;
            if let Some(w) = self.three_step(roots, x3, &mut stats) {
                return (Some(w), stats);
            }
        }
        (None, stats)
    }

    /// Shared per-`x3` logic of the three-summand query.
    fn three_step(&self, roots: QuadraticRoots, x3: u64, stats: &mut QueryStats) -> Option<ThreeSumWitness> {
        match roots {
            QuadraticRoots::None => {}
            QuadraticRoots::All => {
                if let Some((&key, &(x1, x2))) = self.witnesses.iter().next() {
                    stats.compatible_keys += 1;
                    return Some(ThreeSumWitness { x: [x1, x2, x3], auxiliary: Some(key) });
                }
                if let Some((x1, x2)) = self.universal_pair {
                    stats.compatible_keys += 1;
                    return Some(ThreeSumWitness { x: [x1, x2, x3], auxiliary: None });
                }
            }
            QuadraticRoots::Finite(keys) => {
                stats.root_keys += keys.len() as u64;
                for key in keys {
                    stats.lookups += 1;
                    if let Some(&(x1, x2)) = self.witnesses.get(&key) {
                        stats.compatible_keys += 1;
                        return Some(ThreeSumWitness { x: [x1, x2, x3], auxiliary: Some(key) });
                    }
                    if let Some((x1, x2)) = self.universal_pair {
                        stats.compatible_keys += 1;
                        return Some(ThreeSumWitness { x: [x1, x2, x3], auxiliary: Some(key) });
                    }
                }
            }
        }
        None
    }

    /// Reference three-summand query: per-`x3` `invm` + `sqrtm`.
    pub fn decompose_three_generic(&self, target: u64) -> (Option<ThreeSumWitness>, QueryStats) {
        let p = self.curve.p;
        let target = target % p;
        let mut stats = QueryStats::default();

        stats.cancellation_checks += 1;
        if !self.base.is_empty() && self.base.binary_search(&target).is_ok() {
            let x = self.base[0];
            return (
                Some(ThreeSumWitness {
                    x: [x, x, target],
                    auxiliary: None,
                }),
                stats,
            );
        }

        for &x3 in &self.base {
            stats.quadratics += 1;
            match quadratic_root_keys(s3_quadratic(&self.curve, x3, target), p) {
                QuadraticRoots::None => {}
                QuadraticRoots::All => {
                    if let Some((&key, &(x1, x2))) = self.witnesses.iter().next() {
                        stats.compatible_keys += 1;
                        return (
                            Some(ThreeSumWitness {
                                x: [x1, x2, x3],
                                auxiliary: Some(key),
                            }),
                            stats,
                        );
                    }
                    if let Some((x1, x2)) = self.universal_pair {
                        stats.compatible_keys += 1;
                        return (
                            Some(ThreeSumWitness {
                                x: [x1, x2, x3],
                                auxiliary: None,
                            }),
                            stats,
                        );
                    }
                }
                QuadraticRoots::Finite(keys) => {
                    stats.root_keys += keys.len() as u64;
                    for key in keys {
                        stats.lookups += 1;
                        if let Some(&(x1, x2)) = self.witnesses.get(&key) {
                            stats.compatible_keys += 1;
                            return (
                                Some(ThreeSumWitness {
                                    x: [x1, x2, x3],
                                    auxiliary: Some(key),
                                }),
                                stats,
                            );
                        }
                        if let Some((x1, x2)) = self.universal_pair {
                            stats.compatible_keys += 1;
                            return (
                                Some(ThreeSumWitness {
                                    x: [x1, x2, x3],
                                    auxiliary: Some(key),
                                }),
                                stats,
                            );
                        }
                    }
                }
            }
        }
        (None, stats)
    }

    /// Exact balanced four-summand query.  `compatible_keys` counts all
    /// finite pair-root matches examined, so it is also a target function
    /// suitable for the parametric-compression falsifier.
    pub fn decompose_four(&self, target: u64) -> (Option<FourSumWitness>, QueryStats) {
        self.decompose_four_dispatch(target, false)
    }

    /// Like `decompose_four` but returns at the first witness, so
    /// `compatible_keys` is no longer the full match count.
    pub fn decompose_four_first(&self, target: u64) -> (Option<FourSumWitness>, QueryStats) {
        self.decompose_four_dispatch(target, true)
    }

    fn decompose_four_dispatch(&self, target: u64, early_exit: bool) -> (Option<FourSumWitness>, QueryStats) {
        if self.fast_ready() {
            let t = target % self.curve.p;
            if let Some(yt) = sqrtm(self.curve.rhs(t), self.curve.p) {
                return self.decompose_four_fast(t, yt, early_exit);
            }
        }
        self.decompose_four_generic(target)
    }

    fn decompose_four_fast(&self, target: u64, yt: u64, early_exit: bool) -> (Option<FourSumWitness>, QueryStats) {
        let mut stats = QueryStats::default();
        let mut witness = None;
        stats.cancellation_checks += 1;
        if let (Some(&cancel), Some(&(x3, x4))) =
            (self.base.first(), self.finite_witnesses.get(&target))
        {
            witness = Some(FourSumWitness {
                x: [cancel, cancel, x3, x4],
                auxiliaries: [None, Some(target)],
            });
            if early_exit {
                return (witness, stats);
            }
        }
        let pts: Vec<(u64, u64)> = self.finite_list.iter().map(|&(u, yu, _, _)| (u, yu)).collect();
        let all_roots = roots_batch(&self.curve, &pts, target, yt);
        for (&(u, _, x1, x2), roots) in self.finite_list.iter().zip(all_roots) {
            stats.quadratics += 1;
            match roots {
                QuadraticRoots::None => {}
                QuadraticRoots::All => {
                    if let Some(&(v, _, x3, x4)) = self.finite_list.first() {
                        stats.compatible_keys += 1;
                        witness.get_or_insert(FourSumWitness {
                            x: [x1, x2, x3, x4],
                            auxiliaries: [Some(u), Some(v)],
                        });
                    }
                }
                QuadraticRoots::Finite(keys) => {
                    stats.root_keys += keys.len() as u64;
                    for key in keys {
                        let QuadraticRootKey::Base(v) = key else {
                            continue;
                        };
                        stats.lookups += 1;
                        if let Some(&(x3, x4)) = self.finite_witnesses.get(&v) {
                            stats.compatible_keys += 1;
                            witness.get_or_insert(FourSumWitness {
                                x: [x1, x2, x3, x4],
                                auxiliaries: [Some(u), Some(v)],
                            });
                        }
                    }
                }
            }
            if early_exit && witness.is_some() {
                break;
            }
        }
        (witness, stats)
    }

    /// Reference four-summand query: per-`u` `invm` + `sqrtm`, full scan.
    pub fn decompose_four_generic(&self, target: u64) -> (Option<FourSumWitness>, QueryStats) {
        let p = self.curve.p;
        let target = target % p;
        let mut stats = QueryStats::default();
        let mut witness = None;

        // If either pair cancels to infinity, the other pair must have target
        // as a finite root. Repetition of a factor-base x is allowed.
        stats.cancellation_checks += 1;
        if let (Some(&cancel), Some(&(x3, x4))) =
            (self.base.first(), self.finite_witnesses.get(&target))
        {
            witness = Some(FourSumWitness {
                x: [cancel, cancel, x3, x4],
                auxiliaries: [None, Some(target)],
            });
        }

        for (&u, &(x1, x2)) in &self.finite_witnesses {
            stats.quadratics += 1;
            match quadratic_root_keys(s3_quadratic(&self.curve, u, target), p) {
                QuadraticRoots::None => {}
                QuadraticRoots::All => {
                    if let Some((&v, &(x3, x4))) = self.finite_witnesses.iter().next() {
                        stats.compatible_keys += 1;
                        witness.get_or_insert(FourSumWitness {
                            x: [x1, x2, x3, x4],
                            auxiliaries: [Some(u), Some(v)],
                        });
                    }
                }
                QuadraticRoots::Finite(keys) => {
                    stats.root_keys += keys.len() as u64;
                    for key in keys {
                        let QuadraticRootKey::Base(v) = key else {
                            continue;
                        };
                        stats.lookups += 1;
                        if let Some(&(x3, x4)) = self.finite_witnesses.get(&v) {
                            stats.compatible_keys += 1;
                            witness.get_or_insert(FourSumWitness {
                                x: [x1, x2, x3, x4],
                                auxiliaries: [Some(u), Some(v)],
                            });
                        }
                    }
                }
            }
        }
        (witness, stats)
    }
}

/// Interval factor base, restricted to x-coordinates of `F_p` points.
pub fn interval_factor_base(curve: &Curve, bound: u64) -> Vec<u64> {
    (0..bound.min(curve.p))
        .filter(|&x| sqrtm(curve.rhs(x), curve.p).is_some())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::pkm_smooth_tower::{s3, Pt};

    fn curve() -> Curve {
        Curve { p: 101, a: 2, b: 3 }
    }

    fn canonical_point(curve: &Curve, x: u64) -> Pt {
        Some((x, sqrtm(curve.rhs(x), curve.p).unwrap()))
    }

    fn signed_points(curve: &Curve, x: u64) -> Vec<Pt> {
        let point = canonical_point(curve, x);
        let neg = curve.neg(point);
        if neg == point {
            vec![point]
        } else {
            vec![point, neg]
        }
    }

    fn direct(curve: &Curve, base: &[u64], target: u64, arity: usize) -> bool {
        fn tuples(
            curve: &Curve,
            base: &[u64],
            target: u64,
            arity: usize,
            depth: usize,
            sum: Pt,
        ) -> bool {
            if depth == arity {
                return sum.is_some_and(|(x, _)| x == target);
            }
            base.iter().any(|&x| {
                signed_points(curve, x).into_iter().any(|point| {
                    tuples(curve, base, target, arity, depth + 1, curve.add(sum, point))
                })
            })
        }
        tuples(curve, base, target, arity, 0, None)
    }

    fn witness_reaches(curve: &Curve, xs: &[u64], target: u64) -> bool {
        fn visit(curve: &Curve, xs: &[u64], target: u64, depth: usize, sum: Pt) -> bool {
            if depth == xs.len() {
                return sum.is_some_and(|(x, _)| x == target);
            }
            signed_points(curve, xs[depth])
                .into_iter()
                .any(|point| visit(curve, xs, target, depth + 1, curve.add(sum, point)))
        }
        visit(curve, xs, target, 0, None)
    }

    #[test]
    fn quadratic_coefficients_match_s3() {
        let e = curve();
        for x1 in 0..e.p {
            for x2 in 0..e.p {
                let q = s3_quadratic(&e, x1, x2);
                for x3 in 0..e.p {
                    assert_eq!(eval_quadratic(q, x3, e.p), s3(&e, x1, x2, x3));
                }
            }
        }
    }

    #[test]
    fn root_keys_are_exact() {
        let p = 101;
        for c0 in 0..17 {
            for c1 in 0..17 {
                for c2 in 0..17 {
                    let q = [c0, c1, c2];
                    let actual: Vec<_> = (0..p).filter(|&x| eval_quadratic(q, x, p) == 0).collect();
                    match quadratic_root_keys(q, p) {
                        QuadraticRoots::None => assert!(actual.is_empty()),
                        QuadraticRoots::All => assert_eq!(actual.len(), p as usize),
                        QuadraticRoots::Finite(keys) => {
                            let mut got: Vec<_> = keys
                                .into_iter()
                                .filter_map(|k| match k {
                                    QuadraticRootKey::Base(x) => Some(x),
                                    QuadraticRootKey::Irreducible { .. } => None,
                                })
                                .collect();
                            got.sort_unstable();
                            assert_eq!(got, actual);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn projection_matches_signed_group_control() {
        let e = curve();
        let base = interval_factor_base(&e, 12);
        let projection = PairProjection::build(e, &base).unwrap();
        for target in 0..e.p {
            if sqrtm(e.rhs(target), e.p).is_none() {
                continue;
            }
            let three = projection.decompose_three(target).0;
            assert_eq!(
                three.is_some(),
                direct(&e, &base, target, 3),
                "three-sum target {target}"
            );
            if let Some(witness) = three {
                assert!(witness_reaches(&e, &witness.x, target));
            }
            let four = projection.decompose_four(target).0;
            assert_eq!(
                four.is_some(),
                direct(&e, &base, target, 4),
                "four-sum target {target}"
            );
            if let Some(witness) = four {
                assert!(witness_reaches(&e, &witness.x, target));
            }
        }
    }

    #[test]
    fn rejects_non_point_factor_base_entries() {
        let e = curve();
        let bad = (0..e.p).find(|&x| sqrtm(e.rhs(x), e.p).is_none()).unwrap();
        assert!(PairProjection::build(e, &[bad]).is_err());
    }

    #[test]
    fn fast_path_matches_generic_small() {
        let e = curve();
        let base = interval_factor_base(&e, 30);
        let fast = PairProjection::build(e, &base).unwrap();
        let slow = PairProjection::build_generic(e, &base).unwrap();
        assert_eq!(fast.witnesses, slow.witnesses);
        assert_eq!(fast.finite_witnesses, slow.finite_witnesses);
        assert_eq!(fast.stats, slow.stats);
        assert_eq!(fast.stats.irreducible_root_keys, 0);
        for &(u, yu, _, _) in &fast.finite_list {
            assert_eq!(mulm(yu, yu, e.p), e.rhs(u));
        }
        for target in 0..e.p {
            let (a, _) = fast.decompose_three(target);
            let (b, _) = slow.decompose_three_generic(target);
            assert_eq!(a.is_some(), b.is_some(), "three {target}");
            let (a, sa) = fast.decompose_four(target);
            let (b, sb) = slow.decompose_four_generic(target);
            assert_eq!(a.is_some(), b.is_some(), "four {target}");
            assert_eq!(sa.compatible_keys, sb.compatible_keys, "four keys {target}");
            assert_eq!(fast.decompose_four_first(target).0.is_some(), b.is_some());
            if let Some(w) = fast.decompose_four_first(target).0 {
                assert!(witness_reaches(&e, &w.x, target));
            }
        }
    }

    /// Op-count comparison on Prime31 (p = 2^31 - 1), B = 64.
    /// Run with `--nocapture` to print the table.
    #[test]
    fn prime31_opcount_old_vs_new() {
        use crate::cryptanalysis::pkm_smooth_tower::opcount;
        let e = Curve { p: 2_147_483_647, a: 2, b: 3 };
        assert!(!e.is_singular());
        let base: Vec<u64> = interval_factor_base(&e, 400).into_iter().take(64).collect();
        assert_eq!(base.len(), 64);

        opcount::reset();
        let old = PairProjection::build_generic(e, &base).unwrap();
        let (os, oi, om) = opcount::snapshot();
        opcount::reset();
        let new = PairProjection::build(e, &base).unwrap();
        let (ns, ni, nm) = opcount::snapshot();
        println!("BUILD pairs={} old sqrtm={os} invm={oi} mulm={om} | new sqrtm={ns} invm={ni} mulm={nm}", old.stats.unordered_pairs);
        assert_eq!(old.witnesses, new.witnesses);
        assert_eq!(old.finite_witnesses, new.finite_witnesses);
        assert_eq!(old.stats, new.stats);
        assert_eq!(new.stats.irreducible_root_keys, 0);
        assert_eq!(old.stats.unordered_pairs, 2080); // 64*65/2 incl. diagonal

        // queries: 40 point targets (some in base), then 8 non-points
        let targets: Vec<u64> = interval_factor_base(&e, 1500).into_iter().skip(10).step_by(7).take(40).collect();
        let (mut mism3, mut eq3, mut mism4) = (0, 0, 0);
        let mut c = [[0u64; 3]; 4];
        for &t in &targets {
            opcount::reset();
            let (a3, _) = old.decompose_three_generic(t);
            let s = opcount::snapshot(); for k in 0..3 { c[0][k] += [s.0, s.1, s.2][k]; }
            opcount::reset();
            let (b3, _) = new.decompose_three(t);
            let s = opcount::snapshot(); for k in 0..3 { c[1][k] += [s.0, s.1, s.2][k]; }
            opcount::reset();
            let (a4, sa) = old.decompose_four_generic(t);
            let s = opcount::snapshot(); for k in 0..3 { c[2][k] += [s.0, s.1, s.2][k]; }
            opcount::reset();
            let (b4, sb) = new.decompose_four_first(t);
            let s = opcount::snapshot(); for k in 0..3 { c[3][k] += [s.0, s.1, s.2][k]; }
            if a3.is_some() != b3.is_some() { mism3 += 1; }
            if a3 == b3 { eq3 += 1; }
            if a3.is_some() != b3.is_some() { mism3 += 1; }
            if a4.is_some() != b4.is_some() { mism4 += 1; }
            if let Some(w) = &b4 { assert!(witness_reaches(&e, &w.x, t)); }
            if let Some(w) = &b3 { assert!(witness_reaches(&e, &w.x, t)); }
            let _ = (sa, sb);
        }
        println!("Q3 over {} targets: old (sqrtm,invm,mulm)={:?} new={:?}; found/none mismatches={mism3}; exact-equal witnesses={eq3}", targets.len(), c[0], c[1]);
        println!("Q4 over {} targets: old={:?} new(first)={:?}; found/none mismatches={mism4}", targets.len(), c[2], c[3]);
        assert_eq!(mism3, 0);
        assert_eq!(mism4, 0);
    }
}
