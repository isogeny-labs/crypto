import MixedTransport

namespace WeilMixedLevelAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit WeilDyadicAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

/-- An actual product-level lemma; every premise is discharged in the mixed family. -/
theorem actual_product_level (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (t m : ℤ)
    (hnt : ∀ P : torsionPoints W t,(∀ Q,geometricPairing W hΔ t P Q=1)→P=0)
    (hnm : ∀ Q : torsionPoints W m,(∀ P,geometricPairing W hΔ m P Q=1)→Q=0)
    (hsurj : Function.Surjective (fun P : W.Point=>t • P))
    (P : torsionPoints W (t*m))
    (h : ∀ Q,geometricPairing W hΔ (t*m) P Q=1) : P=0 := by
  have hn : lowerLevel W t m P=0 := hnt _ (by
    intro Q
    rw [←geometric_pairing_level W hΔ t m P Q]
    exact h _)
  have hpm : m • P.val=0 := congrArg Subtype.val hn
  let p : torsionPoints W m:=⟨P.val,hpm⟩
  have hp0 : p=0 := by
    apply hnm p
    intro Q
    obtain ⟨R,hR⟩:=hsurj Q.val
    change t • R=Q.val at hR
    let Rh : torsionPoints W (t*m):=⟨R,by
      change (t*m) • R=0
      rw [mul_comm t m,mul_smul,hR]
      exact Q.property⟩
    let Rl : torsionPoints W (m*t):=⟨R,by
      change (m*t) • R=0
      rw [mul_smul,hR]
      exact Q.property⟩
    have hu : geometricPairing W hΔ (t*m) Rh P=1 := by
      rw [geometric_pairing_swap,h Rh,inv_one]
    have hc : geometricPairing W hΔ (m*t) Rl (raiseLevel W m t p)=
        geometricPairing W hΔ (t*m) Rh P :=
      geometric_pairing_transport W hΔ (mul_comm m t) _ _ _ _ rfl rfl
    have hlow : lowerLevel W m t Rl=Q := Subtype.ext hR
    have hh:=geometric_pairing_level W hΔ m t Rl p
    rw [hc,hlow] at hh
    exact hh.symm.trans hu
  exact Subtype.ext (congrArg (fun X : torsionPoints W m=>X.val) hp0)

variable [NeZero (2 : K)] [NeZero (3 : K)]

theorem geometric_mixed_left_nondegenerate (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (n : ℕ) (P : torsionPoints W (mixedLevel n))
    (h : ∀ Q,geometricPairing W hΔ (mixedLevel n) P Q=1) : P=0 :=
  actual_product_level W hΔ ((2 : ℤ)^n) 3
    (geometric_dyadic_left_nondegenerate W hΔ n)
    (geometric_pairing_three_right_nondegenerate W hΔ)
    (actual_dyadic_zsmul_surjective W hΔ n) P h

theorem geometric_mixed_right_nondegenerate (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (n : ℕ) (Q : torsionPoints W (mixedLevel n))
    (h : ∀ P,geometricPairing W hΔ (mixedLevel n) P Q=1) : Q=0 := by
  apply geometric_mixed_left_nondegenerate W hΔ n Q
  intro P
  rw [geometric_pairing_swap,h P,inv_one]

#print axioms actual_product_level
#print axioms geometric_mixed_left_nondegenerate
#print axioms geometric_mixed_right_nondegenerate
end
end WeilMixedLevelAudit
