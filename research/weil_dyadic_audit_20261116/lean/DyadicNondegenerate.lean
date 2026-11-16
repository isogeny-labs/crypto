import DyadicTransport

namespace WeilDyadicAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit WeilTwoNondegeneracyAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K] [NeZero (2 : K)]

/-- Composition on actual torsion groups; both premises are discharged below. -/
theorem actual_double_level (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (t : ℤ)
    (hnd : ∀ P : torsionPoints W t,(∀ Q,geometricPairing W hΔ t P Q=1)→P=0)
    (hsurj : Function.Surjective (fun P : W.Point=>t • P))
    (P : torsionPoints W (t*2))
    (h : ∀ Q : torsionPoints W (t*2),geometricPairing W hΔ (t*2) P Q=1) : P=0 := by
  have hn : lowerLevel W t 2 P=0 := hnd _ (by
    intro Q
    rw [←geometric_pairing_level W hΔ t 2 P Q]
    exact h _)
  have hp2 : (2 : ℤ) • P.val=0 := congrArg Subtype.val hn
  let p : torsionPoints W 2:=⟨P.val,hp2⟩
  have hp0 : p=0 := by
    apply geometric_two_right_nondegenerate W hΔ p
    intro Q
    obtain ⟨R,hR⟩:=hsurj Q.val
    change t • R=Q.val at hR
    let Rh : torsionPoints W (t*2):=⟨R,by
      change (t*2) • R=0
      rw [mul_comm t 2,mul_smul,hR]
      exact Q.property⟩
    let Rl : torsionPoints W (2*t):=⟨R,by
      change (2*t) • R=0
      rw [mul_smul,hR]
      exact Q.property⟩
    have hu : geometricPairing W hΔ (t*2) Rh P=1 := by
      rw [geometric_pairing_swap,h Rh,inv_one]
    have hc : geometricPairing W hΔ (2*t) Rl (raiseLevel W 2 t p)=
        geometricPairing W hΔ (t*2) Rh P :=
      geometric_pairing_transport W hΔ (mul_comm 2 t) _ _ _ _ rfl rfl
    have hlow : lowerLevel W 2 t Rl=Q := Subtype.ext hR
    have hh:=geometric_pairing_level W hΔ 2 t Rl p
    rw [hc,hlow] at hh
    exact hh.symm.trans hu
  exact Subtype.ext (congrArg (fun X : torsionPoints W 2=>X.val) hp0)

/-- No abstract nondegeneracy or divisibility remains among the hypotheses. -/
theorem geometric_dyadic_left_nondegenerate (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    ∀ n : ℕ,∀ P : torsionPoints W ((2 : ℤ)^n),
      (∀ Q,geometricPairing W hΔ ((2 : ℤ)^n) P Q=1)→P=0 := by
  intro n
  induction n with
  | zero =>
    intro P _
    apply Subtype.ext
    change P.val=0
    have hp:=P.property
    change (1 : ℤ) • P.val=0 at hp
    simpa only [one_zsmul] using hp
  | succ n ih =>
    rw [pow_succ]
    exact actual_double_level W hΔ ((2 : ℤ)^n) ih
      (actual_dyadic_zsmul_surjective W hΔ n)

theorem geometric_dyadic_right_nondegenerate (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (n : ℕ) (Q : torsionPoints W ((2 : ℤ)^n))
    (h : ∀ P,geometricPairing W hΔ ((2 : ℤ)^n) P Q=1) : Q=0 := by
  apply geometric_dyadic_left_nondegenerate W hΔ n Q
  intro P
  rw [geometric_pairing_swap,h P,inv_one]

#print axioms actual_double_level
#print axioms geometric_dyadic_left_nondegenerate
#print axioms geometric_dyadic_right_nondegenerate
end
end WeilDyadicAudit
