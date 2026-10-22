//! Exact certificate for the coprime 2+2, C=0, degree-one output branch.
//! Parametrizes C=0 as (ds,dt)=h*(dq,dr) when B!=0 over a field.
#[allow(dead_code)]
#[path = "projective_symbolic_12.rs"]
mod base;

use base::{P, U};
use serde_json::json;

fn rem_monic(mut rem: U, divisor: &U) -> U {
    assert_eq!(divisor.coeff(2), P::c(1));
    for degree in (2..=6).rev() {
        let lead = rem.coeff(degree);
        rem = rem.sub(&U::x().pow(degree - 2).mul(&divisor.scale(&lead)));
    }
    rem
}

trait PowerU {
    fn pow(&self, n: usize) -> U;
}
impl PowerU for U {
    fn pow(&self, n: usize) -> U {
        let mut result = U::c(P::c(1));
        for _ in 0..n {
            result = result.mul(self);
        }
        result
    }
}

fn main() {
    // Independent q1,r1,s1,t1,q2,r2,h,z1,w1,z2,w2.
    let v: Vec<P> = (0..11).map(P::var).collect();
    let (q1, r1, s1, t1, q2, r2, h) = (&v[0], &v[1], &v[2], &v[3], &v[4], &v[5], &v[6]);
    let (z1, w1, z2, w2) = (&v[7], &v[8], &v[9], &v[10]);
    let dq = q1.sub(q2);
    let dr = r1.sub(r2);
    let s2 = s1.sub(&dq.mul(h));
    let t2 = t1.sub(&dr.mul(h));
    let ds = s1.sub(&s2);
    let dt = t1.sub(&t2);
    let u1 = U::new(vec![r1.clone(), q1.clone(), P::c(1)]);
    let u2 = U::new(vec![r2.clone(), q2.clone(), P::c(1)]);
    let v1 = U::new(vec![t1.clone(), s1.clone()]);
    let v2 = U::new(vec![t2.clone(), s2.clone()]);
    let b = dq.mul(&q2.mul(r1).sub(&q1.mul(r2))).sub(&dr.sq());
    let a = dq.mul(&dt.mul(q2).sub(&ds.mul(r2))).sub(&dr.mul(&dt));
    let c = dq.mul(&dt).sub(&dr.mul(&ds));
    assert_eq!(c, P::c(0));
    assert_eq!(a, b.mul(h));

    let n = v1.scale(&b).sub(&u1.scale(&a));
    assert_eq!(rem_monic(n.sub(&v1.scale(&b)), &u1), U::new(vec![]));
    assert_eq!(rem_monic(n.sub(&v2.scale(&b)), &u2), U::new(vec![]));
    let xnum = q1.add(q2).mul(&b.sq()).add(&a.sq());
    let e = n
        .mul(&n)
        .add(&u1.mul(&u2).mul(&U::new(vec![xnum.neg(), b.sq()])));
    assert_eq!(e.coeff(6), P::c(0));
    assert_eq!(e.coeff(5), b.sq());
    assert_eq!(e.coeff(4), P::c(0));
    assert_eq!(
        rem_monic(e.sub(&v1.mul(&v1).scale(&b.sq())), &u1),
        U::new(vec![])
    );
    assert_eq!(
        rem_monic(e.sub(&v2.mul(&v2).scale(&b.sq())), &u2),
        U::new(vec![])
    );

    // The literal affine output y numerator equals -B^5 L(xnum/B^2),
    // where L=N/B=v1-h*u1. This is a polynomial equality without division.
    let ynum = a
        .mul(&q1.mul(&b.sq()).add(&xnum))
        .sub(&s1.mul(&b.pow(3)))
        .mul(&xnum)
        .add(&a.mul(r1).sub(&t1.mul(&b)).mul(&b.pow(4)));
    let y_expected = a
        .mul(
            &xnum
                .sq()
                .add(&q1.mul(&b.sq()).mul(&xnum))
                .add(&r1.mul(&b.pow(4))),
        )
        .sub(&s1.mul(&b.pow(3)).mul(&xnum))
        .sub(&t1.mul(&b.pow(5)));
    assert_eq!(ynum, y_expected);
    assert_eq!(
        ynum,
        n.coeff(2)
            .mul(&xnum.sq())
            .add(&n.coeff(1).mul(&xnum).mul(&b.sq()))
            .add(&n.coeff(0).mul(&b.pow(4)))
            .neg()
    );

    // Recompute literal weighted C=0 formulas. Z=z1*z2, W=w1*w2.
    let z = z1.mul(z2);
    let w = w1.mul(w2);
    let (q1c, q2c) = (q1.mul(&z.sq()), q2.mul(&z.sq()));
    let (r1c, r2c) = (r1.mul(&z.pow(4)), r2.mul(&z.pow(4)));
    let (s1c, s2c) = (s1.mul(&z.pow(3)).mul(&w), s2.mul(&z.pow(3)).mul(&w));
    let (t1c, t2c) = (t1.mul(&z.pow(5)).mul(&w), t2.mul(&z.pow(5)).mul(&w));
    let dqw = q1c.sub(&q2c);
    let drw = r1c.sub(&r2c);
    let dsw = s1c.sub(&s2c);
    let dtw = t1c.sub(&t2c);
    let bw0 = dqw.mul(&q2c.mul(&r1c).sub(&q1c.mul(&r2c))).sub(&drw.sq());
    let aw = dqw
        .mul(&dtw.mul(&q2c).sub(&dsw.mul(&r2c)))
        .sub(&drw.mul(&dtw));
    let cw = dqw.mul(&dtw).sub(&drw.mul(&dsw));
    let bww = bw0.mul(&w);
    let xw = q1c.add(&q2c).mul(&bww.sq()).add(&aw.sq());
    let yw = aw
        .mul(&q1c.mul(&bww.sq()).add(&xw))
        .sub(&s1c.mul(&bw0).mul(&bww.sq()))
        .mul(&xw)
        .add(&aw.mul(&r1c).sub(&t1c.mul(&bw0)).mul(&bww.pow(4)));
    let zw = bww.mul(&z);
    assert_eq!(cw, P::c(0));
    assert_eq!(bw0, z.pow(8).mul(&b));
    assert_eq!(aw, z.pow(9).mul(&w).mul(&a));
    assert_eq!(bww, z.pow(8).mul(&w).mul(&b));
    assert_eq!(xw, z.pow(18).mul(&w.sq()).mul(&xnum));
    assert_eq!(yw, z.pow(45).mul(&w.pow(5)).mul(&ynum));
    assert_eq!(zw, z.pow(9).mul(&w).mul(&b));
    println!(
        "{}",
        json!({"schema":"projective-symbolic-22-drop/v1","status":"pass",
            "coefficient_ring":"Z[q1,r1,s1,t1,q2,r2,h,z1,w1,z2,w2]",
            "affine_exact_equalities":11,"weighted_scale_equalities":7,
            "exact_integer_polynomial_equalities":18,
            "interpolant_terms":n.terms(),"reconstructed_curve_terms":e.terms(),
            "scope":"coprime B!=0 and C=0 weighted 2+2 degree-one branch with nonzero input weights; C=0 parametrized over a field by (ds,dt)=h(dq,dr); divisor-class bridge in PROOF_AUDIT.md"})
    );
}
