//! Follow-ups F2 (guess-and-solve floors) and F4 (amortising across targets) of
//! `research/rr_directions_20261115/PREREG_FOLLOWUPS.md`, on the `norm` arm of
//! `examples/rr_quadratic_panel.rs`, with the repository CDCL solver
//! (`cryptanalysis::sat::Solver`, native XOR) and its assumptions API
//! (`Solver::solve_with_assumptions`).
//!
//! The curve, draw, norm-arm builder, Tseitin/XOR encoding, ground truth and decoder are
//! copied from the panel (not imported), with the same seed scheme
//! `seed ^ (m << 40) ^ (ℓ << 32)` and the same draw order, so draw `d` of a cell here is
//! draw `d` there when `--planted/--unplanted` are the panel's (default 4/4).  Draws not
//! listed in `--draws` are drawn (the RNG stream is unchanged) and skipped.
//!
//! Variables of the norm system: `u_{i,k}` (`x_i = Σ_k u_{i,k} v_k`, bits `iℓ + k`), then
//! `a`, `b` (`m` bits each).  Solver variable `j + 1` is system bit `j`; Tseitin variables
//! follow, one per distinct monomial of degree ≥ 2.
//!
//! `guess`: for each draw and `k = 0 … min(2ℓ, --max-k 12)`, every assignment of the
//! first `k` system bits (branch `b`: bit `j` of `b` is the value of system bit `j`,
//! branches in increasing `b`), solved
//!   * `warm`: one solver per (draw, k), branches as assumption sets, learnt clauses kept;
//!   * `cold`: a fresh solver per branch, the `k` bits as unit clauses.
//! The deadline `--budget-s` covers all branches of one (draw, k, mode); branches left
//! when it expires are `undecided`.  Every SAT model is decoded and checked by group
//! arithmetic; the expected verdict of every branch is computed from the ground truth
//! (the ordered triples of the factor base plus `(0, 1)` summing to `R`, as subspace
//! coordinates), and the warm and cold branch verdict strings are recorded.
//!
//! `targets`: for each draw's `V`, ONE norm system with `x_R`, `y_R` as `2m` further
//! variables (bits `3ℓ + 2m …`, `3ℓ + 3m …`; `c = x_R² + a x_R + b (x_R + y_R)` and the
//! symmetric functions with `x_R` symbolic; Boolean degree 4 through `e4 = x_0 x_1 x_2 x_R`).
//! The builder is checked against the panel's constant-target system on `--check-targets
//! 20` random curve points by substituting the `x_R`, `y_R` bits and comparing the two
//! ordered, zero-free polynomial lists.  Then `--targets 64` targets per `V` (the first
//! `round(targets · --planted-share)` planted as in the panel, the rest `[4]P` with `P`
//! uniform; both redrawn while `R = O` or `x_R ∈ V`; all uniform when `V` holds fewer
//! than three abscissas of curve points, so that nothing can be planted), drawn from
//! `StdRng::seed_from_u64(cell_seed ^ 0x7A52_6574 ^ (d << 16))`, solved
//!   * `warm`: one solver on the symbolic system, the `2m` target bits as assumptions;
//!   * `cold`: a fresh solver per target on the panel's constant-target system;
//!   * `cold_sym`: a fresh solver per target on the symbolic system (warm's encoding),
//!     the `2m` target bits as unit clauses.
//! Summary ratios `ratio_<a>_over_<b>_{conflicts,decisions,wall}` for cold/warm,
//! cold_sym/warm (incrementality alone) and cold/cold_sym (encoding size alone).
//! `--budget-s` is per target per mode.  `wall_s` is loading the solver (cold, cold_sym) plus the
//! solve; cold's building and encoding of the constant system is `build_encode_s`; the
//! warm solver is built and loaded once per `V` (`warm_load_s` in the summary line).
//!
//!     rr_sat_amortise guess   --out DIR [--m 11] [--ell (m+2)/3] [--planted 4] [--unplanted 4]
//!         [--draws 0,1,..] [--seed 20261115] [--budget-s 600] [--max-k 12] [--modes warm,cold]
//!     rr_sat_amortise targets --out DIR [--m 11] [--ell ..] [--planted 4] [--unplanted 4]
//!         [--draws ..] [--seed ..] [--budget-s 600] [--targets 64] [--planted-share 0.5]
//!         [--check-targets 20] [--modes warm,cold,cold_sym]
//!     rr_sat_amortise draws   --m 9 [--ell ..] [--planted 4] [--unplanted 4] [--seed ..]
//!
//! JSON lines are appended to `DIR/results.jsonl` (field `kind`); a progress line per
//! record goes to stdout.  `draws` prints the draw records only (no solving, no file).
use crypto_lib::binary_ecc::curve::{point_add, point_neg, scalar_mul};
use crypto_lib::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_bench::random_subspace_basis;
use crypto_lib::cryptanalysis::koblitz_groebner::{FieldStructure, SymElement};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    find_irreducible_sparse, koblitz_point_count, points_with_x, KoblitzCurve,
};
use crypto_lib::cryptanalysis::pq_groebner_f2::{bit, F2BoolPoly, Mask, MASK_BITS};
use crypto_lib::cryptanalysis::sat::{Lit, SolveResult, Solver};
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::time::{Duration, Instant};

// ───────────────────────── copied from rr_quadratic_panel ─────────────────────────

fn word(e: &F2mElement) -> u64 {
    e.raw_bits().first().copied().unwrap_or(0)
}

fn elem(w: u64, m: u32) -> F2mElement {
    F2mElement::from_words(&[w], m)
}

fn pt_words(p: &BinaryPoint) -> Option<(u64, u64)> {
    match p {
        BinaryPoint::Infinity => None,
        BinaryPoint::Affine { x, y } => Some((word(x), word(y))),
    }
}

