use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::{
    descent_lowgamma::{descend_on_subspace, FactorSubspace},
    f4_fp::{normalise, Ordering, Poly},
    ic_boundary::{BinaryInstance, PrimeCurve},
};
use std::collections::BTreeMap;

type P = BTreeMap<Vec<u32>, u64>;
fn add(a: &P, b: &P, p: u64) -> P {
    let mut r = a.clone();
    for (m, c) in b {
        let v = r.entry(m.clone()).or_default();
        *v = (*v + c) % p;
    }
    r.retain(|_, v| *v != 0);
    r
}
fn scale(a: &P, c: u64, p: u64) -> P {
    a.iter()
        .filter_map(|(m, v)| {
            let c = (*v as u128 * c as u128 % p as u128) as u64;
            (c != 0).then(|| (m.clone(), c))
        })
        .collect()
}
fn mul(a: &P, b: &P, p: u64) -> P {
    let mut r = P::new();
    for (m, c) in a {
        for (n, d) in b {
            let e = m.iter().zip(n).map(|(x, y)| x + y).collect();
            let v = r.entry(e).or_default();
            *v = (*v + (*c as u128 * *d as u128 % p as u128) as u64) % p;
        }
    }
    r.retain(|_, v| *v != 0);
    r
}
fn constant(c: u64, p: u64) -> P {
    if c.is_multiple_of(p) {
        P::new()
    } else {
        [(vec![0, 0], c % p)].into()
    }
}
fn var(k: usize) -> P {
    let mut e = vec![0, 0];
    e[k] = 1;
    [(e, 1)].into()
}
fn finish(p: &P, modulus: u64) -> Poly {
    normalise(
        &p.iter().map(|(m, c)| (m.clone(), *c)).collect::<Vec<_>>(),
        modulus,
        Ordering::Grevlex,
    )
}

/// Direct S3 with exact finite-set membership. This is deliberately named
/// separately from low-degree tower/incidence formulations.
pub fn prime(c: &PrimeCurve, x3: u64, domain: &[u64]) -> Vec<Poly> {
    let p = c.p;
    let x = var(0);
    let y = var(1);
    let one = constant(1, p);
    let sum = add(&x, &y, p);
    let dif = add(&x, &scale(&y, p - 1, p), p);
    let xy = mul(&x, &y, p);
    let term1 = scale(
        &mul(&dif, &dif, p),
        (x3 as u128 * x3 as u128 % p as u128) as u64,
        p,
    );
    let mid = add(
        &mul(&sum, &add(&xy, &constant(c.a, p), p), p),
        &constant(2 * c.b % p, p),
        p,
    );
    let t = add(&xy, &constant((p - c.a) % p, p), p);
    let mut f = add(&term1, &scale(&mid, (p - 2 * x3 % p) % p, p), p);
    f = add(&f, &mul(&t, &t, p), p);
    f = add(&f, &scale(&sum, (p - 4 * c.b % p) % p, p), p);
    let mut out = vec![finish(&f, p)];
    for v in [x, y] {
        let mut member = one.clone();
        for &root in domain {
            member = mul(&member, &add(&v, &constant((p - root) % p, p), p), p);
        }
        out.push(finish(&member, p));
    }
    out
}

pub fn binary(i: &BinaryInstance, v: &FactorSubspace, x3: u64) -> Vec<Poly> {
    let n = 2 * v.n_sub as usize;
    let eqs = descend_on_subspace(
        i.n,
        v,
        &i.irreducible,
        &F2mElement::from_biguint(&i.b.into(), i.n),
        &F2mElement::from_biguint(&x3.into(), i.n),
    );
    let mut out = Vec::new();
    for eq in eqs {
        let mut terms = Vec::new();
        if eq.coeffs[0] {
            terms.push((vec![0; n], 1));
        }
        for j in 0..n {
            if eq.coeffs[j + 1] {
                let mut m = vec![0; n];
                m[j] = 1;
                terms.push((m, 1));
            }
        }
        for a in 0..n {
            for b in a + 1..n {
                let idx = crypto_lib::cryptanalysis::ffd_harness::quad_monomial_index(
                    a as u32, b as u32, n as u32,
                );
                if eq.coeffs[idx] {
                    let mut m = vec![0; n];
                    m[a] = 1;
                    m[b] = 1;
                    terms.push((m, 1));
                }
            }
        }
        out.push(normalise(&terms, 2, Ordering::Grevlex));
    }
    for j in 0..n {
        let mut a = vec![0; n];
        a[j] = 2;
        let mut b = vec![0; n];
        b[j] = 1;
        out.push(normalise(&[(a, 1), (b, 1)], 2, Ordering::Grevlex));
    }
    out
}

pub fn evaluate(f: &Poly, assignment: &[u64], p: u64) -> u64 {
    f.iter().fold(0, |s, (m, c)| {
        let mut t = *c;
        for (&e, &x) in m.iter().zip(assignment) {
            for _ in 0..e {
                t = (t as u128 * x as u128 % p as u128) as u64;
            }
        }
        (s + t) % p
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::{
        f4_fp::{self, F4Options, Verdict},
        ic_boundary::{koblitz_instance, CountedGroup, GroupOps, PrimePoint},
    };
    #[test]
    fn prime_s3_accepts_real_point_sums_and_membership_is_exact() {
        let c = PrimeCurve { p: 127, a: 0, b: 7 };
        let domain = vec![1, 2, 3, 4, 5];
        let points: Vec<_> = domain
            .iter()
            .flat_map(|&x| {
                c.sqrt(c.rhs(x)).into_iter().flat_map(move |y| {
                    [
                        PrimePoint::affine(x, y),
                        PrimePoint::affine(x, (127 - y) % 127),
                    ]
                })
            })
            .collect();
        let mut checked = 0;
        for a in &points {
            for b in &points {
                let q = c.add(&mut GroupOps::default(), *a, *b);
                if !q.infinity {
                    let eqs = prime(&c, q.x, &domain);
                    assert!(eqs.iter().all(|f| evaluate(f, &[a.x, b.x], 127) == 0));
                    checked += 1;
                }
            }
        }
        assert!(checked > 0);
        let eqs = prime(&c, 13, &domain);
        for x in 0..127 {
            assert_eq!(evaluate(&eqs[1], &[x, 1], 127) == 0, domain.contains(&x));
        }
    }
    #[test]
    fn descended_f4_positive_roots_match_the_original_polynomials() {
        let i = koblitz_instance(1, 7).unwrap();
        let cols = vec![1, 2];
        let v = crate::shapes::subspace(&i, &cols);
        let pts: Vec<_> = (1..4)
            .flat_map(|m| {
                let x = (if m & 1 != 0 { 1 } else { 0 }) ^ (if m & 2 != 0 { 2 } else { 0 });
                i.points_with_x(x)
            })
            .collect();
        let mut target = None;
        for a in &pts {
            for b in &pts {
                let q = i.fast.add(*a, *b);
                if !q.infinity {
                    target = Some(q);
                    break;
                }
            }
            if target.is_some() {
                break;
            }
        }
        let q = target.unwrap();
        let eqs = binary(&i, &v, q.x);
        let result = f4_fp::solve(&eqs, 4, 2, &F4Options::new(Ordering::Grevlex, 8));
        match result.verdict {
            Verdict::Solutions(roots) => {
                assert!(!roots.is_empty());
                for root in roots {
                    assert!(eqs.iter().all(|f| evaluate(f, &root, 2) == 0));
                }
            }
            other => panic!("{other:?}"),
        }
    }
}
