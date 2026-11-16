import TateMillerImports

namespace TateMillerEvaluationAudit
noncomputable section
open WeierstrassCurve.Affine MillerRecursionAudit MillerNormValuationAudit
open MillerBasechangeAudit WeilAuxiliaryRatioAudit
variable {K : Type*} [Field K] [DecidableEq K]

/-- Descend the actual geometric identity; rational-only divisor uniqueness is not used. -/
theorem actual_chosen_three_tangent (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {x y : K} (hP : W.Nonsingular x y) (h3 : (3 : ℕ) • Point.some x y hP=0) :
    millerFunction W (.some x y hP) 3=lineFunction W x y (W.slope x x y y) := by
  let : DecidableEq (AlgebraicClosure K):=Classical.decEq _
  let V:=extendedCurve (L:=AlgebraicClosure K) W
  have hp:=base_nonsingular (L:=AlgebraicClosure K) W hP
  have ht : (3 : ℕ) • Point.some (algebraMap K (AlgebraicClosure K) x)
      (algebraMap K (AlgebraicClosure K) y) hp=0 := by
    have hh:=congrArg (pointBaseChange (L:=AlgebraicClosure K) W) h3
    simpa only [map_nsmul,map_zero,point_base_some] using hh
  apply field_base_injective (L:=AlgebraicClosure K) W
  rw [miller_function_basechange,point_base_some,field_base_line,
    ←WeierstrassCurve.Affine.map_slope]
  have he:=chosen_three_tangent V (extended_delta_nonzero W hΔ) hp ht
  change millerFunction V (.some _ _ hp) 3=tangentFunction V _ _ at he
  rw [he,tangentFunction,tangent_slope_agrees V hp ht]

def shortLine (W : WeierstrassCurve.Affine K) (x y u v : K) : K:=
  v-y-((3*x^2+W.a₄)/(2*y))*(u-x)

#print axioms actual_chosen_three_tangent
end
end TateMillerEvaluationAudit
