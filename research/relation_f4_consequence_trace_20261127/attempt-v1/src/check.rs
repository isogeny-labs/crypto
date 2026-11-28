//! Independent small sparse checker: no F4 arithmetic/reduction calls.
use crypto_lib::cryptanalysis::f4_fp::Poly;
use std::{cmp::Ordering, collections::BTreeMap};
type Sparse = BTreeMap<Vec<u32>, u64>;

fn cmp(a: &[u32], b: &[u32]) -> Ordering {
    a.iter()
        .map(|x| *x as u64)
        .sum::<u64>()
        .cmp(&b.iter().map(|x| *x as u64).sum::<u64>())
        .then_with(|| {
            a.iter()
                .rev()
                .zip(b.iter().rev())
                .find_map(|(a, b)| (a != b).then(|| b.cmp(a)))
                .unwrap_or(Ordering::Equal)
        })
}
fn lead(f: &Sparse) -> Option<(&Vec<u32>, u64)> {
    f.iter()
        .max_by(|(a, _), (b, _)| cmp(a, b))
        .map(|(m, c)| (m, *c))
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    (a as u128 * b as u128 % p as u128) as u64
}
fn inv(mut a: u64, p: u64) -> u64 {
    let mut r = 1;
    let mut e = p - 2;
    while e > 0 {
        if e & 1 == 1 {
            r = mul(r, a, p);
        }
        a = mul(a, a, p);
        e >>= 1;
    }
    r
}
fn decode(f: &Poly, n: usize, p: u64) -> Result<Sparse, String> {
    let mut out = Sparse::new();
    for (m, c) in f {
        if m.len() != n
            || m.iter().any(|e| *e > 64)
            || *c == 0
            || *c >= p
            || out.insert(m.clone(), *c).is_some()
        {
            return Err("noncanonical polynomial".into());
        }
    }
    Ok(out)
}
fn add(f: &mut Sparse, m: Vec<u32>, c: u64, p: u64) {
    let value = ((f.get(&m).copied().unwrap_or(0) as u128 + c as u128) % p as u128) as u64;
    if value == 0 {
        f.remove(&m);
    } else {
        f.insert(m, value);
    }
}
fn axpy(
    f: &mut Sparse,
    g: &Sparse,
    shift: &[u32],
    c: u64,
    p: u64,
    budget: &mut usize,
) -> Result<(), String> {
    for (m, d) in g {
        *budget = budget.checked_sub(1).ok_or("independent checker budget")?;
        let monomial = m
            .iter()
            .zip(shift)
            .map(|(a, b)| a.checked_add(*b).ok_or("exponent overflow"))
            .collect::<Result<Vec<_>, _>>()?;
        add(f, monomial, mul(c, *d, p), p);
    }
    Ok(())
}
fn reduce(mut f: Sparse, gb: &[Sparse], p: u64, budget: &mut usize) -> Result<bool, String> {
    while let Some((m, c)) = lead(&f).map(|(m, c)| (m.clone(), c)) {
        *budget = budget.checked_sub(1).ok_or("independent checker budget")?;
        let Some((g, l, d)) = gb
            .iter()
            .filter_map(|g| lead(g).map(|(l, d)| (g, l, d)))
            .find(|(_, l, _)| l.iter().zip(&m).all(|(a, b)| a <= b))
        else {
            return Ok(false);
        };
        let shift: Vec<_> = m.iter().zip(l).map(|(a, b)| a - b).collect();
        let factor = mul(c, inv(d, p), p);
        axpy(&mut f, g, &shift, p - factor, p, budget)?;
        if lead(&f).is_some_and(|(next, _)| cmp(next, &m) != Ordering::Less) {
            return Err("reduction did not decrease grevlex leading term".into());
        }
    }
    Ok(true)
}

