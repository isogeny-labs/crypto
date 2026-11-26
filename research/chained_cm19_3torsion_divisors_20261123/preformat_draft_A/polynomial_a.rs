use crate::algebra::A;
use crate::field::{F, ZERO};
use crate::linear::{self, RankCertificate};
use std::collections::BTreeMap;

pub type Mono = [u8; 3];
pub type Polynomial = Vec<(Mono, F)>;
pub const CONICS: [Mono; 6] = [[2, 0, 0], [1, 1, 0], [1, 0, 1], [0, 2, 0], [0, 1, 1], [0, 0, 2]];

fn normalized(terms: Polynomial) -> Polynomial {
    let mut out = BTreeMap::new();
    for (m, c) in terms {
        let old = out.get(&m).copied().unwrap_or(ZERO);
        out.insert(m, old.plus(c));
    }
    out.into_iter().filter(|&(_, c)| c != ZERO).collect()
}

fn product(a: &Polynomial, b: &Polynomial) -> Polynomial {
    normalized(a.iter().flat_map(|&(m, c)| b.iter().map(move |&(n, d)| (std::array::from_fn(|i| m[i] + n[i]), c.times(d)))).collect())
}

pub fn quartic() -> Polynomial {
    let literal = normalized(vec![
        ([4, 0, 0], F::c(1)), ([1, 3, 0], F::c(4)),
        ([0, 4, 0], F::c(2)), ([2, 0, 2], F::c(5)),
        ([1, 1, 2], F::c(6)), ([0, 2, 2], F::c(2)),
        ([0, 0, 4], F::c(1)),
    ]);
    let g = normalized(vec![([2, 0, 0], F::c(6)), ([1, 1, 0], F::c(3)), ([0, 2, 0], F::c(1)), ([0, 0, 2], F::c(1))]);
    let mut original = product(&g, &g);
    original.extend([([3, 1, 0], F::c(-1)), ([1, 3, 0], F::c(-2)), ([0, 4, 0], F::c(-6))]);
    let original = normalized(original);
    assert_eq!(literal, original);
    let mut changed = literal.clone();
    changed[0].1 = changed[0].1.plus(F::c(1));
    assert_ne!(changed, original);
    literal
}

pub fn conic(coefficients: [F; 6]) -> Polynomial {
    normalized(CONICS.into_iter().zip(coefficients).collect())
}

pub fn monomial(m: Mono, p: [A; 3]) -> A {
    m.into_iter().zip(p).fold(p[0].ring.scalar(F::c(1)), |s, (e, x)| s.times(x.power(e)))
}

pub fn evaluate(poly: &Polynomial, p: [A; 3]) -> A {
    poly.iter().fold(p[0].ring.scalar(ZERO), |s, &(m, c)| s.plus(monomial(m, p).scale(c)))
}

pub fn constraints(p: [A; 3]) -> Vec<Vec<F>> {
    let values = CONICS.map(|m| monomial(m, p));
    (0..p[0].ring.degree).map(|i| values.iter().map(|v| v.values[i]).collect()).collect()
}

fn monomials(degree: u8) -> Vec<Mono> {
    (0..=degree).rev().flat_map(|x| (0..=degree - x).rev().map(move |y| [x, y, degree - x - y])).collect()
}

fn multiples(poly: &Polynomial, shifts: &[Mono], degree: u8) -> Vec<Vec<F>> {
    let basis = monomials(degree);
    shifts.iter().map(|shift| {
        let mut row = vec![ZERO; basis.len()];
        for &(m, c) in poly {
            let n = std::array::from_fn(|i| m[i] + shift[i]);
            let column = basis.iter().position(|&v| v == n).expect("homogeneous degree");
            row[column] = row[column].plus(c);
        }
        row
    }).collect()
}

pub fn common_divisor(a: &Polynomial, b: &Polynomial, f: &Polynomial) -> (RankCertificate, Option<RankCertificate>) {
    let mut rows = multiples(a, &monomials(1), 3);
    rows.extend(multiples(b, &monomials(1), 3));
    assert_eq!(rows.len(), 6);
    let coprime = linear::certificate(rows, 10);
    if coprime.pivots.len() != 6 {
        return (coprime, None);
    }
    let mut rows = multiples(a, &monomials(4), 6);
    rows.extend(multiples(b, &monomials(4), 6));
    rows.extend(multiples(f, &monomials(2), 6));
    assert_eq!(rows.len(), 36);
    assert_eq!(rows[0].len(), 28);
    (coprime, Some(linear::certificate(rows, 28)))
}
