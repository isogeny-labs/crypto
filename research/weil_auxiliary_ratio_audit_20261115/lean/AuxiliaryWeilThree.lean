import AuxiliaryComparison

namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open GeneralTranslationAudit ThreeTorsionRankAudit.General ThreeTangentPairingAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

/-- An actual admissible point, obtained from the proved finite-avoidance construction. -/
def sourcePairingPoint (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : ThreeTorsion W) : W.Point :=
  Classical.choose (exists_auxiliary_admissible W hΔ P.val Q.val
    (chosenFunctionUnit W P.val 3) (chosenFunctionUnit W Q.val 3))

theorem source_pairing_point_admissible (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : ThreeTorsion W) :
    AuxiliaryAdmissible W P.val Q.val (chosenFunctionUnit W P.val 3) (chosenFunctionUnit W Q.val 3)
      (sourcePairingPoint W hΔ P Q) :=
  Classical.choose_spec (exists_auxiliary_admissible W hΔ P.val Q.val
    (chosenFunctionUnit W P.val 3) (chosenFunctionUnit W Q.val 3))

/-- Actual Galbraith26.1 divisor-formula value on actual geometric three-torsion.
Comparison to the multiplication-isogeny definition is a separate obligation. -/
def divisorWeilThree (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : ThreeTorsion W) : Kˣ :=
  sourceAuxiliaryValue W hΔ P.val Q.val (sourcePairingPoint W hΔ P Q)
    (chosenFunctionUnit W P.val 3) (chosenFunctionUnit W Q.val 3)
    (source_pairing_point_admissible W hΔ P Q)

theorem divisor_weil_three_value (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : ThreeTorsion W) :
    (divisorWeilThree W hΔ P Q : K)=(tangentPair W P.val Q.val)⁻¹ :=
  chosen_source_inverse W hΔ P.val Q.val P.property Q.property _ _

/-- Every admissible actual source representative has this same constructed value. -/
theorem divisor_weil_three_auxiliary_independent
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P Q : ThreeTorsion W) (S : W.Point)
    (h : AuxiliaryAdmissible W P.val Q.val (chosenFunctionUnit W P.val 3) (chosenFunctionUnit W Q.val 3) S) :
    sourceAuxiliaryValue W hΔ P.val Q.val S (chosenFunctionUnit W P.val 3) (chosenFunctionUnit W Q.val 3) h=
      divisorWeilThree W hΔ P Q := by
  apply Units.ext
  rw [divisor_weil_three_value,chosen_source_inverse W hΔ P.val Q.val P.property Q.property]

/-- Actual degree-zero, disjointness and the two actual principal-divisor equations. -/
theorem divisor_weil_three_representatives (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : ThreeTorsion W) :
    divisorDegree W (pointRepresentative W P.val)=0 ∧
    divisorDegree W (shiftedRepresentative W (sourcePairingPoint W hΔ P Q) Q.val)=0 ∧
    Disjoint (pointRepresentative W P.val).support
      (shiftedRepresentative W (sourcePairingPoint W hΔ P Q) Q.val).support ∧
    functionDivisor W (chosenFunctionUnit W P.val 3) (chosenFunctionUnit W P.val 3).ne_zero=
      (3 : ℤ) • pointRepresentative W P.val ∧
    functionDivisor W (generalTranslatedUnit W hΔ (-(sourcePairingPoint W hΔ P Q))
        (chosenFunctionUnit W Q.val 3))
        (generalTranslatedUnit W hΔ (-(sourcePairingPoint W hΔ P Q)) (chosenFunctionUnit W Q.val 3)).ne_zero=
      (3 : ℤ) • shiftedRepresentative W (sourcePairingPoint W hΔ P Q) Q.val := by
  refine ⟨point_representative_degree W P.val,shifted_representative_degree W _ Q.val,?_,?_,?_⟩
  · exact admissible_representatives_disjoint W P.val Q.val _ 3 (by decide) _ _
      (chosen_unit_divisor W P.val 3 P.property) (source_pairing_point_admissible W hΔ P Q)
  · rw [chosen_unit_divisor W P.val 3 P.property]
    norm_num [torsionDivisor,pointRepresentative,smul_sub]
  · exact shifted_function_divisor W hΔ Q.val _ 3 _ (chosen_unit_divisor W Q.val 3 Q.property)

theorem divisor_weil_three_add_left [NeZero (3 : K)] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q R : ThreeTorsion W) :
    divisorWeilThree W hΔ (P+Q) R=divisorWeilThree W hΔ P R*divisorWeilThree W hΔ Q R := by
  apply Units.ext
  simp only [Units.val_mul,divisor_weil_three_value]
  rw [tangentPair_add_left W hΔ,mul_inv]

theorem divisor_weil_three_add_right [NeZero (3 : K)] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q R : ThreeTorsion W) :
    divisorWeilThree W hΔ P (Q+R)=divisorWeilThree W hΔ P Q*divisorWeilThree W hΔ P R := by
  apply Units.ext
  simp only [Units.val_mul,divisor_weil_three_value]
  rw [tangentPair_add_right W hΔ,mul_inv]

theorem divisor_weil_three_alternating (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P : ThreeTorsion W) : divisorWeilThree W hΔ P P=1 := by
  apply Units.ext
  rw [divisor_weil_three_value,tangentPair_self,inv_one]
  rfl

theorem divisor_weil_three_cube (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : ThreeTorsion W) : (divisorWeilThree W hΔ P Q)^3=1 := by
  apply Units.ext
  change (divisorWeilThree W hΔ P Q : K)^3=1
  rw [divisor_weil_three_value,inv_pow,tangentPair_cube W hΔ P Q,inv_one]

theorem divisor_weil_three_left_nondegenerate [NeZero (3 : K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P : ThreeTorsion W)
    (h : ∀ Q : ThreeTorsion W,divisorWeilThree W hΔ P Q=1) : P=0 := by
  apply tangentPair_left_nondegenerate W hΔ P
  intro Q
  have he := congrArg (fun z : Kˣ => (z : K)) (h Q)
  rw [divisor_weil_three_value,Units.val_one] at he
  exact inv_eq_one.mp he

theorem divisor_weil_three_right_nondegenerate [NeZero (3 : K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (Q : ThreeTorsion W)
    (h : ∀ P : ThreeTorsion W,divisorWeilThree W hΔ P Q=1) : Q=0 := by
  apply tangentPair_right_nondegenerate W hΔ Q
  intro P
  have he := congrArg (fun z : Kˣ => (z : K)) (h P)
  rw [divisor_weil_three_value,Units.val_one] at he
  exact inv_eq_one.mp he

#print axioms divisor_weil_three_value
#print axioms divisor_weil_three_representatives
#print axioms divisor_weil_three_add_left
#print axioms divisor_weil_three_add_right
#print axioms divisor_weil_three_alternating
#print axioms divisor_weil_three_cube
#print axioms divisor_weil_three_left_nondegenerate
#print axioms divisor_weil_three_right_nondegenerate
end
end WeilAuxiliaryRatioAudit
