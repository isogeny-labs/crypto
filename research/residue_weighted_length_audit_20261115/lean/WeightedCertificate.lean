import SchemeWeights

namespace ResidueWeightedLengthAudit
noncomputable section

/-- Established residue-weighted finite-algebra accounting with actual controls.
This does not assert a projective multihomogeneous intersection bound. -/
theorem residue_weighted_length_certificate :
    (∀ (K A : Type) [Field K] [CommRing A] [Algebra K A] [Module.Finite K A],
      Module.finrank K A = weightedDegree K A) ∧
    (∀ (K A : Type) [Field K] [CommRing A] [Algebra K A] [Module.Finite K A],
      Module.finrank K A = schemeWeightedDegree K A) ∧
    (∀ (R : Type) [CommRing R] (p : PrimeSpectrum R),
      Nonempty (p.asIdeal.ResidueField ≃+* IsLocalRing.ResidueField (ActualAffineStalk R p))) ∧
    (∀ (K R : Type) [Field K] [CommRing R] [Algebra K R] (p : PrimeSpectrum R),
      Nonempty (p.asIdeal.ResidueField ≃ₐ[K] IsLocalRing.ResidueField (ActualAffineStalk R p))) ∧
    (∀ (K B M : Type) [Field K] [CommRing B] [IsLocalRing B] [Algebra K B]
      [Module.Finite K B] [AddCommGroup M] [Module K M] [Module B M]
      [IsScalarTower K B M], Module.length K M =
        Module.length B M * Module.finrank K (IsLocalRing.ResidueField B)) ∧
    (∀ (K A : Type) [Field K] [CommRing A] [Algebra K A] [Module.Finite K A]
      [IsReduced A], unweightedDegree K A = Nat.card (PrimeSpectrum A)) ∧
    Module.finrank Base ProductAlgebra = 3 ∧
    Module.length ProductAlgebra ProductAlgebra = 2 ∧
    Nat.card (PrimeSpectrum ProductAlgebra) = 2 ∧
    (∀ p : PrimeSpectrum ProductAlgebra, p = firstPoint ∨ p = secondPoint) ∧
    firstPoint ≠ secondPoint ∧
    (∀ p : PrimeSpectrum ProductAlgebra,
      Module.length (Localization.AtPrime p.asIdeal) (Localization.AtPrime p.asIdeal) = 1) ∧
    Nonempty (firstIdeal.ResidueField ≃ₐ[Base] Base) ∧
    Nonempty (secondIdeal.ResidueField ≃ₐ[Base] Quad) ∧
    Module.finrank Base firstIdeal.ResidueField = 1 ∧
    Module.finrank Base secondIdeal.ResidueField = 2 ∧
    weightedDegree Base ProductAlgebra = 3 ∧
    unweightedDegree Base ProductAlgebra = 2 ∧
    weightedDegree Base ProductAlgebra ≠ unweightedDegree Base ProductAlgebra ∧
    ¬ IsLocalRing ProductAlgebra ∧
    weightedDegree Base (Base × Base) = 2 ∧
    unweightedDegree Base (Base × Base) = 2 ∧
    weightedDegree JVHilbertAudit.F JVHilbertAudit.A = 8 ∧
    Module.finrank JVHilbertAudit.F JVLocalMultiplicityAudit.originIdeal.ResidueField = 1 ∧
    weightedDegree JVHilbertAudit.F JVLocalMultiplicityAudit.FirstChart = 0 := by
  exact ⟨fun K A _ _ _ _ => scalar_dimension_eq_weighted_degree K A,
    fun K A _ _ _ _ => scalar_dimension_eq_scheme_weighted_degree K A,
    fun R _ p => ⟨affineStalkResidueEquiv R p⟩,
    fun K R _ _ _ p => ⟨affineStalkResidueAlgEquiv K R p⟩,
    fun K B M _ _ _ _ _ _ _ _ _ => local_module_weight K B M,
    fun K A _ _ _ _ _ => reduced_unweighted_degree K A,
    product_scalar_dimension, product_ring_length, product_prime_card,
    product_prime_classification, product_points_distinct, product_local_length_one,
    ⟨firstResidueEquiv⟩, ⟨secondResidueEquiv⟩,
    product_residue_degrees.1, product_residue_degrees.2,
    product_weighted_degree, product_unweighted_degree, deleting_residue_weights_fails,
    product_not_local, split_product_weighted, split_product_unweighted,
    JVCalibration.actual_jv_weighted_degree, JVCalibration.actual_jv_residue_degree,
    JVCalibration.actual_zero_chart_weighted_degree⟩

#print axioms residue_weighted_length_certificate
end
end ResidueWeightedLengthAudit
