//! Exact integer and kernel-checked statements for the two orbit coordinates.
use super::*;
use identity_certificate::{check_with_extra, issue, Expr, IdentityStatement, Verdict};
use integer::ZPoly as Z;

fn expression(p: &Z) -> Expr {
    let names = ["u", "T", "B", "w", "v"];
    Expr::sum(
        p.0.iter()
            .map(|(e, &c)| {
                let mut factors = vec![Expr::Const(c.try_into().unwrap())];
                for i in 0..5 {
                    for _ in 0..e[i] {
                        factors.push(Expr::var(names[i]));
                    }
                }
                Expr::product(factors)
            })
            .collect(),
    )
}
fn products() -> Vec<(&'static str, Z, Z, Expr)> {
    let [u, t, b, w, _v] = std::array::from_fn(Z::var);
    let phi_n = w.pow(2).mul(&u.pow(3).add(&b.scale(4))).scale(-1);
    let phi_d = u.pow(2).scale(3);
    let phi_orbit_n = w
        .pow(2)
        .mul(&t.mul(&u).add(&b.mul(&u.pow(10)).scale(4)))
        .scale(-1);
    let phi_orbit_d = t.scale(3);
    let phi_lhs = phi_d.mul(&phi_orbit_n).sub(&phi_orbit_d.mul(&phi_n));
    let phi_rhs = w.pow(2).mul(&b).mul(&t.sub(&u.pow(12))).scale(12);
    let double_n = u.pow(4).sub(&b.mul(&u).scale(8));
    let double_d = u.pow(3).add(&b).scale(4);
    let double_orbit_n = u
        .mul(&t.add(&b.pow(4).scale(8)))
        .sub(&b.mul(&u.pow(10)).scale(9))
        .add(&b.pow(2).mul(&u.pow(7)).scale(9))
        .sub(&b.pow(3).mul(&u.pow(4)).scale(9));
    let double_orbit_d = t.sub(&b.pow(4)).scale(4);
    let double_lhs = double_d
        .mul(&double_orbit_n)
        .sub(&double_orbit_d.mul(&double_n));
    let double_rhs = b.mul(&u).mul(&t.sub(&u.pow(12))).scale(36);
    let factored = |d: &Z, on: &Z, od: &Z, n: &Z| {
        Expr::plus(
            Expr::times(expression(d), expression(on)),
            Expr::times(Expr::Const(-1), Expr::times(expression(od), expression(n))),
        )
    };
    let (quartic_lhs, quartic_rhs) = variants::integer_quartic_discriminant();
    let quartic_expression = expression(&quartic_lhs);
    vec![
        (
            "phi",
            phi_lhs,
            phi_rhs,
            factored(&phi_d, &phi_orbit_n, &phi_orbit_d, &phi_n),
        ),
        (
            "double",
            double_lhs,
            double_rhs,
            factored(&double_d, &double_orbit_n, &double_orbit_d, &double_n),
        ),
        (
            "quartic-discriminant",
            quartic_lhs,
            quartic_rhs,
            quartic_expression,
        ),
    ]
}
pub(super) fn write_proofs(out: &Path) {
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let mut proofs = Vec::new();
    for (i, (name, lhs, rhs, lhs_expr)) in products().into_iter().enumerate() {
        assert_eq!(lhs, rhs, "exact orbit map cross product failed: {name}");
        let s=IdentityStatement {
            name:format!("joint source-orbit {name} coordinate cross product"),
            source:"endomorphism_image_base/native/src/joint_proof.rs; exact cross product before imposing u^12=T".into(),
            variables:["u","T","B","w","v"].map(str::to_string).to_vec(),
            degree_bound:16,lhs:lhs_expr,rhs:expression(&rhs),
        };
        let cert = issue(
            s.clone(),
            0x4f524249545f5048 + i as u64,
            32,
            "Exact integer remainder zero; 32 checker-domain points",
            None,
        )
        .unwrap();
        assert!(matches!(
            check_with_extra(&cert, Some((0x4f524249545f4558 + i as u64, 32))).unwrap(),
            Verdict::Accept { .. }
        ));
        let mut bad = s;
        bad.rhs = Expr::plus(bad.rhs, Expr::Const(1));
        assert!(issue(bad, 1, 32, "plus-one negative control", None).is_err());
        let path = out.join(format!("{name}.identity.json"));
        std::fs::write(&path, serde_json::to_vec_pretty(&cert).unwrap()).unwrap();
        proofs.push(json!({"map":name,"certificate_id":cert.id,"exact_integer_remainder_terms":0,"additional_checks":32,"mutated_identity_rejected":true,"certificate_path":path.file_name().unwrap().to_str().unwrap()}));
    }
    std::fs::write(
        out.join("OrbitCoordinates.lean"),
        r#"import Std.Tactic
set_option maxRecDepth 10000
set_option maxHeartbeats 3000000
namespace JointOrbitCoordinates
variable {R : Type} [Lean.Grind.CommRing R]
theorem phi_cross (u T B w : R) :
    (3*u^2)*(-w^2*(T*u+4*B*u^10)) -
    (3*T)*(-w^2*(u^3+4*B)) = 12*w^2*B*(T-u^12) := by
  grind
theorem phi_on_orbit (u T B w : R) (hu : u^12=T) :
    (3*u^2)*(-w^2*(T*u+4*B*u^10)) =
    (3*T)*(-w^2*(u^3+4*B)) := by
  have h := phi_cross u T B w
  rw [hu] at h
  grind
theorem double_cross (u T B : R) :
    (4*(u^3+B))*(u*(T+8*B^4)-9*B*u^10+9*B^2*u^7-9*B^3*u^4) -
    (4*(T-B^4))*(u^4-8*B*u) = 36*B*u*(T-u^12) := by
  grind
theorem double_on_orbit (u T B : R) (hu : u^12=T) :
    (4*(u^3+B))*(u*(T+8*B^4)-9*B*u^10+9*B^2*u^7-9*B^3*u^4) =
    (4*(T-B^4))*(u^4-8*B*u) := by
  have h := double_cross u T B
  rw [hu] at h
  grind
def quarticD (a b c d : R) : R :=
  256*d^3-192*a*c*d^2-128*b^2*d^2+144*b*c^2*d-27*c^4+
  144*a^2*b*d^2-6*a^2*c^2*d-80*a*b^2*c*d+18*a*b*c^3+
  16*b^4*d-4*b^3*c^2-27*a^4*d^2+18*a^3*b*c*d-
  4*a^3*c^3-4*a^2*b^3*d+a^2*b^2*c^2
theorem quartic_discriminant (x y z w : R) :
    quarticD (x+y+z+w) (x*y+x*z+x*w+y*z+y*w+z*w)
      (x*y*z+x*y*w+x*z*w+y*z*w) (x*y*z*w) =
    (x-y)^2*(x-z)^2*(x-w)^2*(y-z)^2*(y-w)^2*(z-w)^2 := by
  unfold quarticD
  grind
theorem quartic_nonrepetition (x y z w : R)
    (h : quarticD (x+y+z+w) (x*y+x*z+x*w+y*z+y*w+z*w)
      (x*y*z+x*y*w+x*z*w+y*z*w) (x*y*z*w) ≠ 0) :
    x≠y ∧ x≠z ∧ x≠w ∧ y≠z ∧ y≠w ∧ z≠w := by
  rw [quartic_discriminant] at h
  grind
#print axioms phi_cross
#print axioms phi_on_orbit
#print axioms double_cross
#print axioms double_on_orbit
#print axioms quartic_discriminant
#print axioms quartic_nonrepetition
end JointOrbitCoordinates
"#,
    )
    .unwrap();
    write(
        &out.join("summary.json"),
        &json!({"schema":"joint-image-polynomial-proofs/v2","status":"integer-and-IDC-pass-Lean-pending","proofs":proofs,"scope":"Universal cross products, their u^12=T specializations and quartic discriminant identity/nonrepetition. Field division, curve/group semantics, source-orbit partition, resultant correctness and complexity remain separate obligations."}),
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn exact_orbit_cross_products_and_mutations() {
        for (name, a, b, _) in super::products() {
            assert_eq!(a, b, "{name}");
            assert_ne!(a.add(&super::Z::constant(1)), b);
        }
    }
}
