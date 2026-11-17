use full_trace_zero_backend::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityStatement, Verdict,
};
fn main() {
    let (x, y, z) = (Expr::var("x"), Expr::var("y"), Expr::var("z"));
    let xy = Expr::times(x.clone(), y.clone());
    let sum = Expr::sum(vec![
        xy.clone(),
        Expr::times(x.clone(), z.clone()),
        Expr::times(y.clone(), z.clone()),
    ]);
    let u = Expr::plus(x, y);
    let statement=IdentityStatement{name:"Integral expansion underlying the existing symmetric binary S3 form".into(),source:"src/cryptanalysis/binary_semaev.rs S3 derivation; reduce the explicit even correction in characteristic two".into(),variables:vec!["x".into(),"y".into(),"z".into()],degree_bound:4,
 lhs:Expr::times(sum.clone(),sum),rhs:Expr::sum(vec![Expr::times(xy.clone(),xy.clone()),Expr::times(Expr::times(z.clone(),z.clone()),Expr::times(u.clone(),u.clone())),Expr::times(Expr::Const(2),Expr::times(Expr::times(xy,z),u))])};
    let c=issue(statement,2026111512,32,"Local producer issue/replay; evaluated integral expansion only, not a formal F2 solver/completeness proof",None).unwrap();
    let v = check_with_extra(&c, Some((0x73796d6d65747279, 32))).unwrap();
    assert!(matches!(v, Verdict::Accept { .. }));
    eprintln!("{} {v:?}", c.id);
    println!("{}", serde_json::to_string(&c).unwrap());
}
