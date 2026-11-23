use hec_native_ic::{
    fp::Fp,
    jac::{Curve, Div},
    poly::Poly,
};

/// Extended Euclid, retaining just the coefficient of the input.
fn inverse(input: &Poly, modulus: &Poly, fp: &Fp) -> Option<Poly> {
    let (mut prev, mut rem) = (*modulus, input.rem(modulus, fp));
    let (mut old, mut coeff) = (Poly::zero(), Poly::one());
    while !rem.is_zero() {
        let (q, next) = prev.divrem(&rem, fp);
        (prev, rem) = (rem, next);
        (old, coeff) = (coeff, old.sub(&q.mul(&coeff, fp), fp));
    }
    (prev.deg() == 0).then(|| old.scale(fp.inv(prev.lc()), fp))
}

pub fn double(c: &Curve, d: &Div) -> Div {
    if c.is_zero(d) {
        return *d;
    }
    let fp = &c.fp;
    let twice = d.v.add(&d.v, fp);
    let Some(inv) = inverse(&twice, &d.u, fp) else {
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

pub fn add(c: &Curve, a: &Div, b: &Div) -> Div {
    if c.is_zero(a) {
        return *b;
    }
    if c.is_zero(b) {
        return *a;
    }
    let fp = &c.fp;
    let Some(inv) = inverse(&a.u, &b.u, fp) else {
        return c.add(a, b);
    };
    let k = b.v.sub(&a.v, fp).mulmod(&inv, &b.u, fp);
    let v = a.v.add(&a.u.mul(&k, fp), fp);
    c.reduce(a.u.mul(&b.u, fp), v)
}