/// Validates both inclusions and Buchberger, relative to an independently
/// certified reference basis of the originals. Never constructs a basis.
pub fn validate(
    basis: &[Poly],
    reference: &[Poly],
    n: usize,
    p: u64,
    mut budget: usize,
) -> Result<usize, String> {
    let initial = budget;
    if basis.is_empty() || reference.is_empty() {
        return Err("empty basis/reference".into());
    }
    let gb = basis
        .iter()
        .map(|g| decode(g, n, p))
        .collect::<Result<Vec<_>, _>>()?;
    let rb = reference
        .iter()
        .map(|g| decode(g, n, p))
        .collect::<Result<Vec<_>, _>>()?;
    for g in &gb {
        if lead(g).is_none_or(|(_, c)| c != 1) {
            return Err("basis not nonzero monic".into());
        }
        if !reduce(g.clone(), &rb, p, &mut budget)? {
            return Err("F4 row outside original ideal".into());
        }
    }
    for g in &rb {
        if !reduce(g.clone(), &gb, p, &mut budget)? {
            return Err("reference row outside F4 ideal".into());
        }
    }
    for i in 0..gb.len() {
        for j in i + 1..gb.len() {
            let (a, ac) = lead(&gb[i]).unwrap();
            let (b, bc) = lead(&gb[j]).unwrap();
            let lcm: Vec<_> = a.iter().zip(b).map(|(a, b)| (*a).max(*b)).collect();
            let sa: Vec<_> = lcm.iter().zip(a).map(|(l, a)| l - a).collect();
            let sb: Vec<_> = lcm.iter().zip(b).map(|(l, b)| l - b).collect();
            let mut s = Sparse::new();
            axpy(&mut s, &gb[i], &sa, inv(ac, p), p, &mut budget)?;
            axpy(&mut s, &gb[j], &sb, p - inv(bc, p), p, &mut budget)?;
            if !reduce(s, &gb, p, &mut budget)? {
                return Err("Buchberger S-pair nonzero".into());
            }
        }
    }
    Ok(initial - budget)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn p(terms: &[(&[u32], u64)]) -> Poly {
        terms.iter().map(|(m, c)| (m.to_vec(), *c)).collect()
    }
    #[test]
    fn accepts_basis_and_nonreduced_equal_ideal() {
        let x = p(&[(&[1, 0], 1)]);
        let y = p(&[(&[0, 1], 1)]);
        let xy = p(&[(&[1, 1], 1)]);
        assert!(validate(&[x.clone(), y.clone(), xy], &[x, y], 2, 2147483647, 10000).is_ok());
    }
    #[test]
    fn rejects_strict_subideal() {
        let x = p(&[(&[1, 0], 1)]);
        let y = p(&[(&[0, 1], 1)]);
        assert!(validate(&[x.clone()], &[x, y], 2, 2147483647, 10000).is_err());
    }
    #[test]
    fn rejects_coefficient_mutation_both_fields() {
        for modulus in [2147483647, 9007199254740881] {
            let x = p(&[(&[1], 1), (&[0], modulus - 1)]);
            let bad = p(&[(&[1], 1), (&[0], modulus - 2)]);
            assert!(validate(&[x.clone()], &[x.clone()], 1, modulus, 10000).is_ok());
            assert!(validate(&[bad], &[x], 1, modulus, 10000).is_err());
        }
    }
    #[test]
    fn rejects_equal_ideal_non_groebner_generators() {
        // <x^2-y,xy-1> has S-pair x-y^2, irreducible by these rows.
        let a = p(&[(&[2, 0], 1), (&[0, 1], 100)]);
        let b = p(&[(&[1, 1], 1), (&[0, 0], 100)]);
        let c = p(&[(&[0, 2], 1), (&[1, 0], 100)]);
        let d = p(&[(&[1, 1], 1), (&[0, 0], 100)]);
        let e = p(&[(&[2, 0], 1), (&[0, 1], 100)]);
        let error = validate(&[a.clone(), b.clone()], &[a, b], 2, 101, 10000).unwrap_err();
        assert!(error.contains("Buchberger"));
        assert!(validate(
            &[c.clone(), d.clone(), e.clone()],
            &[c, d, e],
            2,
            101,
            10000
        )
        .is_ok());
    }
    #[test]
    fn budget_and_malformed_fail_closed() {
        let x = p(&[(&[1], 1)]);
        assert!(validate(&[x.clone()], &[x.clone()], 1, 101, 0).is_err());
        assert!(validate(
            &[vec![(vec![1], 1), (vec![1], 1)]],
            &[x.clone()],
            1,
            101,
            100
        )
        .is_err());
        assert!(validate(&[p(&[(&[1], 101)])], &[x], 1, 101, 100).is_err());
        assert!(validate(&[], &[], 1, 101, 100).is_err());
    }
    #[test]
    fn grevlex_order_and_exact_large_products() {
        assert_eq!(cmp(&[2, 0], &[1, 1]), Ordering::Greater);
        assert_eq!(cmp(&[0, 2], &[3, 0]), Ordering::Less);
        for p in [2147483647, 9007199254740881] {
            assert_eq!(mul(p - 1, p - 1, p), 1);
            assert_eq!(mul(1234567, inv(1234567, p), p), 1);
        }
    }
}
