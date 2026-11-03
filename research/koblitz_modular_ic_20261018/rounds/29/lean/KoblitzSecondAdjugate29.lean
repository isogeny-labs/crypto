import Mathlib.Tactic.FieldSimp
import Mathlib.Tactic.Ring

/-! Second-order elementary symmetric recovery. Actual norm-specialization
and finite-cover geometry remain native and written, not axioms here. -/
namespace KoblitzSecondAdjugate29

theorem recover_pair_sum {K : Type*} [Field K] (c x y : K) (hc : c ≠ 0) :
    (c*(1+x)*(1+y))/c - 1 - (c*x*y)/c = x+y := by
  field_simp
  ring

theorem pair_polynomial {K : Type*} [CommRing K] (t x y : K) :
    (t-x)*(t-y) = t^2-(x+y)*t+x*y := by ring

end KoblitzSecondAdjugate29
