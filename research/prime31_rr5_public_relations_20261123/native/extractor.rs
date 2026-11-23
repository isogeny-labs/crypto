//! Four distinct-x signed leaves, extracted from a public Riemann--Roch fibre.
//!
//! This specializes the repository RR9 identity to pole order five. Neither
//! construction nor extraction enumerates factor-base pairs or tuples. F4's
//! fallback branches are refused before their first specialization. A degree,
//! time, output or construction cap is explicitly incomplete, never UNSAT.

use crate::relation_adapter::SignedRelation;
use crypto_lib::cryptanalysis::f4_fp::{
    normalise, solve_visit_traced, F4Options, Ordering, Poly, VisitEvent, VisitReport,
};
use crypto_lib::cryptanalysis::glv_invariant_base::poly_roots_fp_counted_until;
use prime31_algebraic_ic::{Arithmetic, Counts, Curve, Point};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

type MPoly = BTreeMap<[u32; 3], u64>;

/// Public polynomial presentation and monomial order. Selection changes no
/// chart assumptions, input data, root-factoring backend, or fallback policy.
/// The existing public APIs retain their historical Lex configurations.
#[derive(Clone, Copy, Debug, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtractionForm {
    #[default]
    ExpandedLex,
    CofactorLex,
    CofactorGrevlex,
}

impl ExtractionForm {
    fn ordering(self) -> Ordering {
        match self {
            Self::ExpandedLex | Self::CofactorLex => Ordering::Lex,
            Self::CofactorGrevlex => Ordering::Grevlex,
        }
    }

    fn presentation(self) -> &'static str {
        match self {
            Self::ExpandedLex => "expanded_remainder",
            Self::CofactorLex | Self::CofactorGrevlex => "monic_cofactor",
        }
    }

    fn monomial_order(self) -> &'static str {
        match self {
            Self::ExpandedLex | Self::CofactorLex => "lex",
            Self::CofactorGrevlex => "grevlex",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_degree: u32,
    pub seconds: u64,
    pub max_polynomial_terms: usize,
    pub max_assignments: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_degree: 5,
            seconds: 5,
            max_polynomial_terms: 20_000,
            max_assignments: 4096,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum Status {
    Complete,
    Empty,
    UnsupportedChart,
    InvalidInput,
    BuildLimit,
    Inconclusive,
    Deadline,
    OutputLimit,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Diagnostics {
    pub base_size: usize,
    pub presentation: String,
    pub monomial_order: String,
    pub input_variables: usize,
    pub input_polynomials: usize,
    pub input_terms: usize,
    pub input_max_degree: u32,
    pub symbolic_additions: u64,
    pub symbolic_multiplications: u64,
    pub root_multiplications: u64,
    pub assignments_extracted: usize,
    pub accepted_relations: usize,
    pub rejected_assignments: usize,
    pub forbidden_branches_executed: usize,
    pub forbidden_fallbacks_refused: usize,
    pub univariate_branches_observed: usize,
    pub curve_work: Counts,
    pub phase_accounting_complete: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct ResultRow {
    pub status: Status,
    pub reason: String,
    pub relations: Vec<SignedRelation>,
    pub diagnostics: Diagnostics,
    pub traversal: Option<VisitReport>,
}

struct Builder {
    p: u64,
    deadline: Instant,
    term_cap: usize,
    adds: u64,
    muls: u64,
}

impl Builder {
    fn am(&mut self, a: u64, b: u64) -> u64 {
        self.adds += 1;
        (a + b) % self.p
    }
    fn mm(&mut self, a: u64, b: u64) -> u64 {
        self.muls += 1;
        ((a as u128 * b as u128) % self.p as u128) as u64
    }
    fn guard(&self, terms: usize) -> Result<(), String> {
        if Instant::now() >= self.deadline {
            return Err("construction deadline".into());
        }
        if terms > self.term_cap {
            return Err("symbolic term cap".into());
        }
        Ok(())
    }
    fn c(&self, x: u64) -> MPoly {
        if x % self.p == 0 {
            MPoly::new()
        } else {
            BTreeMap::from([([0; 3], x % self.p)])
        }
    }
    fn var(&self, i: usize) -> MPoly {
        let mut e = [0; 3];
        e[i] = 1;
        BTreeMap::from([(e, 1)])
    }
    fn add(&mut self, a: &MPoly, b: &MPoly) -> Result<MPoly, String> {
        let mut out = a.clone();
        for (i, (&e, &c)) in b.iter().enumerate() {
            if i & 255 == 0 {
                self.guard(out.len())?;
            }
            let v = self.am(out.get(&e).copied().unwrap_or(0), c);
            if v == 0 {
                out.remove(&e);
            } else {
                out.insert(e, v);
            }
        }
        self.guard(out.len())?;
        Ok(out)
    }
    fn scale(&mut self, a: &MPoly, c: u64) -> Result<MPoly, String> {
        let mut out = MPoly::new();
        for (i, (&e, &v)) in a.iter().enumerate() {
            if i & 255 == 0 {
                self.guard(out.len())?;
            }
            let w = self.mm(v, c);
            if w != 0 {
                out.insert(e, w);
            }
        }
        self.guard(out.len())?;
        Ok(out)
    }
    fn mul(&mut self, a: &MPoly, b: &MPoly) -> Result<MPoly, String> {
        let mut out = MPoly::new();
        for (&e, &c) in a {
            self.guard(out.len())?;
            for (i, (&f, &d)) in b.iter().enumerate() {
                if i & 255 == 0 {
                    self.guard(out.len())?;
                }
                let g = [e[0] + f[0], e[1] + f[1], e[2] + f[2]];
                let cd = self.mm(c, d);
                let v = self.am(out.get(&g).copied().unwrap_or(0), cd);
                if v == 0 {
                    out.remove(&g);
                } else {
                    out.insert(g, v);
                }
            }
        }
        self.guard(out.len())?;
        Ok(out)
    }
    fn conv(&mut self, a: &[MPoly], b: &[MPoly]) -> Result<Vec<MPoly>, String> {
        let mut out = vec![MPoly::new(); a.len() + b.len() - 1];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                let xy = self.mul(x, y)?;
                out[i + j] = self.add(&out[i + j], &xy)?;
            }
        }
        Ok(out)
    }
}

fn mm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut z = 1;
    while n > 0 {
        if n & 1 != 0 {
            z = mm(z, a, p)
        }
        a = mm(a, a, p);
        n >>= 1;
    }
    z
}
fn eval(f: &MPoly, point: &[u64; 3], p: u64) -> u64 {
    f.iter().fold(0, |acc, (e, c)| {
        let mut v = *c;
        for j in 0..3 {
            v = mm(v, pow(point[j], e[j] as u64, p), p)
        }
        (acc + v) % p
    })
}

