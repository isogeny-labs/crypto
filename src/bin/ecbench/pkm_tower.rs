//! A prime-field algebraic decomposition oracle on a Petit–Kosters–Messeng
//! Kummer tower, as an `ic.pipeline` plug-in, with F4, signature-F5 and an
//! F6-style geometric closure as interchangeable engines.
//!
//! `research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md` designed this
//! oracle (§4) and measured its systems' solving degree with the standalone
//! tower engines; nothing ran it inside `ic.pipeline` until now.
//!
//! ## The base
//!
//! `prime-tower:t=T,seed=S` is a Kummer tower: `V = { g·ζ^k : k < 2^T }`
//! with `ζ` of exact order `2^T` in `F_p^*` (so `2^T | p − 1`) and `g` a
//! random coset representative drawn from `seed`. Every `x ∈ V` that carries
//! a point gives both points above it, one column per abscissa (negation
//! fold). `x ∈ V` is the tower `y_0 = x/g`, `y_{j+1} = y_j²`, `y_{T−1}² = 1`.
//!
//! ## The oracle
//!
//! `pkm-tower:t=T,seed=S,engine=E` decomposes `R` into two base points
//! (`m = 2` only). The system is Semaev's `S₃(g·y_{1,0}, g·y_{2,0}, x_R)`
//! in the tower ring `F_p[y_1, y_2]/(towers)`, `N = 2T` unknowns.
//!
//! Every engine but `enum` searches one splitting tree:
//!
//! - A node fixes the top `d` levels of block 1's tower. That is one linear
//!   constraint `y_{1,T−d} = v`: the levels above follow from the rules, and
//!   `2^{T−d}` abscissae `x_1` remain. The two children are the two square
//!   roots of `v`.
//! - At each node the engine decides the system plus that constraint.
//!   **Inconsistent** prunes the node. Otherwise the search descends,
//!   because a degree-capped basis that is not inconsistent proves nothing.
//! - A leaf (`d = T`) fixes `x_1`. `S₃(x_1, X, x_R)` is then a quadratic in
//!   `X`: its roots are solved and looked up in the base, and the signs are
//!   lifted by group arithmetic (`lift_abscissae`).
//!
//! The engines:
//!
//! | engine   | node decision                         | closure |
//! |----------|---------------------------------------|---------|
//! | `f4`     | `f4_fp_tower` (F4 on the tower ring)  | leaves only |
//! | `sig-f5` | `sig_fp_tower` (signature F4, F5/GVW criteria) | leaves only |
//! | `f6`     | `f4_fp_tower`                         | a node with at most `cap` candidates is closed geometrically |
//! | `enum`   | none                                  | the root is closed geometrically (no algebra) |
//!
//! A **geometric closure** enumerates the node's candidates `x_1`. For each
//! one that carries a point `P_1` it computes `R − P_1` and `R + P_1` (two
//! charged additions) and looks their abscissae up in the base. That is
//! F6-IC's idea, exact factor-base geometry closing a branch of the
//! algebraic search, carried to a field where the base is not an
//! `F_2`-subspace. The Boolean gate's subspace encoder is not needed.
//!
//! ## What the oracle charges
//!
//! - Every group operation (closure additions, sign lifting) is on the
//!   `GroupOps` ledger.
//! - The tower engines' work is reported through `solver_totals` in `F_p`
//!   multiply-adds. That unit has no pinned price, so `ecbench` leaves it
//!   out of `S` and marks the record a lower bound, as it does for the
//!   Boolean solvers' word XORs.
//! - Square roots and base lookups are counted natively (`sqrt_solves`,
//!   `lookups`).
//!
//! ## Why it lives in the binary
//!
//! For the reason `pdp3_koblitz` gives: frozen-source replays
//! rebuild only the library, and plug-ins resolved by name keep both
//! builds compiling.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crypto_lib::cryptanalysis::ecbench::methods::PrimePlugins;
use crypto_lib::cryptanalysis::f4_fp_tower::{self, RPoly, SquareRule, TowerF4Options, TowerRing};
use crypto_lib::cryptanalysis::ic_boundary::{
    lift_abscissae, semaev_s3_quadratic, CountedGroup, FactorBase, GroupOps, OracleCounters,
    PrimeCurve, PrimeInstance, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::stages::{
    DecompositionOracle, FactorBaseBuilder, InstanceCtx, Params, SolverCost, SolverTotals,
    SystemShape,
};
use crypto_lib::cryptanalysis::sig_fp_tower::{self, SigOptions};

/// The unit of the tower engines' count.
pub const MULADD_UNIT: &str = "F_p multiply-adds (tower F4 elimination)";

const BASE_NAME: &str = "prime-tower";
const ORACLE_NAME: &str = "pkm-tower";

// ── Field arithmetic ───────────────────────────────────────────────

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn addm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + b as u128) % p as u128) as u64
}

