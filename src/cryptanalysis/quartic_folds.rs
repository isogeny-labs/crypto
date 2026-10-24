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
use super::gaudry_quartic::{mitm_signed, pair_table, Curve4, Pt4, QuarticSolve, E4};
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
}

/// An r-adding walk with exhaustive storage on class representatives.
/// A step whose successor would land in its own partition takes the next
/// partition instead (the look-ahead that keeps a folded walk out of
/// fruitless 2-cycles); a self-collision — a longer fruitless cycle — is
/// escaped by doubling the current point.  Every group operation and field
/// multiplication is charged, set-up included.
pub fn rho4_folded(inst: &FoldInstance4, fold: RhoFold, seed: u64) -> RhoRun4F {
    assert!(fold != RhoFold::Psi || inst.family == Family4::Psi);
    assert!(fold != RhoFold::Gls || inst.gls.is_some());
    let curve = inst.curve();
    let f = &curve.f;
    let n = inst.n;
    let mut rng = StdRng::seed_from_u64(seed ^ 0x8D04_F01D);
    let r = 32usize;
    let lam = [1, inst.lambda % n, mulm(inst.lambda, inst.lambda, n)];
    let canon = |pt: Pt4| -> (Pt4, u64) {
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
                let img = inst.psi_gls(&curve, &pt);
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
                let mut best = (pt, 1u64);
                for (k, &l) in lam.iter().enumerate().skip(1) {
                    let img = inst.psi(&curve, &pt, k as u32);
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
        }
    };
    let part = |pt: &Pt4| -> usize {
        let h = pt.x.0[0] ^ pt.x.0[1].wrapping_mul(0x9E37_79B9) ^ pt.x.0[2].rotate_left(17);
        (mix64(h ^ pt.x.0[3].rotate_left(29)) % r as u64) as usize
    };
    let mults: Vec<(u64, u64, Pt4)> = (0..r)
        .map(|_| {
            let al = rng.gen_range(0..n);
            let be = rng.gen_range(1..n);
            (
                al,
                be,
                curve.add(&curve.mul(&inst.g, al), &curve.mul(&inst.q, be)),
            )
        })
        .collect();
    let mut table: HashMap<Pt4, (u64, u64)> = HashMap::new();
    let (mut steps, mut cycle_escapes) = (0u64, 0u64);
    let mut found = None;
    let cap = 64 * (n as f64).sqrt() as u64 + 1_000_000;
    let mut a = rng.gen_range(0..n);
    let mut b = rng.gen_range(1..n);
    let start = curve.add(&curve.mul(&inst.g, a), &curve.mul(&inst.q, b));
    let (mut l, m) = canon(start);
    a = mulm(a, m, n);
    b = mulm(b, m, n);
    while steps <= cap {
        steps += 1;
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
    RhoRun4F {
        fold,
        seed,
        steps,
        group_ops: curve.ops(),
        fp_muls: f.muls(),
        cycle_escapes,
        correct: found == Some(inst.d),
    }
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
    collect_and_solve(
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
    )
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
