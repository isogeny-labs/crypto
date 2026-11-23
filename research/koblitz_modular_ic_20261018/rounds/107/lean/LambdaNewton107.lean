import Std.Tactic

namespace LambdaNewton107

-- Exact source-bound orders are supplied by R107; these lemmas check
-- the lower-hull arithmetic and degree/ramification bookkeeping.
theorem yZeroEdges :
    28 - 23 = 5 ∧ 5 * 7 > 23 ∧
    7 * 23 > 7 * 23 - 23 * (4 - 1) ∧
    23 < 32 ∧ 28 < 32 := by omega

theorem yOneEdges :
    27 - 23 = 4 ∧ 4 * 7 > 23 ∧
    7 * 23 > 7 * 23 - 23 * (4 - 1) ∧
    23 < 32 ∧ 27 < 32 := by omega

theorem coprimeSecondEdge : Nat.gcd 23 7 = 1 := by decide

theorem completeDegreeAccounting :
    3 * (2 * 1 + 2 * 7) = 48 ∧
    4 * (2 * 1 + 2 * 7) = 64 ∧
    48 + 64 = 112 ∧
    3 * 7 = 21 ∧ 4 * 7 = 28 := by omega

end LambdaNewton107
