import Mathlib.Tactic

/-! Arithmetic screening criterion from the existing quadratic-LA cost model.
No curve family, oracle exponent, rank/yield law, setup bound or Rust cost
semantics is supplied by this lemma. It is not an ECDLP complexity theorem. -/
namespace PKMGoalModel

theorem m4_threshold (gamma : ℚ) :
    2 * (1 + gamma) / 5 < (1 / 2 : ℚ) ↔ gamma < (1 / 4 : ℚ) := by
  constructor <;> intro h <;> linarith

#print axioms m4_threshold
end PKMGoalModel
