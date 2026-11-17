import Mathlib.Data.Rat.Lemmas
import Mathlib.Tactic.Linarith
import Mathlib.Tactic.NormNum

/- Numeric instances of a proposed saturated-yield cost model only.
   No solver, distribution, real-power asymptotic or admissibility theorem. -/
set_option autoImplicit false
namespace Curve25519ScalingAudit

example (a b : ℚ) : a ≤ max a b := le_max_left a b

def fourEnvelope (beta : ℚ) : ℚ :=
  max (2 * beta) (((3 : ℚ) / 2) * beta + max 0 (1 - 4 * beta))

theorem four_lower (beta : ℚ) : (4 : ℚ) / 9 ≤ fourEnvelope beta := by
  by_cases h : (2 : ℚ) / 9 ≤ beta
  · exact le_trans (by linarith) (le_max_left _ _)
  · have hcap : 1 - 4 * beta ≤ max (0 : ℚ) (1 - 4 * beta) := le_max_right _ _
    exact le_trans (by linarith) (le_max_right _ _)

theorem four_attained : fourEnvelope ((2 : ℚ) / 9) = (4 : ℚ) / 9 := by
  norm_num [fourEnvelope]

def sixEnvelope (beta : ℚ) : ℚ :=
  max (2 * beta) (((5 : ℚ) / 2) * beta + max 0 (1 - 6 * beta))

theorem six_lower (beta : ℚ) : (5 : ℚ) / 12 ≤ sixEnvelope beta := by
  by_cases h : (1 : ℚ) / 6 ≤ beta
  · have hcap : (0 : ℚ) ≤ max 0 (1 - 6 * beta) := le_max_left _ _
    exact le_trans (by linarith) (le_max_right _ _)
  · have hcap : 1 - 6 * beta ≤ max (0 : ℚ) (1 - 6 * beta) := le_max_right _ _
    exact le_trans (by linarith) (le_max_right _ _)

theorem six_attained : sixEnvelope ((1 : ℚ) / 6) = (5 : ℚ) / 12 := by
  norm_num [sixEnvelope]

theorem six_naive_balance : sixEnvelope ((2 : ℚ) / 11) = (5 : ℚ) / 11 := by
  norm_num [sixEnvelope]

theorem numeric_model_below_half : (4 : ℚ) / 9 < 1 / 2 ∧ (5 : ℚ) / 12 < 1 / 2 := by
  norm_num

#print axioms four_lower
#print axioms four_attained
#print axioms six_lower
#print axioms six_attained
#print axioms six_naive_balance
#print axioms numeric_model_below_half
end Curve25519ScalingAudit
