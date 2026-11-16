import Init

/-! Checked finite arithmetic for the trace -14 fixed-pair search.
The geometric parity lemma and exhaustive Sage elimination are separate evidence. -/
namespace GHSTraceMinus14Arithmetic

theorem denominator_budget (s h : Nat) :
    2*s + 2*h ≤ 6 ↔ s + h ≤ 3 := by omega

def admissiblePairs : List (Nat × Nat) :=
  (List.range 4).flatMap fun s =>
    (List.range 4).filterMap fun h =>
      if 2*s + 2*h ≤ 6 then some (s,h) else none

theorem admissible_pairs_exact : admissiblePairs =
    [(0,0),(0,1),(0,2),(0,3),
     (1,0),(1,1),(1,2),
     (2,0),(2,1),(3,0)] := by decide

theorem source_order_factorization : 258 = 2 * 3 * 43 := by decide

theorem howe_ceiling_arithmetic :
    5669 * 776^5 < 4^5 * 34^10 := by decide

theorem selected_subgroup_exceeds_ceiling : 33 < 43 := by decide

theorem two_poles_candidate_count :
    10 * 27 * 242 * 121 = 7906140 := by decide

theorem three_poles_candidate_count :
    10 * 9 * 121 * 243 * 243 = 643043610 := by decide

end GHSTraceMinus14Arithmetic
