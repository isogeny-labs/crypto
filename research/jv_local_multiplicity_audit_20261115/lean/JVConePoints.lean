import JVCharts

namespace JVLocalMultiplicityAudit
noncomputable section
open JVHilbertAudit

/-- A cardinality corollary of the credited all-field zero characterization. -/
def ConeSolutions (L : Type*) [Field L] [CharP L 101] :=
  {p : Fin 3 → L // JVMacaulayAudit.f0 (p 0) (p 1) (p 2) = 0 ∧
    JVMacaulayAudit.f1 (p 0) (p 1) (p 2) = 0 ∧
    JVMacaulayAudit.f2 (p 0) (p 1) (p 2) = 0}

def coneOrigin (L : Type*) [Field L] [CharP L 101] : ConeSolutions L :=
  ⟨fun _ => 0, (actual_geometric_zero_iff 0 0 0).mpr ⟨rfl, rfl, rfl⟩⟩

theorem cone_unique (L : Type*) [Field L] [CharP L 101] (p : ConeSolutions L) :
    p = coneOrigin L := by
  have hv := (actual_geometric_zero_iff (p.val 0) (p.val 1) (p.val 2)).mp p.property
  apply Subtype.ext
  funext i
  fin_cases i
  · exact hv.1
  · exact hv.2.1
  · exact hv.2.2

def coneEquivUnit (L : Type*) [Field L] [CharP L 101] : ConeSolutions L ≃ PUnit.{1} where
  toFun _ := PUnit.unit
  invFun _ := coneOrigin L
  left_inv p := (cone_unique L p).symm
  right_inv p := by cases p; rfl

theorem cone_card (L : Type*) [Field L] [CharP L 101] : Nat.card (ConeSolutions L) = 1 := by
  calc
    Nat.card (ConeSolutions L) = Nat.card PUnit.{1} := Nat.card_congr (coneEquivUnit L)
    _ = 1 := by simp

theorem length_not_point_count : Module.length A A ≠ Nat.card (ConeSolutions F) := by
  rw [actual_ring_length, cone_card]
  decide

#print axioms cone_card
#print axioms length_not_point_count
end
end JVLocalMultiplicityAudit
