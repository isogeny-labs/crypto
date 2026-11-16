import Lean

/-! Section 7c of the README, arithmetic step only (core Lean, no Mathlib).

The reach theorem of `research/smallroot_optimal_reach_20261026` says that every
lattice in `(p, f)^t` on a convex monomial set, with equal per-variable bounds,
has per-variable reach `delta <= (n! * vol Newt f)^(-1/n) / n`. That theorem is
NOT formalized here, and neither is the measured fact that the Newton polytopes
of S_4 and S_5 are the full boxes `[0,4]^3` and `[0,8]^4` (README L4-B).

For positive reals, `(n! V)^(-1/n) / n <= 1/k` is equivalent to
`k^n <= n^n * (n! V)`. The real-root equivalence needs Mathlib and is also not
formalized. This file checks the resulting integer inequalities. -/
namespace PrimeS5SmallRoot

/-- `k^n <= n^n * (n! * V)`: the reach bound `(n! V)^(-1/n) / n` is at most `1/k`. -/
def ReachAtMost (n nfact V k : Nat) : Prop := k ^ n ≤ n ^ n * (nfact * V)

instance (n nfact V k : Nat) : Decidable (ReachAtMost n nfact V k) := by
  unfold ReachAtMost; exact inferInstance

theorem s4_box_volume : (4 : Nat) ^ 3 = 64 := by decide
theorem s5_box_volume : (8 : Nat) ^ 4 = 4096 := by decide
theorem fact3 : 3 * 2 * 1 = 6 := by decide
theorem fact4 : 4 * 3 * 2 * 1 = 24 := by decide

/-- S_4 (n = 3, V = 64): reach at most 1/21, so short of 1/4 by at least 5x (21 >= 5 * 4). -/
theorem s4_reach_le_1_21 : ReachAtMost 3 6 64 21 := by decide
theorem s4_misses_quarter_by_5x : ReachAtMost 3 6 64 (5 * 4) := by decide

/-- S_5 (n = 4, V = 4096): reach at most 1/70, so short of 1/5 by at least 14x (70 = 14 * 5). -/
theorem s5_reach_le_1_70 : ReachAtMost 4 24 4096 70 := by decide
theorem s5_misses_fifth_by_14x : ReachAtMost 4 24 4096 (14 * 5) := by decide

/-- The README's rounded values 1/21.8 and 1/70.8 are tight to one unit:
the bound is not at most 1/22 or 1/71. -/
theorem s4_not_le_1_22 : ¬ ReachAtMost 3 6 64 22 := by decide
theorem s5_not_le_1_71 : ¬ ReachAtMost 4 24 4096 71 := by decide

end PrimeS5SmallRoot
