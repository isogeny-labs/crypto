import Mathlib

namespace H4PackedPointKey

/-- The low coordinate and shifted high coordinate occupy disjoint bits. -/
theorem or_eq_add (x y : ℕ) (hx : x < 2 ^ 31) :
    x ||| (y <<< 31) = x + 2 ^ 31 * y := by
  calc
    x ||| (y <<< 31) = (y <<< 31) ||| x := Nat.or_comm ..
    _ = (y <<< 31) + x := (Nat.shiftLeft_add_eq_or_of_lt hx y).symm
    _ = x + 2 ^ 31 * y := by simp [Nat.shiftLeft_eq, Nat.mul_comm, Nat.add_comm]

/-- Bounded binary coordinates have an injective packed key. -/
theorem key_injective (x₁ y₁ x₂ y₂ : ℕ)
    (hx₁ : x₁ < 2 ^ 31) (hx₂ : x₂ < 2 ^ 31)
    (h : x₁ ||| (y₁ <<< 31) = x₂ ||| (y₂ <<< 31)) :
    x₁ = x₂ ∧ y₁ = y₂ := by
  rw [or_eq_add x₁ y₁ hx₁, or_eq_add x₂ y₂ hx₂] at h
  omega

/-- A valid packed point key is below the reserved identity sentinel. -/
theorem key_lt_sentinel (x y : ℕ)
    (hx : x < 2 ^ 31) (hy : y < 2 ^ 31) :
    x ||| (y <<< 31) < 2 ^ 62 := by
  rw [or_eq_add x y hx]
  omega

#print axioms H4PackedPointKey.or_eq_add
#print axioms H4PackedPointKey.key_injective
#print axioms H4PackedPointKey.key_lt_sentinel
end H4PackedPointKey
