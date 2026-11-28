//! Emit an identity certificate for one substituted mixed-edge quadratic lift of
//! the characteristic-two three-variable summation polynomial.
//!
//! ```sh
//! cargo run --release --example identity_certificate_ecc2k130_edge_lift -- ac
//! cargo run --release --example identity_certificate_ecc2k130_edge_lift -- bc
//! ```

use crypto_lib::cryptanalysis::identity_certificate::{
    check, check_with_extra, issue, Expr, IdentityStatement, Verdict,
};

fn mul(xs: Vec<Expr>) -> Expr {
    Expr::product(xs)
}

fn sq(x: Expr) -> Expr {
    mul(vec![x.clone(), x])
}

fn v(name: &str) -> Expr {
    Expr::var(name)
}

fn statement(edge: &str) -> IdentityStatement {
    let (a, b, c, beta) = (v("x_a"), v("x_b"), v("x_c"), v("beta"));
    let ab = mul(vec![a.clone(), b.clone()]);
    let ac = mul(vec![a.clone(), c.clone()]);
    let bc = mul(vec![b.clone(), c.clone()]);
    let abc = mul(vec![a.clone(), b.clone(), c.clone()]);
    let expanded = Expr::sum(vec![
        sq(ab.clone()),
        sq(ac.clone()),
        sq(bc.clone()),
        abc,
        beta.clone(),
    ]);
    let substituted_lift = match edge {
        "ac" => Expr::sum(vec![sq(ac.clone()), sq(ab), sq(bc), mul(vec![ac, b]), beta]),
        "bc" => Expr::sum(vec![sq(bc.clone()), sq(ab), sq(ac), mul(vec![bc, a]), beta]),
        _ => panic!("edge must be ac or bc"),
    };
    IdentityStatement {
        name: format!("ECC2K-130 S3 substituted {edge}-edge lift"),
        source: "research/ecc2k130_edge_lift_forms_20261126/README.md section 1".into(),
        variables: vec!["x_a".into(), "x_b".into(), "x_c".into(), "beta".into()],
        degree_bound: 4,
        lhs: expanded,
        rhs: substituted_lift,
    }
}

fn main() {
    let edge = std::env::args().nth(1).expect("usage: ... ac|bc");
    let cert = issue(
        statement(&edge),
        0xECC2_0130_ED63_0001 ^ if edge == "ac" { 0xac } else { 0xbc },
        32,
        "issued by examples/identity_certificate_ecc2k130_edge_lift.rs from an explicit expression tree; checked by cryptanalysis::identity_certificate::check replaying from the sealed id and seed",
        None,
    )
    .expect("the substituted lift identity holds");
    match check(&cert).expect("well-formed certificate") {
        Verdict::Accept { .. } => {}
        other => panic!("self-check failed: {other:?}"),
    }
    match check_with_extra(&cert, Some((0xECC2_0130_C0DE_0001, 16)))
        .expect("well-formed extra check")
    {
        Verdict::Accept {
            extra_points: 16, ..
        } => {}
        other => panic!("independent-point check failed: {other:?}"),
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&cert).expect("serialises")
    );
}