fn make_curve(m: u32) -> (BinaryCurve, &'static str) {
    if let Some(kc) = KoblitzCurve::new(0, m) {
        return (kc.curve, "KoblitzCurve::new");
    }
    let irreducible = find_irreducible_sparse(m).expect("irreducible polynomial");
    (
        BinaryCurve {
            m,
            irreducible,
            a: F2mElement::zero(m),
            b: F2mElement::one(m),
            generator: BinaryPoint::Infinity,
            order: koblitz_point_count(0, m),
            cofactor: BigUint::from(1u32),
        },
        "shell (KoblitzCurve::new refused)",
    )
}

/// `span[i]` = XOR of `basis[j]` over the set bits `j` of `i`.
fn span_words(basis: &[F2mElement]) -> Vec<u64> {
    let mut out = vec![0u64];
    for b in basis {
        let w = word(b);
        let more: Vec<u64> = out.iter().map(|v| v ^ w).collect();
        out.extend(more);
    }
    out
}

fn factor_base(curve: &BinaryCurve, span: &[u64]) -> Vec<BinaryPoint> {
    let m = curve.m;
    span.iter()
        .filter(|&&x| x != 0)
        .flat_map(|&x| points_with_x(curve, &elem(x, m)))
        .collect()
}

struct Draw {
    basis: Vec<F2mElement>,
    span: Vec<u64>,
    target: BinaryPoint,
}

/// A planted target over `span`: three distinct base abscissas, a random sign each.
fn planted_target(curve: &BinaryCurve, base: &[BinaryPoint], rng: &mut StdRng) -> Option<BinaryPoint> {
    let m = curve.m;
    let mut xs: Vec<u64> = base.iter().filter_map(|p| pt_words(p).map(|w| w.0)).collect();
    xs.sort_unstable();
    xs.dedup();
    if xs.len() < 3 {
        return None;
    }
    let mut idx: Vec<usize> = Vec::new();
    while idx.len() < 3 {
        let i = rng.gen_range(0..xs.len());
        if !idx.contains(&i) {
            idx.push(i);
        }
    }
    let mut acc = BinaryPoint::Infinity;
    for &i in &idx {
        let pts = points_with_x(curve, &elem(xs[i], m));
        let p = if rng.gen::<bool>() {
            point_neg(&pts[0])
        } else {
            pts[0].clone()
        };
        acc = point_add(curve, &acc, &p);
    }
    Some(acc)
}

/// `[4]P`, `P` the first point over a uniform abscissa in `[1, 2^m)` on the curve.
fn unplanted_target(curve: &BinaryCurve, rng: &mut StdRng) -> BinaryPoint {
    let m = curve.m;
    let p = loop {
        let x = rng.gen_range(1u64..(1u64 << m));
        let pts = points_with_x(curve, &elem(x, m));
        if let Some(p) = pts.into_iter().next() {
            break p;
        }
    };
    scalar_mul(curve, &p, &BigUint::from(4u32))
}

/// The panel's `draw_instance`, the same RNG calls in the same order.
fn draw_instance(curve: &BinaryCurve, ell: usize, rng: &mut StdRng, planted: bool) -> Draw {
    let m = curve.m;
    for _ in 0..10000 {
        let basis = random_subspace_basis(m, ell, rng);
        let span = span_words(&basis);
        let base = factor_base(curve, &span);
        let target = if planted {
            match planted_target(curve, &base, rng) {
                Some(t) => t,
                None => continue,
            }
        } else {
            unplanted_target(curve, rng)
        };
        match pt_words(&target) {
            None => continue,
            Some((xr, _)) if span.contains(&xr) => continue,
            Some(_) => {}
        }
        return Draw {
            basis,
            span,
            target,
        };
    }
    panic!("could not draw an instance");
}

/// (ordered total, ordered with pairwise distinct abscissas, factor-base size).
fn count_truth(curve: &BinaryCurve, base: &[BinaryPoint], target: &BinaryPoint) -> (u64, u64, usize) {
    let have: HashSet<(u64, u64)> = base.iter().filter_map(pt_words).collect();
    let (mut total, mut distinct) = (0u64, 0u64);
    for p in base {
        let rp = point_add(curve, target, &point_neg(p));
        for q in base {
            let r = point_add(curve, &rp, &point_neg(q));
            if let Some(rw) = pt_words(&r) {
                if have.contains(&rw) {
                    total += 1;
                    let (xp, xq) = (pt_words(p).unwrap().0, pt_words(q).unwrap().0);
                    if xp != xq && xp != rw.0 && xq != rw.0 {
                        distinct += 1;
                    }
                }
            }
        }
    }
    (total, distinct, base.len())
}

/// The abscissa triples `(x_0, x_1, x_2)` of the ordered triples of `base` summing to
/// `target` (the ground truth behind `count_truth`, kept as triples).
fn truth_triples(curve: &BinaryCurve, base: &[BinaryPoint], target: &BinaryPoint) -> HashSet<(u64, u64, u64)> {
    let have: HashSet<(u64, u64)> = base.iter().filter_map(pt_words).collect();
    let mut out = HashSet::new();
    for p in base {
        let rp = point_add(curve, target, &point_neg(p));
        for q in base {
            let r = point_add(curve, &rp, &point_neg(q));
            if let Some(rw) = pt_words(&r) {
                if have.contains(&rw) {
                    out.insert((pt_words(p).unwrap().0, pt_words(q).unwrap().0, rw.0));
                }
            }
        }
    }
    out
}

