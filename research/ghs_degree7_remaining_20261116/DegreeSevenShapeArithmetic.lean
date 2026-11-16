import Init

/-! Finite arithmetic used in the denominator partition and exact search counts.
    The geometric branch-parity argument and Sage algebra are stated separately. -/
namespace GHSDegreeSevenShapeArithmetic

theorem parity_budget (s h : Nat) :
    2*s + 2*h ≤ 6 ↔ s + h ≤ 3 := by omega

theorem bounded_branch_and_square_degrees (s h : Nat)
    (bound : 2*s + 2*h ≤ 6) : s ≤ 3 ∧ h ≤ 3 := by omega

def admissiblePairs : List (Nat × Nat) :=
  (List.range 4).flatMap fun s =>
    (List.range 4).filterMap fun h =>
      if 2*s + 2*h ≤ 6 then some (s,h) else none

theorem admissible_pairs_exact : admissiblePairs =
    [(0,0),(0,1),(0,2),(0,3),
     (1,0),(1,1),(1,2),
     (2,0),(2,1),(3,0)] := by decide

theorem admissible_pairs_count : admissiblePairs.length = 10 := by decide

theorem two_poles_candidate_count :
    10 * 27 * 242 * 121 = 7906140 := by decide

theorem three_poles_candidate_count :
    10 * 9 * 121 * 243 * 243 = 643043610 := by decide

theorem cubic_square_system_count : 3^5 = 243 := by decide

theorem two_orbit_source_count : 2 * 5 = 10 := by decide

theorem howe_trace22_ceiling_arithmetic :
    5669 * 488^5 < 4^5 * 27^10 := by decide

theorem order37_exceeds_ceiling : 26 < 37 := by decide

end GHSDegreeSevenShapeArithmetic
