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
    let file = "research/subgroup_extraction_audit_20261114/lean/PrimeRecovery.lean";
    let proof_hash = std::env::args().nth(1).expect("Lean source SHA256 argument");
    let a = E::var("a"); let b = E::var("b"); let x = E::var("x");
    let statement = IdentityStatement {
        name: "Scalar commutative rearrangement in subgroup extraction".into(),
        source: "SubgroupExtractionAudit.recovery_ring_identity; scalar supplement only, not the group-order, nonunit or curve proof.".into(),
        variables: vec!["a".into(),"b".into(),"x".into()], degree_bound: 2,
        lhs: E::plus(E::times(x.clone(),b.clone()),a.clone()),
        rhs: E::plus(a,E::times(b,x)),
    };
    let cert = identity::issue(statement, 20261114, 32,
        "Producer-issued and same-host official checker,32extra points. No independent-host/signed standing; actual group and curve model rests on Lean.",
        Some(ProofRef{kind:"lean4".into(),path:file.into(),sha256:proof_hash})).expect("true identity");
    let verdict = identity::check_with_extra(&cert, Some((831987,32))).expect("well formed");
    assert!(matches!(verdict, Verdict::Accept{points:32,extra_points:32,..}));
    eprintln!("{verdict:?}");
    println!("{}", serde_json::to_string(&cert).unwrap());
}