fn subm(a: u64, b: u64, p: u64) -> u64 {
    addm(a, p - b % p, p)
}

fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}

// ── The Kummer tower ───────────────────────────────────────────────

/// `V = { g·ζ^k : k < 2^t }`.
#[derive(Clone, Debug)]
struct Kummer {
    t: u32,
    g: u64,
    /// `V`, in order of `k`.
    v: Vec<u64>,
}

impl Kummer {
    /// The tower for `(p, t, seed)`; the same inputs give the same tower in
    /// the base and in the oracle.
    fn new(p: u64, t: u32, seed: u64) -> Result<Self, String> {
        if !(1..=20).contains(&t) {
            return Err(format!("tower height t = {t} outside 1..=20"));
        }
        if !(p - 1).is_multiple_of(1u64 << t) {
            return Err(format!(
                "2^{t} does not divide p − 1 = {}: a Kummer tower of height {t} needs it",
                p - 1
            ));
        }
        let mut rng = StdRng::seed_from_u64(seed ^ 0x504b_4d54_4f57_4552);
        let zeta = loop {
            let h = rng.gen_range(2..p);
            // A non-residue's (p−1)/2^t power has exact order 2^t.
            if powm(h, (p - 1) / 2, p) == p - 1 {
                break powm(h, (p - 1) >> t, p);
            }
        };
        let g = rng.gen_range(2..p);
        let v = (0..1u64 << t)
            .map(|k| mulm(g, powm(zeta, k, p), p))
            .collect();
        Ok(Kummer { t, g, v })
    }

    /// The square rules of the tower ring: block `b`'s variables are
    /// `b·t .. (b+1)·t`, `y_{j+1} = y_j²` below the top, `y_{t−1}² = 1`.
    fn ring(&self, p: u64) -> TowerRing {
        let t = self.t as usize;
        let mut rules = Vec::with_capacity(2 * t);
        for block in 0..2 {
            for j in 0..t {
                let top = j + 1 == t;
                rules.push(SquareRule {
                    next: (!top).then_some((block * t + j + 1) as u8),
                    a: 0,
                    b: if top { 0 } else { 1 },
                    c: 0,
                    d: if top { 1 } else { 0 },
                });
            }
        }
        TowerRing {
            p,
            rules,
            n_free: 0,
        }
    }
}

// ── Polynomials while building a system ────────────────────────────

/// A polynomial over `F_p` in `n` variables, exponent vector → coefficient.
#[derive(Clone, Debug)]
struct Pol {
    n: usize,
    terms: BTreeMap<Vec<u32>, u64>,
}

impl Pol {
    fn constant(n: usize, c: u64, p: u64) -> Self {
        let mut terms = BTreeMap::new();
        if !c.is_multiple_of(p) {
            terms.insert(vec![0; n], c % p);
        }
        Pol { n, terms }
    }

