//! Checked four-point/one-quadratic Nagao complements and anchor cancellation.
use crate::remainder::{self, Certificate, Poly, Remainder};
use crypto_lib::cryptanalysis::{
    jv_bitops as work,
    jv_cover::{BaseEl, Div, Fld, Fq, Hyp, E2},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
fn neg(f: &Fq, a: &[E2]) -> Poly {
    remainder::trim(a.iter().map(|x| f.neg(x)).collect())
}
fn add(f: &Fq, a: &[E2], b: &[E2]) -> Poly {
    remainder::sub(f, a, &neg(f, b))
}
fn monic(f: &Fq, a: &[E2]) -> Poly {
    let Some(last) = a.last() else {
        return vec![];
    };
    let inv = f.inv(last);
    remainder::trim(a.iter().map(|x| f.mul(x, &inv)).collect())
}
fn gcd(f: &Fq, a: &[E2], b: &[E2]) -> Poly {
    let (mut a, mut b) = (remainder::trim(a.to_vec()), monic(f, b));
    while !b.is_empty() {
        if b.len() == 1 {
            return vec![E2::ONE];
        }
        let c = remainder::modulo(f, &a, &b).expect("monic gcd divisor");
        a = b;
        b = monic(f, &c);
    }
    monic(f, &a)
}
fn derivative(f: &Fq, a: &[E2]) -> Poly {
    remainder::trim(
        a.iter()
            .enumerate()
            .skip(1)
            .map(|(i, x)| f.scale(x, i as u64))
            .collect(),
    )
}
fn canonical(f: &Fq, u: &[E2], v: &[E2]) -> Remainder {
    let a = v.first().copied().unwrap_or(E2::ZERO);
    let b = v.get(1).copied().unwrap_or(E2::ZERO);
    let (na, nb) = (f.neg(&a), f.neg(&b));
    let key = [u[0].0[0], u[1].0[0], a.0[0], a.0[1], b.0[0], b.0[1]];
    let opposite = [u[0].0[0], u[1].0[0], na.0[0], na.0[1], nb.0[0], nb.0[1]];
    work::words(18);
    if opposite < key {
        Remainder {
            key: opposite,
            coefficient: -1,
        }
    } else {
        Remainder {
            key,
            coefficient: 1,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Partial {
    /// Principal-function signs: R + sum(sign*B) + coefficient*D = 0.
    pub terms: Vec<(usize, i8)>,
    pub remainder: Remainder,
    pub a: Vec<[u64; 2]>,
    pub c: Vec<[u64; 2]>,
    pub sextic: Vec<u64>,
}
#[derive(Clone)]
pub struct Verified {
    certificate: Partial,
    residual: Div<E2>,
}
impl Verified {
    pub fn certificate(&self) -> &Partial {
        &self.certificate
    }
    pub fn into_certificate(self) -> Partial {
        self.certificate
    }
}
/// Prove an exact principal divisor with disjoint, nonramified support.
/// This constructor does not use the Cantor group-law verifier below.
pub fn verify(
    f: &Fq,
    h: &[E2],
    base: &[BaseEl],
    r: &Div<E2>,
    a: &[E2],
    c: &[E2],
    sextic: &[u64],
    terms: &[(usize, i8)],
    tail: &[u64],
) -> Result<Verified, &'static str> {
    if h.len() != 8
        || h[7] != E2::ONE
        || r.u.len() != 4
        || r.u[3] != E2::ONE
        || r.v.len() > 3
        || a.len() > 5
        || c.len() != 2
        || c[1] != E2::ONE
        || sextic.len() != 7
        || sextic[6] != 1
        || terms.len() != 4
        || tail.len() != 3
        || tail[2] != 1
        || terms.windows(2).any(|w| w[0].0 >= w[1].0)
        || sextic.iter().chain(tail).any(|&x| x >= f.p)
        || h.iter()
            .chain(&r.u)
            .chain(&r.v)
            .chain(a)
            .chain(c)
            .any(|x| x.0.iter().any(|&v| v >= f.p))
    {
        return Err("canonical principal shapes");
    }
    let jac = Hyp {
        f,
        h: h.to_vec(),
        g: 3,
    };
    if !jac.is_valid(r) {
        return Err("invalid residual divisor");
    }
    if gcd(f, &r.u, &derivative(f, &r.u)).len() != 1
        || gcd(f, &r.u, h).len() != 1
        || gcd(f, c, &r.u).len() != 1
        || gcd(f, &r.u, &remainder::embed(sextic)).len() != 1
    {
        return Err("residual squarefree/disjoint/nonramified/coprime");
    }
    if !remainder::modulo(f, &add(f, a, &remainder::mul(f, c, &r.v)), &r.u)?.is_empty() {
        return Err("principal residual ordinate");
    }
    let actual = remainder::sub(
        f,
        &remainder::mul(f, a, a),
        &remainder::mul(f, &remainder::mul(f, c, c), h),
    );
    if actual != neg(f, &remainder::mul(f, &r.u, &remainder::embed(sextic))) {
        return Err("exact residual norm quotient");
    }
    let mut support = vec![E2::ONE];
    let mut previous = None;
    for &(index, sign) in terms {
        let b = base.get(index).ok_or("small point index")?;
        if ![-1, 1].contains(&sign)
            || b.x >= f.p
            || b.y == E2::ZERO
            || b.y.0.iter().any(|&x| x >= f.p)
            || previous.is_some_and(|x| x >= b.x)
            || b.d
                != (Div {
                    u: vec![f.neg(&f.from_fp(b.x)), E2::ONE],
                    v: vec![b.y],
                })
        {
            return Err("small support shape");
        }
        previous = Some(b.x);
        let y = if sign == 1 { b.y } else { f.neg(&b.y) };
        let den = remainder::eval(f, c, b.x);
        if den == E2::ZERO
            || f.mul(&y, &y) != remainder::eval(f, h, b.x)
            || f.add(&remainder::eval(f, a, b.x), &f.mul(&den, &y)) != E2::ZERO
        {
            return Err("small point principal binding");
        }
        support = remainder::mul(f, &support, &[f.neg(&f.from_fp(b.x)), E2::ONE]);
        work::words(7);
    }
    if remainder::mul(f, &support, &remainder::embed(tail)) != remainder::embed(sextic) {
        return Err("exact small/quadratic support");
    }
    let u = remainder::embed(tail);
    let disc = f.sub(&f.mul(&u[1], &u[1]), &f.scale(&u[0], 4));
    if disc == E2::ZERO || f.pow(&disc, (f.p - 1) as u128 / 2) != f.from_fp(f.p - 1) {
        return Err("irreducible quadratic required");
    }
    let inv = remainder::inverse_linear(f, c, &u)?;
    let v = neg(
        f,
        &remainder::modulo(
            f,
            &remainder::mul(f, &remainder::modulo(f, a, &u)?, &inv),
            &u,
        )?,
    );
    if !remainder::modulo(f, &remainder::sub(f, &remainder::mul(f, &v, &v), h), &u)?.is_empty() {
        return Err("quadratic class validity");
    }
    Ok(Verified {
        residual: r.clone(),
        certificate: Partial {
            terms: terms.to_vec(),
            remainder: canonical(f, &u, &v),
            a: remainder::encode(a),
            c: remainder::encode(c),
            sextic: sextic.to_vec(),
        },
    })
}
/// Independent group-law replay of R+small+D=0, without norm construction.
pub fn replay(jac: &Hyp<Fq>, base: &[BaseEl], r: &Div<E2>, row: &Partial) -> bool {
    if !jac.is_valid(r)
        || row.terms.len() != 4
        || row.terms.windows(2).any(|w| w[0].0 >= w[1].0)
        || ![-1, 1].contains(&row.remainder.coefficient)
        || row.remainder.key.iter().any(|&x| x >= jac.f.p)
    {
        return false;
    }
    let mut sum = r.clone();
    for &(i, s) in &row.terms {
        let Some(b) = base.get(i) else {
            return false;
        };
        if ![-1, 1].contains(&s) {
            return false;
        }
        let d = if s == 1 { b.d.clone() } else { jac.neg(&b.d) };
        if !jac.is_valid(&d) {
            return false;
        }
        sum = jac.add(&sum, &d);
    }
    let d = row.remainder.divisor();
    if !jac.is_valid(&d) {
        return false;
    }
    let d = if row.remainder.coefficient == 1 {
        d
    } else {
        jac.neg(&d)
    };
    jac.is_identity(&jac.add(&sum, &d))
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Projected {
    pub small: Vec<(usize, i64)>,
    pub anchor_coefficient: i8,
    pub eta: u64,
}
/// R=-sum(principal small)+cD*cA*sum(anchor small). The witness is
/// integer-valued; reducing coefficients modulo L happens only for eta.
pub fn project(
    jac: &Hyp<Fq>,
    base: &[BaseEl],
    checked: &Verified,
    anchor: &Certificate,
    h: &[u64],
    l: u64,
) -> Result<Projected, &'static str> {
    let row = checked.certificate();
    let ar = anchor.remainder.as_ref().ok_or("anchor is not partial")?;
    if ar.key != row.remainder.key
        || ![-1, 1].contains(&ar.coefficient)
        || h.len() != base.len()
        || h.iter().any(|&x| x >= l)
        || !(3..1u64 << 63).contains(&l)
        || !remainder::replay_group(jac, base, anchor)
        || !replay(jac, base, &checked.residual, row)
        || row.terms.iter().chain(&anchor.terms).any(|&(i, _)| {
            base.get(i).is_none_or(|b| {
                b.x >= jac.f.p
                    || b.y.0.iter().any(|&x| x >= jac.f.p)
                    || b.d
                        != (Div {
                            u: vec![jac.f.neg(&jac.f.from_fp(b.x)), E2::ONE],
                            v: vec![b.y],
                        })
            })
        })
    {
        return Err("anchor or kernel binding");
    }
    let coefficient = row.remainder.coefficient * ar.coefficient;
    let mut small = BTreeMap::new();
    for &(i, s) in &row.terms {
        *small.entry(i).or_insert(0i64) -= s as i64;
        work::words(4);
        work::probe(1);
    }
    for &(i, s) in &anchor.terms {
        *small.entry(i).or_insert(0i64) += (coefficient * s) as i64;
        work::words(4);
        work::probe(1);
    }
    let small: Vec<_> = small.into_iter().filter(|&(_, x)| x != 0).collect();
    let mut eta = 0;
    for &(i, x) in &small {
        let v = if x >= 0 {
            x as u64 % l
        } else {
            (l - ((-x) as u64 % l)) % l
        };
        if v != 0 && h[i] != 0 {
            work::hook_mul(l);
            let term = (v as u128 * h[i] as u128 % l as u128) as u64;
            work::hook_add(l);
            eta = ((eta as u128 + term as u128) % l as u128) as u64;
        }
        work::words(5);
    }
    Ok(Projected {
        small,
        anchor_coefficient: coefficient,
        eta,
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub(crate) fn fixture() -> (
        Fq,
        Poly,
        Vec<BaseEl>,
        Div<E2>,
        Poly,
        Poly,
        Vec<u64>,
        Vec<(usize, i8)>,
        Vec<u64>,
    ) {
        let f = Fq::new(101);
        let tail = vec![99, 0, 1];
        let mut small = remainder::embed(&tail);
        for x in 1..=4 {
            small = remainder::mul(&f, &small, &[f.neg(&f.from_fp(x)), E2::ONE]);
        }
        let mut ur = vec![E2::ONE];
        for x in [8, 9, 10] {
            ur = remainder::mul(&f, &ur, &[f.neg(&f.from_fp(x)), E2::ONE]);
        }
        let norm = neg(&f, &remainder::mul(&f, &ur, &small));
        let a0 = E2([0, 30]);
        assert_eq!(f.mul(&a0, &a0), norm[0]);
        let a1 = f.mul(&norm[1], &f.inv(&f.scale(&a0, 2)));
        let a = vec![a0, a1, E2([1, 3])];
        let c = vec![E2::ZERO, E2::ONE];
        let numerator = remainder::sub(&f, &remainder::mul(&f, &a, &a), &norm);
        assert_eq!(&numerator[..2], &[E2::ZERO, E2::ZERO]);
        let h = numerator[2..].to_vec();
        let u0inv = f.inv(&ur[0]);
        let invx = vec![
            f.neg(&f.mul(&ur[1], &u0inv)),
            f.neg(&f.mul(&ur[2], &u0inv)),
            f.neg(&u0inv),
        ];
        let vr = neg(
            &f,
            &remainder::modulo(&f, &remainder::mul(&f, &a, &invx), &ur).unwrap(),
        );
        let r = Div { u: ur, v: vr };
        let base = [1, 2, 3, 4, 8, 9, 10]
            .into_iter()
            .map(|x| {
                let y = f.neg(&f.mul(&remainder::eval(&f, &a, x), &f.inv(&f.from_fp(x))));
                BaseEl {
                    x,
                    y,
                    d: Div {
                        u: vec![f.neg(&f.from_fp(x)), E2::ONE],
                        v: vec![y],
                    },
                }
            })
            .collect();
        let sextic = small
            .iter()
            .map(|x| {
                assert!(x.in_fp());
                x.0[0]
            })
            .collect();
        (
            f,
            h,
            base,
            r,
            a,
            c,
            sextic,
            (0..4).map(|i| (i, 1)).collect(),
            tail,
        )
    }
    #[test]
    fn principal_four_plus_quadratic_agrees_with_independent_cantor() {
        let (f, h, base, r, a, c, s, terms, tail) = fixture();
        let checked = verify(&f, &h, &base, &r, &a, &c, &s, &terms, &tail).unwrap();
        let jac = Hyp { f: &f, h, g: 3 };
        assert!(replay(&jac, &base, &r, checked.certificate()));
    }
    #[test]
    fn signed_anchor_witness_cancels_overlap_and_returns_actual_residual() {
        let (f, h, base, r, a, c, s, terms, tail) = fixture();
        let checked = verify(&f, &h, &base, &r, &a, &c, &s, &terms, &tail).unwrap();
        let norm = neg(&f, &remainder::mul(&f, &r.u, &remainder::embed(&s)))
            .iter()
            .map(|x| {
                assert!(x.in_fp());
                x.0[0]
            })
            .collect::<Vec<_>>();
        let all = (0..7).map(|i| (i, 1)).collect::<Vec<_>>();
        let anchor_tail = tail.iter().map(|&x| (f.p - x) % f.p).collect::<Vec<_>>();
        let anchor = remainder::check_equation(&f, &h, &base, &a, &c, &norm, &all, &anchor_tail)
            .unwrap()
            .into_certificate();
        let jac = Hyp { f: &f, h, g: 3 };
        let out = project(&jac, &base, &checked, &anchor, &[1, 2, 3, 4, 5, 6, 7], 103).unwrap();
        assert_eq!(out.small, vec![(4, 1), (5, 1), (6, 1)]);
        assert_eq!(out.anchor_coefficient, 1);
        assert_eq!(out.eta, 18);
        let mut sum = jac.identity();
        for &(i, x) in &out.small {
            let d = jac.mul(&base[i].d, x.unsigned_abs() as u128);
            let d = if x < 0 { jac.neg(&d) } else { d };
            sum = jac.add(&sum, &d);
        }
        assert_eq!(sum, r);
        let mut bad = anchor;
        bad.remainder.as_mut().unwrap().key[2] = (bad.remainder.as_ref().unwrap().key[2] + 1) % f.p;
        assert!(project(&jac, &base, &checked, &bad, &[1, 2, 3, 4, 5, 6, 7], 103).is_err());
    }
    #[test]
    fn forged_norm_residual_support_and_ordinates_refused() {
        let (f, h, mut base, r, mut a, c, mut s, terms, tail) = fixture();
        s[0] = (s[0] + 1) % f.p;
        assert!(verify(&f, &h, &base, &r, &a, &c, &s, &terms, &tail).is_err());
        s[0] = (s[0] + f.p - 1) % f.p;
        a[0] = f.add(&a[0], &E2::ONE);
        assert!(verify(&f, &h, &base, &r, &a, &c, &s, &terms, &tail).is_err());
        a[0] = f.sub(&a[0], &E2::ONE);
        base[0].y = f.neg(&base[0].y);
        assert!(verify(&f, &h, &base, &r, &a, &c, &s, &terms, &tail).is_err());
        base[0].y = f.neg(&base[0].y);
        let mut bad = r.clone();
        bad.v[0] = f.add(&bad.v[0], &E2::ONE);
        assert!(verify(&f, &h, &base, &bad, &a, &c, &s, &terms, &tail).is_err());
        let mut bad = terms;
        bad[1].0 = bad[0].0;
        assert!(verify(&f, &h, &base, &r, &a, &c, &s, &bad, &tail).is_err());
    }
}
