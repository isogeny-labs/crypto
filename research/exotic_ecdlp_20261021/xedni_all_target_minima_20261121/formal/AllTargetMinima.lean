import Mathlib.Analysis.Complex.ExponentialBounds
import Mathlib.Tactic

/-!
The centered representatives cover every nonzero residue modulo 43. With
the displayed curve-specific canonical-height inputs, no multiplier outside
[-31,31] can beat a centered representative of coordinate height < 2^306.
-/

namespace XedniAllTargetMinima

theorem tail_square (n : ℤ) (hn : 32 ≤ |n|) : 1024 ≤ n ^ 2 := by
  have hsplit : n ≤ -32 ∨ 32 ≤ n := by
    rcases le_total n 0 with hneg | hpos
    · left
      rw [abs_of_nonpos hneg] at hn
      omega
    · right
      rw [abs_of_nonneg hpos] at hn
      exact hn
  rcases hsplit with hneg | hpos
  · nlinarith [sq_nonneg (n + 32)]
  · nlinarith [sq_nonneg (n - 32)]

theorem tail_cannot_match_centered_height
    (n : ℤ) (hn : 32 ≤ |n|) (hQ h65 hN : ℝ)
    (hQnonneg : 0 ≤ hQ)
    (h65quad : h65 = 4225 * hQ)
    (h65lower : 2893 * Real.log 2 ≤ h65 + 14)
    (hNquad : hN = (n : ℝ) ^ 2 * hQ)
    (hNsmall : hN ≤ 612 * Real.log 2 + 14) : False := by
  have hsq : (1024 : ℝ) ≤ (n : ℝ) ^ 2 := by
    exact_mod_cast tail_square n hn
  have hmin : 1024 * hQ ≤ hN := by
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

def centeredResidues : Finset (ZMod 43) :=
  (Finset.Icc (1 : ℕ) 21).image (fun n : ℕ => (n : ZMod 43)) ∪
  (Finset.Icc (1 : ℕ) 21).image (fun n : ℕ => -(n : ZMod 43))

theorem centeredResidues_card : centeredResidues.card = 42 := by
  decide

theorem nonzero_iff_centered (r : ZMod 43) : r ≠ 0 ↔ r ∈ centeredResidues := by
  have hzero : (0 : ZMod 43) ∉ centeredResidues := by decide
  have hsub : centeredResidues ⊆ Finset.univ.erase (0 : ZMod 43) := by
    intro x hx
    simp only [Finset.mem_erase, Finset.mem_univ, and_true]
    intro h
    subst x
    exact hzero hx
  have hcard : (Finset.univ.erase (0 : ZMod 43)).card = 42 := by decide
  have heq : centeredResidues = Finset.univ.erase (0 : ZMod 43) :=
    Finset.eq_of_subset_of_card_le hsub (by rw [hcard, centeredResidues_card])
  rw [heq]
  simp

#print axioms tail_square
#print axioms tail_cannot_match_centered_height
#print axioms centeredResidues_card
#print axioms nonzero_iff_centered

end XedniAllTargetMinima
