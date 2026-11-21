import Mathlib.LinearAlgebra.Matrix.Rank
import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

/-! Signed relation rows in the fixed 21-column subgroup quotient. -/

namespace XedniRelationRank

set_option maxHeartbeats 0

abbrev K := ZMod 43

def baseRow (j : Fin 21) : K :=
  if j = 5 then 1 else if j = 8 then 2 else 0

def relationMatrix : Matrix (Fin 264) (Fin 21) K :=
  fun i j => if i.val < 132 then baseRow j else -baseRow j

theorem mirror_row (i : Fin 264) (hi : 132 ≤ i.val) :
    relationMatrix.row i = -baseRow := by
  funext j
  simp [relationMatrix, Matrix.row, Nat.not_lt.mpr hi]

theorem base_row (i : Fin 264) (hi : i.val < 132) :
    relationMatrix.row i = baseRow := by
  funext j
  simp [relationMatrix, Matrix.row, hi]

theorem baseRow_ne_zero : baseRow ≠ 0 := by
  intro h
  have h5 := congrArg (fun f : Fin 21 → K => f 5) h
  norm_num [baseRow] at h5
  have hneq : (1 : K) ≠ 0 := by decide
  exact hneq h5

theorem relationMatrix_row_span :
    Submodule.span K (Set.range relationMatrix.row) = K ∙ baseRow := by
  let S : Submodule K (Fin 21 → K) := K ∙ baseRow
  have hbase : baseRow ∈ S := by
    change baseRow ∈ K ∙ baseRow
    exact Submodule.subset_span (by simp)
  have hspan_le : Submodule.span K (Set.range relationMatrix.row) ≤ S := by
    rw [Submodule.span_le]
    rintro v ⟨i, rfl⟩
    by_cases hi : i.val < 132
    · simp [S, base_row i hi]
    · have hneg : -baseRow ∈ S := S.neg_mem hbase
      simpa [mirror_row i (Nat.le_of_not_lt hi)] using hneg
  have hle_span : S ≤ Submodule.span K (Set.range relationMatrix.row) := by
    change K ∙ baseRow ≤ Submodule.span K (Set.range relationMatrix.row)
    rw [Submodule.span_le]
    intro v hv
    have hv' : v = baseRow := by simpa using hv
    subst v
    apply Submodule.subset_span
    refine ⟨(0 : Fin 264), ?_⟩
    exact (base_row 0 (by decide)).symm
  exact le_antisymm hspan_le hle_span

#print axioms mirror_row
#print axioms relationMatrix_row_span
#print axioms baseRow_ne_zero

end XedniRelationRank
