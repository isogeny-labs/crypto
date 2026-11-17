import Std

def maxStates (k : Nat) : Nat := 83 * (k * (k - 1) / 2) + 42 * k

theorem maxStates_mono {a b : Nat} (h : a ≤ b) : maxStates a ≤ maxStates b := by
  unfold maxStates
  have hsub : a - 1 ≤ b - 1 := Nat.sub_le_sub_right h 1
  have hmul : a * (a - 1) ≤ b * (b - 1) := Nat.mul_le_mul h hsub
  have hdiv : a * (a - 1) / 2 ≤ b * (b - 1) / 2 := Nat.div_le_div_right hmul
  omega

theorem load75_4405 : 8 * maxStates 4405 ≤ 3 * 2^31 := by decide
theorem load75_4406 : 3 * 2^31 < 8 * maxStates 4406 := by decide

theorem max_k_load75 (k : Nat) (h : 8 * maxStates k ≤ 3 * 2^31) : k ≤ 4405 := by
  by_cases hc : k ≤ 4405
  · exact hc
  have hk : 4406 ≤ k := by omega
  have hm := maxStates_mono hk
  have he := load75_4406
  omega

theorem load80_4549 : 5 * maxStates 4549 ≤ 2 * 2^31 := by decide
theorem load80_4550 : 2 * 2^31 < 5 * maxStates 4550 := by decide

theorem max_k_load80 (k : Nat) (h : 5 * maxStates k ≤ 2 * 2^31) : k ≤ 4549 := by
  by_cases hc : k ≤ 4549
  · exact hc
  have hk : 4550 ≤ k := by omega
  have hm := maxStates_mono hk
  have he := load80_4550
  omega

#print axioms max_k_load75
#print axioms max_k_load80
