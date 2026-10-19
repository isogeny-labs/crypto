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
use std::time::{Duration, Instant};

use super::prime_tower::primitive_two_power_root;
use super::stages::{
    DecompositionOracle, InstanceCtx, Params, SolverCost, SolverTotals, SystemShape,
};
use crate::cryptanalysis::f4_fp_tower::{
    f4_tower, Fp, RPoly, SquareRule, TowerF4Options, TowerRing,
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
        if c % p != 0 {
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

/// The oracle.  Built by the harness with the base's `t` and `g`, which it
/// reads from the `prime-kummer` spec; `prepare` checks that every
/// abscissa of the base is in `V` and refuses any other base.
pub struct PkmSemaevOracle {
    m: u32,
    t: u32,
    g: u64,
    mode: PkmMode,
    engine: PkmEngine,
    max_degree: u32,
    max_nnz: Option<u64>,
    stop_staircase: Option<usize>,
    budget: Option<Duration>,
    crosscheck: bool,
    prepared: Option<Prepared>,
    totals: SolverTotals,
    shape: Option<SystemShape>,
    setup_mults: u64,
    native: BTreeMap<String, u64>,
}

impl PkmSemaevOracle {
    /// `m` summands over the Kummer tower `g·μ_{2^t}`.
    pub fn new(m: u32, t: u32, g: u64) -> Self {
        PkmSemaevOracle {
            m,
            t,
            g,
            mode: PkmMode::Chain,
            engine: PkmEngine::F4,
            max_degree: 12,
            max_nnz: None,
            stop_staircase: None,
            budget: None,
            crosscheck: false,
            prepared: None,
            totals: SolverTotals {
                solver: "f4-fp-tower".into(),
                op_unit: "F_p multiply-adds (elimination only)".into(),
                ..Default::default()
            },
            shape: None,
            setup_mults: 0,
            native: BTreeMap::new(),
        }
    }

    /// A wall-clock budget per F4 call (the harness's
    /// `solver_budget_seconds`); `None` is unbounded.
    pub fn with_budget(mut self, budget: Option<Duration>) -> Self {
        self.budget = budget;
        self
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
    fn build_system(&self, curve: &PrimeCurve) -> Prepared {
        let p = curve.p;
        let (a, b) = (curve.a % p, curve.b % p);
        let (m, t) = (self.m as usize, self.t as usize);
        let n_t = m * t;
        let n_free = if self.mode == PkmMode::Chain {
            m - 2
        } else {
            0
        };
        let n = n_t + n_free + 1; // last variable: x_R
        let xr_var = n - 1;
        let x = |i: usize| Pol::var(n, i * t).scale(self.g, p);
        let xr = Pol::var(n, xr_var);
        let raw: Vec<Pol> = match (self.mode, m) {
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
        };
        let mut rules = Vec::with_capacity(n_t);
        for i in 0..m {
            for j in 0..t {
                let top = j + 1 == t;
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
        let zeta = primitive_two_power_root(p, self.t).expect("checked in prepare");
        let mut v = Vec::with_capacity(1 << t);
        let mut z = self.g % p;
        for _ in 0..(1u64 << t) {
            v.push(z);
            z = mulm(z, zeta, p);
        }
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
    fn specialise(&mut self, x_r: u64) -> Vec<RPoly> {
        let prep = self.prepared.as_ref().expect("prepared");
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
        self.bump("pkm_system_fp_mults_uncharged", mults);
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

impl DecompositionOracle<PrimeCurve> for PkmSemaevOracle {
    fn name(&self) -> &str {
        "pkm-semaev"
    }

    fn summands(&self) -> u32 {
        self.m
    }

    fn describe(&self, params: &Params) -> String {
        format!(
            "Semaev S_(m+1) over F_p with each x_i on the base's Kummer tower (x/g)^(2^t) = 1 \
             (PKM), m = {}, t = {}, g = {}, mode {}, engine {}",
            self.m,
            self.t,
            self.g,
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
        ]
    }

    fn prepare(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        params: &Params,
        _ops: &mut GroupOps,
    ) -> Result<(), String> {
        self.read_params(params)?;
        let curve = ctx.group;
        let p = curve.p;
        if p >= 1 << 31 {
            return Err("pkm-semaev: the tower engine needs p < 2^31".into());
        }
        primitive_two_power_root(p, self.t)
            .ok_or_else(|| format!("pkm-semaev: 2^{} does not divide p − 1", self.t))?;
        // The base must be this tower: every abscissa x has (x/g)^(2^t) = 1.
        let g = self.g % p;
        if g == 0 {
            return Err("pkm-semaev: g must be non-zero".into());
        }
        let ginv = crate::cryptanalysis::residual_walk::pow_mod(g, p - 2, p);
        let mut check_mults = 0u64;
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
        let prep = self.build_system(curve);
        let n_vars = prep.ring.n_vars();
        let mut degrees = prep.degrees.clone();
        degrees.extend(std::iter::repeat_n(2, prep.tower_eqs));
        self.shape = Some(SystemShape {
            n_vars,
            n_equations: prep.eqs.len() + prep.tower_eqs,
            degrees,
            semi_regular_degree: None,
        });
        self.setup_mults = check_mults;
        self.prepared = Some(prep);
        Ok(())
    }

    fn setup_native(&self) -> BTreeMap<String, u64> {
        let mut m = BTreeMap::new();
        if self.setup_mults > 0 {
            m.insert("pkm_base_check_fp_mults_uncharged".into(), self.setup_mults);
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
        if point.infinity || self.prepared.is_none() {
            return None;
        }
        let curve = ctx.group;
        let x_r = point.x;
        let solutions = match self.engine {
            PkmEngine::Exhaustive => {
                let started = Instant::now();
                let (sols, mults) = self.grid(curve, x_r, false);
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
                let sys = self.specialise(x_r);
                let ring = self.prepared.as_ref().expect("prepared").ring.clone();
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
                let rep = f4_tower(&sys, &ring, &opts);
                let undecided = !rep.inconsistent
                    && rep.staircase_at_stop.is_none()
                    && (rep.timed_out || rep.oversize || rep.pairs_above_bound > 0);
                let mut extra = BTreeMap::new();
                extra.insert("max_cols_max".to_string(), rep.max_cols as u64);
                extra.insert("steps".to_string(), rep.steps as u64);
                extra.insert("refutations".to_string(), u64::from(rep.inconsistent));
                extra.insert(
                    "staircase_stops".to_string(),
                    u64::from(rep.staircase_at_stop.is_some()),
                );
                let cost = SolverCost {
                    ops: rep.muladds,
                    op_unit: "F_p multiply-adds (elimination only)".into(),
                    wall_ns: (rep.ms * 1e6) as u64,
                    peak_bytes: 0,
                    degree_reached: Some(rep.degree_reached),
                    solving_degree: (!undecided).then_some(rep.solving_degree_max),
                    timed_out: rep.timed_out,
                    extra,
                };
                let shape = self.shape.clone().unwrap_or_default();
                self.totals.absorb(&shape, Some(&cost), undecided);
                if rep.inconsistent {
                    if self.crosscheck {
                        let (sols, mults) = self.grid(curve, x_r, true);
                        self.bump("pkm_crosscheck_fp_mults", mults);
                        if !sols.is_empty() {
                            self.bump("pkm_crosscheck_disagreements", 1);
                        }
                    }
                    return None;
                }
                // Consistent, or undecided: the grid reads the solutions.
                let (sols, mults) = self.grid(curve, x_r, false);
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
            let target = if k % 4 == 0 {
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
}
