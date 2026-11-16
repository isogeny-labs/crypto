import Mathlib

/-!
# Binary weight (popcount) with the three facts the chain algebra needs

`popcount n = popcount (n / 2) + n % 2`; it does not increase under reduction modulo `2^m`;
adding a clear bit raises it by exactly one; it vanishes only at `0`.
-/

namespace ChainSd

/-- Number of set bits. -/
def popcount (n : ℕ) : ℕ :=
  if h : n = 0 then 0 else popcount (n / 2) + n % 2
decreasing_by exact Nat.div_lt_self (Nat.pos_of_ne_zero h) (by norm_num)

theorem popcount_zero : popcount 0 = 0 := by
  rw [popcount]; simp

theorem popcount_eq (n : ℕ) : popcount n = popcount (n / 2) + n % 2 := by
  rcases Nat.eq_zero_or_pos n with rfl | h
  · simp [popcount_zero]
  · rw [popcount]; simp [Nat.pos_iff_ne_zero.mp h]

/-- Reduction modulo `2^m` does not increase the binary weight. -/
theorem popcount_mod_pow_le (m : ℕ) : ∀ x : ℕ, popcount (x % 2 ^ m) ≤ popcount x := by
  induction m with
  | zero => intro x; simp [Nat.mod_one, popcount_zero]
  | succ m ih =>
    intro x
    have h1 : x % 2 ^ (m + 1) / 2 = x / 2 % 2 ^ m := by
      rw [pow_succ, mul_comm, Nat.mod_mul_right_div_self]
    have h2 : x % 2 ^ (m + 1) % 2 = x % 2 := by
      apply Nat.mod_mod_of_dvd; exact dvd_pow_self 2 (Nat.succ_ne_zero m)
    rw [popcount_eq (x % 2 ^ (m + 1)), h1, h2, popcount_eq x]
    exact Nat.add_le_add_right (ih (x / 2)) _

/-- Adding a clear bit raises the binary weight by exactly one. -/
theorem popcount_add_pow_of_testBit_false (i : ℕ) :
    ∀ x : ℕ, x.testBit i = false → popcount (x + 2 ^ i) = popcount x + 1 := by
  induction i with
  | zero =>
    intro x hx
    rw [Nat.testBit_zero] at hx
    have hx2 : x % 2 = 0 := by
      have := Nat.mod_two_eq_zero_or_one x
      rcases this with h | h
      · exact h
      · simp [h] at hx
    rw [pow_zero, popcount_eq (x + 1), popcount_eq x]
    have e1 : (x + 1) / 2 = x / 2 := by omega
    have e2 : (x + 1) % 2 = 1 := by omega
    rw [e1, e2, hx2]
  | succ i ih =>
    intro x hx
    rw [Nat.testBit_succ] at hx
    have e : x + 2 ^ (i + 1) = (x / 2 + 2 ^ i) * 2 + x % 2 := by
      rw [pow_succ]; omega
    rw [popcount_eq (x + 2 ^ (i + 1)), e]
    have e1 : ((x / 2 + 2 ^ i) * 2 + x % 2) / 2 = x / 2 + 2 ^ i := by omega
    have e2 : ((x / 2 + 2 ^ i) * 2 + x % 2) % 2 = x % 2 := by omega
    rw [e1, e2, ih (x / 2) hx, popcount_eq x]
    ring

/-- Only `0` has binary weight `0`. -/
theorem eq_zero_of_popcount_eq_zero : ∀ x : ℕ, popcount x = 0 → x = 0 := by
  intro x
  induction x using Nat.strong_induction_on with
  | _ x ih =>
    intro h
    rw [popcount_eq] at h
    have h2 : x % 2 = 0 := by omega
    have h1 : popcount (x / 2) = 0 := by omega
    rcases Nat.eq_zero_or_pos x with hx | hx
    · exact hx
    · have := ih (x / 2) (Nat.div_lt_self hx (by norm_num)) h1
      omega

/-- Bits above `2^m` are clear for `x < 2^m`. -/
theorem testBit_false_of_lt {x m : ℕ} (h : x < 2 ^ m) : x.testBit m = false :=
  Nat.testBit_lt_two_pow h

end ChainSd