/// The panel's `build_system(\"norm\", ..)`: `3ℓ + 2m` bits, constant target.
fn build_norm(
    curve: &BinaryCurve,
    st: &FieldStructure,
    basis: &[F2mElement],
    target: &BinaryPoint,
) -> (Vec<F2BoolPoly>, usize) {
    let m = curve.m;
    let ell = basis.len();
    let mu = m as usize;
    let n_vars = 3 * ell + 2 * mu;
    assert!(n_vars <= MASK_BITS, "n_vars {n_vars} > MASK_BITS {MASK_BITS}");
    let irr = &curve.irreducible;
    let BinaryPoint::Affine { x: xr, y: yr } = target else {
        panic!("target at infinity");
    };
    let k = |v: &F2mElement| SymElement::constant(v, m, n_vars);
    let xs: Vec<SymElement> = (0..3)
        .map(|i| SymElement::from_subspace_vars(basis, i * ell, m, n_vars))
        .collect();
    let a = SymElement::from_free_vars(3 * ell, m, n_vars);
    let b = SymElement::from_free_vars(3 * ell + mu, m, n_vars);
    let mut polys: Vec<F2BoolPoly> = Vec::new();
    let c = k(&xr.square(irr))
        .add(&a.mul_constant(xr, st))
        .add(&b.mul_constant(&xr.add(yr), st));
    let u = xs[0].add(&xs[1]);
    let v = xs[0].mul(&xs[1], st);
    let w = xs[2].add(&k(xr));
    let z = xs[2].mul_constant(xr, st);
    let e1 = u.add(&w);
    let e2 = v.add(&z).add(&u.mul(&w, st));
    let e3 = u.mul(&z, st).add(&v.mul(&w, st));
    let e4 = v.mul(&z, st);
    let n1 = b.add(&b.square(st));
    let n2 = a.square(st).add(&a.mul(&b, st));
    let n3 = b.mul(&c, st);
    let n4 = c.square(st).add(&b.square(st));
    for (e, nn) in [(e1, n1), (e2, n2), (e3, n3), (e4, n4)] {
        polys.extend(e.add(&nn).coords);
    }
    polys.retain(|p| !p.is_zero());
    (polys, n_vars)
}

/// The norm system with the target symbolic: `x_R` at bits `3ℓ + 2m ..`, `y_R` at
/// `3ℓ + 3m ..` (`3ℓ + 4m` bits).  Same construction order as [`build_norm`].
fn build_norm_symbolic(curve: &BinaryCurve, st: &FieldStructure, basis: &[F2mElement]) -> (Vec<F2BoolPoly>, usize) {
    let m = curve.m;
    let ell = basis.len();
    let mu = m as usize;
    let n_vars = 3 * ell + 4 * mu;
    assert!(n_vars <= MASK_BITS, "n_vars {n_vars} > MASK_BITS {MASK_BITS}");
    let xs: Vec<SymElement> = (0..3)
        .map(|i| SymElement::from_subspace_vars(basis, i * ell, m, n_vars))
        .collect();
    let a = SymElement::from_free_vars(3 * ell, m, n_vars);
    let b = SymElement::from_free_vars(3 * ell + mu, m, n_vars);
    let xr = SymElement::from_free_vars(3 * ell + 2 * mu, m, n_vars);
    let yr = SymElement::from_free_vars(3 * ell + 3 * mu, m, n_vars);
    let mut polys: Vec<F2BoolPoly> = Vec::new();
    // c = x_R² + a x_R + b (x_R + y_R): x_R² is linear (Frobenius), a x_R and
    // b (x_R + y_R) are of Boolean degree 2.
    let c = xr.square(st).add(&a.mul(&xr, st)).add(&b.mul(&xr.add(&yr), st));
    let u = xs[0].add(&xs[1]);
    let v = xs[0].mul(&xs[1], st);
    let w = xs[2].add(&xr);
    let z = xs[2].mul(&xr, st);
    let e1 = u.add(&w);
    let e2 = v.add(&z).add(&u.mul(&w, st));
    let e3 = u.mul(&z, st).add(&v.mul(&w, st));
    let e4 = v.mul(&z, st);
    let n1 = b.add(&b.square(st));
    let n2 = a.square(st).add(&a.mul(&b, st));
    let n3 = b.mul(&c, st);
    let n4 = c.square(st).add(&b.square(st));
    for (e, nn) in [(e1, n1), (e2, n2), (e3, n3), (e4, n4)] {
        polys.extend(e.add(&nn).coords);
    }
    polys.retain(|p| !p.is_zero());
    (polys, n_vars)
}

/// The panel's norm decoder: some choice of signs over `x_0, x_1, x_2` sums to `R`.
fn decode_ok(curve: &BinaryCurve, basis: &[F2mElement], target: &BinaryPoint, bits: &[bool]) -> bool {
    let m = curve.m;
    let ell = basis.len();
    let xs: Vec<u64> = (0..3)
        .map(|i| {
            (0..ell)
                .filter(|&k| bits[i * ell + k])
                .fold(0u64, |acc, k| acc ^ word(&basis[k]))
        })
        .collect();
    let mut cands = Vec::new();
    for &x in &xs {
        let pts = points_with_x(curve, &elem(x, m));
        if pts.is_empty() {
            return false;
        }
        cands.push(pts[0].clone());
    }
    (0..8u32).any(|signs| {
        let mut acc = BinaryPoint::Infinity;
        for (i, p) in cands.iter().enumerate() {
            let q = if signs >> i & 1 == 1 {
                point_neg(p)
            } else {
                p.clone()
            };
            acc = point_add(curve, &acc, &q);
        }
        acc == *target
    })
}

struct Encoding {
    clauses: Vec<Vec<Lit>>,
    xors: Vec<(Vec<u32>, bool)>,
    n_total: u32,
    n_tseitin: u32,
}

/// The panel's Tseitin + native-XOR encoding.
fn encode(polys: &[F2BoolPoly], n_vars: usize) -> Encoding {
    let mut mono_var: HashMap<Mask, u32> = HashMap::new();
    let mut next = n_vars as u32 + 1;
    let mut clauses = Vec::new();
    let mut xors = Vec::new();
    for p in polys {
        let mut vars = Vec::new();
        let mut rhs = false;
        for t in &p.terms {
            let deg = t.degree();
            if deg == 0 {
                rhs ^= true;
                continue;
            }
            let lits: Vec<u32> = (0..n_vars as u32)
                .filter(|&k| t.mask & bit(k) != 0)
                .map(|k| k + 1)
                .collect();
            if deg == 1 {
                vars.push(lits[0]);
                continue;
            }
            let tv = *mono_var.entry(t.mask).or_insert_with(|| {
                let tv = next;
                next += 1;
                for &l in &lits {
                    clauses.push(vec![-(tv as Lit), l as Lit]);
                }
                let mut big: Vec<Lit> = lits.iter().map(|&l| -(l as Lit)).collect();
                big.insert(0, tv as Lit);
                clauses.push(big);
                tv
            });
            vars.push(tv);
        }
        xors.push((vars, rhs));
    }
    Encoding {
        clauses,
        xors,
        n_total: next - 1,
        n_tseitin: next - 1 - n_vars as u32,
    }
}

