import LocalWeights

namespace ResidueWeightedLengthAudit
noncomputable section

variable (K A : Type*) [Field K] [CommRing A] [Algebra K A] [Module.Finite K A]

include K in
theorem finite_algebra_artinian : IsArtinianRing A :=
  isArtinian_of_tower K inferInstance

theorem localization_finite (p : PrimeSpectrum A) :
    Module.Finite K (Localization.AtPrime p.asIdeal) := by
  let : IsArtinianRing A := finite_algebra_artinian K A
  exact Module.Finite.of_surjective
    (Algebra.algHom K A (Localization.AtPrime p.asIdeal)).toLinearMap
    (IsArtinianRing.localization_surjective p.asIdeal.primeCompl (Localization.AtPrime p.asIdeal))

/-- Actual canonical local lengths, weighted by the actual prime-residue degrees.
The finite prime population is derived, not supplied as a list. -/
def weightedDegree : ℕ := by
  let : IsArtinianRing A := finite_algebra_artinian K A
  let : Fintype (PrimeSpectrum A) := Fintype.ofFinite _
  exact ∑ p : PrimeSpectrum A,
    (Module.length (Localization.AtPrime p.asIdeal) (Localization.AtPrime p.asIdeal)).toNat *
      Module.finrank K p.asIdeal.ResidueField

theorem scalar_dimension_eq_weighted_degree : Module.finrank K A = weightedDegree K A := by
  classical
  let : IsArtinianRing A := finite_algebra_artinian K A
  let : Fintype (PrimeSpectrum A) := Fintype.ofFinite _
  change Module.finrank K A = ∑ p : PrimeSpectrum A,
    (Module.length (Localization.AtPrime p.asIdeal) (Localization.AtPrime p.asIdeal)).toNat *
      Module.finrank K p.asIdeal.ResidueField
  rw [IsArtinianRing.finrank_eq_sum_primeSpectrum A K]
  apply Finset.sum_congr rfl
  intro p _
  let : Module.Finite K (Localization.AtPrime p.asIdeal) := localization_finite K A p
  exact local_self_dimension K (Localization.AtPrime p.asIdeal)

theorem zero_algebra_weighted_degree [Subsingleton A] : weightedDegree K A = 0 := by
  rw [← scalar_dimension_eq_weighted_degree K A]
  exact Module.finrank_zero_of_subsingleton

def unweightedDegree : ℕ := by
  let : IsArtinianRing A := finite_algebra_artinian K A
  let : Fintype (PrimeSpectrum A) := Fintype.ofFinite _
  exact ∑ p : PrimeSpectrum A,
    (Module.length (Localization.AtPrime p.asIdeal) (Localization.AtPrime p.asIdeal)).toNat

include K in
theorem reduced_prime_local_length [IsReduced A] (p : PrimeSpectrum A) :
    Module.length (Localization.AtPrime p.asIdeal) (Localization.AtPrime p.asIdeal) = 1 := by
  let : IsArtinianRing A := finite_algebra_artinian K A
  let : Field (Localization.AtPrime p.asIdeal) :=
    (IsArtinianRing.isField_of_isReduced_of_isLocalRing
      (R := Localization.AtPrime p.asIdeal)).toField
  rw [Module.length_eq_finrank, Module.finrank_self]
  rfl

theorem reduced_unweighted_degree [IsReduced A] :
    unweightedDegree K A = Nat.card (PrimeSpectrum A) := by
  classical
  let : IsArtinianRing A := finite_algebra_artinian K A
  let : Fintype (PrimeSpectrum A) := Fintype.ofFinite _
  change (∑ p : PrimeSpectrum A,
    (Module.length (Localization.AtPrime p.asIdeal) (Localization.AtPrime p.asIdeal)).toNat) = _
  simp only [reduced_prime_local_length K A, ENat.toNat_one]
  simp

#print axioms localization_finite
#print axioms scalar_dimension_eq_weighted_degree
end
end ResidueWeightedLengthAudit
