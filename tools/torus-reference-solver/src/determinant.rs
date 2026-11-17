//! Standard Vandermonde identity, evaluated with the maintained native checker.
use torus_s4_tree::identity::{self, Expr, IdentityStatement};
pub fn statement() -> IdentityStatement {
    fn visit(p: &mut Vec<usize>, used: &mut [bool; 6], out: &mut Vec<Expr>) {
        if p.len() == 6 {
            let inversions = (0..6)
                .flat_map(|i| (i + 1..6).map(move |j| (i, j)))
                .filter(|(i, j)| p[*i] > p[*j])
                .count();
            let mut terms = vec![];
            for (i, column) in p.iter().enumerate() {
                for _ in 0..i {
                    terms.push(Expr::var(&format!("x{column}")));
                }
            }
            let term = Expr::product(terms);
            out.push(if inversions % 2 == 0 {
                term
            } else {
                Expr::minus(term)
            });
            return;
        }
        for i in 0..6 {
            if !used[i] {
                used[i] = true;
                p.push(i);
                visit(p, used, out);
                p.pop();
                used[i] = false;
            }
        }
    }
    let mut terms = vec![];
    visit(&mut vec![], &mut [false; 6], &mut terms);
    let factors = (0..6)
        .flat_map(|i| {
            (i + 1..6).map(move |j| {
                Expr::plus(
                    Expr::var(&format!("x{j}")),
                    Expr::minus(Expr::var(&format!("x{i}"))),
                )
            })
        })
        .collect();
    IdentityStatement{name:"Six-column Vandermonde determinant convention".into(),
        source:"Classical Vandermonde determinant; uniform matroid representation in arXiv1304.4626v4 section2.5".into(),
        variables:(0..6).map(|i|format!("x{i}")).collect(),degree_bound:15,
        lhs:Expr::sum(terms),rhs:Expr::product(factors)}
}
pub fn certificate() -> crate::Result<identity::IdentityCertificate> {
    let c = identity::issue(
        statement(),
        202611160643,
        32,
        "Producer-local native evaluated identity only; no formal or signed proof",
        None,
    )
    .map_err(|e| e.to_string())?;
    if !matches!(
        identity::check_with_extra(&c, Some((202611160649, 32))).map_err(|e| e.to_string())?,
        identity::Verdict::Accept { .. }
    ) {
        return Err("Vandermonde extra-point check rejected".into());
    }
    let mut bad = statement();
    bad.rhs = Expr::plus(bad.rhs, Expr::Const(1));
    if identity::issue(bad, 202611160643, 32, "mutation", None).is_ok() {
        return Err("corrupt identity sealed".into());
    }
    Ok(c)
}
