//! Exact integer-polynomial certificate for the weighted tripling branch.
//! The divisor-class argument and hypotheses are in PROOF_AUDIT.md.
#[allow(dead_code)]
#[path = "projective_symbolic_12.rs"]
mod base;

use base::{P, U};
use serde_json::json;

fn main() {
    // Independent variables x,y,a0,a1,a2,a3,alpha,mu.
    let v: Vec<P> = (0..8).map(P::var).collect();
    let (x, y, a0, a1, a2, a3, alpha, mu) =
        (&v[0], &v[1], &v[2], &v[3], &v[4], &v[5], &v[6], &v[7]);
    let f = U::new(vec![
        a0.clone(),
        a1.clone(),
        a2.clone(),
        a3.clone(),
        P::c(0),
        P::c(1),
    ]);
    let fp = a1
        .add(&P::c(2).mul(a2).mul(x))
        .add(&P::c(3).mul(a3).mul(&x.sq()))
        .add(&P::c(5).mul(&x.pow(4)));
    let fpp = P::c(2).mul(
        &a2.add(&P::c(3).mul(a3).mul(x))
            .add(&P::c(10).mul(&x.pow(3))),
    );
    let k = P::c(8).mul(&y.pow(3));
    let aa = P::c(2).mul(&fpp).mul(&y.sq()).sub(&fp.sq());
    let bb = P::c(2).mul(&fp).mul(&y.sq()).sub(&aa.mul(x));
    let cc = y.mul(&k).sub(&aa.mul(x).add(&P::c(2).mul(&bb)).mul(x));
    let n = U::new(vec![cc.clone(), P::c(2).mul(&bb), aa.clone()]);
    let k2 = k.sq();
    let q = P::c(3).mul(x).mul(&k2).sub(&aa.sq());
    let r = a3
        .mul(&k2)
        .sub(&P::c(4).mul(&aa).mul(&bb))
        .add(&P::c(3).mul(x).mul(&q.sub(&x.mul(&k2))));
    let output_u_scaled = U::new(vec![r.clone(), q.clone(), k2.clone()]);
    let s = aa.mul(&q).sub(&P::c(2).mul(&bb).mul(&k2));
    let t = aa.mul(&r).mul(&k2).sub(&cc.mul(&k2.sq()));
    let output_v_cleared = U::new(vec![t.clone(), k2.mul(&s)]);
    let x_minus_x = U::x().sub(&U::c(x.clone()));
    let b = f.scale(&k2).sub(&n.mul(&n));

    // The coefficient equalities, including the threefold zero at x,
    // hold in the unconstrained integer polynomial ring. Only the final
    // constant remainder uses the curve equation y^2=F(x).
    assert_eq!(n.eval(x), y.mul(&k));
    assert_eq!(b.eval(x), k2.mul(&f.eval(x).sub(&y.sq())));
    assert_eq!(
        b,
        x_minus_x
            .pow(3)
            .mul(&output_u_scaled)
            .add(&U::c(k2.mul(&f.eval(x).sub(&y.sq()))))
    );
    assert_eq!(
        n.scale(&k2.sq())
            .sub(&output_u_scaled.scale(&aa.mul(&k2)))
            .add(&output_v_cleared),
        U::new(vec![])
    );
    // r3 in the literal code is k^2 times the quotient's constant term.
    assert_eq!(
        r.mul(&k2),
        a3.mul(&k.pow(4)).sub(
            &P::c(4)
                .mul(&aa)
                .mul(&bb)
                .sub(&P::c(3).mul(x).mul(&q.sub(&x.mul(&k2))))
                .mul(&k2)
        )
    );

    // Recompute the literal weighted code, then compare homogeneous scales.
    let (az2, az4, az5, az6, az8) = (
        alpha.sq(),
        alpha.pow(4),
        alpha.pow(5),
        alpha.pow(6),
        alpha.pow(8),
    );
    let px = x.mul(&az2);
    let py = y.mul(&az5).mul(mu);
    let fpw = a1
        .mul(&az8)
        .add(&P::c(2).mul(a2).mul(&px).mul(&az6))
        .add(&P::c(3).mul(a3).mul(&px.sq()).mul(&az4))
        .add(&P::c(5).mul(&px.pow(4)))
        .mul(mu);
    let fppw = P::c(2).mul(
        &a2.mul(&az6)
            .add(&P::c(3).mul(a3).mul(&px).mul(&az4))
            .add(&P::c(10).mul(&px.pow(3))),
    );
    let kw = P::c(8).mul(&py.pow(3));
    let aaw = P::c(2).mul(&fppw).mul(&py.sq()).sub(&fpw.sq()).mul(mu);
    let bbw = P::c(2).mul(&fpw).mul(&py.sq()).sub(&aaw.mul(&px));
    let ccw = py
        .mul(&kw)
        .sub(&aaw.mul(&px).add(&P::c(2).mul(&bbw)).mul(&px).mul(mu));
    let z3w = kw.mul(alpha);
    let qw = P::c(3).mul(&px).mul(&kw.sq()).sub(&aaw.sq());
    let rw = a3.mul(&z3w.pow(4)).sub(
        &P::c(4)
            .mul(&aaw)
            .mul(&bbw)
            .sub(&P::c(3).mul(&px).mul(&qw.sub(&px.mul(&kw.sq()))))
            .mul(&kw.sq()),
    );
    let sw = aaw
        .mul(&qw)
        .mul(mu)
        .sub(&P::c(2).mul(&bbw).mul(&kw.sq()).mul(mu));
    let tw = aaw.mul(&rw).mul(mu).sub(&ccw.mul(&kw.pow(4)));
    assert_eq!(fpw, az8.mul(mu).mul(&fp));
    assert_eq!(fppw, az6.mul(&fpp));
    assert_eq!(kw, alpha.pow(15).mul(&mu.pow(3)).mul(&k));
    assert_eq!(aaw, alpha.pow(16).mul(&mu.pow(3)).mul(&aa));
    assert_eq!(bbw, alpha.pow(18).mul(&mu.pow(3)).mul(&bb));
    assert_eq!(ccw, alpha.pow(20).mul(&mu.pow(4)).mul(&cc));
    assert_eq!(z3w, alpha.pow(16).mul(&mu.pow(3)).mul(&k));
    assert_eq!(qw, alpha.pow(32).mul(&mu.pow(6)).mul(&q));
    assert_eq!(rw, alpha.pow(64).mul(&mu.pow(12)).mul(&r.mul(&k2)));
    assert_eq!(sw, alpha.pow(48).mul(&mu.pow(10)).mul(&s));
    assert_eq!(tw, alpha.pow(80).mul(&mu.pow(16)).mul(&t));
    println!(
        "{}",
        json!({"schema":"projective-symbolic-triple/v1","status":"pass",
            "coefficient_ring":"Z[x,y,a0,a1,a2,a3,alpha,mu]",
            "affine_exact_equalities":5,"weighted_scale_equalities":11,
            "exact_integer_polynomial_equalities":16,
            "intersection_polynomial_terms":b.terms(),
            "output_u_scaled_terms":output_u_scaled.terms(),
            "scope":"tripling x4=x of a non-Weierstrass point on monic quintic; y!=0 and nonzero input weights; divisor-class bridge in PROOF_AUDIT.md"})
    );
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
