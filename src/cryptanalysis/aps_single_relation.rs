//! Bounded prime-field single-relation experiments (APS 2018, section 3).
//!
//! Known labels `R_i = u_i G + v_i Q` are generated without the target log.
//! F4 solves exact coordinate constraints; point addition checks the signs,
//! and only an informative, independently verified relation returns a log.
//! This module has no generic-DLP or log-table fallback. It is a correctness
//! instrument for small prime fields, not a performance or scaling claim.

use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Serialize;

use super::f4_fp::{self, F4Options, Ordering, Poly, Verdict};
use super::prime_fast::{FastCurve, Pt};
use super::residual_walk::{inv_mod, is_prime_u64};

/// Independent, bounded replay of the public fixture transcripts.
pub mod replay;

/// A deliberately bounded instrument: the substitution solver may enumerate F_p.
pub const MAX_FIELD: u64 = 65_521;

fn mul(a: u64, b: u64, p: u64) -> u64 {
    (u128::from(a) * u128::from(b) % u128::from(p)) as u64
}
fn neg(a: u64, p: u64) -> u64 {
    (p - a % p) % p
}

// F4's `normalise` makes a polynomial monic. That is safe for whole
// equations, but changes relative coefficients inside an expression.
fn collect_terms(terms: Poly, p: u64) -> Poly {
    let mut coefficients = BTreeMap::new();
    for (exponents, coefficient) in terms {
        let entry = coefficients.entry(exponents).or_insert(0);
        *entry = (*entry + coefficient) % p;
    }
    coefficients.into_iter().filter(|(_, c)| *c != 0).collect()
}

#[derive(Clone)]
struct Expr {
    terms: Poly,
    vars: usize,
    p: u64,
}
impl Expr {
    fn constant(c: u64, vars: usize, p: u64) -> Self {
        Self {
            terms: collect_terms(vec![(vec![0; vars], c % p)], p),
            vars,
            p,
        }
    }
    fn var(i: usize, vars: usize, p: u64) -> Self {
        let mut e = vec![0; vars];
        e[i] = 1;
        Self {
            terms: vec![(e, 1)],
            vars,
            p,
        }
    }
    fn c(&self, c: u64) -> Self {
        Self::constant(c, self.vars, self.p)
    }
    fn add(&self, rhs: &Self) -> Self {
        let mut terms = self.terms.clone();
        terms.extend(rhs.terms.clone());
        Self {
            terms: collect_terms(terms, self.p),
            vars: self.vars,
            p: self.p,
        }
    }
    fn sub(&self, rhs: &Self) -> Self {
        let minus = Self {
            terms: rhs
                .terms
                .iter()
                .map(|(e, c)| (e.clone(), neg(*c, self.p)))
                .collect(),
            vars: self.vars,
            p: self.p,
        };
        self.add(&minus)
    }
    fn mul(&self, rhs: &Self) -> Self {
        let terms: Poly = self
            .terms
            .iter()
            .flat_map(|(e, a)| {
                rhs.terms.iter().map(move |(f, b)| {
                    (
                        e.iter().zip(f).map(|(x, y)| x + y).collect(),
                        mul(*a, *b, self.p),
                    )
                })
            })
            .collect();
        Self {
            terms: collect_terms(terms, self.p),
            vars: self.vars,
            p: self.p,
        }
    }
    fn square(&self) -> Self {
        self.mul(self)
    }
}

fn s3_coefficients(x: &Expr, y: &Expr, a: u64, b: u64) -> [Expr; 3] {
    let sum = x.add(y);
    let product = x.mul(y);
    [
        product
            .sub(&x.c(a))
            .square()
            .sub(&sum.mul(&x.c(4 * b % x.p))),
        sum.mul(&product.add(&x.c(a)))
            .add(&x.c(2 * b % x.p))
            .mul(&x.c(neg(2, x.p))),
        x.sub(y).square(),
    ]
}
fn s3(x: &Expr, y: &Expr, z: &Expr, a: u64, b: u64) -> Expr {
    let [c, linear, quadratic] = s3_coefficients(x, y, a, b);
    quadratic.mul(&z.square()).add(&linear.mul(z)).add(&c)
}
fn pair_s3(sum: &Expr, product: &Expr, z: &Expr, a: u64, b: u64) -> Expr {
    sum.square()
        .sub(&product.mul(&sum.c(4)))
        .mul(&z.square())
        .sub(
            &sum.mul(&product.add(&sum.c(a)))
                .add(&sum.c(2 * b % sum.p))
                .mul(z)
                .mul(&sum.c(2)),
        )
        .add(&product.sub(&sum.c(a)).square())
        .sub(&sum.mul(&sum.c(4 * b % sum.p)))
}

fn checked_coordinates(values: &[u64], p: u64) -> Result<(), String> {
    if values.is_empty()
        || values.iter().any(|&x| x >= p)
        || values.iter().copied().collect::<BTreeSet<_>>().len() != values.len()
    {
        return Err("coordinate sets must be nonempty, canonical and squarefree".into());
    }
    Ok(())
}
fn vanish(values: &[u64], x: &Expr) -> Expr {
    values
        .iter()
        .fold(x.c(1), |f, &root| f.mul(&x.sub(&x.c(root))))
}
/// Numeric monic vanishing polynomial, ascending coefficients.
pub fn vanishing_coefficients(values: &[u64], p: u64) -> Result<Vec<u64>, String> {
    if !is_prime_u64(p) || p > MAX_FIELD {
        return Err("unsupported prime field".into());
    }
    checked_coordinates(values, p)?;
    let mut f = vec![1];
    for &root in values {
        let mut next = vec![0; f.len() + 1];
        for (i, &c) in f.iter().enumerate() {
            next[i] = (next[i] + mul(c, neg(root, p), p)) % p;
            next[i + 1] = (next[i + 1] + c) % p;
        }
        f = next;
    }
    Ok(f)
}

