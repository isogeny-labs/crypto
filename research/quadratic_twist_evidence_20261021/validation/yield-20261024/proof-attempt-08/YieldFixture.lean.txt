import RelationYield
import RelationCost
import Mathlib.Data.ZMod.Basic

/-! A kernel-checked control: higher raw zero-relation yield caused by smaller
order need not give any nonzero relation row modulo the target order. -/
namespace YieldFixture

set_option maxRecDepth 100000
open WeierstrassCurve.Affine

abbrev F := ZMod 5
instance : Fact (Nat.Prime 5) := ⟨by decide⟩
abbrev E := TwistEmbedding.short (1 : F) 1
abbrev T := TwistEmbedding.twist (1 : F) 1 2

instance : E.IsElliptic := by
  constructor
  apply isUnit_iff_ne_zero.mpr
  decide
instance : T.IsElliptic := by
  constructor
  apply isUnit_iff_ne_zero.mpr
  decide

def P : E.Point := Point.some 0 1
  ((equation_iff_nonsingular_of_Δ_ne_zero (WeierstrassCurve.isUnit_Δ E).ne_zero).mp
    (by rw [equation_iff]; decide))
def U : T.Point := Point.some 2 2
  ((equation_iff_nonsingular_of_Δ_ne_zero (WeierstrassCurve.isUnit_Δ T).ne_zero).mp
    (by rw [equation_iff]; decide))

def mainZeros : Finset (Fin 9) := Finset.univ.filter (fun n => n.val • P = 0)
def twistZeros : Finset (Fin 9) := Finset.univ.filter (fun n => n.val • U = 0)
def twistNonzeroRows : Finset (Fin 9) :=
  Finset.univ.filter (fun n => n.val • U = 0 ∧ n.val % 3 ≠ 0)

theorem main_annihilator : 9 • P = 0 := by decide +kernel
theorem main_not_three : 3 • P ≠ 0 := by decide +kernel
theorem twist_annihilator : 3 • U = 0 := by decide +kernel
theorem twist_nonzero : U ≠ 0 := by decide +kernel
theorem main_zero_count : mainZeros.card = 1 := by decide +kernel
theorem twist_zero_count : twistZeros.card = 3 := by decide +kernel
theorem raw_count_higher : mainZeros.card < twistZeros.card := by
  rw [main_zero_count, twist_zero_count]
  decide
theorem all_twist_rows_trivial : ∀ n : Fin 9, n.val • U = 0 ↔ n.val % 3 = 0 := by
  decide +kernel
theorem no_nonzero_twist_rows : twistNonzeroRows.card = 0 := by decide +kernel

/-- Synthetic rational-budget illustration, not measurements on the F5 curves above. -/
theorem model_can_improve :
    RelationCost.budget 5000 10 (RelationCost.macaulayWork 2 3 2) (1/5) <
      RelationCost.budget 0 10 (RelationCost.macaulayWork 2 3 3) (1/10) := by decide +kernel

/-- The same model inputs lose when the additional overhead exceeds the savings. -/
theorem overhead_can_reverse :
    RelationCost.budget 100000 1 (RelationCost.macaulayWork 2 3 2) (1/5) >
      RelationCost.budget 0 1 (RelationCost.macaulayWork 2 3 3) (1/10) := by decide +kernel

#print axioms main_annihilator
#print axioms main_not_three
#print axioms twist_annihilator
#print axioms twist_nonzero
#print axioms main_zero_count
#print axioms twist_zero_count
#print axioms raw_count_higher
#print axioms all_twist_rows_trivial
#print axioms no_nonzero_twist_rows
#print axioms model_can_improve
#print axioms overhead_can_reverse

end YieldFixture
