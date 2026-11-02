//! # Folds of the base on the `k = 4` linear algebra (E18).
//!
//! `RESEARCH_RESIDUAL_WALKS.md` §11.19 measured the asymptotic ratio of a
//! plain `k = 4` index calculus on `E(F_{p⁴})` to rho: `r∞ = 0.518`, the
//! sparse linear algebra over `≈ p/2` unknowns against an unfolded rho
//! walk.  Past the handover the method beats rho by `1/r∞` and no more.
//! This module measures what a fold of the base does to that ratio
//! (`research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`
//! §9.1, registered before the code was written):
//!
//! * **`ψ` (`j = 0`).**  `y² = x³ + b` over `F_{p⁴}` with `b ∉ F_{p²}`,
//!   `p ≡ 1 (mod 12)`, prime order `n`.  `ψ(x, y) = (ωx, y)` with
//!   `ω ∈ F_p` of order 3 keeps the base `x ∈ F_p^*`, so its columns are
//!   the `⟨ψ⟩`-orbits of abscissae and a point's logarithm is `±λ^k` times
//!   its orbit representative's (`ψ = [λ]` on `⟨G⟩`).  The matched rho
//!   walks the classes `{±ψ^k P}`.
//! * **`τ_T` (2-torsion).**  `fghr_full`'s construction over `F_{p⁴}`:
//!   `x₀ ∉ F_{p²}`, `c ∈ F_p^*`, `a = c² − 3x₀²`, `b = −x₀³ − a x₀`, so
//!   `T = (x₀, 0)` and `x(P + T) − x₀ = c²/(x − x₀)`; `#E = h·n`.  The base
//!   `x ∈ x₀ + F_p` (offset `u ∉ {0, ±c}`) pairs `u` with `c²/u`; `P` and
//!   `P + T` share a column with coefficient `±1`.  Rho cannot use `τ_T`,
//!   so the matched rho is the negation walk.
//!
//! One relation stream — §11.19's meet-in-the-middle oracle, every
//! decomposition re-added in the group — feeds a control arm (one column
//! per abscissa, §11.19's matrix) and the folded arm.  Each arm is filtered
//! to a square core and solved by §11.7's Wiedemann, and its logarithm is
//! accepted only if `[d]G = Q`.  Rho runs unfolded, negation-folded and (on
//! `j = 0`) `ψ`-folded on one code path, every field multiplication
//! counted, so the three walks are priced alike.

use std::collections::HashMap;
use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use serde::Serialize;

use super::d4_weighted_s5::{lift_back, solve_weighted, weighted_system, MACAULAY_BOUND};
use super::gaudry_cubic::{square_core, wiedemann_u64, SparseRel};
use super::gaudry_quartic::{
    batch_inv_fp, mitm_signed, pair_table, AddJob, Curve4, Pt4, QuarticSolve, E4,
};
use super::gaudry_quartic_d4::{D4Instance, SymmetrisedD4};
use super::glv_invariant_base::factor_u64;
use super::residual_walk::{inv_mod, is_prime_u64, mix64, pow_mod, sqrt_mod};

fn mulm(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}

/// Which fold an instance is built for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Family4 {
    /// `j = 0`, prime order, `ψ` of order 3.
    Psi,
    /// A rational 2-torsion point `T` whose translation keeps the base.
    Translation,
    /// A GLS twist over `F_{p⁴}` of a curve over `F_{p²}`, full 2-torsion,
    /// `ψ² = −1` (E25).
    Gls,
}

impl Family4 {
    pub fn name(self) -> &'static str {
        match self {
            Self::Psi => "j0-psi",
            Self::Translation => "2torsion-tau",
            Self::Gls => "gls-v4",
        }
    }
}

/// A known-answer instance over `F_{p⁴}`.
#[derive(Clone, Copy, Debug)]
pub struct FoldInstance4 {
    pub family: Family4,
    pub p: u64,
    pub a: E4,
    pub b: E4,
    pub group_order: u64,
    pub n: u64,
    pub cofactor: u64,
    pub g: Pt4,
    pub d: u64,
    pub q: Pt4,
    /// `ω ∈ F_p` of order 3 and `λ` with `ψ = [λ]` on `⟨G⟩` (`Psi`).
    pub omega: u64,
    pub lambda: u64,
    /// `T = (x₀, 0)`, `f'(x₀) = c²` (`Translation`).
    pub x0: E4,
    pub c: u64,
    /// `(c_x, c_y)` with `ψ(x, y) = (c_x·x^{p²}, c_y·y^{p²})` (`Gls`).
    pub gls: Option<(E4, E4)>,
}

impl FoldInstance4 {
    /// A fresh curve with its own counters.
    pub fn curve(&self) -> Curve4 {
        Curve4::with_params(self.p, self.a, self.b)
    }

    /// The GLS endomorphism (E25); `x^{p²}` is free in `F_p[t]/(t⁴ − c)`,
    /// the two scalings are charged.
    pub fn psi_gls(&self, curve: &Curve4, pt: &Pt4) -> Pt4 {
        let (cx, cy) = self.gls.expect("a GLS instance");
        if pt.inf {
            return *pt;
        }
        Pt4 {
            x: curve.f.mul(&cx, &sigma(curve.f.p, &pt.x)),
            y: curve.f.mul(&cy, &sigma(curve.f.p, &pt.y)),
            inf: false,
        }
    }

    pub fn psi(&self, curve: &Curve4, pt: &Pt4, k: u32) -> Pt4 {
        if pt.inf || k.is_multiple_of(3) {
            return *pt;
        }
        let w = pow_mod(self.omega, k as u64 % 3, self.p);
        Pt4 {
            x: curve.f.scale(&pt.x, w),
            y: pt.y,
            inf: false,
        }
    }
}

fn random_point(curve: &Curve4, rng: &mut StdRng) -> Pt4 {
    loop {
        let x = curve.f.random(rng);
        if let Some(pt) = curve.lift_x(&x, rng) {
            if !pt.y.is_zero() {
                return pt;
            }
        }
    }
}

/// The multiple of a random point's order in the Hasse interval
/// `p⁴ + 1 ± 2p²`, when it is unique there (it is `#E` once the order
/// exceeds the interval's width).
fn hasse_multiple(curve: &Curve4, rng: &mut StdRng) -> Option<u64> {
    let p = curve.f.p;
    let q = p.pow(4);
    let lo = q + 1 - 2 * p * p;
    let width = 4 * p * p;
    let pt = random_point(curve, rng);
    let steps = (width as f64).sqrt() as u64 + 1;
    let mut table: HashMap<(E4, E4), u64> = HashMap::new();
    let mut jp = Pt4::INF;
    for j in 0..steps {
        if !jp.inf {
            table.entry((jp.x, jp.y)).or_insert(j);
        }
        jp = curve.add(&jp, &pt);
    }
    let giant = curve.mul(&pt, steps);
    let mut t = curve.mul(&pt, lo);
    let mut found = None;
    let mut i = 0u64;
    while i * steps <= width + steps {
        let m = if t.inf {
            Some(lo + i * steps)
        } else {
            let neg = curve.neg(&t);
            table.get(&(neg.x, neg.y)).map(|&j| lo + i * steps + j)
        };
        if let Some(m) = m {
            if m <= lo + width {
                if found.is_some_and(|f| f != m) {
                    return None;
                }
                found.get_or_insert(m);
            }
        }
        t = curve.add(&t, &giant);
        i += 1;
    }
    found
}

/// The `p²`-power map on `F_p[t]/(t⁴ − c)`: `t ↦ −t`.
fn sigma(p: u64, a: &E4) -> E4 {
    let neg = |v: u64| if v == 0 { 0 } else { p - v };
    E4([a.0[0], neg(a.0[1]), a.0[2], neg(a.0[3])])
}

fn not_in_fp2(a: &E4) -> bool {
    a.0[1] != 0 || a.0[3] != 0
}

/// A `j = 0` instance of prime order with `ψ = [λ]` verified on `G`.
pub fn generate_psi_instance(p: u64, seed: u64) -> Result<FoldInstance4, String> {
    if p % 12 != 1 || !is_prime_u64(p) {
        return Err(format!("p = {p} must be a prime ≡ 1 (mod 12)"));
    }
    let mut rng = StdRng::seed_from_u64(seed ^ 0x9510_4F0D);
    let omega = (2..p)
        .map(|g| pow_mod(g, (p - 1) / 3, p))
        .find(|&w| w != 1)
        .ok_or("no cube root of unity")?;
    for _ in 0..10_000 {
        let probe = Curve4::with_params(p, E4::ZERO, E4::ZERO);
        let b = probe.f.random(&mut rng);
        if b.is_zero() || !not_in_fp2(&b) {
            continue;
        }
        let curve = Curve4::with_params(p, E4::ZERO, b);
        let Some(n) = hasse_multiple(&curve, &mut rng) else {
            continue;
        };
        if !is_prime_u64(n) || n % 3 != 1 {
            continue;
        }
        let g = random_point(&curve, &mut rng);
        if !curve.mul(&g, n).inf {
            continue;
        }
        let s = sqrt_mod(n - 3, n).ok_or("−3 is not a square modulo n")?;
        let half = inv_mod(2, n);
        let roots = [
            mulm((n - 1 + s) % n, half, n),
            mulm((2 * n - 1 - s) % n, half, n),
        ];
        let mut inst = FoldInstance4 {
            family: Family4::Psi,
            p,
            a: E4::ZERO,
            b,
            group_order: n,
            n,
            cofactor: 1,
            g,
            d: 0,
            q: Pt4::INF,
            omega,
            lambda: 0,
            x0: E4::ZERO,
            c: 0,
            gls: None,
        };
        let psi_g = inst.psi(&curve, &g, 1);
        if !curve.on_curve(&psi_g) {
            return Err("ψ(G) is not on the curve".into());
        }
        let Some(&lambda) = roots.iter().find(|&&l| curve.mul(&g, l) == psi_g) else {
            return Err("neither root of λ² + λ + 1 is ψ's eigenvalue".into());
        };
        inst.lambda = lambda;
        inst.d = rng.gen_range(1..n);
        inst.q = curve.mul(&g, inst.d);
        return Ok(inst);
    }
    Err(format!("no prime-order j = 0 curve found at p = {p}"))
}

/// A 2-torsion instance with `#E = h·n`, `h ≤ max_cofactor`, the smallest
/// `h` the search meets within its budget preferred.
pub fn generate_translation_instance(
    p: u64,
    seed: u64,
    max_cofactor: u64,
) -> Result<FoldInstance4, String> {
    if p % 4 != 1 || !is_prime_u64(p) {
        return Err(format!("p = {p} must be a prime ≡ 1 (mod 4)"));
    }
    let mut rng = StdRng::seed_from_u64(seed ^ 0x7A07_2B05);
    let mut fallback: Option<FoldInstance4> = None;
    for attempt in 0..4_000u32 {
        if attempt == 2_000 && fallback.is_some() {
            break;
        }
        let probe = Curve4::with_params(p, E4::ZERO, E4::ZERO);
        let f = &probe.f;
        let x0 = f.random(&mut rng);
        if !not_in_fp2(&x0) {
            continue;
        }
        let c = rng.gen_range(1..p);
        let x0sq = f.sq(&x0);
        let a = f.sub(&f.from_fp(c * c % p), &f.scale(&x0sq, 3));
        let b = f.neg(&f.add(&f.mul(&x0sq, &x0), &f.mul(&a, &x0)));
        let a3x4 = f.scale(&f.mul(&f.sq(&a), &a), 4);
        let disc = f.add(&a3x4, &f.scale(&f.sq(&b), 27));
        if disc.is_zero() {
            continue;
        }
        let j = f.mul(&a3x4, &f.inv(&disc));
        if !not_in_fp2(&j) {
            continue;
        }
        let curve = Curve4::with_params(p, a, b);
        let Some(order) = hasse_multiple(&curve, &mut rng) else {
            continue;
        };
        if order % 2 != 0 {
            return Err(format!(
                "#E = {order} is odd although T = (x₀, 0) is rational"
            ));
        }
        let Some(&(n, _)) = factor_u64(order).last() else {
            continue;
        };
        let h = order / n;
        if h > max_cofactor || order % (n * n) == 0 || n <= p * p * p {
            continue;
        }
        let g = loop {
            let g = curve.mul(&random_point(&curve, &mut rng), h);
            if !g.inf {
                break g;
            }
        };
        if !curve.mul(&g, n).inf {
            return Err("n·G ≠ O: the order is wrong".into());
        }
        let t = Pt4 {
            x: x0,
            y: E4::ZERO,
            inf: false,
        };
        if !curve.on_curve(&t) {
            return Err("T is not on the curve".into());
        }
        let d = rng.gen_range(1..n);
        let inst = FoldInstance4 {
            family: Family4::Translation,
            p,
            a,
            b,
            group_order: order,
            n,
            cofactor: h,
            g,
            d,
            q: curve.mul(&g, d),
            omega: 0,
            lambda: 0,
            x0,
            c,
            gls: None,
        };
        if h == 2 {
            return Ok(inst);
        }
        if fallback.as_ref().is_none_or(|f| h < f.cofactor) {
            fallback = Some(inst);
        }
    }
    fallback.ok_or_else(|| format!("no 2-torsion instance with h ≤ {max_cofactor} at p = {p}"))
}

/// The base, the same points for both arms, and the folded arm's column
/// map: `log P_i = coef_i · X_{col_i}`.
#[derive(Clone, Debug)]
pub struct FoldedBase4 {
    pub points: Vec<Pt4>,
    pub col: Vec<usize>,
    pub coef: Vec<u64>,
    pub columns: usize,
    /// Points per column, mean.
    pub points_per_column: f64,
    /// `F_p` multiplications spent building it (the lifts and the orbit
    /// checks).
    pub build_muls: u64,
}

/// Build the base and check every column coefficient on points.
pub fn folded_base(inst: &FoldInstance4, rng: &mut StdRng) -> Result<FoldedBase4, String> {
    let curve = inst.curve();
    let f = &curve.f;
    let p = inst.p;
    let n = inst.n;
    let mut points = Vec::new();
    let mut key: Vec<u64> = Vec::new(); // the F_p parameter of each point
    if inst.family == Family4::Gls {
        return Err("a GLS instance's base is klein_base_with".into());
    }
    match inst.family {
        Family4::Gls => unreachable!("refused above"),
        Family4::Psi => {
            for x in 1..p {
                if let Some(pt) = curve.lift_x(&f.from_fp(x), rng) {
                    points.push(pt);
                    key.push(x);
                }
            }
        }
        Family4::Translation => {
            for u in 1..p {
                if u == inst.c || u == p - inst.c {
                    continue;
                }
                let x = f.add(&inst.x0, &f.from_fp(u));
                if let Some(pt) = curve.lift_x(&x, rng) {
                    points.push(pt);
                    key.push(u);
                }
            }
        }
    }
    let index: HashMap<u64, usize> = key.iter().enumerate().map(|(i, &k)| (k, i)).collect();
    let mut col = vec![usize::MAX; points.len()];
    let mut coef = vec![0u64; points.len()];
    let mut columns = 0usize;
    for i in 0..points.len() {
        if col[i] != usize::MAX {
            continue;
        }
        // i is a fresh representative: walk its orbit.
        let rep = points[i];
        col[i] = columns;
        coef[i] = 1;
        let images: Vec<(u64, Pt4, u64)> = match inst.family {
            Family4::Gls => unreachable!("refused above"),
            Family4::Psi => (1..3u32)
                .map(|k| {
                    let w = pow_mod(inst.omega, k as u64, p);
                    (
                        mulm(key[i], w, p),
                        inst.psi(&curve, &rep, k),
                        pow_mod(inst.lambda, k as u64, n),
                    )
                })
                .collect(),
            Family4::Translation => {
                let t = Pt4 {
                    x: inst.x0,
                    y: E4::ZERO,
                    inf: false,
                };
                let u2 = mulm(mulm(inst.c, inst.c, p), inv_mod(key[i], p), p);
                vec![(u2, curve.add(&rep, &t), 1)]
            }
        };
        for (k2, img, mult) in images {
            let Some(&j) = index.get(&k2) else {
                return Err(format!("the orbit of {} leaves the base", key[i]));
            };
            if j == i {
                continue;
            }
            if points[j].x != img.x {
                return Err("an image's abscissa is not the base point's".into());
            }
            let s = if points[j].y == img.y {
                mult
            } else if points[j].y == f.neg(&img.y) {
                (n - mult) % n
            } else {
                return Err("an image is neither P nor −P".into());
            };
            if col[j] != usize::MAX && (col[j] != columns || coef[j] != s) {
                return Err("an orbit closes inconsistently".into());
            }
            col[j] = columns;
            coef[j] = s;
        }
        columns += 1;
    }
    let ppc = points.len() as f64 / columns.max(1) as f64;
    Ok(FoldedBase4 {
        points,
        col,
        coef,
        columns,
        points_per_column: ppc,
        build_muls: curve.f.muls(),
    })
}

/// One arm's matrix and its solve.
#[derive(Clone, Debug, Default, Serialize)]
pub struct ArmReport {
    pub arm: String,
    pub columns: usize,
    pub relations: usize,
    pub residuals_at_solve: u64,
    pub unknowns: usize,
    pub filtered_out: usize,
    pub phi: f64,
    pub row_weight: f64,
    pub la_attempts: u64,
    /// Multiplications mod `n`, every attempt.
    pub la_ops: u64,
    pub la_ops_last: u64,
    pub wiedemann_constant: f64,
    pub solved: bool,
    pub correct: bool,
    /// Scalar recovered from the square core, retained for independent replay.
    pub recovered: Option<u64>,
    /// Rows identical to one already kept, skipped.  On the `ψ` arm two
    /// decompositions of one target that differ by `P + ψP + ψ²P = O`
    /// fold to the same row; the control keeps them in distinct columns.
    pub duplicate_rows: u64,
}

struct Arm {
    rep: ArmReport,
    rels: Vec<SparseRel>,
    seen: std::collections::HashSet<(Vec<(usize, u64)>, u64)>,
    small: usize,
    next_attempt: usize,
}

impl Arm {
    fn new(name: &str, columns: usize) -> Self {
        Arm {
            rep: ArmReport {
                arm: name.into(),
                columns,
                ..Default::default()
            },
            rels: Vec::new(),
            seen: Default::default(),
            small: columns,
            next_attempt: columns + 1,
        }
    }

    /// Keep a row unless an identical one is already in.
    fn push(&mut self, row: SparseRel) {
        if self.seen.insert((row.cols.clone(), row.rhs)) {
            self.rels.push(row);
        } else {
            self.rep.duplicate_rows += 1;
        }
    }

    /// Try the solve once enough rows are in.
    fn attempt(&mut self, inst: &FoldInstance4, curve: &Curve4, rng: &mut StdRng, residuals: u64) {
        if self.rep.solved || self.rels.len() < self.next_attempt {
            return;
        }
        let unknowns = self.small + 1;
        let n = inst.n;
        let Some(core) = square_core(&self.rels, self.small) else {
            self.next_attempt = self.rels.len() + (unknowns / 20).max(1);
            return;
        };
        self.rep.la_attempts += 1;
        let mut map = vec![usize::MAX; unknowns];
        let mut k = 0;
        for c in 0..unknowns {
            if core.columns.get(c).copied().unwrap_or(false) {
                map[c] = k;
                k += 1;
            }
        }
        let sel: Vec<SparseRel> = core
            .rows
            .iter()
            .map(|&i| SparseRel {
                cols: self.rels[i]
                    .cols
                    .iter()
                    .map(|&(c, v)| (map[c], v))
                    .collect(),
                rhs: self.rels[i].rhs,
            })
            .collect();
        let before = self.rep.la_ops;
        let x = wiedemann_u64(&sel, sel.len(), n, rng, &mut self.rep.la_ops);
        let dd = x.map(|x| x[map[self.small]]);
        if dd.is_some_and(|dd| curve.mul(&inst.g, dd) == inst.q) {
            self.rep.solved = true;
            self.rep.correct = dd == Some(inst.d);
            self.rep.recovered = dd;
            self.rep.unknowns = sel.len();
            self.rep.filtered_out = self.rels.len() - sel.len();
            self.rep.la_ops_last = self.rep.la_ops - before;
            self.rep.relations = self.rels.len();
            self.rep.residuals_at_solve = residuals;
            self.rep.phi = sel.len() as f64 / self.small as f64;
            self.rep.row_weight = self.rels.iter().map(|r| r.cols.len()).sum::<usize>() as f64
                / self.rels.len().max(1) as f64;
            self.rep.wiedemann_constant =
                self.rep.la_ops as f64 / (self.rep.unknowns.max(1) as f64).powi(2);
        } else {
            self.next_attempt = self.rels.len() + (sel.len() / 20).max(1);
        }
    }
}

/// Which classes a rho walk folds by.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum RhoFold {
    None,
    Negation,
    /// `{±ψ^k P}` (`j = 0` instances only).
    Psi,
    /// `{±P, ±ψP}`, `ψ` the GLS endomorphism (`Gls` instances only).
    Gls,
    /// The quartic twist Frobenius and cubic automorphism, order 24.
    Twist24,
}

#[derive(Clone, Debug, Serialize)]
pub struct RhoRun4F {
    pub fold: RhoFold,
    pub seed: u64,
    pub steps: u64,
    pub group_ops: u64,
    /// Every `F_p` multiplication the walk spent, set-up and the class
    /// representative included (an `F_{p⁴}` inversion at `40`).
    pub fp_muls: u64,
    /// Fruitless cycles met and escaped by doubling.
    pub cycle_escapes: u64,
    pub correct: bool,
    /// Collision-recovered scalar, including failed or unverified candidates.
    pub recovered: Option<u64>,
    /// A digest of every class representative the walk visited, in order
    /// (E27; `0` where a walk does not compute it).
    pub visit_digest: u64,
    /// The part of `fp_muls` and `group_ops` spent verifying `[d]G = Q` after
    /// the collision (charged since 2026-10-24; E25's records did not charge
    /// it).  `0` where a walk does not separate it.
    pub verify_fp_muls: u64,
    pub verify_group_ops: u64,
}

/// The class representative of `pt` under `fold` and the scalar `m` with
/// `rep = [m]·pt` on `⟨G⟩` (the walk's canonical form).
fn rho_canon(inst: &FoldInstance4, curve: &Curve4, fold: RhoFold, pt: Pt4) -> (Pt4, u64) {
    let f = &curve.f;
    let n = inst.n;
    match fold {
        RhoFold::None => (pt, 1),
        RhoFold::Negation => {
            let ny = f.neg(&pt.y);
            if ny < pt.y {
                (Pt4 { y: ny, ..pt }, n - 1)
            } else {
                (pt, 1)
            }
        }
        RhoFold::Gls => {
            let img = inst.psi_gls(curve, &pt);
            let best = if img.x < pt.x {
                (img, inst.lambda % n)
            } else {
                (pt, 1)
            };
            let ny = f.neg(&best.0.y);
            if ny < best.0.y {
                (Pt4 { y: ny, ..best.0 }, (n - best.1) % n)
            } else {
                best
            }
        }
        RhoFold::Psi => {
            let lam = [1, inst.lambda % n, mulm(inst.lambda, inst.lambda, n)];
            let mut best = (pt, 1u64);
            for (k, &l) in lam.iter().enumerate().skip(1) {
                let img = inst.psi(curve, &pt, k as u32);
                if img.x < best.0.x {
                    best = (img, l);
                }
            }
            let ny = f.neg(&best.0.y);
            if ny < best.0.y {
                (Pt4 { y: ny, ..best.0 }, (n - best.1) % n)
            } else {
                best
            }
        }
        RhoFold::Twist24 => unreachable!("use rho_twist24 for this action"),
    }
}

