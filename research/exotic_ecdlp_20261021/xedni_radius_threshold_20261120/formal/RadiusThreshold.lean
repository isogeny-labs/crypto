import Mathlib.Tactic

namespace XedniRadiusThreshold

/-- The radius-64 box misses residue 769 in the order-1553 scalar group. -/
theorem radius64_misses_769 (a b : ℤ)
    (ha_lo : -64 ≤ a) (ha_hi : a ≤ 64)
    (hb_lo : -64 ≤ b) (hb_hi : b ≤ 64) :
    (a + 11 * b) % 1553 ≠ 769 := by
  have hlow : -768 ≤ a + 11 * b := by omega
  have hhigh : a + 11 * b ≤ 768 := by omega
  omega

/-- Every smaller nonnegative radius also misses residue 769. -/
theorem below65_misses_769 (B a b : ℤ) (hB : 0 ≤ B) (hB_hi : B < 65)
    (ha_lo : -B ≤ a) (ha_hi : a ≤ B)
    (hb_lo : -B ≤ b) (hb_hi : b ≤ B) :
    (a + 11 * b) % 1553 ≠ 769 := by
  apply radius64_misses_769 a b <;> omega

/-- Every scalar residue of the order-1553 group has a radius-65
representation under the observed `R̄ = 11Q̄` relation. -/
theorem radius65_covers (t : ℤ) (ht_lo : 0 ≤ t) (ht_hi : t < 1553) :
    ∃ a b : ℤ, -65 ≤ a ∧ a ≤ 65 ∧ -65 ≤ b ∧ b ≤ 65 ∧
      (a + 11 * b) % 1553 = t := by
  let z : ℤ := if t ≤ 776 then t else t - 1553
  have hz_lo : -776 ≤ z := by dsimp [z]; split_ifs <;> omega
  have hz_hi : z ≤ 776 := by dsimp [z]; split_ifs <;> omega
  let b : ℤ := if z < -715 then -65 else if 715 < z then 65 else z / 11
  let a : ℤ := z - 11 * b
  have hb_lo : -65 ≤ b := by dsimp [b]; split_ifs <;> omega
  have hb_hi : b ≤ 65 := by dsimp [b]; split_ifs <;> omega
  have ha_lo : -65 ≤ a := by dsimp [a, b]; split_ifs <;> omega
  have ha_hi : a ≤ 65 := by dsimp [a, b]; split_ifs <;> omega
  refine ⟨a, b, ha_lo, ha_hi, hb_lo, hb_hi, ?_⟩
  have hz_mod : z % 1553 = t := by dsimp [z]; split_ifs <;> omega
  have hsum : a + 11 * b = z := by dsimp [a]; ring
  rw [hsum]
  exact hz_mod

#print axioms radius64_misses_769
#print axioms below65_misses_769
#print axioms radius65_covers

end XedniRadiusThreshold