// ───────────────────────────────── shared helpers ─────────────────────────────────

/// A solver loaded with `enc`, and whether loading already found it unsat.
fn load(enc: &Encoding) -> (Solver, bool) {
    let mut s = Solver::new(enc.n_total);
    let mut ok = true;
    for c in &enc.clauses {
        ok &= s.add_clause(c.clone());
    }
    for (vars, rhs) in &enc.xors {
        ok &= s.add_xor(vars, *rhs);
    }
    (s, ok)
}

fn status(r: SolveResult) -> &'static str {
    match r {
        SolveResult::Sat => "sat",
        SolveResult::Unsat => "unsat",
        SolveResult::Unknown => "undecided",
    }
}

fn degree(polys: &[F2BoolPoly]) -> u32 {
    polys
        .iter()
        .flat_map(|p| p.terms.iter().map(|t| t.degree()))
        .max()
        .unwrap_or(0)
}

/// The literal fixing system bit `j` to `value`.
fn lit(j: usize, value: bool) -> Lit {
    let l = (j + 1) as Lit;
    if value {
        l
    } else {
        -l
    }
}

/// Subspace coordinates (`span` index) of an abscissa.
fn coords_of(span: &[u64], x: u64) -> usize {
    span.iter().position(|&s| s == x).expect("abscissa in V")
}

struct Args(Vec<String>);

impl Args {
    fn get(&self, name: &str) -> Option<String> {
        self.0
            .iter()
            .position(|a| a == name)
            .and_then(|i| self.0.get(i + 1))
            .cloned()
    }
    fn num<T: std::str::FromStr>(&self, name: &str, d: &str) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.get(name).unwrap_or_else(|| d.to_string()).parse().expect(name)
    }
    fn list(&self, name: &str, d: &str) -> Vec<String> {
        self.get(name)
            .unwrap_or_else(|| d.to_string())
            .split(',')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    }
}

struct Cell {
    m: u32,
    ell: usize,
    curve: BinaryCurve,
    curve_source: &'static str,
    st: FieldStructure,
    cell_seed: u64,
    seed: u64,
}

/// Every draw of the cell in panel order; `(index, planted, draw)`.
fn all_draws(cell: &Cell, planted: u32, unplanted: u32) -> Vec<(u32, bool, Draw)> {
    let mut rng = StdRng::seed_from_u64(cell.cell_seed);
    (0..planted + unplanted)
        .map(|d| {
            let is_planted = d < planted;
            (d, is_planted, draw_instance(&cell.curve, cell.ell, &mut rng, is_planted))
        })
        .collect()
}

fn draw_head(cell: &Cell, d: u32, planted: bool, dr: &Draw) -> Value {
    let base = factor_base(&cell.curve, &dr.span);
    let (total, distinct, nbase) = count_truth(&cell.curve, &base, &dr.target);
    let mut base0 = base.clone();
    base0.extend(points_with_x(&cell.curve, &F2mElement::zero(cell.m)));
    let (total0, distinct0, _) = count_truth(&cell.curve, &base0, &dr.target);
    let (txr, tyr) = pt_words(&dr.target).unwrap();
    let irr_word: u64 = cell
        .curve
        .irreducible
        .low_terms
        .iter()
        .fold(1u64 << cell.m, |a, &t| a | 1 << t);
    json!({
        "m": cell.m, "ell": cell.ell, "draw": d, "planted": planted,
        "curve": "y^2+xy=x^3+1", "curve_source": cell.curve_source, "irreducible": irr_word,
        "seed": cell.seed, "cell_seed": cell.cell_seed,
        "basis": dr.basis.iter().map(word).collect::<Vec<_>>(),
        "target": [txr, tyr],
        "truth_ordered_total": total, "truth_ordered_distinct": distinct,
        "factor_base_points": nbase,
        "truth_ordered_total_incl_2torsion": total0,
        "truth_ordered_distinct_incl_2torsion": distinct0,
    })
}

// ───────────────────────────────────── guess ─────────────────────────────────────

struct BranchRun {
    conflicts: u64,
    decisions: u64,
    assumption_decisions: u64,
    propagations: u64,
    wall: f64,
    verdicts: Vec<u8>,
    n_sat: u64,
    n_unsat: u64,
    n_undecided: u64,
    first_sat: Option<u64>,
    conflicts_to_first_sat: Option<u64>,
    decisions_to_first_sat: Option<u64>,
    wall_to_first_sat: Option<f64>,
    models_verified: u64,
    models_failed: u64,
}