fn divisor_constraints(values: &[u64], elementary: &[Expr]) -> Result<Vec<Expr>, String> {
    let m = elementary.len();
    let ctx = &elementary[0];
    if values.len() < m {
        return Err("not enough distinct coordinates for the selected divisor".into());
    }
    let f = vanishing_coefficients(values, ctx.p)?;
    let mut h = vec![ctx.c(0); m + 1];
    h[m] = ctx.c(1);
    for (j, e) in elementary.iter().enumerate() {
        h[m - j - 1] = if j % 2 == 0 {
            ctx.c(0).sub(e)
        } else {
            e.clone()
        };
    }
    let mut remainder: Vec<_> = f.iter().map(|&c| ctx.c(c)).collect();
    for degree in (m..remainder.len()).rev() {
        let lead = remainder[degree].clone();
        for (j, coefficient) in h.iter().enumerate() {
            let i = degree - m + j;
            remainder[i] = remainder[i].sub(&lead.mul(coefficient));
        }
    }
    remainder.truncate(m);
    Ok(remainder)
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Encoding {
    /// S3 and one exact vanishing constraint per disjoint list.
    Partitioned3,
    /// Select an unordered cubic divisor of the entire base polynomial.
    Unordered3,
    /// Select that cubic as the intersection with y=tX+c (two parameters).
    Line3,
    /// S4 as the exact resultant of two quadratics; four disjoint lists.
    Expanded4,
    /// Two S3 equations joined by a full-field auxiliary abscissa.
    Split4,
    /// Two unordered pairs, one per disjoint block, joined through S3.
    BlockSymmetric4,
}

#[derive(Clone)]
enum Decode {
    Coordinates(usize),
    Cubic(Vec<u64>),
    Line { values: Vec<u64>, a: u64, b: u64 },
    TwoPairs(Vec<Vec<u64>>),
}
#[derive(Clone)]
pub struct System {
    variables: usize,
    equations: Vec<Poly>,
    modulus: u64,
    decode: Decode,
    /// Exact support tags constructed with the vanishing equations. `None`
    /// on every auxiliary, line or elementary-symmetric parameter.
    domains: Vec<Option<Vec<u64>>>,
    model: (u64, u64),
}

impl System {
    pub fn modulus(&self) -> u64 {
        self.modulus
    }

    pub fn variables(&self) -> usize {
        self.variables
    }

    pub fn equations(&self) -> &[Poly] {
        &self.equations
    }
}

/// Build exactly the requested encoding; unsupported arity is an error.
pub fn build_system(
    curve: &FastCurve,
    parts: &[Vec<u64>],
    encoding: Encoding,
) -> Result<System, String> {
    let p = curve.f.p;
    if p <= 3 || p > MAX_FIELD || !is_prime_u64(p) {
        return Err("unsupported prime field".into());
    }
    for part in parts {
        checked_coordinates(part, p)?;
    }
    let all: Vec<_> = parts.iter().flatten().copied().collect();
    checked_coordinates(&all, p)?;
    if all.iter().any(|&x| curve.lift_x(x).is_none()) {
        return Err("coordinate without a rational curve lift".into());
    }
    let a = curve.f.from_mont(curve.a);
    let b = curve.f.from_mont(curve.b);
    if (4 * mul(mul(a, a, p), a, p) + 27 * mul(b, b, p)).is_multiple_of(p) {
        return Err("singular curve".into());
    }
    let (vars, expected) = match encoding {
        Encoding::Partitioned3 => (3, 3),
        Encoding::Unordered3 => (3, 1),
        Encoding::Line3 => (2, 1),
        Encoding::Expanded4 => (4, 4),
        Encoding::Split4 => (5, 4),
        Encoding::BlockSymmetric4 => (5, 2),
    };
    if parts.len() != expected {
        return Err(format!(
            "encoding {encoding:?} requires {expected} coordinate blocks"
        ));
    }
    let x: Vec<_> = (0..vars).map(|i| Expr::var(i, vars, p)).collect();
    let (mut equations, decode) = match encoding {
        Encoding::Partitioned3 => (vec![s3(&x[0], &x[1], &x[2], a, b)], Decode::Coordinates(3)),
        Encoding::Unordered3 => {
            let mut equations = divisor_constraints(&all, &x)?;
            equations.push(
                x[1].sub(&x[0].c(a))
                    .square()
                    .sub(&x[0].mul(&x[2].add(&x[0].c(b))).mul(&x[0].c(4))),
            );
            (equations, Decode::Cubic(all.clone()))
        }
        Encoding::Line3 => {
            // H=X^3-t^2 X^2+(a-2tc)X+b-c^2. No division by t:
            // horizontal lines remain unless labels certify safe pruning.
            let elementary = vec![
                x[0].square(),
                x[0].c(a).sub(&x[0].mul(&x[1]).mul(&x[0].c(2))),
                x[1].square().sub(&x[0].c(b)),
            ];
            (
                divisor_constraints(&all, &elementary)?,
                Decode::Line {
                    values: all.clone(),
                    a,
                    b,
                },
            )
        }
        Encoding::Split4 => (
            vec![s3(&x[0], &x[1], &x[4], a, b), s3(&x[2], &x[3], &x[4], a, b)],
            Decode::Coordinates(4),
        ),
        Encoding::Expanded4 => {
            let [c, l, q] = s3_coefficients(&x[0], &x[1], a, b);
            let [f, e, d] = s3_coefficients(&x[2], &x[3], a, b);
            let resultant = q
                .mul(&f)
                .sub(&c.mul(&d))
                .square()
                .sub(&q.mul(&e).sub(&l.mul(&d)).mul(&l.mul(&f).sub(&c.mul(&e))));
            (vec![resultant], Decode::Coordinates(4))
        }
        Encoding::BlockSymmetric4 => {
            let mut equations = divisor_constraints(&parts[0], &x[..2])?;
            equations.extend(divisor_constraints(&parts[1], &x[2..4])?);
            equations.push(pair_s3(&x[0], &x[1], &x[4], a, b));
            equations.push(pair_s3(&x[2], &x[3], &x[4], a, b));
            (equations, Decode::TwoPairs(parts.to_vec()))
        }
    };
    if matches!(
        encoding,
        Encoding::Partitioned3 | Encoding::Expanded4 | Encoding::Split4
    ) {
        equations.extend(parts.iter().zip(&x).map(|(values, x)| vanish(values, x)));
    }
    let mut domains = vec![None; vars];
    if matches!(
        encoding,
        Encoding::Partitioned3 | Encoding::Expanded4 | Encoding::Split4
    ) {
        for (domain, values) in domains.iter_mut().zip(parts) {
            *domain = Some(values.clone());
        }
    }
    Ok(System {
        variables: vars,
        equations: equations
            .into_iter()
            .map(|f| f4_fp::normalise(&f.terms, p, Ordering::Grevlex))
            .collect(),
        modulus: p,
        decode,
        domains,
        model: (a, b),
    })
}

fn decode_root(system: &System, root: &[u64]) -> Result<Vec<u64>, String> {
    let p = system.modulus;
    let mut coordinates = match &system.decode {
        Decode::Coordinates(m) => root[..*m].to_vec(),
        Decode::Cubic(values) => divisor_roots(values, &root[..3], p)?,
        Decode::Line { values, a, b } => {
            let (t, c) = (root[0], root[1]);
            divisor_roots(
                values,
                &[
                    mul(t, t, p),
                    (a + p - mul(2 * t % p, c, p)) % p,
                    (mul(c, c, p) + p - b) % p,
                ],
                p,
            )?
        }
        Decode::TwoPairs(parts) => {
            let mut values = divisor_roots(&parts[0], &root[..2], p)?;
            values.extend(divisor_roots(&parts[1], &root[2..4], p)?);
            values
        }
    };
    coordinates.sort_unstable();
    if coordinates.windows(2).any(|w| w[0] == w[1]) {
        return Err("repeated coordinates are not permitted".into());
    }
    Ok(coordinates)
}

fn divisor_roots(values: &[u64], elementary: &[u64], p: u64) -> Result<Vec<u64>, String> {
    let roots: Vec<_> = values
        .iter()
        .copied()
        .filter(|&x| {
            let mut result = 1;
            for (i, &e) in elementary.iter().enumerate() {
                result = (mul(result, x, p) + if i % 2 == 0 { neg(e, p) } else { e }) % p;
            }
            result == 0
        })
        .collect();
    if roots.len() != elementary.len() {
        return Err("divisor did not decode to distinct rational base coordinates".into());
    }
    Ok(roots)
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RootDiagnostics {
    pub equation_roots_visited: usize,
    pub candidates_rejected: usize,
    pub domain_evaluations: u64,
    pub factored_univariates: usize,
    pub root_field_multiplications: u64,
    pub full_field_branches: u64,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AlgebraReport {
    pub complete: bool,
    /// A verified visitor requested a stop; does not imply enumeration complete.
    pub stopped_early: bool,
    pub coordinate_sets: Vec<Vec<u64>>,
    pub solver_calls: usize,
    pub solving_degree: u32,
    pub max_rows: usize,
    pub max_columns: usize,
    /// F4 row-reduction work only, not a full-cost counter.
    pub f4_field_multiplications: u64,
    pub timed_out: bool,
    /// Absent for the legacy scan: its root-stage work is not instrumented.
    pub roots: Option<RootDiagnostics>,
}

fn validate_system(system: &System, p: u64, degree: u32, budget: Duration) -> Result<(), String> {
    if p != system.modulus || p > MAX_FIELD || p <= 3 || !is_prime_u64(p) {
        return Err("solver field does not match the encoded prime field".into());
    }
    if system.variables == 0
        || system.variables > 5
        || system
            .equations
            .iter()
            .flatten()
            .any(|(e, c)| e.len() != system.variables || *c == 0 || *c >= p)
    {
        return Err("malformed polynomial system".into());
    }
    if budget.is_zero() || degree == 0 {
        return Err("a positive deadline and degree bound are required".into());
    }
    Ok(())
}

/// Exhaustive legacy full-field scan, retained as an exact-set control.
pub fn solve_system_reference(
    system: &System,
    p: u64,
    degree: u32,
    budget: Duration,
) -> Result<AlgebraReport, String> {
    validate_system(system, p, degree, budget)?;
    let report = f4_fp::solve(
        &system.equations,
        system.variables,
        p,
        &F4Options::new(Ordering::Grevlex, degree).with_budget(budget),
    );
    let complete = !report.timed_out && !matches!(report.verdict, Verdict::Undetermined);
    let mut sets = BTreeSet::new();
    if let Verdict::Solutions(roots) = report.verdict {
        for root in roots {
            if !system
                .equations
                .iter()
                .all(|f| f4_fp::eval(f, &root, p) == 0)
            {
                return Err("F4 returned a false solution".into());
            }
            sets.insert(decode_root(system, &root)?);
        }
    }
    Ok(AlgebraReport {
        complete,
        coordinate_sets: sets.into_iter().collect(),
        solver_calls: report.f4_runs,
        solving_degree: report.solving_degree,
        max_rows: report.max_rows,
        max_columns: report.max_cols,
        f4_field_multiplications: report.field_ops,
        timed_out: report.timed_out,
        ..AlgebraReport::default()
    })
}

/// Exhaustively visit every relation using exact coordinate domains and
/// rational-factor extraction. This remains the default for oracle tests.
pub fn solve_system(
    system: &System,
    p: u64,
    degree: u32,
    budget: Duration,
) -> Result<AlgebraReport, String> {
    solve_system_visit(system, p, degree, budget, |_| Ok(false))
}

/// Visit decoded, deduplicated coordinate sets from the existing exact-domain
/// solver. Returning `true` requests an early stop; the report then preserves
/// partial-enumeration status. A visitor error is propagated, never a solution.
/// This exposes the same path used by [`find_relation`] to measurement drivers.
pub fn visit_system(
    system: &System,
    p: u64,
    degree: u32,
    budget: Duration,
    visitor: impl FnMut(&[u64]) -> Result<bool, String>,
) -> Result<AlgebraReport, String> {
    solve_system_visit(system, p, degree, budget, visitor)
}

fn solve_system_visit(
    system: &System,
    p: u64,
    degree: u32,
    budget: Duration,
    mut visitor: impl FnMut(&[u64]) -> Result<bool, String>,
) -> Result<AlgebraReport, String> {
    let start = Instant::now();
    validate_system(system, p, degree, budget)?;
    let Some(budget) = budget.checked_sub(start.elapsed()).filter(|d| !d.is_zero()) else {
        return Ok(AlgebraReport {
            timed_out: true,
            ..AlgebraReport::default()
        });
    };
    let mut sets = BTreeSet::new();
    let mut error = None;
    let report = f4_fp::solve_visit(
        &system.equations,
        system.variables,
        p,
        &F4Options::new(Ordering::Grevlex, degree).with_budget(budget),
        &system.domains,
        |root| {
            let result = decode_root(system, root).and_then(|coordinates| {
                if sets.insert(coordinates.clone()) {
                    visitor(&coordinates)
                } else {
                    Ok(false)
                }
            });
            match result {
                Ok(stop) => stop,
                Err(e) => {
                    error = Some(e);
                    true
                }
            }
        },
    )?;
    if let Some(e) = error {
        return Err(e);
    }
    Ok(AlgebraReport {
        complete: report.complete,
        stopped_early: report.stopped_early,
        coordinate_sets: sets.into_iter().collect(),
        solver_calls: report.f4_runs,
        solving_degree: report.solving_degree,
        max_rows: report.max_rows,
        max_columns: report.max_cols,
        f4_field_multiplications: report.field_ops,
        timed_out: report.timed_out,
        roots: Some(RootDiagnostics {
            equation_roots_visited: report.solutions_visited,
            candidates_rejected: report.candidates_rejected,
            domain_evaluations: report.domain_evaluations,
            factored_univariates: report.factored_univariates,
            root_field_multiplications: report.root_field_multiplications,
            full_field_branches: report.full_field_branches,
        }),
    })
}

impl AlgebraReport {
    fn empty() -> Self {
        Self {
            complete: true,
            ..Self::default()
        }
    }

    fn include(&mut self, other: &Self) {
        self.complete &= other.complete;
        self.stopped_early |= other.stopped_early;
        self.timed_out |= other.timed_out;
        self.solver_calls += other.solver_calls;
        self.solving_degree = self.solving_degree.max(other.solving_degree);
        self.max_rows = self.max_rows.max(other.max_rows);
        self.max_columns = self.max_columns.max(other.max_columns);
        self.f4_field_multiplications += other.f4_field_multiplications;
        if let Some(other) = &other.roots {
            let roots = self.roots.get_or_insert_with(RootDiagnostics::default);
            roots.equation_roots_visited += other.equation_roots_visited;
            roots.candidates_rejected += other.candidates_rejected;
            roots.domain_evaluations += other.domain_evaluations;
            roots.factored_univariates += other.factored_univariates;
            roots.root_field_multiplications += other.root_field_multiplications;
            roots.full_field_branches += other.full_field_branches;
        }
    }
}

/// Guess the first coordinate and charge every branch to one shared deadline.
/// This is a solver variant, not an oracle substituting the known logarithm.
pub fn solve_hybrid(
    curve: &FastCurve,
    parts: &[Vec<u64>],
    encoding: Encoding,
    degree: u32,
    budget: Duration,
) -> Result<AlgebraReport, String> {
    solve_hybrid_visit(curve, parts, encoding, degree, budget, |_| Ok(false))
}

fn solve_hybrid_visit(
    curve: &FastCurve,
    parts: &[Vec<u64>],
    encoding: Encoding,
    degree: u32,
    budget: Duration,
    mut visitor: impl FnMut(&[u64]) -> Result<bool, String>,
) -> Result<AlgebraReport, String> {
    let start = Instant::now();
    if degree == 0 || budget.is_zero() {
        return Err("a positive deadline and degree bound are required".into());
    }
    if !matches!(
        encoding,
        Encoding::Expanded4 | Encoding::Split4 | Encoding::Partitioned3
    ) {
        return Err("hybrid guessing applies to coordinate encodings".into());
    }
    let original = build_system(curve, parts, encoding)?;
    let m = parts.len();
    let mut total = AlgebraReport::empty();
    let mut sets = BTreeSet::new();
    for &guess in &parts[0] {
        if start.elapsed() >= budget {
            total.complete = false;
            total.timed_out = true;
            break;
        }
        let equations = original
            .equations
            .iter()
            .map(|f| {
                let terms: Poly = f
                    .iter()
                    .map(|(e, c)| {
                        let mut factor = 1;
                        for _ in 0..e[0] {
                            factor = mul(factor, guess, curve.f.p);
                        }
                        (e[1..].to_vec(), mul(*c, factor, curve.f.p))
                    })
                    .collect();
                f4_fp::normalise(&terms, curve.f.p, Ordering::Grevlex)
            })
            .collect();
        let branch = System {
            variables: original.variables - 1,
            equations,
            modulus: curve.f.p,
            decode: Decode::Coordinates(m - 1),
            domains: original.domains[1..].to_vec(),
            model: original.model,
        };
        let Some(remaining) = budget.checked_sub(start.elapsed()).filter(|d| !d.is_zero()) else {
            total.complete = false;
            total.timed_out = true;
            break;
        };
        let report = solve_system_visit(&branch, curve.f.p, degree, remaining, |coordinates| {
            let mut coordinates = coordinates.to_vec();
            coordinates.push(guess);
            coordinates.sort_unstable();
            if sets.insert(coordinates.clone()) {
                visitor(&coordinates)
            } else {
                Ok(false)
            }
        })?;
        total.include(&report);
        if !total.complete {
            break;
        }
    }
    total.coordinate_sets = sets.into_iter().collect();
    Ok(total)
}

/// A deterministic separating family for triples of indices. It uses at most
/// `2*ceil(log2(s))*(ceil(log2(s))-1)` colorings, without examining any point
/// relation. Class sizes are not claimed balanced. Empty classes are dropped.
pub fn triple_cover(size: usize) -> Result<Vec<Vec<Vec<usize>>>, String> {
    if !(3..=64).contains(&size) {
        return Err("triple-cover instrument supports 3..=64 entries".into());
    }
    let bits = usize::BITS - (size - 1).leading_zeros();
    let mut seen = BTreeSet::new();
    let mut family = Vec::new();
    for first in 0..bits {
        for second in 0..bits {
            if first == second {
                continue;
            }
            for singleton_bit in 0..2 {
                let mut parts = vec![vec![], vec![], vec![]];
                for i in 0..size {
                    let color = if (i >> first) & 1 == singleton_bit {
                        0
                    } else {
                        1 + ((i >> second) & 1)
                    };
                    parts[color].push(i);
                }
                if parts.iter().any(Vec::is_empty) {
                    continue;
                }
                // Relabeling colors does not change which unordered triples appear.
                parts.sort();
                if seen.insert(parts.clone()) {
                    family.push(parts);
                }
            }
        }
    }
    Ok(family)
}

/// Solve every coloring in a relation-independent separating family. All
/// branches share one deadline, and repeated coordinate sets count once.
pub fn solve_cover(
    curve: &FastCurve,
    values: &[u64],
    degree: u32,
    budget: Duration,
) -> Result<AlgebraReport, String> {
    solve_cover_visit(curve, values, degree, budget, |_| Ok(false))
}

fn solve_cover_visit(
    curve: &FastCurve,
    values: &[u64],
    degree: u32,
    budget: Duration,
    mut visitor: impl FnMut(&[u64]) -> Result<bool, String>,
) -> Result<AlgebraReport, String> {
    let start = Instant::now();
    checked_coordinates(values, curve.f.p)?;
    let family = triple_cover(values.len())?;
    if degree == 0 || budget.is_zero() {
        return Err("a positive deadline and degree bound are required".into());
    }
    let mut total = AlgebraReport::empty();
    let mut sets = BTreeSet::new();
    for indices in family {
        if start.elapsed() >= budget {
            total.complete = false;
            total.timed_out = true;
            break;
        }
        let parts: Vec<Vec<u64>> = indices
            .iter()
            .map(|part| part.iter().map(|&i| values[i]).collect())
            .collect();
        let system = build_system(curve, &parts, Encoding::Partitioned3)?;
        let Some(remaining) = budget.checked_sub(start.elapsed()).filter(|d| !d.is_zero()) else {
            total.complete = false;
            total.timed_out = true;
            break;
        };
        let report = solve_system_visit(&system, curve.f.p, degree, remaining, |coordinates| {
            if sets.insert(coordinates.to_vec()) {
                visitor(coordinates)
            } else {
                Ok(false)
            }
        })?;
        total.include(&report);
        if !total.complete {
            break;
        }
    }
    total.coordinate_sets = sets.into_iter().collect();
    Ok(total)
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Labels {
    Random,
    Guaranteed,
}

/// Coefficients are chosen at construction, never retrofitted onto fixed points.
pub fn guaranteed_weights(arity: usize, n: u64) -> Result<Vec<u64>, String> {
    if !(3..=16).contains(&arity) {
        return Err("supported relation arity is 3..=16".into());
    }
    let weights = if arity % 2 == 1 {
        vec![1; arity]
    } else {
        (0..arity).map(|i| 1u64 << i).collect()
    };
    if weights.iter().sum::<u64>() >= n {
        return Err("group order must exceed the total coefficient weight".into());
    }
    Ok(weights)
}

#[derive(Clone, Debug, Serialize)]
pub struct LabeledPoint {
    /// Canonical field residues (not Montgomery residues).
    pub x: u64,
    pub y: u64,
    pub u: u64,
    pub v: u64,
}
#[derive(Clone, Debug, Serialize)]
pub struct LabeledBase {
    pub parts: Vec<Vec<u64>>,
    pub points: BTreeMap<u64, LabeledPoint>,
    pub draws: usize,
}

pub fn validate_instance(curve: &FastCurve, g: Pt, q: Pt) -> Result<(), String> {
    let p = curve.f.p;
    if p <= 3 || p > MAX_FIELD || !is_prime_u64(p) || !is_prime_u64(curve.n) || curve.n <= 3 {
        return Err("prime field <= MAX_FIELD and a prime subgroup order > 3 required".into());
    }
    let a = curve.f.from_mont(curve.a);
    let b = curve.f.from_mont(curve.b);
    if (4 * mul(mul(a, a, p), a, p) + 27 * mul(b, b, p)).is_multiple_of(p) {
        return Err("singular curve".into());
    }
    if !curve.is_on_curve(g)
        || !curve.is_on_curve(q)
        || curve.mul(Some(g), curve.n).is_some()
        || curve.mul(Some(q), curve.n).is_some()
    {
        return Err("points are not in the declared prime-order subgroup".into());
    }
    Ok(())
}

fn labeled(curve: &FastCurve, point: Pt, u: u64, v: u64) -> LabeledPoint {
    let (point, flipped) = curve.canonical_sign(point);
    let (x, y) = curve.canonical(point);
    LabeledPoint {
        x,
        y,
        u: if flipped { neg(u, curve.n) } else { u },
        v: if flipped { neg(v, curve.n) } else { v },
    }
}

#[allow(clippy::too_many_arguments)]
pub fn sample_base(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    sizes: &[usize],
    labels: Labels,
    seed: u64,
    max_draws: usize,
) -> Result<LabeledBase, String> {
    validate_instance(curve, g, q)?;
    if sizes.is_empty()
        || sizes.contains(&0)
        || sizes.iter().sum::<usize>() > 32
        || sizes.iter().sum::<usize>() > (curve.n as usize - 1) / 2
    {
        return Err("nonempty lists totaling at most 32 distinct abscissae required".into());
    }
    let weights = if labels == Labels::Guaranteed {
        guaranteed_weights(sizes.len(), curve.n)?
    } else {
        vec![0; sizes.len()]
    };
    let shifts: Vec<_> = weights.iter().map(|&w| curve.mul(Some(q), w)).collect();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut base = LabeledBase {
        parts: vec![vec![]; sizes.len()],
        points: BTreeMap::new(),
        draws: 0,
    };
    for (slot, &size) in sizes.iter().enumerate() {
        while base.parts[slot].len() < size {
            if base.draws >= max_draws {
                return Err("factor-base sampling budget exhausted".into());
            }
            let u = rng.gen_range(0..curve.n);
            let v = if labels == Labels::Guaranteed {
                weights[slot]
            } else {
                rng.gen_range(0..curve.n)
            };
            let point = if labels == Labels::Guaranteed {
                curve.add(curve.mul(Some(g), u), shifts[slot])
            } else {
                curve.combine(g, q, u, v)
            };
            base.draws += 1;
            let Some(point) = point else {
                continue;
            };
            let entry = labeled(curve, point, u, v);
            if base.points.contains_key(&entry.x) {
                continue;
            }
            base.parts[slot].push(entry.x);
            base.points.insert(entry.x, entry);
        }
    }
    validate_base(curve, g, q, &base)?;
    Ok(base)
}

pub fn validate_base(curve: &FastCurve, g: Pt, q: Pt, base: &LabeledBase) -> Result<(), String> {
    let all: Vec<_> = base.parts.iter().flatten().copied().collect();
    checked_coordinates(&all, curve.f.p)?;
    if all.len() != base.points.len() || all.iter().any(|x| !base.points.contains_key(x)) {
        return Err("factor-base coordinate/label mismatch".into());
    }
    for (&x, entry) in &base.points {
        if x != entry.x || entry.y >= curve.f.p || entry.u >= curve.n || entry.v >= curve.n {
            return Err("noncanonical labeled point".into());
        }
        let point = curve.point(entry.x, entry.y);
        if !curve.is_on_curve(point) || curve.combine(g, q, entry.u, entry.v) != Some(point) {
            return Err("invalid known point representation".into());
        }
    }
    Ok(())
}

/// Expand independent seeds under the actual curve automorphism, preserving
/// labels. Refuse short/overlapping orbits and a wrong subgroup eigenvalue.
pub fn orbit_base(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    seeds: &LabeledBase,
    beta: u64,
    lambda: u64,
) -> Result<LabeledBase, String> {
    validate_instance(curve, g, q)?;
    validate_base(curve, g, q, seeds)?;
    let beta_mont = checked_order_three(curve, g, q, beta, lambda)?;
    let n = curve.n;
    let phi = |point: Pt| Pt {
        x: curve.f.mul(point.x, beta_mont),
        y: point.y,
    };
    let mut result = LabeledBase {
        parts: vec![vec![]; seeds.parts.len()],
        points: BTreeMap::new(),
        draws: seeds.draws,
    };
    for (slot, part) in seeds.parts.iter().enumerate() {
        for x in part {
            let entry = &seeds.points[x];
            let mut point = curve.point(entry.x, entry.y);
            let mut u = entry.u;
            let mut v = entry.v;
            for _ in 0..3 {
                let entry = labeled(curve, point, u, v);
                if result.points.contains_key(&entry.x) {
                    return Err("short or overlapping automorphism orbits".into());
                }
                result.parts[slot].push(entry.x);
                result.points.insert(entry.x, entry);
                point = phi(point);
                u = mul(u, lambda, n);
                v = mul(v, lambda, n);
            }
        }
    }
    validate_base(curve, g, q, &result)?;
    Ok(result)
}

fn checked_order_three(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    beta: u64,
    lambda: u64,
) -> Result<u64, String> {
    let p = curve.f.p;
    let n = curve.n;
    if curve.a != 0
        || beta >= p
        || beta <= 1
        || mul(mul(beta, beta, p), beta, p) != 1
        || lambda >= n
        || lambda <= 1
        || !(mul(lambda, lambda, n) + lambda + 1).is_multiple_of(n)
    {
        return Err(
            "a rational order-three automorphism and subgroup eigenvalue are required".into(),
        );
    }
    let beta_mont = curve.f.to_mont(beta);
    let phi = |point: Pt| Pt {
        x: curve.f.mul(point.x, beta_mont),
        y: point.y,
    };
    if curve.mul(Some(g), lambda) != Some(phi(g)) || curve.mul(Some(q), lambda) != Some(phi(q)) {
        return Err("incorrect automorphism eigenvalue".into());
    }
    Ok(beta_mont)
}

/// Select all distinct-coordinate triples except horizontal intersections.
/// The added saturation equation t*z-1=0 is authorized only after checking
/// full orbit closure AND label propagation, including sign changes.
pub fn build_pruned_line_system(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    base: &LabeledBase,
    beta: u64,
    lambda: u64,
) -> Result<System, String> {
    validate_instance(curve, g, q)?;
    validate_base(curve, g, q, base)?;
    checked_order_three(curve, g, q, beta, lambda)?;
    let p = curve.f.p;
    for entry in base.points.values() {
        let next_x = mul(beta, entry.x, p);
        if next_x == entry.x {
            return Err("short automorphism orbit".into());
        }
        let next = base
            .points
            .get(&next_x)
            .ok_or("base is not closed under the automorphism")?;
        let flipped = if next.y == entry.y {
            false
        } else if next.y == neg(entry.y, p) {
            true
        } else {
            return Err("orbit point has the wrong ordinate".into());
        };
        let u = mul(lambda, entry.u, curve.n);
        let v = mul(lambda, entry.v, curve.n);
        if next.u != (if flipped { neg(u, curve.n) } else { u })
            || next.v != (if flipped { neg(v, curve.n) } else { v })
        {
            return Err("orbit labels are independent; horizontal pruning is not justified".into());
        }
    }
    let values: Vec<_> = base.points.keys().copied().collect();
    let mut system = build_system(curve, &[values], Encoding::Line3)?;
    // Extend the two line parameters by the inverse of t. Do not give
    // that inverse a coordinate-domain tag.
    for equation in &mut system.equations {
        for (e, _) in equation {
            e.push(0);
        }
    }
    let t = Expr::var(0, 3, p);
    let z = Expr::var(2, 3, p);
    system.equations.push(f4_fp::normalise(
        &t.mul(&z).sub(&t.c(1)).terms,
        p,
        Ordering::Grevlex,
    ));
    system.variables = 3;
    system.domains.push(None);
    Ok(system)
}

#[derive(Clone, Debug, Serialize)]
pub struct Recovery {
    pub logarithm: u64,
    pub coordinates: Vec<u64>,
    pub signs: Vec<i8>,
    pub a: u64,
    pub b: u64,
}

/// Boundaries inside signed recovery, before the named work begins. Observers
/// do not supply points, scalars, stopping decisions or acceptance decisions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecoveryPhase {
    RelationCheck,
    Descent,
    Verification,
}

/// A validated, immutably borrowed base for checking many candidate sets.
/// This gives streaming drivers the validation hoisting used by
/// [`find_relation`], while retaining every per-candidate coordinate, sign,
/// nonzero-denominator and scalar check. Private fields prevent constructing
/// an unchecked context; the borrow prevents changing its base or curve.
pub struct RecoveryContext<'a> {
    curve: &'a FastCurve,
    g: Pt,
    q: Pt,
    base: &'a LabeledBase,
}

impl<'a> RecoveryContext<'a> {
    pub fn new(curve: &'a FastCurve, g: Pt, q: Pt, base: &'a LabeledBase) -> Result<Self, String> {
        validate_instance(curve, g, q)?;
        validate_base(curve, g, q, base)?;
        Ok(Self { curve, g, q, base })
    }

    pub fn recover_observed(
        &self,
        sets: &[Vec<u64>],
        mut observer: impl FnMut(RecoveryPhase),
    ) -> Result<Option<Recovery>, String> {
        observer(RecoveryPhase::RelationCheck);
        recover_validated_observed(self.curve, self.g, self.q, self.base, sets, &mut observer)
    }
}

/// [`recover`] with an opt-in phase observer. Existing recovery callers keep
/// their algorithm and validation unchanged; clocks can account for all signs,
/// rejected B=0 relations, modular recovery and the final scalar check.
pub fn recover_observed(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    base: &LabeledBase,
    sets: &[Vec<u64>],
    mut observer: impl FnMut(RecoveryPhase),
) -> Result<Option<Recovery>, String> {
    observer(RecoveryPhase::RelationCheck);
    RecoveryContext::new(curve, g, q, base)?.recover_observed(sets, observer)
}

pub fn recover(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    base: &LabeledBase,
    sets: &[Vec<u64>],
) -> Result<Option<Recovery>, String> {
    validate_instance(curve, g, q)?;
    validate_base(curve, g, q, base)?;
    recover_validated(curve, g, q, base, sets)
}

fn recover_validated(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    base: &LabeledBase,
    sets: &[Vec<u64>],
) -> Result<Option<Recovery>, String> {
    recover_validated_observed(curve, g, q, base, sets, &mut |_| {})
}

fn recover_validated_observed(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    base: &LabeledBase,
    sets: &[Vec<u64>],
    observer: &mut impl FnMut(RecoveryPhase),
) -> Result<Option<Recovery>, String> {
    for coordinates in sets {
        if !(3..=16).contains(&coordinates.len()) {
            return Err("unsupported relation arity".into());
        }
        checked_coordinates(coordinates, curve.f.p)?;
        let entries: Vec<_> = coordinates
            .iter()
            .map(|x| {
                base.points
                    .get(x)
                    .ok_or_else(|| "solver coordinate outside factor base".to_string())
            })
            .collect::<Result<_, _>>()?;
        for mask in 0..(1usize << (entries.len() - 1)) {
            observer(RecoveryPhase::RelationCheck);
            let signs: Vec<i8> = (0..entries.len())
                .map(|i| {
                    if i == 0 || mask & (1 << (i - 1)) == 0 {
                        1
                    } else {
                        -1
                    }
                })
                .collect();
            let mut sum = None;
            let mut a = 0;
            let mut b = 0;
            for (entry, &sign) in entries.iter().zip(&signs) {
                let point = curve.point(entry.x, entry.y);
                sum = curve.add(sum, Some(if sign == 1 { point } else { curve.neg(point) }));
                a = (a + if sign == 1 {
                    entry.u
                } else {
                    neg(entry.u, curve.n)
                }) % curve.n;
                b = (b + if sign == 1 {
                    entry.v
                } else {
                    neg(entry.v, curve.n)
                }) % curve.n;
            }
            if sum.is_some() || b == 0 {
                continue;
            }
            observer(RecoveryPhase::Descent);
            let logarithm = mul(neg(a, curve.n), inv_mod(b, curve.n), curve.n);
            observer(RecoveryPhase::Verification);
            if curve.mul(Some(g), logarithm) != Some(q) {
                return Err("signed point relation failed logarithm verification".into());
            }
            return Ok(Some(Recovery {
                logarithm,
                coordinates: coordinates.clone(),
                signs,
                a,
                b,
            }));
        }
    }
    Ok(None)
}

#[derive(Clone, Copy, Debug)]
pub enum RelationSearch {
    System(Encoding),
    Hybrid(Encoding),
    Cover3,
    PrunedLine3 { beta: u64, lambda: u64 },
}

fn grouped_parts(base: &LabeledBase, encoding: Encoding) -> Vec<Vec<u64>> {
    match encoding {
        Encoding::Unordered3 | Encoding::Line3 => vec![base.points.keys().copied().collect()],
        Encoding::BlockSymmetric4 if base.parts.len() == 4 => vec![
            base.parts[..2].iter().flatten().copied().collect(),
            base.parts[2..].iter().flatten().copied().collect(),
        ],
        _ => base.parts.clone(),
    }
}

/// Stream candidates through the actual point/label checks, continuing past
/// B=0 relations. Return only after independently verifying dG=Q, or after
/// exhaustion/timeout. Validation, construction and all branches share a
/// single cooperative budget; no planted scalar is an argument.
#[allow(clippy::too_many_arguments)]
pub fn find_relation(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    base: &LabeledBase,
    search: RelationSearch,
    degree: u32,
    budget: Duration,
) -> Result<(AlgebraReport, Option<Recovery>), String> {
    if degree == 0 || budget.is_zero() {
        return Err("a positive deadline and degree bound are required".into());
    }
    let start = Instant::now();
    validate_instance(curve, g, q)?;
    validate_base(curve, g, q, base)?;
    let remaining = || budget.checked_sub(start.elapsed()).filter(|d| !d.is_zero());
    let mut recovery = None;
    let mut visit = |coordinates: &[u64]| {
        recovery = recover_validated(curve, g, q, base, &[coordinates.to_vec()])?;
        Ok(recovery.is_some())
    };
    let exhausted = || AlgebraReport {
        timed_out: true,
        ..AlgebraReport::default()
    };
    let report = match search {
        RelationSearch::System(encoding) => {
            let system = build_system(curve, &grouped_parts(base, encoding), encoding)?;
            match remaining() {
                Some(budget) => solve_system_visit(&system, curve.f.p, degree, budget, &mut visit)?,
                None => exhausted(),
            }
        }
        RelationSearch::PrunedLine3 { beta, lambda } => {
            let system = build_pruned_line_system(curve, g, q, base, beta, lambda)?;
            match remaining() {
                Some(budget) => solve_system_visit(&system, curve.f.p, degree, budget, &mut visit)?,
                None => exhausted(),
            }
        }
        RelationSearch::Hybrid(encoding) => match remaining() {
            Some(budget) => {
                solve_hybrid_visit(curve, &base.parts, encoding, degree, budget, &mut visit)?
            }
            None => exhausted(),
        },
        RelationSearch::Cover3 => {
            let values: Vec<_> = base.points.keys().copied().collect();
            match remaining() {
                Some(budget) => solve_cover_visit(curve, &values, degree, budget, &mut visit)?,
                None => exhausted(),
            }
        }
    };
    Ok((report, recovery))
}

#[derive(Clone, Debug, Serialize)]
pub struct Attempt {
    pub base: LabeledBase,
    pub algebra: AlgebraReport,
    pub recovery: Option<Recovery>,
}

/// Bounded single-relation DLP route. The target scalar is not an argument.
/// Each attempt has its own solver deadline; the attempt cap bounds retries.
/// Guaranteed even-arity weights apply to one point per original partition,
/// so regrouping into unordered blocks still requires the `B != 0` check.
#[allow(clippy::too_many_arguments)]
pub fn solve_dlp(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    encoding: Encoding,
    labels: Labels,
    subset_size: usize,
    seed: u64,
    max_attempts: usize,
    budget: Duration,
) -> Result<Vec<Attempt>, String> {
    solve_dlp_with_mode(
        curve,
        g,
        q,
        encoding,
        labels,
        subset_size,
        seed,
        max_attempts,
        budget,
        false,
    )
}

/// The same bounded retry route with an explicit streaming/exhaustive choice.
/// The legacy [`solve_dlp`] retains exhaustive behavior for callers/tests.
#[allow(clippy::too_many_arguments)]
pub fn solve_dlp_with_mode(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    encoding: Encoding,
    labels: Labels,
    subset_size: usize,
    seed: u64,
    max_attempts: usize,
    budget: Duration,
    first_informative: bool,
) -> Result<Vec<Attempt>, String> {
    validate_instance(curve, g, q)?;
    let arity = match encoding {
        Encoding::Partitioned3 | Encoding::Unordered3 | Encoding::Line3 => 3,
        _ => 4,
    };
    if subset_size == 0
        || subset_size > 4
        || max_attempts == 0
        || max_attempts > 128
        || budget.is_zero()
    {
        return Err(
            "bounded instrument requires subset size 1..=4, attempts 1..=128 and a deadline".into(),
        );
    }
    let mut attempts = Vec::new();
    for i in 0..max_attempts {
        let base = sample_base(
            curve,
            g,
            q,
            &vec![subset_size; arity],
            labels,
            seed.wrapping_add(i as u64),
            10_000,
        )?;
        let (algebra, recovery) = if first_informative {
            find_relation(
                curve,
                g,
                q,
                &base,
                RelationSearch::System(encoding),
                24,
                budget,
            )?
        } else {
            let system = build_system(curve, &grouped_parts(&base, encoding), encoding)?;
            let algebra = solve_system(&system, curve.f.p, 24, budget)?;
            let recovery = recover(curve, g, q, &base, &algebra.coordinate_sets)?;
            (algebra, recovery)
        };
        let stop = recovery.is_some();
        attempts.push(Attempt {
            base,
            algebra,
            recovery,
        });
        if stop {
            break;
        }
    }
    Ok(attempts)
}

/// Existing symmetric S3 identity, expressed in the repository's certificate
/// format. Issuing/checking a certificate evaluates it over the format's field;
/// it is not a formal proof over every characteristic.
pub fn symmetric_s3_identity() -> super::identity_certificate::IdentityStatement {
    use super::identity_certificate::{Expr as E, IdentityStatement};
    fn plus(xs: Vec<E>) -> E {
        E::sum(xs)
    }
    fn times(xs: Vec<E>) -> E {
        E::product(xs)
    }
    fn minus(x: E, y: E) -> E {
        plus(vec![x, E::minus(y)])
    }
    fn square(x: E) -> E {
        times(vec![x.clone(), x])
    }
    let [x, y, z, a, b] = ["x", "y", "z", "a", "b"].map(E::var);
    let e1 = plus(vec![x.clone(), y.clone(), z.clone()]);
    let e2 = plus(vec![
        times(vec![x.clone(), y.clone()]),
        times(vec![x.clone(), z.clone()]),
        times(vec![y.clone(), z.clone()]),
    ]);
    let e3 = times(vec![x.clone(), y.clone(), z.clone()]);
    let lhs = plus(vec![
        times(vec![square(minus(x.clone(), y.clone())), square(z.clone())]),
        E::minus(times(vec![
            E::Const(2),
            plus(vec![
                times(vec![
                    plus(vec![x.clone(), y.clone()]),
                    plus(vec![times(vec![x.clone(), y.clone()]), a.clone()]),
                ]),
                times(vec![E::Const(2), b.clone()]),
            ]),
            z,
        ])),
        square(minus(times(vec![x.clone(), y.clone()]), a.clone())),
        E::minus(times(vec![E::Const(4), b.clone(), plus(vec![x, y])])),
    ]);
    let rhs = minus(
        square(minus(e2, a)),
        times(vec![E::Const(4), e1, plus(vec![e3, b])]),
    );
    IdentityStatement {
        name: "S3 in elementary symmetric coordinates".into(),
        source:
            "APS 2018 section 2 S3; established symmetric-coordinate rewrite, not a novelty claim"
                .into(),
        variables: ["x", "y", "z", "a", "b"].map(str::to_string).to_vec(),
        degree_bound: 4,
        lhs,
        rhs,
    }
}

/// The standard substitution y=tX+c, recorded in the native identity format.
pub fn line_intersection_identity() -> super::identity_certificate::IdentityStatement {
    use super::identity_certificate::{Expr as E, IdentityStatement};
    fn add(xs: Vec<E>) -> E {
        E::sum(xs)
    }
    fn product(xs: Vec<E>) -> E {
        E::product(xs)
    }
    fn square(x: E) -> E {
        product(vec![x.clone(), x])
    }
    let [x, t, c, a, b] = ["x", "t", "c", "a", "b"].map(E::var);
    let cubic = product(vec![x.clone(), x.clone(), x.clone()]);
    let lhs = add(vec![
        cubic.clone(),
        product(vec![a.clone(), x.clone()]),
        b.clone(),
        E::minus(square(add(vec![
            product(vec![t.clone(), x.clone()]),
            c.clone(),
        ]))),
    ]);
    let rhs = add(vec![
        cubic,
        E::minus(product(vec![square(t.clone()), square(x.clone())])),
        product(vec![
            add(vec![a, E::minus(product(vec![E::Const(2), t, c.clone()]))]),
            x,
        ]),
        b,
        E::minus(square(c)),
    ]);
    IdentityStatement {
        name: "Short-Weierstrass line intersection cubic".into(),
        source: "Milne, Elliptic Curves, second edition, I section 3; direct coefficient expansion, not a novelty claim".into(),
        variables: ["x", "t", "c", "a", "b"].map(str::to_string).to_vec(),
        degree_bound: 4, lhs, rhs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expression_arithmetic_preserves_relative_coefficients() {
        let x = Expr::var(0, 1, 101);
        let expression = x.c(4).mul(&x).add(&x.c(7)).square().sub(&x.c(2));
        for value in 0..101 {
            assert_eq!(
                f4_fp::eval(&expression.terms, &[value], 101),
                ((4 * value + 7).pow(2) + 101 - 2) % 101
            );
        }
    }

    fn fixture() -> (FastCurve, Pt, Pt, u64) {
        let curve = FastCurve::new("aps-test-p101", 101, 59, 6, 107);
        let g = curve.point(78, 4);
        let d = 37;
        let q = curve.mul(Some(g), d).unwrap();
        (curve, g, q, d)
    }
    /// Validation oracle only: enumerate coordinates, then add actual curve
    /// points. Neither polynomial solving nor the target logarithm is used.
    fn direct_sets(curve: &FastCurve, parts: &[Vec<u64>]) -> Vec<Vec<u64>> {
        fn walk(
            curve: &FastCurve,
            parts: &[Vec<u64>],
            coords: &mut Vec<u64>,
            out: &mut BTreeSet<Vec<u64>>,
        ) {
            if coords.len() == parts.len() {
                for mask in 0..(1usize << (coords.len() - 1)) {
                    let mut sum = None;
                    for (i, &x) in coords.iter().enumerate() {
                        let point = curve.lift_x(x).unwrap();
                        sum = curve.add(
                            sum,
                            Some(if i == 0 || mask & (1 << (i - 1)) == 0 {
                                point
                            } else {
                                curve.neg(point)
                            }),
                        );
                    }
                    if sum.is_none() {
                        let mut c = coords.clone();
                        c.sort_unstable();
                        out.insert(c);
                        break;
                    }
                }
                return;
            }
            for &x in &parts[coords.len()] {
                coords.push(x);
                walk(curve, parts, coords, out);
                coords.pop();
            }
        }
        let mut result = BTreeSet::new();
        walk(curve, parts, &mut vec![], &mut result);
        result.into_iter().collect()
    }
    fn all_triples(values: &[u64]) -> Vec<Vec<Vec<u64>>> {
        let mut out = vec![];
        for i in 0..values.len() {
            for j in i + 1..values.len() {
                for k in j + 1..values.len() {
                    out.push(vec![vec![values[i]], vec![values[j]], vec![values[k]]]);
                }
            }
        }
        out
    }
    fn solve(curve: &FastCurve, parts: &[Vec<u64>], encoding: Encoding) -> AlgebraReport {
        let system = build_system(curve, parts, encoding).unwrap();
        let report = solve_system(&system, curve.f.p, 24, Duration::from_secs(10)).unwrap();
        assert!(report.complete, "incomplete {encoding:?}");
        report
    }

    // Construct a positive unit-test input with the independent point
    // oracle. This is not a performance sample or relation-finding path.
    fn positive_test_base(curve: &FastCurve, g: Pt, q: Pt) -> LabeledBase {
        for seed in 717..749 {
            let base =
                sample_base(curve, g, q, &[3, 3, 3], Labels::Guaranteed, seed, 1000).unwrap();
            let values: Vec<_> = base.points.keys().copied().collect();
            let triples: BTreeSet<_> = all_triples(&values)
                .iter()
                .flat_map(|p| direct_sets(curve, p))
                .collect();
            if !direct_sets(curve, &base.parts).is_empty() && triples.len() >= 2 {
                return base;
            }
        }
        panic!("positive fixture construction exhausted its fixed test seed range");
    }

    #[test]
    fn weights_exclude_zero_for_every_sign_choice_in_supported_fixture_range() {
        for m in 3..=10 {
            let n = 2053;
            let weights = guaranteed_weights(m, n).unwrap();
            for mask in 0..1usize << m {
                let b = weights.iter().enumerate().fold(0, |acc, (i, &w)| {
                    (acc + if mask & (1 << i) == 0 { w } else { neg(w, n) }) % n
                });
                assert_ne!(b, 0, "m={m}, mask={mask}");
            }
        }
        assert!(guaranteed_weights(4, 13).is_err());
        assert!(guaranteed_weights(17, 1 << 20).is_err());
    }

    #[test]
    fn generated_labels_and_canonical_signs_are_actual_group_representations() {
        let (curve, g, q, _) = fixture();
        for labels in [Labels::Random, Labels::Guaranteed] {
            for m in [3, 4] {
                let base = sample_base(&curve, g, q, &vec![2; m], labels, 1234, 1000).unwrap();
                validate_base(&curve, g, q, &base).unwrap();
                assert_eq!(base.points.len(), m * 2);
                for part in &base.parts {
                    assert_eq!(part.len(), 2);
                }
            }
        }
    }

    #[test]
    fn partitioned_and_hybrid_s3_match_all_direct_point_solutions() {
        let (curve, g, q, _) = fixture();
        for seed in [101, 211, 503] {
            let base =
                sample_base(&curve, g, q, &[2, 2, 2], Labels::Guaranteed, seed, 1000).unwrap();
            let want = direct_sets(&curve, &base.parts);
            assert_eq!(
                solve(&curve, &base.parts, Encoding::Partitioned3).coordinate_sets,
                want
            );
            let hybrid = solve_hybrid(
                &curve,
                &base.parts,
                Encoding::Partitioned3,
                24,
                Duration::from_secs(10),
            )
            .unwrap();
            assert!(hybrid.complete);
            assert_eq!(hybrid.coordinate_sets, want);
        }
    }

    #[test]
    fn unordered_divisor_encoding_retains_every_distinct_coordinate_triple() {
        let (curve, g, q, _) = fixture();
        let base = sample_base(&curve, g, q, &[2, 2, 2], Labels::Guaranteed, 90210, 1000).unwrap();
        let values: Vec<_> = base.points.keys().copied().collect();
        let want: BTreeSet<_> = all_triples(&values)
            .iter()
            .flat_map(|parts| direct_sets(&curve, parts))
            .collect();
        let report = solve(&curve, &[values], Encoding::Unordered3);
        assert_eq!(report.coordinate_sets, want.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn line_divisors_and_domain_solving_match_both_the_point_oracle_and_legacy_scan() {
        let (curve, g, q, _) = fixture();
        for seed in [717, 90210] {
            let base =
                sample_base(&curve, g, q, &[2, 2, 2], Labels::Guaranteed, seed, 1000).unwrap();
            let values: Vec<_> = base.points.keys().copied().collect();
            let want: BTreeSet<_> = all_triples(&values)
                .iter()
                .flat_map(|parts| direct_sets(&curve, parts))
                .collect();
            let want: Vec<_> = want.into_iter().collect();
            for encoding in [Encoding::Unordered3, Encoding::Line3] {
                let system = build_system(&curve, std::slice::from_ref(&values), encoding).unwrap();
                assert_eq!(
                    system.variables(),
                    if encoding == Encoding::Line3 { 2 } else { 3 }
                );
                assert!(system.domains.iter().all(Option::is_none));
                let optimized =
                    solve_system(&system, curve.f.p, 24, Duration::from_secs(20)).unwrap();
                let reference =
                    solve_system_reference(&system, curve.f.p, 24, Duration::from_secs(20))
                        .unwrap();
                assert!(optimized.complete && reference.complete);
                assert_eq!(optimized.coordinate_sets, want);
                assert_eq!(reference.coordinate_sets, want);
            }
            for encoding in [Encoding::Partitioned3, Encoding::Split4] {
                let parts = if encoding == Encoding::Split4 {
                    sample_base(&curve, g, q, &[2, 2, 2, 2], Labels::Guaranteed, seed, 1000)
                        .unwrap()
                        .parts
                } else {
                    base.parts.clone()
                };
                let system = build_system(&curve, &parts, encoding).unwrap();
                assert!(system.domains.iter().take(parts.len()).all(Option::is_some));
                assert!(system.domains.iter().skip(parts.len()).all(Option::is_none));
                let optimized =
                    solve_system(&system, curve.f.p, 24, Duration::from_secs(10)).unwrap();
                let reference =
                    solve_system_reference(&system, curve.f.p, 24, Duration::from_secs(10))
                        .unwrap();
                assert!(optimized.complete && reference.complete);
                assert_eq!(optimized.coordinate_sets, reference.coordinate_sets);
                assert_eq!(optimized.coordinate_sets, direct_sets(&curve, &parts));
            }
        }
    }

    #[test]
    fn early_exit_requires_an_informative_verified_relation_across_all_search_routes() {
        let (curve, g, q, d) = fixture();
        let base = positive_test_base(&curve, g, q);
        for search in [
            RelationSearch::System(Encoding::Partitioned3),
            RelationSearch::System(Encoding::Unordered3),
            RelationSearch::System(Encoding::Line3),
            RelationSearch::Hybrid(Encoding::Partitioned3),
            RelationSearch::Cover3,
        ] {
            let (report, recovered) =
                find_relation(&curve, g, q, &base, search, 24, Duration::from_secs(20)).unwrap();
            let recovered = recovered.unwrap_or_else(|| panic!("no relation for {search:?}"));
            assert_eq!(recovered.logarithm, d);
            assert_ne!(recovered.b, 0);
            assert_eq!(curve.mul(Some(g), recovered.logarithm), Some(q));
            assert!(report.stopped_early && !report.complete && !report.timed_out);
            assert_eq!(report.coordinate_sets.len(), 1);
        }
        let attempts = solve_dlp_with_mode(
            &curve,
            g,
            q,
            Encoding::Line3,
            Labels::Guaranteed,
            2,
            20261026,
            32,
            Duration::from_secs(20),
            true,
        )
        .unwrap();
        assert_eq!(
            attempts
                .last()
                .unwrap()
                .recovery
                .as_ref()
                .unwrap()
                .logarithm,
            d
        );
        assert!(attempts.last().unwrap().algebra.stopped_early);
    }

    #[test]
    fn horizontal_lines_are_kept_unless_propagated_orbit_labels_certify_pruning() {
        let curve = FastCurve::new("aps-j0-p103", 103, 0, 5, 97);
        let g = curve.point(2, 42);
        let q = curve.mul(Some(g), 17).unwrap();
        let seeds = sample_base(&curve, g, q, &[1, 1, 1], Labels::Random, 20261026, 1000).unwrap();
        let base = orbit_base(&curve, g, q, &seeds, 56, 35).unwrap();
        let values: Vec<_> = base.points.keys().copied().collect();
        let full = build_system(&curve, std::slice::from_ref(&values), Encoding::Line3).unwrap();
        let pruned = build_pruned_line_system(&curve, g, q, &base, 56, 35).unwrap();
        assert_eq!(pruned.variables(), 3);
        assert!(pruned.domains.iter().all(Option::is_none));
        // Exhaustively evaluate parameter-space equations as a second,
        // finite oracle, independent of F4's root extraction and branching.
        let mut full_sets = BTreeSet::new();
        let mut pruned_sets = BTreeSet::new();
        for t in 0..103 {
            for c in 0..103 {
                if full
                    .equations
                    .iter()
                    .all(|f| f4_fp::eval(f, &[t, c], 103) == 0)
                {
                    let coordinates = decode_root(&full, &[t, c]).unwrap();
                    full_sets.insert(coordinates.clone());
                    if t != 0 {
                        let root = [t, c, inv_mod(t, 103)];
                        assert!(pruned
                            .equations
                            .iter()
                            .all(|f| f4_fp::eval(f, &root, 103) == 0));
                        pruned_sets.insert(coordinates);
                    } else {
                        assert!(recover(&curve, g, q, &base, &[coordinates])
                            .unwrap()
                            .is_none());
                    }
                }
            }
        }
        let point_sets: BTreeSet<_> = all_triples(&values)
            .iter()
            .flat_map(|p| direct_sets(&curve, p))
            .collect();
        assert_eq!(full_sets, point_sets);
        assert_eq!(full_sets.len() - pruned_sets.len(), base.points.len() / 3);
        assert!(!pruned_sets.is_empty());
        for coordinates in &full_sets {
            if recover(&curve, g, q, &base, std::slice::from_ref(coordinates))
                .unwrap()
                .is_some()
            {
                assert!(pruned_sets.contains(coordinates));
            }
        }
        let solved = solve_system(&pruned, 103, 24, Duration::from_secs(30)).unwrap();
        assert!(solved.complete);
        assert_eq!(
            solved.coordinate_sets.into_iter().collect::<BTreeSet<_>>(),
            pruned_sets
        );
        let (stream, recovered) = find_relation(
            &curve,
            g,
            q,
            &base,
            RelationSearch::PrunedLine3 {
                beta: 56,
                lambda: 35,
            },
            24,
            Duration::from_secs(30),
        )
        .unwrap();
        assert!(stream.stopped_early && !stream.complete);
        assert_eq!(recovered.unwrap().logarithm, 17);

        // Same geometry, different valid known labels: (u-d,v+1) names
        // the same point because this is a public known-answer fixture.
        // The formerly tautological horizontal relation is now informative.
        let mut independent = base.clone();
        let x = independent.parts[0][0];
        let entry = independent.points.get_mut(&x).unwrap();
        entry.u = (entry.u + 97 - 17) % 97;
        entry.v = (entry.v + 1) % 97;
        validate_base(&curve, g, q, &independent).unwrap();
        assert_eq!(
            recover(
                &curve,
                g,
                q,
                &independent,
                std::slice::from_ref(&independent.parts[0])
            )
            .unwrap()
            .unwrap()
            .logarithm,
            17
        );
        assert!(build_pruned_line_system(&curve, g, q, &independent, 56, 35).is_err());
        assert!(build_pruned_line_system(&curve, g, q, &base, 56, 36).is_err());
        let mut unclosed = base.clone();
        let removed = unclosed.parts[0].pop().unwrap();
        unclosed.points.remove(&removed);
        assert!(build_pruned_line_system(&curve, g, q, &unclosed, 56, 35).is_err());
    }

    #[test]
    fn streaming_passes_over_uninformative_roots_and_reports_exhaustion_honestly() {
        let curve = FastCurve::new("aps-j0-p103", 103, 0, 5, 97);
        let g = curve.point(2, 42);
        let q = curve.mul(Some(g), 17).unwrap();
        let seeds = sample_base(&curve, g, q, &[1], Labels::Random, 24, 100).unwrap();
        let base = orbit_base(&curve, g, q, &seeds, 56, 35).unwrap();
        let (report, recovery) = find_relation(
            &curve,
            g,
            q,
            &base,
            RelationSearch::System(Encoding::Line3),
            24,
            Duration::from_secs(10),
        )
        .unwrap();
        assert!(recovery.is_none());
        assert!(report.complete && !report.stopped_early && !report.timed_out);
        assert_eq!(report.coordinate_sets.len(), 1);
        let full_seeds =
            sample_base(&curve, g, q, &[1, 1, 1], Labels::Random, 20261026, 1000).unwrap();
        let full = orbit_base(&curve, g, q, &full_seeds, 56, 35).unwrap();
        let (report, recovery) = find_relation(
            &curve,
            g,
            q,
            &full,
            RelationSearch::System(Encoding::Line3),
            24,
            Duration::from_secs(20),
        )
        .unwrap();
        assert_eq!(recovery.unwrap().logarithm, 17);
        assert!(report.stopped_early && !report.complete);
        assert!(!report.coordinate_sets.is_empty());
        let (timeout, none) = find_relation(
            &curve,
            g,
            q,
            &full,
            RelationSearch::System(Encoding::Line3),
            24,
            Duration::from_nanos(1),
        )
        .unwrap();
        assert!(none.is_none() && timeout.timed_out && !timeout.complete && !timeout.stopped_early);
    }

    #[test]
    fn a_mixed_stream_continues_after_a_deliberately_uninformative_first_relation() {
        let (curve, g, q, d) = fixture();
        let mut base = positive_test_base(&curve, g, q);
        let system = build_system(
            &curve,
            &grouped_parts(&base, Encoding::Line3),
            Encoding::Line3,
        )
        .unwrap();
        let mut order = Vec::new();
        let full = solve_system_visit(&system, 101, 24, Duration::from_secs(20), |coordinates| {
            order.push(coordinates.to_vec());
            Ok(false)
        })
        .unwrap();
        assert!(full.complete);
        assert!(order.len() > 1);
        let later_x = order
            .iter()
            .skip(1)
            .flatten()
            .find(|x| !order[0].contains(x))
            .copied()
            .unwrap();
        // Public-fixture label construction is confined to this test.
        // Every point remains unchanged. The first relation has B=0,
        // while a later relation containing later_x has B=+/-1.
        for entry in base.points.values_mut() {
            entry.u = (entry.u + mul(d, entry.v, curve.n)) % curve.n;
            entry.v = 0;
        }
        let entry = base.points.get_mut(&later_x).unwrap();
        entry.u = (entry.u + curve.n - d) % curve.n;
        entry.v = 1;
        validate_base(&curve, g, q, &base).unwrap();
        assert!(
            recover(&curve, g, q, &base, std::slice::from_ref(&order[0]))
                .unwrap()
                .is_none()
        );
        let (report, recovered) = find_relation(
            &curve,
            g,
            q,
            &base,
            RelationSearch::System(Encoding::Line3),
            24,
            Duration::from_secs(20),
        )
        .unwrap();
        assert_eq!(recovered.unwrap().logarithm, d);
        assert!(report.stopped_early && !report.complete && !report.timed_out);
        assert!(report.coordinate_sets.len() >= 2);
    }

    #[test]
    fn separating_family_covers_every_triple_without_inspecting_relations() {
        for size in (3..=32).chain([64]) {
            let family = triple_cover(size).unwrap();
            for i in 0..size {
                for j in i + 1..size {
                    for k in j + 1..size {
                        assert!(
                            family.iter().any(|parts| {
                                let colors: Vec<_> = [i, j, k]
                                    .iter()
                                    .map(|x| parts.iter().position(|p| p.contains(x)).unwrap())
                                    .collect();
                                colors[0] != colors[1]
                                    && colors[0] != colors[2]
                                    && colors[1] != colors[2]
                            }),
                            "uncovered {size}: {i},{j},{k}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn covering_system_union_equals_unordered_point_relation_set() {
        let (curve, g, q, _) = fixture();
        let base = sample_base(&curve, g, q, &[2, 2, 2], Labels::Guaranteed, 717, 1000).unwrap();
        let values: Vec<_> = base.points.keys().copied().collect();
        let want: BTreeSet<_> = all_triples(&values)
            .iter()
            .flat_map(|parts| direct_sets(&curve, parts))
            .collect();
        let mut got = BTreeSet::new();
        for partition in triple_cover(values.len()).unwrap() {
            let parts: Vec<Vec<u64>> = partition
                .iter()
                .map(|part| part.iter().map(|&i| values[i]).collect())
                .collect();
            got.extend(solve(&curve, &parts, Encoding::Partitioned3).coordinate_sets);
        }
        assert_eq!(got, want);
        let cover = solve_cover(&curve, &values, 24, Duration::from_secs(10)).unwrap();
        assert!(cover.complete);
        assert_eq!(
            cover.coordinate_sets.into_iter().collect::<BTreeSet<_>>(),
            want
        );
    }

    #[test]
    fn expanded_split_and_hybrid_s4_have_identical_complete_point_solutions() {
        let (curve, g, q, _) = fixture();
        let base = sample_base(&curve, g, q, &[2, 2, 2, 2], Labels::Guaranteed, 890, 1000).unwrap();
        let want = direct_sets(&curve, &base.parts);
        for encoding in [Encoding::Expanded4, Encoding::Split4] {
            assert_eq!(solve(&curve, &base.parts, encoding).coordinate_sets, want);
            let hybrid =
                solve_hybrid(&curve, &base.parts, encoding, 24, Duration::from_secs(10)).unwrap();
            assert!(hybrid.complete);
            assert_eq!(hybrid.coordinate_sets, want);
        }
    }

    #[test]
    fn two_pair_divisors_match_unordered_pair_choices() {
        let (curve, g, q, _) = fixture();
        let base = sample_base(&curve, g, q, &[3, 3], Labels::Random, 333, 1000).unwrap();
        let mut want = BTreeSet::new();
        for i in 0..3 {
            for j in i + 1..3 {
                for k in 0..3 {
                    for l in k + 1..3 {
                        want.extend(direct_sets(
                            &curve,
                            &[
                                vec![base.parts[0][i]],
                                vec![base.parts[0][j]],
                                vec![base.parts[1][k]],
                                vec![base.parts[1][l]],
                            ],
                        ));
                    }
                }
            }
        }
        assert_eq!(
            solve(&curve, &base.parts, Encoding::BlockSymmetric4).coordinate_sets,
            want.into_iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn solver_recovers_target_without_receiving_the_planted_scalar() {
        for (p, a, b, n, gx, gy, d) in [
            (101, 59, 6, 107, 78, 4, 37),
            (211, 166, 158, 227, 155, 121, 73),
        ] {
            let curve = FastCurve::new("aps-public-fixture", p, a, b, n);
            let g = curve.point(gx, gy);
            let q = curve.mul(Some(g), d).unwrap();
            let runs = solve_dlp(
                &curve,
                g,
                q,
                Encoding::Partitioned3,
                Labels::Guaranteed,
                3,
                20261026,
                32,
                Duration::from_secs(10),
            )
            .unwrap();
            assert_eq!(
                runs.last()
                    .and_then(|r| r.recovery.as_ref())
                    .unwrap()
                    .logarithm,
                d
            );
            for run in runs {
                assert!(run.algebra.complete);
                assert_eq!(
                    run.algebra.coordinate_sets,
                    direct_sets(&curve, &run.base.parts)
                );
            }
        }
    }

    #[test]
    fn every_encoding_recovers_a_logarithm_through_the_same_checked_relation_path() {
        let (curve, g, q, d) = fixture();
        for encoding in [
            Encoding::Partitioned3,
            Encoding::Unordered3,
            Encoding::Expanded4,
            Encoding::Split4,
            Encoding::BlockSymmetric4,
        ] {
            let attempts = solve_dlp(
                &curve,
                g,
                q,
                encoding,
                Labels::Guaranteed,
                2,
                20261026,
                32,
                Duration::from_secs(10),
            )
            .unwrap();
            assert_eq!(
                attempts
                    .last()
                    .and_then(|r| r.recovery.as_ref())
                    .map(|r| r.logarithm),
                Some(d),
                "{encoding:?}",
            );
        }
        let base = sample_base(&curve, g, q, &[2, 2, 2], Labels::Guaranteed, 717, 1000).unwrap();
        let values: Vec<_> = base.points.keys().copied().collect();
        let cover = solve_cover(&curve, &values, 24, Duration::from_secs(10)).unwrap();
        assert!(cover.complete);
        assert_eq!(
            recover(&curve, g, q, &base, &cover.coordinate_sets)
                .unwrap()
                .unwrap()
                .logarithm,
            d
        );
    }

    #[test]
    fn automorphism_orbits_preserve_sparse_constraints_and_labels_but_tautologies_reveal_nothing() {
        let curve = FastCurve::new("aps-j0-p103", 103, 0, 5, 97);
        let g = curve.point(2, 42);
        let q = curve.mul(Some(g), 17).unwrap();
        let seeds = sample_base(&curve, g, q, &[1, 1, 1], Labels::Random, 20261026, 1000).unwrap();
        let base = orbit_base(&curve, g, q, &seeds, 56, 35).unwrap();
        for part in &base.parts {
            let poly = vanishing_coefficients(part, 103).unwrap();
            assert_eq!(poly.len(), 4);
            assert_eq!(poly.iter().filter(|&&c| c != 0).count(), 2);
            assert!(recover(&curve, g, q, &base, std::slice::from_ref(part))
                .unwrap()
                .is_none());
        }
        let solutions = solve(&curve, &base.parts, Encoding::Partitioned3).coordinate_sets;
        assert_eq!(solutions, direct_sets(&curve, &base.parts));
        assert_eq!(
            recover(&curve, g, q, &base, &solutions)
                .unwrap()
                .unwrap()
                .logarithm,
            17
        );
        assert!(orbit_base(&curve, g, q, &seeds, 56, 36).is_err());
        assert!(orbit_base(&curve, g, q, &seeds, 1, 35).is_err());
    }

    #[test]
    fn invalid_instances_labels_overlaps_and_budgets_fail_closed() {
        let (curve, g, q, _) = fixture();
        let bad = FastCurve::new("wrong-order", 101, 59, 6, 109);
        assert!(validate_instance(&bad, g, q).is_err());
        let mut base = sample_base(&curve, g, q, &[2, 2, 2], Labels::Guaranteed, 40, 1000).unwrap();
        let x = *base.points.keys().next().unwrap();
        base.points.get_mut(&x).unwrap().v = (base.points[&x].v + 1) % curve.n;
        assert!(recover(&curve, g, q, &base, &[]).is_err());
        assert!(build_system(
            &curve,
            &[vec![78], vec![78], vec![2]],
            Encoding::Partitioned3
        )
        .is_err());
        assert!(sample_base(&curve, g, q, &[2, 2, 2], Labels::Random, 1, 0).is_err());
        assert!(solve_dlp(
            &curve,
            g,
            q,
            Encoding::Partitioned3,
            Labels::Guaranteed,
            0,
            1,
            1,
            Duration::from_secs(1)
        )
        .is_err());
        assert!(triple_cover(2).is_err());
    }

    #[test]
    fn cover_and_hybrid_deadlines_never_report_an_incomplete_search_as_complete() {
        let (curve, g, q, _) = fixture();
        let base = sample_base(&curve, g, q, &[2, 2, 2], Labels::Guaranteed, 717, 1000).unwrap();
        let values: Vec<_> = base.points.keys().copied().collect();
        for report in [
            solve_cover(&curve, &values, 24, Duration::from_nanos(1)).unwrap(),
            solve_hybrid(
                &curve,
                &base.parts,
                Encoding::Partitioned3,
                24,
                Duration::from_nanos(1),
            )
            .unwrap(),
        ] {
            assert!(!report.complete);
            assert!(report.timed_out);
        }
        assert!(solve_cover(&curve, &values, 24, Duration::ZERO).is_err());
        assert!(solve_hybrid(
            &curve,
            &base.parts,
            Encoding::Partitioned3,
            24,
            Duration::ZERO
        )
        .is_err());
    }

    #[test]
    fn symmetric_identity_certificate_is_independently_replayed() {
        use super::super::identity_certificate as ic;
        let cert = ic::issue(
            symmetric_s3_identity(),
            20261026,
            32,
            "native constructor; independent domain-separated checker seed",
            None,
        )
        .unwrap();
        assert!(matches!(
            ic::check_with_extra(&cert, Some((9001, 32))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
        let mut forged = cert.clone();
        forged.statement.degree_bound = 1;
        assert!(ic::check(&forged).is_err());
    }
}
