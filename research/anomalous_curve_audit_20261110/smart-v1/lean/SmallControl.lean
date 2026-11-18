import SmartArithmetic

namespace SmartSmallControlAudit
open AnomalousAffineCertificateAudit
instance : Fact (Nat.Prime 5) := ⟨by decide +kernel⟩
abbrev F := ZMod 5
def curve : WeierstrassCurve.Affine F := ⟨0,0,0,3,2⟩
theorem nonsingular : curve.Δ≠0 := by decide +kernel

theorem equation_G : curve.Equation (1:F) (1:F) := by
  rw [WeierstrassCurve.Affine.equation_iff]
  decide +kernel
theorem equation_Q : curve.Equation (2:F) (1:F) := by
  rw [WeierstrassCurve.Affine.equation_iff]
  decide +kernel
theorem offcurve_rejected : ¬curve.Equation (2:F) (2:F) := by
  rw [WeierstrassCurve.Affine.equation_iff]
  decide +kernel

noncomputable def G : curve.Point := .some 1 1
  ((WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero nonsingular).mp equation_G)
noncomputable def Q : curve.Point := .some 2 1
  ((WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero nonsingular).mp equation_Q)

theorem G_double : G+G=Q := by
  dsimp only [G,Q]
  apply tangent_double (ℓ:=(3:F))
  all_goals decide +kernel

theorem Q_double : Q+Q= -G := by
  dsimp only [G,Q]
  rw [WeierstrassCurve.Affine.Point.neg_some]
  apply tangent_double (ℓ:=(0:F))
  all_goals decide +kernel

theorem G_order : addOrderOf G=5 := by
  have h₂ : (2:ℕ) • G=Q := by rw [two_nsmul]; exact G_double
  have h₄ : (4:ℕ) • G= -G := by
    rw [show (4:ℕ)=2+2 by rfl,add_nsmul,h₂]
    exact Q_double
  apply addOrderOf_eq_prime
  · calc (5:ℕ) • G=(4:ℕ) • G+G := succ_nsmul G 4
         _ = -G+G := by rw [h₄]
         _ = 0 := neg_add_cancel G
  · simp [G]

theorem control_certificate : curve.Δ≠0 ∧ addOrderOf G=5 ∧
    (2:ℕ) • G=Q ∧ ¬curve.Equation (2:F) (2:F) := by
  refine ⟨nonsingular,G_order,?_,offcurve_rejected⟩
  rw [two_nsmul]
  exact G_double

#print axioms control_certificate
end SmartSmallControlAudit
