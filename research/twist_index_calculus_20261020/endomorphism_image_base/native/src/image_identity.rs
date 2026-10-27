//! An exact polynomial derivation of x(P-psi(P)), before division.
use super::*;
use identity_certificate::{check_with_extra, issue, Expr, IdentityStatement, Verdict};
fn expression(poly: &integer::ZPoly) -> Expr {
    let names = ["x", "y", "z", "w", "B"];
    Expr::sum(
        poly.0
            .iter()
            .map(|(e, &c)| {
                let mut f = vec![Expr::Const(c.try_into().unwrap())];
                for i in 0..5 {
                    for _ in 0..e[i] {
                        f.push(Expr::var(names[i]));
                    }
                }
                Expr::product(f)
            })
            .collect(),
    )
}
pub(super) fn derive() -> (integer::ZPoly, integer::ZPoly) {
    use integer::ZPoly as Z;
    let [x, y, z, w, b] = std::array::from_fn(Z::var);
    let one = Z::constant(1);
    let hw = w.pow(2).add(&w).add(&one);
    let hc = y.pow(2).sub(&x.pow(3)).sub(&b);
    let hl = w
        .sub(&one)
        .pow(2)
        .mul(&x.pow(2))
        .mul(&z.add(&one.add(&w).mul(&x)))
        .sub(&y.pow(2).scale(4));
    let target = x
        .pow(2)
        .mul(&z)
        .scale(3)
        .add(&w.pow(2).mul(&x.pow(3).add(&b.scale(4))));
    let mult = x
        .pow(2)
        .mul(&z)
        .mul(&w.pow(2).sub(&w.scale(3)).add(&Z::constant(3)))
        .add(&x.pow(3).mul(&w.pow(2)).mul(&w.sub(&Z::constant(2))));
    let rhs = w
        .pow(2)
        .mul(&hl)
        .scale(-1)
        .sub(&w.pow(2).mul(&hc).scale(4))
        .add(&hw.mul(&mult));
    (target, rhs)
}
pub(super) fn write(out: &Path) {
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let (lhs, rhs) = derive();
    assert_eq!(lhs, rhs, "exact integer map derivation failed");
    use integer::ZPoly as Z;
    let [x, y, z, w, b] = std::array::from_fn(Z::var);
    let one = Z::constant(1);
    let hw = w.pow(2).add(&w).add(&one);
    let hc = y.pow(2).sub(&x.pow(3)).sub(&b);
    let hl = w
        .sub(&one)
        .pow(2)
        .mul(&x.pow(2))
        .mul(&z.add(&one.add(&w).mul(&x)))
        .sub(&y.pow(2).scale(4));
    let mult = x
        .pow(2)
        .mul(&z)
        .mul(&w.pow(2).sub(&w.scale(3)).add(&Z::constant(3)))
        .add(&x.pow(3).mul(&w.pow(2)).mul(&w.sub(&Z::constant(2))));
    let rhs_expr = Expr::sum(vec![
        Expr::product(vec![
            Expr::Const(-1),
            expression(&w.pow(2)),
            expression(&hl),
        ]),
        Expr::product(vec![
            Expr::Const(-4),
            expression(&w.pow(2)),
            expression(&hc),
        ]),
        Expr::times(expression(&hw), expression(&mult)),
    ]);
    let statement=IdentityStatement {name:"j0 degree-three image coordinate polynomial derivation".into(),source:"endomorphism_image_base/native/src/image_identity.rs; explicit combination of slope, curve and cube-root equations".into(),variables:["x","y","z","w","B"].map(str::to_string).to_vec(),degree_bound:8,lhs:expression(&lhs),rhs:rhs_expr};
    let cert = issue(
        statement.clone(),
        0x5048495F494D4147,
        32,
        "zero exact integer remainder; 32 additional checker-domain points",
        None,
    )
    .unwrap();
    assert!(matches!(
        check_with_extra(&cert, Some((0x5048495F45585452, 32))).unwrap(),
        Verdict::Accept { .. }
    ));
    let mut bad = statement;
    bad.rhs = Expr::plus(bad.rhs, Expr::Const(1));
    assert!(issue(bad, 1, 32, "negative control", None).is_err());
    std::fs::write(
        out.join("identity.json"),
        serde_json::to_vec_pretty(&cert).unwrap(),
    )
    .unwrap();
    std::fs::write(
        out.join("ImageCoordinate.lean"),
        r#"import Std.Tactic
set_option maxRecDepth 10000
set_option maxHeartbeats 1000000
namespace ImageCoordinate
variable {R : Type} [Lean.Grind.CommRing R]
theorem coordinate (x y z w B : R)
    (hw : w^2+w+1=0) (hc : y^2=x^3+B)
    (hl : (w-1)^2*x^2*(z+(1+w)*x)=4*y^2) :
    3*x^2*z = -w^2*(x^3+4*B) := by
  grind
#print axioms coordinate
end ImageCoordinate
"#,
    )
    .unwrap();
    std::fs::write(out.join("summary.json"),serde_json::to_vec_pretty(&json!({"status":"pass","certificate_id":cert.id,"integer_remainder_terms":0,"mutated_identity_rejected":true})).unwrap()).unwrap();
}
#[cfg(test)]
mod tests {
    #[test]
    fn exact_map_identity_and_corruption() {
        let (a, b) = super::derive();
        assert_eq!(a, b);
        assert_ne!(a.add(&crate::integer::ZPoly::constant(1)), b);
    }
}
