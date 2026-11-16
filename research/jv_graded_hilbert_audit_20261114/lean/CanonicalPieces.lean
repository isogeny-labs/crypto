import GradedBasis
import Mathlib.LinearAlgebra.Isomorphisms
import Mathlib.RingTheory.GradedAlgebra.Homogeneous.Ideal

namespace JVGradedHilbertAudit
noncomputable section
open MvPolynomial JVHilbertAudit
attribute [local instance] MvPolynomial.gradedAlgebra

abbrev sourceGrading : ℕ → Submodule F Poly := homogeneousSubmodule (Fin 3) F

/-- The actual quotient map restricted to degree-d homogeneous polynomials. -/
def degreeMap (d : ℕ) : sourceGrading d →ₗ[F] A :=
  quotientMap.toLinearMap.comp (sourceGrading d).subtype

def degreeImage (d : ℕ) : Submodule F A := LinearMap.range (degreeMap d)

/-- The canonical degree piece P_d/(I ∩ P_d), with the intersection expressed
    as the actual kernel of the restricted quotient map. -/
abbrev degreePiece (d : ℕ) := sourceGrading d ⧸ LinearMap.ker (degreeMap d)

theorem degreeMap_kernel_mem (d : ℕ) (p : sourceGrading d) :
    p ∈ LinearMap.ker (degreeMap d) ↔ (p : Poly) ∈ JVMacaulayAudit.sourceIdeal := by
  change quotientMap (p : Poly) = 0 ↔ (p : Poly) ∈ JVMacaulayAudit.sourceIdeal
  exact Ideal.Quotient.eq_zero_iff_mem

def pieceEquivImage (d : ℕ) : degreePiece d ≃ₗ[F] degreeImage d :=
  (degreeMap d).quotKerEquivRange

theorem degreeImage_eq_gradedSpan (d : ℕ) : degreeImage d = gradedSpan d := by
  apply le_antisymm
  · rintro z ⟨p, rfl⟩
    exact homogeneous_image_mem p.property
  · apply Submodule.span_le.mpr
    rintro z ⟨i, rfl⟩
    refine ⟨⟨basisPoly i.val, ?_⟩, rfl⟩
    change (basisPoly i.val).IsHomogeneous d
    simpa only [i.property] using basisPoly_homogeneous i.val

def hilbertFunction (d : ℕ) : ℕ := Module.finrank F (degreePiece d)

theorem canonical_piece_dimension (d : ℕ) : hilbertFunction d = expectedHilbert d := by
  unfold hilbertFunction
  rw [(pieceEquivImage d).finrank_eq, degreeImage_eq_gradedSpan,
    gradedSpan_dimension, degreeIndex_card]

theorem source_generators_homogeneous :
    (JVMacaulayAudit.f0 (X 0) (X 1) (X 2) : Poly).IsHomogeneous 2 ∧
    (JVMacaulayAudit.f1 (X 0) (X 1) (X 2) : Poly).IsHomogeneous 2 ∧
    (JVMacaulayAudit.f2 (X 0) (X 1) (X 2) : Poly).IsHomogeneous 2 := by
  rw [actual_source_as_monomials.1, actual_source_as_monomials.2.1,
    actual_source_as_monomials.2.2]
  refine ⟨?_, ?_, ?_⟩
  all_goals repeat' apply IsHomogeneous.add
  all_goals apply isHomogeneous_monomial
  all_goals rw [exponent_degree]

theorem sourceIdeal_homogeneous :
    Ideal.IsHomogeneous sourceGrading JVMacaulayAudit.sourceIdeal := by
  unfold JVMacaulayAudit.sourceIdeal
  apply Ideal.homogeneous_span
  intro p hp
  simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hp
  rcases hp with rfl | rfl | rfl
  · exact ⟨2, source_generators_homogeneous.1⟩
  · exact ⟨2, source_generators_homogeneous.2.1⟩
  · exact ⟨2, source_generators_homogeneous.2.2⟩

theorem low_degrees :
    hilbertFunction 0 = 1 ∧ hilbertFunction 1 = 3 ∧
    hilbertFunction 2 = 3 ∧ hilbertFunction 3 = 1 := by
  simp only [canonical_piece_dimension, expectedHilbert]
  decide

theorem high_degrees_zero (d : ℕ) (hd : 4 ≤ d) : hilbertFunction d = 0 := by
  rw [canonical_piece_dimension]
  simp [expectedHilbert, show d ≠ 0 by omega, show d ≠ 1 by omega,
    show d ≠ 2 by omega, show d ≠ 3 by omega]

theorem zero_piece_iff (d : ℕ) : hilbertFunction d = 0 ↔ 4 ≤ d := by
  constructor
  · intro hz
    by_contra hd
    have hlt : d < 4 := by omega
    interval_cases d <;> simp [canonical_piece_dimension, expectedHilbert] at hz
  · exact high_degrees_zero d

/-- A direct control against replacing graded dimensions by total length. -/
theorem graded_dimension_not_total_dimension :
    hilbertFunction 0 ≠ Module.finrank F A := by
  rw [low_degrees.1, actual_coordinate_algebra_dimension]
  decide

#print axioms degreeMap_kernel_mem
#print axioms canonical_piece_dimension
#print axioms sourceIdeal_homogeneous
#print axioms graded_dimension_not_total_dimension
end
end JVGradedHilbertAudit
