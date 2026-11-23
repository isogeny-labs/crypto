import CanonicalPieces

namespace JVGradedHilbertAudit
noncomputable section
open JVHilbertAudit
attribute [local instance] MvPolynomial.gradedAlgebra

theorem canonical_graded_hilbert_certificate :
    Ideal.IsHomogeneous sourceGrading JVMacaulayAudit.sourceIdeal ∧
    (∀ d : ℕ, ∀ p : sourceGrading d,
      p ∈ LinearMap.ker (degreeMap d) ↔ (p : Poly) ∈ JVMacaulayAudit.sourceIdeal) ∧
    (∀ d : ℕ, Nonempty (degreePiece d ≃ₗ[F] degreeImage d)) ∧
    (∀ d : ℕ, degreeImage d = gradedSpan d) ∧
    (∀ d : ℕ, hilbertFunction d =
      if d = 0 then 1 else if d = 1 then 3 else if d = 2 then 3 else if d = 3 then 1 else 0) ∧
    (hilbertFunction 0 = 1 ∧ hilbertFunction 1 = 3 ∧
      hilbertFunction 2 = 3 ∧ hilbertFunction 3 = 1) ∧
    (∀ d : ℕ, 4 ≤ d → hilbertFunction d = 0) ∧
    (∀ d : ℕ, hilbertFunction d = 0 ↔ 4 ≤ d) ∧
    Module.finrank F A = 8 ∧ Module.length F A = 8 ∧
    hilbertFunction 0 ≠ Module.finrank F A := by
  refine ⟨sourceIdeal_homogeneous, degreeMap_kernel_mem,
    (fun d => ⟨pieceEquivImage d⟩), degreeImage_eq_gradedSpan,
    canonical_piece_dimension, low_degrees, high_degrees_zero, zero_piece_iff,
    actual_coordinate_algebra_dimension, actual_scalar_module_length,
    graded_dimension_not_total_dimension⟩

#print axioms canonical_graded_hilbert_certificate
end
end JVGradedHilbertAudit
