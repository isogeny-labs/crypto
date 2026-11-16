module
public import TranslationTransport

@[expose] public section
namespace LocalTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit MillerFinitePointAudit FinitePointEvaluationAudit
open FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K]

/-- Product helper only. It agrees with actual evaluation on the actual local-unit domain;
the value outside that domain is not claimed to be a residue of a meromorphic function. -/
def guardedPointValue (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ) (P : W.Point) : Kˣ := by
  classical
  exact if h : pointOrder W f P=0 then
    pointUnitEvaluation W P ⟨f,(mem_point_units_iff W P f).mpr h⟩ else 1

theorem evaluate_divisor_guarded_product (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
    (f : W.FunctionFieldˣ) (hd : Disjoint D.support (functionDivisor W f f.ne_zero).support) :
    evaluateDivisor W D f hd=D.prod (fun P n => guardedPointValue W f P^n) := by
  classical
  rw [evaluateDivisor,divisorEvaluation,divisor_evaluation_on_apply]
  apply Finsupp.prod_congr
  intro P hP
  have hfP : pointOrder W f P=0 :=
    (mem_support_units_iff W D.support f).mp ((disjoint_principal_support_iff W f D).mp hd) P hP
  simp only [guardedPointValue,dite_eq_left hfP,supportPointEvaluation,dite_eq_left hP,
    MonoidHom.comp_apply]
  rfl

variable [DecidableEq K] [IsAlgClosed K]

theorem translation_guarded_point_value (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (f : W.FunctionFieldˣ) :
    guardedPointValue W (translatedUnit W hΔ Q f) P=guardedPointValue W f (P+Q) := by
  classical
  have ho : pointOrder W (translatedUnit W hΔ Q f) P=pointOrder W f (P+Q) := by
    rw [translated_unit_coe,translation_point_order]
  by_cases hf : pointOrder W f (P+Q)=0
  · have ht := ho.trans hf
    simp only [guardedPointValue,dite_eq_left hf,dite_eq_left ht]
    exact translation_point_unit_evaluation W hΔ P Q ⟨f,(mem_point_units_iff W (P+Q) f).mpr hf⟩
  · have ht : pointOrder W (translatedUnit W hΔ Q f) P≠0 := by rwa [ho]
    simp only [guardedPointValue,dite_eq_right hf,dite_eq_right ht]

/-- Actual support guard is transported by the actual translation, with no added premise. -/
theorem translation_disjoint_support (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) (D : W.Point →₀ ℤ) (f : W.FunctionFieldˣ) :
    Disjoint D.support
      (functionDivisor W (translatedUnit W hΔ Q f) (translatedUnit W hΔ Q f).ne_zero).support ↔
    Disjoint (divisorTranslation W Q D).support (functionDivisor W f f.ne_zero).support := by
  rw [disjoint_principal_support_iff,disjoint_principal_support_iff,mem_support_units_iff,
    mem_support_units_iff]
  constructor
  · intro h R hR
    have hd : D (R-Q)≠0 := by
      simpa only [Finsupp.mem_support_iff,divisor_translation_apply] using hR
    have ho := h (R-Q) (Finsupp.mem_support_iff.mpr hd)
    rw [translated_unit_coe,translation_point_order,sub_add_cancel] at ho
    exact ho
  · intro h P hP
    rw [translated_unit_coe,translation_point_order]
    apply h (P+Q)
    simpa only [Finsupp.mem_support_iff,divisor_translation_apply,add_sub_cancel_right] using hP

/-- Actual divisor evaluation transport, retaining and deriving the disjoint-support domains. -/
theorem translation_divisor_evaluation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) (D : W.Point →₀ ℤ) (f : W.FunctionFieldˣ)
    (hd : Disjoint (divisorTranslation W Q D).support (functionDivisor W f f.ne_zero).support) :
    evaluateDivisor W D (translatedUnit W hΔ Q f)
      ((translation_disjoint_support W hΔ Q D f).mpr hd)=
    evaluateDivisor W (divisorTranslation W Q D) f hd := by
  rw [evaluate_divisor_guarded_product,evaluate_divisor_guarded_product]
  change D.prod (fun P n => guardedPointValue W (translatedUnit W hΔ Q f) P^n)=
    (Finsupp.equivMapDomain (pointTranslation W Q) D).prod (fun P n => guardedPointValue W f P^n)
  rw [Finsupp.prod_equivMapDomain]
  apply Finsupp.prod_congr
  intro P _
  rw [translation_guarded_point_value]
  rfl

#print axioms translation_guarded_point_value
#print axioms translation_disjoint_support
#print axioms translation_divisor_evaluation
end
end LocalTranslationAudit
