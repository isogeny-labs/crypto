import Init.Grind.Module.Basic

/-!
The executable field census and Magma cover maps are external certificates.
This file checks the integer consequences of the cubic product-family theorem
and the abstract prime-subgroup transport step. It does not define finite
fields, elliptic curves, or the geometric Hess map.
-/

set_option autoImplicit false

namespace GhsHessCubicBounds

open Lean.Grind

theorem genus_three_from_double_trace_zero :
    2^2 - 2^(2-2) - 2^(2-2) + 1 = 3 := by decide

theorem q2_exact_count : (2-1)*(2+1)*(2+2)/2 = 6 := by decide
theorem q4_exact_count : (4-1)*(4+1)*(4+2)/2 = 45 := by decide
theorem q8_exact_count : (8-1)*(8+1)*(8+2)/2 = 315 := by decide
theorem q16_exact_count : (16-1)*(16+1)*(16+2)/2 = 2295 := by decide
theorem q8_repeated_factor_count : (8-1)*(8+1) = 63 := by decide
theorem q8_distinct_factor_count : (8-1)*8*(8+1)/2 = 252 := by decide
theorem q8_partition : 63 + 252 = 315 := by decide
theorem q16_direct_intersection_count : 16^2-1 = 255 := by decide
theorem q16_outside_direct_count : (16-1)*16*(16+1)/2 = 2040 := by decide
theorem q16_partition : 255+2040 = 2295 := by decide
theorem q8_source_model_count : 511 = 8^3-1 := by decide
theorem q8_outside_direct_order_closure : 126 + 189 = 315 := by decide
theorem q8_outside_mod8_partition : 45 + 144 = 189 := by decide
theorem q8_order_four_mod8_count : 144 = 9+18+45+18+36+9+9 := by decide
theorem order_548 : 4*137 = 548 := by decide
theorem order_556 : 4*139 = 556 := by decide
theorem order_548_mod8 : 548 % 8 = 4 := by decide
theorem order_556_mod8 : 556 % 8 = 4 := by decide
theorem cofactor_137 : Nat.Coprime 4 137 := by decide
theorem cofactor_139 : Nat.Coprime 4 139 := by decide

variable {E J : Type} [IntModule E] [IntModule J]

structure AdditiveTransfer (E J : Type) [IntModule E] [IntModule J] where
  apply : E → J
  map_zero : apply 0 = 0
  map_zsmul : ∀ (n : Int) (x : E), apply (n • x) = n • apply x

/-- A checked scalar relation on the source is preserved on its divisor image. -/
theorem preserves_scalar (transfer : AdditiveTransfer E J)
    (generator target : E) (n : Int)
    (relation : target = n • generator) :
    transfer.apply target = n • transfer.apply generator := by
  rw [relation, transfer.map_zsmul]

/-- A nonzero image of prime order is enough to retain the target logarithm. -/
theorem prime_image_order (transfer : AdditiveTransfer E J)
    (generator : E) (r : Int)
    (annihilates : r • generator = 0)
    (nonzero : transfer.apply generator ≠ 0) :
    r • transfer.apply generator = 0 ∧ transfer.apply generator ≠ 0 := by
  constructor
  · rw [← transfer.map_zsmul, annihilates, transfer.map_zero]
  · exact nonzero

end GhsHessCubicBounds
