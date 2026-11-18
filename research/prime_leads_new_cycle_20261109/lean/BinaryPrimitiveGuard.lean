import Mathlib.RingTheory.RootsOfUnity.PrimitiveRoots

/-! Standard exact-order criterion for a binary full-power/half-power guard.

This independently authored module uses Mathlib's `orderOf_eq_prime_pow` and
`IsPrimitiveRoot.iff_orderOf`. It imports no PKM implementation or research
module. It states mathematical equations, not Rust execution, primality testing
or a discrete-logarithm cost result. This specialization is not a novelty claim.
-/
namespace BinaryPrimitiveGuard

/-- The two adjacent binary-power tests force exact order at positive depth. -/
theorem primitive_root_of_binary_guard_succ {K : Type*} [Field K]
    (ζ : K) (s : ℕ)
    (hfull : ζ ^ (2 ^ (s + 1)) = 1)
    (hhalf : ζ ^ (2 ^ s) ≠ 1) : IsPrimitiveRoot ζ (2 ^ (s + 1)) := by
  have : Fact (Nat.Prime 2) := ⟨Nat.prime_two⟩
  apply IsPrimitiveRoot.iff_orderOf.mpr
  exact orderOf_eq_prime_pow hhalf hfull

/-- Positive-depth form with the exact truncated-subtraction half exponent. -/
theorem primitive_root_of_binary_guard {K : Type*} [Field K]
    (ζ : K) (t : ℕ) (ht : 0 < t)
    (hfull : ζ ^ (2 ^ t) = 1)
    (hhalf : ζ ^ (2 ^ (t - 1)) ≠ 1) : IsPrimitiveRoot ζ (2 ^ t) := by
  have hsucc : t - 1 + 1 = t := Nat.sub_add_cancel (Nat.succ_le_of_lt ht)
  have h := primitive_root_of_binary_guard_succ ζ (t - 1)
    (by simpa [hsucc] using hfull) hhalf
  simpa [hsucc] using h

#print axioms primitive_root_of_binary_guard_succ
#print axioms primitive_root_of_binary_guard
end BinaryPrimitiveGuard
