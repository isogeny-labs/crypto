//! Relation lattice of small split prime ideals and the cheapest isogeny
//! loops under a weighted (possibly asymmetric) 1-norm.
//!
//! For an imaginary quadratic order with class group `Cl` and prime ideals
//! `𝔩_1, …, 𝔩_k` above split primes `ℓ_i ≤ L`, the relation lattice is
//! `Λ = { e ∈ Z^k : ∏ [𝔩_i]^{e_i} = 1 }`.  A nonzero `e ∈ Λ` is a closed
//! walk of horizontal isogenies (an isogeny loop), i.e. an endomorphism
//! `α` with `(α) = ∏ 𝔩_i^{e_i}` (`𝔩^{−1}` read as the conjugate `𝔩̄`), of
//! norm `∏ ℓ_i^{|e_i|}`, costing `Σ c_i(sign e_i) |e_i|` field
//! multiplications (Koshelev–Sanso, ePrint 2024/1985).  This module ports the
//! search of `scripts/isogeny_loop_sweeper.py`:
//!
//! * the basis of `Λ` in PARI `mathnf` form (upper triangular columns);
//! * exact integral LLL (de Weger / Cohen GTM 138 Algorithm 2.6.7,
//!   `δ = 99/100`, the default of PARI's `qflllgram`) on the weighted Gram
//!   matrix `Bᵀ W B` — an upper bound on the cheapest loop;
//! * an exact depth-first enumeration of the weighted 1-norm ball of radius
//!   `R` with asymmetric (forward, backward) step costs, membership tested
//!   through the class-group discrete logs;
//! * the generator `α = (A + B√D)/2` of `∏ 𝔩_i^{e_i}`, computed by
//!   composing the prime forms without reduction (giving the form
//!   `(N, b, c)` of the ideal), reducing it to the principal form while
//!   tracking the `SL₂(Z)` transform, and reading off a representation of 1;
//!   the result is verified (`A² − D B² = 4N` and `α ∈ ∏ 𝔩_i^{e_i}`).

use super::class_group_subexp::{
    compose_unreduced, reduce_tracking, Hnf, SubexpClassGroup,
};
use super::class_group::BinaryQuadraticForm;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// Isogeny step-cost model.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CostModel {
    /// `7.5ℓ` M per ℓ-step (paper, Section 2.2: projective Vélu, Horner).
    Projective,
    /// `2ℓ + 2` M (Kohel-affine, inversion batched and not charged).
    Affine,
    /// `2ℓ` M for odd ℓ (Costello–Hisil x-only odd-degree evaluation,
    /// `(2ℓ − 2)M + 2S` with `S = M`); `c(2) = 4` M in odd characteristic.
    XOnly,
}

impl CostModel {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "projective" => Some(CostModel::Projective),
            "affine" => Some(CostModel::Affine),
            "xonly" | "x-only" => Some(CostModel::XOnly),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            CostModel::Projective => "projective",
            CostModel::Affine => "affine",
            CostModel::XOnly => "xonly",
        }
    }
}

/// Step costs `(forward, backward)` in **half-M units**.  In characteristic 2
/// and `ℓ = 2` the forward direction is the prime containing `π` (Frobenius
/// `F`, 2 M) and the backward one its conjugate (Verschiebung
/// `V = [2]∘F⁻¹`, `c_dbl + 2` M), in every model.
pub fn step_cost_half(ell: u64, char2: bool, model: CostModel, dbl_half: u64) -> (u64, u64) {
    if char2 && ell == 2 {
        return (4, dbl_half + 4);
    }
    let c = match model {
        CostModel::Projective => 15 * ell,
        CostModel::Affine => 4 * ell + 4,
        CostModel::XOnly => {
            if ell == 2 {
                8
            } else {
                4 * ell
            }
        }
    };
    (c, c)
}

