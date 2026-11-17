//! Exact weighted point-extraction identities for same-u mixed 2+2 inputs.
//! The valid-divisor and cancellation argument is in PROOF_AUDIT.md.
#[allow(dead_code)]
#[path = "projective_symbolic_12.rs"]
mod base;

use base::P;
use serde_json::json;

fn main() {
    // Independent q,r,s1,t1,s2,t2,z1,w1,z2,w2.
    let v: Vec<P> = (0..10).map(P::var).collect();
    let (q, r, s1, t1, s2, t2, z1, w1, z2, w2) = (
        &v[0], &v[1], &v[2], &v[3], &v[4], &v[5], &v[6], &v[7], &v[8], &v[9],
    );
    let z = z1.mul(z2);
    let w = w1.mul(w2);
    let k = s1.sub(s2);
    let xnum = t2.sub(t1).mul(&k);
    let ynum = s1.mul(&xnum).add(&t1.mul(&k.sq())).mul(&k.pow(3));
    let q1c = q.mul(&z1.sq()).mul(&z2.sq());
    let q2c = q.mul(&z2.sq()).mul(&z1.sq());
    let r1c = r.mul(&z1.pow(4)).mul(&z2.pow(4));
    let r2c = r.mul(&z2.pow(4)).mul(&z1.pow(4));
    let s1w = s1.mul(&z1.pow(3)).mul(w1);
    let t1w = t1.mul(&z1.pow(5)).mul(w1);
    let s1c = s1w.mul(&z2.pow(3)).mul(w2);
    let s2c = s2.mul(&z2.pow(3)).mul(w2).mul(&z1.pow(3)).mul(w1);
    let t1c = t1w.mul(&z2.pow(5)).mul(w2);
    let t2c = t2.mul(&z2.pow(5)).mul(w2).mul(&z1.pow(5)).mul(w1);
    let kw = s1c.sub(&s2c);
    let xw = t2c.sub(&t1c).mul(&kw);
    let yw = s1w
        .mul(&xw)
        .add(&t1w.mul(&z2.sq()).mul(&kw.sq()))
        .mul(&kw.pow(3))
        .mul(&z2.pow(3));
    let zw = kw.mul(&z);
    assert_eq!(q1c, q2c);
    assert_eq!(r1c, r2c);
    assert_eq!(kw, z.pow(3).mul(&w).mul(&k));
    assert_eq!(xw, z.pow(8).mul(&w.sq()).mul(&xnum));
    assert_eq!(yw, z.pow(20).mul(&w.pow(5)).mul(w1).mul(&ynum));
    assert_eq!(zw, z.pow(4).mul(&w).mul(&k));
    // xnum/k² is the root of v1-v2; ynum/k⁵=v1(xnum/k²).
    assert_eq!(k.mul(&xnum).add(&t1.sub(t2).mul(&k.sq())), P::c(0));
    println!(
        "{}",
        json!({"schema":"projective-symbolic-22-mixed/v1","status":"pass",
            "coefficient_ring":"Z[q,r,s1,t1,s2,t2,z1,w1,z2,w2]",
            "exact_integer_polynomial_equalities":7,
            "scope":"same-u mixed branch weighted extraction for k=s1-s2 != 0, then previously certified dbl(P); valid-divisor cancellation bridge in PROOF_AUDIT.md"})
    );
}
