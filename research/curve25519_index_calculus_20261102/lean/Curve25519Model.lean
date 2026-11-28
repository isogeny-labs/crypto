import Mathlib.Tactic.Ring

/-!
Kernel-checkable algebra behind the independent Curve25519 model oracle.

These theorems certify only the polynomial translation and the exact
two-primary factorizations of `p - 1` and `p + 1`. They do not certify the Rust
implementation, a factor-base construction, solver cost, or an ECDLP result.
-/

namespace Curve25519Model

variable {K : Type*} [CommRing K]

/-- Integer-polynomial identity for depressing `u^3 + A*u^2 + u`. -/
theorem montgomery_translation_identity (A u : K) :
    27 * (u ^ 3 + A * u ^ 2 + u) =
      (3 * u + A) ^ 3 + 3 * (3 - A ^ 2) * (3 * u + A) + 2 * A ^ 3 - 9 * A := by
  ring

/-- `p - 1` has a factor `4` when `p = 2^255 - 19`. -/
theorem p_minus_one_factorization :
    (2 : ℤ) ^ 255 - 19 - 1 = 4 * ((2 : ℤ) ^ 253 - 5) := by
  ring

/-- The cofactor after removing `4` from `p - 1` is odd. -/
theorem p_minus_one_cofactor_odd : Odd ((2 : ℤ) ^ 253 - 5) := by
  refine ⟨(2 : ℤ) ^ 252 - 3, ?_⟩
  ring

/-- `p + 1` has a factor `2` when `p = 2^255 - 19`. -/
theorem p_plus_one_factorization :
    (2 : ℤ) ^ 255 - 19 + 1 = 2 * ((2 : ℤ) ^ 254 - 9) := by
  ring

/-- The cofactor after removing `2` from `p + 1` is odd. -/
theorem p_plus_one_cofactor_odd : Odd ((2 : ℤ) ^ 254 - 9) := by
  refine ⟨(2 : ℤ) ^ 253 - 5, ?_⟩
  ring

#print axioms montgomery_translation_identity
#print axioms p_minus_one_factorization
#print axioms p_minus_one_cofactor_odd
#print axioms p_plus_one_factorization
#print axioms p_plus_one_cofactor_odd

end Curve25519Model
