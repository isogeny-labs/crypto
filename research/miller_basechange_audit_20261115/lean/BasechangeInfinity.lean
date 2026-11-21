module
public import BasechangeNorm

@[expose] public section
namespace MillerBasechangeAudit
noncomputable section
open MillerNormValuationAudit MillerRecursionAudit
variable {K L : Type*} [Field K] [Field L] [Algebra K L]

theorem infinity_scaled_basechange (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    infinityScaled (extendedCurve (L := L) W) (fieldBaseChange (L := L) W f) =
      fieldBaseChange (L := L) W (infinityScaled W f) := by
  simp only [infinityScaled, infinity_order_basechange, map_mul, map_zpow₀, field_base_parameter]

theorem infinity_monic_basechange (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    InfinityMonic (extendedCurve (L := L) W) (fieldBaseChange (L := L) W f) ↔
      InfinityMonic W f := by
  unfold InfinityMonic
  rw [infinity_scaled_basechange]
  have he := (fieldBaseChange (L := L) W).map_sub (infinityScaled W f) 1
  rw [map_one] at he
  rw [← he, function_valuation_basechange]

#print axioms infinity_monic_basechange
end
end MillerBasechangeAudit
