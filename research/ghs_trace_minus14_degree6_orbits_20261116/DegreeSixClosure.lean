import Init
import Init.Omega

/-!
Arithmetic closure for the trace-minus14 degree-six orbit receipts. The Lean
proofs certify finite indexing and the logical implication of an exclusion
through degree six. They do not certify Sage elimination, geometric pole
classification, or PGL2 coordinate identities.
-/
namespace TraceMinus14DegreeSix

def sourceRepresentatives : List Nat := [11, 202, 38, 80]
def carrierRepresentatives : List Nat := [9735, 17371]
def representativeCells : List (Nat × Nat) :=
  sourceRepresentatives.flatMap fun b => carrierRepresentatives.map fun f => (b, f)

theorem representative_cell_count :
    representativeCells.length = 8 ∧ representativeCells.Nodup := by decide

theorem recorded_equation_pair_count : (20 : Nat) * 12 = 240 := by decide

theorem assignments_per_cell :
    (243 : Nat) + 243 + 405 + 58806 + 243 + 405 + 58806 + 270 + 98010 + 14288643
      = 14506074 := by decide

theorem assignments_all_representatives : (8 : Nat) * 14506074 = 116048592 := by decide

theorem cover_degree_window (d : Nat)
    (none_small : ∀ e : Nat, e ≤ 6 → e ≠ d)
    (howe_ceiling : d ≤ 33) : 7 ≤ d ∧ d ≤ 33 := by
  constructor
  · have hnot : ¬ d ≤ 6 := by
      intro h6
      exact none_small d h6 rfl
    omega
  · exact howe_ceiling

theorem degree_below_selected_prime (d : Nat)
    (window : 7 ≤ d ∧ d ≤ 33) : d < 43 := by omega

end TraceMinus14DegreeSix
