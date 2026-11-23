#![allow(dead_code, unused_imports)]
use hec_native_ic::{
    fp::Fp,
    ic::{solve_dlp, IcParams},
    instance::{factor_base, order_bsgs, prime_factors},
    jac::{Curve, Div},
    poly::Poly,
};
use std::{fs, io::Write};
pub fn irreducible(p: &Poly, f: &Fp) -> bool {
    let n = p.deg() as usize;
    let mut xp = Poly::x();
    for i in 1..=n {
        xp = xp.powmod(f.p, p, f);
        if i <= n / 2 && Poly::gcd(p, &xp.sub(&Poly::x(), f), f).deg() > 0 {
            return false;
        }
    }
    xp == Poly::x()
}
fn primitive(f: &Fp, n: usize) -> u64 {
    let factors = prime_factors(n as u64);
    (2..f.p)
        .find(|&z| f.pow(z, n as u64) == 1 && factors.iter().all(|&q| f.pow(z, n as u64 / q) != 1))
        .expect("rational primitive root")
}
pub fn character(c: &Curve) -> (u64, Vec<i64>) {
    let n = 2 * c.g + 1;
    assert_eq!((c.fp.p - 1) % n as u64, 0);
    let z = primitive(&c.fp, n);
    let mut powers = vec![];
    let mut v = 1;
    for _ in 0..n {
        powers.push(v);
        v = c.fp.mul(v, z)
    }
    assert_eq!(v, 1);
    let mut a = vec![0i64; n];
    for t in 1..c.fp.p {
        let chi = c.fp.pow(t, (c.fp.p - 1) / n as u64);
        let j = powers.iter().position(|&v| v == chi).unwrap();
        a[j] -= c.fp.legendre(c.fp.add(t, c.f.c[0])) as i64;
    }
    (z, a)
}
pub fn certificate(a: &[i64], ell: u64) -> (Poly, Poly, usize, Option<u64>) {
    let f = Fp::new(ell);
    let p = Poly::from_coeffs(&a.iter().map(|&x| f.from_i64(x)).collect::<Vec<_>>());
    let q = Poly::from_coeffs(&vec![1; a.len()]);
    let b = p.sub(&Poly::one(), &f);
    let (g, s, t) = Poly::xgcd(&q, &b, &f);
    assert_eq!(s.mul(&q, &f).add(&t.mul(&b, &f), &f), g);
    let deg = g.deg() as usize;
    let lambda = if deg == 1 {
        assert_eq!(g.lc(), 1);
        let l = f.neg(g.c[0]);
        assert_eq!(p.eval(l, &f), 1);
        assert_eq!(f.pow(l, a.len() as u64), 1);
        Some(l)
    } else {
        None
    };
    (q, b, deg, lambda)
}
pub fn action(c: &Curve, d: &Div, z: u64) -> Div {
    let n = d.u.deg() as u64;
    let iz = c.fp.inv(z);
    let mut u = Poly::zero();
    let mut v = Poly::zero();
    for i in 0..d.u.n {
        u.c[i] = c.fp.mul(d.u.c[i], c.fp.pow(z, n - i as u64))
    }
    u.n = d.u.n;
    for i in 0..d.v.n {
        v.c[i] = c.fp.mul(d.v.c[i], c.fp.pow(iz, i as u64))
    }
    v.n = d.v.n;
    let out = Div { u, v };
    assert!(c.is_valid(&out));
    out
}
fn logs(c: &Curve, fb: &[(u64, u64)], ell: u64) -> Vec<u64> {
    let n = 2 * c.g + 1;
    let f = Fp::new(ell);
    let (z, a) = character(c);
    let (_, _, _, lambda) = certificate(&a, ell);
    let lambda = lambda.expect("linear certificate");
    let mut out = vec![None; fb.len()];
    let (mut x, y, mut value) = (fb[0].0, fb[0].1, 1);
    for _ in 0..n {
        let j = fb.iter().position(|p| p.0 == x).unwrap();
        assert!(out[j].is_none());
        out[j] = Some(if fb[j].1 == y {
            value
        } else {
            assert_eq!(fb[j].1, c.fp.neg(y));
            f.neg(value)
        });
        x = c.fp.mul(x, z);
        value = f.mul(value, lambda);
    }
    assert_eq!(x, fb[0].0);
    assert_eq!(value, 1);
    out.into_iter().map(Option::unwrap).collect()
}
pub fn raw_sum(c: &Curve, fb: &[(u64, u64)], r: &[i64]) -> Div {
    let mut out = c.zero();
    for (&(x, y), &s) in fb.iter().zip(r) {
        let d = c.mul(&c.place(x, y), s.unsigned_abs());
        out = c.add(&out, &if s < 0 { c.neg(&d) } else { d });
    }
    out
}
fn descend(c: &Curve, source: &Div, target: &Div, ell: u64, h: u64) -> (Option<u64>, usize, bool) {
    let f = Fp::new(ell);
    let fb = factor_base(c);
    let l = logs(c, &fb, ell);
    let mut r = *target;
    for a in 0..2048 {
        let mut u = r.u;
        let mut row = vec![0i64; fb.len()];
        for x in 0..c.fp.p {
            let mut m = 0;
            while u.deg() > 0 && u.eval(x, &c.fp) == 0 {
                u = u.div_exact(&Poly::x_minus(x, &c.fp), &c.fp);
                m += 1
            }
            if m != 0 {
                let j = fb.iter().position(|p| p.0 == x).unwrap();
                let y = r.v.eval(x, &c.fp);
                row[j] = if y == fb[j].1 {
                    m
                } else {
                    assert_eq!(y, c.fp.neg(fb[j].1));
                    -m
                };
            }
        }
        if u.deg() == 0 {
            assert_eq!(raw_sum(c, &fb, &row), r);
            let sum = row
                .iter()
                .zip(&l)
                .fold(0, |s, (&a, &b)| f.add(s, f.mul(f.from_i64(a), b)));
            let k = f.sub(f.mul(f.inv(h % ell), sum), a as u64 % ell);
            assert_eq!(c.mul(source, k), *target);
            return (Some(k), a + 1, c.is_zero(&r));
        }
        if a + 1 < 2048 {
            r = c.add(&r, source)
        }
    }
    (None, 2048, false)
}
fn qualify(
    c: &Curve,
    fb: &[(u64, u64)],
    source: &Div,
    ell: u64,
    h: u64,
    z: u64,
    a: &[i64],
    lambda: u64,
) {
    for &(x, y) in fb {
        let d = c.place(x, y);
        let mut current = d;
        let mut sum = c.zero();
        for &v in a {
            let term = c.mul(&current, v.unsigned_abs());
            sum = c.add(&sum, &if v < 0 { c.neg(&term) } else { term });
            current = action(c, &current, z)
        }
        assert_eq!(current, d);
        assert_eq!(sum, d);
    }
    assert_eq!(action(c, source, z), c.mul(source, lambda));
    let l = logs(c, fb, ell);
    for (j, &v) in l.iter().enumerate() {
        assert_eq!(c.mul(source, v), c.mul(&c.place(fb[j].0, fb[j].1), h));
    }
    let n = a.len();
    let mut rev = vec![0; n];
    for j in 0..n {
        rev[(n - j) % n] = a[j]
    }
    let (_, _, deg, inv) = certificate(&rev, ell);
    assert_eq!(deg, 1);
    let inv = inv.unwrap();
    assert_eq!(inv, Fp::new(ell).inv(lambda));
    assert_ne!(c.mul(source, inv), action(c, source, z));
}