    fn var(n: usize, i: usize, scale: u64, p: u64) -> Self {
        let mut e = vec![0; n];
        e[i] = 1;
        let mut terms = BTreeMap::new();
        terms.insert(e, scale % p);
        Pol { n, terms }
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
        let mut f = Pol {
            n: self.n,
            terms: BTreeMap::new(),
        };
        for (e, c) in &self.terms {
            let v = mulm(*c, k, p);
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
        let mut f = Pol {
            n: self.n,
            terms: BTreeMap::new(),
        };
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

    fn raw(&self) -> Vec<(Vec<u32>, u64)> {
        self.terms.iter().map(|(e, c)| (e.clone(), *c)).collect()
    }
}

/// `S₃(X1, X2, X3) = (X1 − X2)²X3² − 2((X1 + X2)(X1X2 + a) + 2b)X3
/// + (X1X2 − a)² − 4b(X1 + X2)`.
fn s3(x1: &Pol, x2: &Pol, x3: &Pol, curve: &PrimeCurve) -> Pol {
    let (p, a, b) = (curve.p, curve.a, curve.b);
    let n = x1.n;
    let d = x1.sub(x2, p);
    let t1 = d.mul(&d, p).mul(x3, p).mul(x3, p);
    let s = x1.add(x2, p);
    let pr = x1.mul(x2, p);
    let inner = s
        .mul(&pr.add(&Pol::constant(n, a, p), p), p)
        .add(&Pol::constant(n, mulm(2, b, p), p), p);
    let t2 = inner.mul(x3, p).scale(p - 2, p);
    let q = pr.sub(&Pol::constant(n, a, p), p);
    let t3 = q.mul(&q, p).sub(&s.scale(mulm(4, b, p), p), p);
    t1.add(&t2, p).add(&t3, p)
}

// ── The base ───────────────────────────────────────────────────────

/// `prime-tower`: the points above a Kummer tower's abscissae.
pub struct PrimeTowerBase<'i> {
    instance: &'i PrimeInstance,
}

fn tower_params(params: &Params) -> Result<(u32, u64), String> {
    Ok((params.u64("t")? as u32, params.u64_or("seed", 1)?))
}

impl FactorBaseBuilder<PrimeCurve> for PrimeTowerBase<'_> {
    fn name(&self) -> &str {
        BASE_NAME
    }

    fn describe(&self, params: &Params) -> String {
        format!(
            "points above a Kummer tower V = g·μ_(2^t), t = {}, seed {}; negation fold",
            params.get("t").unwrap_or("?"),
            params.get("seed").unwrap_or("1")
        )
    }

    fn parameters(&self) -> &[(&str, &str)] {
        &[
            ("t", "tower height: |V| = 2^t abscissae, needs 2^t | p − 1"),
            ("seed", "draws ζ and the coset representative g (default 1)"),
        ]
    }

    fn build(
        &self,
        _ctx: &InstanceCtx<PrimeCurve>,
        params: &Params,
        _ops: &mut GroupOps,
    ) -> Result<FactorBase<PrimePoint>, String> {
        let start = Instant::now();
        let (t, seed) = tower_params(params)?;
        let curve = &self.instance.curve;
        let r = self.instance.r;
        let tower = Kummer::new(curve.p, t, seed)?;
        let mut xs: Vec<u64> = tower.v.clone();
        xs.sort_unstable();
        let (mut points, mut col_of, mut coef_of) = (Vec::new(), Vec::new(), Vec::new());
        let mut columns = 0usize;
        let mut sqrts = 0u64;
        for &x in &xs {
            let rhs = curve.rhs(x);
            if curve.legendre(rhs) < 0 {
                continue;
            }
            sqrts += 1;
            let Some(y) = curve.sqrt(rhs) else { continue };
            let y = y.min(curve.p - y);
            let pt = PrimePoint::affine(x, y);
            points.push(pt);
            col_of.push(columns);
            coef_of.push(1);
            if y != 0 {
                points.push(curve.neg(pt));
                col_of.push(columns);
                coef_of.push(r - 1);
            }
            columns += 1;
        }
        let mut fb = FactorBase::from_column_map(
            format!("Kummer tower t = {t}, seed {seed}: {columns} abscissae of 2^{t}"),
            points,
            col_of,
            coef_of,
            columns,
            |p| curve.key(p),
            |p| curve.key(&curve.neg(*p)),
            |p| p.x,
        )?;
        fb.cost.wall_ns = start.elapsed().as_nanos() as u64;
        fb.cost.count("abscissae_scanned", xs.len() as u64);
        fb.cost.count("legendre_symbols", xs.len() as u64);
        fb.cost.count("sqrt_solves", sqrts);
        Ok(fb)
    }
}

