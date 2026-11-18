module
public import GeneralValuationBasechange

@[expose] public section
namespace GeneralTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit MillerFinitePointAudit MillerBasechangeAudit
open FinitePointEvaluationAudit
variable {K : Type*} [Field K] [DecidableEq K]

/-- Actual all-point normalized translation compatibility over every coefficient field.
An actual algebraic closure is used in the proof, not assumed of the coefficient field. -/
theorem general_translation_valuation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (f : W.FunctionField) :
    localFunctionValuation W P (generalTranslation W hΔ Q f)=localFunctionValuation W (P+Q) f := by
  let : DecidableEq (AlgebraicClosure K) := Classical.decEq _
  have he := RingHom.congr_fun (general_translation_basechange (L := AlgebraicClosure K) W hΔ Q) f
  simp only [RingHom.comp_apply] at he
  calc
    _ = localFunctionValuation (extendedCurve (L := AlgebraicClosure K) W)
        (pointBaseChange (L := AlgebraicClosure K) W P)
        (fieldBaseChange (L := AlgebraicClosure K) W (generalTranslation W hΔ Q f)) :=
      (local_valuation_basechange W P (generalTranslation W hΔ Q f)).symm
    _ = localFunctionValuation (extendedCurve (L := AlgebraicClosure K) W)
        (pointBaseChange (L := AlgebraicClosure K) W P)
        (fieldTranslation (extendedCurve (L := AlgebraicClosure K) W) (extended_delta_nonzero W hΔ)
          (pointBaseChange (L := AlgebraicClosure K) W Q) (fieldBaseChange (L := AlgebraicClosure K) W f)) := by
      rw [he,general_translation_agrees]
    _ = localFunctionValuation (extendedCurve (L := AlgebraicClosure K) W)
        (pointBaseChange (L := AlgebraicClosure K) W P+pointBaseChange (L := AlgebraicClosure K) W Q)
        (fieldBaseChange (L := AlgebraicClosure K) W f) :=
      LocalTranslationAudit.translation_valuation _ _ _ _ _
    _ = localFunctionValuation (extendedCurve (L := AlgebraicClosure K) W)
        (pointBaseChange (L := AlgebraicClosure K) W (P+Q))
        (fieldBaseChange (L := AlgebraicClosure K) W f) := by rw [map_add]
    _ = _ := local_valuation_basechange W (P+Q) f

theorem general_translation_order (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (f : W.FunctionField) :
    pointOrder W (generalTranslation W hΔ Q f) P=pointOrder W f (P+Q) := by
  rw [point_order_formula,point_order_formula,general_translation_valuation]

def generalRegularTranslation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P Q : W.Point) :
    RegularRing W (P+Q) →+* RegularRing W P where
  toFun r := ⟨generalTranslation W hΔ Q r, by
    change localFunctionValuation W P (generalTranslation W hΔ Q r)≤1
    rw [general_translation_valuation]
    exact r.property⟩
  map_zero' := Subtype.ext (map_zero _)
  map_one' := Subtype.ext (map_one _)
  map_add' a b := Subtype.ext (map_add (generalTranslation W hΔ Q)
    (a : W.FunctionField) (b : W.FunctionField))
  map_mul' a b := Subtype.ext (map_mul (generalTranslation W hΔ Q)
    (a : W.FunctionField) (b : W.FunctionField))

theorem general_regular_translation_coe (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (r : RegularRing W (P+Q)) :
    (generalRegularTranslation W hΔ P Q r : W.FunctionField)=generalTranslation W hΔ Q r := rfl

theorem general_regular_translation_constant (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (d : K) :
    generalRegularTranslation W hΔ P Q (regularConstant W (P+Q) d)=regularConstant W P d :=
  Subtype.ext (general_translation_constant W hΔ Q d)

/-- Actual regular residue naturality over every field, derived from actual maximal ideals. -/
theorem general_translation_residue (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (r : RegularRing W (P+Q)) :
    regularEvaluation W P (generalRegularTranslation W hΔ P Q r)=regularEvaluation W (P+Q) r := by
  let a := r-regularConstant W (P+Q) (regularEvaluation W (P+Q) r)
  have hz : regularEvaluation W (P+Q) a=0 := by
    simp only [a,map_sub,regular_eval_constant,sub_self]
  have hm : a ∈ IsLocalRing.maximalIdeal (RegularRing W (P+Q)) := by
    rw [← regular_evaluation_kernel]
    exact hz
  have hs : localFunctionValuation W (P+Q) (a : W.FunctionField)<1 :=
    (Valuation.mem_maximalIdeal_iff (K := W.FunctionField) (v := localFunctionValuation W (P+Q))).mp hm
  have ht : localFunctionValuation W P (generalRegularTranslation W hΔ P Q a : W.FunctionField)<1 := by
    rw [general_regular_translation_coe,general_translation_valuation]
    exact hs
  have hm' : generalRegularTranslation W hΔ P Q a ∈ IsLocalRing.maximalIdeal (RegularRing W P) :=
    (Valuation.mem_maximalIdeal_iff (K := W.FunctionField) (v := localFunctionValuation W P)).mpr ht
  have hz' : regularEvaluation W P (generalRegularTranslation W hΔ P Q a)=0 := by
    change generalRegularTranslation W hΔ P Q a ∈ RingHom.ker (regularEvaluation W P)
    rw [regular_evaluation_kernel]
    exact hm'
  simpa only [a,map_sub,general_regular_translation_constant,regular_eval_constant,sub_eq_zero] using hz'

#print axioms general_translation_valuation
#print axioms general_translation_order
#print axioms general_translation_residue
end
end GeneralTranslationAudit