/// One split prime of the loop lattice, with its orientation.
#[derive(Clone, Debug)]
pub struct LoopPrime {
    pub ell: u64,
    /// Index in the class-group factor base.
    pub fb_index: usize,
    /// `b` of the oriented ideal `𝔩 = (ℓ, (−b + √D)/2)` (`e > 0` direction).
    pub b: BigInt,
    /// Discrete log of the oriented ideal.
    pub dlog: Vec<BigInt>,
    /// Forward / backward step costs (half-M units).
    pub wf: u64,
    pub wb: u64,
}

impl LoopPrime {
    pub fn cost(&self, x: i64) -> u64 {
        if x >= 0 {
            self.wf * x as u64
        } else {
            self.wb * x.unsigned_abs()
        }
    }
}

/// Does `α = (A + B√D)/2` lie in the ideal `(m, (−b + √D)/2)`?
/// (`α = B·(−b+√D)/2 + (A + Bb)/2`, so iff `(A + Bb)/2 ≡ 0 mod m`.)
pub fn in_ideal(a: &BigInt, b_coef: &BigInt, ideal_b: &BigInt, m: &BigInt) -> bool {
    let t = a + b_coef * ideal_b;
    t.mod_floor(&(m * 2)).is_zero()
}

/// The split primes `ℓ ≤ bound` of the factor base, oriented by default as the
/// factor-base ideal; `flip(ℓ)` returns true to use the conjugate instead.
pub fn loop_primes(
    cg: &SubexpClassGroup,
    bound: u64,
    char2: bool,
    model: CostModel,
    dbl_half: u64,
    flip: impl Fn(&super::class_group_subexp::FbPrime) -> bool,
) -> Vec<LoopPrime> {
    let mut out = Vec::new();
    for (i, f) in cg.fb.iter().enumerate() {
        if f.p > bound || f.ramified {
            continue;
        }
        let fl = flip(f);
        let (b, dlog) = if fl {
            (
                -f.b.clone(),
                cg.dlogs[i]
                    .iter()
                    .zip(&cg.invariants)
                    .map(|(x, d)| (-x).mod_floor(d))
                    .collect(),
            )
        } else {
            (f.b.clone(), cg.dlogs[i].clone())
        };
        let (wf, wb) = step_cost_half(f.p, char2, model, dbl_half);
        out.push(LoopPrime {
            ell: f.p,
            fb_index: i,
            b,
            dlog,
            wf,
            wb,
        });
    }
    out
}

/// Basis of `Λ = {e : Σ e_i dlog_i ≡ 0 mod invariants}` in PARI `mathnf`
/// form: basis vector `j` has its last nonzero coordinate at `j`, positive
/// there, and coordinate `i < j` reduced into `[0, b_i[i])`.
pub fn relation_lattice_basis(dlogs: &[Vec<BigInt>], invariants: &[BigInt]) -> Vec<Vec<BigInt>> {
    let k = dlogs.len();
    let m = invariants.len();
    if m == 0 {
        return (0..k)
            .map(|j| {
                (0..k)
                    .map(|i| if i == j { BigInt::one() } else { BigInt::zero() })
                    .collect()
            })
            .collect();
    }
    // columns: [m class-group coordinates | k reversed loop coordinates]
    let n = m + k;
    let mut hnf = Hnf::new(n);
    for t in 0..m {
        let mut v = vec![BigInt::zero(); n];
        v[t] = invariants[t].clone();
        hnf.insert(v);
    }
    for (i, d) in dlogs.iter().enumerate() {
        let mut v = vec![BigInt::zero(); n];
        v[..m].clone_from_slice(&d[..m]);
        v[m + (k - 1 - i)] = BigInt::one();
        hnf.insert(v);
    }
    hnf.finish();
    let mut basis = Vec::with_capacity(k);
    for j in 0..k {
        let row = hnf.rows[m + (k - 1 - j)]
            .as_ref()
            .expect("relation lattice has full rank");
        assert!(row[..m].iter().all(|x| x.is_zero()));
        let v: Vec<BigInt> = (0..k).map(|i| row[m + (k - 1 - i)].clone()).collect();
        basis.push(v);
    }
    basis
}

