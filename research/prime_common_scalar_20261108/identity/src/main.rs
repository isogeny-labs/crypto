//! Certificate for the polynomial rearrangement in affineFactorClearing.
//! The unit equation is proved in Lean; the certificate does not assume it.
#[path = "../../../../src/hash/sha256.rs"]
pub mod sha256_source;
pub mod hash {
    pub use crate::sha256_source as sha256;
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity;
use identity::{Expr as E, IdentityStatement, ProofRef};
fn main() {
    let source = "research/prime_common_scalar_20261108/lean/BooleanContraction.lean";
    let digest = sha256_source::sha256(&std::fs::read(source).expect("formal source"));
    let proof = ProofRef {
        kind: "lean4".into(),
        path: source.into(),
        sha256: digest.iter().map(|b| format!("{b:02x}")).collect(),
    };
    let d = E::var("d");
    let z = E::var("z");
    let i = E::var("i");
    let lhs = E::times(
        d.clone(),
        E::plus(
            E::Const(1),
            E::Neg(Box::new(E::times(z.clone(), i.clone()))),
        ),
    );
    let rhs = E::plus(
        d.clone(),
        E::Neg(Box::new(E::times(z.clone(), E::times(d, i)))),
    );
    let statement = IdentityStatement {
        name: "Affine zero-output clearing rearrangement".into(),
        source: format!("{source}:affineFactorClearing; unit equation is separate"),
        variables: vec!["d".into(), "z".into(), "i".into()],
        degree_bound: 3,
        lhs,
        rhs,
    };
    let cert=identity::issue(statement.clone(),0x20261109,32,
        "Unsigned producer issuance and replay using unchanged repository checker; no independent attestation",Some(proof)).expect("polynomial identity");
    let verdict = identity::check_with_extra(&cert, Some((0xdeadbeef, 32))).expect("well formed");
    assert!(matches!(verdict, identity::Verdict::Accept { .. }));
    let mut false_statement = statement;
    false_statement.rhs = E::plus(E::Const(1), E::Neg(Box::new(z)));
    assert!(matches!(
        identity::issue(false_statement, 0x20261109, 32, "negative control", None),
        Err(identity::IssueError::False(_))
    ));
    eprintln!("extra-point replay accepted; false naive numerator substitution rejected");
    println!("{}", serde_json::to_string_pretty(&cert).expect("JSON"));
}
