//! Exact CRT/intersection certificate for the generic weighted 2+2 law.
//! Hypotheses and the divisor-class argument are in PROOF_AUDIT.md.
#[allow(dead_code)]
#[path = "projective_symbolic_12.rs"]
mod base;

use base::{P, U};
use serde_json::json;

fn div_rem_monic(mut rem: U, divisor: &U) -> (U, U) {
    assert_eq!(divisor.coeff(2), P::c(1));
    let mut quotient = vec![P::c(0); 5];
    for degree in (2..=6).rev() {
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
    // Independent q1,r1,s1,t1,q2,r2,s2,t2,z1,w1,z2,w2.
    let v: Vec<P> = (0..12).map(P::var).collect();
    let (q1, r1, s1, t1, q2, r2, s2, t2) = (&v[0], &v[1], &v[2], &v[3], &v[4], &v[5], &v[6], &v[7]);
    let (z1, w1, z2, w2) = (&v[8], &v[9], &v[10], &v[11]);
    let u1 = U::new(vec![r1.clone(), q1.clone(), P::c(1)]);
    let u2 = U::new(vec![r2.clone(), q2.clone(), P::c(1)]);
    let v1 = U::new(vec![t1.clone(), s1.clone()]);
    let v2 = U::new(vec![t2.clone(), s2.clone()]);
    let dq = q1.sub(q2);
    let dr = r1.sub(r2);
    let ds = s1.sub(s2);
    let dt = t1.sub(t2);
    let b = dq.mul(&q2.mul(r1).sub(&q1.mul(r2))).sub(&dr.sq());
    let a = dq.mul(&dt.mul(q2).sub(&ds.mul(r2))).sub(&dr.mul(&dt));
    let c = dq.mul(&dt).sub(&dr.mul(&ds));
    let n = v1
        .scale(&b)
        .sub(&u1.mul(&U::new(vec![a.clone(), c.clone()])));
    assert_eq!(div_rem_monic(n.sub(&v1.scale(&b)), &u1).1, U::new(vec![]));
    assert_eq!(div_rem_monic(n.sub(&v2.scale(&b)), &u2).1, U::new(vec![]));

    let ac = a.mul(&c);
    let d = dq.mul(&c.sq()).add(&ac);
    let q = d.add(&ac).sub(&b.sq());
    let rbar = a
        .mul(&dq.mul(&c).add(&a))
        .sub(&s1.add(s2).mul(&b).mul(&c))
        .add(&q1.add(q2).mul(&b.sq()));
    let r = rbar.mul(&c.sq());
    let qdelta = q1.mul(&c.sq()).sub(&q);
    let rdelta = r1.mul(&c.pow(4)).sub(&r);
    let s = rdelta
        .add(&ac.sub(&q).mul(&qdelta))
        .sub(&s1.mul(&b).mul(&c.pow(3)));
    let t = rdelta
        .mul(&ac)
        .sub(&r.mul(&qdelta))
        .sub(&t1.mul(&b).mul(&c.pow(5)));
    let output_u_scaled = U::new(vec![rbar.clone(), q.clone(), c.sq()]);

    // Reconstruct B^2 F from the input support data. Its X^5 and X^4
    // coefficients are exactly B^2 and zero; it interpolates both v_i^2.
    // For a valid monic quintic with no X^4 term, uniqueness modulo the
    // coprime quartic u1*u2 identifies this with B^2 F.
    let e = n.mul(&n).sub(&u1.mul(&u2).mul(&output_u_scaled));
    assert_eq!(e.coeff(6), P::c(0));
    assert_eq!(e.coeff(5), b.sq());
    assert_eq!(e.coeff(4), P::c(0));
    assert_eq!(
        div_rem_monic(e.sub(&v1.mul(&v1).scale(&b.sq())), &u1).1,
        U::new(vec![])
    );
    assert_eq!(
        div_rem_monic(e.sub(&v2.mul(&v2).scale(&b.sq())), &u2).1,
        U::new(vec![])
    );

    // The code's output v is -N/B modulo U/c^2, after clearing B*c^5.
    let n2 = c.mul(q1).add(&a).neg();
    assert_eq!(n.coeff(2), n2);
    let quotient = U::new(vec![c.pow(3).mul(&n2).add(&c.sq().mul(&q)), c.pow(4).neg()]);
    assert_eq!(
        n.scale(&c.pow(5))
            .add(&U::new(vec![t.clone(), s.mul(&c.sq())])),
        output_u_scaled.mul(&quotient)
    );

    // Literal weighted common-branch expressions, compared to predicted
    // homogeneous scales. Z=z1*z2 and W=w1*w2.
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
    let acw = aw.mul(&cw);
    let bww = bw0.mul(&w);
    let dw = dqw.mul(&cw.sq()).add(&acw);
    let qw = dw.add(&acw).sub(&bww.sq());
    let rw = acw
        .mul(&dw)
        .sub(&s1c.add(&s2c).mul(&bw0).mul(&cw.pow(3)))
        .add(&q1c.add(&q2c).mul(&bww.sq()).mul(&cw.sq()));
    let qdeltaw = q1c.mul(&cw.sq()).sub(&qw);
    let rdeltaw = r1c.mul(&cw.pow(4)).sub(&rw);
    let sw = rdeltaw
        .add(&acw.sub(&qw).mul(&qdeltaw))
        .sub(&s1c.mul(&bw0).mul(&cw.pow(3)));
    let tw = rdeltaw
        .mul(&acw)
        .sub(&rw.mul(&qdeltaw))
        .sub(&t1c.mul(&bw0).mul(&cw.pow(5)));
    let zout = z.mul(&cw);
    assert_eq!(bw0, z.pow(8).mul(&b));
    assert_eq!(aw, z.pow(9).mul(&w).mul(&a));
    assert_eq!(cw, z.pow(7).mul(&w).mul(&c));
    assert_eq!(bww, z.pow(8).mul(&w).mul(&b));
    assert_eq!(dw, z.pow(16).mul(&w.sq()).mul(&d));
    assert_eq!(qw, z.pow(16).mul(&w.sq()).mul(&q));
    assert_eq!(rw, z.pow(32).mul(&w.pow(4)).mul(&r));
    assert_eq!(sw, z.pow(32).mul(&w.pow(4)).mul(&s));
    assert_eq!(tw, z.pow(48).mul(&w.pow(6)).mul(&t));
    assert_eq!(zout, z.pow(8).mul(&w).mul(&c));
    println!(
        "{}",
        json!({"schema":"projective-symbolic-22-common/v1","status":"pass",
            "coefficient_ring":"Z[q1,r1,s1,t1,q2,r2,s2,t2,z1,w1,z2,w2]",
            "affine_exact_equalities":9,"weighted_scale_equalities":10,
            "exact_integer_polynomial_equalities":19,
            "interpolant_terms":n.terms(),"reconstructed_curve_terms":e.terms(),
            "scope":"generic weighted 2+2 with coprime u1,u2, B!=0, C!=0 and nonzero weights; divisor-class bridge in PROOF_AUDIT.md"})
    );
}
