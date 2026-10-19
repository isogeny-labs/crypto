//! Exact finite-domain S3 encodings. Boolean degree is not regularity.
use crypto_lib::cryptanalysis::{
    f4_fp::{self, Ordering, Poly},
    ic_boundary::BinaryInstance,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

type P = BTreeMap<u64, u64>; // squarefree Boolean monomial -> field coefficient
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct System {
    pub basis_left: Vec<u64>,
    pub basis_right: Vec<u64>,
    pub variables: usize,
    pub equations: Vec<Poly>,
    pub encoding_term_products: u64,
}
pub fn span(xs: &[u64]) -> Vec<u64> {
    let mut pivots: BTreeMap<u32, u64> = BTreeMap::new();
    for &x in xs {
        let mut y = x;
        for (&j, &b) in pivots.iter().rev() {
            if y & (1u64 << j) != 0 {
                y ^= b;
            }
        }
        if y != 0 {
            pivots.insert(63 - y.leading_zeros(), y);
        }
    }
    pivots.into_values().collect()
}
fn plus(mut a: P, b: P) -> P {
    for (m, c) in b {
        let value = a.entry(m).or_default();
        *value ^= c;
    }
    a.retain(|_, c| *c != 0);
    a
}
fn mul(i: &BinaryInstance, a: &P, b: &P, ops: &mut u64) -> P {
    let mut out = P::new();
    for (&m, &c) in a {
        for (&n, &d) in b {
            *ops += 1;
            *out.entry(m | n).or_default() ^= i.fast.field.mul(c, d);
        }
    }
    out.retain(|_, c| *c != 0);
    out
}
fn square(i: &BinaryInstance, a: &P) -> P {
    a.iter().map(|(&m, &c)| (m, i.fast.field.sqr(c))).collect()
}
fn scale(i: &BinaryInstance, a: P, c: u64) -> P {
    a.into_iter()
        .filter_map(|(m, d)| {
            let v = i.fast.field.mul(c, d);
            (v != 0).then_some((m, v))
        })
        .collect()
}
fn constant(c: u64) -> P {
    if c == 0 {
        P::new()
    } else {
        [(0, c)].into()
    }
}
fn linear(b: &[u64], offset: usize) -> P {
    b.iter()
        .enumerate()
        .map(|(j, &x)| (1 << (j + offset), x))
        .collect()
}
fn membership(i: &BinaryInstance, x: &P, roots: &[u64], ops: &mut u64) -> P {
    roots.iter().fold(constant(1), |p, &r| {
        mul(i, &p, &plus(x.clone(), constant(r)), ops)
    })
}
pub fn decode(root: &[u64], basis: &[u64], offset: usize) -> u64 {
    basis.iter().enumerate().fold(0, |x, (j, b)| {
        x ^ if root[offset + j] == 1 { *b } else { 0 }
    })
}
pub fn encode(
    i: &BinaryInstance,
    left: &[u64],
    right: &[u64],
    charts: Option<(&[u64], &[u64])>,
    target_x: u64,
) -> System {
    let (bl, br) = (span(left), span(right));
    let selector = usize::from(charts.is_some());
    let n = bl.len() + br.len() + 2 * selector;
    assert!(n < 63);
    let (x, y) = (linear(&bl, 0), linear(&br, bl.len()));
    let mut ops = 0;
    // S3=(xy)^2 + t^2(x+y)^2 + txy + b, for b=1.
    let xy = mul(i, &x, &y, &mut ops);
    let f = plus(
        plus(
            square(i, &xy),
            scale(
                i,
                square(i, &plus(x.clone(), y.clone())),
                i.fast.field.sqr(target_x),
            ),
        ),
        plus(scale(i, xy, target_x), constant(i.b)),
    );
    let mut field_equations = vec![f];
    if let Some((a, b)) = charts {
        for (j, v) in [&x, &y].into_iter().enumerate() {
            let s: P = [(1u64 << (bl.len() + br.len() + j), 1)].into();
            let ga = membership(i, v, a, &mut ops);
            let gb = membership(i, v, b, &mut ops);
            field_equations.push(mul(i, &plus(constant(1), s.clone()), &ga, &mut ops));
            field_equations.push(mul(i, &s, &gb, &mut ops));
        }
    } else {
        field_equations.push(membership(i, &x, left, &mut ops));
        field_equations.push(membership(i, &y, right, &mut ops));
    }
    let mut equations = Vec::new();
    for f in field_equations {
        for bit in 0..i.n {
            let terms: Vec<_> = f
                .iter()
                .filter(|(_, c)| (*c >> bit) & 1 == 1)
                .map(|(&m, _)| ((0..n).map(|j| u32::from(m & (1 << j) != 0)).collect(), 1))
                .collect();
            if !terms.is_empty() {
                equations.push(f4_fp::normalise(&terms, 2, Ordering::Grevlex));
            }
        }
    }
    for j in 0..n {
        let mut a = vec![0; n];
        a[j] = 2;
        let mut b = vec![0; n];
        b[j] = 1;
        equations.push(f4_fp::normalise(&[(a, 1), (b, 1)], 2, Ordering::Grevlex));
    }
    System {
        basis_left: bl,
        basis_right: br,
        variables: n,
        equations,
        encoding_term_products: ops,
    }
}
pub fn xs(points: &[super::leads::Pt]) -> Vec<u64> {
    points
        .iter()
        .map(|p| p.x)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::ic_boundary::koblitz_instance;
    #[test]
    fn membership_and_selector_zero_sets_agree_exhaustively() {
        let i = koblitz_instance(1, 7).unwrap();
        let a = vec![1, 2];
        let b = vec![2, 5];
        let u = vec![1, 2, 5];
        let s = encode(&i, &u, &u, Some((&a, &b)), 7);
        for mask in 0..1u64 << s.variables {
            let root: Vec<_> = (0..s.variables).map(|j| (mask >> j) & 1).collect();
            let x = decode(&root, &s.basis_left, 0);
            let y = decode(&root, &s.basis_right, s.basis_left.len());
            let selx = root[s.variables - 2];
            let sely = root[s.variables - 1];
            let membership = (if selx == 0 { &a } else { &b }).contains(&x)
                && (if sely == 0 { &a } else { &b }).contains(&y);
            let f = i.fast.field.mul(i.fast.field.sqr(x), i.fast.field.sqr(y))
                ^ i.fast
                    .field
                    .mul(i.fast.field.sqr(7), i.fast.field.sqr(x ^ y))
                ^ i.fast.field.mul(7, i.fast.field.mul(x, y))
                ^ 1;
            assert_eq!(
                s.equations.iter().all(|f| f4_fp::eval(f, &root, 2) == 0),
                membership && f == 0
            );
        }
    }
    #[test]
    fn asymmetric_system_has_exact_separate_membership() {
        let i = koblitz_instance(1, 7).unwrap();
        let a = vec![1, 2, 3];
        let b = vec![4];
        let s = encode(&i, &a, &b, None, 7);
        for mask in 0..1u64 << s.variables {
            let root: Vec<_> = (0..s.variables).map(|j| (mask >> j) & 1).collect();
            let x = decode(&root, &s.basis_left, 0);
            let y = decode(&root, &s.basis_right, s.basis_left.len());
            let f = i.fast.field.mul(i.fast.field.sqr(x), i.fast.field.sqr(y))
                ^ i.fast
                    .field
                    .mul(i.fast.field.sqr(7), i.fast.field.sqr(x ^ y))
                ^ i.fast.field.mul(7, i.fast.field.mul(x, y))
                ^ 1;
            assert_eq!(
                s.equations.iter().all(|f| f4_fp::eval(f, &root, 2) == 0),
                a.contains(&x) && b.contains(&y) && f == 0
            );
        }
    }
}
