import ChoiceGeneral

namespace WeilDivisorChoiceAudit
noncomputable section
open WeierstrassCurve.Affine
open GeneralTranslationAudit WeilAuxiliaryRatioAudit
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
attribute [local instance] generalF2CommRing

/-- The actual binary curve has exactly these three rational points, proved by coordinates. -/
theorem binary_point_population (R : binaryCurve.Point) :
    R=0 ∨ R=binaryPoint ∨ R= -binaryPoint := by
  cases R with
  | zero => exact Or.inl rfl
  | some x y h =>
    fin_cases x <;> fin_cases y
    · exact Or.inr (Or.inl rfl)
    · right;right
      rw [binaryPoint,Point.neg_some]
      congr 1
    · have hn : ¬binaryCurve.Equation 1 0 := by rw [equation_iff];decide +kernel
      exact False.elim (hn h.1)
    · have hn : ¬binaryCurve.Equation 1 1 := by rw [equation_iff];decide +kernel
      exact False.elim (hn h.1)

/-- No rational shift has the disjoint support required by the source's example construction. -/
theorem binary_no_rational_shift :
    binaryCurve.Δ≠0 ∧ binaryPoint≠0 ∧
    ¬∃ S : binaryCurve.Point,
      Disjoint (shiftedRepresentative binaryCurve S binaryPoint).support {0,binaryPoint} := by
  refine ⟨binary_delta,binary_point_nonzero,?_⟩
  rintro ⟨S,hd⟩
  have hs : S∈(shiftedRepresentative binaryCurve S binaryPoint).support := by
    apply Finsupp.mem_support_iff.mpr
    have hn : binaryPoint+S≠S := by
      intro he
      exact binary_point_nonzero (add_right_cancel (he.trans (zero_add S).symm))
    simp [shiftedRepresentative,Ne.symm hn]
  rcases binary_point_population S with hS|hS|hS
  · exact Finset.disjoint_left.mp hd hs (by simp [hS])
  · exact Finset.disjoint_left.mp hd hs (by simp [hS])
  · subst S
    have hz : (0 : binaryCurve.Point)∈(shiftedRepresentative binaryCurve (-binaryPoint) binaryPoint).support := by
      apply Finsupp.mem_support_iff.mpr
      simp [shiftedRepresentative,binary_point_nonzero]
    exact Finset.disjoint_left.mp hd hz (by simp)

theorem binary_point_not_negative : binaryPoint≠ -binaryPoint := by
  rw [binaryPoint,Point.neg_some]
  simp only [ne_eq,Point.some.injEq]
  decide +kernel

#print axioms binary_point_population
#print axioms binary_no_rational_shift
end
end WeilDivisorChoiceAudit