fn run_branches(
    mode: &str,
    enc: &Encoding,
    k: usize,
    budget: Duration,
    verify: &dyn Fn(&[bool]) -> bool,
    n_vars: usize,
) -> BranchRun {
    let t0 = Instant::now();
    let deadline = t0 + budget;
    let n_branches = 1u64 << k;
    let mut r = BranchRun {
        conflicts: 0,
        decisions: 0,
        assumption_decisions: 0,
        propagations: 0,
        wall: 0.0,
        verdicts: Vec::with_capacity(n_branches as usize),
        n_sat: 0,
        n_unsat: 0,
        n_undecided: 0,
        first_sat: None,
        conflicts_to_first_sat: None,
        decisions_to_first_sat: None,
        wall_to_first_sat: None,
        models_verified: 0,
        models_failed: 0,
    };
    let mut warm = if mode == "warm" { Some(load(enc)) } else { None };
    for br in 0..n_branches {
        let assumptions: Vec<Lit> = (0..k).map(|j| lit(j, br >> j & 1 == 1)).collect();
        let res = if Instant::now() >= deadline {
            SolveResult::Unknown
        } else if let Some((s, ok)) = warm.as_mut() {
            if !*ok {
                SolveResult::Unsat
            } else {
                s.deadline = Some(deadline);
                let before = s.stats;
                let ad0 = s.assumption_decisions();
                let res = s.solve_with_assumptions(&assumptions);
                let d = s.stats.since(&before);
                r.conflicts += d.conflicts;
                r.decisions += d.decisions;
                r.propagations += d.propagations;
                r.assumption_decisions += s.assumption_decisions() - ad0;
                if res == SolveResult::Sat {
                    let model = s.model();
                    let ok_a = assumptions.iter().all(|&l| model[(l.unsigned_abs() - 1) as usize] == (l > 0));
                    if ok_a && verify(&model[..n_vars]) {
                        r.models_verified += 1;
                    } else {
                        r.models_failed += 1;
                    }
                }
                res
            }
        } else {
            let (mut s, mut ok) = load(enc);
            for &l in &assumptions {
                ok &= s.add_clause(vec![l]);
            }
            if !ok {
                SolveResult::Unsat
            } else {
                s.deadline = Some(deadline);
                let res = s.solve();
                r.conflicts += s.stats.conflicts;
                r.decisions += s.stats.decisions;
                r.propagations += s.stats.propagations;
                if res == SolveResult::Sat {
                    let model = s.model();
                    let ok_a = assumptions.iter().all(|&l| model[(l.unsigned_abs() - 1) as usize] == (l > 0));
                    if ok_a && verify(&model[..n_vars]) {
                        r.models_verified += 1;
                    } else {
                        r.models_failed += 1;
                    }
                }
                res
            }
        };
        match res {
            SolveResult::Sat => {
                r.n_sat += 1;
                r.verdicts.push(b's');
                if r.first_sat.is_none() {
                    r.first_sat = Some(br);
                    r.conflicts_to_first_sat = Some(r.conflicts);
                    r.decisions_to_first_sat = Some(r.decisions);
                    r.wall_to_first_sat = Some(t0.elapsed().as_secs_f64());
                }
            }
            SolveResult::Unsat => {
                r.n_unsat += 1;
                r.verdicts.push(b'u');
            }
            SolveResult::Unknown => {
                r.n_undecided += 1;
                r.verdicts.push(b'?');
            }
        }
    }
    r.wall = t0.elapsed().as_secs_f64();
    r
}

fn cmd_guess(args: &Args, cell: &Cell, out: &mut std::fs::File) {
    let planted: u32 = args.num("--planted", "4");
    let unplanted: u32 = args.num("--unplanted", "4");
    let budget = Duration::from_secs_f64(args.num("--budget-s", "600"));
    let max_k: usize = args.num("--max-k", "12");
    let modes = args.list("--modes", "warm,cold");
    let only: Option<Vec<u32>> = args
        .get("--draws")
        .map(|s| s.split(',').map(|x| x.parse().expect("--draws")).collect());
    let k_top = (2 * cell.ell).min(max_k);
    for (d, is_planted, dr) in all_draws(cell, planted, unplanted) {
        if only.as_ref().is_some_and(|o| !o.contains(&d)) {
            continue;
        }
        let head = draw_head(cell, d, is_planted, &dr);
        let mut base0 = factor_base(&cell.curve, &dr.span);
        base0.extend(points_with_x(&cell.curve, &F2mElement::zero(cell.m)));
        // Ground truth as packed subspace coordinates c0 | c1 << ℓ | c2 << 2ℓ.
        let packed: Vec<u64> = truth_triples(&cell.curve, &base0, &dr.target)
            .iter()
            .map(|&(x0, x1, x2)| {
                let ell = cell.ell;
                (coords_of(&dr.span, x0) | coords_of(&dr.span, x1) << ell | coords_of(&dr.span, x2) << (2 * ell)) as u64
            })
            .collect();
        let (polys, n_vars) = build_norm(&cell.curve, &cell.st, &dr.basis, &dr.target);
        let enc = encode(&polys, n_vars);
        let mut line = head.clone();
        line["kind"] = json!("draw");
        line["mode"] = json!("guess");
        line["n_vars"] = json!(n_vars);
        line["n_polys"] = json!(polys.len());
        line["degree"] = json!(degree(&polys));
        line["n_tseitin"] = json!(enc.n_tseitin);
        line["n_solver_vars"] = json!(enc.n_total);
        line["n_clauses"] = json!(enc.clauses.len());
        line["n_xors"] = json!(enc.xors.len());
        line["truth_solution_triples"] = json!(packed.len());
        writeln!(out, "{line}").expect("write");
        out.flush().expect("flush");
        println!(
            "draw m={} l={} d={d} planted={is_planted} basis={} target={} truth_tot={} truth0_tot={} nv={n_vars} tseitin={}",
            cell.m, cell.ell, head["basis"], head["target"], head["truth_ordered_total"],
            head["truth_ordered_total_incl_2torsion"], enc.n_tseitin
        );
        let verify = |bits: &[bool]| decode_ok(&cell.curve, &dr.basis, &dr.target, bits);
        for k in 0..=k_top {
            let mask = (1u64 << k) - 1;
            let expected: Vec<u8> = (0..1u64 << k)
                .map(|br| if packed.iter().any(|&p| p & mask == br) { b's' } else { b'u' })
                .collect();
            let exp_sat = expected.iter().filter(|&&c| c == b's').count();
            let exp_first = expected.iter().position(|&c| c == b's');
            for mode in &modes {
                let r = run_branches(mode, &enc, k, budget, &verify, n_vars);
                let mismatches = r
                    .verdicts
                    .iter()
                    .zip(&expected)
                    .filter(|(g, e)| **g != b'?' && g != e)
                    .count();
                let mut line = json!({
                    "kind": "guess", "m": cell.m, "ell": cell.ell, "draw": d, "planted": is_planted,
                    "k": k, "mode": mode, "branches": 1u64 << k,
                    "time_budget_s": budget.as_secs_f64(),
                    "conflicts_total": r.conflicts, "decisions_total": r.decisions,
                    "assumption_decisions_total": r.assumption_decisions,
                    "propagations_total": r.propagations,
                    "wall_s": r.wall,
                    "n_sat": r.n_sat, "n_unsat": r.n_unsat, "n_undecided": r.n_undecided,
                    "first_sat_branch": r.first_sat,
                    "conflicts_to_first_sat": r.conflicts_to_first_sat,
                    "decisions_to_first_sat": r.decisions_to_first_sat,
                    "wall_to_first_sat_s": r.wall_to_first_sat,
                    "models_verified": r.models_verified, "models_failed": r.models_failed,
                    "expected_sat_branches": exp_sat, "expected_first_sat_branch": exp_first,
                    "verdict_mismatches": mismatches,
                    "branch_verdicts": String::from_utf8(r.verdicts.clone()).unwrap(),
                });
                line["truth_ordered_total_incl_2torsion"] = head["truth_ordered_total_incl_2torsion"].clone();
                writeln!(out, "{line}").expect("write");
                out.flush().expect("flush");
                println!(
                    "guess m={} l={} d={d} k={k} {mode} confl={} dec={} wall={:.3} sat={} unsat={} undec={} first={:?} confl_first={:?} exp_sat={exp_sat} exp_first={exp_first:?} mismatch={mismatches} ver={} fail={}",
                    cell.m, cell.ell, r.conflicts, r.decisions, r.wall, r.n_sat, r.n_unsat, r.n_undecided,
                    r.first_sat, r.conflicts_to_first_sat, r.models_verified, r.models_failed
                );
            }
        }
    }
}

