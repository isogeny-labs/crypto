import JVConePoints

namespace JVLocalMultiplicityAudit
noncomputable section
open JVHilbertAudit

/-- The literal first homogeneous source quotient, with actual chart controls.
The general multihomogeneous intersection bound is not asserted here. -/
theorem jv_local_multiplicity_certificate :
    Module.length A A = 8 ∧
    IsLocalRing A ∧
    originIdeal.IsMaximal ∧
    Nonempty (A ⧸ originIdeal ≃+* F) ∧
    (∀ p : PrimeSpectrum A, p = originPoint) ∧
    IsOpen ({originPoint} : Set (PrimeSpectrum A)) ∧
    Nat.card (PrimeSpectrum A) = 1 ∧
    Function.Bijective (algebraMap A OriginLocalization) ∧
    Module.length OriginLocalization OriginLocalization = 8 ∧
    IsLocalRing OriginLocalization ∧
    Nonempty (ActualLocalizedResidue ≃+* F) ∧
    h ^ 3 ≠ 0 ∧ h ^ 4 = 0 ∧ ¬ IsReduced A ∧
    Subsingleton FirstChart ∧ Module.length FirstChart FirstChart = 0 ∧
    ¬ IsLocalRing FirstChart ∧
    Subsingleton FirstOpen ∧ Module.length FirstOpen FirstOpen = 0 ∧
    ¬ IsLocalRing FirstOpen ∧
    secondF0 (69 : F) 75 1 = 0 ∧ secondF1 (69 : F) 75 1 = 0 ∧
    secondF2 (69 : F) 75 1 = 0 ∧
    Nontrivial SecondChart ∧ Nontrivial SecondOpen ∧
    Module.length A A ≠ Nat.card (ConeSolutions F) ∧
    ∀ (L : Type) [Field L] [CharP L 101], Nat.card (ConeSolutions L) = 1 := by
  exact ⟨actual_ring_length, actual_local, origin_maximal, ⟨originResidueEquiv⟩,
    prime_unique, origin_isolated, spectrum_card, localization_map_bijective,
    actual_localization_length, localization_local, ⟨localizedResidueEquiv⟩,
    h_cube_nonzero, h_four_zero, actual_not_reduced,
    first_chart_subsingleton, first_chart_length, first_chart_not_local,
    first_open_subsingleton, first_open_length, first_open_not_local,
    second_source_root.1, second_source_root.2.1, second_source_root.2.2,
    second_chart_nontrivial, second_open_nontrivial, length_not_point_count,
    fun L _ _ => cone_card L⟩

#print axioms jv_local_multiplicity_certificate
end
end JVLocalMultiplicityAudit