// ── The oracle ─────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Engine {
    F4,
    SigF5,
    F6,
    Enum,
}

impl Engine {
    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "f4" => Ok(Engine::F4),
            "sig-f5" => Ok(Engine::SigF5),
            "f6" => Ok(Engine::F6),
            "enum" => Ok(Engine::Enum),
            other => Err(format!(
                "pkm-tower engine `{other}`: use f4, sig-f5, f6 or enum"
            )),
        }
    }
}

/// What one decomposition's search did, summed over the run.
#[derive(Default)]
struct Tally {
    nodes: u64,
    refuted: u64,
    leaves: u64,
    closures: u64,
    closure_candidates: u64,
    geometric_group_additions: u64,
    quadratic_solves: u64,
    tower_sqrts: u64,
}

pub struct PkmTowerOracle<'i> {
    instance: &'i PrimeInstance,
    summands: u32,
    engine: Engine,
    max_degree: u32,
    cap: u64,
    budget: Option<Duration>,
    tower: Option<Kummer>,
    ring: Option<TowerRing>,
    totals: SolverTotals,
    tally: Tally,
    shape: Option<SystemShape>,
    /// A one-thread pool the tower engines run in: `f4_fp_tower` sizes its
    /// elimination chunks by `rayon::current_num_threads()`, so its counted
    /// multiply-adds would otherwise depend on the CPUs a run is given, and
    /// a replay on another CPU set would not reproduce them.
    pool: Option<rayon::ThreadPool>,
}

impl<'i> PkmTowerOracle<'i> {
    fn new(summands: u32, instance: &'i PrimeInstance) -> Self {
        PkmTowerOracle {
            instance,
            summands,
            engine: Engine::F4,
            max_degree: 20,
            cap: 1,
            budget: None,
            tower: None,
            ring: None,
            totals: SolverTotals::default(),
            tally: Tally::default(),
            shape: None,
            pool: None,
        }
    }

    fn curve(&self) -> &PrimeCurve {
        &self.instance.curve
    }

    /// `S₃(g·y_{1,0}, g·y_{2,0}, x_R)` in the tower ring, plus the node's
    /// constraint `y_{1,level} = value` when there is one.
    fn system(&self, x_r: u64, constraint: Option<(usize, u64)>) -> Vec<RPoly> {
        let tower = self.tower.as_ref().expect("prepared");
        let ring = self.ring.as_ref().expect("prepared");
        let p = self.curve().p;
        let t = tower.t as usize;
        let n = 2 * t;
        let x1 = Pol::var(n, 0, tower.g, p);
        let x2 = Pol::var(n, t, tower.g, p);
        let xr = Pol::constant(n, x_r, p);
        // The constraint goes first: under position over term the
        // signature engine finishes each generator's ideal before it
        // looks at the next, and S₃'s alone does not finish below degree
        // 14 at t = 6, so a constraint listed second is never used.
        let mut input = Vec::with_capacity(2);
        if let Some((level, value)) = constraint {
            let c = Pol::var(n, level, 1, p).sub(&Pol::constant(n, value, p), p);
            input.push(ring.from_raw(&c.raw()));
        }
        input.push(ring.from_raw(&s3(&x1, &x2, &xr, self.curve()).raw()));
        input.retain(|f| !f.is_empty());
        input
    }