// ──────────────────────────────────── targets ────────────────────────────────────

/// Polynomials as sorted term-mask lists (comparable across `n_vars`).
fn poly_key(p: &F2BoolPoly) -> Vec<Mask> {
    let mut v: Vec<Mask> = p.terms.iter().map(|t| t.mask).collect();
    v.sort_unstable();
    v
}

/// Substitute the `x_R`, `y_R` bits of the symbolic system and compare with the panel's
/// constant-target system: (ordered lists equal, multisets equal).
fn check_substitution(
    cell: &Cell,
    basis: &[F2mElement],
    sym: &[F2BoolPoly],
    target: &BinaryPoint,
) -> (bool, bool) {
    let mu = cell.m as usize;
    let ell = basis.len();
    let (xr, yr) = pt_words(target).unwrap();
    let mut subst: Vec<Vec<Mask>> = Vec::new();
    for p in sym {
        let mut q = p.clone();
        for j in 0..mu {
            q = q.substitute((3 * ell + 2 * mu + j) as u32, xr >> j & 1 == 1);
            q = q.substitute((3 * ell + 3 * mu + j) as u32, yr >> j & 1 == 1);
        }
        if !q.is_zero() {
            subst.push(poly_key(&q));
        }
    }
    let (cst, _) = build_norm(&cell.curve, &cell.st, basis, target);
    let cst: Vec<Vec<Mask>> = cst.iter().map(poly_key).collect();
    let ordered = subst == cst;
    let (mut a, mut b) = (subst, cst);
    a.sort();
    b.sort();
    (ordered, a == b)
}

fn random_point(curve: &BinaryCurve, rng: &mut StdRng) -> BinaryPoint {
    let m = curve.m;
    loop {
        let x = rng.gen_range(1u64..(1u64 << m));
        let pts = points_with_x(curve, &elem(x, m));
        if pts.is_empty() {
            continue;
        }
        let p = pts[0].clone();
        return if rng.gen::<bool>() { point_neg(&p) } else { p };
    }
}

struct TargetRun {
    status: SolveResult,
    conflicts: u64,
    decisions: u64,
    assumption_decisions: u64,
    propagations: u64,
    wall: f64,
    model_verified: Option<bool>,
}

fn target_json(r: &TargetRun) -> Value {
    json!({
        "status": status(r.status), "decided": r.status != SolveResult::Unknown,
        "conflicts": r.conflicts, "decisions": r.decisions,
        "assumption_decisions": r.assumption_decisions,
        "propagations": r.propagations, "wall_s": r.wall,
        "model_verified": r.model_verified,
    })
}

