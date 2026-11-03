//! `pkm-semaev`: a prime-field summation-polynomial decomposition oracle
//! that uses the factor base's algebraic description.
//!
//! The base is `prime-kummer:t=T[,g=G]`: every point whose abscissa lies in
//! `V = g·μ_{2^t}`, the zero set of `L(x) = (x/g)^{2^t} − 1`
//! (Petit–Kosters–Messeng, PKC 2016, §3).  For a target `R` the oracle
//! writes `S_{m+1}(x_1, …, x_m, x_R) = 0` with `x_i = g·y_{i,0}` and one
//! tower per summand, `y_{i,j}² = y_{i,j+1}`, `y_{i,t−1}² = 1`
//! (`research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md` §2.2), and
//! hands the system to F4 over the tower quotient ring
//! ([`f4_tower`](crate::cryptanalysis::f4_fp_tower::f4_tower)).
//!
//! - `mode=chain` (default, `m = 3`): `S₃(x_1, x_2, u_1)`, …,
//!   `S₃(u_{m−2}, x_m, x_R)` with free unknowns `u_k`.
//! - `mode=full` (`m = 2`, or `m = 3` on request): `S_{m+1}` itself; `S₄` is the resultant
//!   `Res_U(S₃(x_1, x_2, U), S₃(x_3, x_R, U))` by the two-quadratic formula.
//! - `engine=exhaustive` replaces F4 by the grid reference of the note's
//!   §4.4: every unordered `(x_1, …, x_m) ∈ V^m` is tested against
//!   `S_{m+1} = 0`, `F_p` multiplications counted.  It is the cross-check
//!   and the cost an algebraic engine has to beat on the same system.
//!
//! **What F4 decides, and what happens after.**  A refutation (`1` in the
//! basis) is the common case and ends the target.  A system F4 does not
//! refute is consistent over `F̄_p`: the solutions are then read off the
//! grid (the same counted search as the reference, run only on these
//! systems), every solution whose abscissae are all on the curve is lifted
//! by [`lift_abscissae`], which re-adds the summands on the counted ledger,
//! and the first sum equal to `R` is the relation.  A system F4 could not
//! decide within its bounds (`max_degree`, `max_nnz`, a budget) is counted
//! as `budget_exceeded` and decided on the grid, so the verdict stays exact
//! and the count says how often F4 did not carry it.
//!
//! **Accounting.**  Nothing here is a group operation except the lift, so
//! nearly all of the oracle's work is native and unpriced, and AGENTS.md
//! §5a prices it in the band:
//! - the solver block: F4's `F_p` multiply-adds (elimination only), the
//!   engine's own count ([`TowerF4Report::muladds`]);
//! - `pkm_system_fp_mults_uncharged`: specialising the prepared system at
//!   `x_R`, one multiplication a coefficient;
//! - `pkm_grid_fp_mults_uncharged`: the grid search (the whole decision for
//!   `engine=exhaustive`, the recovery after a consistent F4 verdict).
//!
//! The target-independent work (the symbolic system and its tower normal
//! forms) is done once in `prepare` and reported as setup native work.

use std::collections::{BTreeMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::fiber::{cofactor_fiber, CofactorFiber};
use super::prime_tower::primitive_two_power_root;
use super::stages::{
    DecompositionOracle, InstanceCtx, Params, SolverCost, SolverTotals, SystemShape,
};
use crate::cryptanalysis::f4_fp_tower::{
    f4_tower, mono, F4Trace, Fp, RPoly, SquareRule, TowerF4Options, TowerRing, MAX_FREE,
};
use crate::cryptanalysis::ic_boundary::{
    lift_abscissae, CountedGroup, FactorBase, GroupOps, OracleCounters, PrimeCurve, PrimePoint,
};

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn addm(a: u64, b: u64, p: u64) -> u64 {
    let s = a + b;
    if s >= p {
        s - p
    } else {
        s
    }
}
fn subm(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + p - b
    }
}

// ── A small symbolic polynomial over F_p ─────────────────────────────

/// Sparse multivariate polynomial, for building the system once.
#[derive(Clone, Debug)]
struct Pol {
    n: usize,
    terms: BTreeMap<Vec<u32>, u64>,
}

impl Pol {
    fn zero(n: usize) -> Self {
        Pol {
            n,
            terms: BTreeMap::new(),
        }
    }
    fn constant(n: usize, c: u64, p: u64) -> Self {
        let mut f = Pol::zero(n);
        if !c.is_multiple_of(p) {
            f.terms.insert(vec![0; n], c % p);
        }
        f
    }
    fn var(n: usize, i: usize) -> Self {
        let mut e = vec![0; n];
        e[i] = 1;
        let mut f = Pol::zero(n);
        f.terms.insert(e, 1);
        f
    }
    fn add(&self, o: &Pol, p: u64) -> Pol {
        let mut f = self.clone();
        for (e, c) in &o.terms {
            let v = f.terms.entry(e.clone()).or_insert(0);
            *v = addm(*v, *c, p);
        }
        f.terms.retain(|_, c| *c != 0);
        f
    }
    fn scale(&self, k: u64, p: u64) -> Pol {
        let mut f = Pol::zero(self.n);
        for (e, c) in &self.terms {
            let v = mulm(*c, k % p, p);
            if v != 0 {
                f.terms.insert(e.clone(), v);
            }
        }
        f
    }
    fn sub(&self, o: &Pol, p: u64) -> Pol {
        self.add(&o.scale(p - 1, p), p)
    }
    fn mul(&self, o: &Pol, p: u64) -> Pol {
        let mut f = Pol::zero(self.n);
        for (e1, c1) in &self.terms {
            for (e2, c2) in &o.terms {
                let e: Vec<u32> = e1.iter().zip(e2).map(|(a, b)| a + b).collect();
                let v = f.terms.entry(e).or_insert(0);
                *v = addm(*v, mulm(*c1, *c2, p), p);
            }
        }
        f.terms.retain(|_, c| *c != 0);
        f
    }
}

/// The three coefficients of `S₃(A, B, U)` as a quadratic in `U`:
/// `(A − B)²`, `−2((A + B)(AB + a) + 2b)`, `(AB − a)² − 4b(A + B)`.
fn s3_in_u(x1: &Pol, x2: &Pol, a: u64, b: u64, p: u64) -> [Pol; 3] {
    let n = x1.n;
    let d = x1.sub(x2, p);
    let s = x1.add(x2, p);
    let pr = x1.mul(x2, p);
    let beta = s
        .mul(&pr.add(&Pol::constant(n, a, p), p), p)
        .add(&Pol::constant(n, mulm(2, b, p), p), p)
        .scale(p - 2, p);
    let q = pr.sub(&Pol::constant(n, a, p), p);
    let gamma = q.mul(&q, p).sub(&s.scale(mulm(4, b, p), p), p);
    [d.mul(&d, p), beta, gamma]
}

/// `S₃(x1, x2, x3)` for `y² = x³ + ax + b`.
fn s3(x1: &Pol, x2: &Pol, x3: &Pol, a: u64, b: u64, p: u64) -> Pol {
    let [al, be, ga] = s3_in_u(x1, x2, a, b, p);
    al.mul(x3, p).mul(x3, p).add(&be.mul(x3, p), p).add(&ga, p)
}

/// `S₄(x1, x2, x3, x4) = Res_U(S₃(x1, x2, U), S₃(x3, x4, U))`, by
/// `(αγ′ − α′γ)² − (αβ′ − α′β)(βγ′ − β′γ)`.
fn s4(x1: &Pol, x2: &Pol, x3: &Pol, x4: &Pol, a: u64, b: u64, p: u64) -> Pol {
    let [al, be, ga] = s3_in_u(x1, x2, a, b, p);
    let [al2, be2, ga2] = s3_in_u(x3, x4, a, b, p);
    let ag = al.mul(&ga2, p).sub(&al2.mul(&ga, p), p);
    let ab = al.mul(&be2, p).sub(&al2.mul(&be, p), p);
    let bg = be.mul(&ga2, p).sub(&be2.mul(&ga, p), p);
    ag.mul(&ag, p).sub(&ab.mul(&bg, p), p)
}

/// The three coefficients of `S₃(x₁, x₂, U)` in `U` with `x_i = N_i/D_i`,
/// multiplied by `D₁²D₂²` so that they are polynomials:
/// `(N₁D₂ − N₂D₁)²`, `−2((N₁D₂ + N₂D₁)(N₁N₂ + aD₁D₂) + 2b(D₁D₂)²)` and
/// `(N₁N₂ − aD₁D₂)² − 4b(N₁D₂ + N₂D₁)D₁D₂`.  With `D = 1` these are the
/// coefficients of [`s3_in_u`].
fn s3_in_u_frac(
    (n1, d1): (&Pol, &Pol),
    (n2, d2): (&Pol, &Pol),
    a: u64,
    b: u64,
    p: u64,
) -> [Pol; 3] {
    let dd = d1.mul(d2, p);
    let (n1d2, n2d1) = (n1.mul(d2, p), n2.mul(d1, p));
    let cd = n1d2.sub(&n2d1, p);
    let cs = n1d2.add(&n2d1, p);
    let nn = n1.mul(n2, p);
    let al = cd.mul(&cd, p);
    let inner = cs
        .mul(&nn.add(&dd.scale(a, p), p), p)
        .add(&dd.mul(&dd, p).scale(mulm(2, b, p), p), p);
    let be = inner.scale(p - 2, p);
    let q = nn.sub(&dd.scale(a, p), p);
    let ga = q.mul(&q, p).sub(&cs.mul(&dd, p).scale(mulm(4, b, p), p), p);
    [al, be, ga]
}

/// `Res_U` of two quadratics given by their coefficient triples (the
/// combination used by [`s4`]).
fn resultant_of_triples(t1: &[Pol; 3], t2: &[Pol; 3], p: u64) -> Pol {
    let [al, be, ga] = t1;
    let [al2, be2, ga2] = t2;
    let ag = al.mul(ga2, p).sub(&al2.mul(ga, p), p);
    let ab = al.mul(be2, p).sub(&al2.mul(be, p), p);
    let bg = be.mul(ga2, p).sub(&be2.mul(ga, p), p);
    ag.mul(&ag, p).sub(&ab.mul(&bg, p), p)
}

fn poly_trim(v: &mut Vec<u64>) {
    while v.last() == Some(&0) {
        v.pop();
    }
}

/// `a mod b` over `F_p` (`b` non-zero, trimmed).
fn poly_rem(mut a: Vec<u64>, b: &[u64], p: u64) -> Vec<u64> {
    poly_trim(&mut a);
    let lead_inv = crate::cryptanalysis::residual_walk::pow_mod(*b.last().unwrap(), p - 2, p);
    while a.len() >= b.len() {
        let q = mulm(*a.last().unwrap(), lead_inv, p);
        let shift = a.len() - b.len();
        for (i, &c) in b.iter().enumerate() {
            a[shift + i] = subm(a[shift + i], mulm(q, c, p), p);
        }
        poly_trim(&mut a);
    }
    a
}

/// The Montgomery shift `A/3` of a curve `y² = x³ + ax + b` that came from
/// `y² = x³ + A x² + x`: the abscissa of the 2-torsion point `T₀ = (0, 0)`,
/// which is the unique root of `x³ + ax + b` in `F_p` when the 2-part of the
/// group is cyclic.  Exact: `gcd(x³ + ax + b, x^p − x)` by repeated squaring
/// modulo the cubic; an error unless it has degree one (no root, or three).
pub fn edwards_shift(curve: &PrimeCurve) -> Result<u64, String> {
    let p = curve.p;
    let f = vec![curve.b % p, curve.a % p, 0, 1];
    let mulmod = |u: &[u64], v: &[u64]| -> Vec<u64> {
        let mut r = vec![0u64; u.len() + v.len()];
        for (i, &x) in u.iter().enumerate() {
            for (j, &y) in v.iter().enumerate() {
                r[i + j] = addm(r[i + j], mulm(x, y, p), p);
            }
        }
        poly_rem(r, &f, p)
    };
    let (mut base, mut acc, mut e) = (vec![0u64, 1], vec![1u64], p);
    while e > 0 {
        if e & 1 == 1 {
            acc = mulmod(&acc, &base);
        }
        base = mulmod(&base, &base);
        e >>= 1;
    }
    // h = x^p − x
    acc.resize(acc.len().max(2), 0);
    acc[1] = subm(acc[1], 1, p);
    poly_trim(&mut acc);
    let (mut g, mut h) = (f, acc);
    while !h.is_empty() {
        let r = poly_rem(g, &h, p);
        g = h;
        h = r;
    }
    if g.len() != 2 {
        return Err(format!(
            "pkm-semaev: x³ + ax + b has {} rational roots, not one; pass edwards_shift=A/3",
            if g.len() == 4 { "three" } else { "no" }
        ));
    }
    let inv = crate::cryptanalysis::residual_walk::pow_mod(g[1], p - 2, p);
    Ok(mulm(subm(0, g[0], p), inv, p))
}

/// Weierstrass abscissae `x_W = (1 + y)/(1 − y) + shift` of `y ∈ [0, 2^t) ∖ {1}`
/// (`y = 1` is `u = ∞`), in the order of `y`.
pub fn edwards_abscissae(p: u64, t: u32, shift: u64) -> Vec<u64> {
    (0..(1u64 << t))
        .filter(|&y| y != 1)
        .map(|y| {
            let inv = crate::cryptanalysis::residual_walk::pow_mod(subm(1, y % p, p), p - 2, p);
            addm(mulm(addm(1, y, p), inv, p), shift, p)
        })
        .collect()
}

/// The harness-side base of an Edwards interval: every point of the curve
/// whose Edwards `y` lies in `[0, 2^t) ∖ {1}`, laid out like
/// `prime_factor_base` (one column per abscissa, `P` and `−P` adjacent).
pub fn edwards_factor_base(
    inst: &crate::cryptanalysis::ic_boundary::PrimeInstance,
    t: u32,
    shift: u64,
) -> FactorBase<PrimePoint> {
    let curve = &inst.curve;
    let mut xs = edwards_abscissae(curve.p, t, shift);
    xs.sort_unstable();
    xs.dedup();
    let mut fb = FactorBase::empty(format!("Edwards y in [0, 2^{t}), shift {shift}"));
    for x in xs {
        let rhs = curve.rhs(x);
        let Some(y) = curve.sqrt(rhs) else { continue };
        let y = if y == 0 { 0 } else { y.min(curve.p - y) };
        let pt = PrimePoint::affine(x, y);
        let q = curve.neg(pt);
        let col = fb.abscissae;
        let i = fb.points.len();
        fb.points.push(pt);
        fb.col_of.push(col);
        fb.coef_of.push(1);
        if q != pt {
            fb.points.push(q);
            fb.col_of.push(col);
            fb.coef_of.push(inst.r - 1);
            fb.neg_index.push(i + 1);
            fb.neg_index.push(i);
            fb.x_index.insert(x, vec![i, i + 1]);
        } else {
            fb.neg_index.push(i);
            fb.x_index.insert(x, vec![i]);
        }
        fb.abscissae += 1;
    }
    fb.columns = fb.abscissae;
    for (i, pt) in fb.points.iter().enumerate() {
        fb.point_index.insert(curve.key(pt), i);
    }
    fb.abscissa_list = fb.x_index.keys().copied().collect();
    fb.abscissa_list.sort_unstable();
    fb
}

