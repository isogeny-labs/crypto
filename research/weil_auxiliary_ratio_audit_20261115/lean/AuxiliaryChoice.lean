import AuxiliaryWeilThree

namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit GeneralTranslationAudit ThreeTorsionRankAudit.General
variable {K : Type*} [Field K] [DecidableEq K]

theorem equal_divisors_admissible (W : WeierstrassCurve.Affine K) (P Q S : W.Point)
    (f g f' g' : W.FunctionFieldˣ)
    (hf : functionDivisor W f f.ne_zero=functionDivisor W f' f'.ne_zero)
    (hg : functionDivisor W g g.ne_zero=functionDivisor W g' g'.ne_zero)
    (h : AuxiliaryAdmissible W P Q f g S) : AuxiliaryAdmissible W P Q f' g' S := by
  have hfo (R : W.Point) : pointOrder W f R=pointOrder W f' R :=
    congrArg (fun D : W.Point →₀ ℤ => D R) hf
  have hgo (R : W.Point) : pointOrder W g R=pointOrder W g' R :=
    congrArg (fun D : W.Point →₀ ℤ => D R) hg
  exact ⟨(hfo _).symm.trans h.1,(hgo _).symm.trans h.2.1,
    (hfo _).symm.trans h.2.2.1,(hgo _).symm.trans h.2.2.2⟩

/-- Actual function choices with the same full geometric divisors give equal source values. -/
theorem source_function_choice [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P Q S : W.Point)
    (f g f' g' : W.FunctionFieldˣ)
    (hf : functionDivisor W f f.ne_zero=functionDivisor W f' f'.ne_zero)
    (hg : functionDivisor W g g.ne_zero=functionDivisor W g' g'.ne_zero)
    (h : AuxiliaryAdmissible W P Q f g S) :
    sourceAuxiliaryValue W hΔ P Q S f g h=
      sourceAuxiliaryValue W hΔ P Q S f' g' (equal_divisors_admissible W P Q S f g f' g' hf hg h) := by
  let h' := equal_divisors_admissible W P Q S f g f' g' hf hg h
  have hgt : functionDivisor W (generalTranslatedUnit W hΔ (-S) g)
        (generalTranslatedUnit W hΔ (-S) g).ne_zero=
      functionDivisor W (generalTranslatedUnit W hΔ (-S) g')
        (generalTranslatedUnit W hΔ (-S) g').ne_zero := by
    change functionDivisor W (generalTranslation W hΔ (-S) g)
        (general_translation_nonzero W hΔ (-S) g.ne_zero)=
      functionDivisor W (generalTranslation W hΔ (-S) g')
        (general_translation_nonzero W hΔ (-S) g'.ne_zero)
    rw [general_translation_function_divisor W hΔ (-S) g.ne_zero,
      general_translation_function_divisor W hΔ (-S) g'.ne_zero,hg]
  have ha := geometric_degree_zero_choice W hΔ (pointRepresentative W P)
    (point_representative_degree W P) (generalTranslatedUnit W hΔ (-S) g)
    (generalTranslatedUnit W hΔ (-S) g') (point_evaluation_domain W hΔ P Q S f g h)
    (point_evaluation_domain W hΔ P Q S f' g' h') hgt
  have hb := geometric_degree_zero_choice W hΔ (shiftedRepresentative W S Q)
    (shifted_representative_degree W S Q) f f' (shifted_evaluation_domain W P Q S f g h)
    (shifted_evaluation_domain W P Q S f' g' h') hf
  unfold sourceAuxiliaryValue
  rw [ha,hb]

/-- Normalization is unnecessary: every actual valid three-torsion function choice agrees. -/
theorem arbitrary_three_functions_source [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P Q : ThreeTorsion W)
    (f g : W.FunctionFieldˣ)
    (hf : functionDivisor W f f.ne_zero=torsionDivisor W P.val 3)
    (hg : functionDivisor W g g.ne_zero=torsionDivisor W Q.val 3)
    (S : W.Point) (h : AuxiliaryAdmissible W P.val Q.val f g S) :
    sourceAuxiliaryValue W hΔ P.val Q.val S f g h=divisorWeilThree W hΔ P Q := by
  have hf' := hf.trans (chosen_unit_divisor W P.val 3 P.property).symm
  have hg' := hg.trans (chosen_unit_divisor W Q.val 3 Q.property).symm
  rw [source_function_choice W hΔ P.val Q.val S f g
    (chosenFunctionUnit W P.val 3) (chosenFunctionUnit W Q.val 3) hf' hg' h]
  exact divisor_weil_three_auxiliary_independent W hΔ P Q S _

#print axioms source_function_choice
#print axioms arbitrary_three_functions_source
end
end WeilAuxiliaryRatioAudit
