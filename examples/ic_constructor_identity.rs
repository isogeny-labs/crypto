//! Seal the integer quartic-difference identity proved by ConstructorCore.lean.
use crypto_lib::cryptanalysis::identity_certificate::{
    self as ic, Expr, IdentityStatement, ProofRef, Verdict,
};
use sha2::{Digest, Sha256};
fn power(x: Expr, k: usize) -> Expr {
    Expr::product(vec![x; k])
}
fn main() {
    let (a, u, c1, c0) = (
        Expr::var("a"),
        Expr::var("u"),
        Expr::var("c1"),
        Expr::var("c0"),
    );
    let polynomial = |x: Expr| {
        Expr::sum(vec![
            power(x.clone(), 4),
            Expr::times(c1.clone(), x),
            c0.clone(),
        ])
    };
    let statement=IdentityStatement{name:"Integer quartic root-difference expansion".into(),
        source:"CONSTRUCTOR_EXCLUSION.md; faithful Lean theorem ICConstructor.quartic_difference_identity over every commutative ring. This integer identity is one dependency, not the entire finite-field irreducibility exclusion.".into(),
        variables:vec!["a".into(),"u".into(),"c1".into(),"c0".into()],degree_bound:4,
        lhs:Expr::plus(polynomial(Expr::plus(a.clone(),u.clone())),Expr::minus(polynomial(a.clone()))),
        rhs:Expr::sum(vec![power(u.clone(),4),Expr::times(c1,u.clone()),
            Expr::product(vec![Expr::Const(4),power(a.clone(),3),u.clone()]),
            Expr::product(vec![Expr::Const(6),power(a.clone(),2),power(u.clone(),2)]),
            Expr::product(vec![Expr::Const(4),a,power(u,3)])])};
    let path = "research/ic_algebraic_crossover_20261023/lean/ConstructorCore.lean";
    let cert=ic::issue(statement,202610250004,32,"Native certificate issuer; 256 extra domain-separated checks; faithful commutative-ring Lean proof built and axiom-audited separately. Fixed v1 evaluation modulus does not itself certify characteristic-two specialization.",Some(ProofRef{kind:"lean4-v4.34.1".into(),path:path.into(),sha256:hex::encode(Sha256::digest(std::fs::read(path).unwrap()))})).unwrap();
    assert!(matches!(
        ic::check_with_extra(&cert, Some((202610250005, 256))).unwrap(),
        Verdict::Accept { .. }
    ));
    println!("{}", serde_json::to_string_pretty(&cert).unwrap());
}