// ── The oracle ───────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PkmMode {
    Chain,
    Full,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PkmEngine {
    F4,
    Exhaustive,
}

/// What a prepared run holds: the ring, the system split by powers of
/// `x_R`, and the grid.
struct Prepared {
    ring: TowerRing,
    /// `eqs[e][k]`: the part of equation `e` with `x_R^k`, in tower normal
    /// form.
    eqs: Vec<Vec<RPoly>>,
    tower_eqs: usize,
    degrees: Vec<u32>,
    /// `V`, in the order `g·ζ^k`.
    v: Vec<u64>,
}

/// How each summand's abscissa is tied to its `t` tower variables
/// `y_(i,0) … y_(i,t−1)`, i.e. how factor-base membership is imposed.
#[derive(Clone, Debug)]
pub enum Membership {
    /// `x = g·y_0`, `y_j² = y_(j+1)`, `y_(t−1)² = 1`: `V = g·μ_(2^t)`, which
    /// needs `2^t | p − 1` (PKM's Kummer form).
    Kummer,
    /// PKM's isogeny form: `x = y_0`, `y_(j+1) = y_j + τ_j/(y_j − ξ_j)` (a
    /// Vélu 2-isogeny x-map), `y_t = c`, cleared of denominators:
    /// `y_j² = y_j·y_(j+1) − ξ_j·y_(j+1) + ξ_j·y_j − τ_j` and, at the top,
    /// `y_(t−1)² = (c + ξ)·y_(t−1) − cξ − τ`.  No condition on `p`.
    /// `v` is the zero set `x(R' + ⟨Q⟩)` of `prime-isogeny`.
    Isogeny {
        v: Vec<u64>,
        steps: Vec<(u64, u64)>,
        c: u64,
    },
    /// Interval by bits: `x = lo + Σ_j 2^j·b_j`, `b_j² = b_j` (each bit a
    /// top-level tower variable): `V = [lo, lo + 2^t)`.  No condition on `p`.
    Bits { lo: u64 },
    /// Interval in Ed25519's Edwards coordinate: `y = Σ_j 2^j b_j`,
    /// `b_j² = b_j`, `y ∈ [0, 2^t) ∖ {1}`, and `x_W = (1 + y)/(1 − y) + s`
    /// (`s = A/3`, the abscissa of `T₀`; `None` until `prepare` derives it
    /// with [`edwards_shift`] or reads `edwards_shift=`).  The summands'
    /// denominators `(1 − y_i)²` are cleared from each `S₃`; `x_R` stays a
    /// variable.  `y = 1` is not in `V`, but the cleared system admits it
    /// when `x_R` is itself in `V` (then `S₃ = N²(x₂ − x_R)²` vanishes at
    /// `D₁ = 0, x₂ = x_R`).
    EdwardsBits { shift: Option<u64> },
}

/// The oracle.  Built by the harness with the base's `t` and `g`, which it
/// reads from the `prime-kummer` spec; `prepare` checks that every
/// abscissa of the base is in `V` and refuses any other base.  The
/// `prime-isogeny` base and an interval base (`bits=T`) select the other
/// [`Membership`] forms.
pub struct PkmSemaevOracle {
    m: u32,
    t: u32,
    g: u64,
    membership: Membership,
    mode: PkmMode,
    engine: PkmEngine,
    max_degree: u32,
    max_nnz: Option<u64>,
    stop_staircase: Option<usize>,
    budget: Option<Duration>,
    crosscheck: bool,
    /// Hybrid guessing (bits and isogeny forms): the top `guess` bits (or
    /// tower levels) of the first summand are enumerated, one F4 system per
    /// assignment.
    guess: u32,
    /// F4 trace (`trace=1`): learn one refutation trace per branch system at
    /// `prepare`, replay it per target, fall back to plain F4 when it fails.
    trace: bool,
    /// One learned trace per branch system (one entry without guessing).
    traces: Vec<Option<Arc<F4Trace>>>,
    /// Learning cost and shape counters, reported as setup native work.
    trace_setup: BTreeMap<String, u64>,
    /// Cofactor-fiber lifts (`lifts=1`): the rational `h`-torsion `E(F_q)[h]`,
    /// computed once in `prepare`; `decompose` tries the targets `R + K`.
    lifts: bool,
    /// `lifts=2`: ONE F4 system per target (per guess branch) for all the
    /// lifts. The third abscissa is a free ring variable `X`, constrained by
    /// `P_R(X) = Π (X − x(R+K))` over the distinct lift abscissae, built
    /// numerically per target. Set in `prepare` once the fiber is known
    /// (false when the fiber is trivial: then the oracle is the blind one).
    shared_req: bool,
    shared: bool,
    /// The lift targets `R + K` of the call in progress (shared system only),
    /// in the order `lifts=1` tries them.
    shared_pts: Vec<PrimePoint>,
    fiber: Option<CofactorFiber<PrimePoint>>,
    in_lift: bool,
    branches: Vec<Prepared>,
    prepared: Option<Prepared>,
    totals: SolverTotals,
    shape: Option<SystemShape>,
    setup_mults: u64,
    native: BTreeMap<String, u64>,
    // Opt-in dataset snapshots. Ordinary callers pay no JSON/trace cost.
    dataset_trace: bool,
    dataset_artifact: Option<serde_json::Value>,
}

impl PkmSemaevOracle {
    /// `m` summands over the Kummer tower `g·μ_{2^t}`.
    pub fn new(m: u32, t: u32, g: u64) -> Self {
        PkmSemaevOracle {
            m,
            t,
            g,
            membership: Membership::Kummer,
            mode: PkmMode::Chain,
            engine: PkmEngine::F4,
            max_degree: 12,
            max_nnz: None,
            stop_staircase: None,
            budget: None,
            crosscheck: false,
            guess: 0,
            trace: false,
            traces: Vec::new(),
            trace_setup: BTreeMap::new(),
            lifts: false,
            shared_req: false,
            shared: false,
            shared_pts: Vec::new(),
            fiber: None,
            in_lift: false,
            branches: Vec::new(),
            prepared: None,
            totals: SolverTotals {
                solver: "f4-fp-tower".into(),
                op_unit: "F_p multiply-adds (elimination only)".into(),
                ..Default::default()
            },
            shape: None,
            setup_mults: 0,
            native: BTreeMap::new(),
            dataset_trace: false,
            dataset_artifact: None,
        }
    }

    /// `m` summands on PKM's isogeny tower (`prime-isogeny:t=T`): `v` its
    /// zero set, `steps` the `(ξ_j, τ_j)` of the chain, `c = x(Φ(R'))`.
    pub fn new_isogeny(m: u32, t: u32, v: Vec<u64>, steps: Vec<(u64, u64)>, c: u64) -> Self {
        let mut o = Self::new(m, t, 1);
        o.membership = Membership::Isogeny { v, steps, c };
        o
    }

    /// `m` summands with abscissae in `[lo, lo + 2^t)`, written in bits.
    pub fn new_bits(m: u32, t: u32, lo: u64) -> Self {
        let mut o = Self::new(m, t, 1);
        o.membership = Membership::Bits { lo };
        o
    }

    /// The oracle a pipeline spec names, on the tower its factor base was
    /// built from: `prime-kummer:t,g` (the `μ_{2^t}` coset), PKM's isogeny
    /// form `prime-isogeny:t,seed,aux` (the tower is rebuilt
    /// deterministically from `(p, t, seed, aux)`; the rebuild is not
    /// charged, since the base's build on the factor-base ledger is the
    /// same work), or any base with `bits=T[,lo=L]` on the oracle (an
    /// interval written in bits). ecbench and `ic bench` both build the
    /// oracle here, so the two harnesses solve the same system.
    pub fn from_spec(
        m: u32,
        spec: &super::PipelineSpec,
        inst: &crate::cryptanalysis::ic_boundary::PrimeInstance,
    ) -> Result<Self, String> {
        use super::prime_tower::{isogeny_tower, supersingular_tower, AuxKind};
        let fb = &spec.factor_base_params;
        let oracle = match spec.factor_base.as_str() {
            "prime-kummer" => Self::new(m, fb.u64("t")? as u32, fb.u64_or("g", 1)?),
            "prime-isogeny" => {
                let p = inst.curve.p;
                let t = fb.u64("t")? as u32;
                let seed = fb.u64_or("seed", 0)?;
                let kind = AuxKind::parse(fb.get("aux").unwrap_or("ordinary"), p)?;
                let mut scratch = GroupOps::default();
                let tower = if kind == AuxKind::Ordinary {
                    isogeny_tower(p, t, seed, &mut scratch)?
                } else {
                    supersingular_tower(p, t, seed, kind, &mut scratch)?
                };
                let steps = tower.steps.iter().map(|s| (s.xi, s.tau)).collect();
                Self::new_isogeny(m, t, tower.v.clone(), steps, tower.c)
            }
            _ if spec.oracle_params.get("bits").is_some() => Self::new_bits(
                m,
                spec.oracle_params.u64("bits")? as u32,
                spec.oracle_params.u64_or("lo", 0)?,
            ),
            _ if spec.oracle_params.get("edwards_bits").is_some() => {
                Self::new_edwards_bits(m, spec.oracle_params.u64("edwards_bits")? as u32)
            }
            other => {
                return Err(format!(
                    "oracle `pkm-semaev` solves on prime-kummer or prime-isogeny, or on any base \
                     with `bits=T` (an interval [lo, lo + 2^T) written in bits) or \
                     `edwards_bits=T` (Edwards y in [0, 2^T)), not `{other}`"
                ))
            }
        };
        let budget = (spec.solver_budget_seconds > 0)
            .then(|| Duration::from_secs(spec.solver_budget_seconds));
        Ok(oracle.with_budget(budget))
    }

    /// `m` summands with Edwards `y ∈ [0, 2^t)` written in bits; the shift
    /// `A/3` is derived from the curve in `prepare` unless `edwards_shift=`.
    pub fn new_edwards_bits(m: u32, t: u32) -> Self {
        let mut o = Self::new(m, t, 1);
        o.membership = Membership::EdwardsBits { shift: None };
        o
    }

    /// A wall-clock budget per F4 call (the harness's
    /// `solver_budget_seconds`); `None` is unbounded.
    pub fn with_budget(mut self, budget: Option<Duration>) -> Self {
        self.budget = budget;
        self
    }

    /// Retain the exact prepared quotient system, native F4 report, and
    /// extracted roots for a dataset caller. Private elimination states are
    /// not exposed by the underlying engine and are explicitly identified.
    pub fn enable_dataset_trace(&mut self) {
        self.dataset_trace = true;
        self.dataset_artifact = None;
    }

    pub fn dataset_artifact(&self) -> Option<&serde_json::Value> {
        self.dataset_artifact.as_ref()
    }

    fn polynomial_artifact(poly: &RPoly) -> serde_json::Value {
        use crate::cryptanalysis::f4_fp_tower::{free_of, mask_of};
        serde_json::json!(poly
            .monos
            .iter()
            .zip(&poly.coefs)
            .map(|(&mono, &coefficient)| {
                serde_json::json!({"tower_mask_hex":format!("{:x}",mask_of(mono)),
                "free_exponents":free_of(mono),"coefficient":coefficient.to_string()})
            })
            .collect::<Vec<_>>())
    }

    /// Exact normal-form coefficients of each power of x_R. Together with
    /// the square rules this defines every target-specialized input exactly.
    pub fn prepared_system_artifact(&self) -> Option<serde_json::Value> {
        let prep = self.prepared.as_ref()?;
        Some(serde_json::json!({"schema":"pkm-prepared-system/v1",
            "modulus":prep.ring.p.to_string(),"m":self.m,"t":self.t,"g":self.g.to_string(),
            "mode":format!("{:?}",self.mode),"tower_variables":prep.ring.n_tower(),
            "free_variables":prep.ring.n_free,"variable_order":"tower blocks by summand, then chain auxiliaries",
            "summand_map":"x_i = g * y_(i,0)",
            "square_rule_convention":"y_j^2 = a*y_j*y_next + b*y_next + c*y_j + d",
            "square_rules":prep.ring.rules.iter().map(|r|serde_json::json!({"next":r.next,
                "a":r.a.to_string(),"b":r.b.to_string(),"c":r.c.to_string(),"d":r.d.to_string()})).collect::<Vec<_>>(),
            "equations_by_target_x_power":prep.eqs.iter().map(|parts|parts.iter().map(Self::polynomial_artifact).collect::<Vec<_>>()).collect::<Vec<_>>(),
            "grid_x":prep.v.iter().map(ToString::to_string).collect::<Vec<_>>(),"original_equation_degrees":prep.degrees,
            "original_equation_recipe":if self.mode==PkmMode::Chain{"chained S3 with affine intermediate x coordinates"}else{"S_(m+1), m=2 S3 or m=3 S4 resultant"},
            "trace_gaps":["private internal row/pivot operations and pair rejection witnesses"]}))
    }

    fn bump(&mut self, key: &str, by: u64) {
        *self.native.entry(key.to_string()).or_insert(0) += by;
    }

    fn read_params(&mut self, params: &Params) -> Result<(), String> {
        self.mode = match params.get("mode") {
            None if self.m >= 3 => PkmMode::Chain,
            None => PkmMode::Full,
            Some("chain") => PkmMode::Chain,
            Some("full") => PkmMode::Full,
            Some(o) => return Err(format!("pkm-semaev: mode is chain or full, not {o}")),
        };
        self.engine = match params.get("engine") {
            None | Some("f4") => PkmEngine::F4,
            Some("exhaustive") => PkmEngine::Exhaustive,
            Some(o) => return Err(format!("pkm-semaev: engine is f4 or exhaustive, not {o}")),
        };
        if self.engine == PkmEngine::Exhaustive {
            self.totals.solver = "tower-exhaustive".into();
        }
        self.max_degree = params.u64_or("degree", 12)? as u32;
        self.max_nnz = params
            .get("max_nnz")
            .map(|_| params.u64("max_nnz"))
            .transpose()?;
        self.stop_staircase = params
            .get("staircase")
            .map(|_| params.u64("staircase").map(|v| v as usize))
            .transpose()?;
        self.crosscheck = params.flag("crosscheck");
        self.guess = params.u64_or("guess", 0)? as u32;
        self.trace = match params.u64_or("trace", 0)? {
            0 => false,
            1 => true,
            o => return Err(format!("pkm-semaev: trace is 0 or 1, not {o}")),
        };
        if self.trace && (self.engine != PkmEngine::F4 || self.stop_staircase.is_some()) {
            return Err("pkm-semaev: trace=1 needs engine=f4 and no staircase stop".into());
        }
        let lifts = params.u64_or("lifts", 0)?;
        if lifts > 2 {
            return Err(format!("pkm-semaev: lifts is 0, 1 or 2, not {lifts}"));
        }
        self.lifts = lifts >= 1;
        self.shared_req = lifts == 2;
        if self.shared_req
            && (self.engine != PkmEngine::F4 || self.stop_staircase.is_some() || self.m != 2)
        {
            return Err(
                "pkm-semaev: lifts=2 (shared system) needs engine=f4, m=2 and no staircase stop"
                    .into(),
            );
        }
        if self.guess > 0 {
            if matches!(self.membership, Membership::Kummer) {
                return Err("pkm-semaev: guess=G applies to the bits and isogeny forms".into());
            }
            if self.guess >= self.t || self.guess > 12 {
                return Err(format!(
                    "pkm-semaev: guess = {} must be below the tower height {} and at most 12",
                    self.guess, self.t
                ));
            }
        }
        // The grid that reads a consistent system's solutions takes m ≤ 3.
        if !(2..=3).contains(&self.m) {
            return Err(format!("pkm-semaev: m is 2 or 3, not {}", self.m));
        }
        if self.mode == PkmMode::Chain && self.m < 3 {
            self.mode = PkmMode::Full; // m = 2: S₃ alone, no free unknown
        }
        if self.t == 0 || (self.m * self.t) as usize > 64 {
            return Err(format!(
                "pkm-semaev: m·t = {} tower variables; the engine takes 1 to 64",
                self.m * self.t
            ));
        }
        Ok(())
    }

