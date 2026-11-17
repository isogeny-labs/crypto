import ShortPointCount

namespace CardinalityControlAudit
noncomputable section
open ShortPointCountAudit
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩

def E5 : WeierstrassCurve.Affine (ZMod 5) := shortCurve 3 0

theorem E5_nonsingular : E5.Δ≠0 := by
  unfold E5 shortCurve WeierstrassCurve.Δ WeierstrassCurve.b₂
    WeierstrassCurve.b₄ WeierstrassCurve.b₆ WeierstrassCurve.b₈
  decide +kernel

theorem E5_cardinality : Nat.card E5.Point=10 := by
  rw [Nat.card_congr E5.nonsingularPointEquiv]
  change Nat.card (Option {xy : ZMod 5 × ZMod 5 // E5.Nonsingular xy.1 xy.2})=10
  let : DecidablePred (fun xy : ZMod 5 × ZMod 5 => E5.Nonsingular xy.1 xy.2) :=
    fun xy => decidable_of_iff (xy.2^2=xy.1^3+3*xy.1) (by
      change xy.2^2=xy.1^3+3*xy.1 ↔ E5.Nonsingular xy.1 xy.2
      erw [←WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
        E5_nonsingular,WeierstrassCurve.Affine.equation_iff]
      simp [E5,shortCurve])
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem E5_order_five_point : ∃ P : E5.Point,addOrderOf P=5 := by
  apply SmallCharacteristicPrimeAudit.actual_prime_order_point E5 5
  rw [E5_cardinality]
  decide +kernel

theorem E5_no_missing_fiber : ∀ x : ZMod 5,∃ y : ZMod 5,y^2=x^3+3*x := by
  decide +kernel

/-- A rational point of order equal to the characteristic does not alone
force the whole curve point count. The missing fiber must actually be checked. -/
theorem unrestricted_point_order_forces_cardinality_false :
    ¬ ∀ (q : ℕ) [Fact (Nat.Prime q)] (E : WeierstrassCurve.Affine (ZMod q))
      (P : E.Point), E.Δ≠0 → addOrderOf P=q → Nat.card E.Point=q := by
  intro h
  obtain ⟨P,hP⟩ := E5_order_five_point
  have hh := h 5 E5 P E5_nonsingular hP
  rw [E5_cardinality] at hh
  contradiction

#print axioms E5_cardinality
#print axioms unrestricted_point_order_forces_cardinality_false
end
end CardinalityControlAudit