    /// Decide the node's system; `true` when the engine proved it
    /// inconsistent.
    fn refutes(&mut self, x_r: u64, constraint: Option<(usize, u64)>) -> bool {
        let input = self.system(x_r, constraint);
        let ring = self.ring.as_ref().expect("prepared");
        let mut opts = TowerF4Options::new(self.max_degree);
        if let Some(b) = self.budget {
            opts = opts.with_budget(b);
        }
        let engine = self.engine;
        let solve = || match engine {
            Engine::SigF5 => {
                sig_fp_tower::sig_f4_tower_with(&input, ring, &opts, SigOptions::default()).report
            }
            _ => f4_fp_tower::f4_tower(&input, ring, &opts),
        };
        let start = Instant::now();
        let rep = match &self.pool {
            Some(pool) => pool.install(solve),
            None => solve(),
        };
        let wall_ns = start.elapsed().as_nanos() as u64;
        let shape = SystemShape {
            n_vars: ring.rules.len(),
            n_equations: input.len(),
            degrees: input.iter().map(RPoly::degree).collect(),
            semi_regular_degree: None,
        };
        let stopped = rep.timed_out || rep.oversize;
        let mut extra = BTreeMap::new();
        extra.insert("steps".into(), rep.steps as u64);
        extra.insert("max_cols".into(), rep.max_cols as u64);
        let cost = SolverCost {
            ops: rep.muladds,
            op_unit: MULADD_UNIT.into(),
            wall_ns,
            peak_bytes: 0,
            degree_reached: Some(rep.degree_reached),
            solving_degree: Some(rep.solving_degree_max),
            timed_out: stopped,
            extra,
        };
        self.totals.absorb(&shape, Some(&cost), stopped);
        self.shape = Some(shape);
        self.tally.nodes += 1;
        let refuted = rep.inconsistent && !stopped;
        self.tally.refuted += u64::from(refuted);
        refuted
    }

    /// The abscissae `x_1 = g·w` with `w^(2^k) = v`: the candidates of a
    /// node `k` levels above the bottom whose fixed level holds `v`.
    fn candidates(&mut self, k: u32, v: u64) -> Vec<u64> {
        let p = self.curve().p;
        let mut level = vec![v];
        for _ in 0..k {
            let mut next = Vec::with_capacity(level.len() * 2);
            for &u in &level {
                self.tally.tower_sqrts += 1;
                if let Some(w) = self.curve().sqrt(u) {
                    next.push(w);
                    if w != 0 {
                        next.push(p - w);
                    }
                }
            }
            level = next;
        }
        let g = self.tower.as_ref().expect("prepared").g;
        level.into_iter().map(|w| mulm(g, w, p)).collect()
    }

    /// Close a node geometrically: for every candidate `x_1` above a base
    /// point `P_1`, look `R ∓ P_1` up in the base.
    fn close(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        xs: &[u64],
        point: PrimePoint,
    ) -> Option<Vec<usize>> {
        self.tally.closures += 1;
        for &x1 in xs {
            self.tally.closure_candidates += 1;
            counters.lookups += 1;
            let Some(above) = fb.x_index.get(&x1) else {
                continue;
            };
            let i = above[0];
            let p1 = fb.points[i];
            for (sign_i, q) in [
                (i, ctx.group.add(ops, point, ctx.group.neg(p1))),
                (fb.neg_index[i], ctx.group.add(ops, point, p1)),
            ] {
                self.tally.geometric_group_additions += 1;
                counters.lookups += 1;
                if let Some(j) = fb.index_of_key(ctx.group.key(&q)) {
                    return Some(vec![sign_i, j]);
                }
            }
        }
        None
    }