    /// Build the ring and the system with `x_R` as a variable, then split
    /// by powers of `x_R`.
    fn finish_prepare(&mut self, curve: &PrimeCurve, check_mults: u64) -> Result<(), String> {
        let prep = self.build_system(curve);
        let n_vars = prep.ring.n_vars();
        let mut degrees = prep.degrees.clone();
        degrees.extend(std::iter::repeat_n(2, prep.tower_eqs));
        if self.shared {
            degrees.push(self.fiber.as_ref().map_or(1, |f| f.size() as u32));
        }
        self.shape = Some(SystemShape {
            n_vars,
            n_equations: prep.eqs.len() + prep.tower_eqs + usize::from(self.shared),
            degrees,
            semi_regular_degree: None,
        });
        self.setup_mults = check_mults;
        self.branches = (0..(1u64 << self.guess))
            .filter(|_| self.guess > 0)
            .map(|a| self.build_system_fixed(curve, Some(a)))
            .collect();
        self.prepared = Some(prep);
        self.traces.clear();
        self.trace_setup.clear();
        if self.trace {
            self.learn_traces();
        }
        Ok(())
    }

    /// The F4 options of a run (the budget clock starts here).
    fn f4_options(&self) -> TowerF4Options {
        let mut opts = TowerF4Options::new(self.max_degree);
        if let Some(b) = self.budget {
            opts = opts.with_budget(b);
        }
        if let Some(n) = self.max_nnz {
            opts = opts.with_max_nnz(n);
        }
        if let Some(s) = self.stop_staircase {
            opts = opts.stopping_below(s);
        }
        opts
    }

    /// Learn one refutation trace per branch system (one system without
    /// guessing): plain F4 with recording on `x_R` values drawn from a fixed
    /// stream, until one refutes (at most 8 tries). Every try's
    /// multiply-adds are charged to the learning ledger, once per system
    /// shape; a branch with no refuting try gets no trace and runs plain.
    fn learn_traces(&mut self) {
        let n = self.branches.len().max(1);
        let p = self.prepared.as_ref().expect("prepared").ring.p;
        let (mut learn, mut runs, mut learned, mut kept, mut spec, mut selfok) =
            (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
        for br in 0..n {
            let branch = (!self.branches.is_empty()).then_some(br);
            let mut found = None;
            for attempt in 0..8u64 {
                // splitmix64 of a fixed counter: the learning x_R do not
                // depend on the targets.
                let mut z = (br as u64 * 16 + attempt + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15);
                z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
                z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
                let x_r = (z ^ (z >> 31)) % p;
                let before = self.native.get("pkm_system_fp_mults_uncharged").copied();
                let shared_before = self
                    .native
                    .get("pkm_shared_poly_fp_mults_uncharged")
                    .copied();
                // Shared system: learn on a made-up P_R (fiber-size random
                // roots from the same fixed stream).
                let d = self.fiber.as_ref().map_or(1, |f| f.size() as u64);
                let learn_roots: Vec<u64> = (0..d)
                    .map(|j| {
                        let mut z = ((br as u64 * 16 + attempt + 1) * 64 + j + 1)
                            .wrapping_mul(0x9E37_79B9_7F4A_7C15);
                        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
                        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
                        (z ^ (z >> 31)) % p
                    })
                    .collect();
                let roots = self.shared.then_some(&learn_roots[..]);
                let sys = self.specialise_branch(branch, x_r, roots);
                match shared_before {
                    Some(b) => {
                        self.native
                            .insert("pkm_shared_poly_fp_mults_uncharged".into(), b);
                    }
                    None => {
                        self.native.remove("pkm_shared_poly_fp_mults_uncharged");
                    }
                }
                let after = self.native.get("pkm_system_fp_mults_uncharged").copied();
                spec += after.unwrap_or(0) - before.unwrap_or(0);
                // Not a per-target cost: take it back off the target ledger.
                match before {
                    Some(b) => {
                        self.native
                            .insert("pkm_system_fp_mults_uncharged".into(), b);
                    }
                    None => {
                        self.native.remove("pkm_system_fp_mults_uncharged");
                    }
                }
                let ring = match branch {
                    Some(b) => self.branches[b].ring.clone(),
                    None => self.prepared.as_ref().expect("prepared").ring.clone(),
                };
                let mut opts = self.f4_options();
                opts.record_trace = true;
                let rep = f4_tower(&sys, &ring, &opts);
                learn += rep.muladds;
                runs += 1;
                if let (true, Some(tr)) = (rep.inconsistent, rep.learned) {
                    kept += tr.kept_rows() as u64;
                    // Diagnostic, not charged: the trace replayed on its own x_R.
                    let mut ropts = self.f4_options();
                    ropts.replay = Some(Arc::new(tr.clone()));
                    let again = f4_tower(&sys, &ring, &ropts);
                    selfok += u64::from(again.inconsistent);
                    if let Some(m) = again.replay_mismatch {
                        self.trace_setup.insert(
                            format!("pkm_trace_self_mismatch_step{}_got{}_want{}", m.0, m.1, m.2),
                            1,
                        );
                    }
                    found = Some(Arc::new(tr));
                    learned += 1;
                    break;
                }
            }
            self.traces.push(found);
        }
        for (k, v) in [
            ("pkm_trace_branches", n as u64),
            ("pkm_trace_branches_learned", learned),
            ("pkm_trace_learn_runs", runs),
            ("pkm_trace_learn_muladds", learn),
            ("pkm_trace_kept_rows", kept),
            ("pkm_trace_selfcheck_refuted", selfok),
            ("pkm_trace_learn_spec_fp_mults_uncharged", spec),
        ] {
            self.trace_setup.insert(k.into(), v);
        }
    }

    /// Isogeny-form guessing: apply the first `t − G` Vélu maps to every
    /// element of `V` and group `V` by the value reached at level `t − G`.
    /// Returns the sorted `(c', members)` pairs, `2^G` of them; each group is
    /// the zero set of a sub-tower of height `t − G` whose top value is `c'`.
    fn isogeny_groups(&self, p: u64) -> Vec<(u64, Vec<u64>)> {
        let Membership::Isogeny { v, steps, .. } = &self.membership else {
            return Vec::new();
        };
        let lev = self.t as usize - self.guess as usize;
        let mut groups: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
        for &x in v {
            let mut y = x % p;
            for &(xi, tau) in &steps[..lev] {
                let d = subm(y, xi % p, p);
                assert!(
                    d != 0,
                    "pkm-semaev: a zero of the tower hits a kernel point"
                );
                let inv = crate::cryptanalysis::residual_walk::pow_mod(d, p - 2, p);
                y = addm(y, mulm(tau % p, inv, p), p);
            }
            groups.entry(y).or_default().push(x);
        }
        groups.into_iter().collect()
    }

    fn build_system(&self, curve: &PrimeCurve) -> Prepared {
        self.build_system_fixed(curve, None)
    }

    /// The system, with the top `guess` bits of the first summand fixed to
    /// `fixed` when it is given (bits form; hybrid guessing).
    fn build_system_fixed(&self, curve: &PrimeCurve, fixed: Option<u64>) -> Prepared {
        let p = curve.p;
        let (a, b) = (curve.a % p, curve.b % p);
        let (m, t) = (self.m as usize, self.t as usize);
        let n_t = m * t;
        // Isogeny guessing: the group (sub-tower) summand 1 is restricted to.
        let iso_group: Option<(u64, Vec<u64>)> = match (&self.membership, fixed) {
            (Membership::Isogeny { .. }, Some(a)) => {
                Some(self.isogeny_groups(p)[a as usize].clone())
            }
            _ => None,
        };
        let n_free_base = if self.mode == PkmMode::Chain {
            m - 2
        } else {
            0
        };
        // Shared system (`lifts=2`): the third abscissa is one more free
        // ring variable X (after the chain unknowns), not a parameter.
        let n_free = n_free_base + usize::from(self.shared);
        let n = n_t + n_free + 1; // last variable: x_R (unused when shared)
        let xr_var = n - 1;
        let x = |i: usize| match &self.membership {
            Membership::Kummer => Pol::var(n, i * t).scale(self.g, p),
            Membership::Isogeny { .. } => Pol::var(n, i * t),
            Membership::Bits { .. } | Membership::EdwardsBits { .. } => {
                let lo = match &self.membership {
                    Membership::Bits { lo } => lo,
                    _ => &0,
                };
                let g = self.guess as usize;
                let (base, free_bits) = match fixed {
                    Some(a) if i == 0 => ((lo + (a << (t - g))) % p, t - g),
                    _ => (lo % p, t),
                };
                let mut acc = Pol::constant(n, base, p);
                for j in 0..free_bits {
                    acc = acc.add(&Pol::var(n, i * t + j).scale((1u64 << j) % p, p), p);
                }
                acc
            }
        };
        let xr = if self.shared {
            Pol::var(n, n_t + n_free_base)
        } else {
            Pol::var(n, xr_var)
        };
        let raw: Vec<Pol> = if let Membership::EdwardsBits { shift } = &self.membership {
            // x_i = N_i/D_i with N_i = (1 + s) + (1 − s)·y_i, D_i = 1 − y_i.
            let s = shift.expect("edwards shift resolved in prepare") % p;
            let (c1, c2) = (addm(1, s, p), subm(1, s, p));
            let one = Pol::constant(n, 1, p);
            let fr: Vec<(Pol, Pol)> = (0..m)
                .map(|i| {
                    let y = x(i);
                    (
                        Pol::constant(n, c1, p).add(&y.scale(c2, p), p),
                        one.sub(&y, p),
                    )
                })
                .collect();
            let tri =
                |u: &(Pol, Pol), v: &(Pol, Pol)| s3_in_u_frac((&u.0, &u.1), (&v.0, &v.1), a, b, p);
            let quad = |tr: [Pol; 3], x3: &Pol| {
                tr[0]
                    .mul(x3, p)
                    .mul(x3, p)
                    .add(&tr[1].mul(x3, p), p)
                    .add(&tr[2], p)
            };
            let plain = |v: Pol| (v, one.clone());
            let xr_f = plain(xr.clone());
            match (self.mode, m) {
                (PkmMode::Full, 2) => vec![quad(tri(&fr[0], &fr[1]), &xr)],
                (PkmMode::Full, 3) => vec![resultant_of_triples(
                    &tri(&fr[0], &fr[1]),
                    &tri(&fr[2], &xr_f),
                    p,
                )],
                (PkmMode::Full, _) => unreachable!("checked in read_params"),
                (PkmMode::Chain, _) => {
                    let u = |k: usize| Pol::var(n, n_t + k);
                    let mut eqs = vec![quad(tri(&fr[0], &fr[1]), &u(0))];
                    for k in 1..m - 2 {
                        eqs.push(quad(tri(&plain(u(k - 1)), &fr[k + 1]), &u(k)));
                    }
                    eqs.push(quad(tri(&plain(u(m - 3)), &fr[m - 1]), &xr));
                    eqs
                }
            }
        } else {
            match (self.mode, m) {
                (PkmMode::Full, 2) => vec![s3(&x(0), &x(1), &xr, a, b, p)],
                (PkmMode::Full, 3) => vec![s4(&x(0), &x(1), &x(2), &xr, a, b, p)],
                (PkmMode::Full, _) => unreachable!("checked in read_params"),
                (PkmMode::Chain, _) => {
                    let u = |k: usize| Pol::var(n, n_t + k);
                    let mut eqs = vec![s3(&x(0), &x(1), &u(0), a, b, p)];
                    for k in 1..m - 2 {
                        eqs.push(s3(&u(k - 1), &x(k + 1), &u(k), a, b, p));
                    }
                    eqs.push(s3(&u(m - 3), &x(m - 1), &xr, a, b, p));
                    eqs
                }
            }
        };
        let mut rules = Vec::with_capacity(n_t);
        for i in 0..m {
            for j in 0..t {
                let top = j + 1 == t;
                if let Membership::Isogeny { steps, c, .. } = &self.membership {
                    let (xi, tau) = (steps[j].0 % p, steps[j].1 % p);
                    // Guessed summand: the sub-tower tops out at level t − G
                    // with that group's value; the levels above are unused.
                    let (top, c) = match &iso_group {
                        Some((cg, _)) if i == 0 => (j + 1 == t - self.guess as usize, cg),
                        _ => (top, c),
                    };
                    rules.push(if top {
                        SquareRule {
                            next: None,
                            a: 0,
                            b: 0,
                            c: addm(*c % p, xi, p),
                            d: subm(0, addm(mulm(*c % p, xi, p), tau, p), p),
                        }
                    } else {
                        SquareRule {
                            next: Some((i * t + j + 1) as u8),
                            a: 1,
                            b: subm(0, xi, p),
                            c: xi,
                            d: subm(0, tau, p),
                        }
                    });
                    continue;
                }
                if let Membership::Bits { .. } | Membership::EdwardsBits { .. } = &self.membership {
                    rules.push(SquareRule {
                        next: None,
                        a: 0,
                        b: 0,
                        c: 1,
                        d: 0,
                    });
                    continue;
                }
                rules.push(if top {
                    SquareRule {
                        next: None,
                        a: 0,
                        b: 0,
                        c: 0,
                        d: 1,
                    }
                } else {
                    SquareRule {
                        next: Some((i * t + j + 1) as u8),
                        a: 0,
                        b: 1,
                        c: 0,
                        d: 0,
                    }
                });
            }
        }
        let ring = TowerRing { p, rules, n_free };
        ring.check();
        let degrees = raw
            .iter()
            .map(|f| {
                f.terms
                    .keys()
                    .map(|e| e[..n - 1].iter().sum::<u32>())
                    .max()
                    .unwrap_or(0)
            })
            .collect();
        let eqs = raw
            .iter()
            .map(|f| {
                let kmax = f.terms.keys().map(|e| e[xr_var]).max().unwrap_or(0) as usize;
                (0..=kmax)
                    .map(|k| {
                        let part: Vec<(Vec<u32>, u64)> = f
                            .terms
                            .iter()
                            .filter(|(e, _)| e[xr_var] as usize == k)
                            .map(|(e, c)| (e[..n - 1].to_vec(), *c))
                            .collect();
                        ring.from_raw(&part)
                    })
                    .collect()
            })
            .collect();
        let v = match &self.membership {
            Membership::Kummer => {
                let zeta = primitive_two_power_root(p, self.t).expect("checked in prepare");
                let mut v = Vec::with_capacity(1 << t);
                let mut z = self.g % p;
                for _ in 0..(1u64 << t) {
                    v.push(z);
                    z = mulm(z, zeta, p);
                }
                v
            }
            Membership::Isogeny { v, .. } => match &iso_group {
                Some((_, members)) => members.clone(),
                None => v.clone(),
            },
            Membership::Bits { lo } => (0..(1u64 << t)).map(|k| (lo + k) % p).collect(),
            Membership::EdwardsBits { shift } => {
                edwards_abscissae(p, self.t, shift.expect("resolved in prepare") % p)
            }
        };
        Prepared {
            ring,
            eqs,
            tower_eqs: n_t,
            degrees,
            v,
        }
    }

    /// The system at `x_R`: `Σ_k x_R^k E_k`, one multiplication per
    /// coefficient of every `k ≥ 1` part (and one per power of `x_R`).
    /// The system at `x_R` on guessing branch `b` (or the full system):
    /// `Σ_k x_R^k E_k`, one multiplication per coefficient of every `k ≥ 1`
    /// part (and one per power of `x_R`).
    fn specialise_branch(
        &mut self,
        branch: Option<usize>,
        x_r: u64,
        roots: Option<&[u64]>,
    ) -> Vec<RPoly> {
        let prep = match branch {
            Some(b) => &self.branches[b],
            None => self.prepared.as_ref().expect("prepared"),
        };
        let p = prep.ring.p;
        let fp = Fp::new(p);
        let mut mults = 0u64;
        let mut out = Vec::with_capacity(prep.eqs.len());
        for parts in &prep.eqs {
            let mut terms = Vec::new();
            let mut pw = 1u64;
            for (k, part) in parts.iter().enumerate() {
                if k > 0 {
                    pw = mulm(pw, x_r, p);
                    mults += 1;
                }
                for (&mo, &c) in part.monos.iter().zip(&part.coefs) {
                    let v = if k == 0 {
                        u64::from(c)
                    } else {
                        mults += 1;
                        mulm(u64::from(c), pw, p)
                    };
                    terms.push((mo, v));
                }
            }
            out.push(RPoly::from_terms(fp, terms));
        }
        // Shared system: P_R(X) = Π (X − root) over the lift abscissae,
        // built numerically per target (one multiplication per coefficient
        // step; native, listed apart, unpriced like the specialisation).
        let mut shared_mults = 0u64;
        if let (true, Some(roots)) = (self.shared, roots) {
            let mut c = vec![1u64];
            for &r in roots {
                let mut nx = vec![0u64; c.len() + 1];
                for (k, &ck) in c.iter().enumerate() {
                    nx[k + 1] = addm(nx[k + 1], ck, p);
                    nx[k] = subm(nx[k], mulm(ck, r, p), p);
                    shared_mults += 1;
                }
                c = nx;
            }
            let mut free = [0u8; MAX_FREE];
            let terms: Vec<_> = c
                .iter()
                .enumerate()
                .map(|(k, &ck)| {
                    free[0] = k as u8;
                    (mono(0, free), ck)
                })
                .collect();
            out.push(RPoly::from_terms(fp, terms));
            self.bump("pkm_shared_poly_fp_mults_uncharged", shared_mults);
        }
        self.bump("pkm_system_fp_mults_uncharged", mults);
        let terms = out.iter().map(|q| q.len() as u64).sum::<u64>();
        self.bump("pkm_system_terms", terms);
        self.bump("pkm_system_specialisations", 1);
        out
    }

    /// Every unordered `(x_1, …, x_m) ∈ V^m` with `S_{m+1}(x, x_R) = 0`,
    /// `F_p` multiplications counted.  `first_only` stops at the first.
    fn grid(&self, curve: &PrimeCurve, x_r: u64, first_only: bool) -> (Vec<Vec<u64>>, u64) {
        let p = curve.p;
        let (a, b) = (curve.a % p, curve.b % p);
        let v = &self.prepared.as_ref().expect("prepared").v;
        let mut mults = 0u64;
        let mut out = Vec::new();
        // S₃(A, B, U) coefficients, numeric.
        let s3c = |x1: u64, x2: u64, mults: &mut u64| -> (u64, u64, u64) {
            *mults += 6;
            let d = subm(x1, x2, p);
            let s = addm(x1, x2, p);
            let pr = mulm(x1, x2, p);
            let al = mulm(d, d, p);
            let be = mulm(p - 2, addm(mulm(s, addm(pr, a, p), p), mulm(2, b, p), p), p);
            let q = subm(pr, a, p);
            let ga = subm(mulm(q, q, p), mulm(mulm(4, b, p), s, p), p);
            (al, be, ga)
        };
        match self.m {
            2 => {
                for i in 0..v.len() {
                    let (al, be, ga) = s3c(v[i], x_r, &mut mults);
                    for &x2 in &v[i..] {
                        mults += 2;
                        let val = addm(
                            addm(mulm(mulm(al, x2, p), x2, p), mulm(be, x2, p), p),
                            ga,
                            p,
                        );
                        if val == 0 {
                            out.push(vec![v[i], x2]);
                            if first_only {
                                return (out, mults);
                            }
                        }
                    }
                }
            }
            3 => {
                // S₃(x_3, x_R, U) for every x_3 ∈ V, once per target.
                let right: Vec<(u64, u64, u64)> =
                    v.iter().map(|&x3| s3c(x3, x_r, &mut mults)).collect();
                for i in 0..v.len() {
                    for j in i..v.len() {
                        let (al, be, ga) = s3c(v[i], v[j], &mut mults);
                        for (k, &(al2, be2, ga2)) in right.iter().enumerate().skip(j) {
                            mults += 8;
                            let ag = subm(mulm(al, ga2, p), mulm(al2, ga, p), p);
                            let ab = subm(mulm(al, be2, p), mulm(al2, be, p), p);
                            let bg = subm(mulm(be, ga2, p), mulm(be2, ga, p), p);
                            if subm(mulm(ag, ag, p), mulm(ab, bg, p), p) == 0 {
                                out.push(vec![v[i], v[j], v[k]]);
                                if first_only {
                                    return (out, mults);
                                }
                            }
                        }
                    }
                }
            }
            _ => unreachable!("read_params admits m ≤ 3"),
        }
        (out, mults)
    }
}

impl PkmSemaevOracle {
    /// The shared system was not refuted (consistent, or undecided): read
    /// the solutions of each lift's grid, in the order `lifts=1` tries the
    /// lifts, and return the first relation that lifts to that lift target.
    /// The grid is exact, so the verdict equals the per-lift one.
    fn read_shared(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        undecided: bool,
    ) -> Option<Vec<usize>> {
        let curve = ctx.group;
        let pts = self.shared_pts.clone();
        let mut any = false;
        for (i, t) in pts.iter().enumerate() {
            let (sols, mults) = self.grid(curve, t.x, false);
            self.bump("pkm_grid_fp_mults_uncharged", mults);
            for xs in &sols {
                any = true;
                if xs.iter().all(|x| fb.x_index.contains_key(x)) {
                    if let Some(idx) = lift_abscissae(curve, fb, ops, xs, *t) {
                        if i != 0 {
                            self.bump("pkm_lift_hits_nonidentity", 1);
                        }
                        self.bump("pkm_consistent_systems", 1);
                        return Some(idx);
                    }
                }
                counters.lift_failures += 1;
            }
        }
        if undecided {
            self.bump("pkm_undecided_systems", 1);
        } else if !any {
            self.bump("pkm_consistent_without_grid_solution", 1);
        } else {
            self.bump("pkm_consistent_systems", 1);
        }
        if any {
            counters.unliftable_systems += 1;
        }
        None
    }
}

impl DecompositionOracle<PrimeCurve> for PkmSemaevOracle {
    fn name(&self) -> &str {
        "pkm-semaev"
    }

