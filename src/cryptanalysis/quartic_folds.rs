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

use super::gaudry_cubic::{square_core, wiedemann_u64, SparseRel};
use super::gaudry_quartic::{mitm_signed, pair_table, Curve4, Pt4, E4};
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
}

impl Family4 {
    pub fn name(self) -> &'static str {
        match self {
            Self::Psi => "j0-psi",
            Self::Translation => "2torsion-tau",
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
}

impl FoldInstance4 {
    /// A fresh curve with its own counters.
    pub fn curve(&self) -> Curve4 {
        Curve4::with_params(self.p, self.a, self.b)
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
}

/// Build the base and check every column coefficient on points.
pub fn folded_base(inst: &FoldInstance4, rng: &mut StdRng) -> Result<FoldedBase4, String> {
    let curve = inst.curve();
    let f = &curve.f;
    let p = inst.p;
    let n = inst.n;
    let mut points = Vec::new();
    let mut key: Vec<u64> = Vec::new(); // the F_p parameter of each point
    match inst.family {
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
}

/// An r-adding walk with exhaustive storage on class representatives.
/// A step whose successor would land in its own partition takes the next
/// partition instead (the look-ahead that keeps a folded walk out of
/// fruitless 2-cycles); a self-collision — a longer fruitless cycle — is
/// escaped by doubling the current point.  Every group operation and field
/// multiplication is charged, set-up included.
pub fn rho4_folded(inst: &FoldInstance4, fold: RhoFold, seed: u64) -> RhoRun4F {
    assert!(fold != RhoFold::Psi || inst.family == Family4::Psi);
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
            RhoFold::Twist24 => unreachable!("use rho_twist24 for this action"),
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
    let correct = found == Some(inst.d) && found.is_some_and(|d| curve.mul(&inst.g, d) == inst.q);
    RhoRun4F {
        fold,
        seed,
        steps,
        group_ops: curve.ops(),
        fp_muls: f.muls(),
        cycle_escapes,
        correct,
        recovered: found,
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
                image = inst.frob(&curve, &image);
            }
            let fj = pow_mod(inst.lambda_frob, j, inst.core.n);
            for k in 0..3 {
                let transformed = inst.core.psi(&curve, &image, k);
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
    }
}

pub fn twist24_base(inst: &Twist24Instance, seed: u64, combined: bool) -> FoldedBase4 {
    let curve = inst.core.curve();
    twist24_columns(inst, &curve, twist24_points(inst, &curve, seed), combined)
}

/// A matched rho walk on all 24 classes.  It uses E18's jump table,
/// look-ahead, fruitless-cycle escape and charging convention.
pub fn rho_twist24(inst: &Twist24Instance, seed: u64) -> RhoRun4F {
    let curve = inst.core.curve();
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
