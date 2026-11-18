module
public import GeneralLocal

@[expose] public section
namespace GeneralTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerBasechangeAudit FinitePointEvaluationAudit
variable {K L : Type*} [Field K] [Field L] [Algebra K L] [DecidableEq K] [DecidableEq L]

/-- Actual regular functions map to actual regular functions at the image rational point. -/
def regularBasechange (W : WeierstrassCurve.Affine K) (P : W.Point) :
    RegularRing W P →+* RegularRing (extendedCurve (L := L) W) (pointBaseChange (L := L) W P) where
  toFun r := ⟨fieldBaseChange (L := L) W r, by
    change localFunctionValuation (extendedCurve (L := L) W) (pointBaseChange (L := L) W P)
      (fieldBaseChange (L := L) W r)≤1
    rw [local_valuation_basechange]
    exact r.property⟩
  map_zero' := Subtype.ext (map_zero _)
  map_one' := Subtype.ext (map_one _)
  map_add' a b := Subtype.ext (map_add (fieldBaseChange (L := L) W)
    (a : W.FunctionField) (b : W.FunctionField))
  map_mul' a b := Subtype.ext (map_mul (fieldBaseChange (L := L) W)
    (a : W.FunctionField) (b : W.FunctionField))

theorem regular_basechange_coe (W : WeierstrassCurve.Affine K) (P : W.Point) (r : RegularRing W P) :
    (regularBasechange (L := L) W P r : (extendedCurve (L := L) W).FunctionField)=
      fieldBaseChange (L := L) W r := rfl

theorem regular_basechange_injective (W : WeierstrassCurve.Affine K) (P : W.Point) :
    Function.Injective (regularBasechange (L := L) W P) := by
  intro a b he
  exact Subtype.ext (field_base_injective (L := L) W (congrArg Subtype.val he))

theorem regular_basechange_constant (W : WeierstrassCurve.Affine K) (P : W.Point) (d : K) :
    regularBasechange (L := L) W P (regularConstant W P d)=
      regularConstant (extendedCurve (L := L) W) (pointBaseChange (L := L) W P) (algebraMap K L d) :=
  Subtype.ext (field_base_constant W d)

/-- Actual residue naturality at every actual image rational point, including infinity. -/
theorem regular_basechange_residue (W : WeierstrassCurve.Affine K) (P : W.Point)
    (r : RegularRing W P) :
    regularEvaluation (extendedCurve (L := L) W) (pointBaseChange (L := L) W P)
      (regularBasechange (L := L) W P r)=algebraMap K L (regularEvaluation W P r) := by
  let a := r-regularConstant W P (regularEvaluation W P r)
  have hz : regularEvaluation W P a=0 := by simp only [a,map_sub,regular_eval_constant,sub_self]
  have hm : a ∈ IsLocalRing.maximalIdeal (RegularRing W P) := by
    rw [← regular_evaluation_kernel]
    exact hz
  have hs : localFunctionValuation W P (a : W.FunctionField)<1 :=
    (Valuation.mem_maximalIdeal_iff (K := W.FunctionField) (v := localFunctionValuation W P)).mp hm
  have ht : localFunctionValuation (extendedCurve (L := L) W) (pointBaseChange (L := L) W P)
      (regularBasechange (L := L) W P a : (extendedCurve (L := L) W).FunctionField)<1 := by
    rw [regular_basechange_coe,local_valuation_basechange]
    exact hs
  have hm' : regularBasechange (L := L) W P a ∈ IsLocalRing.maximalIdeal
      (RegularRing (extendedCurve (L := L) W) (pointBaseChange (L := L) W P)) :=
    (Valuation.mem_maximalIdeal_iff (K := (extendedCurve (L := L) W).FunctionField)
      (v := localFunctionValuation (extendedCurve (L := L) W) (pointBaseChange (L := L) W P))).mpr ht
  have hz' : regularEvaluation (extendedCurve (L := L) W) (pointBaseChange (L := L) W P)
      (regularBasechange (L := L) W P a)=0 := by
    change regularBasechange (L := L) W P a ∈ RingHom.ker
      (regularEvaluation (extendedCurve (L := L) W) (pointBaseChange (L := L) W P))
    rw [regular_evaluation_kernel]
    exact hm'
  simpa only [a,map_sub,regular_basechange_constant,regular_eval_constant,sub_eq_zero] using hz'

#print axioms regular_basechange_injective
#print axioms regular_basechange_residue
end
end GeneralTranslationAudit