/// Exact integral LLL (Cohen, GTM 138, Algorithm 2.6.7) with `δ = 99/100`
/// for the inner product `⟨x, y⟩ = Σ w_i x_i y_i`.  Returns the reduced basis.
pub fn lll_weighted(basis: &[Vec<BigInt>], w: &[u64]) -> Vec<Vec<BigInt>> {
    let n = basis.len();
    let mut b: Vec<Vec<BigInt>> = basis.to_vec();
    if n <= 1 {
        return b;
    }
    let ip = |x: &[BigInt], y: &[BigInt]| -> BigInt {
        let mut s = BigInt::zero();
        for i in 0..x.len() {
            if !x[i].is_zero() && !y[i].is_zero() {
                s += &x[i] * &y[i] * w[i];
            }
        }
        s
    };
    // d[0] = 1, d[i+1] for vector i; lambda[k][j] for j < k
    let mut d = vec![BigInt::zero(); n + 1];
    let mut lam = vec![vec![BigInt::zero(); n]; n];
    d[0] = BigInt::one();
    d[1] = ip(&b[0], &b[0]);
    let mut kmax = 0usize;
    let mut k = 1usize;
    let (dp, dq) = (BigInt::from(99), BigInt::from(100));

    fn redi(
        k: usize,
        l: usize,
        b: &mut [Vec<BigInt>],
        d: &[BigInt],
        lam: &mut [Vec<BigInt>],
    ) {
        let two: BigInt = &lam[k][l] * 2;
        if two.abs() > d[l + 1] {
            // q = round(lam / d)
            let num: BigInt = &lam[k][l] * 2 + &d[l + 1];
            let den: BigInt = &d[l + 1] * 2;
            let q = num.div_floor(&den);
            let bl = b[l].clone();
            for (x, y) in b[k].iter_mut().zip(&bl) {
                *x -= &q * y;
            }
            lam[k][l] -= &q * &d[l + 1];
            for i in 0..l {
                let t = &q * &lam[l][i];
                lam[k][i] -= t;
            }
        }
    }

    while k < n {
        if k > kmax {
            kmax = k;
            for j in 0..=k {
                let mut u = ip(&b[k], &b[j]);
                for i in 0..j {
                    u = (&d[i + 1] * &u - &lam[k][i] * &lam[j][i]) / &d[i];
                }
                if j < k {
                    lam[k][j] = u;
                } else {
                    d[k + 1] = u;
                    assert!(!d[k + 1].is_zero(), "LLL input is linearly dependent");
                }
            }
        }
        loop {
            redi(k, k - 1, &mut b, &d, &mut lam);
            // Lovász: 100 d_{k+1} d_{k-1} < 99 d_k^2 - 100 lam^2  => swap
            let lhs = &dq * &d[k + 1] * &d[k - 1];
            let rhs = &dp * &d[k] * &d[k] - &dq * &lam[k][k - 1] * &lam[k][k - 1];
            if lhs < rhs {
                // SWAPI(k)
                b.swap(k, k - 1);
                for j in 0..k - 1 {
                    let t = lam[k][j].clone();
                    lam[k][j] = lam[k - 1][j].clone();
                    lam[k - 1][j] = t;
                }
                let l = lam[k][k - 1].clone();
                let bb = (&d[k - 1] * &d[k + 1] + &l * &l) / &d[k];
                for i in k + 1..=kmax {
                    let t = lam[i][k].clone();
                    lam[i][k] = (&d[k + 1] * &lam[i][k - 1] - &l * &t) / &d[k];
                    lam[i][k - 1] = (&bb * &t + &l * &lam[i][k]) / &d[k + 1];
                }
                d[k] = bb;
                if k > 1 {
                    k -= 1;
                }
                continue;
            }
            for l in (0..k - 1).rev() {
                redi(k, l, &mut b, &d, &mut lam);
            }
            k += 1;
            break;
        }
    }
    b
}