/// The walk's partition of a representative into `r` classes.
fn rho_part(pt: &Pt4, r: usize) -> usize {
    let h = pt.x.0[0] ^ pt.x.0[1].wrapping_mul(0x9E37_79B9) ^ pt.x.0[2].rotate_left(17);
    (mix64(h ^ pt.x.0[3].rotate_left(29)) % r as u64) as usize
}

/// One step of a running digest of the representatives a walk visits (E27's
/// bit-identity check).
fn visit_digest(acc: u64, pt: &Pt4) -> u64 {
    let mut h = acc;
    for v in pt.x.0.iter().chain(pt.y.0.iter()) {
        h = mix64(h ^ v.wrapping_mul(0x9E37_79B9_7F4A_7C15));
    }
    h
}

/// The walk's set-up draws, in the order the serial walk draws them: the
/// `r` multipliers `(α, β)` and the start `(a, b)`.
fn rho_setup_draws(n: u64, seed: u64, r: usize) -> (Vec<(u64, u64)>, (u64, u64)) {
    let mut rng = StdRng::seed_from_u64(seed ^ 0x8D04_F01D);
    let mults = (0..r)
        .map(|_| (rng.gen_range(0..n), rng.gen_range(1..n)))
        .collect();
    let a = rng.gen_range(0..n);
    let b = rng.gen_range(1..n);
    (mults, (a, b))
}

const RHO_PARTITIONS: usize = 32;

/// An r-adding walk with exhaustive storage on class representatives.
/// A step whose successor would land in its own partition takes the next
/// partition instead (the look-ahead that keeps a folded walk out of
/// fruitless 2-cycles); a self-collision — a longer fruitless cycle — is
/// escaped by doubling the current point.  Every group operation and field
/// multiplication is charged, set-up included.
pub fn rho4_folded(inst: &FoldInstance4, fold: RhoFold, seed: u64) -> RhoRun4F {
    rho4_folded_with(inst, fold, seed, super::gaudry_quartic::karatsuba_default())
}

/// [`rho4_folded`] with the `F_{p⁴}` product chosen (E27): the same walk,
/// priced by the schoolbook (`false`) or the Karatsuba tower (`true`).
pub fn rho4_folded_with(
    inst: &FoldInstance4,
    fold: RhoFold,
    seed: u64,
    karatsuba: bool,
) -> RhoRun4F {
    assert!(fold != RhoFold::Psi || inst.family == Family4::Psi);
    assert!(fold != RhoFold::Gls || inst.gls.is_some());
    let curve = Curve4::with_params_arith(inst.p, inst.a, inst.b, karatsuba);
    let n = inst.n;
    let r = RHO_PARTITIONS;
    let canon = |pt: Pt4| rho_canon(inst, &curve, fold, pt);
    let part = |pt: &Pt4| rho_part(pt, r);
    let (draws, (mut a, mut b)) = rho_setup_draws(n, seed, r);
    let mults: Vec<(u64, u64, Pt4)> = draws
        .iter()
        .map(|&(al, be)| {
            (
                al,
                be,
                curve.add(&curve.mul(&inst.g, al), &curve.mul(&inst.q, be)),
            )
        })
        .collect();
    let mut table: HashMap<Pt4, (u64, u64)> = HashMap::new();
    let (mut steps, mut cycle_escapes) = (0u64, 0u64);
    let mut digest = 0u64;
    let mut found = None;
    let cap = 64 * (n as f64).sqrt() as u64 + 1_000_000;
    let start = curve.add(&curve.mul(&inst.g, a), &curve.mul(&inst.q, b));
    let (mut l, m) = canon(start);
    a = mulm(a, m, n);
    b = mulm(b, m, n);
    while steps <= cap {
        steps += 1;
        digest = visit_digest(digest, &l);
        if let Some(&(a2, b2)) = table.get(&l) {
            let db = (b + n - b2) % n;
            if db != 0 {
                let da = (a2 + n - a) % n;
                found = Some(mulm(da, inv_mod(db, n), n));
                break;
            }
            // A fruitless cycle: escape by doubling, which costs one group
            // operation where a restart costs two scalar multiplications.
            cycle_escapes += 1;
            let (dbl, m) = canon(curve.add(&l, &l));
            l = dbl;
            a = mulm(mulm(2, a, n), m, n);
            b = mulm(mulm(2, b, n), m, n);
            continue;
        }
        table.insert(l, (a, b));
        let mut j = part(&l);
        let (mut next, mut m) = canon(curve.add(&l, &mults[j].2));
        if fold != RhoFold::None && part(&next) == j {
            j = (j + 1) % r;
            (next, m) = canon(curve.add(&l, &mults[j].2));
        }
        a = mulm((a + mults[j].0) % n, m, n);
        b = mulm((b + mults[j].1) % n, m, n);
        l = next;
    }
    let (walk_muls, walk_ops) = (curve.f.muls(), curve.ops());
    let correct = found == Some(inst.d) && found.is_some_and(|d| curve.mul(&inst.g, d) == inst.q);
    RhoRun4F {
        fold,
        seed,
        steps,
        group_ops: curve.ops(),
        fp_muls: curve.f.muls(),
        cycle_escapes,
        correct,
        recovered: found,
        visit_digest: digest,
        verify_fp_muls: curve.f.muls() - walk_muls,
        verify_group_ops: curve.ops() - walk_ops,
    }
}

/// A double-and-add in flight, one bit per round ([`Curve4::mul`]'s order).
#[derive(Clone, Copy)]
struct ScalarJob {
    acc: Pt4,
    base: Pt4,
    k: u64,
}

impl ScalarJob {
    fn new(pt: Pt4, k: u64) -> ScalarJob {
        ScalarJob {
            acc: Pt4::INF,
            base: pt,
            k,
        }
    }
    fn requests(&self, out: &mut Vec<(Pt4, Pt4)>) {
        if self.k > 0 {
            if self.k & 1 == 1 {
                out.push((self.acc, self.base));
            }
            out.push((self.base, self.base));
        }
    }
    fn deliver(&mut self, res: &mut std::slice::Iter<Pt4>) {
        if self.k > 0 {
            if self.k & 1 == 1 {
                self.acc = *res.next().unwrap();
            }
            self.base = *res.next().unwrap();
            self.k >>= 1;
        }
    }
}

enum WalkPhase {
    /// The `2r + 2` scalar multiplications of the set-up.
    Setup(Vec<ScalarJob>),
    /// The `r + 1` sums `[α]G + [β]Q` and the start.
    Combine(Vec<ScalarJob>),
    /// One addition in flight: a step (`Some(j)`, `second`) or an escape.
    Step {
        j: Option<usize>,
        second: bool,
    },
    /// The verification `[d]G` of a recovered logarithm.
    Verify(ScalarJob),
    Done,
}

/// One walk of [`rho4_folded`] as a state machine whose additions are
/// handed out one round at a time, so that the inversions of a pool of
/// walks can be batched.  Its sequence of group operations, and every
/// point it computes, are the serial walk's.
struct WalkMachine<'a> {
    inst: &'a FoldInstance4,
    fold: RhoFold,
    seed: u64,
    curve: Curve4,
    draws: Vec<(u64, u64)>,
    start_ab: (u64, u64),
    mults: Vec<(u64, u64, Pt4)>,
    table: HashMap<Pt4, (u64, u64)>,
    l: Pt4,
    a: u64,
    b: u64,
    steps: u64,
    cycle_escapes: u64,
    digest: u64,
    cap: u64,
    found: Option<u64>,
    correct: bool,
    /// Counters when the collision was found (the verification follows).
    walk_cost: Option<(u64, u64)>,
    phase: WalkPhase,
}

