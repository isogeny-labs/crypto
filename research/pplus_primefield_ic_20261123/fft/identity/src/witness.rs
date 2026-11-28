//! These witnesses state elementary polynomial equalities, not the FFT theorem.
use crate::identity_certificate::{Counterexample, Expr, IdentityStatement, Verdict};
use serde_json::{json, Value};

pub const ISSUE_SEED: u64 = 20261123;
pub const CHECK_SEEDS: [u64; 2] = [20261124, 20261125];
pub const POINTS: u32 = 32;
pub const INDEPENDENCE: &str = "Same team and production implementation; separate issuer and checker processes; unsigned; deterministic finite evaluations only; no formal proof, independent-verifier, or PCC claim.";
pub const KEYS: [&str; 3] = ["butterfly_sum", "butterfly_difference", "cleared_trace_doubling"];

fn square(e: Expr) -> Expr { Expr::times(e.clone(), e) }
fn subtract(a: Expr, b: Expr) -> Expr { Expr::plus(a, Expr::minus(b)) }

pub fn statement(key: &str) -> IdentityStatement {
    let e = Expr::var("E");
    let o = Expr::var("O");
    let x = Expr::var("X");
    let xo = Expr::times(x, o);
    let positive = Expr::plus(e.clone(), xo.clone());
    let negative = subtract(e.clone(), xo.clone());
    let (name, variables, degree_bound, lhs, rhs) = match key {
        "butterfly_sum" => (
            "Line FFT: sum of butterfly outputs",
            vec!["E", "O", "X"], 2,
            Expr::plus(positive, negative),
            Expr::times(Expr::Const(2), e),
        ),
        "butterfly_difference" => (
            "Line FFT: difference of butterfly outputs",
            vec!["E", "O", "X"], 2,
            subtract(positive, negative),
            Expr::times(Expr::Const(2), xo),
        ),
        "cleared_trace_doubling" => {
            let z2 = square(Expr::var("Z"));
            (
                "Norm-one trace: denominator-cleared doubling identity",
                vec!["Z"], 4,
                subtract(square(Expr::plus(z2.clone(), Expr::Const(1))),
                         Expr::times(Expr::Const(2), z2.clone())),
                Expr::plus(square(z2), Expr::Const(1)),
            )
        },
        _ => panic!("unknown statement key: {key}"),
    };
    IdentityStatement {
        name: name.into(),
        source: "crypto PR #5049; fft/DERIVATION.md; identity/PROTOCOL.md. Elementary butterfly algebra and (Z^2+1)^2-2Z^2=Z^4+1. Certificate scope is only polynomial evaluations over F_(2^61-1).".into(),
        variables: variables.into_iter().map(String::from).collect(),
        degree_bound, lhs, rhs,
    }
}

pub fn false_statement(key: &str) -> IdentityStatement {
    let mut s = statement(key);
    s.name.push_str(" [RHS plus one negative control]");
    s.rhs = Expr::plus(s.rhs, Expr::Const(1));
    s
}

pub fn counterexample_json(c: &Counterexample, variables: &[String]) -> Value {
    json!({"index": c.index, "variable_order": variables,
           "assignment": c.assignment, "lhs": c.lhs, "rhs": c.rhs})
}

pub fn verdict_json(v: &Verdict, variables: &[String]) -> Value {
    match v {
        Verdict::Accept { points, extra_points, false_accept_log2 } => json!({
            "kind": "accept", "points": points, "extra_points": extra_points,
            "api_false_accept_log2": false_accept_log2,
            "interpretation": "Fixed deterministic evaluation receipt only; no rigorous failure-probability or formal-proof claim."
        }),
        Verdict::Reject(c) => json!({"kind": "reject", "counterexample": counterexample_json(c, variables)}),
        Verdict::DigestMismatch { recorded, recomputed } => json!({
            "kind": "digest_mismatch", "recorded": recorded, "recomputed": recomputed
        }),
    }
}
