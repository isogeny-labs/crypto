import Init

namespace H4PackedPointKey35

/-- The low and shifted high coordinates occupy disjoint bit ranges. -/
theorem or_eq_add (x y : Nat) (hx : x < 2 ^ 35) :
    x ||| (y <<< 35) = x + 2 ^ 35 * y := by
  calc
    x ||| (y <<< 35) = (y <<< 35) ||| x := Nat.or_comm ..
    _ = (y <<< 35) + x := (Nat.shiftLeft_add_eq_or_of_lt hx y).symm
    _ = x + 2 ^ 35 * y := by rw [Nat.shiftLeft_eq, Nat.mul_comm, Nat.add_comm]

/-- Two bounded-coordinate points with equal packed keys have equal coordinates. -/
theorem key_injective (x₁ y₁ x₂ y₂ : Nat)
    (hx₁ : x₁ < 2 ^ 35) (hx₂ : x₂ < 2 ^ 35)
    (h : x₁ ||| (y₁ <<< 35) = x₂ ||| (y₂ <<< 35)) :
    x₁ = x₂ ∧ y₁ = y₂ := by
  rw [or_eq_add x₁ y₁ hx₁, or_eq_add x₂ y₂ hx₂] at h
  have hx : x₁ = x₂ := by
    have hm := congrArg (fun z : Nat => z % (2 ^ 35)) h
    simpa [Nat.add_mul_mod_self_left, Nat.mod_eq_of_lt hx₁,
      Nat.mod_eq_of_lt hx₂] using hm
  constructor
  · exact hx
  · subst x₂
    have hm : 2 ^ 35 * y₁ = 2 ^ 35 * y₂ := Nat.add_left_cancel h
    exact Nat.mul_left_cancel (by decide : 0 < 2 ^ 35) hm

/-- Valid coordinate keys lie below the reserved identity sentinel. -/
theorem key_lt_sentinel (x y : Nat)
    (hx : x < 2 ^ 35) (hy : y < 2 ^ 35) :
    x ||| (y <<< 35) < 2 ^ 70 := by
  rw [or_eq_add x y hx]
  have hy_le : y + 1 ≤ 2 ^ 35 := Nat.succ_le_of_lt hy
  calc
    x + 2 ^ 35 * y < 2 ^ 35 + 2 ^ 35 * y := Nat.add_lt_add_right hx _
    _ = 2 ^ 35 * (y + 1) := by rw [Nat.mul_succ, Nat.add_comm]
    _ ≤ 2 ^ 35 * 2 ^ 35 := Nat.mul_le_mul_left _ hy_le
    _ = 2 ^ 70 := by rw [← Nat.pow_add]

#print axioms H4PackedPointKey35.or_eq_add
#print axioms H4PackedPointKey35.key_injective
#print axioms H4PackedPointKey35.key_lt_sentinel
end H4PackedPointKey35
