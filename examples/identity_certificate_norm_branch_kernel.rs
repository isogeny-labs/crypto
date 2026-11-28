//! Emit the identity certificate for the two-by-two signed norm-branch
//! coefficient determinant factorization.

use crypto_lib::cryptanalysis::identity_certificate::{
    check, issue, Expr, IdentityStatement, ProofRef, Verdict,
};
use std::{fs::OpenOptions, io::Write};

fn var(name: &str) -> Expr {
    Expr::var(name)
}

fn sub(a: Expr, b: Expr) -> Expr {
    Expr::plus(a, Expr::minus(b))
}

fn product(xs: Vec<Expr>) -> Expr {
    Expr::product(xs)
}

fn difference(x: &str, y: &str) -> Expr {
    sub(var(x), var(y))
}

fn square(x: Expr) -> Expr {
    Expr::times(x.clone(), x)
}

fn statement() -> IdentityStatement {
    let c00 = product(vec![
        difference("c", "a"),
        difference("d", "b"),
        difference("e", "b"),
        difference("e", "d"),
    ]);
    let c01 = product(vec![
        difference("d", "a"),
        difference("c", "b"),
        difference("e", "b"),
        difference("e", "c"),
    ]);
    let c10 = product(vec![
        difference("c", "b"),
        difference("d", "a"),
        difference("e", "a"),
        difference("e", "d"),
    ]);
    let c11 = product(vec![
        difference("d", "b"),
        difference("c", "a"),
        difference("e", "a"),
        difference("e", "c"),
    ]);
    let node_factor = product(vec![
        difference("e", "a"),
        difference("e", "b"),
        difference("e", "c"),
        difference("e", "d"),
    ]);
    let kernel_det = sub(
        Expr::times(square(difference("c", "a")), square(difference("d", "b"))),
        Expr::times(square(difference("d", "a")), square(difference("c", "b"))),
    );
    IdentityStatement {
        name: "signed norm-branch 2x2 coefficient determinant factorization".into(),
        source: "research/prime_leads_chase_20261113/norm-branch-census/formal/NormBranchKernel.lean; direct Vandermonde partition algebra".into(),
        variables: ["a", "b", "c", "d", "e"].into_iter().map(str::to_string).collect(),
        degree_bound: 8,
        lhs: sub(Expr::times(c00, c11), Expr::times(c01, c10)),
        rhs: Expr::times(node_factor, kernel_det),
    }
}

fn main() {
    let cert = issue(
        statement(),
        0x4e42_4349_4443_0001,
        32,
        "issued by examples/identity_certificate_norm_branch_kernel.rs; checked by the repository identity-certificate verifier and separately formalized in Lean",
        Some(ProofRef {
            kind: "lean4".into(),
            path: "research/prime_leads_chase_20261113/norm-branch-census/formal/NormBranchKernel.lean".into(),
            sha256: "e04d56f09e92939de2efdb571251eef031e456200835f59b55ced38a0d5f3b11".into(),
        }),
    )
    .expect("the norm-branch determinant factorization holds");
    match check(&cert).expect("certificate is well formed") {
        Verdict::Accept { .. } => {}
        other => panic!("self-check failed: {other:?}"),
    }
    let mut bytes = serde_json::to_vec_pretty(&cert).expect("serializes");
    bytes.push(b'\n');
    let args: Vec<_> = std::env::args().collect();
    match args.as_slice() {
        [_] => print!("{}", String::from_utf8(bytes).expect("JSON is UTF-8")),
        [_, output] => OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(output)
            .and_then(|mut file| file.write_all(&bytes))
            .expect("writes certificate to a new file"),
        _ => panic!("usage: identity_certificate_norm_branch_kernel [OUTPUT_JSON]"),
    }
}
