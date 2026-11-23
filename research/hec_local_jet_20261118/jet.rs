use hec_native_ic::{
    fp::Fp,
    jac::{Curve, Div},
    poly::Poly,
};

/// Coefficients of h(x+z), by in-place Horner translation.
pub fn shift(h: &Poly, x: u64, fp: &Fp) -> Poly {
    let mut t = [0; 24];
    for (d, &c) in h.c[..h.n].iter().rev().enumerate() {
        t[d + 1] = t[d];
        for i in (1..=d).rev() {
            t[i] = fp.add(t[i - 1], fp.mul(x, t[i]));
        }
        t[0] = fp.add(c, fp.mul(x, t[0]));
    }
    Poly::from_coeffs(&t[..h.n])
}

/// Both reduced supports in the coordinate z=X-x. Only v values at roots
/// are needed in production, so keep the negative interpolant unreduced.
pub fn pair(c: &Curve, x: u64, y: u64) -> [Div; 2] {
    let fp = &c.fp;
    let f = shift(&c.f, x, fp);
    let mut w = [0; 6];
    w[0] = y;
    let inv = fp.inv(fp.add(y, y));
    for k in 1..=c.g + 1 {
        let mut value = f.c[k];
        for i in 1..k {
            value = fp.sub(value, fp.mul(w[i], w[k - i]));
        }
        w[k] = fp.mul(value, inv);
    }
    std::array::from_fn(|index| {
        let n = c.g + 1 + index;
        let mut coeff = [0; 5];
        for k in n..=n + c.g {
            let mut value = f.c[k];
            for i in k.saturating_sub(n - 1)..n {
                value = fp.sub(value, fp.mul(w[i], w[k - i]));
            }
            coeff[k - n] = value;
        }
        let u = Poly::from_coeffs(&coeff[..=c.g]);
        let u = if u.lc() == 1 { u } else { u.monic(fp) };
        Div {
            u,
            v: Poly::from_coeffs(&w[..n]).neg(fp),
        }
    })
}
