//! Exact Hermite-interpolation identity for the same-u equal-v doubling law.
#[allow(dead_code)]
#[path = "projective_symbolic_12.rs"]
mod base;
use base::{P, U};
use serde_json::json;

fn div_rem_monic(mut rem: U, divisor: &U, high: usize) -> (U, U) {
    assert_eq!(divisor.coeff(2), P::c(1));
    let mut quotient = vec![P::c(0); high - 1];
    for degree in (2..=high).rev() {
        let lead = rem.coeff(degree);
        quotient[degree - 2] = lead.clone();
        rem = rem.sub(&U::x().pow(degree - 2).mul(&divisor.scale(&lead)));
    }
    (U::new(quotient), rem)
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
    // q,r,s,t,a0,a1,a2,a3,z,w.
    let v: Vec<P> = (0..10).map(P::var).collect();
    let (q, r, s, t, a0, a1, a2, a3) = (&v[0], &v[1], &v[2], &v[3], &v[4], &v[5], &v[6], &v[7]);
    let u = U::new(vec![r.clone(), q.clone(), P::c(1)]);
    let vv = U::new(vec![t.clone(), s.clone()]);
    let f = U::new(vec![
        a0.clone(),
        a1.clone(),
        a2.clone(),
        a3.clone(),
        P::c(0),
        P::c(1),
    ]);
    let (h, input_rem) = div_rem_monic(f.sub(&vv.mul(&vv)), &u, 5);
    let n1 = q.mul(s).sub(t);
    let n2 = r.mul(s);
    let b = P::c(2).mul(&n1.mul(t).sub(&n2.mul(s)));
    let n5 = s
        .sq()
        .sub(&q.mul(&P::c(4).mul(r).sub(&q.sq())).sub(&q.mul(a3).sub(a2)));
    let n6 = P::c(2).mul(r).sub(&P::c(3).mul(&q.sq())).sub(a3);
    let a = n1.mul(&n5).sub(&n2.mul(&n6));
    let c = s.mul(&n5).sub(&t.mul(&n6));
    let ell = U::new(vec![a.clone(), c.clone()]);
    let n = vv.scale(&b).sub(&u.mul(&ell));
    let (j, hermite_rem) = div_rem_monic(h.scale(&b).add(&vv.mul(&ell).scale(&P::c(2))), &u, 3);
    assert_eq!(hermite_rem, U::new(vec![]));
    let q3 = P::c(2).mul(&a).mul(&c).sub(&b.sq());
    let rbar = a
        .sq()
        .sub(&P::c(2).mul(s).mul(&b).mul(&c))
        .add(&P::c(2).mul(q).mul(&b.sq()));
    let uout = U::new(vec![rbar.clone(), q3.clone(), c.sq()]);
    assert_eq!(j.scale(&b).sub(&ell.mul(&ell)), uout.scale(&P::c(-1)));
    assert_eq!(
        f.scale(&b.sq()).sub(&n.mul(&n)),
        u.mul(&u)
            .mul(&uout)
            .scale(&P::c(-1))
            .add(&input_rem.scale(&b.sq()))
    );
    let r3 = rbar.mul(&c.sq());
    let qdelta = q.mul(&c.sq()).sub(&q3);
    let rdelta = r.mul(&c.pow(4)).sub(&r3);
    let s3 = rdelta
        .sub(&a.mul(&c).sub(&b.sq()).mul(&qdelta))
        .sub(&s.mul(&c.pow(3)).mul(&b));
    let t3 = rdelta
        .mul(&a.mul(&c))
        .sub(&r3.mul(&qdelta))
        .sub(&t.mul(&c.pow(5)).mul(&b));
    let v_clear = n
        .scale(&c.pow(5))
        .add(&U::new(vec![t3.clone(), s3.mul(&c.sq())]));
    let vquot = U::new(vec![
        c.sq().mul(&q3.sub(&c.sq().mul(q))).sub(&a.mul(&c.pow(3))),
        c.pow(4).neg(),
    ]);
    assert_eq!(v_clear, uout.mul(&vquot));
    let (z, w) = (&v[8], &v[9]);
    let (qw, rw, sw, tw) = (
        q.mul(&z.sq()),
        r.mul(&z.pow(4)),
        s.mul(&z.pow(3)).mul(w),
        t.mul(&z.pow(5)).mul(w),
    );
    let n1w = qw.mul(&sw).sub(&tw);
    let n2w = rw.mul(&sw);
    let bw0 = P::c(2).mul(&n1w.mul(&tw).sub(&n2w.mul(&sw)));
    let n5w = sw.sq().sub(
        &qw.mul(&P::c(4).mul(&rw).sub(&qw.sq()))
            .sub(&qw.mul(a3).mul(&z.pow(4)).sub(&a2.mul(&z.pow(6))))
            .mul(&w.sq()),
    );
    let n6w = P::c(2)
        .mul(&rw)
        .sub(&P::c(3).mul(&qw.sq()))
        .sub(&a3.mul(&z.pow(4)))
        .mul(&w.sq());
    let aw = n1w.mul(&n5w).sub(&n2w.mul(&n6w));
    let cw = sw.mul(&n5w).sub(&tw.mul(&n6w));
    let bww = bw0.mul(w);
    let acw = aw.mul(&cw);
    let q3w = P::c(2).mul(&acw).sub(&bww.sq());
    let r3w = acw
        .sq()
        .sub(&P::c(2).mul(&sw).mul(&cw.pow(3)).mul(&bw0))
        .add(&P::c(2).mul(&qw).mul(&cw.sq()).mul(&bww.sq()));
    let qdw = qw.mul(&cw.sq()).sub(&q3w);
    let rdw = rw.mul(&cw.pow(4)).sub(&r3w);
    let s3w = rdw
        .sub(&acw.sub(&bww.sq()).mul(&qdw))
        .sub(&sw.mul(&cw.pow(3)).mul(&bw0));
    let t3w = rdw
        .mul(&acw)
        .sub(&r3w.mul(&qdw))
        .sub(&tw.mul(&cw.pow(5)).mul(&bw0));
    let zout = z.mul(&cw);
    assert_eq!(n1w, z.pow(5).mul(w).mul(&n1));
    assert_eq!(n2w, z.pow(7).mul(w).mul(&n2));
    assert_eq!(bw0, z.pow(10).mul(&w.sq()).mul(&b));
    assert_eq!(n5w, z.pow(6).mul(&w.sq()).mul(&n5));
    assert_eq!(n6w, z.pow(4).mul(&w.sq()).mul(&n6));
    assert_eq!(aw, z.pow(11).mul(&w.pow(3)).mul(&a));
    assert_eq!(cw, z.pow(9).mul(&w.pow(3)).mul(&c));
    assert_eq!(q3w, z.pow(20).mul(&w.pow(6)).mul(&q3));
    assert_eq!(r3w, z.pow(40).mul(&w.pow(12)).mul(&r3));
    assert_eq!(s3w, z.pow(40).mul(&w.pow(12)).mul(&s3));
    assert_eq!(t3w, z.pow(60).mul(&w.pow(18)).mul(&t3));
    assert_eq!(zout, z.pow(10).mul(&w.pow(3)).mul(&c));
    assert_eq!(bww, z.pow(10).mul(&w.pow(3)).mul(&b));
    // The same polynomial intersection identity specializes at C=0 to
    // degree one. Check the literal point-output numerator and scales.
    let xnum = P::c(2).mul(q).mul(&b.sq()).add(&a.sq());
    assert_eq!(q3.add(&b.sq()), P::c(2).mul(&a).mul(&c));
    assert_eq!(rbar.sub(&xnum), P::c(-2).mul(s).mul(&b).mul(&c));
    let n0 = vv.scale(&b).sub(&u.scale(&a));
    let ynum = a
        .mul(&q.mul(&b.sq()).add(&xnum))
        .sub(&b.mul(s).mul(&b.sq()))
        .mul(&xnum)
        .add(&a.mul(r).sub(&b.mul(t)).mul(&b.pow(4)));
    assert_eq!(
        ynum,
        n0.coeff(2)
            .mul(&xnum.sq())
            .add(&n0.coeff(1).mul(&xnum).mul(&b.sq()))
            .add(&n0.coeff(0).mul(&b.pow(4)))
            .neg()
    );
    let xdropw = P::c(2).mul(&qw).mul(&bww.sq()).add(&aw.sq());
    let ydropw = aw
        .mul(&qw.mul(&bww.sq()).add(&xdropw))
        .sub(&bw0.mul(&sw).mul(&bww.sq()))
        .mul(&xdropw)
        .add(&aw.mul(&rw).sub(&bw0.mul(&tw)).mul(&bww.pow(4)));
    let zdropw = z.mul(&bww);
    assert_eq!(xdropw, z.pow(22).mul(&w.pow(6)).mul(&xnum));
    assert_eq!(ydropw, z.pow(55).mul(&w.pow(15)).mul(&ynum));
    assert_eq!(zdropw, z.pow(11).mul(&w.pow(3)).mul(&b));
    // When B=0, one support point is Weierstrass. If s!=0, the code
    // extracts the other point and calls the already certified dbl law.
    let resultant = r.mul(&s.sq()).sub(&q.mul(s).mul(t)).add(&t.sq());
    assert_eq!(b, P::c(-2).mul(&resultant));
    let xpoint = n1.mul(s).neg();
    let ypoint = s.mul(t).add(&xpoint).mul(&s.pow(4));
    assert_eq!(xpoint.add(&q.mul(&s.sq())).sub(&s.mul(t)), P::c(0));
    assert_eq!(ypoint, s.pow(5).mul(&P::c(2).mul(t).sub(&q.mul(s))));
    let xpointw = n1w.mul(&sw).neg();
    let ypointw = sw.mul(&tw).add(&xpointw).mul(&sw.pow(4));
    let zpointw = sw.mul(z);
    assert_eq!(xpointw, z.pow(8).mul(&w.sq()).mul(&xpoint));
    assert_eq!(ypointw, z.pow(20).mul(&w.pow(6)).mul(&ypoint));
    assert_eq!(zpointw, z.pow(4).mul(w).mul(s));
    println!(
        "{}",
        json!({"schema":"projective-symbolic-22-double/v1","status":"pass","coefficient_ring":"Z[q,r,s,t,a0,a1,a2,a3,z,w]","affine_exact_equalities":11,"weighted_scale_equalities":19,"exact_integer_polynomial_equalities":30,"hermite_terms":j.terms(),"intersection_terms":uout.terms(),"v_clear_terms":uout.mul(&vquot).terms(),"scope":"same-u equal-v weighted 2+2 doubling: B!=0 generic and C=0 drop; B=0 identity or extracted-point doubling; divisor-class bridge in PROOF_AUDIT.md"})
    );
}