/// Result of [`weighted_shortest`].
#[derive(Clone, Debug)]
pub struct LoopSearch {
    /// Ranked candidates `(cost in half-M units, exponent vector)`.
    pub ranked: Vec<(u64, Vec<i64>)>,
    pub exhaustive: bool,
    /// Enumeration radius (half-M units).
    pub radius: u64,
    /// Cheapest LLL basis vector (half-M units).
    pub lll_best: u64,
    pub nodes: u64,
}

fn cost_of(primes: &[LoopPrime], e: &[i64]) -> u64 {
    primes.iter().zip(e).map(|(p, &x)| p.cost(x)).sum()
}

/// Cheapest nonzero vectors of `Λ` in the weighted 1-norm.
///
/// Stage 1: LLL on the Gram matrix `Bᵀ W B` with `W_i = w_f + w_b` gives an
/// upper bound.  Stage 2: exact DFS over the ball
/// `{v : Σ c_i(v_i) ≤ R}`, `R = min(LLL bound, radius_cap)`, with membership
/// through the discrete logs; exhaustive unless `node_cap` is hit.  Both a
/// candidate and its negative are kept (conjugate loops).
pub fn weighted_shortest(
    primes: &[LoopPrime],
    invariants: &[BigInt],
    radius_cap: Option<u64>,
    node_cap: u64,
) -> LoopSearch {
    let k = primes.len();
    let dlogs: Vec<Vec<BigInt>> = primes.iter().map(|p| p.dlog.clone()).collect();
    let basis = relation_lattice_basis(&dlogs, invariants);
    let w: Vec<u64> = primes.iter().map(|p| p.wf + p.wb).collect();
    let red = lll_weighted(&basis, &w);
    let mut cands: HashMap<Vec<i64>, u64> = HashMap::new();
    let mut lll_best = u64::MAX;
    for v in &red {
        let Some(e) = v.iter().map(|x| x.to_i64()).collect::<Option<Vec<i64>>>() else {
            continue;
        };
        let c = cost_of(primes, &e).min(cost_of(primes, &e.iter().map(|x| -x).collect::<Vec<_>>()));
        lll_best = lll_best.min(c);
        cands.insert(e.clone(), cost_of(primes, &e));
    }
    let radius = match radius_cap {
        Some(r) => r.min(lll_best),
        None => lll_best,
    };
    // DFS, heaviest coordinates first
    let m = invariants.len();
    let inv: Vec<i128> = invariants
        .iter()
        .map(|x| x.to_i128().expect("invariant fits i128"))
        .collect();
    let dli: Vec<Vec<i128>> = primes
        .iter()
        .map(|p| p.dlog.iter().map(|x| x.to_i128().unwrap()).collect())
        .collect();
    let mut order: Vec<usize> = (0..k).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(primes[i].wf.min(primes[i].wb)));
    struct Dfs<'a> {
        primes: &'a [LoopPrime],
        order: Vec<usize>,
        inv: Vec<i128>,
        dli: Vec<Vec<i128>>,
        m: usize,
        v: Vec<i64>,
        nodes: u64,
        cap: u64,
        hit_cap: bool,
        found: Vec<Vec<i64>>,
    }
    impl Dfs<'_> {
        fn run(&mut self, pos: usize, budget: u64, acc: &[i128]) {
            if self.hit_cap {
                return;
            }
            let k = self.order.len();
            if pos == k {
                if self.v.iter().any(|&x| x != 0) && acc.iter().all(|&a| a == 0) {
                    self.found.push(self.v.clone());
                }
                return;
            }
            let i = self.order[pos];
            let p = &self.primes[i];
            let lo = -((budget / p.wb) as i64);
            let hi = (budget / p.wf) as i64;
            let mut cur: Vec<i128> = (0..self.m)
                .map(|t| (acc[t] + (lo as i128) * self.dli[i][t]).rem_euclid(self.inv[t]))
                .collect();
            for x in lo..=hi {
                self.nodes += 1;
                if self.nodes > self.cap {
                    self.hit_cap = true;
                    return;
                }
                self.v[i] = x;
                let c = p.cost(x);
                self.run(pos + 1, budget - c, &cur);
                if self.hit_cap {
                    return;
                }
                for t in 0..self.m {
                    cur[t] += self.dli[i][t];
                    if cur[t] >= self.inv[t] {
                        cur[t] -= self.inv[t];
                    }
                }
            }
            self.v[i] = 0;
        }
    }
    let mut dfs = Dfs {
        primes,
        order,
        inv,
        dli,
        m,
        v: vec![0; k],
        nodes: 0,
        cap: node_cap,
        hit_cap: false,
        found: Vec::new(),
    };
    let acc0 = vec![0i128; m];
    dfs.run(0, radius, &acc0);
    for e in dfs.found {
        let c = cost_of(primes, &e);
        cands.insert(e, c);
    }
    let keys: Vec<Vec<i64>> = cands.keys().cloned().collect();
    for e in keys {
        let ne: Vec<i64> = e.iter().map(|x| -x).collect();
        cands.entry(ne.clone()).or_insert_with(|| cost_of(primes, &ne));
    }
    let mut ranked: Vec<(u64, Vec<i64>)> = cands.into_iter().map(|(e, c)| (c, e)).collect();
    ranked.sort_by(|a, b| {
        let la: u64 = a.1.iter().map(|x| x.unsigned_abs()).sum();
        let lb: u64 = b.1.iter().map(|x| x.unsigned_abs()).sum();
        a.0.cmp(&b.0)
            .then(la.cmp(&lb))
            .then_with(|| {
                let na: Vec<i64> = a.1.iter().map(|x| -x).collect();
                let nb: Vec<i64> = b.1.iter().map(|x| -x).collect();
                na.cmp(&nb)
            })
    });
    LoopSearch {
        ranked,
        exhaustive: !dfs.hit_cap,
        radius,
        lll_best,
        nodes: dfs.nodes,
    }
}

