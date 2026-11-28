//! RR5 polynomial statements for the unchanged native identity checker.
//!
//! Frozen qualification metadata, written before any invocation:
//! - test: `rr5_sampled_identities_accept_original_checker`;
//! - statements: the RR5 norm identity, then the denominator identity;
//! - evaluation field: the original checker's `MODULUS = 2^61 - 1`;
//! - issue seed `0x5252_355f_4e4f_524d`, 32 issue points per statement;
//! - checker seed `0x5252_355f_4348_454b`, 32 extra points per statement;
//! - the checker uses its existing statement-ID-bound `cert` and `extra`
//!   streams, with distinct frozen seeds; no point is supplied by this module;
//! - norm variable order `[X, A, r, s, b, c, d]`, degree bound 6;
//! - denominator variable order `[A, r, s, b, c, d]`, degree bound 5;
//! - emit each full certificate and checker verdict as one JSON record;
//! - require original-checker `Accept`, with exactly 32 + 32 checked points.
//!
//! The curve coefficient B is eliminated as `s^2-r^3-A*r`, so the norm
//! statement is an unconstrained polynomial identity. No curve membership,
//! denominator nonvanishing or factor-base assumption is imposed on samples.
//! These are deterministic producer-local finite-field checks. Distinct
//! streams do not constitute an independent execution, a fresh random
//! challenge, or a formal proof over the integers. `proof` is always `None`.
//! The checker's reported bound is retained as a diagnostic without a new
//! probability claim. These statements do not certify extraction completeness,
//! relation yield, a discrete logarithm, or solver cost.

use crypto_lib::cryptanalysis::identity_certificate::{Expr, IdentityStatement};

pub const ISSUE_SEED: u64 = 0x5252_355f_4e4f_524d;
pub const CHECKER_SEED: u64 = 0x5252_355f_4348_454b;
pub const ISSUE_POINTS: u32 = 32;
pub const CHECKER_POINTS: u32 = 32;

fn sub(lhs: Expr, rhs: Expr) -> Expr {
    Expr::plus(lhs, Expr::minus(rhs))
}

fn square(value: Expr) -> Expr {
    Expr::times(value.clone(), value)
}

fn cube(value: Expr) -> Expr {
    Expr::product(vec![value.clone(), value.clone(), value])
}

fn linear(x: Expr) -> Expr {
    Expr::plus(Expr::times(Expr::var("c"), x), Expr::var("d"))
}

fn v(x: Expr) -> Expr {
    Expr::plus(x, Expr::var("b"))
}

/// U(X) = (X^2+rX+r^2+A)V(X)^2 - (X-r)L(X)^2 - 2sL(X)V(X).
fn quartic(x: Expr) -> Expr {
    let l = linear(x.clone());
    let vx = v(x.clone());
    let dq = Expr::sum(vec![
        square(x.clone()),
        Expr::times(Expr::var("r"), x.clone()),
        square(Expr::var("r")),
        Expr::var("A"),
    ]);
    Expr::sum(vec![
        Expr::times(dq, square(vx.clone())),
        Expr::minus(Expr::times(sub(x, Expr::var("r")), square(l.clone()))),
        Expr::minus(Expr::product(vec![Expr::Const(2), Expr::var("s"), l, vx])),
    ])
}

