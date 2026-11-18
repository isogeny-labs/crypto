import TruncatedLocal
import Mathlib.FieldTheory.Finite.GaloisField

namespace LocalAlgebraLengthAudit
noncomputable section
private instance primeTwo : Fact (Nat.Prime 2) := ⟨by decide⟩
abbrev F2 := ZMod 2
abbrev F4 : Type := GaloisField 2 2

theorem residue_degree_control :
    Nat.card F4 = 4 ∧ IsLocalRing F4 ∧ Module.finrank F2 F4 = 2 ∧
    Module.length F4 F4 = 1 ∧ Module.length F2 F4 = 2 ∧
    ¬Module.length F4 F4 = Module.length F2 F4 := by
  have hd : Module.finrank F2 F4 = 2 := GaloisField.finrank 2 (by decide)
  have hself : Module.length F4 F4 = 1 := by
    rw [Module.length_eq_finrank, Module.finrank_self]
    rfl
  have hbase : Module.length F2 F4 = 2 := by
    rw [Module.length_eq_finrank, hd]
    rfl
  exact ⟨GaloisField.card 2 2 (by decide), inferInstance, hd, hself, hbase,
    by rw [hself, hbase]; decide⟩

#print axioms residue_degree_control
end
end LocalAlgebraLengthAudit
