//! Exact weighted support extraction and point-point addition identities.
//! Composition with certified 1+2 and doubling laws is in PROOF_AUDIT.md.
#[allow(dead_code)]
#[path = "projective_symbolic_12.rs"]
mod base;
use base::P;
use serde_json::json;

fn main() {
    // Shared-support branch: q1,r1,s1,t1,q2,r2,s2,t2,z1,w1,z2,w2.
    let v: Vec<P> = (0..12).map(P::var).collect();
    let (q1, r1, s1, t1, q2, r2, s2, t2) = (&v[0], &v[1], &v[2], &v[3], &v[4], &v[5], &v[6], &v[7]);
    let (z1, w1, z2, w2) = (&v[8], &v[9], &v[10], &v[11]);
    let z = z1.mul(z2);
    let w = w1.mul(w2);
    let dq = q1.sub(q2);
    let dr = r1.sub(r2);
    let b = dq.mul(&q2.mul(r1).sub(&q1.mul(r2))).sub(&dr.sq());
    let x0num = dq.mul(&dr).neg();
    let x2num = q1.mul(&dq.sq()).neg().sub(&x0num);
    let x4num = q2.mul(&dq.sq()).neg().sub(&x0num);
    assert_eq!(
        x0num
            .sq()
            .add(&q1.mul(&x0num).mul(&dq.sq()))
            .add(&r1.mul(&dq.pow(4))),
        b.neg().mul(&dq.sq())
    );
    assert_eq!(
        x0num
            .sq()
            .add(&q2.mul(&x0num).mul(&dq.sq()))
            .add(&r2.mul(&dq.pow(4))),
        b.neg().mul(&dq.sq())
    );
    assert_eq!(x0num.add(&x2num), q1.mul(&dq.sq()).neg());
    assert_eq!(x0num.add(&x4num), q2.mul(&dq.sq()).neg());
    let q1c = q1.mul(&z.sq());
    let q2c = q2.mul(&z.sq());
    let r1c = r1.mul(&z.pow(4));
    let r2c = r2.mul(&z.pow(4));
    let kw = q1c.sub(&q2c);
    let drw = r1c.sub(&r2c);
    let bw = kw.mul(&q2c.mul(&r1c).sub(&q1c.mul(&r2c))).sub(&drw.sq());
    let zw = z.mul(&kw);
    let x0w = kw.mul(&drw).neg();
    let x2w = q1.mul(&z1.sq()).mul(&kw.sq()).mul(&z2.sq()).neg().sub(&x0w);
    let x4w = q2.mul(&z2.sq()).mul(&kw.sq()).mul(&z1.sq()).neg().sub(&x0w);
    let point_y = |xx: &P, ss: &P, tt: &P, zz2: &P, zz3: &P| {
        ss.mul(xx)
            .add(&tt.mul(&kw.sq()).mul(zz2))
            .mul(&kw.pow(3))
            .mul(zz3)
    };
    let y0_1w = point_y(
        &x0w,
        &s1.mul(&z1.pow(3)).mul(w1),
        &t1.mul(&z1.pow(5)).mul(w1),
        &z2.sq(),
        &z2.pow(3),
    );
    let y2w = point_y(
        &x2w,
        &s1.mul(&z1.pow(3)).mul(w1),
        &t1.mul(&z1.pow(5)).mul(w1),
        &z2.sq(),
        &z2.pow(3),
    );
    let y0_2w = point_y(
        &x0w,
        &s2.mul(&z2.pow(3)).mul(w2),
        &t2.mul(&z2.pow(5)).mul(w2),
        &z1.sq(),
        &z1.pow(3),
    );
    let y4w = point_y(
        &x4w,
        &s2.mul(&z2.pow(3)).mul(w2),
        &t2.mul(&z2.pow(5)).mul(w2),
        &z1.sq(),
        &z1.pow(3),
    );
    let yn = |xx: &P, ss: &P, tt: &P| ss.mul(xx).add(&tt.mul(&dq.sq())).mul(&dq.pow(3));
    assert_eq!(bw, z.pow(8).mul(&b));
    assert_eq!(kw, z.sq().mul(&dq));
    assert_eq!(zw, z.pow(3).mul(&dq));
    assert_eq!(x0w, z.pow(6).mul(&x0num));
    assert_eq!(x2w, z.pow(6).mul(&x2num));
    assert_eq!(x4w, z.pow(6).mul(&x4num));
    assert_eq!(y0_1w, z.pow(15).mul(w1).mul(&yn(&x0num, s1, t1)));
    assert_eq!(y2w, z.pow(15).mul(w1).mul(&yn(&x2num, s1, t1)));
    assert_eq!(y0_2w, z.pow(15).mul(w2).mul(&yn(&x0num, s2, t2)));
    assert_eq!(y4w, z.pow(15).mul(w2).mul(&yn(&x4num, s2, t2)));
    assert_eq!(
        y0_1w.mul(w2).sub(&y0_2w.mul(w1)),
        z.pow(15)
            .mul(&w)
            .mul(&yn(&x0num, s1, t1).sub(&yn(&x0num, s2, t2)))
    );

    // Literal add11 for two distinct weighted points, using fresh
    // independent variables in the same integer-polynomial engine.
    let (x1, y1, xx2, y2, alpha, mu, beta, nu) =
        (&v[0], &v[1], &v[2], &v[3], &v[4], &v[5], &v[6], &v[7]);
    let ab = alpha.mul(beta);
    let mn = mu.mul(nu);
    let ax = x1.mul(&alpha.sq());
    let bx = xx2.mul(&beta.sq());
    let ay = y1.mul(&alpha.pow(5)).mul(mu);
    let by = y2.mul(&beta.pow(5)).mul(nu);
    let axbz = ax.mul(&beta.sq());
    let bxaz = bx.mul(&alpha.sq());
    let ayc = ay.mul(&beta.pow(5)).mul(nu);
    let byc = by.mul(&alpha.pow(5)).mul(mu);
    let qout = axbz.add(&bxaz).neg();
    let rout = axbz.mul(&bxaz);
    let sout = ayc.sub(&byc);
    let tout = axbz.mul(&byc).sub(&bxaz.mul(&ayc));
    let zout = ab.clone();
    let wout = axbz.sub(&bxaz).mul(&mn);
    assert_eq!(axbz, ab.sq().mul(x1));
    assert_eq!(bxaz, ab.sq().mul(xx2));
    assert_eq!(ayc, ab.pow(5).mul(&mn).mul(y1));
    assert_eq!(byc, ab.pow(5).mul(&mn).mul(y2));
    assert_eq!(qout, ab.sq().mul(&x1.add(xx2).neg()));
    assert_eq!(rout, ab.pow(4).mul(&x1.mul(xx2)));
    assert_eq!(sout, ab.pow(5).mul(&mn).mul(&y1.sub(y2)));
    assert_eq!(tout, ab.pow(7).mul(&mn).mul(&x1.mul(y2).sub(&xx2.mul(y1))));
    assert_eq!(zout, alpha.mul(beta));
    assert_eq!(wout, ab.sq().mul(&mn).mul(&x1.sub(xx2)));
    let line_s = y1.sub(y2);
    let line_t = x1.mul(y2).sub(&xx2.mul(y1));
    assert_eq!(line_s.mul(x1).add(&line_t), x1.sub(xx2).mul(y1));
    assert_eq!(line_s.mul(xx2).add(&line_t), x1.sub(xx2).mul(y2));
    println!(
        "{}",
        json!({"schema":"projective-symbolic-22-shared/v1","status":"pass",
        "coefficient_ring":"Z[q1,r1,s1,t1,q2,r2,s2,t2,z1,w1,z2,w2] and Z[x1,y1,x2,y2,alpha,mu,beta,nu]",
        "shared_exact_equalities":15,"point_add_exact_equalities":12,
        "exact_integer_polynomial_equalities":27,
        "scope":"B=0 distinct-u shared-support extraction, comparison, and add11 point addition; compositional divisor argument in PROOF_AUDIT.md"})
    );
}
