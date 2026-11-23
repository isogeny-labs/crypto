import AuxiliaryLocal

namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open NegationPullbackAudit FinitePointEvaluationAudit GeneralTranslationAudit
variable {K : Type*} [Field K] [DecidableEq K]

/-- The four actual local-unit conditions needed for the auxiliary quotient. -/
def AuxiliaryAdmissible (W : WeierstrassCurve.Affine K) (P Q : W.Point)
    (f g : W.FunctionFieldˣ) (S : W.Point) : Prop :=
  pointOrder W f (S+Q)=0 ∧ pointOrder W g (-S)=0 ∧
    pointOrder W f S=0 ∧ pointOrder W g (P-S)=0

/-- Actual values at actual unit domains, with no value assigned at a pole or zero. -/
def auxiliaryValue (W : WeierstrassCurve.Affine K) (P Q : W.Point)
    (f g : W.FunctionFieldˣ) (S : W.Point) (h : AuxiliaryAdmissible W P Q f g S) : Kˣ :=
  localUnitValue W (S+Q) f h.1 * localUnitValue W (-S) g h.2.1 /
    (localUnitValue W S f h.2.2.1 * localUnitValue W (P-S) g h.2.2.2)

/-- Evaluate the actual scalar field identity in the actual regular ring at S. -/
theorem auxiliary_value_of_scalar (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (f g : W.FunctionFieldˣ) (S : W.Point)
    (h : AuxiliaryAdmissible W P Q f g S) (c : K) (hc : c≠0)
    (he : auxiliaryQuotient W hΔ P Q f g=constantHom W c) :
    auxiliaryValue W P Q f g S h=Units.mk0 c hc := by
  let a : (RegularRing W S).unitGroup :=
    generalPointUnitTranslation W hΔ S Q ⟨f,(mem_point_units_iff W (S+Q) f).mpr h.1⟩
  let b : (RegularRing W S).unitGroup :=
    pointUnitNegation W S ⟨g,(mem_point_units_iff W (-S) g).mpr h.2.1⟩
  let d : (RegularRing W S).unitGroup := ⟨f,(mem_point_units_iff W S f).mpr h.2.2.1⟩
  have hs : -(S+ -P)=P-S := by abel
  have ho : pointOrder W (negatedUnit W g) (S+ -P)=0 := by
    change pointOrder W (fieldNeg W (g : W.FunctionField)) (S+ -P)=0
    rw [point_order_pullback,hs]
    exact h.2.2.2
  let e : (RegularRing W S).unitGroup :=
    generalPointUnitTranslation W hΔ S (-P)
      ⟨negatedUnit W g,(mem_point_units_iff W (S+ -P) (negatedUnit W g)).mpr ho⟩
  let u : (RegularRing W S).unitGroup := (a*b)/(d*e)
  have hu : (((RegularRing W S).unitGroupMulEquiv u : RegularRing W S) : W.FunctionField)=
      constantHom W c := by
    change ((u : W.FunctionFieldˣ) : W.FunctionField)=constantHom W c
    simp only [u,Subgroup.coe_div,Subgroup.coe_mul,Units.val_div_eq_div_val,Units.val_mul]
    exact he
  have hv : pointUnitEvaluation W S u=Units.mk0 c hc := by
    apply Units.ext
    have hr : ((RegularRing W S).unitGroupMulEquiv u : RegularRing W S)=regularConstant W S c :=
      Subtype.ext hu
    change regularEvaluation W S ((RegularRing W S).unitGroupMulEquiv u : RegularRing W S)=c
    rw [hr,regular_eval_constant]
  have ha : pointUnitEvaluation W S a=localUnitValue W (S+Q) f h.1 :=
    general_translation_point_unit_evaluation W hΔ S Q _
  have hb : pointUnitEvaluation W S b=localUnitValue W (-S) g h.2.1 :=
    point_unit_negation_value W S _
  have hd : pointUnitEvaluation W S d=localUnitValue W S f h.2.2.1 := rfl
  have hh : pointUnitEvaluation W S e=localUnitValue W (P-S) g h.2.2.2 := by
    change localUnitValue W S (generalTranslatedUnit W hΔ (-P) (negatedUnit W g)) (by
      rw [general_translated_unit_coe,general_translation_order];exact ho)=_
    rw [local_unit_value_translation W hΔ S (-P) (negatedUnit W g) ho]
    simpa only [hs] using local_unit_value_negation W (S+ -P) g
      (by rw [hs];exact h.2.2.2)
  rw [show u=(a*b)/(d*e) from rfl,map_div,map_mul,map_mul,ha,hb,hd,hh] at hv
  exact hv

/-- Every-n actual chosen Miller auxiliary value is independent of every admissible S. -/
theorem chosen_auxiliary_independent [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P Q : W.Point) (n : ℕ)
    (hP : n • P=0) (hQ : n • Q=0) :
    ∃ c : Kˣ, ∀ (S : W.Point)
      (h : AuxiliaryAdmissible W P Q (Units.mk0 (millerFunction W P n) (miller_function_nonzero W P n))
        (Units.mk0 (millerFunction W Q n) (miller_function_nonzero W Q n)) S),
      auxiliaryValue W P Q (Units.mk0 (millerFunction W P n) (miller_function_nonzero W P n))
        (Units.mk0 (millerFunction W Q n) (miller_function_nonzero W Q n)) S h=c := by
  obtain ⟨c,hc,he⟩ := chosen_auxiliary_scalar W hΔ P Q n hP hQ
  exact ⟨Units.mk0 c hc,fun S h => auxiliary_value_of_scalar W hΔ P Q _ _ S h c hc he⟩

#print axioms auxiliary_value_of_scalar
#print axioms chosen_auxiliary_independent
end
end WeilAuxiliaryRatioAudit