fn cmd_targets(args: &Args, cell: &Cell, out: &mut std::fs::File) {
    let planted: u32 = args.num("--planted", "4");
    let unplanted: u32 = args.num("--unplanted", "4");
    let budget = Duration::from_secs_f64(args.num("--budget-s", "600"));
    let n_targets: usize = args.num("--targets", "64");
    let share: f64 = args.num("--planted-share", "0.5");
    let n_check: usize = args.num("--check-targets", "20");
    let modes = args.list("--modes", "warm,cold,cold_sym");
    let only: Option<Vec<u32>> = args
        .get("--draws")
        .map(|s| s.split(',').map(|x| x.parse().expect("--draws")).collect());
    let n_planted_t = ((n_targets as f64) * share).round() as usize;
    let m = cell.m;
    let mu = m as usize;
    for (d, is_planted, dr) in all_draws(cell, planted, unplanted) {
        if only.as_ref().is_some_and(|o| !o.contains(&d)) {
            continue;
        }
        let ell = cell.ell;
        let head = draw_head(cell, d, is_planted, &dr);
        let (sym, n_sym) = build_norm_symbolic(&cell.curve, &cell.st, &dr.basis);
        let enc_sym = encode(&sym, n_sym);
        // Builder check on random curve points.
        let mut trng = StdRng::seed_from_u64(cell.cell_seed ^ 0x5355_4253 ^ (u64::from(d) << 16));
        let (mut ok_ordered, mut ok_multiset) = (0usize, 0usize);
        for _ in 0..n_check {
            let t = random_point(&cell.curve, &mut trng);
            let (o, s) = check_substitution(cell, &dr.basis, &sym, &t);
            ok_ordered += o as usize;
            ok_multiset += s as usize;
        }
        let mut line = head.clone();
        line["kind"] = json!("draw");
        line["mode"] = json!("targets");
        line["n_vars_symbolic"] = json!(n_sym);
        line["n_polys_symbolic"] = json!(sym.len());
        line["degree_symbolic"] = json!(degree(&sym));
        line["n_tseitin_symbolic"] = json!(enc_sym.n_tseitin);
        line["n_solver_vars_symbolic"] = json!(enc_sym.n_total);
        line["n_clauses_symbolic"] = json!(enc_sym.clauses.len());
        line["substitution_checks"] = json!(n_check);
        line["substitution_ordered_equal"] = json!(ok_ordered);
        line["substitution_multiset_equal"] = json!(ok_multiset);
        writeln!(out, "{line}").expect("write");
        out.flush().expect("flush");
        println!(
            "draw m={m} l={ell} d={d} planted={is_planted} basis={} target={} nv_sym={n_sym} deg_sym={} tseitin_sym={} subst_check ordered={ok_ordered}/{n_check} multiset={ok_multiset}/{n_check}",
            head["basis"], head["target"], degree(&sym), enc_sym.n_tseitin
        );
        assert_eq!(ok_multiset, n_check, "symbolic builder disagrees with the constant-target system");

        // Targets over this V.
        let base = factor_base(&cell.curve, &dr.span);
        let mut base0 = base.clone();
        base0.extend(points_with_x(&cell.curve, &F2mElement::zero(m)));
        let mut rng = StdRng::seed_from_u64(cell.cell_seed ^ 0x7A52_6574 ^ (u64::from(d) << 16));
        // A V with fewer than three abscissas of curve points admits no planted target;
        // its targets are then all uniform (`planted_possible: false` in the summary).
        let mut xs_on: Vec<u64> = base.iter().filter_map(|p| pt_words(p).map(|w| w.0)).collect();
        xs_on.sort_unstable();
        xs_on.dedup();
        let planted_possible = xs_on.len() >= 3;
        let n_planted_v = if planted_possible { n_planted_t } else { 0 };
        let mut targets: Vec<(bool, BinaryPoint)> = Vec::new();
        while targets.len() < n_targets {
            let pl = targets.len() < n_planted_v;
            let t = if pl {
                planted_target(&cell.curve, &base, &mut rng).expect("three base abscissas")
            } else {
                unplanted_target(&cell.curve, &mut rng)
            };
            match pt_words(&t) {
                None => continue,
                Some((xr, _)) if dr.span.contains(&xr) => continue,
                Some(_) => targets.push((pl, t)),
            }
        }
        let t_load = Instant::now();
        let mut warm = if modes.iter().any(|x| x == "warm") {
            Some(load(&enc_sym))
        } else {
            None
        };
        let warm_load_s = t_load.elapsed().as_secs_f64();
        let mut tot: HashMap<String, [f64; 6]> = HashMap::new(); // confl, dec, wall, undecided, mismatch, model_fail
        for (ti, (pl, t)) in targets.iter().enumerate() {
            let (xr, yr) = pt_words(t).unwrap();
            let (total, distinct, _) = count_truth(&cell.curve, &base, t);
            let (total0, distinct0, _) = count_truth(&cell.curve, &base0, t);
            let verify = |bits: &[bool]| decode_ok(&cell.curve, &dr.basis, t, bits);
            let mut line = json!({
                "kind": "target", "m": m, "ell": ell, "draw": d, "planted_draw": is_planted,
                "target_index": ti, "target_planted": pl, "target": [xr, yr],
                "truth_ordered_total": total, "truth_ordered_distinct": distinct,
                "truth_ordered_total_incl_2torsion": total0,
                "truth_ordered_distinct_incl_2torsion": distinct0,
                "time_budget_s": budget.as_secs_f64(),
            });
            let mut progress = format!("target m={m} l={ell} d={d} t={ti} planted={pl} truth0={total0}");
            for mode in &modes {
                let t0 = Instant::now();
                let mut build_s = 0.0f64;
                let r = if mode == "warm" {
                    let (s, ok) = warm.as_mut().unwrap();
                    let mut assumptions: Vec<Lit> = Vec::with_capacity(2 * mu);
                    for j in 0..mu {
                        assumptions.push(lit(3 * ell + 2 * mu + j, xr >> j & 1 == 1));
                    }
                    for j in 0..mu {
                        assumptions.push(lit(3 * ell + 3 * mu + j, yr >> j & 1 == 1));
                    }
                    if !*ok {
                        TargetRun {
                            status: SolveResult::Unsat,
                            conflicts: 0,
                            decisions: 0,
                            assumption_decisions: 0,
                            propagations: 0,
                            wall: t0.elapsed().as_secs_f64(),
                            model_verified: None,
                        }
                    } else {
                        s.deadline = Some(Instant::now() + budget);
                        let before = s.stats;
                        let ad0 = s.assumption_decisions();
                        let res = s.solve_with_assumptions(&assumptions);
                        let dd = s.stats.since(&before);
                        let mv = (res == SolveResult::Sat).then(|| {
                            let model = s.model();
                            assumptions.iter().all(|&l| model[(l.unsigned_abs() - 1) as usize] == (l > 0))
                                && verify(&model[..n_sym])
                        });
                        TargetRun {
                            status: res,
                            conflicts: dd.conflicts,
                            decisions: dd.decisions,
                            assumption_decisions: s.assumption_decisions() - ad0,
                            propagations: dd.propagations,
                            wall: t0.elapsed().as_secs_f64(),
                            model_verified: mv,
                        }
                    }
                } else if mode == "cold_sym" {
                    // A fresh solver on the SAME symbolic encoding as warm, the 2m target
                    // bits as unit clauses: separates incrementality from encoding size.
                    let (mut s, mut ok) = load(&enc_sym);
                    let mut units: Vec<Lit> = Vec::with_capacity(2 * mu);
                    for j in 0..mu {
                        units.push(lit(3 * ell + 2 * mu + j, xr >> j & 1 == 1));
                    }
                    for j in 0..mu {
                        units.push(lit(3 * ell + 3 * mu + j, yr >> j & 1 == 1));
                    }
                    for &l in &units {
                        ok &= s.add_clause(vec![l]);
                    }
                    let res = if ok {
                        s.deadline = Some(Instant::now() + budget);
                        s.solve()
                    } else {
                        SolveResult::Unsat
                    };
                    let wall = t0.elapsed().as_secs_f64();
                    let mv = (res == SolveResult::Sat).then(|| {
                        let model = s.model();
                        units.iter().all(|&l| model[(l.unsigned_abs() - 1) as usize] == (l > 0))
                            && verify(&model[..n_sym])
                    });
                    TargetRun {
                        status: res,
                        conflicts: s.stats.conflicts,
                        decisions: s.stats.decisions,
                        assumption_decisions: 0,
                        propagations: s.stats.propagations,
                        wall,
                        model_verified: mv,
                    }
                } else {
                    assert_eq!(mode, "cold", "unknown mode {mode} (warm | cold | cold_sym)");
                    let (polys, n_vars) = build_norm(&cell.curve, &cell.st, &dr.basis, t);
                    let enc = encode(&polys, n_vars);
                    build_s = t0.elapsed().as_secs_f64();
                    let t_solve = Instant::now();
                    let (mut s, ok) = load(&enc);
                    let res = if ok {
                        s.deadline = Some(Instant::now() + budget);
                        s.solve()
                    } else {
                        SolveResult::Unsat
                    };
                    let wall = t_solve.elapsed().as_secs_f64();
                    let mv = (res == SolveResult::Sat).then(|| verify(&s.model()[..n_vars]));
                    TargetRun {
                        status: res,
                        conflicts: s.stats.conflicts,
                        decisions: s.stats.decisions,
                        assumption_decisions: 0,
                        propagations: s.stats.propagations,
                        wall,
                        model_verified: mv,
                    }
                };
                let mismatch = r.status != SolveResult::Unknown && ((r.status == SolveResult::Sat) != (total0 > 0));
                let mut j = target_json(&r);
                j["matches_truth"] = json!(r.status != SolveResult::Unknown && !mismatch);
                if mode == "cold" {
                    // Building and encoding the constant-target system, outside `wall_s`
                    // (which is loading the solver plus solving, as for warm's solve).
                    j["build_encode_s"] = json!(build_s);
                }
                line[mode.as_str()] = j;
                let e = tot.entry(mode.clone()).or_insert([0.0; 6]);
                e[0] += r.conflicts as f64;
                e[1] += r.decisions as f64;
                e[2] += r.wall;
                e[3] += (r.status == SolveResult::Unknown) as u8 as f64;
                e[4] += mismatch as u8 as f64;
                e[5] += (r.model_verified == Some(false)) as u8 as f64;
                progress.push_str(&format!(
                    " {mode}:{} confl={} wall={:.3} ver={:?}",
                    status(r.status), r.conflicts, r.wall, r.model_verified
                ));
            }
            writeln!(out, "{line}").expect("write");
            out.flush().expect("flush");
            println!("{progress}");
        }
        let mut summary = json!({
            "kind": "targets_summary", "m": m, "ell": ell, "draw": d, "planted_draw": is_planted,
            "basis": head["basis"], "targets": n_targets, "targets_planted": n_planted_v,
            "planted_possible": planted_possible, "base_abscissas": xs_on.len(),
            "warm_load_s": warm_load_s,
        });
        for (mode, e) in &tot {
            summary[mode.as_str()] = json!({
                "conflicts_total": e[0] as u64, "decisions_total": e[1] as u64, "wall_s_total": e[2],
                "undecided": e[3] as u64, "truth_mismatches": e[4] as u64, "models_failed": e[5] as u64,
            });
        }
        // ratio_<num>_over_<den>_{conflicts,decisions,wall}: cold/warm (both effects),
        // cold_sym/warm (incrementality, same encoding), cold/cold_sym (encoding cost).
        for (num, den) in [("cold", "warm"), ("cold_sym", "warm"), ("cold", "cold_sym")] {
            if let (Some(a), Some(b)) = (tot.get(num), tot.get(den)) {
                for (i, what) in ["conflicts", "decisions", "wall"].iter().enumerate() {
                    summary[format!("ratio_{num}_over_{den}_{what}").as_str()] =
                        json!(if b[i] > 0.0 { Some(a[i] / b[i]) } else { None });
                }
            }
        }
        writeln!(out, "{summary}").expect("write");
        out.flush().expect("flush");
        println!("summary {summary}");
    }
}

