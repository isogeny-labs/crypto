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

type MPoly = BTreeMap<Vec<u32>, u64>;

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
    /// Native branch attempts, counted before the trace sink accepts a child.
    /// A refused attempt does not imply that specialization was executed.
    pub forbidden_branch_attempts: u64,
    /// Actual finite-domain evaluations from the same native traversal.
    pub forbidden_domain_evaluations: u64,
    pub forbidden_fallbacks_refused: usize,
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
    variables: usize,
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
            BTreeMap::from([(vec![0; self.variables], x % self.p)])
        }
    }
    fn var(&self, i: usize) -> MPoly {
        let mut e = vec![0; self.variables];
        e[i] = 1;
        BTreeMap::from([(e, 1)])
    }
    fn add(&mut self, a: &MPoly, b: &MPoly) -> Result<MPoly, String> {
        let mut out = a.clone();
        for (e, &c) in b {
            let v = self.am(out.get(e).copied().unwrap_or(0), c);
            if v == 0 {
                out.remove(e);
            } else {
                out.insert(e.clone(), v);
            }
        }
        self.guard(out.len())?;
        Ok(out)
    }
    fn scale(&mut self, a: &MPoly, c: u64) -> Result<MPoly, String> {
        let mut out = MPoly::new();
        for (e, &v) in a {
            let w = self.mm(v, c);
            if w != 0 {
                out.insert(e.clone(), w);
            }
        }
        self.guard(out.len())?;
        Ok(out)
    }
    fn mul(&mut self, a: &MPoly, b: &MPoly) -> Result<MPoly, String> {
        let mut out = MPoly::new();
        for (e, &c) in a {
            self.guard(out.len())?;
            for (f, &d) in b {
                let g: Vec<_> = e.iter().zip(f).map(|(x, y)| x + y).collect();
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
fn eval(f: &MPoly, point: &[u64], p: u64) -> u64 {
    f.iter().fold(0, |acc, (e, c)| {
        // Only U is evaluated after projecting away the unique cofactor.
        // Refuse to silently drop a variable on which a coefficient depends.
        assert!(point.len() <= e.len() && e[point.len()..].iter().all(|k| *k == 0));
        let mut v = *c;
        for j in 0..point.len() {
            v = mm(v, pow(point[j], e[j] as u64, p), p)
        }
        (acc + v) % p
    })
}

/// Public monic quartic U and h = product(X - x_i), shared by both presentations.
/// Only the first three variables occur in U; h has scalar coefficients.
fn public_polynomials(
    builder: &mut Builder,
    e: Curve,
    q: (u64, u64),
    xs: &[u64],
) -> Result<(Vec<MPoly>, Vec<u64>), String> {
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
    // h is built in O(B^2) scalar work, with no tuple construction.
    let mut h = vec![1u64];
    for &x in xs {
        let mut next = vec![0; h.len() + 1];
        for (i, &c) in h.iter().enumerate() {
            let z = builder.mm(c, (e.p - x) % e.p);
            next[i] = builder.am(next[i], z);
            next[i + 1] = builder.am(next[i + 1], c);
        }
        h = next;
    }
    Ok((u, h))
}

/// The existing four equations h mod U = 0, with three parameter variables.
fn expanded_remainders(
    builder: &mut Builder,
    u: &[MPoly],
    h: &[u64],
) -> Result<Vec<MPoly>, String> {
    // Horner in F_p[b,c,d][X]/U, degree X <4 throughout.
    let mut rem = vec![MPoly::new(); 4];
    for &c in h.iter().rev() {
        let lead = rem[3].clone();
        let mut next = vec![builder.c(c), rem[0].clone(), rem[1].clone(), rem[2].clone()];
        for j in 0..4 {
            let z = builder.mul(&lead, &u[j])?;
            let neg = builder.scale(&z, builder.p - 1)?;
            next[j] = builder.add(&next[j], &neg)?;
        }
        rem = next;
    }
    Ok(rem)
}

/// All lower coefficients of U*W-h, for monic W of degree B-4.
///
/// Since U and h are monic, U divides h exactly when this unique monic W
/// exists. Its coefficients occupy variables 3..B-1, after b,c,d. The top
/// coefficient is identically one on both sides; every other coefficient is
/// enforced. Quadratic U coefficients times linear W coefficients give total
/// degree at most three, without enumerating any base subset or field domain.
fn quotient_product_equations(
    builder: &mut Builder,
    u: &[MPoly],
    h: &[u64],
) -> Result<Vec<MPoly>, String> {
    let degree = h.len() - 5;
    assert!((1..=4).contains(&degree));
    assert_eq!(builder.variables, 3 + degree);
    assert_eq!(u.len(), 5);
    assert_eq!(u[4], builder.c(1));
    assert_eq!(h.last(), Some(&1));
    let mut w: Vec<_> = (0..degree).map(|j| builder.var(3 + j)).collect();
    w.push(builder.c(1));
    let mut equations = builder.conv(u, &w)?;
    assert_eq!(equations.pop(), Some(builder.c(1)));
    assert_eq!(equations.len() + 1, h.len());
    for (coefficient, &expected) in equations.iter_mut().zip(h) {
        *coefficient = builder.add(coefficient, &builder.c((builder.p - expected) % builder.p))?;
    }
    Ok(equations)
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
    let use_product = (5..=8).contains(&base.len());
    let variables = if use_product { base.len() - 1 } else { 3 };
    let ordering = if use_product {
        Ordering::Grevlex
    } else {
        Ordering::Lex
    };
    let mut result = ResultRow {
        status: Status::InvalidInput,
        reason: String::new(),
        relations: vec![],
        diagnostics: Diagnostics {
            base_size: base.len(),
            presentation: if use_product {
                "quotient_product".into()
            } else {
                "expanded_remainder".into()
            },
            input_variables: variables,
            ..Diagnostics::default()
        },
        traversal: None,
    };
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
        if y == 0 || !ar.on_curve(e, Some((x, y))) || index.insert(x, (i, y)).is_some() {
            result.reason = "base must have distinct valid nonzero-y abscissae".into();
            return result;
        }
    }
    if !ar.on_curve(e, q) {
        result.reason = "invalid public target".into();
        return result;
    }
    if index.contains_key(&target.0) {
        result.status = Status::UnsupportedChart;
        result.reason = "target-in-base chart not implemented".into();
        return result;
    }
    if base.len() < 4 {
        result.status = Status::Empty;
        result.reason = "four distinct abscissae require B>=4".into();
        return result;
    }
    let deadline = Instant::now() + Duration::from_secs(limits.seconds);
    let mut b = Builder {
        p: e.p,
        variables,
        deadline,
        term_cap: limits.max_polynomial_terms,
        adds: 0,
        muls: 0,
    };
    let xs: Vec<_> = base.iter().map(|p| p.0).collect();
    let built = public_polynomials(&mut b, e, target, &xs).and_then(|(u, h)| {
        let equations = if use_product {
            quotient_product_equations(&mut b, &u, &h)
        } else {
            expanded_remainders(&mut b, &u, &h)
        }?;
        Ok((u, equations))
    });
    result.diagnostics.symbolic_additions = b.adds;
    result.diagnostics.symbolic_multiplications = b.muls;
    let (u, constraints) = match built {
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
    let equations: Vec<Poly> = constraints
        .iter()
        .map(|f| {
            normalise(
                &f.iter().map(|(e, c)| (e.clone(), *c)).collect::<Vec<_>>(),
                e.p,
                ordering,
            )
        })
        .collect();
    result.diagnostics.input_polynomials = equations.len();
    result.diagnostics.input_terms = equations.iter().map(Vec::len).sum();
    result.diagnostics.input_max_degree = equations
        .iter()
        .flat_map(|f| f.iter().map(|(e, _)| e.iter().sum()))
        .max()
        .unwrap_or(0);
    let mut opts = F4Options::new(ordering, limits.max_degree);
    opts.deadline = Some(deadline);
    let mut assignments = Vec::new();
    let mut output_cap = false;
    let mut refused = 0usize;
    let traversal = solve_visit_traced(
        &equations,
        variables,
        e.p,
        &opts,
        &vec![None; variables],
        |a| {
            if assignments.len() >= limits.max_assignments {
                output_cap = true;
                return true;
            }
            // The native visitor replays every original equation on the full
            // assignment before this callback. The monic cofactor is unique,
            // so only the three RR5 parameters are needed for signed lifting.
            assignments.push([a[0], a[1], a[2]]);
            false
        },
        |event| {
            match event {
                VisitEvent::NativeResult { generating_set, .. }
                    if !generating_set.iter().any(is_nonzero_constant)
                        && !generating_set.iter().any(is_univariate) =>
                {
                    refused += 1;
                    return Err("no algebraic univariate; enumeration refused".into());
                }
                VisitEvent::Branch { source, .. } if source != "univariate_roots" => {
                    refused += 1;
                    return Err(format!("forbidden enumeration branch {source}"));
                }
                _ => {}
            }
            Ok(())
        },
    );
    result.diagnostics.forbidden_fallbacks_refused = refused;
    let traversal = match traversal {
        Ok(t) => t,
        Err(why) => {
            result.reason = why;
            return result;
        }
    };
    result.diagnostics.forbidden_branch_attempts = traversal.full_field_branches;
    result.diagnostics.forbidden_domain_evaluations = traversal.domain_evaluations;
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
    if result.diagnostics.rejected_assignments > 0 {
        result.status = Status::Inconclusive;
        result.reason = "an algebraic assignment failed signed chart replay".into();
    } else if result.status == Status::Complete && result.relations.is_empty() {
        result.status = Status::Empty;
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
        assert_eq!(got.diagnostics.forbidden_branch_attempts, 0);
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
        assert_eq!(got.status, Status::Inconclusive);
        assert_eq!(got.diagnostics.forbidden_fallbacks_refused, 1);
        assert!(got.reason.contains("enumeration refused"));
        let trace = got.traversal.as_ref().unwrap();
        assert!(trace.trace_stopped);
        assert!(!trace.complete);
        assert_eq!(trace.solutions_visited, 0);
        assert_eq!(trace.full_field_branches, 0);
        assert_eq!(trace.domain_evaluations, 0);
        assert_eq!(got.diagnostics.forbidden_branch_attempts, 0);
        assert_eq!(got.diagnostics.forbidden_domain_evaluations, 0);
    }

    #[test]
    fn quotient_product_agrees_with_scalar_long_division() {
        let e = PRIME31;
        let base = public_base(e, 8);
        let xs: Vec<_> = base.iter().map(|point| point.0).collect();
        for size in 5..=8 {
            let mut builder = Builder {
                p: e.p,
                variables: size - 1,
                deadline: Instant::now() + Duration::from_secs(Limits::default().seconds),
                term_cap: Limits::default().max_polynomial_terms,
                adds: 0,
                muls: 0,
            };
            let (u, h) = public_polynomials(&mut builder, e, e.g, &xs[..size]).unwrap();
            let equations = quotient_product_equations(&mut builder, &u, &h).unwrap();
            assert_eq!(equations.len(), size);
            assert!(equations
                .iter()
                .flat_map(|f| f.keys())
                .all(|exponents| exponents.iter().sum::<u32>() <= 3));

            for parameters in [[0, 0, 0], [1, 2, 3], [e.p - 1, 17, e.p - 2]] {
                let coefficients: Vec<_> = u.iter().map(|f| eval(f, &parameters, e.p)).collect();
                // Independent scalar long division, rather than constructing
                // U*W again: h = U*quotient + remainder with deg remainder <4.
                let mut remainder = h.clone();
                let mut quotient = vec![0; size - 3];
                for k in (4..h.len()).rev() {
                    let lead = remainder[k];
                    quotient[k - 4] = lead;
                    for j in 0..5 {
                        let at = k - 4 + j;
                        remainder[at] =
                            (remainder[at] + e.p - mm(lead, coefficients[j], e.p)) % e.p;
                    }
                }
                assert_eq!(quotient.last(), Some(&1));
                assert!(remainder[4..].iter().all(|c| *c == 0));
                let mut assignment = parameters.to_vec();
                assignment.extend_from_slice(&quotient[..size - 4]);
                for (j, equation) in equations.iter().enumerate() {
                    assert_eq!(
                        eval(equation, &assignment, e.p),
                        (e.p - remainder[j]) % e.p,
                        "B={size}, coefficient {j}"
                    );
                }
                // Each auxiliary is forced by a high coefficient. Altering
                // w_j alone changes the X^(4+j) coefficient by exactly one.
                for j in 0..size - 4 {
                    let mut altered = assignment.clone();
                    altered[3 + j] = (altered[3 + j] + 1) % e.p;
                    assert_eq!(eval(&equations[4 + j], &altered, e.p), 1);
                }
            }
        }
    }
}
