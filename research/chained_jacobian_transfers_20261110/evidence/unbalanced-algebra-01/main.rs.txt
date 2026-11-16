//! Reuse the repository's unchanged identity issuer/checker and SHA-256.
//! These evaluated records are not formal proofs over Z or signed claims.
#[path = "../../../../src/hash/sha256.rs"]
#[allow(clippy::chunks_exact_to_as_chunks)] // Preserve the frozen imported SHA source.
pub mod sha256_native;
mod hash {
    pub use crate::sha256_native as sha256;
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
#[allow(dead_code)]
mod identity;

use identity::{Expr, IdentityStatement};

fn power(x: Expr, n: usize) -> Expr {
    Expr::Mul(vec![x; n])
}
fn duplication() -> IdentityStatement {
    let x = Expr::var("x");
    let a = Expr::var("a");
    let b = Expr::var("b");
    let rhs_curve = Expr::Add(vec![
        power(x.clone(), 3),
        Expr::Mul(vec![a.clone(), x.clone()]),
        b.clone(),
    ]);
    let slope = Expr::Add(vec![
        Expr::Mul(vec![Expr::Const(3), power(x.clone(), 2)]),
        a.clone(),
    ]);
    IdentityStatement {
        name: "Cleared duplication ordinate numerator h4".into(),
        source: "QUARTIC_GLUING_PROOF.md;short-Weierstrass doubling and Galbraithch26 Miller recurrence;Lean QuarticGluing.duplication_ordinate_numerator".into(),
        variables: vec!["x".into(), "a".into(), "b".into()],
        degree_bound: 6,
        lhs: Expr::Add(vec![
            Expr::Mul(vec![Expr::Const(-8), power(rhs_curve.clone(), 2)]),
            Expr::Mul(vec![Expr::Const(12), x.clone(), rhs_curve, slope.clone()]),
            Expr::Neg(Box::new(power(slope, 3))),
        ]),
        rhs: Expr::Add(vec![
            power(x.clone(), 6),
            Expr::Mul(vec![Expr::Const(5), a.clone(), power(x.clone(), 4)]),
            Expr::Mul(vec![Expr::Const(20), b.clone(), power(x.clone(), 3)]),
            Expr::Mul(vec![Expr::Const(-5), power(a.clone(), 2), power(x.clone(), 2)]),
            Expr::Mul(vec![Expr::Const(-4), a.clone(), b.clone(), x]),
            Expr::Mul(vec![Expr::Const(-8), power(b, 2)]),
            Expr::Neg(Box::new(power(a, 3))),
        ]),
    }
}

fn halving() -> IdentityStatement {
    let x = Expr::var("x");
    let a = Expr::var("a");
    let b = Expr::var("b");
    let target = Expr::var("X");
    let slope = Expr::Add(vec![
        Expr::Mul(vec![Expr::Const(3), power(x.clone(), 2)]),
        a.clone(),
    ]);
    let curve = Expr::Add(vec![
        power(x.clone(), 3),
        Expr::Mul(vec![a.clone(), x.clone()]),
        b.clone(),
    ]);
    IdentityStatement {
        name:"Cleared doubling abscissa quartic for complete halving".into(),
        source:"TWO_PRIMARY_QUOTIENT_PROOF.md;Galbraithch9eq9.2;Lean TwoPrimaryQuotient.halving_abscissa_identity".into(),
        variables:vec!["x".into(),"a".into(),"b".into(),"X".into()],degree_bound:4,
        lhs:Expr::Add(vec![power(slope,2),Expr::Mul(vec![Expr::Const(-4),curve,Expr::Add(vec![Expr::Mul(vec![Expr::Const(2),x.clone()]),target.clone()])])]),
        rhs:Expr::Add(vec![power(x.clone(),4),Expr::Mul(vec![Expr::Const(-4),target.clone(),power(x.clone(),3)]),Expr::Mul(vec![Expr::Const(-2),a.clone(),power(x.clone(),2)]),Expr::Mul(vec![Expr::Const(-1),Expr::Add(vec![Expr::Mul(vec![Expr::Const(8),b.clone()]),Expr::Mul(vec![Expr::Const(4),a.clone(),target.clone()])]),x]),power(a,2),Expr::Mul(vec![Expr::Const(-4),b,target])]),
    }
}

fn norm(t: Expr, p: Expr, a: Expr, b: Expr) -> Expr {
    Expr::Add(vec![
        power(a.clone(), 2),
        Expr::Mul(vec![t, a, b.clone()]),
        Expr::Mul(vec![p, power(b, 2)]),
    ])
}

fn norm_multiplicativity() -> IdentityStatement {
    let [t, p, a, b, c, d] = ["t", "p", "a", "b", "c", "d"].map(Expr::var);
    let real = Expr::Add(vec![
        Expr::Mul(vec![a.clone(), c.clone()]),
        Expr::Mul(vec![Expr::Const(-1), p.clone(), b.clone(), d.clone()]),
    ]);
    let imaginary = Expr::Add(vec![
        Expr::Mul(vec![a.clone(), d.clone()]),
        Expr::Mul(vec![b.clone(), c.clone()]),
        Expr::Mul(vec![t.clone(), b.clone(), d.clone()]),
    ]);
    IdentityStatement {
        name: "Quadratic CM norm multiplicativity".into(),
        source: "ISOGENY_CYCLE_PROOF.md;Galbraithch25 ordinary CM;Lean IsogenyCycles.quadratic_norm_multiplicative".into(),
        variables: ["t", "p", "a", "b", "c", "d"].map(String::from).to_vec(),
        degree_bound: 7,
        lhs: Expr::Mul(vec![norm(t.clone(), p.clone(), a, b), norm(t.clone(), p.clone(), c, d)]),
        rhs: norm(t, p, real, imaginary),
    }
}

fn first_monodromy() -> IdentityStatement {
    let x = Expr::var("x");
    IdentityStatement {
        name: "First elliptic cycle polynomial reduction".into(),
        source: "ISOGENY_CYCLE_PROOF.md;Lean IsogenyCycles.first_cycle_polynomial".into(),
        variables: vec!["x".into()],
        degree_bound: 3,
        lhs: Expr::Add(vec![
            power(Expr::Add(vec![x.clone(), Expr::Const(-1)]), 3),
            Expr::Const(-1),
        ]),
        rhs: Expr::Mul(vec![
            Expr::Add(vec![
                power(x.clone(), 2),
                Expr::Neg(Box::new(x.clone())),
                Expr::Const(1),
            ]),
            Expr::Add(vec![x, Expr::Const(-2)]),
        ]),
    }
}

fn second_monodromy() -> IdentityStatement {
    let x = Expr::var("x");
    IdentityStatement {
        name: "Second elliptic cycle polynomial reduction modulo four".into(),
        source: "ISOGENY_CYCLE_PROOF.md;Lean IsogenyCycles.second_cycle_polynomial".into(),
        variables: vec!["x".into()],
        degree_bound: 4,
        lhs: Expr::Add(vec![
            power(Expr::Add(vec![x.clone(), Expr::Const(2)]), 4),
            x.clone(),
        ]),
        rhs: Expr::Add(vec![
            Expr::Mul(vec![
                Expr::Add(vec![
                    power(x.clone(), 2),
                    Expr::Neg(Box::new(x.clone())),
                    Expr::Const(1),
                ]),
                Expr::Add(vec![
                    power(x.clone(), 2),
                    Expr::Mul(vec![Expr::Const(9), x.clone()]),
                    Expr::Const(32),
                ]),
            ]),
            Expr::Mul(vec![
                Expr::Const(4),
                Expr::Add(vec![Expr::Mul(vec![Expr::Const(14), x]), Expr::Const(-4)]),
            ]),
        ]),
    }
}

fn even_cover(direct: bool) -> IdentityStatement {
    let [a, b, c, d, x] = ["a", "b", "c", "d", "x"].map(Expr::var);
    let source = if direct {
        Expr::Add(vec![
            Expr::Mul(vec![a.clone(), power(x.clone(), 6)]),
            Expr::Mul(vec![b.clone(), power(x.clone(), 4)]),
            Expr::Mul(vec![c.clone(), power(x.clone(), 2)]),
            d.clone(),
        ])
    } else {
        Expr::Add(vec![
            a.clone(),
            Expr::Mul(vec![b.clone(), power(x.clone(), 2)]),
            Expr::Mul(vec![c.clone(), power(x.clone(), 4)]),
            Expr::Mul(vec![d.clone(), power(x.clone(), 6)]),
        ])
    };
    let n = if direct {
        Expr::Add(vec![
            Expr::Mul(vec![Expr::Const(3), a.clone(), power(x.clone(), 2)]),
            b.clone(),
        ])
    } else {
        Expr::Add(vec![
            Expr::Mul(vec![Expr::Const(3), a.clone()]),
            Expr::Mul(vec![b.clone(), power(x.clone(), 2)]),
        ])
    };
    let middle = Expr::Mul(vec![
        Expr::Add(vec![
            Expr::Mul(vec![Expr::Const(9), a.clone(), c.clone()]),
            Expr::Mul(vec![Expr::Const(-3), power(b.clone(), 2)]),
        ]),
        n.clone(),
    ]);
    let tail = Expr::Add(vec![
        Expr::Mul(vec![Expr::Const(2), power(b.clone(), 3)]),
        Expr::Mul(vec![Expr::Const(-9), a.clone(), b, c]),
        Expr::Mul(vec![Expr::Const(27), power(a.clone(), 2), d]),
    ]);
    let short = if direct {
        "direct_short_quotient"
    } else {
        "reciprocal_short_quotient"
    };
    IdentityStatement {
        name: format!("Nonmonic even sextic {short} cleared cubic identity"),
        source: format!("UNBALANCED_MODEL_PROOF.md;Bruin-Doerksen3.2 and existing cubic_to_short;Lean UnbalancedCovers.{short}"),
        variables: ["a", "b", "c", "d", "x"].map(String::from).to_vec(),
        degree_bound: 9,
        lhs: Expr::Mul(vec![Expr::Const(27), power(a, 2), source]),
        rhs: Expr::Add(vec![power(n, 3), if direct { middle } else { Expr::Mul(vec![middle, power(x.clone(), 4)]) }, if direct { tail } else { Expr::Mul(vec![tail, power(x, 6)]) }]),
    }
}

fn main() {
    let outputs = std::env::args_os().skip(1).collect::<Vec<_>>();
    let cases: [(IdentityStatement, &[u8], &str, u64); 7] = [
        (
            duplication(),
            include_bytes!("../../lean/QuarticGluing.lean"),
            "research/chained_jacobian_transfers_20261110/lean/QuarticGluing.lean",
            0x384c,
        ),
        (
            halving(),
            include_bytes!("../../lean/TwoPrimaryQuotient.lean"),
            "research/chained_jacobian_transfers_20261110/lean/TwoPrimaryQuotient.lean",
            0x394c,
        ),
        (
            norm_multiplicativity(),
            include_bytes!("../../lean/IsogenyCycles.lean"),
            "research/chained_jacobian_transfers_20261110/lean/IsogenyCycles.lean",
            0x4541,
        ),
        (
            first_monodromy(),
            include_bytes!("../../lean/IsogenyCycles.lean"),
            "research/chained_jacobian_transfers_20261110/lean/IsogenyCycles.lean",
            0x4542,
        ),
        (
            second_monodromy(),
            include_bytes!("../../lean/IsogenyCycles.lean"),
            "research/chained_jacobian_transfers_20261110/lean/IsogenyCycles.lean",
            0x4543,
        ),
        (
            even_cover(true),
            include_bytes!("../../lean/UnbalancedCovers.lean"),
            "research/chained_jacobian_transfers_20261110/lean/UnbalancedCovers.lean",
            0x4641,
        ),
        (
            even_cover(false),
            include_bytes!("../../lean/UnbalancedCovers.lean"),
            "research/chained_jacobian_transfers_20261110/lean/UnbalancedCovers.lean",
            0x4642,
        ),
    ];
    for (i, (statement, source, path, seed)) in cases.into_iter().enumerate() {
        let cert=identity::issue(statement,seed,32,"Same producer issues;existing repository checker code plus domain-separated64extra points;scientific standing unsigned",Some(identity::ProofRef {kind:"lean4;pointer only;requires separate proof gate".into(),path:path.into(),sha256:hex::encode(sha256_native::sha256(source))})).expect("the issuer must reject any false identity");
        let verdict = identity::check_with_extra(&cert, Some((seed + 1, 64))).unwrap();
        assert!(matches!(verdict, identity::Verdict::Accept { .. }));
        if let Some(path) = outputs.get(i) {
            std::fs::write(
                path,
                format!("{}\n", serde_json::to_string_pretty(&cert).unwrap()),
            )
            .unwrap();
        }
        println!("PASS {}:32issuer+64domain-separatedchecker points over fixed2^61-1;existing unchanged checker and SHA;not a signed or all-characteristics proof",cert.id);
    }
}

#[test]
fn mutated_identities_are_rejected() {
    for mut statement in [
        duplication(),
        halving(),
        norm_multiplicativity(),
        first_monodromy(),
        second_monodromy(),
        even_cover(true),
        even_cover(false),
    ] {
        statement.rhs = Expr::Add(vec![statement.rhs, Expr::Const(1)]);
        assert!(matches!(
            identity::issue(statement, 0x384c, 32, "negative control", None),
            Err(identity::IssueError::False(_))
        ));
    }
}