/// Unconstrained norm identity after substituting B = s^2-r^3-A*r.
pub fn rr5_norm_statement() -> IdentityStatement {
    let x = Expr::var("X");
    let f = Expr::sum(vec![
        cube(x.clone()),
        Expr::times(Expr::var("A"), x.clone()),
        square(Expr::var("s")),
        Expr::minus(cube(Expr::var("r"))),
        Expr::minus(Expr::times(Expr::var("A"), Expr::var("r"))),
    ]);
    let x_minus_r = sub(x.clone(), Expr::var("r"));
    let vx = v(x.clone());
    let w = Expr::plus(
        Expr::times(x_minus_r.clone(), linear(x.clone())),
        Expr::times(Expr::var("s"), vx.clone()),
    );
    IdentityStatement {
        name: "RR5 unconstrained norm identity after eliminating curve B".into(),
        source: "research/prime31_four_summand_20261122/MATHEMATICS.md section 3; degree-five specialization of the existing RR9 norm construction; no novelty claim".into(),
        variables: ["X", "A", "r", "s", "b", "c", "d"]
            .into_iter()
            .map(String::from)
            .collect(),
        degree_bound: 6,
        lhs: sub(Expr::times(f, square(vx)), square(w)),
        rhs: Expr::times(x_minus_r, quartic(x)),
    }
}

/// Polynomial denominator identity; it does not assert nonvanishing.
pub fn rr5_denominator_statement() -> IdentityStatement {
    IdentityStatement {
        name: "RR5 U(-b) denominator specialization".into(),
        source: "research/prime31_four_summand_20261122/MATHEMATICS.md denominator lemma; U(-b)=(b+r)(d-b*c)^2; no nonvanishing assumption or formal-proof claim".into(),
        variables: ["A", "r", "s", "b", "c", "d"]
            .into_iter()
            .map(String::from)
            .collect(),
        degree_bound: 5,
        lhs: quartic(Expr::minus(Expr::var("b"))),
        rhs: Expr::times(
            Expr::plus(Expr::var("b"), Expr::var("r")),
            square(sub(
                Expr::var("d"),
                Expr::times(Expr::var("b"), Expr::var("c")),
            )),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::identity_certificate::{check_with_extra, issue, Verdict};
    use serde_json::{json, Value};

    fn verdict_json(verdict: &Verdict) -> Value {
        match verdict {
            Verdict::Accept {
                points,
                extra_points,
                false_accept_log2,
            } => json!({
                "kind": "accept",
                "points": points,
                "extra_points": extra_points,
                "checker_reported_false_accept_log2": false_accept_log2,
            }),
            Verdict::Reject(counterexample) => json!({
                "kind": "reject",
                "index": counterexample.index,
                "assignment": counterexample.assignment,
                "lhs": counterexample.lhs,
                "rhs": counterexample.rhs,
            }),
            Verdict::DigestMismatch {
                recorded,
                recomputed,
            } => json!({
                "kind": "digest_mismatch",
                "recorded": recorded,
                "recomputed": recomputed,
            }),
        }
    }

    #[test]
    fn rr5_sampled_identities_accept_original_checker() {
        for statement in [rr5_norm_statement(), rr5_denominator_statement()] {
            let certificate = issue(
                statement,
                ISSUE_SEED,
                ISSUE_POINTS,
                "Producer-local native issue and replay with frozen distinct issue/checker seeds and the existing domain-separated streams. No independent execution, fresh random challenge, formal proof, or signed PCC verdict.",
                None,
            )
            .expect("RR5 identity must issue under the frozen native checker");
            let verdict = check_with_extra(&certificate, Some((CHECKER_SEED, CHECKER_POINTS)))
                .expect("RR5 identity certificate must be structurally valid");
            let record = json!({
                "schema": "rr5.sampled-identity-check/v1",
                "certificate": certificate,
                "checker_seed": CHECKER_SEED,
                "verdict": verdict_json(&verdict),
                "evidence_tier": "producer_local_deterministic_finite_field_check",
                "formal_proof": false,
                "independent_execution": false,
                "fresh_random_challenge": false,
            });
            println!(
                "RR5_IDENTITY_RECORD {}",
                serde_json::to_string(&record).expect("RR5 identity record must serialize")
            );
            assert!(
                matches!(
                    &verdict,
                    Verdict::Accept { points, extra_points, .. }
                        if *points == ISSUE_POINTS && *extra_points == CHECKER_POINTS
                ),
                "original identity checker refused RR5 statement: {verdict:?}"
            );
        }
    }
}
