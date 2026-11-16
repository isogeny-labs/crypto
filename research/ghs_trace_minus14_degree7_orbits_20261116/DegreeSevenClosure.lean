import Init
import Init.Omega

/-!
Checked indexing and conditional arithmetic closure for the trace-minus14
seven-degree screen. The hypotheses about geometric completeness, Sage
elimination, prior orbit identities, and Howe's theorem are external.
-/
namespace TraceMinus14DegreeSeven

def sourceRepresentatives : List Nat := [11,202,38,80]
def carrierRepresentatives : List Nat := [9735,17371]
def representativeCells : List (Nat × Nat) :=
  sourceRepresentatives.flatMap fun b => carrierRepresentatives.map fun f => (b,f)

theorem representative_cells_complete :
    representativeCells.length = 8 ∧ representativeCells.Nodup := by decide

theorem recorded_pairs : (20 : Nat) * 12 = 240 := by decide

theorem new_receipt_count : (7 : Nat) * 6 = 42 := by decide

def polePairs : List (Nat × Nat) :=
  (List.range 4).flatMap fun s =>
    (List.range 4).filterMap fun h =>
      if 2*s + 2*h ≤ 6 then some (s,h) else none

theorem denominator_parity_pairs : polePairs =
    [(0,0),(0,1),(0,2),(0,3),
     (1,0),(1,1),(1,2),
     (2,0),(2,1),(3,0)] := by decide

theorem cover_degree_window (d : Nat)
    (none_small : ∀ e : Nat, e ≤ 7 → e ≠ d)
    (howe_ceiling : d ≤ 33) : 8 ≤ d ∧ d ≤ 33 := by
  constructor
  · have hnot : ¬ d ≤ 7 := by
      intro h7
      exact none_small d h7 rfl
    omega
  · exact howe_ceiling

theorem degree_below_selected_prime (d : Nat)
    (window : 8 ≤ d ∧ d ≤ 33) : d < 43 := by omega

end TraceMinus14DegreeSeven