    fn summands(&self) -> u32 {
        self.m
    }

    fn describe(&self, params: &Params) -> String {
        let tower = match &self.membership {
            Membership::Kummer => {
                format!("the base's Kummer tower (x/g)^(2^t) = 1, g = {}", self.g)
            }
            Membership::Isogeny { .. } => {
                "the base's Vélu 2-isogeny tower (PKM isogeny form)".into()
            }
            Membership::Bits { lo } => format!("bits: x = {lo} + Σ 2^j b_j, b_j² = b_j"),
            Membership::EdwardsBits { .. } => {
                "Edwards bits: x = (1 + y)/(1 − y) + A/3, y = Σ 2^j b_j, b_j² = b_j".into()
            }
        };
        format!(
            "Semaev S_(m+1) over F_p with each x_i on {} (PKM), m = {}, t = {}, mode {}, engine {}",
            tower,
            self.m,
            self.t,
            params
                .get("mode")
                .unwrap_or(if self.m >= 3 { "chain" } else { "full" }),
            params.get("engine").unwrap_or("f4"),
        )
    }

    fn parameters(&self) -> &[(&str, &str)] {
        &[
            ("m", "summands, 2 or 3"),
            (
                "mode",
                "chain (default for m ≥ 3: S₃ links with free u) or full (S_(m+1))",
            ),
            (
                "engine",
                "f4 (f4_fp_tower, default) or exhaustive (the V^m grid reference)",
            ),
            ("degree", "F4 degree bound, default 12"),
            (
                "max_nnz",
                "F4 matrix size cap; a system past it is decided on the grid and counted",
            ),
            (
                "staircase",
                "stop F4 once at most this many standard monomials remain",
            ),
            (
                "crosscheck",
                "1 to grid-check every F4 refutation (diagnostic, not charged)",
            ),
            (
                "bits",
                "on a base other than prime-kummer/prime-isogeny: write membership in [lo, lo + 2^bits) as bits b_j² = b_j",
            ),
            ("lo", "left end of the bit interval, default 0"),
            (
                "edwards_bits",
                "Edwards y in [0, 2^T) written in bits (x_W = (1+y)/(1-y) + A/3)",
            ),
            (
                "edwards_shift",
                "A/3 (x_W of T₀); default: the unique rational root of x³ + ax + b",
            ),
            (
                "guess",
                "bits or isogeny form: enumerate the top G bits (tower levels) of the first summand, one F4 system per assignment (hybrid guessing)",
            ),
            (
                "trace",
                "1: F4 trace; learn one refutation trace per branch system at prepare (charged once, as setup native work), replay it per target with x_R specialised (only a refutation is accepted), and fall back to plain F4 when the replay does not refute",
            ),
            (
                "lifts",
                "1: cofactor-fiber lifts; try R + K for each x-distinct K in E(F_q)[h], first liftable relation wins; relations are over the projection [h]P (see fiber.rs). 2: the same verdicts from ONE F4 system per target: the third abscissa is a free variable X with P_R(X) = prod (X - x(R+K)) (m = 2, engine f4)",
            ),
        ]
    }

    fn prepare(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        params: &Params,
        ops: &mut GroupOps,
    ) -> Result<(), String> {
        self.read_params(params)?;
        self.fiber = None;
        if self.lifts {
            // E(F_q)[h] once, from [r]P over the base points; its group
            // operations are charged to the set-up ledger.
            self.fiber = Some(cofactor_fiber(ctx.group, fb, ctx.r, ctx.cofactor, ops));
        }
        self.shared = self.shared_req && self.fiber.as_ref().is_some_and(|f| f.size() > 1);
        self.shared_pts.clear();
        let curve = ctx.group;
        let p = curve.p;
        if p >= 1 << 31 {
            return Err("pkm-semaev: the tower engine needs p < 2^31".into());
        }
        let mut check_mults = 0u64;
        if let Membership::EdwardsBits { shift } = self.membership {
            let s = match (params.get("edwards_shift"), shift) {
                (Some(_), _) => params.u64("edwards_shift")? % p,
                (None, Some(s)) => s,
                (None, None) => edwards_shift(curve)?,
            };
            self.membership = Membership::EdwardsBits { shift: Some(s) };
            let set: HashSet<u64> = edwards_abscissae(p, self.t, s).into_iter().collect();
            if let Some(x) = fb.abscissa_list.iter().find(|x| !set.contains(x)) {
                return Err(format!(
                    "pkm-semaev: abscissa {x} of the base has no Edwards y in [0, 2^{})",
                    self.t
                ));
            }
            return self.finish_prepare(curve, 0);
        }
        match &self.membership {
            Membership::Kummer => {}
            Membership::EdwardsBits { .. } => unreachable!("handled above"),
            Membership::Isogeny { v, steps, .. } => {
                if steps.len() != self.t as usize || v.len() != 1usize << self.t {
                    return Err(
                        "pkm-semaev: the isogeny tower does not have t steps and 2^t zeros".into(),
                    );
                }
                let set: HashSet<u64> = v.iter().copied().collect();
                if let Some(x) = fb.abscissa_list.iter().find(|x| !set.contains(x)) {
                    return Err(format!(
                        "pkm-semaev: abscissa {x} of the base is not a zero of the isogeny tower"
                    ));
                }
                return self.finish_prepare(curve, 0);
            }
            Membership::Bits { lo } => {
                let span = 1u64 << self.t;
                if let Some(x) = fb
                    .abscissa_list
                    .iter()
                    .find(|&&x| (x + p - lo % p) % p >= span)
                {
                    return Err(format!(
                        "pkm-semaev: abscissa {x} of the base is outside [{lo}, {lo} + 2^{})",
                        self.t
                    ));
                }
                return self.finish_prepare(curve, 0);
            }
        }
        primitive_two_power_root(p, self.t)
            .ok_or_else(|| format!("pkm-semaev: 2^{} does not divide p − 1", self.t))?;
        // The base must be this tower: every abscissa x has (x/g)^(2^t) = 1.
        let g = self.g % p;
        if g == 0 {
            return Err("pkm-semaev: g must be non-zero".into());
        }
        let ginv = crate::cryptanalysis::residual_walk::pow_mod(g, p - 2, p);
        for &x in &fb.abscissa_list {
            let mut y = mulm(x, ginv, p);
            for _ in 0..self.t {
                y = mulm(y, y, p);
            }
            check_mults += 1 + u64::from(self.t);
            if y != 1 {
                return Err(format!(
                    "pkm-semaev: abscissa {x} of the base is not in g·μ_(2^{}); the oracle runs on \
                     prime-kummer:t={},g={} only",
                    self.t, self.t, self.g
                ));
            }
        }
        self.finish_prepare(curve, check_mults)
    }

    fn setup_native(&self) -> BTreeMap<String, u64> {
        let mut m = BTreeMap::new();
        if self.setup_mults > 0 {
            m.insert("pkm_base_check_fp_mults_uncharged".into(), self.setup_mults);
        }
        m.extend(self.trace_setup.clone());
        if let Some(f) = &self.fiber {
            m.insert("pkm_fiber_size".into(), f.size() as u64);
            m.insert("pkm_fiber_cofactor".into(), f.h);
            m.insert("pkm_fiber_complete".into(), u64::from(f.complete));
            m.insert("pkm_fiber_samples".into(), f.samples);
        }
        m
    }

