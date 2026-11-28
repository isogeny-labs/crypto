#[path = "../../src/hash/sha256.rs"]
#[allow(dead_code)]
mod sha256_impl;

mod hash {
    pub mod sha256 {
        pub use crate::sha256_impl::sha256;
    }
}

#[path = "../../src/cryptanalysis/identity_certificate.rs"]
#[allow(dead_code)]
mod identity_certificate;

use identity_certificate::{
    check_with_extra, issue, Expr, IdentityStatement, ProofRef, Verdict,
};

fn power(base: Expr, exponent: usize) -> Expr {
    Expr::product(std::iter::repeat(base).take(exponent).collect())
}

fn main() {
    let scale = Expr::var("scale");
    let curve_a = Expr::var("A");
    let curve_b = Expr::var("B");
    let z = Expr::var("z");
    let q = Expr::var("Q");

    let target_form = Expr::sum(vec![
        Expr::Const(1),
        Expr::product(vec![
            power(scale.clone(), 4),
            curve_a.clone(),
            power(z.clone(), 4),
            power(q.clone(), 2),
        ]),
        Expr::product(vec![
            power(scale.clone(), 6),
            curve_b.clone(),
            power(z.clone(), 6),
            power(q.clone(), 3),
        ]),
    ]);
    let rescaled_source_form = Expr::sum(vec![
        Expr::Const(1),
        Expr::product(vec![
            curve_a,
            power(Expr::times(scale.clone(), z.clone()), 4),
            power(q.clone(), 2),
        ]),
        Expr::product(vec![
            curve_b,
            power(Expr::times(scale, z), 6),
            power(q, 3),
        ]),
    ]);

    let statement = IdentityStatement {
        name: "short-Weierstrass coordinate-unit target rescaling".to_string(),
        source: "H116FiberTrace.WeierstrassCoordinateUnit.rescale".to_string(),
        variables: ["scale", "A", "B", "z", "Q"]
            .into_iter()
            .map(str::to_string)
            .collect(),
        degree_bound: 16,
        lhs: target_form,
        rhs: rescaled_source_form,
    };
    let proof = ProofRef {
        kind: "lean4".to_string(),
        path: "research/h116_truncated_fiber_traces_20261118/lean/H116FiberTrace.lean"
            .to_string(),
        sha256: "8af7cc31f5ed3749a574d6b76b530108c0528d38fcb004034a82d561e75c14e6"
            .to_string(),
    };
    let certificate = issue(
        statement,
        116_20261118,
        32,
        "native issuer plus 32 domain-separated checker points; Lean proof pointer audited separately",
        Some(proof),
    )
    .expect("the coordinate rescaling identity must issue");
    let verdict = check_with_extra(&certificate, Some((4667, 32)))
        .expect("the coordinate rescaling certificate must be well formed");
    assert!(matches!(verdict, Verdict::Accept { extra_points: 32, .. }));
    println!("{}", serde_json::to_string_pretty(&certificate).unwrap());
}
