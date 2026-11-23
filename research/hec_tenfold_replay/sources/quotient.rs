use hec_native_ic::{
    fp::Fp,
    jac::{Curve, Div},
    poly::Poly,
};

fn coefficient(a: &Poly, b: &Poly, degree: usize, fp: &Fp) -> u64 {
    let mut sum = 0;
    for i in 0..a.n.min(degree + 1) {
        let j = degree - i;
        if j < b.n {
            sum = fp.add(sum, fp.mul(a.c[i], b.c[j]));
        }
    }
    sum
}

fn quadratic(c: &Curve, a: &Poly, b: &Poly, w: &Poly) -> Poly {
    let f = &c.fp;
    let u3 = f.add(a.c[1], b.c[1]);
    let u2 = f.add(f.add(a.c[0], b.c[0]), f.mul(a.c[1], b.c[1]));
    let t = w.c[3];
    let twice = f.add(t, t);
    let q2 = f.neg(f.mul(t, t));
    let q1 = f.sub(f.sub(c.f.c[5], f.mul(twice, w.c[2])), f.mul(u3, q2));
    let n4 = f.sub(f.sub(c.f.c[4], f.mul(w.c[2], w.c[2])), f.mul(twice, w.c[1]));
    let q0 = f.sub(f.sub(n4, f.mul(u3, q1)), f.mul(u2, q2));
    Poly::from_coeffs(&[q0, q1, q2])
}

pub fn reduce(c: &Curve, a: &Poly, b: &Poly, w: &Poly, audit: bool) -> Div {
    let fp = &c.fp;
    let d = a.n + b.n - 2;
    if d <= c.g {
        return c.reduce(a.mul(b, fp), *w);
    }
    let support = if c.g == 2 && a.n == 3 && b.n == 3 {
        quadratic(c, a, b, w)
    } else {
        let top = c.f.deg().max(2 * w.deg()) as usize;
        let e = top - d;
        let mut divisor = [0; 24];
        for j in 1..=e.min(d) {
            divisor[j] = coefficient(a, b, d - j, fp);
        }
        let mut q = [0; 24];
        for i in (0..=e).rev() {
            let k = d + i;
            let mut value = fp.sub(c.f.c[k], coefficient(w, w, k, fp));
            for j in 1..=(e - i).min(d) {
                value = fp.sub(value, fp.mul(divisor[j], q[i + j]));
            }
            q[i] = value;
        }
        Poly::from_coeffs(&q[..=e])
    };
    if audit {
        assert_eq!(support.mul(&a.mul(b, fp), fp), c.f.sub(&w.mul(w, fp), fp));
    }
    c.reduce(support, w.neg(fp))
}