    fn decompose(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: PrimePoint,
    ) -> Option<Vec<usize>> {
        if !self.in_lift {
            self.shared_pts.clear();
            if let Some(ks) = self
                .fiber
                .as_ref()
                .filter(|f| f.size() > 1)
                .map(|f| f.points.clone())
            {
                if point.infinity {
                    return None;
                }
                if self.shared {
                    // One system for all the lifts: same lift list and
                    // charged additions as lifts=1, one F4 call per branch.
                    let mut tried_x: Vec<u64> = Vec::with_capacity(ks.len());
                    let mut pts = Vec::with_capacity(ks.len());
                    for (i, &k) in ks.iter().enumerate() {
                        let t = if i == 0 {
                            point
                        } else {
                            ctx.group.add(ops, point, k)
                        };
                        if t.infinity || tried_x.contains(&t.x) {
                            continue;
                        }
                        tried_x.push(t.x);
                        pts.push(t);
                    }
                    self.bump("pkm_shared_lifts", pts.len() as u64);
                    self.bump("pkm_shared_systems", 1);
                    self.shared_pts = pts;
                } else {
                    // The cofactor fiber of the target: every row is written over
                    // the projection [h]P and [h]K = O, so a decomposition of
                    // R + K is as good a relation as one of R (fiber.rs, `lifts`).
                    // A lift whose abscissa was already tried is skipped
                    // (x(R+K) = x(-R-K)); each addition is charged.
                    let mut tried_x: Vec<u64> = Vec::with_capacity(ks.len());
                    self.in_lift = true;
                    let mut found = None;
                    for (i, &k) in ks.iter().enumerate() {
                        let t = if i == 0 {
                            point
                        } else {
                            ctx.group.add(ops, point, k)
                        };
                        if t.infinity || tried_x.contains(&t.x) {
                            continue;
                        }
                        tried_x.push(t.x);
                        self.bump("pkm_lift_targets_tried", 1);
                        if let Some(idx) = self.decompose(ctx, fb, ops, counters, t) {
                            if i != 0 {
                                self.bump("pkm_lift_hits_nonidentity", 1);
                            }
                            found = Some(idx);
                            break;
                        }
                    }
                    self.in_lift = false;
                    return found;
                }
            }
        }
        if self.dataset_trace {
            self.dataset_artifact = None;
        }
        if point.infinity || self.prepared.is_none() {
            return None;
        }
        let curve = ctx.group;
        let x_r = point.x;
        if self.dataset_trace {
            self.dataset_artifact = self.prepared_system_artifact();
            if let Some(snapshot) = &mut self.dataset_artifact {
                snapshot["target_x"] = serde_json::json!(x_r.to_string());
                snapshot["grid_extraction_attempted"] = serde_json::json!(false);
            }
        }
        let solutions = match self.engine {
            PkmEngine::Exhaustive => {
                let started = Instant::now();
                let (sols, mults) = self.grid(curve, x_r, false);
                if let Some(snapshot) = &mut self.dataset_artifact {
                    snapshot["grid_extraction_attempted"] = serde_json::json!(true);
                    snapshot["grid_root_x"] = serde_json::json!(sols
                        .iter()
                        .map(|xs| xs.iter().map(ToString::to_string).collect::<Vec<_>>())
                        .collect::<Vec<_>>());
                    snapshot["grid_fp_multiplies"] = serde_json::json!(mults);
                }
                self.bump("pkm_grid_fp_mults_uncharged", mults);
                let shape = self.shape.clone().unwrap_or_default();
                let cost = SolverCost {
                    ops: mults,
                    op_unit: "F_p multiplications (grid)".into(),
                    wall_ns: started.elapsed().as_nanos() as u64,
                    ..Default::default()
                };
                self.totals.op_unit = "F_p multiplications (grid)".into();
                // The grid's multiplications are already on the native
                // counter; the solver block carries calls and shape only.
                let cost = SolverCost { ops: 0, ..cost };
                self.totals.absorb(&shape, Some(&cost), false);
                if sols.is_empty() {
                    return None;
                }
                self.bump("pkm_consistent_systems", 1);
                sols
            }
            PkmEngine::F4 => {
                // Hybrid guessing: one F4 system per assignment of the guessed
                // bits; the first consistent branch goes to the grid, and the
                // target is refuted only when every branch is.
                let n_branches = self.branches.len().max(1);
                let mut last = None;
                for br in 0..n_branches {
                    let branch = (!self.branches.is_empty()).then_some(br);
                    let shared_roots: Vec<u64> = self.shared_pts.iter().map(|t| t.x).collect();
                    let roots = self.shared.then_some(&shared_roots[..]);
                    let sys = self.specialise_branch(branch, x_r, roots);
                    if let Some(snapshot) = &mut self.dataset_artifact {
                        snapshot["specialized_equations"] = serde_json::json!(sys
                            .iter()
                            .map(Self::polynomial_artifact)
                            .collect::<Vec<_>>());
                    }
                    let ring = match branch {
                        Some(b) => self.branches[b].ring.clone(),
                        None => self.prepared.as_ref().expect("prepared").ring.clone(),
                    };
                    let opts = self.f4_options();
                    let mut extra_ops = 0u64;
                    let mut extra_ms = 0.0f64;
                    let rep = if self.trace {
                        let tr = self.traces.get(br).cloned().flatten();
                        match tr {
                            None => {
                                self.bump("pkm_trace_untraced_branch_runs", 1);
                                f4_tower(&sys, &ring, &opts)
                            }
                            Some(tr) => {
                                let mut ropts = opts.clone();
                                ropts.replay = Some(tr);
                                let r = f4_tower(&sys, &ring, &ropts);
                                self.bump("pkm_trace_replays", 1);
                                self.bump("pkm_trace_replay_muladds", r.muladds);
                                if r.inconsistent {
                                    self.bump("pkm_trace_refuted", 1);
                                    r
                                } else {
                                    let why = r
                                        .replay_failure
                                        .unwrap_or("no refutation")
                                        .replace(' ', "_");
                                    self.bump(&format!("pkm_trace_failure_{why}"), 1);
                                    if let Some((s, g, w)) = r.replay_mismatch {
                                        self.bump(
                                            &format!("pkm_trace_mismatch_step{s}_got{g}_want{w}"),
                                            1,
                                        );
                                    }
                                    let plain = f4_tower(&sys, &ring, &opts);
                                    self.bump("pkm_trace_fallbacks", 1);
                                    self.bump("pkm_trace_fallback_plain_muladds", plain.muladds);
                                    extra_ops = r.muladds;
                                    extra_ms = r.ms;
                                    plain
                                }
                            }
                        }
                    } else {
                        f4_tower(&sys, &ring, &opts)
                    };
                    if let Some(snapshot) = &mut self.dataset_artifact {
                        snapshot["f4_basis"] = serde_json::json!(rep
                            .basis
                            .iter()
                            .map(Self::polynomial_artifact)
                            .collect::<Vec<_>>());
                        snapshot["f4_report"] = serde_json::json!({"inconsistent":rep.inconsistent,
                            "timed_out":rep.timed_out,"oversize":rep.oversize,"degree_reached":rep.degree_reached,
                            "observed_productive_degree_max":rep.solving_degree_max,"pairs_above_bound":rep.pairs_above_bound,
                            "elimination_fp_muladds":rep.muladds,"max_rows":rep.max_rows,"max_cols":rep.max_cols,
                            "max_nnz":rep.max_nnz,"critical_pairs_reduced":rep.critical_pairs_reduced,
                            "tower_pairs_reduced":rep.tower_pairs_reduced,"pairs_product_skipped":rep.pairs_product_skipped,
                            "pairs_chain_skipped":rep.pairs_chain_skipped,"reducer_rows":rep.reducer_rows,
                            "full_exposed_native_report_debug":format!("{rep:#?}")});
                    }
                    let undecided = !rep.inconsistent
                        && rep.staircase_at_stop.is_none()
                        && (rep.timed_out || rep.oversize || rep.pairs_above_bound > 0);
                    let mut extra = BTreeMap::new();
                    extra.insert("max_cols_max".to_string(), rep.max_cols as u64);
                    extra.insert("max_rows_max".to_string(), rep.max_rows as u64);
                    extra.insert("max_nnz_max".to_string(), rep.max_nnz as u64);
                    extra.insert("steps".to_string(), rep.steps as u64);
                    extra.insert("refutations".to_string(), u64::from(rep.inconsistent));
                    extra.insert(
                        "staircase_stops".to_string(),
                        u64::from(rep.staircase_at_stop.is_some()),
                    );
                    let cost = SolverCost {
                        ops: rep.muladds + extra_ops,
                        op_unit: "F_p multiply-adds (elimination only)".into(),
                        wall_ns: ((rep.ms + extra_ms) * 1e6) as u64,
                        peak_bytes: 0,
                        degree_reached: Some(rep.degree_reached),
                        solving_degree: (!undecided).then_some(rep.solving_degree_max),
                        timed_out: rep.timed_out,
                        extra,
                    };
                    let shape = self.shape.clone().unwrap_or_default();
                    self.totals.absorb(&shape, Some(&cost), undecided);
                    if branch.is_some() {
                        self.bump("pkm_guess_branches_solved", 1);
                    }
                    let stop = !rep.inconsistent;
                    last = Some((rep, undecided));
                    if stop {
                        break;
                    }
                }
                let (rep, undecided) = last.expect("at least one branch");
                if rep.inconsistent {
                    if self.crosscheck {
                        let xs: Vec<u64> = if self.shared_pts.is_empty() {
                            vec![x_r]
                        } else {
                            self.shared_pts.iter().map(|t| t.x).collect()
                        };
                        for x in xs {
                            let (sols, mults) = self.grid(curve, x, true);
                            self.bump("pkm_crosscheck_fp_mults", mults);
                            if !sols.is_empty() {
                                self.bump("pkm_crosscheck_disagreements", 1);
                            }
                        }
                    }
                    return None;
                }
                if !self.shared_pts.is_empty() {
                    return self.read_shared(ctx, fb, ops, counters, undecided);
                }
                // Consistent, or undecided: the grid reads the solutions.
                let (sols, mults) = self.grid(curve, x_r, false);
                if let Some(snapshot) = &mut self.dataset_artifact {
                    snapshot["grid_extraction_attempted"] = serde_json::json!(true);
                    snapshot["grid_root_x"] = serde_json::json!(sols
                        .iter()
                        .map(|xs| xs.iter().map(ToString::to_string).collect::<Vec<_>>())
                        .collect::<Vec<_>>());
                    snapshot["grid_fp_multiplies"] = serde_json::json!(mults);
                }
                self.bump("pkm_grid_fp_mults_uncharged", mults);
                if undecided {
                    self.bump("pkm_undecided_systems", 1);
                } else if sols.is_empty() {
                    // F4 stopped short of a certificate (staircase stop) on
                    // a system with no grid solution.
                    self.bump("pkm_consistent_without_grid_solution", 1);
                } else {
                    self.bump("pkm_consistent_systems", 1);
                }
                if sols.is_empty() {
                    return None;
                }
                sols
            }
        };
        if let Some(snapshot) = &mut self.dataset_artifact {
            snapshot["grid_root_x"] = serde_json::json!(solutions
                .iter()
                .map(|xs| xs.iter().map(ToString::to_string).collect::<Vec<_>>())
                .collect::<Vec<_>>());
        }
        // Lift: every abscissa on the curve, then the signs that sum to R,
        // each try re-added on the counted ledger.
        for xs in &solutions {
            if xs.iter().all(|x| fb.x_index.contains_key(x)) {
                if let Some(idx) = lift_abscissae(curve, fb, ops, xs, point) {
                    return Some(idx);
                }
            }
            counters.lift_failures += 1;
        }
        counters.unliftable_systems += 1;
        None
    }

    fn relation_counters(&self) -> BTreeMap<String, u64> {
        self.native.clone()
    }

    fn last_system(&self) -> Option<SystemShape> {
        self.shape.clone()
    }

    fn solver_totals(&self) -> Option<SolverTotals> {
        Some(self.totals.clone())
    }
}

/// Every point of the curve that is a sum of `m` base points (with
/// repetition, either sign), by enumeration: the exact count behind the
/// hit rate `N_m / r` the experiment checks.  Group operations are
/// charged to `ops`.
pub fn decomposable_points(
    curve: &PrimeCurve,
    fb: &FactorBase<PrimePoint>,
    m: u32,
    ops: &mut GroupOps,
) -> HashSet<u64> {
    let mut level: HashSet<u64> = HashSet::new();
    let mut pts: Vec<PrimePoint> = vec![curve.identity()];
    for _ in 0..m {
        let mut next = Vec::new();
        let mut seen = HashSet::new();
        for &acc in &pts {
            for &q in &fb.points {
                let s = curve.add(ops, acc, q);
                if seen.insert(curve.key(&s)) {
                    next.push(s);
                }
            }
        }
        pts = next;
        level = seen;
    }
    level
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::PrimeInstance;
    use crate::cryptanalysis::ic_framework::prime_tower::PrimeKummerBase;
    use crate::cryptanalysis::ic_framework::stages::FactorBaseBuilder;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    /// A 24-bit prime-order curve `y² = x³ − 3x + b` with `2^7 | p − 1`
    /// (curve e3-c2 of experiment E3, from
    /// `a_minus_3_prime_order_curves 24 4 20261019 6`).  Its `t = 3` tower
    /// carries 5 abscissae, so F4 runs on 10 variables in tens of
    /// milliseconds.
    fn instance() -> PrimeInstance {
        let p = 13_040_257u64;
        PrimeInstance {
            name: "e3-c2".into(),
            curve: PrimeCurve {
                p,
                a: p - 3,
                b: 1_901_252,
            },
            group_order: 13_043_059,
            r: 13_043_059,
            cofactor: 1,
            generator: (0, 6_024_490),
        }
    }

    fn ctx(inst: &PrimeInstance) -> InstanceCtx<'_, PrimeCurve> {
        InstanceCtx {
            group: &inst.curve,
            generator: inst.generator_point(),
            target: inst.generator_point(),
            r: inst.r,
            cofactor: 1,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: None,
        }
    }

    fn base(inst: &PrimeInstance, t: u32) -> FactorBase<PrimePoint> {
        let b = PrimeKummerBase { instance: inst };
        let mut params = Params::default();
        params.set("t", t.to_string());
        b.build(&ctx(inst), &params, &mut GroupOps::default())
            .expect("kummer base")
    }

    fn oracle(
        inst: &PrimeInstance,
        fb: &FactorBase<PrimePoint>,
        m: u32,
        t: u32,
        spec: &str,
    ) -> PkmSemaevOracle {
        let (_, params) = Params::parse_spec(&format!("pkm-semaev:{spec}")).unwrap();
        let mut o = PkmSemaevOracle::new(m, t, 1);
        o.prepare(&ctx(inst), fb, &params, &mut GroupOps::default())
            .expect("prepare");
        o
    }

    fn sum(curve: &PrimeCurve, fb: &FactorBase<PrimePoint>, idx: &[usize]) -> PrimePoint {
        let mut ops = GroupOps::default();
        idx.iter().fold(curve.identity(), |acc, &i| {
            curve.add(&mut ops, acc, fb.points[i])
        })
    }

    /// Planted sums of three base points decompose, under F4 in both
    /// presentations and under the grid, and every returned relation sums
    /// to the target.
    #[test]
    fn planted_m3_decompositions_are_found() {
        let inst = instance();
        let fb = base(&inst, 3);
        assert!(fb.columns >= 3, "the t = 3 tower carries points");
        let mut rng = StdRng::seed_from_u64(7);
        for spec in ["m=3", "m=3,mode=full", "m=3,engine=exhaustive"] {
            let mut o = oracle(&inst, &fb, 3, 3, spec);
            for _ in 0..6 {
                let idx: Vec<usize> = (0..3).map(|_| rng.gen_range(0..fb.points.len())).collect();
                let target = sum(&inst.curve, &fb, &idx);
                if target.infinity {
                    continue;
                }
                let mut ops = GroupOps::default();
                let mut ctr = OracleCounters::default();
                let got = o
                    .decompose(&ctx(&inst), &fb, &mut ops, &mut ctr, target)
                    .unwrap_or_else(|| panic!("{spec}: planted target not decomposed"));
                assert_eq!(got.len(), 3);
                assert_eq!(sum(&inst.curve, &fb, &got), target, "{spec}");
                assert!(ops.adds > 0, "{spec}: the lift re-adds on the ledger");
            }
            if spec != "m=3,engine=exhaustive" {
                let t = o.solver_totals().unwrap();
                assert!(t.calls > 0 && t.ops > 0, "{spec}: F4 work is counted");
            }
        }
    }

