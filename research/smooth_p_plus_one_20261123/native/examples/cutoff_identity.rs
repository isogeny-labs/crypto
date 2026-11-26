//! Issue and replay the classical quartic pushforward identity with the
//! repository's unchanged native certificate and SHA-256 implementations.
#[path = "../../../../src/hash/sha256.rs"]
pub mod hash_sha256;
mod hash {
    pub use crate::hash_sha256 as sha256;
}
#[allow(dead_code)]
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod certificate;

use certificate::{Expr as E, IdentityCertificate, IdentityStatement, Verdict};

fn statement() -> IdentityStatement {
    let xs: Vec<_> = (0..4).map(|i| E::var(&format!("x{i}"))).collect();
    let z = E::plus(E::var("z"), E::Const(2));
    let elementary = |k: u32| {
        E::sum(
            (0u32..16)
                .filter(|bits| bits.count_ones() == k)
                .map(|bits| {
                    E::product(
                        xs.iter()
                            .enumerate()
                            .filter(|(i, _)| bits & (1 << i) != 0)
                            .map(|(_, x)| x.clone())
                            .collect(),
                    )
                })
                .collect(),
        )
    };
    let square = |x: E| E::times(x.clone(), x);
    let even = E::sum(vec![
        square(z.clone()),
        E::times(elementary(2), z.clone()),
        elementary(4),
    ]);
    let odd = E::minus(E::plus(E::times(elementary(1), z.clone()), elementary(3)));
    IdentityStatement {
        name: "Quartic root pushforward by x squared minus two, with multiplicities".into(),
        source: "Classical Graeffe identity Q(t)Q(-t); FULL_CUTOFF.md; no novelty asserted".into(),
        variables: ["x0", "x1", "x2", "x3", "z"].map(str::to_owned).to_vec(),
        degree_bound: 8,
        lhs: E::product(
            xs.iter()
                .map(|x| E::plus(z.clone(), E::minus(square(x.clone()))))
                .collect(),
        ),
        rhs: E::plus(square(even), E::minus(E::times(z, square(odd)))),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let cert = match args.as_slice() {
        [] => certificate::issue(
            statement(), 781815, 32,
            "Issued by this session using the existing native checker; unsigned, no independent runner or formal proof",
            None,
        )?,
        [path] => serde_json::from_slice::<IdentityCertificate>(&std::fs::read(path)?)?,
        _ => return Err("usage: cutoff_identity [certificate.json]".into()),
    };
    if cert.statement != statement() {
        return Err("certificate statement differs from the source-bound identity".into());
    }
    match certificate::check_with_extra(&cert, Some((20261125, 16)))? {
        Verdict::Accept { .. } => {}
        other => return Err(format!("certificate rejected: {other:?}").into()),
    }
    let mut false_statement = statement();
    false_statement.rhs = E::plus(false_statement.rhs, E::Const(1));
    if certificate::issue(false_statement, 781815, 32, "mutation", None).is_ok() {
        return Err("false identity mutation was accepted".into());
    }
    if args.is_empty() {
        println!("{}", serde_json::to_string_pretty(&cert)?);
    } else {
        println!(
            "{}",
            serde_json::json!({"type":"identity_replay","id":cert.id,
            "points":32,"extra_points":16,"false_identity_rejected":true,"signed":false})
        );
    }
    Ok(())
}
