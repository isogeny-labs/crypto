import Mathlib.Analysis.Complex.ExponentialBounds
import Mathlib.Tactic

/-!
For the frozen xedni height cap, the canonical-height input excludes every
multiple outside the finite interval [-20,20]. The curve-specific height
comparison and Mordell-Weil identification are explicit hypotheses here.
-/

namespace XedniHeightCoverage

theorem tail_square (n : ℤ) (hn : 21 ≤ |n|) : 441 ≤ n ^ 2 := by
  have hsplit : n ≤ -21 ∨ 21 ≤ n := by
    rcases le_total n 0 with hneg | hpos
    · left
      rw [abs_of_nonpos hneg] at hn
      omega
    · right
      rw [abs_of_nonneg hpos] at hn
      exact hn
  rcases hsplit with hneg | hpos
  · nlinarith [sq_nonneg (n + 21)]
  · nlinarith [sq_nonneg (n - 21)]

theorem tail_excluded
    (n : ℤ) (hn : 21 ≤ |n|) (hQ h65 hN : ℝ)
    (hQnonneg : 0 ≤ hQ)
    (h65quad : h65 = 4225 * hQ)
    (h65lower : 2893 * Real.log 2 ≤ h65 + 14)
    (hNquad : hN = (n : ℝ) ^ 2 * hQ)
    (hNupper : hN ≤ 260 * Real.log 2 + 14) : False := by
  have hsq : (441 : ℝ) ≤ (n : ℝ) ^ 2 := by
    exact_mod_cast tail_square n hn
  have hmin : 441 * hQ ≤ hN := by
    rw [hNquad]
    exact mul_le_mul_of_nonneg_right hsq hQnonneg
  have hloglo : (2 / 3 : ℝ) < Real.log 2 := by
    have h := Real.log_two_gt_d9
    linarith
  have hloghi : Real.log 2 < (7 / 10 : ℝ) := by
    have h := Real.log_two_lt_d9
    linarith
  rw [h65quad] at h65lower
  linarith

def coveredResidues : Finset (ZMod 43) :=
  (Finset.Icc (1 : ℕ) 13).image (fun n : ℕ => (n : ZMod 43)) ∪
  (Finset.Icc (1 : ℕ) 13).image (fun n : ℕ => -(n : ZMod 43))

theorem coveredResidues_card : coveredResidues.card = 26 := by
  decide

#print axioms tail_square
#print axioms tail_excluded
#print axioms coveredResidues_card

end XedniHeightCoverage
