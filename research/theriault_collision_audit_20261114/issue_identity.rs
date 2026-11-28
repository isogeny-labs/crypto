//! Scalar supplement only. Run from repository root with its existing release dependencies.
//! The Lean proof is authoritative; this official checker does not replay Lean.
mod hash {
    #[path = "/workspace/crypto-audit/src/hash/sha256.rs"]
    pub mod sha256;
}
#[path = "/workspace/crypto-audit/src/cryptanalysis/identity_certificate.rs"]
mod identity;
use identity::{Expr as E, IdentityStatement, ProofRef, Verdict};
fn main() {
    let file = "research/theriault_collision_audit_20261114/lean/RecurrenceBounds.lean";
    let proof_hash = std::env::args().nth(1).expect("Lean source SHA256 argument");
    let n = E::var("n");
    let power = |x: E, k: usize| E::product(vec![x; k]);
    let statement = IdentityStatement {
        name: "Denominator-cleared three-draw source closed form".into(),
        source: "Theriault2003p86; TheriaultCollisionAudit.closed_form_cleared_identity. Scalar supplement only, not the finite occupancy proof or runtime theorem.".into(),
        variables: vec!["n".into()], degree_bound: 3,
        lhs: E::sum(vec![power(E::plus(n.clone(), E::Const(-2)), 3), E::times(E::Const(6), power(n.clone(),2)), E::minus(power(n.clone(),3))]),
        rhs: E::plus(E::times(E::Const(12), n), E::Const(-8)),
    };
    let cert = identity::issue(statement, 20261114, 32,
        "Producer-issued and same-host official checker,32extra points. No independent-host/signed standing; actual finite model rests on Lean.",
        Some(ProofRef{kind:"lean4".into(),path:file.into(),sha256:proof_hash})).expect("true identity");
    let verdict = identity::check_with_extra(&cert, Some((831987,32))).expect("well formed");
    assert!(matches!(verdict, Verdict::Accept{points:32,extra_points:32,..}));
    eprintln!("{verdict:?}");
    println!("{}", serde_json::to_string(&cert).unwrap());
}
