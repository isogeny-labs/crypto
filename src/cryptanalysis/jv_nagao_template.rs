//! Opt-in, provenance-compiled elimination templates for repeated polynomial
//! systems. This first construction uses complete-basis cofactors, without
//! claiming module-syzygy minimization. Refusals require the caller's F4 path.
//! Every accepted application retains exact original-generator membership and
//! checks input containment and Buchberger's criterion, including nonradical
//! and inconsistent ideals. A learned leading ideal alone is not a certificate.
use certified_relations::{self as algebra, Mono, Proof, Row};
use serde::Serialize;
use std::{cmp::Ordering, collections::BTreeSet};

/// Same representation as `f4_fp::Poly`; accepts its values without conversion.
pub type Poly = Vec<(Vec<u32>, u64)>;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Limits {
    pub rows: usize,
    pub columns: usize,
    pub cells: usize,
    pub degree: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            rows: 512,
            columns: 1024,
            cells: 1_000_000,
            degree: 16,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct Work {
    /// Includes exponentiation used by this implementation's inversions and
    /// the explicit provenance columns, not only polynomial elimination.
    pub modular_products: u64,
    pub nontrivial_inversions: u64,
    pub coefficient_updates: u64,
}
impl Work {
    fn mul(&mut self, a: u64, b: u64, p: u64) -> u64 {
        self.modular_products += 1;
        ((u128::from(a) * u128::from(b)) % u128::from(p)) as u64
    }
    fn inverse(&mut self, a: u64, p: u64) -> u64 {
        assert!(a > 0 && a < p);
        if a == 1 {
            return 1;
        }
        self.nontrivial_inversions += 1;
        let (mut base, mut exp, mut result) = (a, p - 2, 1);
        while exp != 0 {
            if exp & 1 != 0 {
                result = self.mul(result, base, p);
            }
            exp >>= 1;
            if exp != 0 {
                base = self.mul(base, base, p);
            }
        }
        result
    }
}

#[derive(Debug, Serialize)]
pub struct Refusal {
    pub reason: String,
    pub attempted: Work,
}
impl Refusal {
    fn new(reason: impl Into<String>, attempted: Work) -> Self {
        Self {
            reason: reason.into(),
            attempted,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Template {
    p: u64,
    n: usize,
    support: Vec<BTreeSet<Mono>>,
    multiples: Vec<(usize, Mono)>,
    columns: Vec<Mono>,
    leading: Vec<Mono>,
    limits: Limits,
}
pub struct Applied {
    pub basis: Vec<Poly>,
    pub certificates: Vec<Row>,
    pub elimination: Work,
    pub rank: usize,
}

fn order(a: &Mono, b: &Mono) -> Ordering {
    a.iter()
        .sum::<usize>()
        .cmp(&b.iter().sum::<usize>())
        .then_with(|| b.iter().rev().cmp(a.iter().rev()))
}
fn lead(f: &algebra::Poly) -> Option<&Mono> {
    f.keys().max_by(|a, b| order(a, b))
}
fn divides(a: &Mono, b: &Mono) -> bool {
    a.iter().zip(b).all(|(x, y)| x <= y)
}
fn shift(a: &Mono, b: &Mono, degree: usize) -> Result<Mono, String> {
    let result = a
        .iter()
        .zip(b)
        .map(|(x, y)| x.checked_add(*y).ok_or("exponent overflow"))
        .collect::<Result<Mono, _>>()?;
    if result.iter().sum::<usize>() > degree {
        return Err("template degree cap".into());
    }
    Ok(result)
}
fn canonical(
    input: &[Poly],
    n: usize,
    p: u64,
    limits: Limits,
) -> Result<Vec<algebra::Poly>, String> {
    if !(1..=16).contains(&n) || p > i64::MAX as u64 || !algebra::prime(p) {
        return Err("template variable/prime domain".into());
    }
    if input.is_empty() || input.len() > 512 || limits.degree > 16 {
        return Err("template input/degree budget".into());
    }
    input
        .iter()
        .map(|f| {
            if f.len() > 5000 {
                return Err("input term budget".into());
            }
            let mut out = algebra::Poly::new();
            for (m, c) in f {
                if m.len() != n
                    || m.iter().map(|e| u64::from(*e)).sum::<u64>() > limits.degree as u64
                {
                    return Err("input exponent shape/degree".into());
                }
                let m: Mono = m.iter().map(|e| *e as usize).collect();
                let v = (out.get(&m).copied().unwrap_or(0) + *c % p) % p;
                if v == 0 {
                    out.remove(&m);
                } else {
                    out.insert(m, v);
                }
            }
            Ok(out)
        })
        .collect()
}

impl Template {
    /// Development-only construction, including all generic provenance work
    /// and exact checking. Callers retain algebra::counters() on every failure.
    pub fn learn(input: &[Poly], n: usize, p: u64, limits: Limits) -> Result<Self, String> {
        let original = canonical(input, n, p, limits)?;
        let witness = algebra::reference_criteria(&original, n, p)?;
        Self::from_witness(input, n, p, &witness, limits)
    }

    /// Accept only an exact original-input basis certificate, not a root list
    /// or the sampled leading-monomial fingerprint by itself.
    pub fn from_witness(
        input: &[Poly],
        n: usize,
        p: u64,
        witness: &[Row],
        limits: Limits,
    ) -> Result<Self, String> {
        let original = canonical(input, n, p, limits)?;
        if witness.is_empty()
            || witness.len() > 512
            || witness.iter().any(|r| {
                r.poly.is_empty()
                    || r.poly.len() > 5000
                    || r.proof.len() > limits.rows
                    || r.poly.iter().any(|(m, c)| {
                        m.len() != n
                            || *c == 0
                            || *c >= p
                            || m.iter()
                                .try_fold(0usize, |a, b| a.checked_add(*b))
                                .is_none_or(|d| d > limits.degree)
                    })
                    || r.proof.iter().any(|((i, m), c)| {
                        *i >= input.len()
                            || m.len() != n
                            || *c == 0
                            || *c >= p
                            || m.iter()
                                .try_fold(0usize, |a, b| a.checked_add(*b))
                                .is_none_or(|d| d > limits.degree)
                    })
            })
        {
            return Err("development witness shape/budget".into());
        }
        if !algebra::check_reference(witness, &original, n, p) {
            return Err("development basis/provenance certificate".into());
        }
        let mut multiples = BTreeSet::new();
        for row in witness {
            for (i, m) in row.proof.keys() {
                if *i >= input.len() || m.len() != n {
                    return Err("development proof shape".into());
                }
                multiples.insert((*i, m.clone()));
                if multiples.len() > limits.rows {
                    return Err("template row cap".into());
                }
            }
        }
        if multiples.is_empty() {
            return Err("empty template".into());
        }
        let multiples: Vec<_> = multiples.into_iter().collect();
        let mut columns = BTreeSet::new();
        for (i, multiplier) in &multiples {
            for m in original[*i].keys() {
                columns.insert(shift(m, multiplier, limits.degree)?);
                if columns.len() > limits.columns {
                    return Err("template column cap".into());
                }
            }
        }
        let cells = multiples
            .len()
            .checked_mul(
                columns
                    .len()
                    .checked_add(multiples.len())
                    .ok_or("cell overflow")?,
            )
            .ok_or("cell overflow")?;
        if cells > limits.cells {
            return Err("coefficient/provenance cell cap".into());
        }
        let mut columns: Vec<_> = columns.into_iter().collect();
        columns.sort_by(|a, b| order(b, a));
        let mut leading: Vec<_> = witness
            .iter()
            .map(|r| lead(&r.poly).expect("checked nonempty").clone())
            .collect();
        leading.sort_by(|a, b| order(b, a));
        Ok(Self {
            p,
            n,
            support: original
                .iter()
                .map(|f| f.keys().cloned().collect())
                .collect(),
            multiples,
            columns,
            leading,
            limits,
        })
    }
    pub fn rows(&self) -> usize {
        self.multiples.len()
    }
    pub fn columns(&self) -> usize {
        self.columns.len()
    }

    /// Complete-basis construction with a mandatory exact check. No solver or
    /// root fallback is hidden here; Err carries all attempted elimination work.
    pub fn apply(&self, input: &[Poly]) -> Result<Applied, Refusal> {
        let mut work = Work::default();
        let original =
            canonical(input, self.n, self.p, self.limits).map_err(|e| Refusal::new(e, work))?;
        if original.len() != self.support.len()
            || original
                .iter()
                .zip(&self.support)
                .any(|(f, support)| f.keys().any(|m| !support.contains(m)))
        {
            return Err(Refusal::new("input support mismatch", work));
        }
        let (rows, columns, p) = (self.rows(), self.columns(), self.p);
        let mut matrix = vec![vec![0; columns + rows]; rows];
        for (r, (i, multiplier)) in self.multiples.iter().enumerate() {
            for (m, c) in &original[*i] {
                let m =
                    shift(m, multiplier, self.limits.degree).map_err(|e| Refusal::new(e, work))?;
                let col = self
                    .columns
                    .binary_search_by(|a| order(&m, a))
                    .map_err(|_| Refusal::new("compiled column mismatch", work))?;
                matrix[r][col] = *c;
            }
            matrix[r][columns + r] = 1;
        }
        let mut pivots = Vec::new();
        for c in 0..columns {
            let r = pivots.len();
            let Some(next) = (r..rows).find(|i| matrix[*i][c] != 0) else {
                continue;
            };
            matrix.swap(r, next);
            let inverse = work.inverse(matrix[r][c], p);
            if inverse != 1 {
                for value in &mut matrix[r][c..] {
                    if *value != 0 {
                        *value = work.mul(*value, inverse, p);
                    }
                }
            }
            let pivot = matrix[r].clone();
            for (i, row) in matrix.iter_mut().enumerate() {
                if i == r || row[c] == 0 {
                    continue;
                }
                let factor = row[c];
                for j in c..columns + rows {
                    if pivot[j] == 0 {
                        continue;
                    }
                    let product = work.mul(factor, pivot[j], p);
                    row[j] = if row[j] >= product {
                        row[j] - product
                    } else {
                        p - (product - row[j])
                    };
                    work.coefficient_updates += 1;
                }
            }
            pivots.push(c);
        }
        let minimal: Vec<_> = (0..pivots.len())
            .filter(|i| {
                !(0..pivots.len()).any(|j| {
                    i != &j && divides(&self.columns[pivots[j]], &self.columns[pivots[*i]])
                })
            })
            .collect();
        let leading: Vec<_> = minimal
            .iter()
            .map(|i| self.columns[pivots[*i]].clone())
            .collect();
        if leading != self.leading {
            return Err(Refusal::new(
                "leading ideal changed or incomplete template",
                work,
            ));
        }
        let certificates: Vec<Row> = minimal
            .iter()
            .map(|i| {
                let poly = self
                    .columns
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| matrix[*i][*j] != 0)
                    .map(|(j, m)| (m.clone(), matrix[*i][j]))
                    .collect();
                let proof: Proof = self
                    .multiples
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| matrix[*i][columns + *j] != 0)
                    .map(|(j, descriptor)| (descriptor.clone(), matrix[*i][columns + j]))
                    .collect();
                Row { poly, proof }
            })
            .collect();
        if !algebra::check_reference(&certificates, &original, self.n, p) {
            return Err(Refusal::new(
                "online membership/input/S-pair certificate",
                work,
            ));
        }
        let basis = certificates
            .iter()
            .map(|r| {
                let mut terms: Vec<_> = r.poly.iter().collect();
                terms.sort_by(|(a, _), (b, _)| order(b, a));
                terms
                    .into_iter()
                    .map(|(m, c)| (m.iter().map(|e| *e as u32).collect(), *c))
                    .collect()
            })
            .collect();
        Ok(Applied {
            basis,
            certificates,
            elimination: work,
            rank: pivots.len(),
        })
    }
}

/// Exact mutual containment for two already checked Groebner bases. This is a
/// qualification control, not a substitute for either basis's certificate.
pub fn same_ideal(
    left: &[Poly],
    right: &[Poly],
    n: usize,
    p: u64,
    limits: Limits,
) -> Result<(bool, Work), Refusal> {
    let mut work = Work::default();
    let left = canonical(left, n, p, limits).map_err(|e| Refusal::new(e, work))?;
    let right = canonical(right, n, p, limits).map_err(|e| Refusal::new(e, work))?;
    for (values, divisors) in [(&left, &right), (&right, &left)] {
        for value in values {
            let mut pending = value.clone();
            let mut steps = 0;
            while let Some(m) = lead(&pending).cloned() {
                steps += 1;
                if steps > 1_000_000 {
                    return Err(Refusal::new("containment step budget", work));
                }
                let Some((g, lm)) = divisors
                    .iter()
                    .filter_map(|g| lead(g).map(|lm| (g, lm)))
                    .find(|(_, lm)| divides(lm, &m))
                else {
                    return Ok((false, work));
                };
                let inverse = work.inverse(g[lm], p);
                let factor = work.mul(pending[&m], inverse, p);
                let multiplier: Mono = m.iter().zip(lm).map(|(a, b)| a - b).collect();
                for (gm, gc) in g {
                    let target =
                        shift(gm, &multiplier, limits.degree).map_err(|e| Refusal::new(e, work))?;
                    let product = work.mul(factor, *gc, p);
                    let old = pending.get(&target).copied().unwrap_or(0);
                    let new = if old >= product {
                        old - product
                    } else {
                        p - (product - old)
                    };
                    work.coefficient_updates += 1;
                    if new == 0 {
                        pending.remove(&target);
                    } else {
                        pending.insert(target, new);
                    }
                }
            }
        }
    }
    Ok((true, work))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn family(a: u64) -> Vec<Poly> {
        vec![
            vec![(vec![2, 0], 1), (vec![0, 0], (101 - a) % 101)],
            vec![(vec![0, 1], 1), (vec![1, 0], 100)],
        ]
    }
    #[test]
    fn ordinary_parameter_change_has_complete_provenance() {
        let t = Template::learn(&family(2), 2, 101, Limits::default()).unwrap();
        let solved = t.apply(&family(3)).unwrap();
        assert!(!solved.basis.is_empty());
        for f in &solved.basis {
            assert_eq!(f[0].1, 1);
            for pair in f.windows(2) {
                let a: Mono = pair[0].0.iter().map(|e| *e as usize).collect();
                let b: Mono = pair[1].0.iter().map(|e| *e as usize).collect();
                assert_eq!(order(&a, &b), Ordering::Greater);
            }
        }
        assert!(solved.elimination.modular_products > 0);
        assert!(algebra::check_reference(
            &solved.certificates,
            &canonical(&family(3), 2, 101, Limits::default()).unwrap(),
            2,
            101
        ));
    }
    #[test]
    fn nonradical_degree_drop_in_constant_is_complete() {
        let t = Template::learn(&family(2), 2, 101, Limits::default()).unwrap();
        let solved = t.apply(&family(0)).unwrap();
        assert_eq!(solved.basis.len(), 2);
    }
    #[test]
    fn changed_dimension_refuses_instead_of_returning_empty_roots() {
        let make = |a| vec![vec![(vec![2], a), (vec![1], 1)]];
        let t = Template::learn(&make(1), 1, 101, Limits::default()).unwrap();
        assert!(t.apply(&make(0)).is_err());
    }
    #[test]
    fn inconsistent_training_and_rank_drop_are_distinguished() {
        let f = |a| vec![(vec![2], 1), (vec![0], a)];
        let t = Template::learn(&[f(100), f(99)], 1, 101, Limits::default()).unwrap();
        assert_eq!(
            t.apply(&[f(98), f(97)]).unwrap().basis,
            vec![vec![(vec![0], 1)]]
        );
        assert!(t.apply(&[f(100), f(100)]).is_err());
    }
    #[test]
    fn changed_support_and_arity_refuse() {
        let t = Template::learn(&family(2), 2, 101, Limits::default()).unwrap();
        let mut input = family(3);
        input[0].push((vec![1, 1], 1));
        assert!(t.apply(&input).is_err());
        assert!(t.apply(&input[..1]).is_err());
        assert!(Template::learn(&family(2), 3, 101, Limits::default()).is_err());
    }
    #[test]
    fn nonprime_empty_and_resource_caps_refuse() {
        assert!(Template::learn(&family(2), 2, 15, Limits::default()).is_err());
        assert!(Template::learn(&[], 2, 101, Limits::default()).is_err());
        assert!(Template::learn(
            &family(2),
            2,
            101,
            Limits {
                rows: 1,
                ..Limits::default()
            }
        )
        .is_err());
        assert!(Template::learn(
            &family(2),
            2,
            101,
            Limits {
                cells: 1,
                ..Limits::default()
            }
        )
        .is_err());
    }
    #[test]
    fn mutated_development_provenance_is_rejected() {
        let input = canonical(&family(2), 2, 101, Limits::default()).unwrap();
        let mut witness = algebra::reference_criteria(&input, 2, 101).unwrap();
        witness[0].proof.clear();
        assert!(Template::from_witness(&family(2), 2, 101, &witness, Limits::default()).is_err());
    }
    #[test]
    fn mutual_containment_rejects_changed_constant() {
        let t = Template::learn(&family(2), 2, 101, Limits::default()).unwrap();
        let a = t.apply(&family(2)).unwrap();
        let b = t.apply(&family(3)).unwrap();
        assert!(
            same_ideal(&a.basis, &a.basis, 2, 101, Limits::default())
                .unwrap()
                .0
        );
        assert!(
            !same_ideal(&a.basis, &b.basis, 2, 101, Limits::default())
                .unwrap()
                .0
        );
    }
    #[test]
    fn malformed_proof_indices_and_exponents_refuse_without_checker_panic() {
        let input = canonical(&family(2), 2, 101, Limits::default()).unwrap();
        let mut witness = algebra::reference_criteria(&input, 2, 101).unwrap();
        witness[0].proof = Proof::from([((usize::MAX, vec![0, 0]), 1)]);
        assert!(Template::from_witness(&family(2), 2, 101, &witness, Limits::default()).is_err());
        witness[0].proof = Proof::from([((0, vec![usize::MAX, usize::MAX]), 1)]);
        assert!(Template::from_witness(&family(2), 2, 101, &witness, Limits::default()).is_err());
    }
}
