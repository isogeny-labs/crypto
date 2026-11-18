//! Native identity certificate for algebraic simplification only. It does not
//! certify independence, moment counting, probability or the proposed energy test.
use torus_s4_tree::identity::{self, Expr as E, IdentityStatement};
pub fn statement() -> IdentityStatement {
    let n = E::var("n");
    let q = E::var("q");
    let t = E::var("t");
    let sub = |a: E, b: E| E::plus(a, E::Neg(Box::new(b)));
    let mul = |v: Vec<E>| E::Mul(v);
    let nm1 = sub(n.clone(), E::Const(1));
    let nm2 = sub(n.clone(), E::Const(2));
    let nm3 = sub(n.clone(), E::Const(3));
    let q2 = E::times(q.clone(), q.clone());
    let lhs = E::Add(vec![
        mul(vec![n.clone(), nm1.clone(), nm2.clone(), nm3, q2.clone()]),
        mul(vec![
            E::Const(4),
            n.clone(),
            nm1.clone(),
            nm2.clone(),
            t.clone(),
        ]),
        mul(vec![E::Const(2), n.clone(), nm1.clone(), q.clone()]),
        E::Neg(Box::new(mul(vec![
            n.clone(),
            n.clone(),
            nm1.clone(),
            nm1.clone(),
            q2.clone(),
        ]))),
    ]);
    let rhs = E::Add(vec![
        mul(vec![
            E::Const(2),
            n.clone(),
            nm1.clone(),
            sub(q, q2.clone()),
        ]),
        mul(vec![E::Const(4), n, nm1, nm2, sub(t, q2)]),
    ]);
    IdentityStatement{name:"Collision-count variance, cleared combinatorial coefficients".into(),source:"DGPP arXiv:1611.03579 section 2.1 Lemmas 2-3. Certificate checks only the polynomial simplification of 4(E[Z^2]-E[Z]^2), not the iid moment interpretation or any probability theorem.".into(),variables:vec!["n".into(),"q".into(),"t".into()],degree_bound:6,lhs,rhs}
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: moment_identity NEW_CERTIFICATE.json".into());
    }
    let cert = identity::issue(
        statement(),
        202611150121,
        32,
        "Producer polynomial identity only; formal probability interpretation remains open",
        None,
    )?;
    if !matches!(
        identity::check_with_extra(&cert, Some((202611150127, 32)))?,
        identity::Verdict::Accept { .. }
    ) {
        return Err("identity check failed".into());
    }
    use std::io::Write;
    let mut out = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[1])?;
    out.write_all(&serde_json::to_vec_pretty(&cert)?)?;
    Ok(())
}
