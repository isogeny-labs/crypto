import Mathlib.Tactic

namespace XedniPreregisteredPanel

/-- With the observed reduction `R̄ = 11G` at `p=1531`, a box of radius 41
cannot reach scalar residue 500 in the order-1553 subgroup. -/
theorem missing_residue_500 (a b : ℤ)
    (ha_lo : -41 ≤ a) (ha_hi : a ≤ 41)
    (hb_lo : -41 ≤ b) (hb_hi : b ≤ 41) :
    (a + 11 * b) % 1553 ≠ 500 := by
  have hlow : -492 ≤ a + 11 * b := by omega
  have hhigh : a + 11 * b ≤ 492 := by omega
  omega

/-- Every reduced scalar in this coefficient box lies in one of two
intervals, with a gap containing 500. -/
theorem image_in_two_intervals (a b : ℤ)
    (ha_lo : -41 ≤ a) (ha_hi : a ≤ 41)
    (hb_lo : -41 ≤ b) (hb_hi : b ≤ 41) :
    (a + 11 * b) % 1553 ≤ 492 ∨ 1061 ≤ (a + 11 * b) % 1553 := by
  have hlow : -492 ≤ a + 11 * b := by omega
  have hhigh : a + 11 * b ≤ 492 := by omega
  omega

#print axioms missing_residue_500
#print axioms image_in_two_intervals

end XedniPreregisteredPanel
