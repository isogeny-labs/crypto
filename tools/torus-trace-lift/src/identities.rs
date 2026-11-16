//! Evaluated classical polynomial identities, not formal or signed proofs.
use torus_s4_tree::identity::{self, Expr as E, IdentityStatement};
fn sub(a: E, b: E) -> E {
    E::plus(a, E::minus(b))
}
fn mul(a: E, b: E) -> E {
    E::product(vec![a, b])
}
fn sq(a: E) -> E {
    mul(a.clone(), a)
}
fn scale(n: i64, a: E) -> E {
    mul(E::Const(n), a)
}

pub fn certificates() -> crate::Result<Vec<identity::IdentityCertificate>> {
    let x = E::var("x");
    let t = E::var("T");
    let u = E::var("U");
    let c = E::var("c");
    let d = E::var("d");
    let a = sub(sq(x.clone()), E::Const(1));
    let residual = sub(
        sub(sq(t.clone()), mul(a.clone(), sq(u.clone()))),
        E::Const(1),
    );
    let omega = sub(sub(sq(c.clone()), scale(2, sq(d.clone()))), E::Const(1));
    let trace = mul(sub(t.clone(), c.clone()), E::plus(t, c));
    let combined = E::plus(sub(trace, residual), omega);
    let t3 = sub(scale(4, mul(sq(x.clone()), x.clone())), scale(3, x.clone()));
    let u2 = sub(scale(4, sq(x)), E::Const(1));
    let source="Classical Chebyshev/Pell and cleared residual algebra; DLMF18.5.E1/E2; trace-lift-v1/DERIVATION.md. Native evaluated only.";
    let statements = vec![
        IdentityStatement {
            name: "Cubic Chebyshev Pell identity".into(),
            source: source.into(),
            variables: vec!["x".into()],
            degree_bound: 6,
            lhs: sub(sq(t3), mul(a.clone(), sq(u2))),
            rhs: E::Const(1),
        },
        IdentityStatement {
            name: "Imaginary lift cleared residual".into(),
            source: source.into(),
            variables: ["x", "T", "U", "c", "d"].map(str::to_string).to_vec(),
            degree_bound: 4,
            lhs: sub(mul(a.clone(), sq(u.clone())), scale(2, sq(d.clone()))),
            rhs: combined.clone(),
        },
        IdentityStatement {
            name: "Norm lift cleared residual".into(),
            source: source.into(),
            variables: ["x", "T", "U", "c", "d"].map(str::to_string).to_vec(),
            degree_bound: 6,
            lhs: sub(
                mul(sq(scale(2, d)), a.clone()),
                scale(2, sq(mul(a.clone(), u))),
            ),
            rhs: scale(-2, mul(a, combined)),
        },
    ];
    statements
        .into_iter()
        .enumerate()
        .map(|(i, s)| {
            let cert = identity::issue(
                s.clone(),
                202611160835 + i as u64,
                32,
                "Producer native evaluated identity, unsigned",
                None,
            )
            .map_err(|e| e.to_string())?;
            if !matches!(
                identity::check_with_extra(&cert, Some((202611160841 + i as u64, 32)))
                    .map_err(|e| e.to_string())?,
                identity::Verdict::Accept { .. }
            ) {
                return Err("extra identity stream rejected".into());
            }
            let mut bad = s;
            bad.rhs = E::plus(bad.rhs, E::Const(1));
            if identity::issue(bad, 202611160835 + i as u64, 32, "plus-one mutation", None).is_ok()
            {
                return Err("identity mutation accepted".into());
            }
            Ok(cert)
        })
        .collect()
}
