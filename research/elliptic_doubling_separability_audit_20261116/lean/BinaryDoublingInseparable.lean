import BinaryDoublingFrobenius

namespace EllipticDoublingSeparabilityAudit
noncomputable section
open EllipticDoublingDegreeAudit GeneralTranslationAudit
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
attribute [local instance] generalF2CommRing

theorem actual_binary_doubling_purely_inseparable :
    IsPurelyInseparable (doublingImage binaryCurve binary_delta) binaryCurve.FunctionField := by
  have : CharP (doublingImage binaryCurve binary_delta) 2 :=
    charP_of_injective_algebraMap' GeneralF2 2
  have : ExpChar (doublingImage binaryCurve binary_delta) 2 := ExpChar.prime Nat.prime_two
  apply (isPurelyInseparable_iff_pow_mem (doublingImage binaryCurve binary_delta) 2).mpr
  intro f
  refine ⟨2,?_⟩
  change ∃ g : doublingImage binaryCurve binary_delta, (g : binaryCurve.FunctionField)=f^(2^2)
  refine ⟨⟨f^4,⟨NegationPullbackAudit.fieldNeg binaryCurve f,actual_binary_fourth_power_preimage f⟩⟩,rfl⟩

theorem actual_binary_doubling_not_surjective :
    ¬ Function.Surjective (doublingAlgHom binaryCurve binary_delta) := by
  intro hs
  have htop : doublingImage binaryCurve binary_delta=⊤ := AlgHom.fieldRange_eq_top.mpr hs
  have hd := actual_binary_doubling_degree
  rw [htop,IntermediateField.finrank_top] at hd
  norm_num at hd

theorem actual_binary_doubling_not_separable :
    ¬ Algebra.IsSeparable (doublingImage binaryCurve binary_delta) binaryCurve.FunctionField := by
  intro hs
  have := hs
  have := actual_binary_doubling_purely_inseparable
  have hsurj := IsPurelyInseparable.surjective_algebraMap_of_isSeparable
    (doublingImage binaryCurve binary_delta) binaryCurve.FunctionField
  apply actual_binary_doubling_not_surjective
  intro x
  obtain ⟨y,hy⟩ := hsurj x
  obtain ⟨z,hz⟩ := y.property
  exact ⟨z,hz.trans hy⟩

def UnguardedDoublingSeparabilityReading : Prop :=
  ∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0),
    Algebra.IsSeparable (doublingImage W hΔ) W.FunctionField

theorem wrong_doubling_separability_reading_false : ¬ UnguardedDoublingSeparabilityReading := by
  intro h
  exact actual_binary_doubling_not_separable (h GeneralF2 binaryCurve binary_delta)

#print axioms actual_binary_doubling_purely_inseparable
#print axioms actual_binary_doubling_not_separable
#print axioms wrong_doubling_separability_reading_false
end
end EllipticDoublingSeparabilityAudit