    /// A leaf: `x_1` is fixed, `S₃(x_1, X, x_R)` is a quadratic in `X`.
    fn leaf(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        x1: u64,
        point: PrimePoint,
    ) -> Option<Vec<usize>> {
        self.tally.leaves += 1;
        counters.lookups += 1;
        fb.x_index.get(&x1)?;
        let inst = self.instance;
        let curve = &inst.curve;
        let p = curve.p;
        let (qa, qb, qc) = semaev_s3_quadratic(curve, x1, point.x);
        self.tally.quadratic_solves += 1;
        let roots: Vec<u64> = if qa == 0 {
            if qb == 0 {
                Vec::new()
            } else {
                vec![mulm(subm(0, qc, p), powm(qb, p - 2, p), p)]
            }
        } else {
            let disc = subm(mulm(qb, qb, p), mulm(4, mulm(qa, qc, p), p), p);
            counters.sqrt_solves += 1;
            match curve.sqrt(disc) {
                None => Vec::new(),
                Some(s) => {
                    let inv = powm(mulm(2, qa, p), p - 2, p);
                    let r1 = mulm(subm(s, qb, p), inv, p);
                    let r2 = mulm(subm(subm(0, s, p), qb, p), inv, p);
                    if r1 == r2 {
                        vec![r1]
                    } else {
                        vec![r1, r2]
                    }
                }
            }
        };
        for x2 in roots {
            counters.lookups += 1;
            if fb.x_index.contains_key(&x2) {
                if let Some(found) = lift_abscissae(ctx.group, fb, ops, &[x1, x2], point) {
                    return Some(found);
                }
                counters.lift_failures += 1;
            }
        }
        None
    }

    /// Depth-first search below the node that fixes block 1's level
    /// `t − depth` to `v` (the root has no constraint).
    #[allow(clippy::too_many_arguments)]
    fn search(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: PrimePoint,
        depth: u32,
        v: u64,
    ) -> Option<Vec<usize>> {
        let t = self.tower.as_ref().expect("prepared").t;
        let below = t - depth;
        let p = self.curve().p;
        if self.engine == Engine::F6 && (1u64 << below) <= self.cap {
            let xs = if depth == 0 {
                self.tower.as_ref().expect("prepared").v.clone()
            } else {
                self.candidates(below, v)
            };
            return self.close(ctx, fb, ops, counters, &xs, point);
        }
        if depth == t {
            let g = self.tower.as_ref().expect("prepared").g;
            return self.leaf(ctx, fb, ops, counters, mulm(g, v, p), point);
        }
        let constraint = (depth > 0).then(|| ((t - depth) as usize, v));
        if self.refutes(point.x, constraint) {
            return None;
        }
        // Children: the square roots of v (the top level's are ±1).
        let roots = if depth == 0 {
            vec![1, p - 1]
        } else {
            self.tally.tower_sqrts += 1;
            match self.curve().sqrt(v) {
                Some(w) if w != 0 => vec![w, p - w],
                Some(w) => vec![w],
                None => Vec::new(),
            }
        };
        for w in roots {
            if let Some(found) = self.search(ctx, fb, ops, counters, point, depth + 1, w) {
                return Some(found);
            }
        }
        None
    }
}

