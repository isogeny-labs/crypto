import AuxiliaryScalar

namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit GeneralTranslationAudit EllipticTranslationAudit
open LocalTranslationAudit
variable {K : Type*} [Field K] [DecidableEq K]

def pointRepresentative (W : WeierstrassCurve.Affine K) (P : W.Point) : W.Point →₀ ℤ :=
  Finsupp.single P 1-Finsupp.single 0 1

def shiftedRepresentative (W : WeierstrassCurve.Affine K) (S Q : W.Point) : W.Point →₀ ℤ :=
  Finsupp.single (Q+S) 1-Finsupp.single S 1

theorem shifted_representative_degree (W : WeierstrassCurve.Affine K) (S Q : W.Point) :
    divisorDegree W (shiftedRepresentative W S Q)=0 := by
  rw [shiftedRepresentative,map_sub,degree_single,degree_single,sub_self]

omit [DecidableEq K] in
theorem point_representative_degree (W : WeierstrassCurve.Affine K) (P : W.Point) :
    divisorDegree W (pointRepresentative W P)=0 := by
  rw [pointRepresentative,map_sub,degree_single,degree_single,sub_self]

theorem shifted_representative_translation (W : WeierstrassCurve.Affine K) (S Q : W.Point) :
    divisorTranslation W S (pointRepresentative W Q)=shiftedRepresentative W S Q := by
  rw [pointRepresentative,map_sub,divisor_translation_single,divisor_translation_single,zero_add]
  rfl

/-- The translated representative differs by the actual divisor of an explicit function. -/
theorem shifted_representative_correction (W : WeierstrassCurve.Affine K) (S Q : W.Point) :
    functionDivisor W (1/stepFunction W S Q) (div_ne_zero one_ne_zero (step_function_nonzero W S Q))=
      shiftedRepresentative W S Q-pointRepresentative W Q := by
  have hd : functionDivisor W (stepFunction W S Q) (step_function_nonzero W S Q)=
      additionDivisor W S Q := by
    ext R
    exact (step_has_point_divisor W S Q).2 R
  rw [function_divisor_div W one_ne_zero (step_function_nonzero W S Q),function_divisor_one,hd]
  unfold shiftedRepresentative pointRepresentative additionDivisor
  rw [add_comm S Q]
  abel

theorem shifted_function_divisor (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q S : W.Point) (n : ℕ) (g : W.FunctionFieldˣ)
    (hg : functionDivisor W g g.ne_zero=torsionDivisor W Q n) :
    functionDivisor W (generalTranslatedUnit W hΔ (-S) g)
      (generalTranslatedUnit W hΔ (-S) g).ne_zero=(n : ℤ) • shiftedRepresentative W S Q := by
  change functionDivisor W (generalTranslation W hΔ (-S) g)
    (general_translation_nonzero W hΔ (-S) g.ne_zero)=_
  rw [general_translation_function_divisor W hΔ (-S) g.ne_zero,neg_neg,hg]
  simp only [torsionDivisor,map_sub,map_zsmul,divisor_translation_single,zero_add,
    shiftedRepresentative,smul_sub]

omit [DecidableEq K] in
/-- Every actual point-difference evaluation reduces to its two genuine local-unit values. -/
theorem evaluate_point_difference (W : WeierstrassCurve.Affine K) (A B : W.Point)
    (f : W.FunctionFieldˣ) (hA : pointOrder W f A=0) (hB : pointOrder W f B=0)
    (hd : Disjoint (Finsupp.single A (1 : ℤ)-Finsupp.single B 1).support
      (functionDivisor W f f.ne_zero).support) :
    evaluateDivisor W (Finsupp.single A 1-Finsupp.single B 1) f hd=
      localUnitValue W A f hA/localUnitValue W B f hB := by
  rw [evaluate_divisor_guarded_product,sub_eq_add_neg]
  rw [Finsupp.prod_add_index' (fun _ => zpow_zero _) (fun _ _ _ => zpow_add _ _ _),
    Finsupp.prod_neg_index (fun _ => zpow_zero _)]
  have hVA : guardedPointValue W f A=localUnitValue W A f hA := by
    simp only [guardedPointValue,dite_eq_left hA]
    rfl
  have hVB : guardedPointValue W f B=localUnitValue W B f hB := by
    simp only [guardedPointValue,dite_eq_left hB]
    rfl
  simp [Finsupp.prod_single_index,hVA,hVB,div_eq_mul_inv]

/-- Actual support guard for evaluating f on the translated representative. -/
theorem shifted_evaluation_domain (W : WeierstrassCurve.Affine K) (P Q S : W.Point)
    (f g : W.FunctionFieldˣ) (h : AuxiliaryAdmissible W P Q f g S) :
    Disjoint (shiftedRepresentative W S Q).support (functionDivisor W f f.ne_zero).support := by
  apply (disjoint_principal_support_iff W f _).mpr
  apply (mem_support_units_iff W _ f).mpr
  intro R hR
  by_cases h1 : R=Q+S
  · subst R
    simpa only [add_comm] using h.1
  by_cases h2 : R=S
  · subst R
    exact h.2.2.1
  have hz : shiftedRepresentative W S Q R=0 := by
    simp [shiftedRepresentative,Ne.symm h1,Ne.symm h2]
  exact False.elim ((Finsupp.mem_support_iff.mp hR) hz)

/-- Actual support guard for evaluating the translated g on (P)-(O). -/
theorem point_evaluation_domain (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q S : W.Point) (f g : W.FunctionFieldˣ) (h : AuxiliaryAdmissible W P Q f g S) :
    Disjoint (pointRepresentative W P).support
      (functionDivisor W (generalTranslatedUnit W hΔ (-S) g)
        (generalTranslatedUnit W hΔ (-S) g).ne_zero).support := by
  apply (disjoint_principal_support_iff W _ _).mpr
  apply (mem_support_units_iff W _ _).mpr
  intro R hR
  rw [general_translated_unit_coe,general_translation_order]
  by_cases h1 : R=P
  · subst R
    simpa only [sub_eq_add_neg] using h.2.2.2
  by_cases h2 : R=0
  · subst R
    simpa only [zero_add] using h.2.1
  have hz : pointRepresentative W P R=0 := by
    simp [pointRepresentative,Ne.symm h1,Ne.symm h2]
  exact False.elim ((Finsupp.mem_support_iff.mp hR) hz)

#print axioms shifted_representative_correction
#print axioms shifted_function_divisor
#print axioms evaluate_point_difference
#print axioms shifted_evaluation_domain
#print axioms point_evaluation_domain
end
end WeilAuxiliaryRatioAudit
