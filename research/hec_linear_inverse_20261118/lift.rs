use hec_native_ic::{
    fp::Fp,
    jac::{Curve, Div},
    poly::Poly,
};

/// Extended Euclid, retaining just the coefficient of the input.
fn compute(input: &Poly, modulus: &Poly, fp: &Fp, mode: usize) -> Option<Poly> {
    let (mut prev, mut rem) = (*modulus, input.rem(modulus, fp));
    let (mut old, mut coeff) = (Poly::zero(), Poly::one());
    if mode >= 2 && modulus.n == 3 && rem.n <= 2 {
        let a = rem.c[1];
        let b = rem.c[0];
        if a == 0 {
            return (b != 0).then(|| Poly::constant(fp.inv(b)));
        }
        let v = fp.sub(b, fp.mul(a, modulus.c[1]));
        let norm = fp.add(fp.mul(b, v), fp.mul(fp.mul(a, a), modulus.c[0]));
        return (norm != 0).then(|| Poly::from_coeffs(&[v, fp.neg(a)]).scale(fp.inv(norm), fp));
    }
    while !rem.is_zero() {
        if mode == 3 && rem.n == 2 {
            let inv_a = fp.inv(rem.c[1]);
            let r = fp.neg(fp.mul(rem.c[0], inv_a));
            let mut q = [0u64; 24];
            let mut carry = prev.lc();
            for i in (1..prev.n).rev() {
                q[i - 1] = carry;
                carry = fp.add(prev.c[i - 1], fp.mul(r, carry));
            }
            if carry == 0 {
                return None;
            }
            let quotient = Poly::from_coeffs(&q[..prev.n - 1]).scale(inv_a, fp);
            return Some(
                old.sub(&coeff.mul(&quotient, fp), fp)
                    .scale(fp.inv(carry), fp),
            );
        }
        let (q, next) = prev.divrem(&rem, fp);

        (prev, rem) = (rem, next);
        (old, coeff) = (coeff, old.sub(&q.mul(&coeff, fp), fp));
    }
    (prev.deg() == 0).then(|| old.scale(fp.inv(prev.lc()), fp))
}

fn inverse(input: &Poly, modulus: &Poly, fp: &Fp, mode: usize, audit: bool) -> Option<Poly> {
    let result = compute(input, modulus, fp, mode);
    if audit {
        match result {
            Some(v) => assert_eq!(v.mulmod(input, modulus, fp), Poly::one()),
            None => assert!(Poly::gcd(input, modulus, fp).deg() > 0),
        }
    }
    result
}
pub fn double(c: &Curve, d: &Div, mode: usize, audit: bool) -> Div {
    if c.is_zero(d) {
        return *d;
    }
    let fp = &c.fp;
    let twice = d.v.add(&d.v, fp);
    let Some(inv) = inverse(&twice, &d.u, fp, mode, audit) else {
        return c.add(d, d);
    };
    let h = c.f.sub(&d.v.mul(&d.v, fp), fp).div_exact(&d.u, fp);
    let k = h.mulmod(&inv, &d.u, fp);
    let w = d.v.add(&d.u.mul(&k, fp), fp);
    if 2 * d.u.deg() <= c.g as isize {
        return Div {
            u: d.u.mul(&d.u, fp),
            v: w,
        };
    }
    let support = h
        .sub(&twice.mul(&k, fp), fp)
        .div_exact(&d.u, fp)
        .sub(&k.mul(&k, fp), fp);
    c.reduce(support, w.neg(fp))
}

pub fn add(c: &Curve, a: &Div, b: &Div, mode: usize, audit: bool) -> Div {
    if c.is_zero(a) {
        return *b;
    }
    if c.is_zero(b) {
        return *a;
    }
    let fp = &c.fp;
    let Some(inv) = inverse(&a.u, &b.u, fp, mode, audit) else {
        return c.add(a, b);
    };
    let k = b.v.sub(&a.v, fp).mulmod(&inv, &b.u, fp);
    let v = a.v.add(&a.u.mul(&k, fp), fp);
    c.reduce(a.u.mul(&b.u, fp), v)
}