impl DecompositionOracle<PrimeCurve> for PkmTowerOracle<'_> {
    fn name(&self) -> &str {
        ORACLE_NAME
    }

    fn summands(&self) -> u32 {
        self.summands
    }

    fn describe(&self, params: &Params) -> String {
        format!(
            "S₃ on a Kummer tower (t = {}), engine {}: splitting tree over block 1's levels",
            params.get("t").unwrap_or("?"),
            params.get("engine").unwrap_or("f4")
        )
    }

    fn parameters(&self) -> &[(&str, &str)] {
        &[
            ("t", "tower height, as the base's"),
            ("seed", "tower seed, as the base's (default 1)"),
            ("engine", "f4, sig-f5, f6 or enum (default f4)"),
            ("degree", "tower engines' degree cap (default 20)"),
            (
                "cap",
                "f6: close a node with at most this many candidates (default 4)",
            ),
            (
                "budget_ms",
                "per-node solver deadline; 0 = none (default 0)",
            ),
        ]
    }

    fn prepare(
        &mut self,
        _ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        params: &Params,
        _ops: &mut GroupOps,
    ) -> Result<(), String> {
        if self.summands != 2 {
            return Err(format!(
                "pkm-tower decomposes into m = 2 points, not {}",
                self.summands
            ));
        }
        self.engine = Engine::parse(params.get("engine").unwrap_or("f4"))?;
        self.max_degree = params.u64_or("degree", 20)? as u32;
        self.cap = match self.engine {
            Engine::F6 => params.u64_or("cap", 4)?,
            Engine::Enum => u64::MAX,
            _ => 0,
        };
        if self.engine == Engine::Enum {
            self.engine = Engine::F6;
        }
        let ms = params.u64_or("budget_ms", 0)?;
        self.budget = (ms > 0).then(|| Duration::from_millis(ms));
        let (t, seed) = tower_params(params)?;
        let p = self.curve().p;
        let tower = Kummer::new(p, t, seed)?;
        // The base must be this tower's: every abscissa of it in V.
        let mut v = tower.v.clone();
        v.sort_unstable();
        if let Some(x) = fb
            .abscissa_list
            .iter()
            .find(|x| v.binary_search(x).is_err())
        {
            return Err(format!(
                "base abscissa {x} is not in the oracle's tower (t = {t}, seed {seed}): use prime-tower with the same t and seed"
            ));
        }
        let ring = tower.ring(p);
        // Check the rules: every element of V satisfies them.
        let tt = t as usize;
        for &x in &tower.v {
            let mut y = vec![0u64; 2 * tt];
            y[0] = mulm(x, powm(tower.g, p - 2, p), p);
            for j in 0..tt - 1 {
                y[j + 1] = mulm(y[j], y[j], p);
            }
            if mulm(y[tt - 1], y[tt - 1], p) != 1 {
                return Err(format!("tower check failed at x = {x}"));
            }
        }
        self.tower = Some(tower);
        self.ring = Some(ring);
        self.pool = Some(
            rayon::ThreadPoolBuilder::new()
                .num_threads(1)
                .build()
                .map_err(|e| format!("tower engine pool: {e}"))?,
        );
        self.totals = SolverTotals {
            solver: format!(
                "pkm-tower {} (degree {})",
                params.get("engine").unwrap_or("f4"),
                self.max_degree
            ),
            op_unit: MULADD_UNIT.into(),
            ..SolverTotals::default()
        };
        Ok(())
    }

    fn decompose(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: PrimePoint,
    ) -> Option<Vec<usize>> {
        if point.infinity {
            return None;
        }
        self.search(ctx, fb, ops, counters, point, 0, 1)
    }

    fn last_system(&self) -> Option<SystemShape> {
        self.shape.clone()
    }

    fn solver_totals(&self) -> Option<SolverTotals> {
        let mut t = self.totals.clone();
        let x = &self.tally;
        for (k, v) in [
            ("nodes", x.nodes),
            ("refuted", x.refuted),
            ("leaves", x.leaves),
            ("closures", x.closures),
            ("closure_candidates", x.closure_candidates),
            ("geometric_group_additions", x.geometric_group_additions),
            ("quadratic_solves", x.quadratic_solves),
            ("tower_sqrts", x.tower_sqrts),
        ] {
            t.extra.insert(k.into(), v);
        }
        Some(t)
    }
}

