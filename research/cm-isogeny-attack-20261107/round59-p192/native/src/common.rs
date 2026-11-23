#[allow(dead_code)]
#[path = "fp.rs"]
pub mod fp;
pub use num_bigint::BigUint as Z;
pub use num_traits::{One, Zero};
pub use serde_json::{json, Value};
pub use std::{fs, path::Path};
pub type A = (Z, Z);
pub type J = (Z, Z, Z);
pub fn double(f: &fp::Fp, a: &Z, p: &J) -> J {
    if p.2.is_zero() || p.1.is_zero() {
        return (Z::zero(), Z::one(), Z::zero());
    }
    let aa = f.mul(&p.0, &p.0);
    let b = f.mul(&p.1, &p.1);
    let c = f.mul(&b, &b);
    let mut d = f.mul(&f.add(&p.0, &b), &f.add(&p.0, &b));
    d = f.mul(&f.c(2), &f.sub(&f.sub(&d, &aa), &c));
    let zz = f.mul(&p.2, &p.2);
    let e = f.add(&f.mul(&f.c(3), &aa), &f.mul(a, &f.mul(&zz, &zz)));
    let ff = f.mul(&e, &e);
    let x = f.sub(&ff, &f.mul(&f.c(2), &d));
    let y = f.sub(&f.mul(&e, &f.sub(&d, &x)), &f.mul(&f.c(8), &c));
    let z = f.mul(&f.c(2), &f.mul(&p.1, &p.2));
    (x, y, z)
}
pub fn mixed(f: &fp::Fp, a: &Z, p: &J, q: &A) -> J {
    if p.2.is_zero() {
        return (q.0.clone(), q.1.clone(), Z::one());
    }
    let zz = f.mul(&p.2, &p.2);
    let u = f.mul(&q.0, &zz);
    let s = f.mul(&q.1, &f.mul(&p.2, &zz));
    let h = f.sub(&u, &p.0);
    let rr = f.mul(&f.c(2), &f.sub(&s, &p.1));
    if h.is_zero() {
        if rr.is_zero() {
            return double(f, a, p);
        }
        return (Z::zero(), Z::one(), Z::zero());
    }
    let hh = f.mul(&h, &h);
    let i = f.mul(&f.c(4), &hh);
    let j = f.mul(&h, &i);
    let v = f.mul(&p.0, &i);
    let x = f.sub(&f.sub(&f.mul(&rr, &rr), &j), &f.mul(&f.c(2), &v));
    let y = f.sub(
        &f.mul(&rr, &f.sub(&v, &x)),
        &f.mul(&f.c(2), &f.mul(&p.1, &j)),
    );
    let z = f.sub(&f.sub(&f.mul(&f.add(&p.2, &h), &f.add(&p.2, &h)), &zz), &hh);
    (x, y, z)
}
pub fn mul(f: &fp::Fp, a: &Z, n: &Z, q: &A) -> A {
    let mut p = (Z::zero(), Z::one(), Z::zero());
    for k in (0..n.bits()).rev() {
        p = double(f, a, &p);
        if n.bit(k) {
            p = mixed(f, a, &p, q)
        }
    }
    assert!(!p.2.is_zero());
    let iz = f.inv(&p.2);
    let xx = f.mul(&p.0, &f.mul(&iz, &iz));
    let yy = f.mul(&p.1, &f.mul(&f.mul(&iz, &iz), &iz));
    (xx, yy)
}
pub fn point(v: &Value) -> A {
    (fp::parse(&v[0]), fp::parse(&v[1]))
}
