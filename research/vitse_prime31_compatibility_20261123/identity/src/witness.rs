//! Explicit expression-tree witness for MATHEMATICAL_REVIEW.md section 3.

use crate::identity_certificate::{Counterexample, Expr, IdentityStatement, Verdict};
use serde_json::{json, Value};

pub const ISSUE_SEED: u64 = 20261123;
pub const CHECK_SEED: u64 = 20261124;
pub const POINTS: u32 = 32;
pub const INDEPENDENCE: &str = "Same team and production implementation; separate issuer and checker processes; unsigned; no formal proof or independent-verifier claim.";

fn squared(x: Expr) -> Expr {
    Expr::times(x.clone(), x)
}

/// Direct transcription of the pinned production S3 normalization.
fn s3(x: Expr, y: Expr, t: Expr, a: Expr, b: Expr) -> Expr {
    let difference = Expr::plus(x.clone(), Expr::minus(y.clone()));
    let sum = Expr::plus(x.clone(), y.clone());
    let xy = Expr::times(x, y);
    Expr::sum(vec![
        Expr::times(squared(difference), squared(t.clone())),
        Expr::product(vec![
            Expr::Const(-2),
            Expr::plus(
                Expr::times(sum.clone(), Expr::plus(xy.clone(), a.clone())),
                Expr::times(Expr::Const(2), b.clone()),
            ),
            t,
        ]),
        squared(Expr::plus(xy, Expr::minus(a))),
        Expr::product(vec![Expr::Const(-4), b, sum]),
    ])
}

pub fn statement() -> IdentityStatement {
    let x = Expr::var("X");
    let y = Expr::var("Y");
    let t = Expr::var("t");
    let a = Expr::var("a");
    let b = Expr::var("b");
    let transformed = s3(
        Expr::minus(x.clone()),
        Expr::minus(y.clone()),
        t.clone(),
        a.clone(),
        b.clone(),
    );
    let original = s3(x.clone(), y.clone(), t.clone(), a.clone(), b.clone());
    let rhs = Expr::product(vec![
        Expr::Const(4),
        Expr::plus(x.clone(), y.clone()),
        Expr::plus(
            Expr::times(t, Expr::plus(Expr::times(x, y), a)),
            Expr::times(Expr::Const(2), b),
        ),
    ]);
    IdentityStatement {
        name: "Vitse Prime31 review: S3 simultaneous coordinate-negation residual".into(),
        source: "research/vitse_prime31_compatibility_20261123/MATHEMATICAL_REVIEW.md section 3; normalization src/cryptanalysis/pkm_smooth_tower.rs::s3 at 7d61e3b45de6e93d235345a0061dae08db1f8b44; pre-certificate review SHA256 30bbfda953e0ec1cecf044a96f849ef823f6e8831f963b4b79b717b884d5c159".into(),
        variables: ["X", "Y", "t", "a", "b"].map(String::from).to_vec(),
        degree_bound: 4,
        lhs: Expr::plus(transformed, Expr::minus(original)),
        rhs,
    }
}

pub fn wrong_sign_statement() -> IdentityStatement {
    let mut bad = statement();
    bad.name.push_str(" [wrong RHS-sign negative control]");
    bad.rhs = Expr::minus(bad.rhs);
    bad
}

pub fn counterexample_json(c: &Counterexample) -> Value {
    json!({"index": c.index, "variable_order": ["X", "Y", "t", "a", "b"],
           "assignment": c.assignment, "lhs": c.lhs, "rhs": c.rhs})
}

pub fn verdict_json(v: &Verdict) -> Value {
    match v {
        Verdict::Accept { points, extra_points, false_accept_log2 } => json!({
            "kind": "accept", "points": points, "extra_points": extra_points,
            "api_false_accept_log2": false_accept_log2,
            "interpretation": "Fixed deterministic evaluation record; no formal proof, independent-checker claim, or rigorous failure-probability claim."
        }),
        Verdict::Reject(c) => json!({"kind": "reject", "counterexample": counterexample_json(c)}),
        Verdict::DigestMismatch { recorded, recomputed } => json!({
            "kind": "digest_mismatch", "recorded": recorded, "recomputed": recomputed
        }),
    }
}
