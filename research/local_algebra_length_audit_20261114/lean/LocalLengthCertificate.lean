import ResidueDegreeControl

namespace LocalAlgebraLengthAudit
noncomputable section
open CoordinateLengthAudit

theorem actual_local_algebra_length_certificate :
    (∀ (K : Type*) [Field K],
      (∀ m : ℕ, Module.length (Thick K m) (Thick K m) = m ∧
        Module.length (BiAlgebra K m) (BiAlgebra K m) = m) ∧
      (∀ m : ℕ, 0 < m → IsLocalRing (Thick K m) ∧ IsLocalRing (BiAlgebra K m) ∧
        ∀ (L : Type*) [Field L] [Algebra K L], Nat.card (Solution (biIdeal K m) L) = 1) ∧
      (Module.length (Thick K 0) (Thick K 0) = 0 ∧ ¬IsLocalRing (Thick K 0) ∧
        Module.length (BiAlgebra K 0) (BiAlgebra K 0) = 0 ∧ ¬IsLocalRing (BiAlgebra K 0)) ∧
      (∀ b : ℕ, ∃ m : ℕ, 1 < m ∧
        (b : ℕ∞) < Module.length (BiAlgebra K m) (BiAlgebra K m) ∧
        IsLocalRing (BiAlgebra K m))) ∧
    (Nat.card F4 = 4 ∧ IsLocalRing F4 ∧ Module.finrank F2 F4 = 2 ∧
      Module.length F4 F4 = 1 ∧ Module.length F2 F4 = 2 ∧
      ¬Module.length F4 F4 = Module.length F2 F4) := by
  refine ⟨?_, residue_degree_control⟩
  intro K _
  refine ⟨(fun m => ⟨thick_ring_length K m, bi_ring_length K m⟩), ?_,
    ⟨thick_ring_length K 0, thick_zero_not_local K,
      bi_ring_length K 0, bi_zero_not_local K⟩, unbounded_ring_length K⟩
  intro m hm
  refine ⟨thick_local K m hm, bi_local K m hm, ?_⟩
  intro L _ _
  exact bi_support_card m hm

#print axioms actual_local_algebra_length_certificate
end
end LocalAlgebraLengthAudit