fn main() {
    let raw: Vec<String> = std::env::args().collect();
    let sub = raw.get(1).cloned().unwrap_or_default();
    let args = Args(raw);
    let m: u32 = args.num("--m", "11");
    let ell: usize = args
        .get("--ell")
        .map(|s| s.parse().expect("--ell"))
        .unwrap_or(m.div_ceil(3) as usize);
    let seed: u64 = args.num("--seed", "20261115");
    let (curve, curve_source) = make_curve(m);
    let st = FieldStructure::new(m, &curve.irreducible);
    let cell = Cell {
        m,
        ell,
        cell_seed: seed ^ (u64::from(m) << 40) ^ ((ell as u64) << 32),
        seed,
        curve,
        curve_source,
        st,
    };
    if sub == "draws" {
        let planted: u32 = args.num("--planted", "4");
        let unplanted: u32 = args.num("--unplanted", "4");
        for (d, p, dr) in all_draws(&cell, planted, unplanted) {
            println!("{}", draw_head(&cell, d, p, &dr));
        }
        return;
    }
    let out_dir = std::path::PathBuf::from(args.get("--out").expect("--out DIR is required"));
    std::fs::create_dir_all(&out_dir).expect("create OUT");
    let mut out = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(out_dir.join("results.jsonl"))
        .expect("open results.jsonl");
    eprintln!("MASK_BITS = {MASK_BITS}");
    match sub.as_str() {
        "guess" => cmd_guess(&args, &cell, &mut out),
        "targets" => cmd_targets(&args, &cell, &mut out),
        other => panic!("unknown subcommand {other:?} (guess | targets | draws)"),
    }
}