pub fn plugins() -> PrimePlugins {
    PrimePlugins {
        base: |name, instance| {
            (name == BASE_NAME).then(|| {
                Box::new(PrimeTowerBase { instance }) as Box<dyn FactorBaseBuilder<PrimeCurve> + '_>
            })
        },
        oracle: |name, summands, instance| {
            (name == ORACLE_NAME).then(|| {
                Box::new(PkmTowerOracle::new(summands, instance))
                    as Box<dyn DecompositionOracle<PrimeCurve> + '_>
            })
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::ic_boundary::find_prime_order_curve;

    /// Both engines on the root systems of a few targets: they must agree
    /// on which are inconsistent.
    #[test]
    #[ignore]
    fn sig_and_f4_agree_on_root_systems() {
        let inst = find_prime_order_curve(11, 20);
        let mut o = PkmTowerOracle::new(2, &inst);
        let p = inst.curve.p;
        let tower = Kummer::new(p, 6, 1).unwrap();
        o.ring = Some(tower.ring(p));
        o.tower = Some(tower);
        for (x_r, c) in [
            (5u64, None),
            (5, Some((5usize, 1u64))),
            (5, Some((5, p - 1))),
            (99, Some((5, 1))),
            (99, Some((5, p - 1))),
            (300, Some((5, 1))),
            (300, Some((5, p - 1))),
        ] {
            let input = o.system(x_r, c);
            let ring = o.ring.as_ref().unwrap();
            for d in [8u32, 12, 20] {
                let opts = TowerF4Options::new(d);
                let f4 = f4_fp_tower::f4_tower(&input, ring, &opts);
                let sig =
                    sig_fp_tower::sig_f4_tower_with(&input, ring, &opts, SigOptions::default());
                eprintln!(
                    "x_R {x_r} {c:?} cap {d}: f4 inc {} D {} reached {} | sig inc {} D {} reached {} steps {} timed {} over {} basis {}",
                    f4.inconsistent, f4.solving_degree_max, f4.degree_reached,
                    sig.report.inconsistent, sig.report.solving_degree_max, sig.report.degree_reached,
                    sig.report.steps, sig.report.timed_out, sig.report.oversize, sig.report.basis.len()
                );
            }
        }
    }

    /// The counted work must not depend on the threads the caller has:
    /// `f4_fp_tower` alone does (its chunks scale with the pool), and the
    /// oracle, which runs it in its own one-thread pool, must not.
    #[test]
    fn counted_work_is_independent_of_the_callers_threads() {
        let inst = find_prime_order_curve(11, 20);
        let mut base_ops = GroupOps::default();
        let ctx = InstanceCtx {
            group: &inst.curve,
            generator: inst.generator_point(),
            target: inst.generator_point(),
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: "test".into(),
            field_degree: None,
        };
        let params = Params::parse_spec("x:t=6,seed=1,engine=f4").unwrap().1;
        let fb = PrimeTowerBase { instance: &inst }
            .build(&ctx, &params, &mut base_ops)
            .unwrap();
        let run = |threads: usize| -> u64 {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                let mut o = PkmTowerOracle::new(2, &inst);
                o.prepare(&ctx, &fb, &params, &mut GroupOps::default())
                    .unwrap();
                // Whole decompositions: deep nodes reduce enough rows for
                // the chunking to matter (the shallow ones do not).
                let mut ops = GroupOps::default();
                let mut counters = OracleCounters::default();
                let mut pt = inst.generator_point();
                for _ in 0..6 {
                    let _ = o.decompose(&ctx, &fb, &mut ops, &mut counters, pt);
                    pt = inst.curve.add(&mut ops, pt, pt);
                }
                o.totals.ops
            })
        };
        assert_eq!(
            run(1),
            run(4),
            "the oracle's count depends on the caller's threads"
        );
    }

    #[test]
    fn kummer_tower_has_2t_distinct_roots() {
        let p = 7681; // 7681 − 1 = 2^9 · 15
        let k = Kummer::new(p, 5, 3).unwrap();
        let mut v = k.v.clone();
        v.sort_unstable();
        v.dedup();
        assert_eq!(v.len(), 32);
        for &x in &k.v {
            let y0 = mulm(x, powm(k.g, p - 2, p), p);
            assert_eq!(powm(y0, 32, p), 1);
        }
        assert!(Kummer::new(p, 10, 1).is_err());
    }
}