    /// On random targets F4's verdict equals the grid's on every one, in
    /// both presentations, and decompositions agree with the exact set of
    /// decomposable points.
    #[test]
    fn f4_agrees_with_grid_on_random_targets() {
        let inst = instance();
        let curve = &inst.curve;
        let fb = base(&inst, 3);
        let decomposable = decomposable_points(curve, &fb, 3, &mut GroupOps::default());
        let mut rng = StdRng::seed_from_u64(11);
        let mut f4 = oracle(&inst, &fb, 3, 3, "m=3,crosscheck=1");
        let mut full = oracle(&inst, &fb, 3, 3, "m=3,mode=full");
        let mut grid = oracle(&inst, &fb, 3, 3, "m=3,engine=exhaustive");
        let g = inst.generator_point();
        let mut hits = 0;
        // Random targets, plus planted ones so both verdicts occur.
        for k in 0..40u64 {
            let target = if k.is_multiple_of(4) {
                let idx: Vec<usize> = (0..3).map(|_| rng.gen_range(0..fb.points.len())).collect();
                sum(curve, &fb, &idx)
            } else {
                let s = rng.gen_range(1..inst.r);
                curve.mul(&mut GroupOps::default(), g, s)
            };
            if target.infinity {
                continue;
            }
            let run = |o: &mut PkmSemaevOracle| {
                o.decompose(
                    &ctx(&inst),
                    &fb,
                    &mut GroupOps::default(),
                    &mut OracleCounters::default(),
                    target,
                )
                .is_some()
            };
            let (a, b, c) = (run(&mut f4), run(&mut full), run(&mut grid));
            assert_eq!(a, c, "chain F4 against the grid, target {k}");
            assert_eq!(b, c, "full F4 against the grid, target {k}");
            assert_eq!(c, decomposable.contains(&curve.key(&target)), "target {k}");
            hits += usize::from(c);
        }
        assert!(hits >= 10, "the planted targets decompose");
        let n = f4.relation_counters();
        assert_eq!(
            n.get("pkm_crosscheck_disagreements").copied().unwrap_or(0),
            0
        );
        assert!(n["pkm_system_fp_mults_uncharged"] > 0);
    }

    /// F4 on a membership form against the grid and the exact set of
    /// decomposable points, on random and planted targets, `m = 2`.
    fn agrees_with_grid(
        inst: &PrimeInstance,
        fb: &FactorBase<PrimePoint>,
        make: &dyn Fn() -> PkmSemaevOracle,
        label: &str,
        extra: &str,
    ) {
        let curve = &inst.curve;
        let decomposable = decomposable_points(curve, fb, 2, &mut GroupOps::default());
        let prep = |spec: &str| {
            let (_, params) = Params::parse_spec(&format!("pkm-semaev:{spec}")).unwrap();
            let mut o = make();
            o.prepare(&ctx(inst), fb, &params, &mut GroupOps::default())
                .unwrap_or_else(|e| panic!("{label}: prepare: {e}"));
            o
        };
        let mut f4 = prep(&format!("m=2,crosscheck=1{extra}"));
        let mut grid = prep("m=2,engine=exhaustive");
        let mut rng = StdRng::seed_from_u64(29);
        let g = inst.generator_point();
        let mut hits = 0;
        for k in 0..32u64 {
            let target = if k.is_multiple_of(3) {
                let idx: Vec<usize> = (0..2).map(|_| rng.gen_range(0..fb.points.len())).collect();
                sum(curve, fb, &idx)
            } else {
                curve.mul(&mut GroupOps::default(), g, rng.gen_range(1..inst.r))
            };
            if target.infinity {
                continue;
            }
            let run = |o: &mut PkmSemaevOracle| {
                let got = o.decompose(
                    &ctx(inst),
                    fb,
                    &mut GroupOps::default(),
                    &mut OracleCounters::default(),
                    target,
                );
                if let Some(idx) = &got {
                    assert_eq!(
                        sum(curve, fb, idx),
                        target,
                        "{label}: relation sums to the target"
                    );
                }
                got.is_some()
            };
            let (a, c) = (run(&mut f4), run(&mut grid));
            assert_eq!(a, c, "{label}: F4 against the grid, target {k}");
            assert_eq!(
                c,
                decomposable.contains(&curve.key(&target)),
                "{label}: target {k}"
            );
            hits += usize::from(c);
        }
        assert!(hits >= 5, "{label}: planted targets decompose ({hits})");
        let n = f4.relation_counters();
        assert_eq!(
            n.get("pkm_crosscheck_disagreements").copied().unwrap_or(0),
            0,
            "{label}"
        );
        assert!(
            f4.solver_totals().unwrap().ops > 0,
            "{label}: F4 work is counted"
        );
    }

    /// PKM's isogeny form (no condition on p): the oracle's tower rules are
    /// the base's Vélu chain, and F4 matches the grid.
    #[test]
    fn isogeny_tower_form_agrees_with_grid() {
        use crate::cryptanalysis::ic_framework::prime_tower::{isogeny_tower, PrimeIsogenyBase};
        let inst = instance();
        let t = 4;
        let b = PrimeIsogenyBase { instance: &inst };
        let mut params = Params::default();
        params.set("t", t.to_string());
        let fb = b
            .build(&ctx(&inst), &params, &mut GroupOps::default())
            .expect("isogeny base");
        let tower = isogeny_tower(inst.curve.p, t, 0, &mut GroupOps::default()).unwrap();
        // Every zero of the rules is a zero of the chain: L(x) = c on V.
        for &x in &tower.v {
            assert_eq!(tower.eval(x), Some(tower.c));
        }
        let steps: Vec<(u64, u64)> = tower.steps.iter().map(|s| (s.xi, s.tau)).collect();
        let (v, c) = (tower.v.clone(), tower.c);
        agrees_with_grid(
            &inst,
            &fb,
            &|| PkmSemaevOracle::new_isogeny(2, t, v.clone(), steps.clone(), c),
            "isogeny",
            "",
        );
    }

    /// F4 trace (`trace=1`): the verdicts equal plain F4's and the grid's
    /// (and the exact decomposable set) on random and planted targets, in
    /// the isogeny form at `G in {0, 1, 2}` and the bit form at `G = 1`; the
    /// fallback is exercised on the planted (consistent) targets and is
    /// charged; the ledger is exactly replay plus fallback work.
    #[test]
    fn trace_agrees_with_plain_f4_and_grid() {
        use crate::cryptanalysis::ic_boundary::prime_factor_base;
        use crate::cryptanalysis::ic_framework::prime_tower::{isogeny_tower, PrimeIsogenyBase};
        let inst = instance();
        let curve = &inst.curve;
        let t = 4;
        let b = PrimeIsogenyBase { instance: &inst };
        let mut params = Params::default();
        params.set("t", t.to_string());
        let fb = b
            .build(&ctx(&inst), &params, &mut GroupOps::default())
            .expect("isogeny base");
        let tower = isogeny_tower(curve.p, t, 0, &mut GroupOps::default()).unwrap();
        let steps: Vec<(u64, u64)> = tower.steps.iter().map(|s| (s.xi, s.tau)).collect();
        let (v, c) = (tower.v.clone(), tower.c);
        let bits_fb = prime_factor_base(&inst, 6);
        let bits_t = 64 - bits_fb.abscissa_list.iter().max().unwrap().leading_zeros();
        let decomposable = |fb: &FactorBase<PrimePoint>| {
            decomposable_points(curve, fb, 2, &mut GroupOps::default())
        };
        for (label, g, use_bits) in [
            ("isogeny G=0", 0u32, false),
            ("isogeny G=1", 1, false),
            ("isogeny G=2", 2, false),
            ("bits G=1", 1, true),
        ] {
            let fb = if use_bits { &bits_fb } else { &fb };
            let exact = decomposable(fb);
            let make = || {
                if use_bits {
                    PkmSemaevOracle::new_bits(2, bits_t, 0)
                } else {
                    PkmSemaevOracle::new_isogeny(2, t, v.clone(), steps.clone(), c)
                }
            };
            let prep = |spec: &str| {
                let (_, params) = Params::parse_spec(&format!("pkm-semaev:{spec}")).unwrap();
                let mut o = make();
                o.prepare(&ctx(&inst), fb, &params, &mut GroupOps::default())
                    .unwrap_or_else(|e| panic!("{label}: prepare: {e}"));
                o
            };
            let gs = if g > 0 {
                format!(",guess={g}")
            } else {
                String::new()
            };
            let mut traced = prep(&format!("m=2,crosscheck=1,trace=1{gs}"));
            let mut plain = prep(&format!("m=2{gs}"));
            let mut grid = prep("m=2,engine=exhaustive");
            let setup = traced.setup_native();
            assert_eq!(setup["pkm_trace_branches"], 1 << g, "{label}");
            assert!(
                setup["pkm_trace_learn_muladds"] > 0,
                "{label}: learning is charged"
            );
            assert_eq!(
                setup["pkm_trace_branches_learned"],
                1 << g,
                "{label}: all learned"
            );
            let mut rng = StdRng::seed_from_u64(77);
            let gen = inst.generator_point();
            let mut hits = 0;
            for k in 0..32u64 {
                let target = if k.is_multiple_of(3) {
                    let idx: Vec<usize> =
                        (0..2).map(|_| rng.gen_range(0..fb.points.len())).collect();
                    sum(curve, fb, &idx)
                } else {
                    curve.mul(&mut GroupOps::default(), gen, rng.gen_range(1..inst.r))
                };
                if target.infinity {
                    continue;
                }
                let run = |o: &mut PkmSemaevOracle| {
                    let got = o.decompose(
                        &ctx(&inst),
                        fb,
                        &mut GroupOps::default(),
                        &mut OracleCounters::default(),
                        target,
                    );
                    if let Some(idx) = &got {
                        assert_eq!(sum(curve, fb, idx), target, "{label}: relation sums to R");
                    }
                    got.is_some()
                };
                let (a, b, c2) = (run(&mut traced), run(&mut plain), run(&mut grid));
                assert_eq!(a, b, "{label}: trace against plain F4, target {k}");
                assert_eq!(a, c2, "{label}: trace against the grid, target {k}");
                assert_eq!(
                    c2,
                    exact.contains(&curve.key(&target)),
                    "{label}: target {k}"
                );
                hits += usize::from(c2);
            }
            assert!(hits >= 5, "{label}: planted targets decompose ({hits})");
            let n = traced.relation_counters();
            let get = |k: &str| n.get(k).copied().unwrap_or(0);
            assert_eq!(get("pkm_crosscheck_disagreements"), 0, "{label}");
            assert!(
                get("pkm_trace_refuted") > 0,
                "{label}: replays refute {n:?} {setup:?}"
            );
            assert!(
                get("pkm_trace_fallbacks") > 0,
                "{label}: fallback exercised"
            );
            assert!(
                get("pkm_trace_fallback_plain_muladds") > 0,
                "{label}: fallback charged"
            );
            assert_eq!(get("pkm_trace_untraced_branch_runs"), 0, "{label}");
            assert_eq!(
                get("pkm_trace_replays"),
                get("pkm_trace_refuted") + get("pkm_trace_fallbacks"),
                "{label}"
            );
            // The solver ledger is replay work plus fallback work: nothing
            // else is charged per target, and the learning is not.
            assert_eq!(
                traced.solver_totals().unwrap().ops,
                get("pkm_trace_replay_muladds") + get("pkm_trace_fallback_plain_muladds"),
                "{label}: ledger"
            );
        }
    }

    /// Isogeny-form hybrid guessing: same verdicts as the grid and the exact
    /// decomposable set with 1 and 2 guessed levels; `G ≥ t` is refused.
    #[test]
    fn isogeny_guessing_agrees_with_grid() {
        use crate::cryptanalysis::ic_framework::prime_tower::{isogeny_tower, PrimeIsogenyBase};
        let inst = instance();
        let t = 4;
        let b = PrimeIsogenyBase { instance: &inst };
        let mut params = Params::default();
        params.set("t", t.to_string());
        let fb = b
            .build(&ctx(&inst), &params, &mut GroupOps::default())
            .expect("isogeny base");
        let tower = isogeny_tower(inst.curve.p, t, 0, &mut GroupOps::default()).unwrap();
        let steps: Vec<(u64, u64)> = tower.steps.iter().map(|s| (s.xi, s.tau)).collect();
        let (v, c) = (tower.v.clone(), tower.c);
        for g in [1, 2] {
            agrees_with_grid(
                &inst,
                &fb,
                &|| PkmSemaevOracle::new_isogeny(2, t, v.clone(), steps.clone(), c),
                &format!("isogeny guess={g}"),
                &format!(",guess={g}"),
            );
        }
        // The groups partition V into 2^G sets of 2^(t-G).
        let mut o = PkmSemaevOracle::new_isogeny(2, t, v.clone(), steps.clone(), c);
        o.guess = 2;
        let groups = o.isogeny_groups(inst.curve.p);
        assert_eq!(groups.len(), 4);
        assert!(groups.iter().all(|(_, m)| m.len() == 4));
        let (_, params) = Params::parse_spec(&format!("pkm-semaev:m=2,guess={t}")).unwrap();
        assert!(
            PkmSemaevOracle::new_isogeny(2, t, v, steps, c)
                .prepare(&ctx(&inst), &fb, &params, &mut GroupOps::default())
                .is_err(),
            "guess must be below the tower height"
        );
    }

    /// The bit form: an interval base `[0, 2^t)` written as `Σ 2^j b_j`,
    /// `b_j² = b_j`; F4 matches the grid, and a base outside the interval is
    /// refused.
    #[test]
    fn bit_interval_form_agrees_with_grid() {
        use crate::cryptanalysis::ic_boundary::prime_factor_base;
        let inst = instance();
        let fb = prime_factor_base(&inst, 6);
        let max = *fb.abscissa_list.iter().max().unwrap();
        let t = 64 - max.leading_zeros();
        agrees_with_grid(
            &inst,
            &fb,
            &|| PkmSemaevOracle::new_bits(2, t, 0),
            "bits",
            "",
        );
        // Hybrid guessing: the same verdicts with 1 and 2 guessed bits, and
        // every branch's F4 work charged.
        for g in [1, 2] {
            agrees_with_grid(
                &inst,
                &fb,
                &|| PkmSemaevOracle::new_bits(2, t, 0),
                &format!("bits guess={g}"),
                &format!(",guess={g}"),
            );
        }
        let (_, params) = Params::parse_spec(&format!("pkm-semaev:m=2,guess={t}")).unwrap();
        assert!(
            PkmSemaevOracle::new_bits(2, t, 0)
                .prepare(&ctx(&inst), &fb, &params, &mut GroupOps::default())
                .is_err(),
            "guess must be below the bit count"
        );
        let (_, params) = Params::parse_spec("pkm-semaev:m=2").unwrap();
        let mut narrow = PkmSemaevOracle::new_bits(2, t - 1, 0);
        assert!(narrow
            .prepare(&ctx(&inst), &fb, &params, &mut GroupOps::default())
            .is_err());
    }