/// Public coefficients of U and the four coefficients of h mod U.
fn quartic(builder: &mut Builder, e: Curve, q: (u64, u64)) -> Result<Vec<MPoly>, String> {
    let (r, s) = q;
    let v = vec![builder.var(0), builder.c(1)];
    let a0 = vec![builder.var(2), builder.var(1)];
    let vv = builder.conv(&v, &v)?;
    let aa = builder.conv(&a0, &a0)?;
    let av = builder.conv(&a0, &v)?;
    let rr = builder.mm(r, r);
    let ra = builder.am(rr, e.a);
    let factor = vec![builder.c(ra), builder.c(r), builder.c(1)];
    let mut u = builder.conv(&factor, &vv)?;
    let second = builder.conv(&vec![builder.c((e.p - r) % e.p), builder.c(1)], &aa)?;
    for (i, f) in second.iter().enumerate() {
        let minus = builder.scale(f, e.p - 1)?;
        u[i] = builder.add(&u[i], &minus)?;
    }
    for (i, f) in av.iter().enumerate() {
        let minus = builder.scale(f, (e.p - 2 * s % e.p) % e.p)?;
        u[i] = builder.add(&u[i], &minus)?;
    }
    assert_eq!(u[4], builder.c(1));
    Ok(u)
}

fn selector(builder: &mut Builder, xs: &[u64]) -> Result<Vec<u64>, String> {
    // h is built in O(B^2) scalar work, with no tuple construction.
    let mut h = vec![1u64];
    for &x in xs {
        builder.guard(0)?;
        let mut next = vec![0; h.len() + 1];
        for (i, &c) in h.iter().enumerate() {
            if i & 255 == 0 {
                builder.guard(0)?;
            }
            let z = builder.mm(c, (builder.p - x) % builder.p);
            next[i] = builder.am(next[i], z);
            next[i + 1] = builder.am(next[i + 1], c);
        }
        h = next;
    }
    Ok(h)
}

