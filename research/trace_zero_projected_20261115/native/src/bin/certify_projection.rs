use full_trace_zero_backend::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityStatement, Verdict,
};
fn main() {
    let (t, z, h) = (Expr::var("t"), Expr::var("z"), Expr::var("h"));
    let square = |x: Expr| Expr::times(x.clone(), x);
    let translated = Expr::plus(t.clone(), Expr::times(z.clone(), h.clone()));
    let statement=IdentityStatement{name:"Integral expansion for Artin-Schreier product-coordinate translation".into(),source:"Known half-trace/Artin-Schreier identity; reduce the explicit even correction in characteristic two. Tr(z)=Tr(u)=0 and the actual curve equation supply the substitutions separately.".into(),variables:vec!["t".into(),"z".into(),"h".into()],degree_bound:4,lhs:Expr::plus(square(translated.clone()),Expr::times(z.clone(),translated)),rhs:Expr::sum(vec![square(t.clone()),Expr::times(z.clone(),t.clone()),Expr::times(square(z.clone()),Expr::plus(square(h.clone()),h.clone())),Expr::times(Expr::Const(2),Expr::times(t,Expr::times(z,h)))])};
    let cert = issue(
        statement,
        2026111523,
        32,
        "Evaluated integral expansion only; not formal half-trace completeness or timing proof",
        None,
    )
    .unwrap();
    let verdict = check_with_extra(&cert, Some((202611152399, 32))).unwrap();
    assert!(matches!(verdict, Verdict::Accept { .. }));
    eprintln!("{} {verdict:?}", cert.id);
    println!("{}", serde_json::to_string(&cert).unwrap());
}
