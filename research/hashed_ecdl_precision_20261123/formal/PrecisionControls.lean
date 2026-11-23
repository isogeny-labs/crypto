import CryptoLeanCore.TaperedPrecision

open CryptoLeanCore.TaperedPrecision

namespace NativePrecisionControls

/-- A non-vacuous scalar instance; this does not assert public graph admission. -/
theorem positive_scalar_instance :
    childPrecision 64 128 16 32 15 * 16 ≤ 34 * 128 * 2 ^ 17 := by
  exact childPrecision_mul_le 64 128 16 0 15 128 8 81
    (by decide) (by decide) (by decide) (by decide) (by decide) (by decide)

/-- All other scalar premises can hold while deleting the active valuation
premise admits a false bound. This is a counterexample to the weakened lemma. -/
theorem without_active_valuation :
    16 ≤ 16 ∧ 16 ≤ 16 * 1 ∧ 1296 ≤ 81 * 16 * 2 ^ 0 ∧
    81 * 1 * 16 ≤ 2 ^ 17 ∧ 8 * 16 * 33 ≤ 2 ^ 17 ∧
    ¬ (childPrecision 1296 16 16 32 32 * 16 ≤ 34 * 16 * 2 ^ 0) := by decide

/-- Removing the scale-versus-edge premise permits an incorrect envelope. -/
theorem without_scale_width :
    15 < 16 ∧ 1 ≤ 1 * 1 ∧ 81 ≤ 81 * 1 * 2 ^ 0 ∧
    81 * 1 * 16 ≤ 2 ^ 17 ∧ 8 * 16 * 33 ≤ 2 ^ 17 ∧
    ¬ (childPrecision 81 1 16 32 15 * 16 ≤ 34 * 1 * 2 ^ 17) := by decide

/-- Removing the input-precision budget permits an incorrect envelope. -/
theorem without_inherited_budget :
    16 ≤ 16 ∧ 15 < 16 ∧ 16 ≤ 16 * 1 ∧
    81 * 1 * 16 ≤ 2 ^ 17 ∧ 8 * 16 * 33 ≤ 2 ^ 17 ∧
    ¬ (childPrecision 1000000000 16 16 32 15 * 16 ≤ 34 * 16 * 2 ^ 17) := by decide

/-- Removing the root-to-current edge comparison permits an incorrect envelope. -/
theorem without_root_comparison :
    16 ≤ 16 ∧ 15 < 16 ∧ 81000000000 ≤ 81 * 1000000000 * 2 ^ 0 ∧
    81 * 1 * 16 ≤ 2 ^ 17 ∧ 8 * 16 * 33 ≤ 2 ^ 17 ∧
    ¬ (childPrecision 81000000000 16 16 32 15 * 16 ≤ 34 * 16 * 2 ^ 17) := by decide

/-- Removing dimension-only inherited absorption permits an incorrect envelope. -/
theorem without_inherited_absorption :
    16 ≤ 16 ∧ 15 < 16 ∧ 1000000000 ≤ 16 * 62500000 ∧
    81000000000 ≤ 81 * 1000000000 * 2 ^ 0 ∧ 8 * 16 * 33 ≤ 2 ^ 17 ∧
    ¬ (childPrecision 81000000000 16 16 32 15 * 16 ≤ 34 * 16 * 2 ^ 17) := by decide

/-- Removing evaluation-loss absorption permits an incorrect envelope. -/
theorem without_evaluation_absorption :
    3 ≤ 3 ∧ 2 < 3 ∧ 3 ≤ 3 * 1 ∧ 15 ≤ 5 * 3 * 2 ^ 0 ∧
    5 * 1 * 3 ≤ 2 ^ 4 ∧
    ¬ (childPrecision 15 3 3 6 2 * 3 ≤ 34 * 3 * 2 ^ 4) := by decide

end NativePrecisionControls