fn system(
    builder: &mut Builder,
    e: Curve,
    q: (u64, u64),
    xs: &[u64],
) -> Result<(Vec<MPoly>, Vec<MPoly>), String> {
    let u = quartic(builder, e, q)?;
    let h = selector(builder, xs)?;
    // Horner in F_p[b,c,d][X]/U, degree X <4 throughout.
    let mut rem = vec![MPoly::new(); 4];
    for &c in h.iter().rev() {
        let lead = rem[3].clone();
        let mut next = vec![builder.c(c), rem[0].clone(), rem[1].clone(), rem[2].clone()];
        for j in 0..4 {
            let z = builder.mul(&lead, &u[j])?;
            let neg = builder.scale(&z, e.p - 1)?;
            next[j] = builder.add(&next[j], &neg)?;
        }
        rem = next;
    }
    Ok((u, rem))
}

/// Exact low-degree lift: U(X) * W(X) = h(X), with monic W of degree B-4.
/// Auxiliary variables are coefficients, not enumerated leaf choices. This
/// removes symbolic remainder expansion, while increasing the number of
/// variables to B-1. It makes no claim about the subsequent F4 complexity.
fn cofactor_system(
    builder: &mut Builder,
    e: Curve,
    q: (u64, u64),
    xs: &[u64],
) -> Result<(Vec<MPoly>, Vec<Poly>, usize), String> {
    cofactor_system_ordered(builder, e, q, xs, Ordering::Lex)
}

fn cofactor_system_ordered(
    builder: &mut Builder,
    e: Curve,
    q: (u64, u64),
    xs: &[u64],
    ordering: Ordering,
) -> Result<(Vec<MPoly>, Vec<Poly>, usize), String> {
    let u = quartic(builder, e, q)?;
    let h = selector(builder, xs)?;
    let wd = xs.len() - 4;
    let nvars = 3 + wd;
    let mut equations = Vec::with_capacity(xs.len());
    for k in 0..xs.len() {
        builder.guard(0)?;
        let mut terms = vec![(vec![0u32; nvars], (e.p - h[k]) % e.p)];
        for j in 0..=4 {
            if k < j || k - j > wd {
                continue;
            }
            let wi = k - j;
            for (exponent, coefficient) in &u[j] {
                let mut ex = vec![0u32; nvars];
                ex[..3].copy_from_slice(exponent);
                if wi < wd {
                    ex[3 + wi] = 1;
                }
                terms.push((ex, *coefficient));
            }
        }
        let polynomial = normalise(&terms, e.p, ordering);
        builder.guard(polynomial.len())?;
        equations.push(polynomial);
    }
    Ok((u, equations, nvars))
}

fn is_univariate(g: &Poly) -> bool {
    let mut used = BTreeSet::new();
    for (e, c) in g {
        if *c != 0 {
            for (j, k) in e.iter().enumerate() {
                if *k > 0 {
                    used.insert(j);
                }
            }
        }
    }
    used.len() == 1
}
fn is_nonzero_constant(g: &Poly) -> bool {
    !g.is_empty() && g.iter().all(|(e, _)| e.iter().all(|k| *k == 0))
}

/// No scalar or decomposition hint is accepted by this interface.
pub fn extract(e: Curve, base: &[(u64, u64)], q: Point, limits: Limits) -> ResultRow {
    extract_until(
        e,
        base,
        q,
        limits,
        Instant::now() + Duration::from_secs(limits.seconds),
    )
}

/// Public-only coefficient lift of the same distinct-abscissa RR5 chart.
pub fn extract_cofactor(e: Curve, base: &[(u64, u64)], q: Point, limits: Limits) -> ResultRow {
    extract_form(
        e,
        base,
        q,
        limits,
        Instant::now() + Duration::from_secs(limits.seconds),
        ExtractionForm::CofactorLex,
    )
}

/// Select an explicit presentation/order without supplying coordinate domains,
/// decomposition witnesses, or hidden logarithms. Grevlex is an alternative
/// elimination configuration, not a claim of lower cost or guaranteed success.
pub fn extract_with_form(
    e: Curve,
    base: &[(u64, u64)],
    q: Point,
    limits: Limits,
    form: ExtractionForm,
) -> ResultRow {
    extract_with_form_until(
        e,
        base,
        q,
        limits,
        Instant::now() + Duration::from_secs(limits.seconds),
        form,
    )
}

/// An absolute cooperative deadline includes input validation and construction.
/// Native calls retain their own deadline checks; a process limit is still
/// required for a hard wall/RSS bound.
pub(crate) fn extract_until(
    e: Curve,
    base: &[(u64, u64)],
    q: Point,
    limits: Limits,
    outer_deadline: Instant,
) -> ResultRow {
    extract_form(
        e,
        base,
        q,
        limits,
        outer_deadline,
        ExtractionForm::ExpandedLex,
    )
}