    /// `m = 3` on a membership form: the chained and the full presentation
    /// and the grid all give the exact decomposability verdict of
    /// `decomposable_points(.., 3, ..)`, and every relation sums to the target.
    fn agrees_with_grid_m3(
        inst: &PrimeInstance,
        fb: &FactorBase<PrimePoint>,
        make: &dyn Fn() -> PkmSemaevOracle,
        label: &str,
        extra: &str,
    ) {
        let curve = &inst.curve;
        let decomposable = decomposable_points(curve, fb, 3, &mut GroupOps::default());
        let prep = |spec: &str| {
            let (_, params) = Params::parse_spec(&format!("pkm-semaev:{spec}")).unwrap();
            let mut o = make();
            o.prepare(&ctx(inst), fb, &params, &mut GroupOps::default())
                .unwrap_or_else(|e| panic!("{label}: prepare: {e}"));
            o
        };
        let mut chain = prep(&format!("m=3,mode=chain,crosscheck=1{extra}"));
        let mut full = prep(&format!("m=3,mode=full,crosscheck=1{extra}"));
        let mut grid = prep("m=3,engine=exhaustive");
        let mut rng = StdRng::seed_from_u64(31);
        let g = inst.generator_point();
        let mut hits = 0;
        for k in 0..32u64 {
            let target = if k.is_multiple_of(3) {
                let idx: Vec<usize> = (0..3).map(|_| rng.gen_range(0..fb.points.len())).collect();
                sum(curve, fb, &idx)
            } else {
                curve.mul(&mut GroupOps::default(), g, rng.gen_range(1..inst.r))
            };
            if target.infinity {
                continue;
            }
            let run = |o: &mut PkmSemaevOracle| {
                let got = o.decompose(
                    &ctx(inst),
                    fb,
                    &mut GroupOps::default(),
                    &mut OracleCounters::default(),
                    target,
                );
                if let Some(idx) = &got {
                    assert_eq!(idx.len(), 3, "{label}");
                    assert_eq!(
                        sum(curve, fb, idx),
                        target,
                        "{label}: relation sums to the target"
                    );
                }
                got.is_some()
            };
            let (a, b, c) = (run(&mut chain), run(&mut full), run(&mut grid));
            assert_eq!(a, c, "{label}: chain F4 against the grid, target {k}");
            assert_eq!(b, c, "{label}: full F4 against the grid, target {k}");
            assert_eq!(
                c,
                decomposable.contains(&curve.key(&target)),
                "{label}: target {k}"
            );
            hits += usize::from(c);
        }
        assert!(hits >= 5, "{label}: planted targets decompose ({hits})");
        for o in [&chain, &full] {
            let n = o.relation_counters();
            assert_eq!(
                n.get("pkm_crosscheck_disagreements").copied().unwrap_or(0),
                0,
                "{label}"
            );
            let tot = o.solver_totals().unwrap();
            // A tiny Boolean system can be decided by the preprocessing alone
            // (zero elimination multiply-adds), so only the calls are required.
            assert!(
                tot.calls > 0,
                "{label}: F4 calls (calls {}, ops {})",
                tot.calls,
                tot.ops
            );
        }
    }

    /// `m = 3` on PKM's isogeny tower, both presentations.
    #[test]
    fn isogeny_tower_m3_agrees_with_grid() {
        use crate::cryptanalysis::ic_framework::prime_tower::{isogeny_tower, PrimeIsogenyBase};
        let inst = instance();
        let t = 3;
        let b = PrimeIsogenyBase { instance: &inst };
        let mut params = Params::default();
        params.set("t", t.to_string());
        let fb = b
            .build(&ctx(&inst), &params, &mut GroupOps::default())
            .expect("isogeny base");
        let tower = isogeny_tower(inst.curve.p, t, 0, &mut GroupOps::default()).unwrap();
        let steps: Vec<(u64, u64)> = tower.steps.iter().map(|s| (s.xi, s.tau)).collect();
        let (v, c) = (tower.v.clone(), tower.c);
        agrees_with_grid_m3(
            &inst,
            &fb,
            &|| PkmSemaevOracle::new_isogeny(3, t, v.clone(), steps.clone(), c),
            "isogeny m=3",
            "",
        );
    }

    /// `m = 3` on the bit form, with and without hybrid guessing.
    #[test]
    fn bit_interval_m3_agrees_with_grid() {
        use crate::cryptanalysis::ic_boundary::prime_factor_base;
        let inst = instance();
        let fb = prime_factor_base(&inst, 4);
        let max = *fb.abscissa_list.iter().max().unwrap();
        let t = 64 - max.leading_zeros();
        for extra in ["", ",guess=1", ",guess=2"] {
            agrees_with_grid_m3(
                &inst,
                &fb,
                &|| PkmSemaevOracle::new_bits(3, t, 0),
                &format!("bits m=3{extra}"),
                extra,
            );
        }
    }

    /// A prime-order curve `y² = x³ + x + b` over a prime near 1000, found
    /// by counting points: small enough that a whole pipeline needs a few
    /// dozen F4 calls.
    fn small_prime_order_instance() -> PrimeInstance {
        let is_prime = |n: u64| {
            n > 1
                && (2..)
                    .take_while(|d| d * d <= n)
                    .all(|d| !n.is_multiple_of(d))
        };
        for p in (1009u64..1200).filter(|&p| is_prime(p)) {
            for b in 1..p {
                let curve = PrimeCurve { p, a: 1, b };
                if (4 + 27 * b % p * b) % p == 0 {
                    continue;
                }
                let n = (p + 1) as i64
                    + (0..p)
                        .map(|x| i64::from(curve.legendre(curve.rhs(x))))
                        .sum::<i64>();
                let n = n as u64;
                if !is_prime(n) {
                    continue;
                }
                let (x, y) = (1..p)
                    .find_map(|x| curve.sqrt(curve.rhs(x)).filter(|&y| y != 0).map(|y| (x, y)))
                    .expect("a point of order n");
                return PrimeInstance {
                    name: format!("small-p{p}-b{b}"),
                    curve,
                    group_order: n,
                    r: n,
                    cofactor: 1,
                    generator: (x, y),
                };
            }
        }
        panic!("no prime-order curve in range")
    }

    /// The two arms of the sweeper's `prime-algebraic` space, end to end
    /// through `from_spec` (the constructor ecbench and `ic bench` share):
    /// Semaev's naive interval `prime-abscissa:below=2^T` with `bits=T`, and
    /// PKM's auxiliary-curve tower `prime-isogeny`.  Both recover the
    /// planted logarithm, and the naive base is exactly the carriers below
    /// `2^T`.
    #[test]
    fn both_prime_algebraic_arms_solve_through_from_spec() {
        use crate::cryptanalysis::ic_boundary::Calibration;
        use crate::cryptanalysis::ic_framework::plugins::PrimeAbscissaBase;
        use crate::cryptanalysis::ic_framework::prime_tower::PrimeIsogenyBase;
        use crate::cryptanalysis::ic_framework::stages::Targets;
        use crate::cryptanalysis::ic_framework::{run_pipeline, PipelineSpec};
        let inst = small_prime_order_instance();
        let curve = &inst.curve;
        let mut ops = GroupOps::default();
        let planted = 1 + 7919 % (inst.r - 1);
        let target = curve.mul(&mut ops, inst.generator_point(), planted);
        let ictx = InstanceCtx {
            group: curve,
            generator: inst.generator_point(),
            target,
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: None,
        };

        let naive = PrimeAbscissaBase { instance: &inst };
        let (_, below) = Params::parse_spec("prime-abscissa:below=32").unwrap();
        let fb = naive.build(&ictx, &below, &mut ops).unwrap();
        let carriers: Vec<u64> = (1..32)
            .filter(|&x| curve.legendre(curve.rhs(x)) >= 0)
            .collect();
        let mut got = fb.abscissa_list.clone();
        got.sort_unstable();
        assert_eq!(got, carriers);
        let (_, both) = Params::parse_spec("prime-abscissa:below=32,size=4").unwrap();
        assert!(naive.build(&ictx, &both, &mut ops).is_err());

        let arms: [(&str, &str, &dyn FactorBaseBuilder<PrimeCurve>); 2] = [
            (
                "prime-abscissa:below=32",
                "pkm-semaev:m=2,bits=5,lo=0",
                &naive,
            ),
            (
                "prime-isogeny:t=4,seed=0,aux=ordinary",
                "pkm-semaev:m=2",
                &PrimeIsogenyBase { instance: &inst },
            ),
        ];
        for (fb_spec, or_spec, builder) in arms {
            let (fb_name, factor_base_params) = Params::parse_spec(fb_spec).unwrap();
            let (oracle, oracle_params) = Params::parse_spec(or_spec).unwrap();
            let spec = PipelineSpec {
                factor_base: fb_name,
                factor_base_params,
                oracle,
                oracle_params,
                targets: Targets::Random,
                linalg: "incremental-gauss".into(),
                max_trials: 20_000,
                seed: 11,
                ..Default::default()
            };
            let mut o = PkmSemaevOracle::from_spec(2, &spec, &inst).unwrap();
            let report = run_pipeline(
                &ictx,
                &spec,
                builder,
                &mut o,
                planted,
                &Calibration::default(),
                Some(1.0),
            )
            .unwrap_or_else(|e| panic!("{fb_spec}: {e}"));
            assert!(report.verified, "{fb_spec}: {report:?}");
        }

        // A base the oracle cannot solve on is refused, not silently run.
        let spec = PipelineSpec {
            factor_base: "glv-orbit".into(),
            oracle: "pkm-semaev".into(),
            ..Default::default()
        };
        assert!(PkmSemaevOracle::from_spec(2, &spec, &inst).is_err());
    }

    /// A small Montgomery miniature `y² = x³ + A x² + x` over `p ≡ 5 (8)`
    /// with `A² − 4` a non-residue (so `T₀ = (0, 0)` is the only rational
    /// 2-torsion point), in short Weierstrass form `x_W = u + A/3`.
    /// Returns the instance and `A/3`.
    fn montgomery_instance() -> (PrimeInstance, u64) {
        let is_prime = |n: u64| {
            n > 1
                && (2..)
                    .take_while(|d| d * d <= n)
                    .all(|d| !n.is_multiple_of(d))
        };
        let p = (10_007u64..).find(|&n| n % 8 == 5 && is_prime(n)).unwrap();
        let leg = |v: u64| crate::cryptanalysis::residual_walk::pow_mod(v % p, (p - 1) / 2, p);
        let inv = |v: u64| crate::cryptanalysis::residual_walk::pow_mod(v % p, p - 2, p);
        let am = (3u64..).find(|&am| leg(am * am + p - 4) == p - 1).unwrap();
        let third = inv(3);
        let a = subm(1, mulm(mulm(am, am, p), third, p), p);
        let b = subm(
            mulm(mulm(2, mulm(am, mulm(am, am, p), p), p), inv(27), p),
            mulm(am, third, p),
            p,
        );
        let curve = PrimeCurve { p, a, b };
        let mut n = 1i64; // infinity
        let mut gen = None;
        for x in 0..p {
            let rhs = curve.rhs(x);
            match curve.legendre(rhs) {
                0 => n += 1,
                1 => {
                    n += 2;
                    if gen.is_none() && x > 2 {
                        gen = Some((x, curve.sqrt(rhs).unwrap()));
                    }
                }
                _ => {}
            }
        }
        let inst = PrimeInstance {
            name: "c25519-toy".into(),
            curve,
            group_order: n as u64,
            r: n as u64,
            cofactor: 1,
            generator: gen.unwrap(),
        };
        (inst, mulm(am, third, p))
    }

    fn edwards_oracle(m: u32, t: u32, shift: Option<u64>) -> PkmSemaevOracle {
        let mut o = PkmSemaevOracle::new_edwards_bits(m, t);
        o.membership = Membership::EdwardsBits { shift };
        o
    }

    /// The shift is the unique rational root of `x³ + ax + b`, i.e. `A/3`.
    #[test]
    fn edwards_shift_is_the_two_torsion_abscissa() {
        let (inst, third) = montgomery_instance();
        assert_eq!(edwards_shift(&inst.curve).unwrap(), third);
        let c = &inst.curve;
        assert_eq!(c.rhs(third), 0, "T₀ is 2-torsion");
        // A curve with no rational root is refused.
        let bad = PrimeCurve {
            p: c.p,
            a: c.a,
            b: addm(c.b, 1, c.p),
        };
        if (0..c.p).all(|x| bad.rhs(x) != 0) {
            assert!(edwards_shift(&bad).is_err());
        }
    }

    /// Edwards bits, `m = 2`, on the Montgomery miniature with the derived
    /// shift: F4 against the grid and the exact decomposable set, with and
    /// without hybrid guessing.
    #[test]
    fn edwards_bits_form_agrees_with_grid() {
        let (inst, third) = montgomery_instance();
        for t in [4u32, 5] {
            let fb = edwards_factor_base(&inst, t, third);
            assert!(fb.columns >= 3, "t = {t} columns = {}", fb.columns);
            agrees_with_grid(
                &inst,
                &fb,
                &|| edwards_oracle(2, t, None),
                &format!("edwards t={t}"),
                "",
            );
            for g in 1..t.min(3) {
                agrees_with_grid(
                    &inst,
                    &fb,
                    &|| edwards_oracle(2, t, None),
                    &format!("edwards t={t} guess={g}"),
                    &format!(",guess={g}"),
                );
            }
        }
        // An explicit shift on a curve that is not Montgomery: the form is
        // a Möbius change of variable for any shift.
        let inst = instance();
        let fb = edwards_factor_base(&inst, 5, 5);
        agrees_with_grid(
            &inst,
            &fb,
            &|| edwards_oracle(2, 5, Some(5)),
            "edwards shift=5",
            "",
        );
        // A base outside the interval is refused.
        let (inst, third) = montgomery_instance();
        let fb = edwards_factor_base(&inst, 5, third);
        let (_, params) = Params::parse_spec("pkm-semaev:m=2").unwrap();
        assert!(edwards_oracle(2, 3, None)
            .prepare(&ctx(&inst), &fb, &params, &mut GroupOps::default())
            .is_err());
    }

    /// Edwards bits, `m = 3`: planted sums of three base points decompose
    /// under the chain, the full S₄ and the grid, and relations sum to the
    /// target.
    #[test]
    fn edwards_bits_m3_planted_decompositions() {
        let (inst, third) = montgomery_instance();
        let t = 3;
        let fb = edwards_factor_base(&inst, t, third);
        let (curve, mut rng) = (&inst.curve, StdRng::seed_from_u64(11));
        for spec in [
            "m=3",
            "m=3,mode=full",
            "m=3,engine=exhaustive",
            "m=3,guess=1",
        ] {
            let (_, params) = Params::parse_spec(&format!("pkm-semaev:{spec}")).unwrap();
            let mut o = edwards_oracle(3, t, None);
            o.prepare(&ctx(&inst), &fb, &params, &mut GroupOps::default())
                .unwrap();
            let mut found = 0;
            for _ in 0..6 {
                let idx: Vec<usize> = (0..3).map(|_| rng.gen_range(0..fb.points.len())).collect();
                let target = sum(curve, &fb, &idx);
                if target.infinity {
                    continue;
                }
                let got = o
                    .decompose(
                        &ctx(&inst),
                        &fb,
                        &mut GroupOps::default(),
                        &mut OracleCounters::default(),
                        target,
                    )
                    .unwrap_or_else(|| panic!("{spec}: planted target not decomposed"));
                assert_eq!(sum(curve, &fb, &got), target, "{spec}");
                found += 1;
            }
            assert!(found > 0, "{spec}");
        }
    }

