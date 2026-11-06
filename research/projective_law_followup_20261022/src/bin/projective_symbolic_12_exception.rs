//! Exact exceptional 1+2 cancellation and point-doubling identities.
//! Uses the canonical integer-polynomial engine of the versioned generic proof.
#[allow(dead_code)]
#[path = "projective_symbolic_12.rs"]
mod base;

use base::{P, U};
use serde_json::json;

fn main() {
    // x,y,q,r,s,t,a0,a1,a2,a3,alpha,mu,zeta,omega.
    let v: Vec<P> = (0..14).map(P::var).collect();
    let (x, y, q, r, s, t) = (&v[0], &v[1], &v[2], &v[3], &v[4], &v[5]);
    let (alpha, mu, zeta, omega) = (&v[10], &v[11], &v[12], &v[13]);
    let (az2, az3, az4, az5, az6, az8, az10) = (
        alpha.sq(),
        alpha.pow(3),
        alpha.pow(4),
        alpha.pow(5),
        alpha.pow(6),
        alpha.pow(8),
        alpha.pow(10),
    );
    let (zz2, zz3, zz4, zz5) = (zeta.sq(), zeta.pow(3), zeta.pow(4), zeta.pow(5));
    let px = x.mul(&az2);
    let py = y.mul(&az5).mul(mu);
    let (qw, rw, sw, tw) = (
        q.mul(&zz2),
        r.mul(&zz4),
        s.mul(&zz3).mul(omega),
        t.mul(&zz5).mul(omega),
    );
    let u = U::new(vec![r.clone(), q.clone(), P::c(1)]);
    let k = u.eval(x);
    let xz2 = px.mul(&zz2);
    let kw = qw
        .mul(&xz2)
        .mul(&az2)
        .add(&px.sq().mul(&zz4))
        .add(&rw.mul(&az4))
        .mul(mu);
    assert_eq!(kw, az4.mul(&zz4).mul(mu).mul(&k));

    // These are the literal k=0 branch formulas in add12_weighted_complete.
    let y3 = sw.mul(&az3).mul(&xz2).add(&tw.mul(&az5));
    let w3 = zz5.mul(omega);
    let m = az5.mul(&zz5).mul(mu).mul(omega);
    let vx = s.mul(x).add(t);
    assert_eq!(y3, az5.mul(&zz5).mul(omega).mul(&vx));
    assert_eq!(py.mul(&w3).add(&y3.mul(mu)), m.mul(&y.add(&vx)));
    let other_x = q.add(x).neg();
    let x4 = qw.mul(&az2).neg().sub(&xz2);
    let y4 = sw.mul(&az3).mul(&x4).add(&tw.mul(&az5));
    assert_eq!(x4, az2.mul(&zz2).mul(&other_x));
    assert_eq!(y4, az5.mul(&zz5).mul(omega).mul(&s.mul(&other_x).add(t)));
    assert_eq!(
        u.sub(
            &U::x()
                .sub(&U::c(x.clone()))
                .mul(&U::x().sub(&U::c(other_x.clone())))
        ),
        U::c(k.clone())
    );

    // The literal dbl(D1) coordinates. The affine normalization is
    // u=(X-x)^2 and v=y+f'(x)(X-x)/(2y) when y != 0.
    let fp = v[7]
        .add(&P::c(2).mul(&v[8]).mul(x))
        .add(&P::c(3).mul(&v[9]).mul(&x.sq()))
        .add(&P::c(5).mul(&x.pow(4)));
    let fx = v[6]
        .add(&v[7].mul(x))
        .add(&v[8].mul(&x.sq()))
        .add(&v[9].mul(&x.pow(3)))
        .add(&x.pow(5));
    let dbl_q = P::c(-2).mul(&px);
    let dbl_r = px.sq();
    let dbl_s = v[7]
        .mul(&az8)
        .add(&P::c(2).mul(&v[8]).mul(&px).mul(&az6))
        .add(&P::c(3).mul(&v[9]).mul(&px.sq()).mul(&az4))
        .add(&P::c(5).mul(&px.pow(4)))
        .mul(mu);
    let dbl_t = P::c(2)
        .mul(&v[6])
        .mul(&az10)
        .add(&v[7].mul(&px).mul(&az8))
        .sub(&v[9].mul(&px.pow(3)).mul(&az4))
        .sub(&P::c(3).mul(&px.pow(5)))
        .mul(mu);
    let dbl_w = P::c(2).mul(&py);
    assert_eq!(dbl_q, P::c(-2).mul(x).mul(&az2));
    assert_eq!(dbl_r, x.sq().mul(&az4));
    assert_eq!(dbl_s, az8.mul(mu).mul(&fp));
    assert_eq!(dbl_t, az10.mul(mu).mul(&P::c(2).mul(&fx).sub(&x.mul(&fp))));
    assert_eq!(dbl_w, P::c(2).mul(y).mul(&az5).mul(mu));
    assert_eq!(
        P::c(2)
            .mul(&fx)
            .sub(&x.mul(&fp))
            .sub(&P::c(2).mul(&y.sq()).sub(&x.mul(&fp))),
        P::c(2).mul(&fx.sub(&y.sq()))
    );
    println!(
        "{}",
        json!({"schema":"projective-symbolic-1plus2-exception/v1","status":"pass",
            "coefficient_ring":"Z[x,y,q,r,s,t,a0,a1,a2,a3,alpha,mu,zeta,omega]",
            "exact_integer_polynomial_equalities":12,
            "scope":"k=0 cancellation coordinates; weighted point doubling with y!=0; positive nontripling composition described in PROOF_AUDIT.md",
            "excluded":"tripling, generic and exceptional 2+2"})
    );
}
