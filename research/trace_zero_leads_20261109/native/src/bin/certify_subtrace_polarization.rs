//! Integral expansion behind subtrace polarization; not a field-model proof.
use full_trace_zero_backend::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityStatement, Verdict,
};
fn main() {
    let x: Vec<_> = (0..16).map(|i| Expr::var(&format!("x{i}"))).collect();
    let z: Vec<_> = (0..16).map(|i| Expr::var(&format!("z{i}"))).collect();
    let s2 = |a: &[Expr]| {
        Expr::sum(
            (0..a.len())
                .flat_map(|i| {
                    (i + 1..a.len()).map(move |j| Expr::times(a[i].clone(), a[j].clone()))
                })
                .collect(),
        )
    };
    let sum: Vec<_> = x
        .iter()
        .zip(&z)
        .map(|(a, b)| Expr::plus(a.clone(), b.clone()))
        .collect();
    let statement=IdentityStatement {
        name:"Integral polarization of the second elementary symmetric polynomial in16 coordinates".into(),
        source:"Expansion of S2; Ri et al. arXiv1304.0521 defines field subtrace; trace_zero_leads_20261109/README.md".into(),
        variables:(0..16).map(|i|format!("x{i}")).chain((0..16).map(|i|format!("z{i}"))).collect(),
        degree_bound:2,
        lhs:Expr::sum(vec![s2(&sum),Expr::minus(s2(&x)),Expr::minus(s2(&z))]),
        rhs:Expr::plus(Expr::times(Expr::sum(x.clone()),Expr::sum(z.clone())),Expr::minus(Expr::sum(x.iter().zip(&z).map(|(a,b)|Expr::times(a.clone(),b.clone())).collect()))),
    };
    let certificate = issue(
        statement,
        20261109,
        32,
        "Local producer issue/replay; no independent signature or formal proof",
        None,
    )
    .unwrap();
    let result = check_with_extra(&certificate, Some((0x7375627472616365, 32))).unwrap();
    assert!(matches!(result, Verdict::Accept { .. }));
    eprintln!("{} {result:?}; certificate concerns an integral polynomial expansion, not the characteristic-two descent",certificate.id);
    println!("{}", serde_json::to_string(&certificate).unwrap());
}