/// A verified generator `α = (A + B√D)/2` of `∏ 𝔩_i^{e_i}`.
#[derive(Clone, Debug)]
pub struct LoopGenerator {
    pub a: BigInt,
    pub b: BigInt,
    pub norm: BigInt,
}

/// Generator of the principal ideal `∏ 𝔩_i^{e_i}` (see module docs).
pub fn loop_generator(
    disc: &BigInt,
    primes: &[LoopPrime],
    e: &[i64],
) -> Result<LoopGenerator, String> {
    let mut f: Option<BinaryQuadraticForm> = None;
    let mut norm = BigInt::one();
    for (p, &x) in primes.iter().zip(e) {
        if x == 0 {
            continue;
        }
        let ell = BigInt::from(p.ell);
        let b = if x > 0 { p.b.clone() } else { -p.b.clone() };
        let c = (&b * &b - disc) / (&ell * 4);
        let g = BinaryQuadraticForm {
            a: ell.clone(),
            b,
            c,
        };
        for _ in 0..x.unsigned_abs() {
            norm *= &ell;
            f = Some(match f {
                None => g.clone(),
                Some(h) => compose_unreduced(&h, &g, disc),
            });
        }
    }
    let Some(f) = f else {
        return Err("zero exponent vector".into());
    };
    if f.a != norm {
        return Err(format!("composed form has a = {} != N = {}", f.a, norm));
    }
    let (red, m) = reduce_tracking(&f);
    if !red.a.is_one() {
        return Err("ideal is not principal (reduced form has a != 1)".into());
    }
    let (x, y) = (&m[0], &m[2]);
    // f(x, y) = 1;  beta = x N - y (-b + sqrt D)/2 has norm N f(x, y) = N
    let a_coef = &norm * x * 2 + y * &f.b;
    let b_coef = -y.clone();
    let lhs = &a_coef * &a_coef - disc * &b_coef * &b_coef;
    if lhs != &norm * 4 {
        return Err("A^2 - D B^2 != 4N".into());
    }
    if !in_ideal(&a_coef, &b_coef, &f.b, &norm) {
        return Err("alpha not in the ideal".into());
    }
    for (p, &x) in primes.iter().zip(e) {
        if x == 0 {
            continue;
        }
        let b = if x > 0 { p.b.clone() } else { -p.b.clone() };
        let m = BigInt::from(p.ell);
        if !in_ideal(&a_coef, &b_coef, &b, &m) {
            return Err(format!("alpha not in the prime above {}", p.ell));
        }
    }
    Ok(LoopGenerator {
        a: a_coef,
        b: b_coef,
        norm,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isogeny::class_group_subexp::{compute_class_group, SubexpParams};

    fn params() -> SubexpParams {
        SubexpParams {
            trials_per_task: 512,
            euler_bound: 1 << 16,
            min_fb_bound: 100,
            parallel: false,
            ..Default::default()
        }
    }

    #[test]
    fn lll_reduces_small_example() {
        let basis: Vec<Vec<BigInt>> = vec![
            vec![1, 0, 0].into_iter().map(BigInt::from).collect(),
            vec![0, 1, 0].into_iter().map(BigInt::from).collect(),
            vec![1000, 1001, 1].into_iter().map(BigInt::from).collect(),
        ];
        let red = lll_weighted(&basis, &[1, 1, 1]);
        for v in &red {
            let n: i64 = v.iter().map(|x| x.to_i64().unwrap().pow(2)).sum();
            assert!(n <= 2, "{v:?}");
        }
    }

    #[test]
    fn cryptopro_b_discriminant_loops() {
        // D = -619, h = 5: cheapest projective loop is 127.5 M ({5: 2, 7: -1}
        // up to orientation), per the frozen PARI evidence.
        let d = BigInt::from(-619);
        let cg = compute_class_group(&d, &params()).unwrap();
        assert_eq!(cg.h, BigInt::from(5));
        let primes = loop_primes(&cg, 100, false, CostModel::Projective, 16, |_| false);
        let res = weighted_shortest(&primes, &cg.invariants, Some(2048), 1 << 30);
        assert!(res.exhaustive);
        let (c, e) = &res.ranked[0];
        assert_eq!(*c, 255);
        let g = loop_generator(&d, &primes, e).unwrap();
        assert_eq!(g.norm, BigInt::from(175));
        // every candidate is a relation and has a verified generator
        for (_, e) in res.ranked.iter().take(50) {
            loop_generator(&d, &primes, e).unwrap();
        }
    }

    #[test]
    fn relation_lattice_has_index_equal_to_subgroup_size() {
        let d = BigInt::from(-4027 * 4);
        let d = if crate::isogeny::class_group_subexp::is_fundamental_small(-4027) {
            BigInt::from(-4027)
        } else {
            d
        };
        let cg = compute_class_group(&d, &params()).unwrap();
        let primes = loop_primes(&cg, 100, false, CostModel::Projective, 16, |_| false);
        let dl: Vec<Vec<BigInt>> = primes.iter().map(|p| p.dlog.clone()).collect();
        let b = relation_lattice_basis(&dl, &cg.invariants);
        let mut det = BigInt::one();
        for (j, v) in b.iter().enumerate() {
            det *= &v[j];
            for x in v.iter().skip(j + 1) {
                assert!(x.is_zero());
            }
        }
        // primes <= 100 generate Cl here, so the index is h
        assert_eq!(det, cg.h);
        for v in &b {
            let e: Vec<i64> = v.iter().map(|x| x.to_i64().unwrap()).collect();
            if e.iter().any(|&x| x != 0) {
                loop_generator(&d, &primes, &e).unwrap();
            }
        }
    }
}
