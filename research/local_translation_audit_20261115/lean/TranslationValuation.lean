module
public import TranslationLocal

@[expose] public section
namespace LocalTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit MillerNormValuationAudit MillerFinitePointAudit
open FinitePointEvaluationAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

theorem infinity_translation_valuation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) (f : W.FunctionField) :
    functionValuation W (fieldTranslation W hΔ Q f)=localFunctionValuation W Q f := by
  cases Q with
  | zero =>
    have hz := field_translation_zero W hΔ
    rw [Point.zero_def] at hz
    rw [hz]
    rfl
  | some b c h => exact finite_infinity_translation_valuation W hΔ h f

/-- Actual normalized local valuations, for all points including every exceptional input. -/
theorem translation_valuation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (f : W.FunctionField) :
    localFunctionValuation W P (fieldTranslation W hΔ Q f)=localFunctionValuation W (P+Q) f := by
  have he := RingHom.congr_fun (field_translation_comp W hΔ P Q) f
  calc
    _ = functionValuation W (fieldTranslation W hΔ P (fieldTranslation W hΔ Q f)) :=
      (infinity_translation_valuation W hΔ P (fieldTranslation W hΔ Q f)).symm
    _ = functionValuation W (fieldTranslation W hΔ (P+Q) f) := congrArg (functionValuation W) he
    _ = _ := infinity_translation_valuation W hΔ (P+Q) f

theorem translation_point_order (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (f : W.FunctionField) :
    pointOrder W (fieldTranslation W hΔ Q f) P=pointOrder W f (P+Q) := by
  rw [point_order_formula,point_order_formula,translation_valuation]

/-- Actual pullback of regular functions, with actual preservation now derived. -/
def regularTranslation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P Q : W.Point) :
    RegularRing W (P+Q) →+* RegularRing W P where
  toFun r := ⟨fieldTranslation W hΔ Q r, by
    change localFunctionValuation W P (fieldTranslation W hΔ Q r)≤1
    rw [translation_valuation]
    exact r.property⟩
  map_zero' := Subtype.ext (map_zero _)
  map_one' := Subtype.ext (map_one _)
  map_add' a b := Subtype.ext (map_add (fieldTranslation W hΔ Q)
    (a : W.FunctionField) (b : W.FunctionField))
  map_mul' a b := Subtype.ext (map_mul (fieldTranslation W hΔ Q)
    (a : W.FunctionField) (b : W.FunctionField))

theorem regular_translation_coe (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (r : RegularRing W (P+Q)) :
    (regularTranslation W hΔ P Q r : W.FunctionField)=fieldTranslation W hΔ Q r := rfl

theorem regular_translation_constant (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (d : K) :
    regularTranslation W hΔ P Q (regularConstant W (P+Q) d)=regularConstant W P d :=
  Subtype.ext (field_translation_constant W hΔ Q d)

/-- Actual residue naturality, proved from actual maximal ideals and derived valuation equality. -/
theorem translation_residue (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (r : RegularRing W (P+Q)) :
    regularEvaluation W P (regularTranslation W hΔ P Q r)=regularEvaluation W (P+Q) r := by
  let a := r-regularConstant W (P+Q) (regularEvaluation W (P+Q) r)
  have hz : regularEvaluation W (P+Q) a=0 := by
    simp only [a,map_sub,regular_eval_constant,sub_self]
  have hm : a ∈ IsLocalRing.maximalIdeal (RegularRing W (P+Q)) := by
    rw [← regular_evaluation_kernel]
    exact hz
  have hs : localFunctionValuation W (P+Q) (a : W.FunctionField)<1 :=
    (Valuation.mem_maximalIdeal_iff (K := W.FunctionField) (v := localFunctionValuation W (P+Q))).mp hm
  have ht : localFunctionValuation W P (regularTranslation W hΔ P Q a : W.FunctionField)<1 := by
    rw [regular_translation_coe,translation_valuation]
    exact hs
  have hm' : regularTranslation W hΔ P Q a ∈ IsLocalRing.maximalIdeal (RegularRing W P) :=
    (Valuation.mem_maximalIdeal_iff (K := W.FunctionField) (v := localFunctionValuation W P)).mpr ht
  have hz' : regularEvaluation W P (regularTranslation W hΔ P Q a)=0 := by
    change regularTranslation W hΔ P Q a ∈ RingHom.ker (regularEvaluation W P)
    rw [regular_evaluation_kernel]
    exact hm'
  simpa only [a,map_sub,regular_translation_constant,regular_eval_constant,sub_eq_zero] using hz'

#print axioms translation_valuation
#print axioms translation_point_order
#print axioms translation_residue
end
end LocalTranslationAudit
