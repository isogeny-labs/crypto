use hec_native_ic::{
    fp::Fp,
    instance::factor_base,
    jac::{Curve, Div},
    poly::Poly,
};
use std::{fs, io::Write};
pub fn action(c: &Curve, d: &Div, i: u64) -> Div {
    let mut u = d.u;
    let mut v = d.v;
    let degree = u.deg() as usize;
    for j in 0..u.n {
        if (degree + j) % 2 != 0 {
            u.c[j] = c.fp.neg(u.c[j])
        }
    }
    for j in 0..v.n {
        v.c[j] = c.fp.mul(v.c[j], if j % 2 == 0 { i } else { c.fp.neg(i) })
    }
    let out = Div { u, v };
    assert!(c.is_valid(&out));
    out
}
pub fn qualify(
    c: &Curve,
    n: u64,
    ell: u64,
    receipt: &mut fs::File,
    p: u64,
    code: usize,
) -> Option<(Div, u64, u64, usize)> {
    let h = n / ell;
    let i = c.fp.sqrt(p - 1).unwrap();
    let f = Fp::new(ell);
    let l = f.sqrt(ell - 1)?;
    let fb = factor_base(c);
    let d = fb
        .iter()
        .map(|&(x, y)| c.mul(&c.place(x, y), h))
        .find(|d| !c.is_zero(d))?;
    assert!(c.is_zero(&c.mul(&d, ell)));
    let matches: Vec<_> = [l, ell - l]
        .into_iter()
        .filter(|&x| c.mul(&d, x) == action(c, &d, i))
        .collect();
    assert_eq!(matches.len(), 1);
    let lambda = matches[0];
    for &(x, y) in &fb {
        let raw = c.place(x, y);
        assert!(c.is_zero(&c.mul(&raw, n)));
        let proj = c.mul(&raw, h);
        assert!(c.is_zero(&c.mul(&proj, ell)));
        assert_eq!(action(c, &proj, i), c.mul(&proj, lambda));
        assert_eq!(action(c, &action(c, &raw, i), i), c.neg(&raw));
        if y == 0 {
            assert!(c.is_zero(&proj))
        } else {
            let bad = Div {
                u: Poly::x_minus(c.fp.neg(x), &c.fp),
                v: Poly::constant(y),
            };
            assert!(!c.is_valid(&bad))
        }
    }
    let mut seen = vec![false; fb.len()];
    let mut orbits = 0;
    for j in 0..fb.len() {
        if seen[j] {
            continue;
        }
        if fb[j].1 == 0 {
            seen[j] = true;
            writeln!(receipt, "{p}\t{code}\t{}\t{}\tnone\t0", fb[j].0, fb[j].1).unwrap();
            continue;
        }
        orbits += 1;
        let (mut x, mut y, mut weight) = (fb[j].0, fb[j].1, 1);
        let rep = c.mul(&c.place(x, y), h);
        for _ in 0..2 {
            let k = fb.iter().position(|v| v.0 == x).unwrap();
            assert!(!seen[k]);
            seen[k] = true;
            let w = if fb[k].1 == y {
                weight
            } else {
                assert_eq!(y, c.fp.neg(fb[k].1));
                f.neg(weight)
            };
            assert_eq!(c.mul(&c.place(fb[k].0, fb[k].1), h), c.mul(&rep, w));
            writeln!(
                receipt,
                "{p}\t{code}\t{}\t{}\t{}\t{w}",
                fb[k].0, fb[k].1, fb[j].0
            )
            .unwrap();
            x = c.fp.neg(x);
            y = c.fp.mul(i, y);
            weight = f.mul(weight, lambda)
        }
        assert_eq!((x, y, weight), (fb[j].0, c.fp.neg(fb[j].1), ell - 1));
    }
    assert!(seen.iter().all(|v| *v));
    Some((d, i, lambda, orbits))
}
