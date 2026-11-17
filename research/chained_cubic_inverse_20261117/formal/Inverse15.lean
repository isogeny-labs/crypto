import Std.Tactic
import Init.Grind.Ring.Field

/-! Conditional field algebra for the guarded inverse. No Rust Field instance,
Euclidean-code correctness or Jacobian/Picard structure is constructed here. -/
set_option autoImplicit false

namespace ChainedCubicInverse15
open Lean.Grind
universe u

def norm {K : Type u} [CommRing K] (a b nu : K) : K :=
  a*a - nu*(b*b)

theorem norm_cubic {K : Type u} [CommRing K] (a nu : K) :
    norm a 0 nu = a*a := by
  unfold norm
  grind

theorem inverse_by_square {K : Type u} [Field K] (a : K) (ha : a ≠ 0) :
    a * (a*a)⁻¹ = a⁻¹ := by
  rw [Field.inv_mul, ← Semiring.mul_assoc, Field.mul_inv_cancel ha, Semiring.one_mul]

theorem cubic_inverse_pair {K : Type u} [Field K] (a nu : K) (ha : a ≠ 0) :
    (a * (norm a 0 nu)⁻¹, (-0 : K) * (norm a 0 nu)⁻¹) = (a⁻¹, 0) := by
  apply Prod.ext
  · rw [norm_cubic, inverse_by_square a ha]
  · rw [AddCommGroup.neg_zero, Semiring.zero_mul]

end ChainedCubicInverse15

#print axioms ChainedCubicInverse15.norm_cubic
#print axioms ChainedCubicInverse15.inverse_by_square
#print axioms ChainedCubicInverse15.cubic_inverse_pair
