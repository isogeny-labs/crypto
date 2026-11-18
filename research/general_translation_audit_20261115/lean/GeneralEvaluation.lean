module
public import GeneralTransport

@[expose] public section
namespace GeneralTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit LocalTranslationAudit
open MillerFinitePointAudit FinitePointEvaluationAudit FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K] [DecidableEq K]

theorem general_translation_guarded_point_value (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (f : W.FunctionFieldˣ) :
    guardedPointValue W (generalTranslatedUnit W hΔ Q f) P=guardedPointValue W f (P+Q) := by
  classical
  have ho : pointOrder W (generalTranslatedUnit W hΔ Q f) P=pointOrder W f (P+Q) := by
    rw [general_translated_unit_coe,general_translation_order]
  by_cases hf : pointOrder W f (P+Q)=0
  · have ht := ho.trans hf
    simp only [guardedPointValue,dite_eq_left hf,dite_eq_left ht]
    exact general_translation_point_unit_evaluation W hΔ P Q ⟨f,(mem_point_units_iff W (P+Q) f).mpr hf⟩
  · have ht : pointOrder W (generalTranslatedUnit W hΔ Q f) P≠0 := by rwa [ho]
    simp only [guardedPointValue,dite_eq_right hf,dite_eq_right ht]

theorem general_translation_disjoint_support (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) (D : W.Point →₀ ℤ) (f : W.FunctionFieldˣ) :
    Disjoint D.support
      (functionDivisor W (generalTranslatedUnit W hΔ Q f) (generalTranslatedUnit W hΔ Q f).ne_zero).support ↔
    Disjoint (divisorTranslation W Q D).support (functionDivisor W f f.ne_zero).support := by
  rw [disjoint_principal_support_iff,disjoint_principal_support_iff,mem_support_units_iff,
    mem_support_units_iff]
  constructor
  · intro h R hR
    have hd : D (R-Q)≠0 := by
      simpa only [Finsupp.mem_support_iff,divisor_translation_apply] using hR
    have ho := h (R-Q) (Finsupp.mem_support_iff.mpr hd)
    rw [general_translated_unit_coe,general_translation_order,sub_add_cancel] at ho
    exact ho
  · intro h P hP
    rw [general_translated_unit_coe,general_translation_order]
    apply h (P+Q)
    simpa only [Finsupp.mem_support_iff,divisor_translation_apply,add_sub_cancel_right] using hP

/-- Actual guarded evaluation identity over every coefficient field, on actual rational points. -/
theorem general_translation_divisor_evaluation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) (D : W.Point →₀ ℤ) (f : W.FunctionFieldˣ)
    (hd : Disjoint (divisorTranslation W Q D).support (functionDivisor W f f.ne_zero).support) :
    evaluateDivisor W D (generalTranslatedUnit W hΔ Q f)
      ((general_translation_disjoint_support W hΔ Q D f).mpr hd)=
    evaluateDivisor W (divisorTranslation W Q D) f hd := by
  rw [evaluate_divisor_guarded_product,evaluate_divisor_guarded_product]
  change D.prod (fun P n => guardedPointValue W (generalTranslatedUnit W hΔ Q f) P^n)=
    (Finsupp.equivMapDomain (pointTranslation W Q) D).prod (fun P n => guardedPointValue W f P^n)
  rw [Finsupp.prod_equivMapDomain]
  apply Finsupp.prod_congr
  intro P _
  rw [general_translation_guarded_point_value]
  rfl

#print axioms general_translation_disjoint_support
#print axioms general_translation_divisor_evaluation
end
end GeneralTranslationAudit