/// The selected form uses the same outer deadline, validation and status
/// precedence as the historical expanded/Lex solver path.
pub(crate) fn extract_with_form_until(
    e: Curve,
    base: &[(u64, u64)],
    q: Point,
    limits: Limits,
    outer_deadline: Instant,
    form: ExtractionForm,
) -> ResultRow {
    extract_form(e, base, q, limits, outer_deadline, form)
}

fn extract_form(
    e: Curve,
    base: &[(u64, u64)],
    q: Point,
    limits: Limits,
    outer_deadline: Instant,
    form: ExtractionForm,
) -> ResultRow {
    let deadline = outer_deadline.min(Instant::now() + Duration::from_secs(limits.seconds));
    let ordering = form.ordering();
    let mut result = ResultRow {
        status: Status::InvalidInput,
        reason: String::new(),
        relations: vec![],
        diagnostics: Diagnostics {
            base_size: base.len(),
            presentation: form.presentation().into(),
            monomial_order: form.monomial_order().into(),
            ..Diagnostics::default()
        },
        traversal: None,
    };
    if Instant::now() >= deadline {
        result.status = Status::Deadline;
        result.reason = "deadline before input validation".into();
        return result;
    }
    let mut ar = Arithmetic::default();
    if e.p < 5
        || e.p >= 1u64 << 32
        || e.a >= e.p
        || e.b >= e.p
        || limits.seconds == 0
        || limits.max_assignments == 0
    {
        result.reason = "invalid field/limits".into();
        return result;
    }
    if !crypto_lib::cryptanalysis::residual_walk::is_prime_u64(e.p) {
        result.reason = "field modulus must be prime".into();
        return result;
    }
    if (mm(4, mm(mm(e.a, e.a, e.p), e.a, e.p), e.p) + mm(27, mm(e.b, e.b, e.p), e.p)) % e.p == 0 {
        result.reason = "singular curve".into();
        return result;
    }
    let Some(target) = q else {
        result.status = Status::UnsupportedChart;
        result.reason = "identity target chart not implemented".into();
        return result;
    };
    let mut index = BTreeMap::new();
    for (i, &(x, y)) in base.iter().enumerate() {
        if Instant::now() >= deadline {
            result.status = Status::Deadline;
            result.reason = "input validation deadline".into();
            result.diagnostics.curve_work = ar.counts;
            return result;
        }
        if y == 0 || !ar.on_curve(e, Some((x, y))) || index.insert(x, (i, y)).is_some() {
            result.reason = "base must have distinct valid nonzero-y abscissae".into();
            result.diagnostics.curve_work = ar.counts;
            return result;
        }
    }
    if !ar.on_curve(e, q) {
        result.reason = "invalid public target".into();
        result.diagnostics.curve_work = ar.counts;
        return result;
    }
    if index.contains_key(&target.0) {
        result.status = Status::UnsupportedChart;
        result.reason = "target-in-base chart not implemented".into();
        result.diagnostics.curve_work = ar.counts;
        return result;
    }
    if base.len() < 4 {
        result.status = Status::Empty;
        result.reason = "four distinct abscissae require B>=4".into();
        result.diagnostics.curve_work = ar.counts;
        return result;
    }
    let mut b = Builder {
        p: e.p,
        deadline,
        term_cap: limits.max_polynomial_terms,
        adds: 0,
        muls: 0,
    };
    let xs: Vec<_> = base.iter().map(|p| p.0).collect();
    let built = match form {
        ExtractionForm::CofactorLex => cofactor_system(&mut b, e, target, &xs),
        ExtractionForm::CofactorGrevlex => {
            cofactor_system_ordered(&mut b, e, target, &xs, ordering)
        }
        ExtractionForm::ExpandedLex => system(&mut b, e, target, &xs).map(|(u, remainders)| {
            let equations: Vec<Poly> = remainders
                .iter()
                .map(|f| {
                    normalise(
                        &f.iter().map(|(e, c)| (e.to_vec(), *c)).collect::<Vec<_>>(),
                        e.p,
                        ordering,
                    )
                })
                .collect();
            (u, equations, 3)
        }),
    };
    result.diagnostics.symbolic_additions = b.adds;
    result.diagnostics.symbolic_multiplications = b.muls;
    let (u, equations, nvars) = match built {
        Ok(v) => v,
        Err(why) => {
            result.status = if Instant::now() >= deadline {
                Status::Deadline
            } else {
                Status::BuildLimit
            };
            result.reason = why;
            result.diagnostics.curve_work = ar.counts;
            return result;
        }
    };
    result.diagnostics.input_variables = nvars;
    result.diagnostics.input_polynomials = equations.len();
    result.diagnostics.input_terms = equations.iter().map(Vec::len).sum();
    result.diagnostics.input_max_degree = equations
        .iter()
        .flat_map(|f| f.iter().map(|(e, _)| e.iter().sum()))
        .max()
        .unwrap_or(0);
    let mut opts = F4Options::new(ordering, limits.max_degree);
    opts.deadline = Some(deadline);
    opts.visit_timing = false;
    let mut assignments = Vec::new();
    let mut output_cap = false;
    let mut refused = 0usize;
    let mut root_branches = 0usize;
    let traversal = solve_visit_traced(
        &equations,
        nvars,
        e.p,
        &opts,
        &vec![None; nvars],
        |a| {
            if assignments.len() >= limits.max_assignments {
                output_cap = true;
                return true;
            }
            assignments.push([a[0], a[1], a[2]]);
            false
        },
        |event| {
            match event {
                VisitEvent::NativeResult {
                    report,
                    generating_set,
                    ..
                } if !report.timed_out
                    && !report.inconsistent
                    && Instant::now() < deadline
                    && !generating_set.iter().any(is_nonzero_constant)
                    && !generating_set.iter().any(is_univariate) =>
                {
                    refused += 1;
                    return Err("no algebraic univariate; enumeration refused".into());
                }
                VisitEvent::Branch { source, .. } if source != "univariate_roots" => {
                    refused += 1;
                    return Err(format!("forbidden enumeration branch {source}"));
                }
                VisitEvent::Branch {
                    source: "univariate_roots",
                    ..
                } => {
                    root_branches += 1;
                }
                _ => {}
            }
            Ok(())
        },
    );
    result.diagnostics.forbidden_fallbacks_refused = refused;
    result.diagnostics.univariate_branches_observed = root_branches;
    let traversal = match traversal {
        Ok(t) => t,
        Err(why) => {
            result.reason = why;
            return result;
        }
    };
    result.diagnostics.assignments_extracted = assignments.len();
    let complete =
        traversal.complete && !traversal.timed_out && !traversal.trace_stopped && !output_cap;
    result.status = if output_cap {
        Status::OutputLimit
    } else if traversal.timed_out {
        Status::Deadline
    } else if traversal.trace_stopped || !complete {
        Status::Inconclusive
    } else {
        Status::Complete
    };
    result.reason = traversal.trace_error.clone().unwrap_or_else(|| {
        if complete {
            "all algebraic branches exhausted".into()
        } else {
            "incomplete algebraic traversal".into()
        }
    });
    result.traversal = Some(traversal);
    let mut distinct = BTreeSet::new();
    for a in assignments {
        let coeff: Vec<_> = u.iter().map(|f| eval(f, &a, e.p)).collect();
        let roots = poly_roots_fp_counted_until(
            &coeff,
            e.p,
            0x5252355f524f4f54,
            &mut result.diagnostics.root_multiplications,
            Some(deadline),
        );
        let Some(roots) = roots else {
            result.status = Status::Deadline;
            result.reason = "quartic root extraction deadline".into();
            break;
        };
        if roots.len() != 4 {
            result.diagnostics.rejected_assignments += 1;
            continue;
        }
        let mut terms = Vec::new();
        let mut sum = None;
        let mut valid = true;
        for x in roots {
            let Some(&(idx, canonical_y)) = index.get(&x) else {
                valid = false;
                break;
            };
            let den = (x + a[0]) % e.p;
            if den == 0 {
                valid = false;
                break;
            }
            let a0 = (mm(a[1], x, e.p) + a[2]) % e.p;
            let fraction = mm(
                mm((x + e.p - target.0) % e.p, a0, e.p),
                pow(den, e.p - 2, e.p),
                e.p,
            );
            let y = (2 * e.p - target.1 - fraction) % e.p;
            let sign = if y == canonical_y {
                1
            } else if y == (e.p - canonical_y) % e.p {
                -1
            } else {
                valid = false;
                break;
            };
            if !ar.on_curve(e, Some((x, y))) {
                valid = false;
                break;
            }
            sum = ar.add(e, sum, Some((x, y)));
            terms.push((idx, sign));
        }
        terms.sort_unstable();
        if valid && sum == q && terms.len() == 4 {
            if distinct.insert(terms.clone()) {
                result.relations.push(SignedRelation { terms });
            }
        } else {
            result.diagnostics.rejected_assignments += 1;
        }
    }
    if result.diagnostics.rejected_assignments > 0
        && !matches!(result.status, Status::Deadline | Status::OutputLimit)
    {
        result.status = Status::Inconclusive;
        result.reason = "an algebraic assignment failed signed chart replay".into();
    } else if result.status == Status::Complete && result.relations.is_empty() {
        result.status = Status::Empty;
        result.reason = "empty distinct-abscissa RR5 chart".into();
    }
    result.diagnostics.accepted_relations = result.relations.len();
    result.diagnostics.curve_work = ar.counts;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use prime31_algebraic_ic::PRIME31;

    pub fn public_base(e: Curve, n: usize) -> Vec<(u64, u64)> {
        let mut ar = Arithmetic::default();
        let mut base = Vec::new();
        for x in 0..e.p {
            let rhs = ar.rhs(e, x);
            if let Some(y) = ar.sqrt(rhs, e.p) {
                if y != 0 {
                    base.push((x, y.min(e.p - y)));
                }
            }
            if base.len() == n {
                break;
            }
        }
        base
    }
    fn sum(e: Curve, base: &[(u64, u64)], terms: &[(usize, i64)]) -> Point {
        let mut ar = Arithmetic::default();
        let mut q = None;
        for &(i, sgn) in terms {
            let p = if sgn < 0 {
                ar.neg(e, Some(base[i]))
            } else {
                Some(base[i])
            };
            q = ar.add(e, q, p);
        }
        q
    }
    fn brute(e: Curve, base: &[(u64, u64)], q: Point) -> BTreeSet<Vec<(usize, i64)>> {
        // Independent, test-only complete signed distinct-four oracle.
        let mut out = BTreeSet::new();
        for a in 0..base.len() {
            for b in a + 1..base.len() {
                for c in b + 1..base.len() {
                    for d in c + 1..base.len() {
                        for mask in 0..16 {
                            let t: Vec<_> = [a, b, c, d]
                                .iter()
                                .enumerate()
                                .map(|(j, &i)| (i, if mask >> j & 1 == 0 { 1 } else { -1 }))
                                .collect();
                            if sum(e, base, &t) == q {
                                out.insert(t);
                            }
                        }
                    }
                }
            }
        }
        out
    }
    #[test]
    fn rr5_prime31_public_signed_b4() {
        let e = PRIME31;
        let base = public_base(e, 4);
        let terms = vec![(0, 1), (1, -1), (2, 1), (3, 1)];
        let q = sum(e, &base, &terms);
        let got = extract(e, &base, q, Limits::default());
        println!("RR5_B4_POSITIVE {}", serde_json::to_string(&got).unwrap());
        assert_eq!(got.status, Status::Complete);
        assert!(got.relations.iter().any(|r| r.terms == terms));
        assert_eq!(
            got.relations
                .iter()
                .map(|r| r.terms.clone())
                .collect::<BTreeSet<_>>(),
            brute(e, &base, q)
        );
        assert_eq!(got.traversal.as_ref().unwrap().full_field_branches, 0);
        assert_eq!(got.diagnostics.forbidden_branches_executed, 0);
    }
    #[test]
    fn rr5_prime31_public_empty_b4() {
        let e = PRIME31;
        let base = public_base(e, 4);
        let q = Some(e.g);
        assert!(brute(e, &base, q).is_empty());
        let got = extract(e, &base, q, Limits::default());
        println!("RR5_B4_EMPTY {}", serde_json::to_string(&got).unwrap());
        assert_eq!(got.status, Status::Empty);
        assert!(got.relations.is_empty());
    }
    #[test]
    fn rr5_chart_errors_are_explicit() {
        let e = PRIME31;
        let base = public_base(e, 4);
        assert_eq!(
            extract(e, &base, None, Limits::default()).status,
            Status::UnsupportedChart
        );
        assert_eq!(
            extract(e, &base, Some(base[0]), Limits::default()).status,
            Status::UnsupportedChart
        );
        let mut bad = base.clone();
        bad[3] = bad[0];
        assert_eq!(
            extract(e, &bad, Some(e.g), Limits::default()).status,
            Status::InvalidInput
        );
    }
    #[test]
    fn rr5_degree_cap_cannot_invent_empty() {
        let e = PRIME31;
        let base = public_base(e, 4);
        let q = sum(e, &base, &[(0, 1), (1, -1), (2, 1), (3, 1)]);
        let got = extract(
            e,
            &base,
            q,
            Limits {
                max_degree: 1,
                ..Limits::default()
            },
        );
        println!("RR5_DEGREE_LIMIT {}", serde_json::to_string(&got).unwrap());
        assert_ne!(got.status, Status::Empty);
        assert_eq!(got.diagnostics.forbidden_branches_executed, 0);
    }

    #[test]
    fn rr5_prime31_b8_bounded_positive_and_empty() {
        let e = PRIME31;
        let base = public_base(e, 8);
        let positive = sum(e, &base, &[(0, 1), (2, -1), (4, 1), (7, 1)]);
        for (label, q) in [("positive", positive), ("empty", Some(e.g))] {
            let truth = brute(e, &base, q);
            if label == "positive" {
                assert!(!truth.is_empty());
            } else {
                assert!(truth.is_empty());
            }
            let got = extract(
                e,
                &base,
                q,
                Limits {
                    max_degree: 10,
                    seconds: 5,
                    ..Limits::default()
                },
            );
            println!("RR5_B8_{} {}", label, serde_json::to_string(&got).unwrap());
            assert!(got.relations.iter().all(|r| truth.contains(&r.terms)));
            if matches!(got.status, Status::Complete | Status::Empty) {
                assert_eq!(
                    got.relations
                        .iter()
                        .map(|r| r.terms.clone())
                        .collect::<BTreeSet<_>>(),
                    truth
                );
            }
            assert_ne!(got.status, Status::InvalidInput);
            assert_eq!(got.diagnostics.forbidden_branches_executed, 0);
            if let Some(t) = got.traversal {
                assert_eq!(t.full_field_branches, 0);
                assert_eq!(t.domain_evaluations, 0);
            }
        }
    }

    #[test]
    fn rr5_original_hybrid_b677_preserves_bounded_outcome() {
        let e = PRIME31;
        let base = original_hybrid();
        let q = sum(e, &base, &[(0, 1), (1, -1), (2, 1), (3, 1)]);
        let got = extract(
            e,
            &base,
            q,
            Limits {
                max_degree: 10,
                seconds: 5,
                max_polynomial_terms: 20000,
                ..Limits::default()
            },
        );
        println!("RR5_ORIGINAL_B677 {}", serde_json::to_string(&got).unwrap());
        assert_ne!(
            got.status,
            Status::Empty,
            "known public positive cannot be a complete empty chart"
        );
        assert_ne!(got.status, Status::InvalidInput);
        assert_eq!(got.diagnostics.forbidden_branches_executed, 0);
    }

    fn original_hybrid() -> Vec<(u64, u64)> {
        use prime31_algebraic_ic::Base;
        let e = PRIME31;
        let mut ar = Arithmetic::default();
        let mut base = Base::build(e, 10, &mut ar).unwrap().points;
        assert_eq!(base.len(), 509);
        for line in include_str!("../../prime31_f4_admission_20261121/RUN66_Y7_EXTRA168.csv")
            .lines()
            .filter(|s| !s.is_empty())
        {
            let (x, y) = line.split_once(',').unwrap();
            base.push((x.parse().unwrap(), y.parse().unwrap()));
        }
        for point in &mut base {
            point.1 = point.1.min(e.p - point.1);
        }
        base.sort_unstable();
        base.dedup();
        assert_eq!(base.len(), 677);
        base
    }

    #[test]
    fn rr5_cofactor_b4_and_b8_public_extraction() {
        let e = PRIME31;
        for size in [4, 8] {
            let base = public_base(e, size);
            let terms = if size == 4 {
                vec![(0, 1), (1, -1), (2, 1), (3, 1)]
            } else {
                vec![(0, 1), (2, -1), (4, 1), (7, 1)]
            };
            let q = sum(e, &base, &terms);
            let truth = brute(e, &base, q);
            let got = extract_cofactor(
                e,
                &base,
                q,
                Limits {
                    max_degree: 5,
                    ..Limits::default()
                },
            );
            println!(
                "RR5_COFACTOR_B{} {}",
                size,
                serde_json::to_string(&got).unwrap()
            );
            assert!(got.relations.iter().all(|r| truth.contains(&r.terms)));
            if size == 4 {
                assert_eq!(got.status, Status::Complete);
            }
            if got.status == Status::Complete {
                assert_eq!(
                    got.relations
                        .iter()
                        .map(|r| r.terms.clone())
                        .collect::<BTreeSet<_>>(),
                    truth
                );
            }
            assert_ne!(got.status, Status::Empty);
            if let Some(t) = got.traversal {
                assert_eq!(t.full_field_branches, 0);
                assert_eq!(t.domain_evaluations, 0);
            }
        }
    }

    /// Test-only interpolation from four supplied leaves. The candidate has
    /// no access to this helper or to this known-answer witness.
    fn witness_parameters(
        e: Curve,
        base: &[(u64, u64)],
        terms: &[(usize, i64)],
        q: (u64, u64),
    ) -> [u64; 3] {
        let (r, s) = q;
        let mut rows: Vec<[u64; 4]> = terms
            .iter()
            .map(|&(i, sign)| {
                let (x, cy) = base[i];
                let y = if sign < 0 { e.p - cy } else { cy };
                let sy = (s + y) % e.p;
                let xr = (x + e.p - r) % e.p;
                [sy, mm(x, xr, e.p), xr, (e.p - mm(x, sy, e.p)) % e.p]
            })
            .collect();
        for column in 0..3 {
            let pivot = (column..rows.len())
                .find(|&i| rows[i][column] != 0)
                .expect("unique RR5 chart");
            rows.swap(column, pivot);
            let inv = pow(rows[column][column], e.p - 2, e.p);
            for j in column..4 {
                rows[column][j] = mm(rows[column][j], inv, e.p);
            }
            for i in 0..rows.len() {
                if i == column {
                    continue;
                }
                let a = rows[i][column];
                for j in column..4 {
                    rows[i][j] = (rows[i][j] + e.p - mm(a, rows[column][j], e.p)) % e.p;
                }
            }
        }
        assert!(rows[3].iter().all(|&a| a == 0));
        [rows[0][3], rows[1][3], rows[2][3]]
    }

    #[test]
    fn rr5_cofactor_b677_exact_input_construction() {
        let e = PRIME31;
        let base = original_hybrid();
        let terms = [(0, 1), (1, -1), (2, 1), (3, 1)];
        let q = sum(e, &base, &terms).unwrap();
        let a = witness_parameters(e, &base, &terms, q);
        let mut builder = Builder {
            p: e.p,
            deadline: Instant::now() + Duration::from_secs(5),
            term_cap: 20_000,
            adds: 0,
            muls: 0,
        };
        let xs: Vec<_> = base.iter().map(|p| p.0).collect();
        let (u, equations, nvars) = cofactor_system(&mut builder, e, q, &xs).unwrap();
        // Independently divide the numeric selector by the numeric quartic.
        let numeric_u: Vec<_> = u.iter().map(|f| eval(f, &a, e.p)).collect();
        let mut remainder = selector(&mut builder, &xs).unwrap();
        let mut quotient = vec![0; base.len() - 3];
        for degree in (4..=base.len()).rev() {
            let lead = remainder[degree];
            quotient[degree - 4] = lead;
            for j in 0..=4 {
                remainder[degree - 4 + j] =
                    (remainder[degree - 4 + j] + e.p - mm(lead, numeric_u[j], e.p)) % e.p;
            }
        }
        assert!(remainder.iter().all(|&c| c == 0));
        assert_eq!(quotient.pop(), Some(1));
        let assignment: Vec<_> = a.into_iter().chain(quotient).collect();
        assert_eq!(assignment.len(), nvars);
        for equation in &equations {
            let value = equation.iter().fold(0, |acc, (ex, c)| {
                let term = ex
                    .iter()
                    .zip(&assignment)
                    .fold(*c, |v, (&k, &x)| mm(v, pow(x, k as u64, e.p), e.p));
                (acc + term) % e.p
            });
            assert_eq!(value, 0);
        }
        println!(
            "RR5_COFACTOR_B677_CONSTRUCTION {}",
            serde_json::json!({
                "base_size":base.len(), "variables":nvars, "polynomials":equations.len(),
                "terms":equations.iter().map(Vec::len).sum::<usize>(),
                "max_degree":equations.iter().flat_map(|f|f.iter().map(|(e,_)|e.iter().sum::<u32>())).max(),
                "known_answer_assignment_satisfies_all_equations":true,
                "root_extraction_executed":false, "witness_is_test_only":true
            })
        );
    }

    #[test]
    fn rr5_expired_absolute_deadline_is_retained() {
        let e = PRIME31;
        let base = public_base(e, 4);
        let got = extract_until(e, &base, Some(e.g), Limits::default(), Instant::now());
        assert_eq!(got.status, Status::Deadline);
        assert!(got.traversal.is_none());
        assert!(got.relations.is_empty());
    }
}