    /// The oracle refuses a base that is not its tower.
    #[test]
    fn refuses_a_foreign_base() {
        let inst = instance();
        let fb = base(&inst, 3);
        let (_, params) = Params::parse_spec("pkm-semaev:m=3").unwrap();
        let mut o = PkmSemaevOracle::new(3, 2, 1);
        let err = o
            .prepare(&ctx(&inst), &fb, &params, &mut GroupOps::default())
            .unwrap_err();
        assert!(err.contains("not in g·μ"), "{err}");
    }

    /// A small curve `y² = x³ − 3x + b` over `F_10007` with `#E = h·r`,
    /// `r` prime, `h ∈ {8, 4}` and the whole `E[h]` rational, found by a
    /// fixed search so the test is deterministic.  Returns the instance and
    /// the brute-force `E[h]` (every point killed by `h`).
    fn cofactor_instance() -> (PrimeInstance, Vec<PrimePoint>) {
        let p = 10_007u64;
        let is_prime = |n: u64| {
            n > 1
                && (2..)
                    .take_while(|d| d * d <= n)
                    .all(|d| !n.is_multiple_of(d))
        };
        for b in 1..p {
            let curve = PrimeCurve { p, a: p - 3, b };
            let mut pts = vec![];
            for x in 0..p {
                let rhs = curve.rhs(x);
                if curve.legendre(rhs) >= 0 {
                    if let Some(y) = curve.sqrt(rhs) {
                        pts.push(PrimePoint::affine(x, y));
                        if y != 0 {
                            pts.push(PrimePoint::affine(x, p - y));
                        }
                    }
                }
            }
            let n = pts.len() as u64 + 1;
            for h in [8u64, 4] {
                if !n.is_multiple_of(h) || !is_prime(n / h) || n / h < 500 {
                    continue;
                }
                let r = n / h;
                let mut ops = GroupOps::default();
                let torsion: Vec<PrimePoint> = std::iter::once(curve.identity())
                    .chain(
                        pts.iter()
                            .copied()
                            .filter(|&q| curve.mul(&mut ops, q, h).infinity),
                    )
                    .collect();
                if torsion.len() as u64 != h {
                    continue;
                }
                let gen = pts
                    .iter()
                    .map(|&q| curve.mul(&mut ops, q, h))
                    .find(|q| !q.infinity)
                    .unwrap();
                let inst = PrimeInstance {
                    name: format!("cof{h}-b{b}"),
                    curve,
                    group_order: n,
                    r,
                    cofactor: h,
                    generator: (gen.x, gen.y),
                };
                return (inst, torsion);
            }
        }
        panic!("no cofactor curve found");
    }

    /// `lifts=1`: the verdict is "R + K decomposes for some K in E[h]"
    /// (brute force over the whole torsion), every relation is accepted
    /// by the pipeline's rule, the fiber is charged to set-up, and `h = 1`
    /// is unchanged.
    #[test]
    fn cofactor_lifts_match_the_brute_force_fiber() {
        use crate::cryptanalysis::ic_boundary::prime_factor_base;
        let (inst, torsion) = cofactor_instance();
        let curve = &inst.curve;
        let h = inst.cofactor;
        let fb = prime_factor_base(&inst, 20);
        let max = *fb.abscissa_list.iter().max().unwrap();
        let t = 64 - max.leading_zeros();
        let decomposable = decomposable_points(curve, &fb, 2, &mut GroupOps::default());
        let c = InstanceCtx {
            cofactor: inst.cofactor,
            ..ctx(&inst)
        };
        let prep = |spec: &str| {
            let (_, params) = Params::parse_spec(&format!("pkm-semaev:{spec}")).unwrap();
            let mut o = PkmSemaevOracle::new_bits(2, t, 0);
            let mut setup = GroupOps::default();
            o.prepare(&c, &fb, &params, &mut setup).expect("prepare");
            (o, setup)
        };
        let (mut blind, setup0) = prep("m=2,mode=full");
        let (mut lifted, setup1) = prep("m=2,mode=full,lifts=1");
        let (mut lifted_g, _) = prep("m=2,mode=full,lifts=1,guess=1");
        assert_eq!(setup0.adds + setup0.doubles, 0, "lifts=0 has no set-up");
        assert!(setup1.adds + setup1.doubles > 0, "the fiber is charged");
        let native = lifted.setup_native();
        assert_eq!(native["pkm_fiber_size"], h);
        assert_eq!(native["pkm_fiber_complete"], 1);
        // The pipeline's row: Σ coef·x_col = h·a (mod r), x_col the
        // logarithm to base G of the projected column point [h]P_col.
        let mut ops = GroupOps::default();
        let mut dlog = std::collections::HashMap::new();
        let mut acc = curve.identity();
        let g = inst.generator_point();
        for j in 0..inst.r {
            dlog.insert(curve.key(&acc), j);
            acc = curve.add(&mut ops, acc, g);
        }
        // One unknown per column: the logarithm of the coef-1 member [h]P_c
        // (its negation carries coef r - 1 on the same unknown).
        let mut col_log = vec![0u64; fb.columns];
        for i in 0..fb.points.len() {
            if fb.coef_of[i] == 1 {
                col_log[fb.col_of[i]] = dlog[&curve.key(&curve.mul(&mut ops, fb.points[i], h))];
            }
        }
        let mut rng = StdRng::seed_from_u64(77);
        let (mut blind_hits, mut lift_hits, mut by_lift) = (0, 0, 0);
        for _ in 0..80u64 {
            let a = rng.gen_range(1..inst.r);
            let target = curve.mul(&mut ops, g, a);
            let truth_blind = decomposable.contains(&curve.key(&target));
            let truth_lift = torsion.iter().any(|&kk| {
                let s = curve.add(&mut ops, target, kk);
                decomposable.contains(&curve.key(&s))
            });
            let run = |o: &mut PkmSemaevOracle| {
                o.decompose(
                    &c,
                    &fb,
                    &mut GroupOps::default(),
                    &mut OracleCounters::default(),
                    target,
                )
            };
            assert_eq!(run(&mut blind).is_some(), truth_blind, "blind, a = {a}");
            for o in [&mut lifted, &mut lifted_g] {
                let got = run(o);
                assert_eq!(got.is_some(), truth_lift, "lifts verdict, a = {a}");
                if let Some(idx) = got {
                    // Sum = R + K for some K in E[h]: equal after projection.
                    let s = sum(curve, &fb, &idx);
                    let d = curve.add(&mut ops, s, curve.neg(target));
                    assert!(torsion.contains(&d), "sum - R is in E[h]");
                    assert_eq!(
                        curve.mul(&mut ops, s, h),
                        curve.mul(&mut ops, target, h),
                        "projected sum equals projected target"
                    );
                    let lhs = idx.iter().fold(0u128, |acc, &i| {
                        (acc + fb.coef_of[i] as u128 * col_log[fb.col_of[i]] as u128)
                            % inst.r as u128
                    });
                    assert_eq!(
                        lhs,
                        (h as u128 * a as u128) % inst.r as u128,
                        "row: Σ coef·x = h·a (mod r)"
                    );
                    if !truth_blind {
                        by_lift += 1;
                    }
                }
            }
            blind_hits += usize::from(truth_blind);
            lift_hits += usize::from(truth_lift);
        }
        eprintln!(
            "cofactor_lifts: h = {h}, 80 targets, blind decomposable {blind_hits}, lift-decomposable {lift_hits}, relations from a non-identity lift (both lift oracles) {by_lift}"
        );
        assert!(lift_hits > blind_hits, "lifts raise the yield");
        assert!(by_lift > 0, "some relations come only from a lift");
        assert!(lifted.relation_counters()["pkm_lift_hits_nonidentity"] > 0);
        assert!(
            lifted.solver_totals().unwrap().calls > blind.solver_totals().unwrap().calls,
            "every extra F4 call is charged"
        );

        // h = 1: the fiber is trivial and lifts=1 is the blind oracle.
        let inst1 = instance();
        let fb1 = base(&inst1, 3);
        let (_, p1) = Params::parse_spec("pkm-semaev:m=2,lifts=1").unwrap();
        let mut o1 = PkmSemaevOracle::new(2, 3, 1);
        o1.prepare(&ctx(&inst1), &fb1, &p1, &mut GroupOps::default())
            .unwrap();
        assert_eq!(o1.setup_native()["pkm_fiber_size"], 1);
        let mut o0 = oracle(&inst1, &fb1, 2, 3, "m=2");
        let mut rng = StdRng::seed_from_u64(5);
        for _ in 0..20 {
            let tg = inst1
                .curve
                .mul(&mut ops, inst1.generator_point(), rng.gen_range(1..inst1.r));
            let run = |o: &mut PkmSemaevOracle| {
                o.decompose(
                    &ctx(&inst1),
                    &fb1,
                    &mut GroupOps::default(),
                    &mut OracleCounters::default(),
                    tg,
                )
            };
            assert_eq!(run(&mut o1), run(&mut o0));
        }
        assert_eq!(
            o1.solver_totals().unwrap().calls,
            o0.solver_totals().unwrap().calls
        );
    }

    /// `lifts=2` (one shared F4 system per target): the verdicts and the
    /// relations equal `lifts=1`'s and the brute-force fiber's, on random
    /// and planted targets, with and without guessing and the F4 trace.
    #[test]
    fn shared_lift_system_matches_lifts_one_and_the_brute_force_fiber() {
        use crate::cryptanalysis::ic_boundary::prime_factor_base;
        let (inst, torsion) = cofactor_instance();
        let curve = &inst.curve;
        let h = inst.cofactor;
        let fb = prime_factor_base(&inst, 20);
        let max = *fb.abscissa_list.iter().max().unwrap();
        let t = 64 - max.leading_zeros();
        let decomposable = decomposable_points(curve, &fb, 2, &mut GroupOps::default());
        let c = InstanceCtx {
            cofactor: inst.cofactor,
            ..ctx(&inst)
        };
        let prep = |spec: &str| {
            let (_, params) = Params::parse_spec(&format!("pkm-semaev:{spec}")).unwrap();
            let mut o = PkmSemaevOracle::new_bits(2, t, 0);
            o.prepare(&c, &fb, &params, &mut GroupOps::default())
                .expect("prepare");
            o
        };
        let mut one = prep("m=2,mode=full,lifts=1");
        let mut shared = vec![
            prep("m=2,mode=full,lifts=2"),
            prep("m=2,mode=full,lifts=2,guess=1"),
            prep("m=2,mode=full,lifts=2,trace=1"),
        ];
        assert!(shared[0].shared, "the fiber is nontrivial: shared system");
        // h^{-1} mod r (r prime), to plant R with S = R + K.
        let mut hinv = 1u128;
        let (mut bsq, mut e) = (h as u128 % inst.r as u128, inst.r as u128 - 2);
        while e > 0 {
            if e & 1 == 1 {
                hinv = hinv * bsq % inst.r as u128;
            }
            bsq = bsq * bsq % inst.r as u128;
            e >>= 1;
        }
        let mut ops = GroupOps::default();
        let g = inst.generator_point();
        let mut rng = StdRng::seed_from_u64(99);
        let mut targets = Vec::new();
        for _ in 0..40 {
            targets.push(curve.mul(&mut ops, g, rng.gen_range(1..inst.r)));
        }
        let mut planted = 0;
        while planted < 12 {
            let (i, j) = (
                rng.gen_range(0..fb.points.len()),
                rng.gen_range(0..fb.points.len()),
            );
            let s = curve.add(&mut ops, fb.points[i], fb.points[j]);
            let ph = curve.mul(&mut ops, s, h);
            if ph.infinity {
                continue;
            }
            targets.push(curve.mul(&mut ops, ph, hinv as u64));
            planted += 1;
        }
        let (mut hits, mut calls_before) = (0, 0u64);
        for (n, &target) in targets.iter().enumerate() {
            let truth = torsion.iter().any(|&kk| {
                let s = curve.add(&mut ops, target, kk);
                decomposable.contains(&curve.key(&s))
            });
            if n >= 40 {
                assert!(truth, "planted target {n} is lift-decomposable");
            }
            let run = |o: &mut PkmSemaevOracle| {
                o.decompose(
                    &c,
                    &fb,
                    &mut GroupOps::default(),
                    &mut OracleCounters::default(),
                    target,
                )
            };
            let want = run(&mut one);
            assert_eq!(want.is_some(), truth, "lifts=1 verdict, target {n}");
            for o in shared.iter_mut() {
                let got = run(o);
                assert_eq!(got.is_some(), truth, "lifts=2 verdict, target {n}");
                if let Some(idx) = &got {
                    let s = sum(curve, &fb, idx);
                    let d = curve.add(&mut ops, s, curve.neg(target));
                    assert!(torsion.contains(&d), "sum - R is in E[h]");
                    assert_eq!(
                        curve.mul(&mut ops, s, h),
                        curve.mul(&mut ops, target, h),
                        "[h]sum = [h]R"
                    );
                    assert_eq!(&got, &want, "same relation as lifts=1");
                }
            }
            hits += usize::from(truth);
        }
        let calls = |o: &PkmSemaevOracle| o.solver_totals().unwrap().calls;
        calls_before += calls(&one);
        assert_eq!(
            calls(&shared[0]),
            targets.len() as u64,
            "one call per target"
        );
        assert_eq!(
            calls(&shared[2]),
            targets.len() as u64,
            "one call per target (trace)"
        );
        // Guessing: one call per branch reached (a non-refuted branch ends the
        // target), so between one and two calls per target with 2 branches.
        let g_calls = calls(&shared[1]);
        assert!(
            g_calls > targets.len() as u64 && g_calls <= 2 * targets.len() as u64,
            "guess=1: {g_calls} calls for {} targets",
            targets.len()
        );
        let nat = shared[0].relation_counters();
        assert!(
            nat["pkm_shared_poly_fp_mults_uncharged"] > 0,
            "P_R construction is listed"
        );
        assert_eq!(nat["pkm_shared_systems"], targets.len() as u64);
        eprintln!(
            "shared_lifts: {} targets, lift-decomposable {hits}, F4 calls lifts=1 {calls_before}, lifts=2 {}",
            targets.len(),
            calls(&shared[0])
        );
        assert!(hits > 0);

        // h = 1: lifts=2 is the blind oracle (no shared system).
        let inst1 = instance();
        let fb1 = base(&inst1, 3);
        let (_, p1) = Params::parse_spec("pkm-semaev:m=2,lifts=2").unwrap();
        let mut o1 = PkmSemaevOracle::new(2, 3, 1);
        o1.prepare(&ctx(&inst1), &fb1, &p1, &mut GroupOps::default())
            .unwrap();
        assert!(!o1.shared);
        let mut o0 = oracle(&inst1, &fb1, 2, 3, "m=2");
        for _ in 0..10 {
            let tg = inst1
                .curve
                .mul(&mut ops, inst1.generator_point(), rng.gen_range(1..inst1.r));
            let run = |o: &mut PkmSemaevOracle| {
                o.decompose(
                    &ctx(&inst1),
                    &fb1,
                    &mut GroupOps::default(),
                    &mut OracleCounters::default(),
                    tg,
                )
            };
            assert_eq!(run(&mut o1), run(&mut o0));
        }
    }
}
