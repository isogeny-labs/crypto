import ChoiceGeneral

namespace WeilDivisorChoiceAudit
noncomputable section
open WeierstrassCurve.Affine
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit FinitePointEvaluationAudit
variable {K : Type*} [Field K] [IsAlgClosed K]

omit [IsAlgClosed K] in
theorem proper_evaluation_one (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
    (hd : Disjoint D.support (functionDivisor W (↑(1 : W.FunctionFieldˣ) : W.FunctionField) (Units.ne_zero 1)).support) :
    evaluateDivisor W D 1 hd=1 := (divisorEvaluation W D).map_one

omit [IsAlgClosed K] in
theorem unit_divisor_one (W : WeierstrassCurve.Affine K) :
    functionDivisor W (↑(1 : W.FunctionFieldˣ) : W.FunctionField) (Units.ne_zero 1)=0 := by
  simpa only [Units.val_one] using function_divisor_one W

/-- The zero-level quotient itself equals1; root membership alone would be vacuous at n=0. -/
theorem pair_value_zero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (A : DivisorPair W 0) : pairValue W 0 A=1 := by
  have hg : functionDivisor W A.g A.g.ne_zero=functionDivisor W (↑(1 : W.FunctionFieldˣ) : W.FunctionField) (Units.ne_zero 1) := by
    rw [A.divG,zero_smul,unit_divisor_one]
  have hf : functionDivisor W A.f A.f.ne_zero=functionDivisor W (↑(1 : W.FunctionFieldˣ) : W.FunctionField) (Units.ne_zero 1) := by
    rw [A.divF,zero_smul,unit_divisor_one]
  have h1D : Disjoint A.D.support (functionDivisor W (↑(1 : W.FunctionFieldˣ) : W.FunctionField) (Units.ne_zero 1)).support := by
    rw [unit_divisor_one];simp
  have h1E : Disjoint A.E.support (functionDivisor W (↑(1 : W.FunctionFieldˣ) : W.FunctionField) (Units.ne_zero 1)).support := by
    rw [unit_divisor_one];simp
  have hgval:=geometric_degree_zero_choice W hΔ A.D A.degD A.g 1
    (domain_of_multiple W A.E A.D 0 A.g A.divG A.disjoint) h1D hg
  have hfval:=geometric_degree_zero_choice W hΔ A.E A.degE A.f 1
    (domain_of_multiple W A.D A.E 0 A.f A.divF A.disjoint.symm) h1E hf
  unfold pairValue
  rw [hgval,hfval,proper_evaluation_one,proper_evaluation_one,one_div_one]

#print axioms pair_value_zero
end
end WeilDivisorChoiceAudit