impl<'a> WalkMachine<'a> {
    fn new(inst: &'a FoldInstance4, fold: RhoFold, seed: u64, karatsuba: bool) -> WalkMachine<'a> {
        let n = inst.n;
        let (draws, start_ab) = rho_setup_draws(n, seed, RHO_PARTITIONS);
        let mut jobs = Vec::with_capacity(2 * RHO_PARTITIONS + 2);
        for &(al, be) in draws.iter().chain(std::iter::once(&start_ab)) {
            jobs.push(ScalarJob::new(inst.g, al));
            jobs.push(ScalarJob::new(inst.q, be));
        }
        WalkMachine {
            inst,
            fold,
            seed,
            curve: Curve4::with_params_arith(inst.p, inst.a, inst.b, karatsuba),
            draws,
            start_ab,
            mults: Vec::new(),
            table: HashMap::new(),
            l: Pt4::INF,
            a: 0,
            b: 0,
            steps: 0,
            cycle_escapes: 0,
            digest: 0,
            cap: 64 * (n as f64).sqrt() as u64 + 1_000_000,
            found: None,
            correct: false,
            walk_cost: None,
            phase: WalkPhase::Setup(jobs),
        }
    }

    fn done(&self) -> bool {
        matches!(self.phase, WalkPhase::Done)
    }

    fn canon(&self, pt: Pt4) -> (Pt4, u64) {
        rho_canon(self.inst, &self.curve, self.fold, pt)
    }

    /// The additions this walk needs next.
    fn requests(&self) -> Vec<(Pt4, Pt4)> {
        let mut out = Vec::new();
        match &self.phase {
            WalkPhase::Setup(jobs) => jobs.iter().for_each(|j| j.requests(&mut out)),
            WalkPhase::Combine(jobs) => {
                for pair in jobs.chunks(2) {
                    out.push((pair[0].acc, pair[1].acc));
                }
            }
            WalkPhase::Step { j: Some(j), .. } => out.push((self.l, self.mults[*j].2)),
            WalkPhase::Step { j: None, .. } => out.push((self.l, self.l)),
            WalkPhase::Verify(job) => job.requests(&mut out),
            WalkPhase::Done => {}
        }
        out
    }

    /// Hand the sums back and advance to the next additions needed.
    fn deliver(&mut self, results: &[Pt4]) {
        let n = self.inst.n;
        let r = RHO_PARTITIONS;
        let mut it = results.iter();
        let phase = std::mem::replace(&mut self.phase, WalkPhase::Done);
        match phase {
            WalkPhase::Setup(mut jobs) => {
                jobs.iter_mut().for_each(|j| j.deliver(&mut it));
                self.phase = if jobs.iter().all(|j| j.k == 0) {
                    WalkPhase::Combine(jobs)
                } else {
                    WalkPhase::Setup(jobs)
                };
            }
            WalkPhase::Combine(_) => {
                let sums: Vec<Pt4> = it.copied().collect();
                self.mults = self
                    .draws
                    .iter()
                    .zip(&sums)
                    .map(|(&(al, be), &pt)| (al, be, pt))
                    .collect();
                let (l, m) = self.canon(sums[r]);
                self.l = l;
                self.a = mulm(self.start_ab.0, m, n);
                self.b = mulm(self.start_ab.1, m, n);
                self.advance();
            }
            WalkPhase::Step { j: None, .. } => {
                let (dbl, m) = self.canon(*it.next().unwrap());
                self.l = dbl;
                self.a = mulm(mulm(2, self.a, n), m, n);
                self.b = mulm(mulm(2, self.b, n), m, n);
                self.advance();
            }
            WalkPhase::Step { j: Some(j), second } => {
                let (next, m) = self.canon(*it.next().unwrap());
                if !second && self.fold != RhoFold::None && rho_part(&next, r) == j {
                    self.phase = WalkPhase::Step {
                        j: Some((j + 1) % r),
                        second: true,
                    };
                    return;
                }
                self.a = mulm((self.a + self.mults[j].0) % n, m, n);
                self.b = mulm((self.b + self.mults[j].1) % n, m, n);
                self.l = next;
                self.advance();
            }
            WalkPhase::Verify(mut job) => {
                job.deliver(&mut it);
                if job.k == 0 {
                    self.correct = job.acc == self.inst.q;
                    self.phase = WalkPhase::Done;
                } else {
                    self.phase = WalkPhase::Verify(job);
                }
            }
            WalkPhase::Done => {}
        }
    }

    /// The serial walk's loop head: run until the next addition.
    fn advance(&mut self) {
        let n = self.inst.n;
        if self.steps > self.cap {
            self.finish();
            return;
        }
        self.steps += 1;
        self.digest = visit_digest(self.digest, &self.l);
        if let Some(&(a2, b2)) = self.table.get(&self.l) {
            let db = (self.b + n - b2) % n;
            if db != 0 {
                let da = (a2 + n - self.a) % n;
                self.found = Some(mulm(da, inv_mod(db, n), n));
                self.finish();
                return;
            }
            self.cycle_escapes += 1;
            self.phase = WalkPhase::Step {
                j: None,
                second: false,
            };
            return;
        }
        self.table.insert(self.l, (self.a, self.b));
        self.phase = WalkPhase::Step {
            j: Some(rho_part(&self.l, RHO_PARTITIONS)),
            second: false,
        };
    }

    fn finish(&mut self) {
        self.walk_cost = Some((self.curve.f.muls(), self.curve.ops()));
        self.phase = match self.found {
            Some(d) if d == self.inst.d => WalkPhase::Verify(ScalarJob::new(self.inst.g, d)),
            _ => WalkPhase::Done,
        };
        if let WalkPhase::Verify(job) = &self.phase {
            if job.k == 0 {
                self.correct = self.inst.q.inf;
                self.phase = WalkPhase::Done;
            }
        }
    }

    fn report(&self) -> RhoRun4F {
        RhoRun4F {
            fold: self.fold,
            seed: self.seed,
            steps: self.steps,
            group_ops: self.curve.ops(),
            fp_muls: self.curve.f.muls(),
            cycle_escapes: self.cycle_escapes,
            correct: self.correct,
            recovered: self.found,
            visit_digest: self.digest,
            verify_fp_muls: self.walk_cost.map_or(0, |(m, _)| self.curve.f.muls() - m),
            verify_group_ops: self.walk_cost.map_or(0, |(_, o)| self.curve.ops() - o),
        }
    }
}

/// What a pool of batched walks spent on its shared inversions (E27).
#[derive(Clone, Debug, Default, Serialize)]
pub struct BatchStats {
    pub pool: usize,
    pub rounds: u64,
    /// `F_p` norms inverted, and the `F_p` multiplications the batches cost
    /// (`3(m − 1) + 16` each), shared among their members.
    pub inversions: u64,
    pub batch_muls: u64,
    pub filler_walks: u64,
    /// Inversions of the recorded walks only, and their share.
    pub recorded_inversions: u64,
    pub recorded_batch_muls: u64,
}

/// E27 (§9.25): the walks of [`rho4_folded`] for `seeds`, run in lockstep
/// in a pool of `pool` walks whose `F_p` norm inversions are inverted
/// together by Montgomery's trick each round.  A finished walk's slot is
/// taken by a filler walk (seeds from `filler_seed_base`) until every
/// recorded walk is done, so the batch stays full; fillers pay their share
/// and are not reported.  A batch's cost is split among its members in
/// integers, the remainder rotating, so the walk totals are exact.  Every
/// recorded walk performs the serial walk's group operations on the same
/// points; only the price of its inversions differs.
pub fn rho4_folded_batched(
    inst: &FoldInstance4,
    fold: RhoFold,
    seeds: &[u64],
    pool: usize,
    filler_seed_base: u64,
    karatsuba: bool,
) -> (Vec<RhoRun4F>, BatchStats) {
    assert!(fold != RhoFold::Psi || inst.family == Family4::Psi);
    assert!(fold != RhoFold::Gls || inst.gls.is_some());
    assert!(pool >= 1);
    let p = inst.p;
    let mut stats = BatchStats {
        pool,
        ..BatchStats::default()
    };
    let mut out: Vec<Option<RhoRun4F>> = vec![None; seeds.len()];
    // (machine, index into `seeds` or None for a filler).
    let mut slots: Vec<(WalkMachine, Option<usize>)> = Vec::new();
    let mut next_seed = 0usize;
    let mut fillers = 0u64;
    let mut round = 0u64;
    loop {
        if out.iter().all(|o| o.is_some()) {
            break;
        }
        while slots.len() < pool {
            if next_seed < seeds.len() {
                slots.push((
                    WalkMachine::new(inst, fold, seeds[next_seed], karatsuba),
                    Some(next_seed),
                ));
                next_seed += 1;
            } else {
                slots.push((
                    WalkMachine::new(inst, fold, filler_seed_base + fillers, karatsuba),
                    None,
                ));
                fillers += 1;
            }
        }
        // Begin every addition the pool needs.
        let mut jobs: Vec<Vec<AddJob>> = Vec::with_capacity(slots.len());
        for (wm, _) in &slots {
            let reqs = wm.requests();
            jobs.push(
                reqs.iter()
                    .map(|(p1, p2)| wm.curve.add_begin(p1, p2))
                    .collect(),
            );
        }
        // One simultaneous inversion of every pending norm.
        let mut owners: Vec<usize> = Vec::new();
        let mut ds: Vec<u64> = Vec::new();
        for (w, js) in jobs.iter().enumerate() {
            for job in js {
                if let Some(d) = job.norm() {
                    owners.push(w);
                    ds.push(d);
                }
            }
        }
        let (invs, cost) = batch_inv_fp(&ds, p);
        let m = ds.len() as u64;
        if let Some(share) = cost.checked_div(m) {
            let rem = cost % m;
            for (k, &w) in owners.iter().enumerate() {
                let rot = (k as u64 + round) % m;
                let c = share + u64::from(rot < rem);
                slots[w].0.curve.f.charge(c);
                if slots[w].1.is_some() {
                    stats.recorded_batch_muls += c;
                    stats.recorded_inversions += 1;
                }
            }
            stats.inversions += m;
            stats.batch_muls += cost;
        }
        // Finish the additions and advance every walk.
        let mut inv_it = invs.into_iter();
        for (w, js) in jobs.iter().enumerate() {
            let sums: Vec<Pt4> = js
                .iter()
                .map(|job| {
                    let dinv = job.norm().map(|_| inv_it.next().unwrap());
                    slots[w].0.curve.add_end(job, dinv)
                })
                .collect();
            slots[w].0.deliver(&sums);
        }
        // Retire the finished walks.
        let mut k = 0;
        while k < slots.len() {
            if slots[k].0.done() {
                let (wm, idx) = slots.swap_remove(k);
                if let Some(i) = idx {
                    out[i] = Some(wm.report());
                }
            } else {
                k += 1;
            }
        }
        round += 1;
    }
    stats.rounds = round;
    stats.filler_walks = fillers;
    (out.into_iter().map(|o| o.unwrap()).collect(), stats)
}

/// One E18 run: the instance, both arms on one relation stream, and the
/// matched walks.
#[derive(Clone, Debug, Serialize)]
pub struct K4FoldReport {
    pub family: Family4,
    pub p: u64,
    pub seed: u64,
    pub n: u64,
    pub bits: f64,
    pub group_order: u64,
    pub cofactor: u64,
    pub base_points: usize,
    pub points_per_column: f64,
    pub residuals: u64,
    /// Every verified decomposition the oracle returned.
    pub decompositions: u64,
    /// Targets with at least one; each contributes one decomposition, drawn
    /// uniformly from its verified ones, to both arms.
    pub targets_decomposed: u64,
    /// How many verified decompositions targets had (`[k]` = targets with
    /// `k`, the last bucket `≥ 9`).  On a curve with rational `T` one
    /// decomposition comes with the `7` others that add `T` to an even
    /// number of summands, and the eight rows have rank `5`.
    pub decompositions_per_target: [u64; 10],
    pub decomposition_rate: f64,
    pub fp_muls_per_add: f64,
    pub control: ArmReport,
    pub folded: ArmReport,
    pub rho: Vec<RhoRun4F>,
    pub wall_ms: f64,
}

pub fn run_k4_folds(
    family: Family4,
    p: u64,
    seed: u64,
    rho_runs: usize,
) -> Result<K4FoldReport, String> {
    let start = Instant::now();
    let inst = match family {
        Family4::Psi => generate_psi_instance(p, seed)?,
        Family4::Translation => generate_translation_instance(p, seed, 4)?,
        Family4::Gls => return Err("E18 has no GLS arm; E25 is run_k4_gls_pipeline".into()),
    };
    let curve = inst.curve();
    let n = inst.n;
    let mut rng = StdRng::seed_from_u64(seed ^ 0x1A4F_01D5);
    let fb = folded_base(&inst, &mut rng)?;
    let base = &fb.points;
    let fp_per_add = {
        curve.f.reset_muls();
        let mut acc = base[0];
        for _ in 0..64 {
            acc = curve.add(&acc, &base[1]);
        }
        curve.f.muls() as f64 / 64.0
    };
    let table = pair_table(&curve, base);
    let mut control = Arm::new("control", base.len());
    let mut folded = Arm::new(family.name(), fb.columns);
    let (mut residuals, mut decompositions, mut targets_decomposed) = (0u64, 0u64, 0u64);
    let mut per_target = [0u64; 10];
    while residuals < 50_000_000 && !(control.rep.solved && folded.rep.solved) {
        let batch: Vec<(u64, u64)> = (0..64)
            .map(|_| (rng.gen_range(0..n), rng.gen_range(1..n)))
            .collect();
        let found: Vec<(u64, u64, Pt4, Vec<Vec<(usize, i64)>>)> = batch
            .par_iter()
            .map(|&(a, b)| {
                let c = inst.curve();
                let r = c.add(&c.mul(&inst.g, a), &c.mul(&inst.q, b));
                let decs = mitm_signed(&c, base, &table, &r);
                (a, b, r, decs)
            })
            .collect();
        for (a, b, r, decs) in found {
            residuals += 1;
            let mut good: Vec<Vec<(usize, i64)>> = Vec::new();
            for terms in decs {
                // Verify aG + bQ = Σ s P before using it.
                let mut acc = r;
                for &(i, s) in &terms {
                    acc = if s == 1 {
                        curve.sub(&acc, &base[i])
                    } else {
                        curve.add(&acc, &base[i])
                    };
                }
                if !acc.inf {
                    continue;
                }
                decompositions += 1;
                good.push(terms);
            }
            per_target[good.len().min(9)] += 1;
            if !good.is_empty() {
                targets_decomposed += 1;
                // One row per target, drawn uniformly from its verified
                // decompositions: the first in the oracle's sorted order would
                // prefer the lower index of every `{P, P + T}` pair and so half
                // fold the control.
                let terms = good.swap_remove(rng.gen_range(0..good.len()));
                let qcol = |small: usize| (small, (n - b) % n);
                if !control.rep.solved {
                    let mut cols: Vec<(usize, u64)> = terms
                        .iter()
                        .map(|&(i, s)| (i, if s == 1 { 1 } else { n - 1 }))
                        .collect();
                    cols.push(qcol(control.small));
                    cols.sort_unstable();
                    control.push(SparseRel { cols, rhs: a });
                }
                if !folded.rep.solved {
                    let mut merged: HashMap<usize, u64> = HashMap::new();
                    for &(i, s) in &terms {
                        let v = if s == 1 {
                            fb.coef[i]
                        } else {
                            (n - fb.coef[i]) % n
                        };
                        let e = merged.entry(fb.col[i]).or_insert(0);
                        *e = (*e + v) % n;
                    }
                    let mut cols: Vec<(usize, u64)> =
                        merged.into_iter().filter(|&(_, v)| v != 0).collect();
                    cols.push(qcol(folded.small));
                    cols.sort_unstable();
                    folded.push(SparseRel { cols, rhs: a });
                }
            }
            control.attempt(&inst, &curve, &mut rng, residuals);
            folded.attempt(&inst, &curve, &mut rng, residuals);
        }
    }
    let mut folds = vec![RhoFold::None, RhoFold::Negation];
    if family == Family4::Psi {
        folds.push(RhoFold::Psi);
    }
    let jobs: Vec<(RhoFold, u64)> = folds
        .iter()
        .flat_map(|&f| (0..rho_runs as u64).map(move |k| (f, seed * 1000 + k)))
        .collect();
    let rho: Vec<RhoRun4F> = jobs
        .par_iter()
        .map(|&(f, s)| rho4_folded(&inst, f, s))
        .collect();
    Ok(K4FoldReport {
        family,
        p,
        seed,
        n,
        bits: (n as f64).log2(),
        group_order: inst.group_order,
        cofactor: inst.cofactor,
        base_points: base.len(),
        points_per_column: fb.points_per_column,
        residuals,
        decompositions,
        targets_decomposed,
        decompositions_per_target: per_target,
        decomposition_rate: decompositions as f64 / residuals.max(1) as f64,
        fp_muls_per_add: fp_per_add,
        control: control.rep,
        folded: folded.rep,
        rho,
        wall_ms: start.elapsed().as_secs_f64() * 1e3,
    })
}

/// Per-call solve cost `C` (Weil restriction and solve, the reruns
/// included), summarised over every call of one E19 run.
#[derive(Clone, Debug, Default, Serialize)]
pub struct CallStats {
    pub calls: u64,
    pub mean: f64,
    pub median: f64,
    pub p10: f64,
    pub p90: f64,
    pub min: u64,
    pub max: u64,
}
/// The canonical model `y²=x³+b₀t³`, where `t⁴=d` and `d` is the least
/// nonsquare, is a quadratic twist of `y²=x³+b₀` over `F_p`.  On this model
/// the transported `p`-Frobenius preserves the existing `x∈F_p` line.
#[derive(Clone, Copy, Debug)]
pub struct Twist24Instance {
    pub core: FoldInstance4,
    pub b0: u64,
    pub z: u64,
    pub h2: u64,
    pub h3_degree: usize,
    pub h3_coeff: u64,
    pub lambda_frob: u64,
    pub model_fp_muls: u64,
    pub twist_geometry_fp_muls: u64,
    pub input_fp_muls: u64,
    pub input_group_ops: u64,
    pub cubic_eigen_fp_muls: u64,
    pub cubic_eigen_group_ops: u64,
    pub frob_eigen_fp_muls: u64,
    pub frob_eigen_group_ops: u64,
}

impl Twist24Instance {
    pub fn frob(&self, curve: &Curve4, pt: &Pt4) -> Pt4 {
        if pt.inf {
            return *pt;
        }
        let f = &curve.f;
        Pt4 {
            x: f.scale(&f.frob_p(&pt.x, self.z), self.h2),
            y: f.mul_monomial(&f.frob_p(&pt.y, self.z), self.h3_coeff, self.h3_degree),
            inf: false,
        }
    }
}

fn root_of_order(curve: &Curve4, g: &Pt4, image: &Pt4, n: u64, order: u64) -> u64 {
    let u = (2..n)
        .map(|a| pow_mod(a, (n - 1) / order, n))
        .find(|&a| {
            if order == 3 {
                a != 1
            } else {
                pow_mod(a, 4, n) == n - 1
            }
        })
        .expect("requested eigenvalue root does not exist");
    (1..order)
        .filter(|&j| order == 3 || j % 2 == 1)
        .map(|j| pow_mod(u, j, n))
        .find(|&l| curve.mul(g, l) == *image)
        .expect("endomorphism is not the predicted eigenvalue")
}

pub fn construct_twist24(p: u64, b0: u64, seed: u64) -> Result<Twist24Instance, String> {
    if p % 12 != 1 || !is_prime_u64(p) || b0 == 0 || b0 >= p {
        return Err("p must be prime ≡1 (mod 12), and 0<b₀<p".into());
    }
    let f0 = Curve4::with_params(p, E4::ZERO, E4::ZERO);
    let f = &f0.f;
    let t = E4([0, 1, 0, 0]);
    if f.is_square(&t) {
        return Err("quartic generator is not a nonsquare".into());
    }
    let z = f.pow(&t, (p - 1) as u128);
    if !z.in_fp() || pow_mod(z.0[0], 2, p) != p - 1 {
        return Err("quartic generator lacks order-four Frobenius eigenvalue".into());
    }
    let h = f.pow(&t, (p as u128).pow(4) - 1 - (p - 1) as u128 / 2);
    let h2 = f.sq(&h);
    let h3 = f.mul(&h2, &h);
    if !h2.in_fp() || h2.is_zero() {
        return Err("Frobenius x multiplier is not in Fp".into());
    }
    let monomials: Vec<_> = h3.0.iter().enumerate().filter(|&(_, &v)| v != 0).collect();
    if monomials.len() != 1 {
        return Err("Frobenius y multiplier is not a monomial".into());
    }
    let (h3_degree, &h3_coeff) = monomials[0];
    let twist_geometry_fp_muls = f.muls();
    let t3 = f.mul(&f.sq(&t), &t);
    let curve = Curve4::with_params(p, E4::ZERO, f.scale(&t3, b0));
    let model_fp_muls = f.muls() - twist_geometry_fp_muls;
    let mut count = 1u64;
    for x in 0..p {
        let v = (pow_mod(x, 3, p) + b0) % p;
        count += if v == 0 {
            1
        } else if pow_mod(v, (p - 1) / 2, p) == 1 {
            2
        } else {
            0
        };
    }
    let tr = (p + 1) as i128 - count as i128;
    let tr2 = tr * tr - 2 * p as i128;
    let tr4 = tr2 * tr2 - 2 * (p as i128).pow(2);
    let order = (p as i128).pow(4) + 1 + tr4;
    if order <= 0 || order > u64::MAX as i128 {
        return Err("group order does not fit u64".into());
    }
    let n = order as u64;
    if !is_prime_u64(n) || n <= 4 * p * p {
        return Err("selected twist has no Hasse-unique prime full order".into());
    }
    let omega = (2..p)
        .map(|a| pow_mod(a, (p - 1) / 3, p))
        .find(|&a| a != 1)
        .ok_or("no cubic automorphism")?;
    let mut rng = StdRng::seed_from_u64(seed ^ 0x2424_474c_53);
    let g = (1..p)
        .find_map(|x| curve.lift_x(&f.from_fp(x), &mut rng))
        .ok_or("the invariant line is empty")?;
    if !curve.mul(&g, n).inf {
        return Err("prime-order certificate failed".into());
    }
    let d = 271u64;
    let q = curve.mul(&g, d);
    let input_fp_muls = curve.f.muls();
    let input_group_ops = curve.ops();
    let mut inst = Twist24Instance {
        core: FoldInstance4 {
            family: Family4::Psi,
            p,
            a: E4::ZERO,
            b: curve.b,
            group_order: n,
            n,
            cofactor: 1,
            g,
            d,
            q,
            omega,
            lambda: 0,
            x0: E4::ZERO,
            c: 0,
            gls: None,
        },
        b0,
        z: z.0[0],
        h2: h2.0[0],
        h3_degree,
        h3_coeff,
        lambda_frob: 0,
        model_fp_muls,
        twist_geometry_fp_muls,
        input_fp_muls,
        input_group_ops,
        cubic_eigen_fp_muls: 0,
        cubic_eigen_group_ops: 0,
        frob_eigen_fp_muls: 0,
        frob_eigen_group_ops: 0,
    };
    let cubic_g = inst.core.psi(&curve, &g, 1);
    inst.core.lambda = root_of_order(&curve, &g, &cubic_g, n, 3);
    inst.cubic_eigen_fp_muls = curve.f.muls() - input_fp_muls;
    inst.cubic_eigen_group_ops = curve.ops() - input_group_ops;
    let frob_g = inst.frob(&curve, &g);
    inst.lambda_frob = root_of_order(&curve, &g, &frob_g, n, 8);
    inst.frob_eigen_fp_muls = curve.f.muls() - input_fp_muls - inst.cubic_eigen_fp_muls;
    inst.frob_eigen_group_ops = curve.ops() - input_group_ops - inst.cubic_eigen_group_ops;
    if pow_mod(inst.lambda_frob, 4, n) != n - 1 || pow_mod(inst.core.lambda, 3, n) != 1 {
        return Err("endomorphism eigenvalues have wrong orders".into());
    }
    Ok(inst)
}

/// One point per liftable abscissa, with coefficients to an orbit
/// representative. `combined=false` is the E18 cubic fold on exactly the
/// same physical base; `combined=true` adds twist Frobenius.
fn twist24_points(inst: &Twist24Instance, curve: &Curve4, seed: u64) -> Vec<Pt4> {
    let mut rng = StdRng::seed_from_u64(seed ^ 0x1A4F_01D5);
    let points: Vec<Pt4> = (1..inst.core.p)
        .filter_map(|x| curve.lift_x(&curve.f.from_fp(x), &mut rng))
        .collect();
    assert!(!points.is_empty());
    points
}

fn twist24_points_mixed(inst: &Twist24Instance, curve: &Curve4, seed: u64) -> Vec<Pt4> {
    let mut points = twist24_points(inst, curve, seed);
    let mut rng = StdRng::seed_from_u64(seed ^ 0x1A4F_01D5 ^ 0x2);
    let t2 = E4([0, 0, 1, 0]);
    points.extend((1..inst.core.p).filter_map(|x| curve.lift_x(&curve.f.scale(&t2, x), &mut rng)));
    points
}

fn twist24_columns(
    inst: &Twist24Instance,
    curve: &Curve4,
    points: Vec<Pt4>,
    combined: bool,
) -> FoldedBase4 {
    let index: HashMap<E4, usize> = points.iter().enumerate().map(|(i, pt)| (pt.x, i)).collect();
    let mut col = vec![usize::MAX; points.len()];
    let mut coef = vec![0u64; points.len()];
    let mut columns = 0;
    for (i, &rep) in points.iter().enumerate() {
        if col[i] != usize::MAX {
            continue;
        }
        for j in 0..(if combined { 4 } else { 1 }) {
            let mut image = rep;
            for _ in 0..j {
                image = inst.frob(curve, &image);
            }
            let fj = pow_mod(inst.lambda_frob, j, inst.core.n);
            for k in 0..3 {
                let transformed = inst.core.psi(curve, &image, k);
                let multiplier = mulm(
                    fj,
                    pow_mod(inst.core.lambda, k as u64, inst.core.n),
                    inst.core.n,
                );
                assert_eq!(
                    curve.mul(&rep, multiplier),
                    transformed,
                    "endomorphism coefficient failed independent scalar replay"
                );
                let &idx = index.get(&transformed.x).expect("orbit left the base");
                let v = if points[idx].y == transformed.y {
                    multiplier
                } else if points[idx].y == curve.f.neg(&transformed.y) {
                    (inst.core.n - multiplier) % inst.core.n
                } else {
                    panic!("orbit point is neither P nor −P");
                };
                if col[idx] != usize::MAX {
                    assert_eq!(col[idx], columns);
                    assert_eq!(coef[idx], v);
                } else {
                    col[idx] = columns;
                    coef[idx] = v;
                }
            }
        }
        columns += 1;
    }
    assert!(col.iter().all(|&v| v != usize::MAX));
    FoldedBase4 {
        points,
        col,
        coef,
        columns,
        points_per_column: index.len() as f64 / columns as f64,
        // Not measured by this producer.
        build_muls: 0,
    }
}

pub fn twist24_base(inst: &Twist24Instance, seed: u64, combined: bool) -> FoldedBase4 {
    let curve = inst.core.curve();
    twist24_columns(inst, &curve, twist24_points(inst, &curve, seed), combined)
}

/// A matched rho walk on all 24 classes.  It uses E18's jump table,
/// look-ahead, fruitless-cycle escape and charging convention.
pub fn rho_twist24(inst: &Twist24Instance, seed: u64) -> RhoRun4F {
    rho_twist24_with(inst, seed, super::gaudry_quartic::karatsuba_default())
}

/// [`rho_twist24`] with the `F_{p⁴}` product chosen (E29, §9.27); the
/// `[d]G` check's share is recorded apart.
pub fn rho_twist24_with(inst: &Twist24Instance, seed: u64, karatsuba: bool) -> RhoRun4F {
    let curve = Curve4::with_params_arith(inst.core.p, inst.core.a, inst.core.b, karatsuba);
    let f = &curve.f;
    let n = inst.core.n;
    let mut rng = StdRng::seed_from_u64(seed ^ 0x8D04_F01D);
    let jumps = 32usize;
    let canon = |pt: Pt4| -> (Pt4, u64) {
        if pt.inf {
            return (pt, 1);
        }
        let mut best = (pt, 1u64);
        let mut image = pt;
        let mut multiplier = 1u64;
        for j in 0..4 {
            if j > 0 {
                image = inst.frob(&curve, &image);
                multiplier = mulm(multiplier, inst.lambda_frob, n);
            }
            for k in 0..3u32 {
                let candidate = inst.core.psi(&curve, &image, k);
                let m = mulm(multiplier, pow_mod(inst.core.lambda, k as u64, n), n);
                if candidate.x < best.0.x {
                    best = (candidate, m);
                }
            }
        }
        let ny = f.neg(&best.0.y);
        if ny < best.0.y {
            (Pt4 { y: ny, ..best.0 }, (n - best.1) % n)
        } else {
            best
        }
    };
    let part = |pt: &Pt4| -> usize {
        let h = pt.x.0[0] ^ pt.x.0[1].wrapping_mul(0x9E37_79B9) ^ pt.x.0[2].rotate_left(17);
        (mix64(h ^ pt.x.0[3].rotate_left(29)) % jumps as u64) as usize
    };
    let mults: Vec<(u64, u64, Pt4)> = (0..jumps)
        .map(|_| {
            let a = rng.gen_range(0..n);
            let b = rng.gen_range(1..n);
            (
                a,
                b,
                curve.add(&curve.mul(&inst.core.g, a), &curve.mul(&inst.core.q, b)),
            )
        })
        .collect();
    let mut table: HashMap<Pt4, (u64, u64)> = HashMap::new();
    let (mut steps, mut cycle_escapes) = (0u64, 0u64);
    let mut found = None;
    let cap = 64 * (n as f64).sqrt() as u64 + 1_000_000;
    let mut a = rng.gen_range(0..n);
    let mut b = rng.gen_range(1..n);
    let (mut pt, mult) = canon(curve.add(&curve.mul(&inst.core.g, a), &curve.mul(&inst.core.q, b)));
    a = mulm(a, mult, n);
    b = mulm(b, mult, n);
    while steps <= cap {
        steps += 1;
        if let Some(&(a2, b2)) = table.get(&pt) {
            let db = (b + n - b2) % n;
            if db != 0 {
                found = Some(mulm((a2 + n - a) % n, inv_mod(db, n), n));
                break;
            }
            cycle_escapes += 1;
            let (doubled, m) = canon(curve.add(&pt, &pt));
            pt = doubled;
            a = mulm(mulm(2, a, n), m, n);
            b = mulm(mulm(2, b, n), m, n);
            continue;
        }
        table.insert(pt, (a, b));
        let mut j = part(&pt);
        let (mut next, mut m) = canon(curve.add(&pt, &mults[j].2));
        if part(&next) == j {
            j = (j + 1) % jumps;
            (next, m) = canon(curve.add(&pt, &mults[j].2));
        }
        a = mulm((a + mults[j].0) % n, m, n);
        b = mulm((b + mults[j].1) % n, m, n);
        pt = next;
    }
    let (walk_muls, walk_ops) = (f.muls(), curve.ops());
    let correct = found == Some(inst.core.d)
        && found.is_some_and(|d| curve.mul(&inst.core.g, d) == inst.core.q);
    RhoRun4F {
        fold: RhoFold::Twist24,
        seed,
        steps,
        group_ops: curve.ops(),
        fp_muls: f.muls(),
        cycle_escapes,
        correct,
        recovered: found,
        visit_digest: 0,
        verify_fp_muls: f.muls() - walk_muls,
        verify_group_ops: curve.ops() - walk_ops,
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct TwistRelationRecord {
    pub a: u64,
    pub b: u64,
    pub terms: Vec<(usize, i64)>,
}

#[derive(Clone, Debug, Serialize)]
pub struct K4Twist24Report {
    pub p: u64,
    pub b0: u64,
    pub seed: u64,
    pub n: u64,
    pub z: u64,
    pub omega: u64,
    pub lambda_frob: u64,
    pub lambda_cubic: u64,
    pub generator_x: [u64; 4],
    pub generator_y: [u64; 4],
    pub target_x: [u64; 4],
    pub target_y: [u64; 4],
    pub known_scalar: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_powers: Option<Vec<u8>>,
    /// Processed targets only; each entry is `(a,b,verified decomposition count)`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_trace: Option<Vec<(u64, u64, u16)>>,
    pub negative_control_rejected: bool,
    pub all_coefficients_checked: bool,
    pub model_fp_muls: u64,
    pub twist_geometry_fp_muls: u64,
    pub input_fp_muls: u64,
    pub input_group_ops: u64,
    pub cubic_eigen_fp_muls: u64,
    pub cubic_eigen_group_ops: u64,
    pub frob_eigen_fp_muls: u64,
    pub frob_eigen_group_ops: u64,
    pub base_points: usize,
    pub base_coordinates: Vec<([u64; 4], [u64; 4])>,
    pub selected_relations: Vec<TwistRelationRecord>,
    pub flat_rows: Vec<SparseRel>,
    pub cubic_rows: Vec<SparseRel>,
    pub twist24_rows: Vec<SparseRel>,
    pub residuals: u64,
    pub targets_computed: u64,
    pub decompositions: u64,
    pub targets_decomposed: u64,
    pub decompositions_per_target: [u64; 10],
    pub base_fp_muls: u64,
    pub cubic_map_fp_muls: u64,
    pub twist_map_fp_muls: u64,
    pub table_fp_muls: u64,
    pub stream_fp_muls: u64,
    pub verification_fp_muls: [u64; 3],
    pub targets_computed_at_solve: [u64; 3],
    pub stream_fp_muls_at_solve: [u64; 3],
    pub flat: ArmReport,
    pub cubic: ArmReport,
    pub twist24: ArmReport,
    pub rho: Vec<RhoRun4F>,
    pub wall_ms: f64,
}

fn folded_row(
    terms: &[(usize, i64)],
    fb: &FoldedBase4,
    n: u64,
    small: usize,
    a: u64,
    b: u64,
) -> SparseRel {
    let mut merged = HashMap::<usize, u64>::new();
    for &(i, s) in terms {
        let v = if s == 1 {
            fb.coef[i]
        } else {
            (n - fb.coef[i]) % n
        };
        let e = merged.entry(fb.col[i]).or_insert(0);
        *e = (*e + v) % n;
    }
    let mut cols: Vec<(usize, u64)> = merged.into_iter().filter(|&(_, v)| v != 0).collect();
    cols.push((small, (n - b) % n));
    cols.sort_unstable();
    SparseRel { cols, rhs: a }
}

/// Three equal-physical-base IC arms and their matched rho controls.
pub fn run_k4_twist24(
    p: u64,
    b0: u64,
    seed: u64,
    rho_runs: usize,
) -> Result<K4Twist24Report, String> {
    run_k4_twist24_inner(p, b0, seed, rho_runs, false, false)
}

/// Paired single-line or union-line run with an independent target RNG.
pub fn run_k4_twist24_line(
    p: u64,
    b0: u64,
    seed: u64,
    mixed: bool,
) -> Result<K4Twist24Report, String> {
    run_k4_twist24_inner(p, b0, seed, 0, mixed, true)
}

fn run_k4_twist24_inner(
    p: u64,
    b0: u64,
    seed: u64,
    rho_runs: usize,
    mixed: bool,
    paired_stream: bool,
) -> Result<K4Twist24Report, String> {
    let started = Instant::now();
    let inst = construct_twist24(p, b0, seed)?;
    let core = &inst.core;
    let n = core.n;
    let curve = core.curve();
    let mut corrupted = inst;
    corrupted.lambda_frob = (inst.lambda_frob + 1) % n;
    let negative_control_rejected =
        curve.mul(&core.g, corrupted.lambda_frob) != corrupted.frob(&curve, &core.g);
    assert!(negative_control_rejected);
    let mut rng = StdRng::seed_from_u64(seed ^ 0x1A4F_01D5);
    let mut query_rng = StdRng::seed_from_u64(seed ^ p.rotate_left(17) ^ 0x7A12_0A02);
    curve.f.reset_muls();
    let base = if mixed {
        twist24_points_mixed(&inst, &curve, seed)
    } else {
        twist24_points(&inst, &curve, seed)
    };
    let base_fp_muls = curve.f.muls();
    let cubic_fb = twist24_columns(&inst, &curve, base.clone(), false);
    let cubic_map_fp_muls = curve.f.muls() - base_fp_muls;
    let twist_fb = twist24_columns(&inst, &curve, base.clone(), true);
    let twist_map_fp_muls = curve.f.muls() - base_fp_muls - cubic_map_fp_muls;
    assert_eq!(cubic_fb.points, twist_fb.points);
    assert_eq!(cubic_fb.columns * 3, base.len());
    assert_eq!(twist_fb.columns * 12, base.len());
    let table = pair_table(&curve, &base);
    let table_fp_muls = curve.f.muls() - base_fp_muls - cubic_map_fp_muls - twist_map_fp_muls;
    let mut arms = [
        Arm::new("flat", base.len()),
        Arm::new("cubic", cubic_fb.columns),
        Arm::new("twist24", twist_fb.columns),
    ];
    let (mut residuals, mut decompositions, mut targets_decomposed) = (0u64, 0u64, 0u64);
    let mut targets_computed = 0u64;
    let mut per_target = [0u64; 10];
    let mut selected_relations = Vec::new();
    let mut query_trace = Vec::new();
    let mut verification_fp_muls = [0u64; 3];
    let mut worker_fp_muls = 0u64;
    let mut targets_computed_at_solve = [0u64; 3];
    let mut stream_fp_muls_at_solve = [0u64; 3];
    while residuals < 50_000_000 && !arms.iter().all(|a| a.rep.solved) {
        let batch: Vec<(u64, u64)> = (0..64)
            .map(|_| {
                if paired_stream {
                    (query_rng.gen_range(0..n), query_rng.gen_range(1..n))
                } else {
                    (rng.gen_range(0..n), rng.gen_range(1..n))
                }
            })
            .collect();
        let found: Vec<(u64, u64, Pt4, Vec<Vec<(usize, i64)>>, u64)> = batch
            .par_iter()
            .map(|&(a, b)| {
                let c = core.curve();
                let r = c.add(&c.mul(&core.g, a), &c.mul(&core.q, b));
                let decs = mitm_signed(&c, &base, &table, &r);
                (a, b, r, decs, c.f.muls())
            })
            .collect();
        targets_computed += found.len() as u64;
        worker_fp_muls += found.iter().map(|row| row.4).sum::<u64>();
        for (a, b, r, decs, _) in found {
            residuals += 1;
            let mut good = Vec::new();
            for terms in decs {
                let mut acc = r;
                for &(i, s) in &terms {
                    acc = if s == 1 {
                        curve.sub(&acc, &base[i])
                    } else {
                        curve.add(&acc, &base[i])
                    };
                }
                if acc.inf {
                    decompositions += 1;
                    good.push(terms);
                }
            }
            per_target[good.len().min(9)] += 1;
            if paired_stream {
                query_trace.push((a, b, good.len().try_into().expect("decomposition count")));
            }
            if !good.is_empty() {
                targets_decomposed += 1;
                let terms = good.swap_remove(rng.gen_range(0..good.len()));
                selected_relations.push(TwistRelationRecord {
                    a,
                    b,
                    terms: terms.clone(),
                });
                if !arms[0].rep.solved {
                    let mut cols: Vec<(usize, u64)> = terms
                        .iter()
                        .map(|&(i, s)| (i, if s == 1 { 1 } else { n - 1 }))
                        .collect();
                    cols.push((arms[0].small, (n - b) % n));
                    cols.sort_unstable();
                    arms[0].push(SparseRel { cols, rhs: a });
                }
                if !arms[1].rep.solved {
                    let small = arms[1].small;
                    arms[1].push(folded_row(&terms, &cubic_fb, n, small, a, b));
                }
                if !arms[2].rep.solved {
                    let small = arms[2].small;
                    arms[2].push(folded_row(&terms, &twist_fb, n, small, a, b));
                }
            }
            for (i, arm) in arms.iter_mut().enumerate() {
                let before = curve.f.muls();
                let was_solved = arm.rep.solved;
                arm.attempt(core, &curve, &mut rng, residuals);
                verification_fp_muls[i] += curve.f.muls() - before;
                if !was_solved && arm.rep.solved {
                    targets_computed_at_solve[i] = targets_computed;
                    stream_fp_muls_at_solve[i] = worker_fp_muls + curve.f.muls()
                        - base_fp_muls
                        - cubic_map_fp_muls
                        - twist_map_fp_muls
                        - table_fp_muls
                        - verification_fp_muls.iter().sum::<u64>();
                }
            }
            if arms.iter().all(|arm| arm.rep.solved) {
                break;
            }
        }
    }
    let stream_fp_muls = worker_fp_muls + curve.f.muls()
        - base_fp_muls
        - cubic_map_fp_muls
        - twist_map_fp_muls
        - table_fp_muls
        - verification_fp_muls.iter().sum::<u64>();
    let jobs: Vec<(RhoFold, u64)> = [RhoFold::Negation, RhoFold::Psi, RhoFold::Twist24]
        .iter()
        .flat_map(|&fold| (0..rho_runs as u64).map(move |k| (fold, seed * 1000 + k)))
        .collect();
    let rho: Vec<RhoRun4F> = jobs
        .par_iter()
        .map(|&(fold, s)| match fold {
            RhoFold::Twist24 => rho_twist24(&inst, s),
            _ => rho4_folded(core, fold, s),
        })
        .collect();
    Ok(K4Twist24Report {
        p,
        b0,
        seed,
        n,
        z: inst.z,
        omega: core.omega,
        lambda_frob: inst.lambda_frob,
        lambda_cubic: core.lambda,
        generator_x: core.g.x.0,
        generator_y: core.g.y.0,
        target_x: core.q.x.0,
        target_y: core.q.y.0,
        known_scalar: core.d,
        line_powers: paired_stream.then(|| if mixed { vec![0, 2] } else { vec![0] }),
        query_trace: paired_stream.then_some(query_trace),
        negative_control_rejected,
        all_coefficients_checked: true,
        model_fp_muls: inst.model_fp_muls,
        twist_geometry_fp_muls: inst.twist_geometry_fp_muls,
        input_fp_muls: inst.input_fp_muls,
        input_group_ops: inst.input_group_ops,
        cubic_eigen_fp_muls: inst.cubic_eigen_fp_muls,
        cubic_eigen_group_ops: inst.cubic_eigen_group_ops,
        frob_eigen_fp_muls: inst.frob_eigen_fp_muls,
        frob_eigen_group_ops: inst.frob_eigen_group_ops,
        base_points: base.len(),
        base_coordinates: base.iter().map(|pt| (pt.x.0, pt.y.0)).collect(),
        selected_relations,
        flat_rows: arms[0].rels.clone(),
        cubic_rows: arms[1].rels.clone(),
        twist24_rows: arms[2].rels.clone(),
        residuals,
        targets_computed,
        decompositions,
        targets_decomposed,
        decompositions_per_target: per_target,
        base_fp_muls,
        cubic_map_fp_muls,
        twist_map_fp_muls,
        table_fp_muls,
        stream_fp_muls,
        verification_fp_muls,
        targets_computed_at_solve,
        stream_fp_muls_at_solve,
        flat: arms[0].rep.clone(),
        cubic: arms[1].rep.clone(),
        twist24: arms[2].rep.clone(),
        rho,
        wall_ms: started.elapsed().as_secs_f64() * 1e3,
    })
}

impl CallStats {
    fn of(mut v: Vec<u64>) -> Self {
        if v.is_empty() {
            return Self::default();
        }
        v.sort_unstable();
        let q = |f: f64| v[((v.len() - 1) as f64 * f).round() as usize] as f64;
        CallStats {
            calls: v.len() as u64,
            mean: v.iter().map(|&c| c as f64).sum::<f64>() / v.len() as f64,
            median: q(0.5),
            p10: q(0.1),
            p90: q(0.9),
            min: v[0],
            max: v[v.len() - 1],
        }
    }
}

/// One E19 run (§9.5): relations collected by the weighted `D₄` solve on
/// E18's `τ_T`-folded base, the folded matrix solved, every phase charged
/// in `F_p` multiplications, and the matched walks.
#[derive(Clone, Debug, Serialize)]
pub struct K4PipelineReport {
    pub p: u64,
    pub seed: u64,
    pub n: u64,
    pub bits: f64,
    pub group_order: u64,
    pub cofactor: u64,
    pub base_points: usize,
    pub points_per_column: f64,
    /// Targets `R = aG + bQ` drawn, each given to the solve once.
    pub residuals: u64,
    /// Calls where the solve returned no answer (a degenerate or
    /// unresolved eigenspace past `d + 2`); charged, never dropped.
    pub unsolved: u64,
    pub unsolved_outcomes: std::collections::BTreeMap<String, u64>,
    /// Calls that needed a weighted degree above 26.
    pub reruns: u64,
    /// Rational invariant solutions, summed over calls.
    pub rational_solutions: u64,
    /// Verified `Y`-quadruples from the lift-back, summed over calls.
    pub lifted: u64,
    /// ... of which use `Y = 0` (`u = c`), a point outside E18's base.
    pub off_base: u64,
    /// Quadruples on the base whose exact signed sum is `R`.
    pub decompositions: u64,
    /// Quadruples on the base with no sign vector summing to `R` exactly
    /// (G2.9: must be zero).
    pub check_failures: u64,
    pub targets_decomposed: u64,
    pub decompositions_per_target: [u64; 10],
    /// The folded rows re-added in the group before the solve (G2.9).
    pub rows_readded: u64,
    pub rows_readd_failures: u64,
    /// `C` per call: `QuarticSolve::total_muls`.
    pub call: CallStats,
    pub precompute_muls: u64,
    pub base_muls: u64,
    pub generation_muls: u64,
    pub weil_muls: u64,
    /// Every solve, the Weil restriction excluded (it is `weil_muls`).
    pub solve_muls: u64,
    pub lift_muls: u64,
    pub check_muls: u64,
    /// `16 ·` the matrix's mod-`n` multiplications, every attempt.
    pub la_fp_muls: u64,
    /// Everything above.
    pub total_muls: u64,
    /// Macaulay-matrix entries (`rows · cols`, last degree of each call):
    /// the native work the multiplication count does not price.
    pub macaulay_entries: u64,
    /// `F_p` multiplications of one affine addition in `E(F_{p⁴})`: the
    /// group-addition equivalent (GAE) that prices unpriced native work.
    pub fp_muls_per_add: f64,
    pub solve_ms: f64,
    pub folded: ArmReport,
    pub rho: Vec<RhoRun4F>,
    /// Mean `F_p` multiplications of the matched walks (`matched_fold`).
    pub rho_matched_muls: f64,
    pub rho_matched_steps: f64,
    /// The negation walk, or the `⟨±ψ⟩` walk on a GLS curve (E25).
    pub matched_fold: RhoFold,
    pub rho_negation_muls: f64,
    pub rho_negation_steps: f64,
    pub rho_unfolded_muls: f64,
    /// `total_muls / rho_matched_muls`.
    pub ratio: f64,
    pub wall_ms: f64,
    /// E31: the `V₄` solver, when not the E24/E25 one (absent otherwise).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solver: Option<String>,
    /// E31: calls the chain solve handed to the existing solver (charged).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solver_fallbacks: Option<u64>,
    /// E27 (§9.25): `"karatsuba"` when every `F_{p⁴}` product was priced by
    /// the Karatsuba tower; absent for the schoolbook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp4_product: Option<String>,
    /// E27: `F_p` inversions performed by `Fp4::inv` from target generation
    /// to the linear algebra (the walks excluded), each charged `16`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ic_fp_inversions: Option<u64>,
    /// E30 (§9.28): `F_p` multiplications the trivial-product audit re-prices
    /// away over the same span as `ic_fp_inversions` (absent when off).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ic_trivial_product_savings: Option<u64>,
    /// E33 (§9.35): the solver's pivot-selection native operations, every
    /// call (`[initial counts, key comparisons, count updates]`), their sum,
    /// and the echelon's lead-scan reads; absent for uncounted solvers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_selection: Option<[u64; 3]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_selection_ops: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_scan_reads: Option<u64>,
    /// E35 (§9.40): the trace solver's learning, itemised (already added to
    /// `total_muls`, `macaulay_entries`, `pivot_selection*` and
    /// `lead_scan_reads`), and its replay diagnostics; absent otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_learning: Option<super::chain_trace::TraceLearning>,
}

/// One target's trip through the solve, priced by its own counters.
struct TargetOut {
    a: u64,
    b: u64,
    r: Pt4,
    solved: bool,
    outcome: String,
    degree: u16,
    rational: usize,
    call_muls: u64,
    gen_muls: u64,
    weil_muls: u64,
    lift_muls: u64,
    check_muls: u64,
    entries: u64,
    ms: f64,
    lifted: usize,
    off_base: usize,
    failures: usize,
    /// Verified decompositions: base indices with signs (`+1` or `-1`).
    decs: Vec<[(usize, i64); 4]>,
    /// E31: the chain solve fell back to the existing solver.
    fallback: bool,
    /// E33: pivot-selection native operations `[initial, comparisons,
    /// updates]` and lead-scan reads (zero for uncounted solvers).
    pivot: [u64; 4],
}

/// Decompose `r` over the base by the weighted `D₄` solve at degree 26:
/// Weil restriction, solve, lift-back, then the map of each `Y`-quadruple
/// to base points (`by_y`) and its signs by a group check against `r`
/// itself (the lift-back matches `±r` on `x` only).  Every phase on its own
/// counters; generation is the caller's.
fn d4_decompose(
    inst: &FoldInstance4,
    pre: &SymmetrisedD4,
    base: &[Pt4],
    by_y: &HashMap<u64, usize>,
    r: &Pt4,
    tseed: u64,
) -> TargetOut {
    let mut trng = StdRng::seed_from_u64(tseed);
    let di = D4Instance {
        curve: inst.curve(),
        x0: inst.x0,
        c: inst.c,
    };
    let c = &di.curve;
    let before = c.f.muls();
    let y_r = di.y_of_x(&r.x);
    let comps = weighted_system(pre, &c.f, &y_r);
    let mut st = QuarticSolve {
        weil_muls: c.f.muls() - before,
        ..Default::default()
    };
    let (sols, degree) = solve_weighted(
        &comps,
        MACAULAY_BOUND,
        inst.p,
        &mut trng,
        &mut st,
        true,
        false,
    );
    let solved = sols.is_some();
    let sols = sols.unwrap_or_default();
    let lb = lift_back(&di, &sols, r, &mut trng);
    let before = c.f.muls();
    let (mut off, mut failures) = (0usize, 0usize);
    let mut decs = Vec::new();
    for ys in &lb.decompositions {
        let idx: Option<Vec<usize>> = ys.iter().map(|y| by_y.get(y).copied()).collect();
        let Some(idx) = idx else {
            off += 1;
            continue;
        };
        let pts: [Pt4; 4] = core::array::from_fn(|i| base[idx[i]]);
        let mut hit = None;
        for s in 0..8u32 {
            let sign = |i: usize| -> i64 {
                if i > 0 && s >> (i - 1) & 1 == 1 {
                    -1
                } else {
                    1
                }
            };
            let mut acc = pts[0];
            for (i, q) in pts.iter().enumerate().skip(1) {
                acc = if sign(i) == 1 {
                    c.add(&acc, q)
                } else {
                    c.sub(&acc, q)
                };
            }
            if !acc.inf && acc.x == r.x {
                // `−r` is the all-flipped pattern.
                let flip = if acc.y == r.y { 1 } else { -1 };
                hit = Some(core::array::from_fn(|i| (idx[i], flip * sign(i))));
                break;
            }
        }
        match hit {
            Some(d) => decs.push(d),
            None => failures += 1,
        }
    }
    TargetOut {
        a: 0,
        b: 0,
        r: *r,
        solved,
        outcome: st.outcome.clone(),
        degree,
        rational: sols.len(),
        call_muls: st.total_muls,
        gen_muls: 0,
        weil_muls: st.weil_muls,
        lift_muls: lb.muls,
        check_muls: c.f.muls() - before,
        entries: st.rows as u64 * st.cols as u64,
        ms: st.total_ms,
        lifted: lb.decompositions.len(),
        off_base: off,
        failures,
        decs,
        fallback: false,
        pivot: [0; 4],
    }
}

/// E19: the first end-to-end `k = 4` index calculus with the weighted `D₄`
/// solve.  Targets are decomposed by `weighted_system`, `solve_weighted`
/// at weighted degree 26 and `lift_back`; each lifted `Y`-quadruple is
/// mapped to base points and its signs fixed by a group check; one verified
/// decomposition per target, drawn uniformly, becomes a `τ_T`-folded row;
/// Wiedemann runs until `[d]G = Q` verifies.  Then `rho_runs` walks of each
/// of the unfolded and negation-folded kinds.
pub fn run_k4_d4_pipeline(
    p: u64,
    seed: u64,
    rho_runs: usize,
    max_residuals: u64,
) -> Result<K4PipelineReport, String> {
    let start = Instant::now();
    let inst = generate_translation_instance(p, seed, 4)?;
    let mut rng = StdRng::seed_from_u64(seed ^ 0xE19D_4A11);
    let fb = folded_base(&inst, &mut rng)?;
    let base = &fb.points;
    let d4 = D4Instance {
        curve: inst.curve(),
        x0: inst.x0,
        c: inst.c,
    };
    let pre = SymmetrisedD4::precompute(&d4, &mut rng, 8)?;
    // Y of every base point, for the map back from the lift; charged.
    let before = d4.curve.f.muls();
    let mut by_y: HashMap<u64, usize> = HashMap::new();
    for (i, pt) in base.iter().enumerate() {
        let y = d4.y_of_x(&pt.x);
        if !y.in_fp() {
            return Err("a base point's Y is not in F_p".into());
        }
        by_y.insert(y.0[0], i);
    }
    let base_muls = fb.build_muls + d4.curve.f.muls() - before;
    let fp_muls_per_add = {
        let c = inst.curve();
        let mut acc = base[0];
        for _ in 0..64 {
            acc = c.add(&acc, &base[1]);
        }
        c.f.muls() as f64 / 64.0
    };

    collect_and_solve(
        &inst,
        &fb,
        seed,
        rho_runs,
        max_residuals,
        &mut rng,
        MACAULAY_BOUND,
        "d4-folded",
        Setup {
            precompute_muls: pre.precompute_muls,
            base_muls,
            fp_muls_per_add,
            start,
            matched: RhoFold::Negation,
            solver: None,
        },
        &|r: &Pt4, tseed: u64| d4_decompose(&inst, &pre, base, &by_y, r, tseed),
    )
}

/// What the instance's set-up cost, carried into the report.
struct Setup {
    precompute_muls: u64,
    base_muls: u64,
    fp_muls_per_add: f64,
    start: Instant,
    /// The walk `ratio` is quoted against: the negation walk, or on a GLS
    /// curve the `⟨±ψ⟩` walk (E25).
    matched: RhoFold,
    /// E31: the `V₄` solver's name when it is not the E24/E25 one.
    solver: Option<&'static str>,
}

/// The relation phase, the solve and the walks shared by E19 and E24: targets
/// drawn in parallel batches, each decomposed by `decompose`, one verified
/// decomposition per target as a folded row, Wiedemann until `[d]G = Q`.
#[allow(clippy::too_many_arguments)]
fn collect_and_solve(
    inst: &FoldInstance4,
    fb: &FoldedBase4,
    seed: u64,
    rho_runs: usize,
    max_residuals: u64,
    rng: &mut StdRng,
    bound: u16,
    arm: &str,
    setup: Setup,
    decompose: &(dyn Fn(&Pt4, u64) -> TargetOut + Sync),
) -> Result<K4PipelineReport, String> {
    let (p, n) = (inst.p, inst.n);
    let curve = inst.curve();
    let base = &fb.points;
    let Setup {
        precompute_muls,
        base_muls,
        fp_muls_per_add,
        start,
        matched,
        solver,
    } = setup;
    let mut fallbacks = 0u64;
    let mut pivot = [0u64; 4];
    super::gaudry_quartic::reset_fp_inversions_total();
    super::gaudry_quartic::reset_trivial_product_savings();
    let mut folded = Arm::new(arm, fb.columns);
    let mut unsolved_outcomes = std::collections::BTreeMap::new();
    let mut per_target = [0u64; 10];
    let mut calls: Vec<u64> = Vec::new();
    let (mut residuals, mut unsolved, mut reruns, mut rational) = (0u64, 0u64, 0u64, 0u64);
    let (mut lifted, mut off_base, mut decompositions, mut check_failures) = (0u64, 0, 0, 0);
    let (mut targets_decomposed, mut rows_readded, mut rows_readd_failures) = (0u64, 0u64, 0u64);
    let (mut gen_muls, mut weil_muls, mut solve_muls) = (0u64, 0u64, 0u64);
    let (mut lift_muls, mut check_muls, mut entries) = (0u64, 0u64, 0u64);
    let mut solve_ms = 0.0;
    let threads = rayon::current_num_threads().max(1);
    while residuals < max_residuals && !folded.rep.solved {
        let batch: Vec<(u64, u64, u64)> = (0..threads as u64 * 2)
            .map(|k| {
                (
                    rng.gen_range(0..n),
                    rng.gen_range(1..n),
                    mix64(seed ^ (residuals + k).wrapping_mul(0x9E37_79B9_7F4A_7C15)),
                )
            })
            .collect();
        let outs: Vec<TargetOut> = batch
            .par_iter()
            .map(|&(a, b, tseed)| {
                let c = inst.curve();
                let r = c.add(&c.mul(&inst.g, a), &c.mul(&inst.q, b));
                let mut o = decompose(&r, tseed);
                o.a = a;
                o.b = b;
                o.gen_muls = c.f.muls();
                o
            })
            .collect();
        for mut o in outs {
            // A sequential run stops at the solve: the rest of the batch is
            // an artefact of the parallel draw and is not counted.
            if folded.rep.solved {
                break;
            }
            residuals += 1;
            fallbacks += u64::from(o.fallback);
            for (t, v) in pivot.iter_mut().zip(o.pivot) {
                *t += v;
            }
            calls.push(o.call_muls);
            gen_muls += o.gen_muls;
            weil_muls += o.weil_muls;
            solve_muls += o.call_muls - o.weil_muls;
            lift_muls += o.lift_muls;
            check_muls += o.check_muls;
            entries += o.entries;
            solve_ms += o.ms;
            rational += o.rational as u64;
            lifted += o.lifted as u64;
            off_base += o.off_base as u64;
            check_failures += o.failures as u64;
            decompositions += o.decs.len() as u64;
            if !o.solved {
                unsolved += 1;
                *unsolved_outcomes.entry(o.outcome.clone()).or_insert(0u64) += 1;
            }
            if o.degree > bound {
                reruns += 1;
            }
            per_target[o.decs.len().min(9)] += 1;
            if o.decs.is_empty() {
                continue;
            }
            targets_decomposed += 1;
            // E18's rule 2: one decomposition per target, drawn uniformly.
            let terms = o.decs.swap_remove(rng.gen_range(0..o.decs.len()));
            // G2.9: the row re-added in the group, aG + bQ − Σ sP = O.
            let mut acc = o.r;
            for &(i, s) in &terms {
                acc = if s == 1 {
                    curve.sub(&acc, &base[i])
                } else {
                    curve.add(&acc, &base[i])
                };
            }
            if !acc.inf || o.r != curve.add(&curve.mul(&inst.g, o.a), &curve.mul(&inst.q, o.b)) {
                rows_readd_failures += 1;
                continue;
            }
            rows_readded += 1;
            let mut merged: HashMap<usize, u64> = HashMap::new();
            for &(i, s) in &terms {
                let v = if s == 1 {
                    fb.coef[i]
                } else {
                    (n - fb.coef[i]) % n
                };
                let e = merged.entry(fb.col[i]).or_insert(0);
                *e = (*e + v) % n;
            }
            let mut cols: Vec<(usize, u64)> = merged.into_iter().filter(|&(_, v)| v != 0).collect();
            cols.push((folded.small, (n - o.b) % n));
            cols.sort_unstable();
            folded.push(SparseRel { cols, rhs: o.a });
            folded.attempt(inst, &curve, rng, residuals);
        }
    }
    if !folded.rep.solved {
        return Err(format!(
            "p = {p} seed {seed}: no solve within {max_residuals} residuals"
        ));
    }
    let ic_fp_inversions = super::gaudry_quartic::fp_inversions_total();
    let ic_trivial = super::gaudry_quartic::trivial_product_savings();
    let mut folds = vec![RhoFold::None, RhoFold::Negation];
    if !folds.contains(&matched) {
        folds.push(matched);
    }
    let jobs: Vec<(RhoFold, u64)> = folds
        .iter()
        .flat_map(|&f| (0..rho_runs as u64).map(move |k| (f, seed * 1000 + k)))
        .collect();
    let rho: Vec<RhoRun4F> = jobs
        .par_iter()
        .map(|&(f, s)| rho4_folded(inst, f, s))
        .collect();
    let mean = |fold: RhoFold, steps: bool| -> f64 {
        let w: Vec<&RhoRun4F> = rho.iter().filter(|w| w.fold == fold).collect();
        w.iter()
            .map(|w| if steps { w.steps } else { w.fp_muls } as f64)
            .sum::<f64>()
            / w.len().max(1) as f64
    };
    let rho_matched_muls = mean(matched, false);
    let la_fp_muls = 16 * folded.rep.la_ops;
    let total_muls = precompute_muls
        + base_muls
        + gen_muls
        + weil_muls
        + solve_muls
        + lift_muls
        + check_muls
        + la_fp_muls;
    let counted = solver
        .and_then(super::klein_v4::V4Solver::parse)
        .is_some_and(|v| v.counts_pivots());
    Ok(K4PipelineReport {
        p,
        seed,
        n,
        bits: (n as f64).log2(),
        group_order: inst.group_order,
        cofactor: inst.cofactor,
        base_points: base.len(),
        points_per_column: fb.points_per_column,
        residuals,
        unsolved,
        unsolved_outcomes,
        reruns,
        rational_solutions: rational,
        lifted,
        off_base,
        decompositions,
        check_failures,
        targets_decomposed,
        decompositions_per_target: per_target,
        rows_readded,
        rows_readd_failures,
        call: CallStats::of(calls),
        precompute_muls,
        base_muls,
        generation_muls: gen_muls,
        weil_muls,
        solve_muls,
        lift_muls,
        check_muls,
        la_fp_muls,
        total_muls,
        macaulay_entries: entries,
        fp_muls_per_add,
        solve_ms,
        folded: folded.rep,
        rho_matched_muls,
        rho_matched_steps: mean(matched, true),
        matched_fold: matched,
        rho_negation_muls: mean(RhoFold::Negation, false),
        rho_negation_steps: mean(RhoFold::Negation, true),
        rho_unfolded_muls: mean(RhoFold::None, false),
        ratio: total_muls as f64 / rho_matched_muls,
        rho,
        wall_ms: start.elapsed().as_secs_f64() * 1e3,
        solver: solver.map(String::from),
        solver_fallbacks: solver.map(|_| fallbacks),
        fp4_product: super::gaudry_quartic::karatsuba_default().then(|| "karatsuba".to_string()),
        ic_fp_inversions: super::gaudry_quartic::karatsuba_default().then_some(ic_fp_inversions),
        ic_trivial_product_savings: super::gaudry_quartic::trivial_product_audit()
            .then_some(ic_trivial),
        pivot_selection: counted.then_some([pivot[0], pivot[1], pivot[2]]),
        pivot_selection_ops: counted.then_some(pivot[0] + pivot[1] + pivot[2]),
        lead_scan_reads: counted.then_some(pivot[3]),
        trace_learning: None,
    })
}

/// E22 Stage A (§9.7): a curve over `F_{p⁴}` with full rational 2-torsion
/// `{T₁, T₂, T₃}` and a Möbius coordinate `z = g⁻¹(x)` in which the
/// translations act as `z ↦ −z`, `z ↦ 1/z` and `z ↦ −1/z`, all over `F_p`:
/// `g` sends `0, ∞` to the fixed points `x₁ ± √f′(x₁)` of `τ_{T₁}` and `1` to
/// a fixed point of `τ_{T₂}`.  The base is `z ∈ F_p`.  It is confined (§9.7,
/// fact 1) exactly when the pole of `g`, `z(O)`, lies in `F_p`.
#[derive(Clone, Copy, Debug)]
pub struct KleinInstance {
    pub inst: FoldInstance4,
    pub roots: [E4; 3],
    /// `g(z) = (b₁c·z + a₁) / (c·z + 1)`.
    pub a1: E4,
    pub b1: E4,
    pub c: E4,
}

impl KleinInstance {
    pub fn x_of_z(&self, f: &super::gaudry_quartic::Fp4, z: &E4) -> Option<E4> {
        let den = f.add(&f.mul(&self.c, z), &E4::ONE);
        if den.is_zero() {
            return None;
        }
        let num = f.add(&f.mul(&f.mul(&self.b1, &self.c), z), &self.a1);
        Some(f.mul(&num, &f.inv(&den)))
    }

    /// `z = (x − a₁) / (c·(b₁ − x))`; `None` at `x = b₁` (`z = ∞`).
    pub fn z_of_x(&self, f: &super::gaudry_quartic::Fp4, x: &E4) -> Option<E4> {
        let den = f.mul(&self.c, &f.sub(&self.b1, x));
        if den.is_zero() {
            return None;
        }
        Some(f.mul(&f.sub(x, &self.a1), &f.inv(&den)))
    }

    /// `z(O)`, the pole of `g`: `−1/c`.
    pub fn pole(&self, f: &super::gaudry_quartic::Fp4) -> E4 {
        f.neg(&f.inv(&self.c))
    }
}

pub fn generate_klein_instance(
    p: u64,
    seed: u64,
    max_cofactor: u64,
) -> Result<KleinInstance, String> {
    if p % 4 != 1 || !is_prime_u64(p) {
        return Err(format!("p = {p} must be a prime ≡ 1 (mod 4)"));
    }
    let mut rng = StdRng::seed_from_u64(seed ^ 0x4B1E_1A22);
    let mut fallback: Option<KleinInstance> = None;
    for attempt in 0..20_000u32 {
        if attempt == 6_000 && fallback.is_some() {
            break;
        }
        let probe = Curve4::with_params(p, E4::ZERO, E4::ZERO);
        let f = &probe.f;
        let x1 = f.random(&mut rng);
        let x2 = f.random(&mut rng);
        let x3 = f.neg(&f.add(&x1, &x2));
        if x1 == x2 || x1 == x3 || x2 == x3 {
            continue;
        }
        // f = (x − x₁)(x − x₂)(x − x₃) = x³ + a x + b.
        let a = f.add(&f.add(&f.mul(&x1, &x2), &f.mul(&x1, &x3)), &f.mul(&x2, &x3));
        let b = f.neg(&f.mul(&f.mul(&x1, &x2), &x3));
        let a3x4 = f.scale(&f.mul(&f.sq(&a), &a), 4);
        let disc = f.add(&a3x4, &f.scale(&f.sq(&b), 27));
        if disc.is_zero() || !not_in_fp2(&f.mul(&a3x4, &f.inv(&disc))) {
            continue;
        }
        let d1 = f.mul(&f.sub(&x1, &x2), &f.sub(&x1, &x3));
        let d2 = f.mul(&f.sub(&x2, &x1), &f.sub(&x2, &x3));
        let (Some(s1), Some(s2)) = (f.sqrt(&d1, &mut rng), f.sqrt(&d2, &mut rng)) else {
            continue;
        };
        let (a1, b1, a2) = (f.add(&x1, &s1), f.sub(&x1, &s1), f.add(&x2, &s2));
        let c = f.mul(&f.sub(&a2, &a1), &f.inv(&f.sub(&b1, &a2)));
        if c.is_zero() {
            continue;
        }
        let curve = Curve4::with_params(p, a, b);
        let Some(order) = hasse_multiple(&curve, &mut rng) else {
            continue;
        };
        if order % 4 != 0 {
            return Err(format!(
                "#E = {order} is not divisible by 4 with full 2-torsion"
            ));
        }
        let Some(&(n, _)) = factor_u64(order).last() else {
            continue;
        };
        let h = order / n;
        if h > max_cofactor || order % (n * n) == 0 || n <= p * p * p {
            continue;
        }
        let g = loop {
            let g = curve.mul(&random_point(&curve, &mut rng), h);
            if !g.inf {
                break g;
            }
        };
        if !curve.mul(&g, n).inf {
            return Err("n·G ≠ O: the order is wrong".into());
        }
        let d = rng.gen_range(1..n);
        let ki = KleinInstance {
            inst: FoldInstance4 {
                family: Family4::Translation,
                p,
                a,
                b,
                group_order: order,
                n,
                cofactor: h,
                g,
                d,
                q: curve.mul(&g, d),
                omega: 0,
                lambda: 0,
                x0: x1,
                c: 0,
                gls: None,
            },
            roots: [x1, x2, x3],
            a1,
            b1,
            c,
        };
        if h == 4 {
            return Ok(ki);
        }
        if fallback.as_ref().is_none_or(|f| h < f.inst.cofactor) {
            fallback = Some(ki);
        }
    }
    fallback.ok_or_else(|| format!("no full-2-torsion instance with h ≤ {max_cofactor} at p = {p}"))
}

/// E25 (§9.15): a GLS twist with the `V₄` line.  `e₂ ∈ F_{p⁴} ∖ F_{p²}`,
/// `e₃ = e₂^{p²}`, `e₁ = −e₂ − e₃`; the roots are `μe_i` with `μ` a non-square,
/// so `E` is the quadratic twist over `F_{p⁴}` of `Π(x − e_i)` over `F_{p²}`,
/// of order `(p² − 1)² + t²` (checked).  `ψ(x, y) = (μ^{1−p²}x^{p²},
/// μ^{3(1−p²)/2}y^{p²})`, verified on points: `ψ² = −1`, `ψ(G) = [λ]G`,
/// `ψ` fixes `T₁` and swaps `T₂, T₃`.  The Möbius coordinate is E24's.  The
/// first curve with `h ≤ max_cofactor` is returned: unlike E24's curves these
/// never showed `h = 4` (§9.16), so no smaller cofactor is searched for.
pub fn generate_gls_klein_instance(
    p: u64,
    seed: u64,
    max_cofactor: u64,
) -> Result<KleinInstance, String> {
    if p % 4 != 1 || !is_prime_u64(p) {
        return Err(format!("p = {p} must be a prime ≡ 1 (mod 4)"));
    }
    let mut rng = StdRng::seed_from_u64(seed ^ 0x6C5E_2B25);
    let q2 = p as u128 * p as u128;
    for _ in 0..20_000u32 {
        let probe = Curve4::with_params(p, E4::ZERO, E4::ZERO);
        let f = &probe.f;
        let e2 = f.random(&mut rng);
        if !not_in_fp2(&e2) {
            continue;
        }
        let e3 = sigma(p, &e2);
        let e1 = f.neg(&f.add(&e2, &e3));
        let mu = f.random(&mut rng);
        if mu.is_zero() || f.is_square(&mu) {
            continue;
        }
        let (x1, x2, x3) = (f.mul(&mu, &e1), f.mul(&mu, &e2), f.mul(&mu, &e3));
        if x1 == x2 || x1 == x3 || x2 == x3 {
            continue;
        }
        let a = f.add(&f.add(&f.mul(&x1, &x2), &f.mul(&x1, &x3)), &f.mul(&x2, &x3));
        let b = f.neg(&f.mul(&f.mul(&x1, &x2), &x3));
        let a3x4 = f.scale(&f.mul(&f.sq(&a), &a), 4);
        let disc = f.add(&a3x4, &f.scale(&f.sq(&b), 27));
        if disc.is_zero() {
            continue;
        }
        let d1 = f.mul(&f.sub(&x1, &x2), &f.sub(&x1, &x3));
        let d2 = f.mul(&f.sub(&x2, &x1), &f.sub(&x2, &x3));
        let (Some(s1), Some(s2)) = (f.sqrt(&d1, &mut rng), f.sqrt(&d2, &mut rng)) else {
            continue;
        };
        let (a1, b1, a2) = (f.add(&x1, &s1), f.sub(&x1, &s1), f.add(&x2, &s2));
        let c = f.mul(&f.sub(&a2, &a1), &f.inv(&f.sub(&b1, &a2)));
        if c.is_zero() {
            continue;
        }
        let curve = Curve4::with_params(p, a, b);
        let Some(order) = hasse_multiple(&curve, &mut rng) else {
            continue;
        };
        let dq = (q2 - 1) * (q2 - 1);
        let t2 = (order as u128)
            .checked_sub(dq)
            .ok_or("#E below (p² − 1)²: not a GLS twist")?;
        let t = (t2 as f64).sqrt().round() as u128;
        if t * t != t2 {
            return Err(format!(
                "#E − (p² − 1)² = {t2} is not a square: not a GLS twist"
            ));
        }
        let Some(&(n, _)) = factor_u64(order).last() else {
            continue;
        };
        let h = order / n;
        if h > max_cofactor || order % (n * n) == 0 || n <= p * p * p {
            continue;
        }
        let g = loop {
            let g = curve.mul(&random_point(&curve, &mut rng), h);
            if !g.inf {
                break g;
            }
        };
        if !curve.mul(&g, n).inf {
            return Err("n·G ≠ O: the order is wrong".into());
        }
        let tp = f.pow(&mu, (q2 - 1) / 2);
        let cx = f.inv(&f.sq(&tp));
        let cy = f.inv(&f.mul(&f.sq(&tp), &tp));
        let d = rng.gen_range(1..n);
        let mut inst = FoldInstance4 {
            family: Family4::Gls,
            p,
            a,
            b,
            group_order: order,
            n,
            cofactor: h,
            g,
            d,
            q: curve.mul(&g, d),
            omega: 0,
            lambda: 0,
            x0: x1,
            c: 0,
            gls: Some((cx, cy)),
        };
        let pg = inst.psi_gls(&curve, &g);
        if !curve.on_curve(&pg) || inst.psi_gls(&curve, &pg) != curve.neg(&g) {
            return Err("ψ² ≠ −1 on G".into());
        }
        let r = random_point(&curve, &mut rng);
        if inst.psi_gls(&curve, &inst.psi_gls(&curve, &r)) != curve.neg(&r) {
            return Err("ψ² ≠ −1 on the full group".into());
        }
        let l = sqrt_mod(n - 1, n).ok_or("−1 is not a square mod n")?;
        inst.lambda = if curve.mul(&g, l) == pg {
            l
        } else if curve.mul(&g, n - l) == pg {
            n - l
        } else {
            return Err("ψ(G) ∉ {[±√−1]G}".into());
        };
        let px = |x: &E4| f.mul(&cx, &sigma(p, x));
        if px(&x1) != x1 || px(&x2) != x3 || px(&x3) != x2 {
            return Err("ψ does not fix T₁ and swap T₂, T₃".into());
        }
        return Ok(KleinInstance {
            inst,
            roots: [x1, x2, x3],
            a1,
            b1,
            c,
        });
    }
    Err(format!(
        "no GLS full-2-torsion instance with h ≤ {max_cofactor} at p = {p}"
    ))
}

/// The base `{P : z(P) ∈ F_p}` (2-torsion excluded) and its columns, the
/// orbits under the three translations, every coefficient checked on points
/// (`Err` if a translate leaves the base).
pub fn klein_base(ki: &KleinInstance, rng: &mut StdRng) -> Result<FoldedBase4, String> {
    klein_base_with(ki, false, rng)
}

/// [`klein_base`], on a GLS curve also folded by `ψ` when `fold_psi` (E25):
/// columns are the orbits under `⟨V₄, ψ⟩`, `ψ`'s coefficient `±λ` checked on
/// points like the translations' `±1`.  On a GLS curve both arms exclude
/// `z⁴ = −1` (fixed by `ψ` up to sign, so `⟨G⟩`-component `0`), so the two
/// arms hold the same points.
pub fn klein_base_with(
    ki: &KleinInstance,
    fold_psi: bool,
    rng: &mut StdRng,
) -> Result<FoldedBase4, String> {
    let inst = &ki.inst;
    if fold_psi && inst.gls.is_none() {
        return Err("the ψ fold needs a GLS instance".into());
    }
    let curve = inst.curve();
    let f = &curve.f;
    let (p, n) = (inst.p, inst.n);
    let ts: Vec<Pt4> = ki
        .roots
        .iter()
        .map(|&x| Pt4 {
            x,
            y: E4::ZERO,
            inf: false,
        })
        .collect();
    let mut points = Vec::new();
    let mut key = Vec::new();
    // The fixed points of the translations, z ∈ {0, ±1, ±i} (the halves of
    // the T's, of order 4), collapse their orbits; excluded, as E19's base
    // excludes Y ∈ {0, ±1}.
    let i = sqrt_mod(p - 1, p).ok_or("p ≢ 1 (mod 4)")?;
    let special = [0, 1, p - 1, i, p - i];
    for z in 0..p {
        if special.contains(&z) || (inst.gls.is_some() && pow_mod(z, 4, p) == p - 1) {
            continue;
        }
        let Some(x) = ki.x_of_z(f, &f.from_fp(z)) else {
            continue;
        };
        if ki.roots.contains(&x) {
            continue;
        }
        if let Some(pt) = curve.lift_x(&x, rng) {
            points.push(pt);
            key.push(z);
        }
    }
    let index: HashMap<u64, usize> = key.iter().enumerate().map(|(i, &k)| (k, i)).collect();
    let mut col = vec![usize::MAX; points.len()];
    let mut coef = vec![0u64; points.len()];
    let mut columns = 0usize;
    for i in 0..points.len() {
        if col[i] != usize::MAX {
            continue;
        }
        col[i] = columns;
        coef[i] = 1;
        let mut images: Vec<(Pt4, u64)> =
            ts.iter().map(|t| (curve.add(&points[i], t), 1)).collect();
        if fold_psi {
            let q = inst.psi_gls(&curve, &points[i]);
            let lam = inst.lambda % n;
            images.push((q, lam));
            images.extend(ts.iter().map(|t| (curve.add(&q, t), lam)));
        }
        for (img, ce) in images {
            let z2 = ki
                .z_of_x(f, &img.x)
                .filter(|z| z.in_fp())
                .ok_or("a translate leaves the line")?;
            let &j = index.get(&z2.0[0]).ok_or("a translate leaves the base")?;
            let s = if points[j].y == img.y {
                ce
            } else if points[j].y == f.neg(&img.y) {
                (n - ce) % n
            } else {
                return Err("a translate is neither P nor −P".into());
            };
            if j == i && s != 1 {
                return Err("an image of P is −P or ψ-fixed".into());
            }
            if j == i {
                continue;
            }
            if col[j] != usize::MAX && (col[j] != columns || coef[j] != s) {
                return Err("an orbit closes inconsistently".into());
            }
            col[j] = columns;
            coef[j] = s;
        }
        columns += 1;
    }
    let ppc = points.len() as f64 / columns.max(1) as f64;
    Ok(FoldedBase4 {
        points,
        col,
        coef,
        columns,
        points_per_column: ppc,
        build_muls: curve.f.muls(),
    })
}

/// One E22 Stage A run.
#[derive(Clone, Debug, Serialize)]
pub struct KleinStageA {
    pub p: u64,
    pub seed: u64,
    pub n: u64,
    pub bits: f64,
    pub cofactor: u64,
    /// `z(O) ∈ F_p`: the line passes through `x(O)` and is confined.
    pub pole_in_fp: bool,
    pub base_points: usize,
    /// `base_points / p`.
    pub density: f64,
    pub columns: usize,
    pub points_per_column: f64,
    pub residuals: u64,
    pub decompositions: u64,
    pub targets_decomposed: u64,
    /// Verified decompositions per target, the last bucket `≥ 99`.
    pub decompositions_per_target: Vec<u64>,
    pub folded: ArmReport,
    pub wall_ms: f64,
}

/// Stage A: the instance, the base with its folds checked on points, and a
/// meet-in-the-middle relation stream on the `V₄`-folded columns (one
/// decomposition per target, drawn uniformly) until `[d]G = Q` verifies.
pub fn run_klein_stage_a(p: u64, seed: u64, max_residuals: u64) -> Result<KleinStageA, String> {
    let start = Instant::now();
    let ki = generate_klein_instance(p, seed, 16)?;
    let inst = ki.inst;
    let curve = inst.curve();
    let n = inst.n;
    let mut rng = StdRng::seed_from_u64(seed ^ 0xE22A);
    let fb = klein_base(&ki, &mut rng)?;
    let base = &fb.points;
    let table = pair_table(&curve, base);
    let mut folded = Arm::new("v4-folded", fb.columns);
    let (mut residuals, mut decompositions, mut targets_decomposed) = (0u64, 0u64, 0u64);
    let mut per_target = vec![0u64; 100];
    while residuals < max_residuals && !folded.rep.solved {
        let batch: Vec<(u64, u64)> = (0..64)
            .map(|_| (rng.gen_range(0..n), rng.gen_range(1..n)))
            .collect();
        let found: Vec<(u64, u64, Pt4, Vec<Vec<(usize, i64)>>)> = batch
            .par_iter()
            .map(|&(a, b)| {
                let c = inst.curve();
                let r = c.add(&c.mul(&inst.g, a), &c.mul(&inst.q, b));
                let decs = mitm_signed(&c, base, &table, &r);
                (a, b, r, decs)
            })
            .collect();
        for (a, b, r, decs) in found {
            if folded.rep.solved {
                break;
            }
            residuals += 1;
            let mut good: Vec<Vec<(usize, i64)>> = Vec::new();
            for terms in decs {
                let mut acc = r;
                for &(i, s) in &terms {
                    acc = if s == 1 {
                        curve.sub(&acc, &base[i])
                    } else {
                        curve.add(&acc, &base[i])
                    };
                }
                if acc.inf {
                    decompositions += 1;
                    good.push(terms);
                }
            }
            per_target[good.len().min(99)] += 1;
            if good.is_empty() {
                continue;
            }
            targets_decomposed += 1;
            let terms = good.swap_remove(rng.gen_range(0..good.len()));
            let mut merged: HashMap<usize, u64> = HashMap::new();
            for &(i, s) in &terms {
                let v = if s == 1 {
                    fb.coef[i]
                } else {
                    (n - fb.coef[i]) % n
                };
                let e = merged.entry(fb.col[i]).or_insert(0);
                *e = (*e + v) % n;
            }
            let mut cols: Vec<(usize, u64)> = merged.into_iter().filter(|&(_, v)| v != 0).collect();
            cols.push((folded.small, (n - b) % n));
            cols.sort_unstable();
            folded.push(SparseRel { cols, rhs: a });
            folded.attempt(&inst, &curve, &mut rng, residuals);
        }
    }
    let pole = ki.pole(&curve.f);
    Ok(KleinStageA {
        p,
        seed,
        n,
        bits: (n as f64).log2(),
        cofactor: inst.cofactor,
        pole_in_fp: pole.in_fp(),
        base_points: base.len(),
        density: base.len() as f64 / p as f64,
        columns: fb.columns,
        points_per_column: fb.points_per_column,
        residuals,
        decompositions,
        targets_decomposed,
        decompositions_per_target: per_target,
        folded: folded.rep,
        wall_ms: start.elapsed().as_secs_f64() * 1e3,
    })
}

/// [`d4_decompose`] on the `V₄` line: Weil restriction in
/// `(e₂, e₃, P₁, P₂)`, the weighted solve at degree 11, the lift-back, and
/// each `z`-quadruple mapped to base points (`by_z`) with its signs fixed by
/// a group check against `r` itself.
fn v4_decompose(
    ki: &KleinInstance,
    pre: &super::klein_v4::SymmetrisedV4,
    base: &[Pt4],
    by_z: &HashMap<u64, usize>,
    r: &Pt4,
    tseed: u64,
    solver: super::klein_v4::V4Solver,
) -> TargetOut {
    use super::klein_v4 as kv;
    let mut trng = StdRng::seed_from_u64(tseed);
    let curve = ki.inst.curve();
    let c = &curve;
    let mut st = QuarticSolve::default();
    let empty = |st: QuarticSolve| TargetOut {
        a: 0,
        b: 0,
        r: *r,
        solved: false,
        outcome: st.outcome.clone(),
        degree: kv::MACAULAY_BOUND,
        rational: 0,
        call_muls: st.total_muls,
        gen_muls: 0,
        weil_muls: st.weil_muls,
        lift_muls: 0,
        check_muls: 0,
        entries: 0,
        ms: 0.0,
        lifted: 0,
        off_base: 0,
        failures: 0,
        decs: Vec::new(),
        fallback: false,
        pivot: [0; 4],
    };
    let before = c.f.muls();
    let Some(z_r) = ki.z_of_x(&c.f, &r.x) else {
        st.weil_muls = c.f.muls() - before;
        st.total_muls = st.weil_muls;
        st.outcome = "z(R) = ∞".into();
        return empty(st);
    };
    let mut fp = 0u64;
    let comps = kv::v4_system(pre, &c.f, &z_r, &mut fp);
    st.weil_muls = c.f.muls() - before + fp;
    // E31: `Macaulay` is exactly the E24/E25 call
    // (`solve_weighted_traced`, degree 11, row selection, no trace).
    let (sols, degree, fallback) = kv::solve_v4(solver, &comps, ki.inst.p, &mut trng, &mut st);
    let solved = sols.is_some();
    let sols = sols.unwrap_or_default();
    let lb = kv::lift_back_v4(ki, c, &sols, r, &mut trng);
    let before = c.f.muls();
    let (mut off, mut failures) = (0usize, 0usize);
    let mut decs = Vec::new();
    for zs in &lb.decompositions {
        let idx: Option<Vec<usize>> = zs.iter().map(|z| by_z.get(z).copied()).collect();
        let Some(idx) = idx else {
            off += 1;
            continue;
        };
        let pts: [Pt4; 4] = core::array::from_fn(|i| base[idx[i]]);
        let mut hit = None;
        for s in 0..8u32 {
            let sign = |i: usize| -> i64 {
                if i > 0 && s >> (i - 1) & 1 == 1 {
                    -1
                } else {
                    1
                }
            };
            let mut acc = pts[0];
            for (i, q) in pts.iter().enumerate().skip(1) {
                acc = if sign(i) == 1 {
                    c.add(&acc, q)
                } else {
                    c.sub(&acc, q)
                };
            }
            if !acc.inf && acc.x == r.x {
                let flip = if acc.y == r.y { 1 } else { -1 };
                hit = Some(core::array::from_fn(|i| (idx[i], flip * sign(i))));
                break;
            }
        }
        match hit {
            Some(d) => decs.push(d),
            None => failures += 1,
        }
    }
    TargetOut {
        a: 0,
        b: 0,
        r: *r,
        solved,
        outcome: st.outcome.clone(),
        degree,
        rational: sols.len(),
        call_muls: st.total_muls,
        gen_muls: 0,
        weil_muls: st.weil_muls,
        lift_muls: lb.muls,
        check_muls: c.f.muls() - before,
        // A fallback built the degree-11 matrix twice.
        entries: st.rows as u64 * st.cols as u64 * (1 + u64::from(fallback)),
        ms: st.total_ms,
        lifted: lb.decompositions.len(),
        off_base: off,
        failures,
        decs,
        fallback,
        pivot: [
            st.pivot_init_ops,
            st.pivot_compare_ops,
            st.pivot_update_ops,
            st.lead_scan_reads,
        ],
    }
}

/// E24 (§9.13): the `k = 4` index calculus end to end on the `V₄` line —
/// E19's protocol with [`generate_klein_instance`], [`klein_base`] and
/// `klein_v4`'s solve at weighted degree 11.
pub fn run_k4_v4_pipeline(
    p: u64,
    seed: u64,
    rho_runs: usize,
    max_residuals: u64,
) -> Result<K4PipelineReport, String> {
    let start = Instant::now();
    let ki = generate_klein_instance(p, seed, 16)?;
    let inst = ki.inst;
    let mut rng = StdRng::seed_from_u64(seed ^ 0xE24A_4B1E);
    let fb = klein_base(&ki, &mut rng)?;
    let base = &fb.points;
    let pcurve = inst.curve();
    let pre = super::klein_v4::SymmetrisedV4::precompute(&ki, &pcurve, &mut rng, 8)?;
    // z of every base point, for the map back from the lift; charged.
    let zc = inst.curve();
    let mut by_z: HashMap<u64, usize> = HashMap::new();
    for (i, pt) in base.iter().enumerate() {
        let z = ki
            .z_of_x(&zc.f, &pt.x)
            .filter(|z| z.in_fp())
            .ok_or("a base point's z is not in F_p")?;
        by_z.insert(z.0[0], i);
    }
    let base_muls = fb.build_muls + zc.f.muls();
    let fp_muls_per_add = {
        let c = inst.curve();
        let mut acc = base[0];
        for _ in 0..64 {
            acc = c.add(&acc, &base[1]);
        }
        c.f.muls() as f64 / 64.0
    };
    collect_and_solve(
        &inst,
        &fb,
        seed,
        rho_runs,
        max_residuals,
        &mut rng,
        super::klein_v4::MACAULAY_BOUND,
        "v4-folded",
        Setup {
            precompute_muls: pre.precompute_muls,
            base_muls,
            fp_muls_per_add,
            start,
            matched: RhoFold::Negation,
            solver: None,
        },
        &|r: &Pt4, tseed: u64| {
            v4_decompose(
                &ki,
                &pre,
                base,
                &by_z,
                r,
                tseed,
                super::klein_v4::V4Solver::Macaulay,
            )
        },
    )
}

/// E26 (§9.17): one GLS twist from E25's construction without the halves
/// condition, and the quantities the derivation ties together.
#[derive(Clone, Debug, Serialize)]
pub struct GlsCensusRow {
    pub p: u64,
    pub seed: u64,
    pub group_order: u64,
    /// `|t|` with `#E = (p² − 1)² + t²`.
    pub t_abs: u64,
    pub t_mod_4: u64,
    pub v2_order: u32,
    /// `f′(x₂)` a square in `F_{p⁴}`: the halves of `T₂` have rational
    /// abscissae (E25's condition).
    pub halves_t2_rational_x: bool,
    /// `E₀(F_{p²})` has a point of order 4, found as a half of `T₁` with both
    /// coordinates in `F_{p²}` (checked on the curve, independent of `t`).
    pub e0_has_order4_point: bool,
}

pub fn gls_cofactor_census(p: u64, seed: u64) -> Result<GlsCensusRow, String> {
    if p % 4 != 1 || !is_prime_u64(p) {
        return Err(format!("p = {p} must be a prime ≡ 1 (mod 4)"));
    }
    let mut rng = StdRng::seed_from_u64(seed ^ 0xE26C_E115);
    let q2 = p as u128 * p as u128;
    for _ in 0..1_000u32 {
        let probe = Curve4::with_params(p, E4::ZERO, E4::ZERO);
        let f = &probe.f;
        let e2 = f.random(&mut rng);
        if !not_in_fp2(&e2) {
            continue;
        }
        let e3 = sigma(p, &e2);
        let e1 = f.neg(&f.add(&e2, &e3));
        let mu = f.random(&mut rng);
        if mu.is_zero() || f.is_square(&mu) {
            continue;
        }
        let (x1, x2, x3) = (f.mul(&mu, &e1), f.mul(&mu, &e2), f.mul(&mu, &e3));
        let a = f.add(&f.add(&f.mul(&x1, &x2), &f.mul(&x1, &x3)), &f.mul(&x2, &x3));
        let b = f.neg(&f.mul(&f.mul(&x1, &x2), &x3));
        let disc = f.add(&f.scale(&f.mul(&f.sq(&a), &a), 4), &f.scale(&f.sq(&b), 27));
        if disc.is_zero() {
            continue;
        }
        let curve = Curve4::with_params(p, a, b);
        let Some(order) = hasse_multiple(&curve, &mut rng) else {
            continue;
        };
        let t2 = (order as u128)
            .checked_sub((q2 - 1) * (q2 - 1))
            .ok_or("#E below (p² − 1)²: not a GLS twist")?;
        let t = (t2 as f64).sqrt().round() as u128;
        if t * t != t2 {
            return Err(format!("#E − (p² − 1)² = {t2} is not a square"));
        }
        let d2 = f.mul(&f.sub(&x2, &x1), &f.sub(&x2, &x3));
        // E₀ : y² = (x − e₁)(x − e₂)(x − e₃) over F_{p²}; the halves of
        // (e₁, 0) have x = e₁ ± √N, N = (e₁ − e₂)(e₁ − e₃) ∈ F_{p²}.
        let n0 = f.mul(&f.sub(&e1, &e2), &f.sub(&e1, &e3));
        let f0 = |x: &E4| f.mul(&f.mul(&f.sub(x, &e1), &f.sub(x, &e2)), &f.sub(x, &e3));
        let mut order4 = false;
        if let Some(s) = f.sqrt(&n0, &mut rng).filter(|s| !not_in_fp2(s)) {
            for x in [f.add(&e1, &s), f.sub(&e1, &s)] {
                let rhs = f0(&x);
                if f.sqrt(&rhs, &mut rng).is_some_and(|y| !not_in_fp2(&y)) {
                    order4 = true;
                }
            }
        }
        return Ok(GlsCensusRow {
            p,
            seed,
            group_order: order,
            t_abs: t as u64,
            t_mod_4: (t % 4) as u64,
            v2_order: order.trailing_zeros(),
            halves_t2_rational_x: f.is_square(&d2),
            e0_has_order4_point: order4,
        });
    }
    Err(format!("no GLS twist found at p = {p}"))
}

/// E25 Stage A (G2.32) on one curve: the base both arms hold, the `ψ` fold,
/// the confinement test (`ν² ∉ F_p`, `ν = −(w² + w⁻²)`, `w = z(O)`), and for
/// `n ≤ 2^{26}` the number of distinct column logarithms (classes under
/// `±λ^e`) by baby-step giant-step: a confined base would show `1`.
#[derive(Clone, Debug, Serialize)]
pub struct GlsStageA {
    pub p: u64,
    pub seed: u64,
    pub n: u64,
    pub bits: f64,
    pub cofactor: u64,
    pub lambda: u64,
    /// `z ∈ F_p` not excluded (`0, ±1, ±i`, `z⁴ = −1`).
    pub eligible: u64,
    pub base_points: usize,
    pub density: f64,
    pub columns_v4: usize,
    pub columns_psi: usize,
    pub points_per_column_v4: f64,
    pub points_per_column_psi: f64,
    pub nu2_in_fp: bool,
    pub nu2_in_fp2: bool,
    pub distinct_column_logs: Option<usize>,
}

pub fn run_gls_stage_a(p: u64, seed: u64) -> Result<GlsStageA, String> {
    let ki = generate_gls_klein_instance(p, seed, 16)?;
    let inst = ki.inst;
    let curve = inst.curve();
    let f = &curve.f;
    let n = inst.n;
    let a = klein_base_with(&ki, false, &mut StdRng::seed_from_u64(seed ^ 0xE25A_6175))?;
    let b = klein_base_with(&ki, true, &mut StdRng::seed_from_u64(seed ^ 0xE25A_6175))?;
    if a.points != b.points {
        return Err("the two arms hold different points".into());
    }
    let w2 = f.sq(&ki.pole(f));
    let nu2 = f.sq(&f.add(&w2, &f.inv(&w2)));
    let distinct = (n <= 1 << 26).then(|| {
        let m = (n as f64).sqrt() as u64 + 1;
        let mut baby: HashMap<Pt4, u64> = HashMap::new();
        let mut cur = Pt4::INF;
        for j in 0..m {
            baby.entry(cur).or_insert(j);
            cur = curve.add(&cur, &inst.g);
        }
        let giant = curve.neg(&curve.mul(&inst.g, m));
        let hinv = inv_mod(inst.cofactor % n, n);
        let mut classes = std::collections::BTreeSet::new();
        for (i, pt) in b.points.iter().enumerate() {
            if b.coef[i] != 1 {
                continue;
            }
            let mut t = curve.mul(pt, inst.cofactor);
            let mut l = None;
            for k in 0..=m {
                if let Some(&j) = baby.get(&t) {
                    l = Some(mulm((k * m + j) % n, hinv, n));
                    break;
                }
                t = curve.add(&t, &giant);
            }
            let l = l.expect("a logarithm in ⟨G⟩");
            let ll = mulm(l, inst.lambda, n);
            classes.insert(*[l, (n - l) % n, ll, (n - ll) % n].iter().min().unwrap());
        }
        classes.len()
    });
    let i = sqrt_mod(p - 1, p).ok_or("p ≢ 1 (mod 4)")?;
    let eligible = (0..p)
        .filter(|&z| ![0, 1, p - 1, i, p - i].contains(&z) && pow_mod(z, 4, p) != p - 1)
        .count() as u64;
    Ok(GlsStageA {
        p,
        seed,
        n,
        bits: (n as f64).log2(),
        cofactor: inst.cofactor,
        lambda: inst.lambda,
        eligible,
        base_points: b.points.len(),
        density: b.points.len() as f64 / eligible as f64,
        columns_v4: a.columns,
        columns_psi: b.columns,
        points_per_column_v4: a.points_per_column,
        points_per_column_psi: b.points_per_column,
        nu2_in_fp: nu2.in_fp(),
        nu2_in_fp2: !not_in_fp2(&nu2),
        distinct_column_logs: distinct,
    })
}

/// E25 (§9.15): E24's pipeline on a GLS curve, one arm: the `V₄`-folded base
/// (`fold_psi = false`, the control) or the same points folded by `⟨V₄, ψ⟩`.
/// Both arms of one `(p, seed)` hold one curve, one base and one target
/// stream; the matched walk is the `⟨±ψ⟩` walk.
pub fn run_k4_gls_pipeline(
    p: u64,
    seed: u64,
    rho_runs: usize,
    max_residuals: u64,
    fold_psi: bool,
) -> Result<K4PipelineReport, String> {
    run_k4_gls_pipeline_with(
        p,
        seed,
        rho_runs,
        max_residuals,
        fold_psi,
        super::klein_v4::V4Solver::Macaulay,
    )
}

/// [`run_k4_gls_pipeline`] with the `V₄` solver chosen (E31, §9.22):
/// `Macaulay` is E25 exactly.
pub fn run_k4_gls_pipeline_with(
    p: u64,
    seed: u64,
    rho_runs: usize,
    max_residuals: u64,
    fold_psi: bool,
    solver: super::klein_v4::V4Solver,
) -> Result<K4PipelineReport, String> {
    let start = Instant::now();
    let ki = generate_gls_klein_instance(p, seed, 16)?;
    let inst = ki.inst;
    let mut rng = StdRng::seed_from_u64(seed ^ 0xE25A_6175);
    let fb = klein_base_with(&ki, fold_psi, &mut rng)?;
    let base = &fb.points;
    let pcurve = inst.curve();
    let pre = super::klein_v4::SymmetrisedV4::precompute(&ki, &pcurve, &mut rng, 8)?;
    let zc = inst.curve();
    let mut by_z: HashMap<u64, usize> = HashMap::new();
    for (i, pt) in base.iter().enumerate() {
        let z = ki
            .z_of_x(&zc.f, &pt.x)
            .filter(|z| z.in_fp())
            .ok_or("a base point's z is not in F_p")?;
        by_z.insert(z.0[0], i);
    }
    let base_muls = fb.build_muls + zc.f.muls();
    let fp_muls_per_add = {
        let c = inst.curve();
        let mut acc = base[0];
        for _ in 0..64 {
            acc = c.add(&acc, &base[1]);
        }
        c.f.muls() as f64 / 64.0
    };
    // E35 (§9.40): the trace solver learns and installs this curve and
    // arm's pivot trace first, from a stream of its own (charged below).
    let learned = if solver == super::klein_v4::V4Solver::ChainSparsestTrace {
        Some(super::chain_trace::learn_and_install(&ki, &pre, seed)?)
    } else {
        None
    };
    let rep = collect_and_solve(
        &inst,
        &fb,
        seed,
        rho_runs,
        max_residuals,
        &mut rng,
        super::klein_v4::MACAULAY_BOUND,
        if fold_psi {
            "v4-psi-folded"
        } else {
            "v4-folded"
        },
        Setup {
            precompute_muls: pre.precompute_muls,
            base_muls,
            fp_muls_per_add,
            start,
            matched: RhoFold::Gls,
            solver: (solver != super::klein_v4::V4Solver::Macaulay).then(|| solver.name()),
        },
        &|r: &Pt4, tseed: u64| v4_decompose(&ki, &pre, base, &by_z, r, tseed, solver),
    )?;
    Ok(match learned {
        Some(l) => super::chain_trace::charge(rep, l),
        None => rep,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn psi_columns_are_orbits_of_three_with_verified_coefficients() {
        let inst = generate_psi_instance(37, 1).unwrap();
        let mut rng = StdRng::seed_from_u64(3);
        let fb = folded_base(&inst, &mut rng).unwrap();
        assert_eq!(fb.points_per_column, 3.0);
        let curve = inst.curve();
        assert_eq!(
            curve.mul(&inst.g, inst.lambda),
            inst.psi(&curve, &inst.g, 1)
        );
    }

    #[test]
    fn translation_columns_pair_p_and_p_plus_t() {
        let inst = generate_translation_instance(37, 2, 4).unwrap();
        let mut rng = StdRng::seed_from_u64(4);
        let fb = folded_base(&inst, &mut rng).unwrap();
        assert!(fb.points_per_column > 1.9 && fb.points_per_column <= 2.0);
        assert_eq!(inst.group_order, inst.cofactor * inst.n);
    }

    #[test]
    fn both_arms_recover_the_logarithm_and_every_walk_is_correct() {
        for family in [Family4::Psi, Family4::Translation] {
            let rep = run_k4_folds(family, 37, 1, 2).unwrap();
            assert!(
                rep.control.solved && rep.control.correct,
                "{family:?} control"
            );
            assert!(rep.folded.solved && rep.folded.correct, "{family:?} folded");
            assert!(rep.rho.iter().all(|w| w.correct), "{family:?} rho");
            assert!(rep.folded.columns * 2 <= rep.control.columns + 2);
        }
    }
}

#[cfg(test)]
mod log_tests {
    use super::*;

    fn dlog(curve: &Curve4, g: &Pt4, h: &Pt4, n: u64) -> u64 {
        let m = (n as f64).sqrt() as u64 + 1;
        let mut baby: HashMap<Pt4, u64> = HashMap::new();
        let mut cur = Pt4::INF;
        for j in 0..m {
            baby.entry(cur).or_insert(j);
            cur = curve.add(&cur, g);
        }
        let giant = curve.neg(&curve.mul(g, m));
        let mut t = *h;
        for i in 0..=m {
            if let Some(&j) = baby.get(&t) {
                return (i * m + j) % n;
            }
            t = curve.add(&t, &giant);
        }
        panic!("no logarithm");
    }

    /// E25, G2.32: on GLS curves (`p ≡ 5` and `p ≡ 1 (mod 8)`, the second
    /// with `z⁴ = −1` points to exclude) `ψ` keeps the `V₄` line, the two
    /// arms hold the same points, the `ψ`-folded columns hold `8`, and every
    /// coefficient is the true logarithm ratio.
    #[test]
    fn gls_psi_folds_the_v4_line_with_true_coefficients() {
        for (p, seed) in [(37, 1), (41, 2)] {
            let ki = generate_gls_klein_instance(p, seed, 16).unwrap();
            let inst = ki.inst;
            let curve = inst.curve();
            let n = inst.n;
            assert_eq!(mulm(inst.lambda, inst.lambda, n), n - 1);
            let a = klein_base_with(&ki, false, &mut StdRng::seed_from_u64(5)).unwrap();
            let b = klein_base_with(&ki, true, &mut StdRng::seed_from_u64(5)).unwrap();
            assert_eq!(a.points, b.points, "p = {p}: the arms hold one base");
            assert!(a.points_per_column <= 4.0);
            assert!(
                b.points_per_column >= 7.8,
                "p = {p}: {}",
                b.points_per_column
            );
            let hinv = inv_mod(inst.cofactor % n, n);
            let logs: Vec<u64> = b
                .points
                .iter()
                .map(|q| {
                    mulm(
                        dlog(&curve, &inst.g, &curve.mul(q, inst.cofactor), n),
                        hinv,
                        n,
                    )
                })
                .collect();
            let mut rep = vec![usize::MAX; b.columns];
            for i in 0..b.points.len() {
                if b.coef[i] == 1 && rep[b.col[i]] == usize::MAX {
                    rep[b.col[i]] = i;
                }
            }
            for i in 0..b.points.len() {
                assert_eq!(
                    mulm(b.coef[i], logs[rep[b.col[i]]], n),
                    logs[i],
                    "p = {p} point {i}"
                );
            }
        }
    }

    /// E26: the census row is internally consistent on small curves.
    #[test]
    fn gls_census_rows_are_consistent() {
        for seed in 1..=12 {
            let r = gls_cofactor_census(37, seed).unwrap();
            let q1 = 37u64 * 37 - 1;
            assert_eq!(r.group_order, q1 * q1 + r.t_abs * r.t_abs);
            assert_eq!(r.v2_order, r.group_order.trailing_zeros());
            assert!(r.v2_order >= 2, "full 2-torsion");
        }
    }

    #[test]
    fn gls_walk_takes_the_steps_its_class_predicts() {
        let ki = generate_gls_klein_instance(37, 1, 16).unwrap();
        for s in 0..4 {
            let w = rho4_folded(&ki.inst, RhoFold::Gls, s);
            assert!(w.correct, "seed {s}");
        }
    }

    /// E25 end to end at `p = 37`, both arms on one curve.
    #[test]
    #[ignore = "thousands of solves: run with --ignored --release"]
    fn the_gls_pipeline_recovers_the_logarithm_on_both_arms() {
        for fold in [false, true] {
            let rep = run_k4_gls_pipeline(37, 1, 2, 200_000, fold).unwrap();
            assert!(rep.folded.solved && rep.folded.correct, "fold_psi = {fold}");
            assert_eq!(rep.check_failures, 0);
            assert_eq!(rep.rows_readd_failures, 0);
            assert!(rep.rho.iter().all(|w| w.correct));
            assert_eq!(rep.matched_fold, RhoFold::Gls);
        }
    }

    /// Every base point's logarithm (of its `⟨G⟩`-component, `h⁻¹·log(hP)`)
    /// is its column coefficient times its representative's.
    #[test]
    fn fold_coefficients_match_true_logarithms() {
        for inst in [
            generate_psi_instance(37, 1).unwrap(),
            generate_translation_instance(37, 2, 4).unwrap(),
        ] {
            let curve = inst.curve();
            let n = inst.n;
            let mut rng = StdRng::seed_from_u64(9);
            let fb = folded_base(&inst, &mut rng).unwrap();
            let hinv = inv_mod(inst.cofactor % n, n);
            let logs: Vec<u64> = fb
                .points
                .iter()
                .map(|p| {
                    let hp = curve.mul(p, inst.cofactor);
                    mulm(dlog(&curve, &inst.g, &hp, n), hinv, n)
                })
                .collect();
            let mut rep = vec![usize::MAX; fb.columns];
            for i in 0..fb.points.len() {
                if fb.coef[i] == 1 && rep[fb.col[i]] == usize::MAX {
                    rep[fb.col[i]] = i;
                }
            }
            for i in 0..fb.points.len() {
                let r = rep[fb.col[i]];
                assert_eq!(
                    mulm(fb.coef[i], logs[r], n),
                    logs[i],
                    "{:?} point {i}",
                    inst.family
                );
            }
        }
    }
}

#[cfg(test)]
mod rho_tests {
    use super::*;
    #[test]
    fn quartic_twist_action_and_line_columns_replay() {
        let inst = construct_twist24(37, 2, 0).unwrap();
        let curve = inst.core.curve();
        let mut bad = inst;
        bad.lambda_frob = (bad.lambda_frob + 1) % bad.core.n;
        assert_ne!(
            curve.mul(&bad.core.g, bad.lambda_frob),
            bad.frob(&curve, &bad.core.g)
        );
        let cubic = twist24_base(&inst, 0, false);
        let combined = twist24_base(&inst, 0, true);
        assert_eq!(cubic.points, combined.points);
        assert_eq!(cubic.columns, 12);
        assert_eq!(combined.columns, 3);
        let h = curve.f.pow(&E4([0, 1, 0, 0]), (37u128).pow(4) - 1 - 18);
        let h2 = curve.f.sq(&h);
        let h3 = curve.f.mul(&h2, &h);
        for pt in &combined.points {
            let generic = Pt4 {
                x: curve.f.mul(&h2, &curve.f.pow(&pt.x, 37)),
                y: curve.f.mul(&h3, &curve.f.pow(&pt.y, 37)),
                inf: false,
            };
            assert_eq!(inst.frob(&curve, pt), generic);
            assert_eq!(inst.frob(&curve, pt), curve.mul(pt, inst.lambda_frob));
            assert_eq!(
                inst.core.psi(&curve, pt, 1),
                curve.mul(pt, inst.core.lambda)
            );
        }
        let a = E4([1, 2, 3, 4]);
        let z = inst.z;
        assert_eq!(curve.f.frob_p(&a, z), curve.f.pow(&a, 37));
        let mut monomial = E4::ZERO;
        monomial.0[inst.h3_degree] = inst.h3_coeff;
        assert_eq!(
            curve.f.mul_monomial(&a, inst.h3_coeff, inst.h3_degree),
            curve.f.mul(&a, &monomial)
        );
        for seed in 0..8 {
            let row = rho_twist24(&inst, seed);
            assert!(row.correct && row.recovered == Some(271));
        }
    }

    /// Each walk takes the steps its class size predicts: `√(π n / 2w)`,
    /// `w = 1, 2, 6`, so the unfolded walk takes `√2` and `√6` times the
    /// steps of the negation and `ψ` walks (bands for `80` walks each).
    #[test]
    fn folded_walks_take_the_steps_their_classes_predict() {
        let inst = generate_psi_instance(97, 1).unwrap();
        let mean_steps = |fold: RhoFold| -> f64 {
            let runs: Vec<RhoRun4F> = (0..80u64)
                .into_par_iter()
                .map(|s| rho4_folded(&inst, fold, s))
                .collect();
            assert!(runs.iter().all(|r| r.correct), "{fold:?}");
            runs.iter().map(|r| r.steps as f64).sum::<f64>() / runs.len() as f64
        };
        let none = mean_steps(RhoFold::None);
        let neg = none / mean_steps(RhoFold::Negation);
        let psi = none / mean_steps(RhoFold::Psi);
        assert!((1.15..1.75).contains(&neg), "unfolded / negation = {neg}");
        assert!((1.9..3.1).contains(&psi), "unfolded / psi = {psi}");
    }
}

#[cfg(test)]
mod pipeline_tests {
    use super::*;

    /// A planted `R = P₀ + P₁ − P₂ + P₃` comes back from the weighted solve
    /// as base points whose signed sum is `R` exactly, the planted one among
    /// them.
    #[test]
    fn d4_decompose_recovers_a_planted_decomposition() {
        let inst = generate_translation_instance(37, 2, 4).unwrap();
        let curve = inst.curve();
        let mut rng = StdRng::seed_from_u64(5);
        let fb = folded_base(&inst, &mut rng).unwrap();
        let d4 = D4Instance {
            curve: inst.curve(),
            x0: inst.x0,
            c: inst.c,
        };
        let pre = SymmetrisedD4::precompute(&d4, &mut rng, 4).unwrap();
        let by_y: HashMap<u64, usize> = fb
            .points
            .iter()
            .enumerate()
            .map(|(i, pt)| (d4.y_of_x(&pt.x).0[0], i))
            .collect();
        // Four points in four distinct columns.
        let mut pick: Vec<usize> = Vec::new();
        for i in 0..fb.points.len() {
            if pick.iter().all(|&j| fb.col[j] != fb.col[i]) {
                pick.push(i);
            }
            if pick.len() == 4 {
                break;
            }
        }
        let b = &fb.points;
        let r = curve.add(
            &curve.sub(&curve.add(&b[pick[0]], &b[pick[1]]), &b[pick[2]]),
            &b[pick[3]],
        );
        let o = d4_decompose(&inst, &pre, b, &by_y, &r, 7);
        assert!(o.solved, "{}", o.outcome);
        assert_eq!(o.failures, 0);
        assert!(o.call_muls > 0 && o.check_muls > 0);
        let mut planted = pick.clone();
        planted.sort_unstable();
        let mut seen = false;
        for d in &o.decs {
            let mut acc = r;
            for &(i, s) in d {
                acc = if s == 1 {
                    curve.sub(&acc, &b[i])
                } else {
                    curve.add(&acc, &b[i])
                };
            }
            assert!(acc.inf, "a decomposition does not re-add to R");
            let mut idx: Vec<usize> = d.iter().map(|&(i, _)| i).collect();
            idx.sort_unstable();
            seen |= idx == planted;
        }
        assert!(seen, "the planted decomposition is missing");
    }

    #[test]
    #[ignore = "hundreds of solves: run with --ignored --release"]
    fn the_pipeline_recovers_the_logarithm_at_p_37() {
        let rep = run_k4_d4_pipeline(37, 2, 2, 20_000).unwrap();
        assert!(rep.folded.solved && rep.folded.correct);
        assert_eq!(rep.check_failures, 0);
        assert_eq!(rep.rows_readd_failures, 0);
        assert!(rep.rho.iter().all(|w| w.correct));
    }
}

#[cfg(test)]
mod e23_tests {
    use super::*;

    /// E23 (§9.7, G2.26): on `j = 0` curves `y² = x³ − x₀³` over `F_{p⁴}`
    /// the line `x ∈ x₀·F_p` is stable under `⟨−1, ψ, τ_{T₁}, τ_{T₂}⟩`
    /// (order 24), and its points are either absent or killed by
    /// `#C(F_{p²})`, `C : v² = t³ − 1`: the base never reaches a large prime.
    #[test]
    fn psi_and_translations_share_only_a_confined_base_on_j0() {
        let (mut empty, mut confined) = (0, 0);
        for p in [37u64, 61, 73, 97] {
            // #C(F_p) by counting, then #C(F_{p²}) = p² + 1 − (a² − 2p).
            let mut cp = 1i64; // the point at infinity
            for t in 0..p {
                let r = (pow_mod(t, 3, p) + p - 1) % p;
                cp += if r == 0 {
                    1
                } else if pow_mod(r, (p - 1) / 2, p) == 1 {
                    2
                } else {
                    0
                };
            }
            let a = p as i64 + 1 - cp;
            let c2 = (p * p + 1) as i64 - (a * a - 2 * p as i64);
            let omega = (2..p)
                .map(|g| pow_mod(g, (p - 1) / 3, p))
                .find(|&w| w != 1)
                .unwrap();
            let mut rng = StdRng::seed_from_u64(p);
            for _ in 0..6 {
                let probe = Curve4::with_params(p, E4::ZERO, E4::ZERO);
                let f = &probe.f;
                let x0 = f.random(&mut rng);
                if x0.in_fp() {
                    continue;
                }
                let b = f.neg(&f.mul(&f.sq(&x0), &x0));
                let curve = Curve4::with_params(p, E4::ZERO, b);
                let f = &curve.f;
                let ts: Vec<Pt4> = (0..3)
                    .map(|i| Pt4 {
                        x: f.scale(&x0, pow_mod(omega, i, p)),
                        y: E4::ZERO,
                        inf: false,
                    })
                    .collect();
                assert!(ts.iter().all(|t| curve.on_curve(t)));
                let on_line = |x: &E4| -> bool {
                    // x = x₀·t with t ∈ F_p.
                    f.mul(x, &f.inv(&x0)).in_fp()
                };
                let base: Vec<Pt4> = (0..p)
                    .filter_map(|t| curve.lift_x(&f.scale(&x0, t), &mut rng))
                    .filter(|pt| !pt.y.is_zero())
                    .collect();
                if base.is_empty() {
                    empty += 1;
                    continue;
                }
                confined += 1;
                for pt in &base {
                    let psi = f.scale(&pt.x, omega);
                    assert!(on_line(&psi), "ψ leaves the line");
                    for t in &ts {
                        assert!(
                            on_line(&curve.add(pt, t).x),
                            "a translation leaves the line"
                        );
                    }
                    assert!(
                        curve.mul(pt, c2 as u64).inf,
                        "a base point outside the F_{{p²}}-points of v² = t³ − 1"
                    );
                }
            }
        }
        // Both branches occur over the tested curves.
        assert!(
            empty > 0 && confined > 0,
            "empty {empty}, confined {confined}"
        );
    }
}

/// E27 (§9.25): the cheaper walk arithmetic gives the same walks.
#[cfg(test)]
mod e27_tests {
    use super::*;
    use crate::cryptanalysis::gaudry_quartic::Fp4;

    #[test]
    fn e27_karatsuba_product_equals_the_schoolbook() {
        for p in [13u64, 101, 397, 1033] {
            let f = Fp4::with_arith(p, true);
            let mut rng = StdRng::seed_from_u64(p);
            let mut differs = 0;
            for _ in 0..20_000 {
                let (a, b) = (f.random(&mut rng), f.random(&mut rng));
                let before = f.muls();
                let k = f.mul_karatsuba(&a, &b);
                assert_eq!(f.muls() - before, 13);
                let before = f.muls();
                assert_eq!(k, f.mul_schoolbook(&a, &b));
                assert_eq!(f.muls() - before, 19);
                // Negative control: one coordinate of one factor moved.
                let mut b2 = b;
                b2.0[2] = (b2.0[2] + 1) % p;
                if f.mul_schoolbook(&a, &b2) != k {
                    differs += 1;
                }
            }
            // a·(b + t²) ≠ a·b unless a = 0.
            assert!(
                differs >= 19_990,
                "negative control not detected at p = {p}"
            );
        }
    }

    #[test]
    fn e27_batched_additions_equal_serial_additions() {
        let ki = generate_gls_klein_instance(101, 1, 16).unwrap();
        let inst = ki.inst;
        for karatsuba in [false, true] {
            let serial = Curve4::with_params_arith(inst.p, inst.a, inst.b, karatsuba);
            let batched = Curve4::with_params_arith(inst.p, inst.a, inst.b, karatsuba);
            let mut pts = vec![inst.g];
            for k in 1..40u64 {
                pts.push(serial.mul(&inst.g, 7919 * k + 3));
            }
            serial.reset_ops();
            serial.f.reset_muls();
            // Distinct points, a doubling, P + (−P) and O + P.
            let mut pairs: Vec<(Pt4, Pt4)> = pts.windows(2).map(|w| (w[0], w[1])).collect();
            pairs.push((pts[3], pts[3]));
            pairs.push((pts[4], serial.neg(&pts[4])));
            pairs.push((Pt4::INF, pts[5]));
            let want: Vec<Pt4> = pairs.iter().map(|(a, b)| serial.add(a, b)).collect();
            let jobs: Vec<AddJob> = pairs.iter().map(|(a, b)| batched.add_begin(a, b)).collect();
            let ds: Vec<u64> = jobs.iter().filter_map(|j| j.norm()).collect();
            let (invs, cost) = batch_inv_fp(&ds, inst.p);
            assert_eq!(cost, 3 * (ds.len() as u64 - 1) + 16);
            let mut it = invs.iter();
            let got: Vec<Pt4> = jobs
                .iter()
                .map(|j| batched.add_end(j, j.norm().map(|_| *it.next().unwrap())))
                .collect();
            assert_eq!(got, want);
            assert_eq!(batched.ops(), serial.ops());
            // Serial pays 16 per F_p inversion; the batch pays nothing yet.
            assert_eq!(batched.f.muls() + 16 * ds.len() as u64, serial.f.muls());
            // Negative control: one perturbed norm inverse is detected.
            let mut bad = batch_inv_fp(&ds, inst.p).0;
            bad[7] = (bad[7] + 1) % inst.p;
            let mut it = bad.iter();
            let wrong: Vec<Pt4> = jobs
                .iter()
                .map(|j| batched.add_end(j, j.norm().map(|_| *it.next().unwrap())))
                .collect();
            let diffs = (0..want.len()).filter(|&i| wrong[i] != want[i]).count();
            assert_eq!(diffs, 1);
            assert!(!serial.on_curve(&wrong[7]));
        }
    }

    #[test]
    fn e27_batched_walks_are_the_serial_walks() {
        let ki = generate_gls_klein_instance(101, 2, 16).unwrap();
        let inst = ki.inst;
        for fold in [RhoFold::Gls, RhoFold::Negation] {
            let seeds: Vec<u64> = (0..6u64).map(|k| 2000 + k).collect();
            let school: Vec<RhoRun4F> = seeds
                .iter()
                .map(|&s| rho4_folded_with(&inst, fold, s, false))
                .collect();
            let kara: Vec<RhoRun4F> = seeds
                .iter()
                .map(|&s| rho4_folded_with(&inst, fold, s, true))
                .collect();
            let (batched, stats) = rho4_folded_batched(&inst, fold, &seeds, 4, 1 << 40, true);
            assert!(stats.filler_walks > 0 && stats.inversions > 0);
            for ((a, b), c) in school.iter().zip(&kara).zip(&batched) {
                for w in [b, c] {
                    assert_eq!(
                        (
                            a.steps,
                            a.group_ops,
                            a.cycle_escapes,
                            a.recovered,
                            a.correct
                        ),
                        (
                            w.steps,
                            w.group_ops,
                            w.cycle_escapes,
                            w.recovered,
                            w.correct
                        )
                    );
                    assert_eq!(a.visit_digest, w.visit_digest);
                }
                assert!(a.correct);
                assert!(b.fp_muls < a.fp_muls && c.fp_muls < b.fp_muls);
            }
            // The digest tells walks apart (its own negative control).
            assert_ne!(school[0].visit_digest, school[1].visit_digest);
        }
    }
}

// ── E28 (§9.26): the GLS fold on the single-`τ_T` line ───────────────────

/// A GLS twist on E19's line (§9.26): `σ(x₀) = −x₀`, `c ∈ F_p^*`,
/// `a = c² − 3x₀²`, `b = −x₀³ − a x₀`; `ψ(x, y) = (−σ(x), i·σ(y))` with
/// `ψ = [λ]` on `⟨G⟩`, verified on `G`.  The least cofactor `≤ max_cofactor`
/// met within the budget is kept (family `Translation`, `gls` set).
pub fn generate_gls_tau_instance(
    p: u64,
    seed: u64,
    max_cofactor: u64,
) -> Result<FoldInstance4, String> {
    if p % 4 != 1 || !is_prime_u64(p) {
        return Err(format!("p = {p} must be a prime ≡ 1 (mod 4)"));
    }
    let i_fp = (2..p)
        .map(|g| pow_mod(g, (p - 1) / 4, p))
        .find(|&w| mulm(w, w, p) == p - 1)
        .ok_or("no square root of −1")?;
    let mut rng = StdRng::seed_from_u64(seed ^ 0xE28A_7A0F);
    let mut best: Option<FoldInstance4> = None;
    for attempt in 0..400u32 {
        if best.as_ref().is_some_and(|b| b.cofactor == 4) || (attempt >= 200 && best.is_some()) {
            break;
        }
        let probe = Curve4::with_params(p, E4::ZERO, E4::ZERO);
        let f = &probe.f;
        let x0 = E4([0, rng.gen_range(0..p), 0, rng.gen_range(0..p)]);
        if x0.is_zero() {
            continue;
        }
        let c = rng.gen_range(1..p);
        let x0sq = f.sq(&x0);
        let a = f.sub(&f.from_fp(c * c % p), &f.scale(&x0sq, 3));
        let b = f.neg(&f.add(&f.mul(&x0sq, &x0), &f.mul(&a, &x0)));
        let a3x4 = f.scale(&f.mul(&f.sq(&a), &a), 4);
        let disc = f.add(&a3x4, &f.scale(&f.sq(&b), 27));
        if disc.is_zero() {
            continue;
        }
        let j = f.mul(&a3x4, &f.inv(&disc));
        if j.in_fp() {
            continue; // a twist of a curve over F_p: excluded.
        }
        let curve = Curve4::with_params(p, a, b);
        let Some(order) = hasse_multiple(&curve, &mut rng) else {
            continue;
        };
        if order % 4 != 0 {
            return Err(format!(
                "#E = {order} is not divisible by 4 although E[2] is rational"
            ));
        }
        let Some(&(n, _)) = factor_u64(order).last() else {
            continue;
        };
        let h = order / n;
        if h > max_cofactor || order % (n * n) == 0 || n <= p * p * p || n % 4 != 1 {
            continue;
        }
        if best.as_ref().is_some_and(|b| b.cofactor <= h) {
            continue;
        }
        let g = loop {
            let g = curve.mul(&random_point(&curve, &mut rng), h);
            if !g.inf {
                break g;
            }
        };
        if !curve.mul(&g, n).inf {
            return Err("n·G ≠ O: the order is wrong".into());
        }
        let mut inst = FoldInstance4 {
            family: Family4::Translation,
            p,
            a,
            b,
            group_order: order,
            n,
            cofactor: h,
            g,
            d: 0,
            q: Pt4::INF,
            omega: 0,
            lambda: 0,
            x0,
            c,
            gls: Some((f.from_fp(p - 1), f.from_fp(i_fp))),
        };
        let psi_g = inst.psi_gls(&curve, &g);
        if !curve.on_curve(&psi_g) {
            return Err("ψ(G) is not on the curve".into());
        }
        let s = sqrt_mod(n - 1, n).ok_or("−1 is not a square modulo n")?;
        let Some(&lambda) = [s, n - s].iter().find(|&&l| curve.mul(&g, l) == psi_g) else {
            return Err("neither square root of −1 is ψ's eigenvalue".into());
        };
        inst.lambda = lambda;
        inst.d = rng.gen_range(1..n);
        inst.q = curve.mul(&g, inst.d);
        best = Some(inst);
    }
    best.ok_or_else(|| format!("no GLS τ_T instance with h ≤ {max_cofactor} at p = {p}"))
}

/// E28's base on the line `x = x₀ + u`, `u ∈ F_p ∖ {0, ±c, ±ic}`, folded by
/// `τ_T` (`u ↦ c²/u`, coefficient `1`) and, with `fold_psi`, by `ψ`
/// (`u ↦ −u`, coefficient `λ`) and `τ_Tψ` (`u ↦ −c²/u`, `λ`).  Every image
/// is computed on points and must be a base point.  The points are the same
/// for both folds.
pub fn gls_tau_base(
    inst: &FoldInstance4,
    rng: &mut StdRng,
    fold_psi: bool,
) -> Result<FoldedBase4, String> {
    let curve = inst.curve();
    let f = &curve.f;
    let (p, n, c) = (inst.p, inst.n, inst.c);
    let i_fp = inst.gls.ok_or("not a GLS τ_T instance")?.1 .0[0];
    let ic = mulm(i_fp, c, p);
    let mut points = Vec::new();
    let mut key: Vec<u64> = Vec::new();
    for u in 1..p {
        if u == c || u == p - c || u == ic || u == p - ic {
            continue;
        }
        let x = f.add(&inst.x0, &f.from_fp(u));
        if let Some(pt) = curve.lift_x(&x, rng) {
            points.push(pt);
            key.push(u);
        }
    }
    let index: HashMap<u64, usize> = key.iter().enumerate().map(|(i, &k)| (k, i)).collect();
    let t = Pt4 {
        x: inst.x0,
        y: E4::ZERO,
        inf: false,
    };
    let c2 = mulm(c, c, p);
    let mut col = vec![usize::MAX; points.len()];
    let mut coef = vec![0u64; points.len()];
    let mut columns = 0usize;
    for i in 0..points.len() {
        if col[i] != usize::MAX {
            continue;
        }
        let rep = points[i];
        col[i] = columns;
        coef[i] = 1;
        let u = key[i];
        let tu = mulm(c2, inv_mod(u, p), p);
        let mut images = vec![(tu, curve.add(&rep, &t), 1u64)];
        if fold_psi {
            let ps = inst.psi_gls(&curve, &rep);
            images.push(((p - u) % p, ps, inst.lambda % n));
            images.push(((p - tu) % p, curve.add(&ps, &t), inst.lambda % n));
        }
        for (k2, img, mult) in images {
            let Some(&j) = index.get(&k2) else {
                return Err(format!("the orbit of u = {u} leaves the base"));
            };
            if points[j].x != img.x {
                return Err("an image's abscissa is not the derived base point's".into());
            }
            let s = if points[j].y == img.y {
                mult
            } else if points[j].y == f.neg(&img.y) {
                (n - mult) % n
            } else {
                return Err("an image is neither P nor −P".into());
            };
            if col[j] != usize::MAX && (col[j] != columns || coef[j] != s) {
                return Err("an orbit closes inconsistently".into());
            }
            col[j] = columns;
            coef[j] = s;
        }
        columns += 1;
    }
    Ok(FoldedBase4 {
        points_per_column: points.len() as f64 / columns.max(1) as f64,
        points,
        col,
        coef,
        columns,
        build_muls: curve.f.muls(),
    })
}

/// One E28 Stage 0 row.
#[derive(Clone, Debug, Serialize)]
pub struct GlsTauStage0 {
    pub p: u64,
    pub seed: u64,
    pub n: u64,
    pub bits: f64,
    pub cofactor: u64,
    pub j_in_fp2: bool,
    pub psi_squared_is_minus_one: bool,
    pub psi_g_is_lambda_g: bool,
    pub lambda_squared_is_minus_one: bool,
    pub psi_fixes_t: bool,
    pub base_points: usize,
    pub density: f64,
    pub cols_tau: usize,
    pub cols_tau_psi: usize,
    pub points_per_column_tau: f64,
    pub points_per_column_tau_psi: f64,
    /// Every image checked on points (the base builds without error).
    pub line_checks: bool,
}

pub fn run_gls_tau_stage0(p: u64, seed: u64) -> Result<GlsTauStage0, String> {
    let inst = generate_gls_tau_instance(p, seed, 16)?;
    let curve = inst.curve();
    let f = &curve.f;
    let n = inst.n;
    let mut rng = StdRng::seed_from_u64(seed ^ 0xE28B_A5E0);
    let pts: Vec<Pt4> = (0..8).map(|_| random_point(&curve, &mut rng)).collect();
    let psi2 = pts.iter().all(|pt| {
        let q = inst.psi_gls(&curve, &inst.psi_gls(&curve, pt));
        q == curve.neg(pt)
    });
    let t = Pt4 {
        x: inst.x0,
        y: E4::ZERO,
        inf: false,
    };
    let psi_t = inst.psi_gls(&curve, &t) == t;
    let psi_g = inst.psi_gls(&curve, &inst.g) == curve.mul(&inst.g, inst.lambda);
    let lam2 = mulm(inst.lambda, inst.lambda, n) == n - 1;
    let a3x4 = f.scale(&f.mul(&f.sq(&inst.a), &inst.a), 4);
    let disc = f.add(&a3x4, &f.scale(&f.sq(&inst.b), 27));
    let j = f.mul(&a3x4, &f.inv(&disc));
    let (tau, both) = (
        gls_tau_base(&inst, &mut StdRng::seed_from_u64(seed), false),
        gls_tau_base(&inst, &mut StdRng::seed_from_u64(seed), true),
    );
    let ok = tau.is_ok() && both.is_ok();
    let (tau, both) = (
        tau.unwrap_or_else(|_| empty_base()),
        both.unwrap_or_else(|_| empty_base()),
    );
    Ok(GlsTauStage0 {
        p,
        seed,
        n,
        bits: (n as f64).log2(),
        cofactor: inst.cofactor,
        j_in_fp2: !not_in_fp2(&j),
        psi_squared_is_minus_one: psi2,
        psi_g_is_lambda_g: psi_g,
        lambda_squared_is_minus_one: lam2,
        psi_fixes_t: psi_t,
        base_points: tau.points.len(),
        density: tau.points.len() as f64 / (p - 5) as f64,
        cols_tau: tau.columns,
        cols_tau_psi: both.columns,
        points_per_column_tau: tau.points_per_column,
        points_per_column_tau_psi: both.points_per_column,
        line_checks: ok && tau.points.iter().zip(&both.points).all(|(a, b)| a == b),
    })
}

fn empty_base() -> FoldedBase4 {
    FoldedBase4 {
        points: Vec::new(),
        col: Vec::new(),
        coef: Vec::new(),
        columns: 0,
        points_per_column: 0.0,
        build_muls: 0,
    }
}

/// One weighted `D₄` solve of the solve-cost control (§9.26, G2.102).
#[derive(Clone, Debug, Serialize)]
pub struct D4ControlCall {
    pub solved: bool,
    pub outcome: String,
    pub degree: u16,
    /// Weil restriction plus solve, `F_p` multiplications (E19's `C`).
    pub call_muls: u64,
    pub decompositions: usize,
    pub mitm_decompositions: usize,
    /// The solve's verified decomposition set equals the oracle's.
    pub agrees: bool,
    pub ms: f64,
}

/// E28 Stage A: one curve, both folds on one stream (E18's protocol).
#[derive(Clone, Debug, Serialize)]
pub struct GlsTauStageA {
    pub p: u64,
    pub seed: u64,
    pub n: u64,
    pub bits: f64,
    pub cofactor: u64,
    pub base_points: usize,
    pub residuals: u64,
    pub decompositions: u64,
    pub targets_decomposed: u64,
    pub decompositions_per_target: [u64; 10],
    /// `F_p` multiplications per target generated (`aG + bQ`).
    pub generation_muls_per_target: f64,
    pub tau: ArmReport,
    pub tau_psi: ArmReport,
    pub control_calls: Vec<D4ControlCall>,
    /// Walks priced by E25's arithmetic (serial) and by E27's (batched).
    pub rho_schoolbook: Vec<RhoRun4F>,
    pub rho_e27: Vec<RhoRun4F>,
    pub wall_ms: f64,
}

/// A decomposition as a sorted multiset of signed base indices, so that
/// the solve and the oracle can be compared.
fn dec_key(terms: &[(usize, i64)]) -> Vec<(usize, i64)> {
    let mut v = terms.to_vec();
    v.sort_unstable();
    v
}

pub fn run_gls_tau_stage_a(
    p: u64,
    seed: u64,
    rho_runs: usize,
    control_targets: usize,
    max_residuals: u64,
) -> Result<GlsTauStageA, String> {
    let start = Instant::now();
    let inst = generate_gls_tau_instance(p, seed, 16)?;
    let curve = inst.curve();
    let n = inst.n;
    let mut rng = StdRng::seed_from_u64(seed ^ 0xE28A_57A6);
    let fb_tau = gls_tau_base(&inst, &mut StdRng::seed_from_u64(seed), false)?;
    let fb_psi = gls_tau_base(&inst, &mut StdRng::seed_from_u64(seed), true)?;
    assert_eq!(fb_tau.points, fb_psi.points);
    let base = &fb_tau.points;
    let table = pair_table(&curve, base);
    let mut tau = Arm::new("tau", fb_tau.columns);
    let mut tau_psi = Arm::new("tau-psi", fb_psi.columns);
    let (mut residuals, mut decompositions, mut targets_decomposed) = (0u64, 0u64, 0u64);
    let mut per_target = [0u64; 10];
    let mut gen_muls = 0u64;
    let fold_row = |fb: &FoldedBase4, terms: &[(usize, i64)], a: u64, b: u64, small: usize| {
        let mut merged: HashMap<usize, u64> = HashMap::new();
        for &(i, s) in terms {
            let v = if s == 1 {
                fb.coef[i]
            } else {
                (n - fb.coef[i]) % n
            };
            let e = merged.entry(fb.col[i]).or_insert(0);
            *e = (*e + v) % n;
        }
        let mut cols: Vec<(usize, u64)> = merged.into_iter().filter(|&(_, v)| v != 0).collect();
        cols.push((small, (n - b) % n));
        cols.sort_unstable();
        SparseRel { cols, rhs: a }
    };
    while residuals < max_residuals && !(tau.rep.solved && tau_psi.rep.solved) {
        let batch: Vec<(u64, u64)> = (0..64)
            .map(|_| (rng.gen_range(0..n), rng.gen_range(1..n)))
            .collect();
        let found: Vec<(u64, u64, u64, Vec<Vec<(usize, i64)>>)> = batch
            .par_iter()
            .map(|&(a, b)| {
                let c = inst.curve();
                let r = c.add(&c.mul(&inst.g, a), &c.mul(&inst.q, b));
                let gm = c.f.muls();
                let decs = mitm_signed(&c, base, &table, &r);
                let decs: Vec<Vec<(usize, i64)>> = decs
                    .into_iter()
                    .filter(|terms| {
                        let mut acc = r;
                        for &(i, s) in terms {
                            acc = if s == 1 {
                                c.sub(&acc, &base[i])
                            } else {
                                c.add(&acc, &base[i])
                            };
                        }
                        acc.inf
                    })
                    .collect();
                (a, b, gm, decs)
            })
            .collect();
        for (a, b, gm, mut good) in found {
            residuals += 1;
            gen_muls += gm;
            decompositions += good.len() as u64;
            per_target[good.len().min(9)] += 1;
            if !good.is_empty() {
                targets_decomposed += 1;
                let terms = good.swap_remove(rng.gen_range(0..good.len()));
                if !tau.rep.solved {
                    tau.push(fold_row(&fb_tau, &terms, a, b, tau.small));
                }
                if !tau_psi.rep.solved {
                    tau_psi.push(fold_row(&fb_psi, &terms, a, b, tau_psi.small));
                }
            }
            tau.attempt(&inst, &curve, &mut rng, residuals);
            tau_psi.attempt(&inst, &curve, &mut rng, residuals);
        }
    }
    if !(tau.rep.solved && tau_psi.rep.solved) {
        return Err(format!(
            "p = {p} seed {seed}: no solve within {max_residuals} residuals"
        ));
    }
    // The solve-cost control: E19's weighted D₄ solve, unchanged.
    let d4 = D4Instance {
        curve: inst.curve(),
        x0: inst.x0,
        c: inst.c,
    };
    let mut crng = StdRng::seed_from_u64(seed ^ 0xE28C_0571);
    let pre = SymmetrisedD4::precompute(&d4, &mut crng, 8)?;
    let mut by_y: HashMap<u64, usize> = HashMap::new();
    for (i, pt) in base.iter().enumerate() {
        let y = d4.y_of_x(&pt.x);
        if !y.in_fp() {
            return Err("a base point's Y is not in F_p".into());
        }
        by_y.insert(y.0[0], i);
    }
    let control_calls: Vec<D4ControlCall> = (0..control_targets as u64)
        .map(|k| {
            let (a, b) = (crng.gen_range(0..n), crng.gen_range(1..n));
            let r = curve.add(&curve.mul(&inst.g, a), &curve.mul(&inst.q, b));
            let out = d4_decompose(&inst, &pre, base, &by_y, &r, seed * 7919 + k);
            let oracle: std::collections::BTreeSet<Vec<(usize, i64)>> =
                mitm_signed(&curve, base, &table, &r)
                    .iter()
                    .filter(|terms| {
                        let mut acc = r;
                        for &(i, s) in terms.iter() {
                            acc = if s == 1 {
                                curve.sub(&acc, &base[i])
                            } else {
                                curve.add(&acc, &base[i])
                            };
                        }
                        acc.inf
                    })
                    .map(|t| dec_key(t))
                    .collect();
            let solve: std::collections::BTreeSet<Vec<(usize, i64)>> =
                out.decs.iter().map(|t| dec_key(t)).collect();
            D4ControlCall {
                solved: out.solved,
                outcome: out.outcome.clone(),
                degree: out.degree,
                call_muls: out.call_muls,
                decompositions: solve.len(),
                mitm_decompositions: oracle.len(),
                agrees: !out.solved || solve == oracle,
                ms: out.ms,
            }
        })
        .collect();
    let seeds: Vec<u64> = (0..rho_runs as u64).map(|k| seed * 1000 + k).collect();
    let mut rho_schoolbook = Vec::new();
    let mut rho_e27 = Vec::new();
    for fold in [RhoFold::Gls, RhoFold::Negation] {
        rho_schoolbook.extend(
            seeds
                .par_iter()
                .map(|&s| rho4_folded_with(&inst, fold, s, false))
                .collect::<Vec<_>>(),
        );
        rho_e27.extend(
            rho4_folded_batched(&inst, fold, &seeds, 16, seed * 1_000_000 + 500_000, true).0,
        );
    }
    Ok(GlsTauStageA {
        p,
        seed,
        n,
        bits: (n as f64).log2(),
        cofactor: inst.cofactor,
        base_points: base.len(),
        residuals,
        decompositions,
        targets_decomposed,
        decompositions_per_target: per_target,
        generation_muls_per_target: gen_muls as f64 / residuals.max(1) as f64,
        tau: tau.rep,
        tau_psi: tau_psi.rep,
        control_calls,
        rho_schoolbook,
        rho_e27,
        wall_ms: start.elapsed().as_secs_f64() * 1e3,
    })
}

// ── E29 (§9.27): the order-24 twist action on E18's `j = 0` line ────────

/// The order of `x` in `(Z/n)^*` given that it divides `m` (`m | n − 1`).
fn order_dividing(x: u64, m: u64, n: u64) -> u64 {
    let mut best = m;
    for &(q, _) in factor_u64(m).iter() {
        while best.is_multiple_of(q) && pow_mod(x, best / q, n) == 1 {
            best /= q;
        }
    }
    best
}

/// The order of `y² = x³ + b₀t³` over `F_{p⁴}` (the lane's formula in
/// [`construct_twist24`]), without building the curve.
pub fn twist24_order(p: u64, b0: u64) -> Option<u64> {
    let mut count = 1u64;
    for x in 0..p {
        let v = (pow_mod(x, 3, p) + b0) % p;
        count += if v == 0 {
            1
        } else if pow_mod(v, (p - 1) / 2, p) == 1 {
            2
        } else {
            0
        };
    }
    let tr = (p + 1) as i128 - count as i128;
    let tr2 = tr * tr - 2 * p as i128;
    let tr4 = tr2 * tr2 - 2 * (p as i128).pow(2);
    let order = (p as i128).pow(4) + 1 + tr4;
    (order > 0 && order <= u64::MAX as i128).then_some(order as u64)
}

/// One E29 kill-test row (§9.27, G2.104).
#[derive(Clone, Debug, Serialize)]
pub struct E29Kill {
    pub p: u64,
    pub seed: u64,
    pub n: u64,
    pub bits: f64,
    /// `ψ_p(x, y) = (u²x^p, u³y^p)`, `u = b^{−(p−1)/6}`, maps random points
    /// of `E` to points of `E`.
    pub psi_p_on_curve: bool,
    /// `ψ_p(G) = [λ_p]G` with `λ_p` a 24th root of unity mod `n`.
    pub lambda_p_found: bool,
    pub order_lambda_p: u64,
    /// `χ₆(b) = b^{(p⁴−1)/6}` and its order (derived: `ord ψ_p = 4·ord χ₆`).
    pub order_chi6_b: u64,
    pub order_lambda_omega: u64,
    /// `|⟨λ_p, λ_ω, −1⟩|` in `(Z/n)^*`.
    pub group_order: u64,
    /// Whether `u² ∈ F_p` (then `ψ_p` keeps E18's base `x ∈ F_p`).
    pub u2_in_fp: bool,
    /// `b₀` with `#(y² = x³ + b₀t³) = n` (the least), if any.
    pub canonical_b0: Option<u64>,
    pub n_mod_24: u64,
    pub n_mod_96: u64,
}

pub fn run_e29_kill(p: u64, seed: u64) -> Result<E29Kill, String> {
    let inst = generate_psi_instance(p, seed)?;
    let curve = inst.curve();
    let f = &curve.f;
    let n = inst.n;
    let q4 = (p as u128).pow(4) - 1;
    // u = b^{−(p−1)/6}.
    let u = f.pow(&inst.b, q4 - (p as u128 - 1) / 6);
    let (u2, u3) = {
        let u2 = f.sq(&u);
        (u2, f.mul(&u2, &u))
    };
    let z = f.pow(&E4([0, 1, 0, 0]), (p - 1) as u128);
    if !z.in_fp() {
        return Err("t^{p−1} is not in F_p".into());
    }
    let psi_p = |pt: &Pt4| -> Pt4 {
        if pt.inf {
            return *pt;
        }
        Pt4 {
            x: f.mul(&u2, &f.frob_p(&pt.x, z.0[0])),
            y: f.mul(&u3, &f.frob_p(&pt.y, z.0[0])),
            inf: false,
        }
    };
    let mut rng = StdRng::seed_from_u64(seed ^ 0xE29C_4111);
    let on_curve = (0..8).all(|_| {
        let pt = random_point(&curve, &mut rng);
        curve.on_curve(&psi_p(&pt))
    });
    let chi = f.pow(&inst.b, q4 / 6);
    let order_chi6 = (1..=6u128)
        .find(|&k| f.pow(&chi, k) == E4::ONE)
        .unwrap_or(0) as u64;
    // A generator of the 24th roots of unity mod n.
    let zeta = (2..n)
        .map(|g| pow_mod(g, (n - 1) / 24, n))
        .find(|&w| order_dividing(w, 24, n) == 24)
        .ok_or("no element of order 24 mod n")?;
    let pg = psi_p(&inst.g);
    let k_p = (0..24u64).find(|&k| curve.mul(&inst.g, pow_mod(zeta, k, n)) == pg);
    let k_w = (0..24u64)
        .find(|&k| pow_mod(zeta, k, n) == inst.lambda)
        .ok_or("λ_ω is not a 24th root of unity")?;
    let (order_lambda_p, group_order) = match k_p {
        Some(k) => {
            let g = [k, k_w, 12, 24].iter().fold(0u64, |a, &b| gcd(a, b));
            (24 / gcd(k, 24), 24 / g)
        }
        None => (0, 0),
    };
    let canonical_b0 = (1..p).find(|&b0| twist24_order(p, b0) == Some(n));
    Ok(E29Kill {
        p,
        seed,
        n,
        bits: (n as f64).log2(),
        psi_p_on_curve: on_curve,
        lambda_p_found: k_p.is_some(),
        order_lambda_p,
        order_chi6_b: order_chi6,
        order_lambda_omega: 24 / gcd(k_w, 24),
        group_order,
        u2_in_fp: u2.in_fp(),
        canonical_b0,
        n_mod_24: n % 24,
        n_mod_96: n % 96,
    })
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// E29 Stage R: order-6 and order-24 walks on one canonical model.
#[derive(Clone, Debug, Serialize)]
pub struct E29StageR {
    pub p: u64,
    pub b0: u64,
    pub n: u64,
    pub bits: f64,
    pub karatsuba: bool,
    pub order6: Vec<RhoRun4F>,
    pub order24: Vec<RhoRun4F>,
}

pub fn run_e29_stage_r(
    p: u64,
    b0: u64,
    seed: u64,
    walks: usize,
    karatsuba: bool,
) -> Result<E29StageR, String> {
    let inst = construct_twist24(p, b0, seed)?;
    let seeds: Vec<u64> = (0..walks as u64).map(|k| seed * 1000 + k).collect();
    let order6: Vec<RhoRun4F> = seeds
        .par_iter()
        .map(|&s| rho4_folded_with(&inst.core, RhoFold::Psi, s, karatsuba))
        .collect();
    let order24: Vec<RhoRun4F> = seeds
        .par_iter()
        .map(|&s| rho_twist24_with(&inst, s, karatsuba))
        .collect();
    Ok(E29StageR {
        p,
        b0,
        n: inst.core.n,
        bits: (inst.core.n as f64).log2(),
        karatsuba,
        order6,
        order24,
    })
}

/// E28 supplementary control (§9.30, a recorded deviation): the weighted
/// `D₄` solve on targets the exact oracle does decompose, so that the
/// agreement check is not vacuous.  Draws uniform targets until `k` have a
/// verified decomposition and reports how many were drawn.
pub fn run_gls_tau_control_decomposed(
    p: u64,
    seed: u64,
    k: usize,
) -> Result<(Vec<D4ControlCall>, u64), String> {
    let inst = generate_gls_tau_instance(p, seed, 16)?;
    let curve = inst.curve();
    let n = inst.n;
    let fb = gls_tau_base(&inst, &mut StdRng::seed_from_u64(seed), false)?;
    let base = &fb.points;
    let table = pair_table(&curve, base);
    let d4 = D4Instance {
        curve: inst.curve(),
        x0: inst.x0,
        c: inst.c,
    };
    let mut crng = StdRng::seed_from_u64(seed ^ 0xE28D_EC05);
    let pre = SymmetrisedD4::precompute(&d4, &mut crng, 8)?;
    let mut by_y: HashMap<u64, usize> = HashMap::new();
    for (i, pt) in base.iter().enumerate() {
        by_y.insert(d4.y_of_x(&pt.x).0[0], i);
    }
    let verified = |r: &Pt4| -> std::collections::BTreeSet<Vec<(usize, i64)>> {
        mitm_signed(&curve, base, &table, r)
            .iter()
            .filter(|terms| {
                let mut acc = *r;
                for &(i, s) in terms.iter() {
                    acc = if s == 1 {
                        curve.sub(&acc, &base[i])
                    } else {
                        curve.add(&acc, &base[i])
                    };
                }
                acc.inf
            })
            .map(|t| dec_key(t))
            .collect()
    };
    let (mut calls, mut drawn) = (Vec::new(), 0u64);
    while calls.len() < k && drawn < 1_000_000 {
        drawn += 1;
        let (a, b) = (crng.gen_range(0..n), crng.gen_range(1..n));
        let r = curve.add(&curve.mul(&inst.g, a), &curve.mul(&inst.q, b));
        let oracle = verified(&r);
        if oracle.is_empty() {
            continue;
        }
        let out = d4_decompose(&inst, &pre, base, &by_y, &r, seed * 104_729 + drawn);
        let solve: std::collections::BTreeSet<Vec<(usize, i64)>> =
            out.decs.iter().map(|t| dec_key(t)).collect();
        calls.push(D4ControlCall {
            solved: out.solved,
            outcome: out.outcome.clone(),
            degree: out.degree,
            call_muls: out.call_muls,
            decompositions: solve.len(),
            mitm_decompositions: oracle.len(),
            agrees: out.solved && solve == oracle,
            ms: out.ms,
        });
    }
    Ok((calls, drawn))
}

/// E28–E30 (§9.26–§9.28): the constructions and the audit, with negative
/// controls.
#[cfg(test)]
mod e28_e30_tests {
    use super::*;
    use crate::cryptanalysis::gaudry_quartic::{audited_product_cost, Fp4};

    #[test]
    fn e28_psi_keeps_t_and_the_line_and_a_wrong_sign_does_not() {
        let inst = generate_gls_tau_instance(53, 2, 16).unwrap();
        let curve = inst.curve();
        let base = gls_tau_base(&inst, &mut StdRng::seed_from_u64(2), true).unwrap();
        assert!((base.points_per_column - 4.0).abs() < 1e-9);
        let t = Pt4 {
            x: inst.x0,
            y: E4::ZERO,
            inf: false,
        };
        assert_eq!(inst.psi_gls(&curve, &t), t);
        assert_eq!(
            inst.psi_gls(&curve, &inst.g),
            curve.mul(&inst.g, inst.lambda)
        );
        // Negative control: x ↦ +σ(x) is not an endomorphism of E.
        let (_, cy) = inst.gls.unwrap();
        let mut bad = inst;
        bad.gls = Some((curve.f.from_fp(1), cy));
        let img = bad.psi_gls(&curve, &inst.g);
        assert!(!curve.on_curve(&img) || img != curve.mul(&inst.g, inst.lambda));
    }

    #[test]
    fn e29_psi_p_has_order_24_and_a_wrong_twist_factor_fails() {
        let k = run_e29_kill(241, 1).unwrap();
        assert!(k.psi_p_on_curve && k.lambda_p_found);
        assert_eq!(
            (k.order_lambda_p, k.order_chi6_b, k.group_order),
            (24, 6, 24)
        );
        assert_eq!(twist24_order(241, k.canonical_b0.unwrap()), Some(k.n));
        // Negative control: u = b^{−(p−1)/3} (a cube root, not a sixth root of
        // b^{1−p}) does not give a map onto E.
        let inst = generate_psi_instance(241, 1).unwrap();
        let curve = inst.curve();
        let f = &curve.f;
        let q4 = 241u128.pow(4) - 1;
        let u = f.pow(&inst.b, q4 - 240 / 3);
        let z = f.pow(&E4([0, 1, 0, 0]), 240).0[0];
        let pt = inst.g;
        let img = Pt4 {
            x: f.mul(&f.sq(&u), &f.frob_p(&pt.x, z)),
            y: f.mul(&f.mul(&f.sq(&u), &u), &f.frob_p(&pt.y, z)),
            inf: false,
        };
        assert!(!curve.on_curve(&img));
    }

    #[test]
    fn e30_audit_reprices_only_trivial_products() {
        let f = Fp4::with_arith(101, true);
        let mut rng = StdRng::seed_from_u64(1);
        let a = loop {
            let a = f.random(&mut rng);
            if !a.in_fp() {
                break a;
            }
        };
        assert_eq!(audited_product_cost(&a, &E4::ONE, 13), 0);
        assert_eq!(audited_product_cost(&E4::ZERO, &a, 19), 0);
        assert_eq!(audited_product_cost(&a, &f.from_fp(7), 13), 4);
        // Negative control: two generic elements keep their full price.
        let b = loop {
            let b = f.random(&mut rng);
            if !b.in_fp() {
                break b;
            }
        };
        assert_eq!(audited_product_cost(&a, &b, 13), 13);
        assert_eq!(audited_product_cost(&a, &b, 19), 19);
    }
}
