import JVWeights

namespace ResidueWeightedLengthAudit
noncomputable section

abbrev ActualAffineStalk (R : Type*) [CommRing R] (p : PrimeSpectrum R) :=
  (AlgebraicGeometry.Spec (CommRingCat.of R)).presheaf.stalk p

instance affineStalkAlgebra (R : Type*) [CommRing R] (p : PrimeSpectrum R) :
    Algebra R (ActualAffineStalk R p) :=
  AlgebraicGeometry.StructureSheaf.stalkAlgebra R p

def affineStalkEquiv (R : Type*) [CommRing R] (p : PrimeSpectrum R) :
    Localization.AtPrime p.asIdeal ≃ₐ[R] ActualAffineStalk R p :=
  AlgebraicGeometry.StructureSheaf.stalkIso R p

instance affineStalkLocal (R : Type*) [CommRing R] (p : PrimeSpectrum R) :
    IsLocalRing (ActualAffineStalk R p) :=
  (affineStalkEquiv R p).toRingEquiv.isLocalRing

/-- The actual coefficient composite gives the base algebra; direct R-scalars take priority. -/
instance (priority := 100) affineStalkBaseAlgebra (K R : Type*) [CommRing K] [CommRing R]
    [Algebra K R] (p : PrimeSpectrum R) : Algebra K (ActualAffineStalk R p) :=
  ((algebraMap R (ActualAffineStalk R p)).comp (algebraMap K R)).toAlgebra

instance affineStalkBaseTower (K R : Type*) [CommRing K] [CommRing R] [Algebra K R]
    (p : PrimeSpectrum R) : IsScalarTower K R (ActualAffineStalk R p) :=
  IsScalarTower.of_algebraMap_eq' rfl

def affineStalkResidueEquiv (R : Type*) [CommRing R] (p : PrimeSpectrum R) :
    p.asIdeal.ResidueField ≃+* IsLocalRing.ResidueField (ActualAffineStalk R p) :=
  IsLocalRing.ResidueField.mapEquiv (affineStalkEquiv R p).toRingEquiv

def affineStalkResidueAlgEquiv (K R : Type*) [CommRing K] [CommRing R] [Algebra K R]
    (p : PrimeSpectrum R) :
    p.asIdeal.ResidueField ≃ₐ[K] IsLocalRing.ResidueField (ActualAffineStalk R p) :=
  IsLocalRing.ResidueField.mapAlgEquiv ((affineStalkEquiv R p).restrictScalars K)

theorem artinian_stalk_coefficient_surjective (R : Type*) [CommRing R] [IsArtinianRing R]
    (p : PrimeSpectrum R) : Function.Surjective (algebraMap R (ActualAffineStalk R p)) := by
  intro z
  obtain ⟨v, hv⟩ := (affineStalkEquiv R p).surjective z
  obtain ⟨a, ha⟩ := IsArtinianRing.localization_surjective p.asIdeal.primeCompl
    (Localization.AtPrime p.asIdeal) v
  refine ⟨a, ?_⟩
  calc
    algebraMap R (ActualAffineStalk R p) a =
        affineStalkEquiv R p (algebraMap R (Localization.AtPrime p.asIdeal) a) :=
      ((affineStalkEquiv R p).commutes a).symm
    _ = affineStalkEquiv R p v := congrArg (affineStalkEquiv R p) ha
    _ = z := hv

theorem actual_stalk_self_length (R : Type*) [CommRing R] [IsArtinianRing R]
    (p : PrimeSpectrum R) : Module.length (ActualAffineStalk R p) (ActualAffineStalk R p) =
      Module.length (Localization.AtPrime p.asIdeal) (Localization.AtPrime p.asIdeal) := by
  have hs := Module.length_eq_of_surjective (S := R) (R := ActualAffineStalk R p)
    (M := ActualAffineStalk R p) (artinian_stalk_coefficient_surjective R p)
  have hl := Module.length_eq_of_surjective (S := R) (R := Localization.AtPrime p.asIdeal)
    (M := Localization.AtPrime p.asIdeal)
    (IsArtinianRing.localization_surjective p.asIdeal.primeCompl (Localization.AtPrime p.asIdeal))
  calc
    Module.length (ActualAffineStalk R p) (ActualAffineStalk R p) =
        Module.length R (ActualAffineStalk R p) := hs.symm
    _ = Module.length R (Localization.AtPrime p.asIdeal) :=
      (affineStalkEquiv R p).toLinearEquiv.length_eq.symm
    _ = Module.length (Localization.AtPrime p.asIdeal) (Localization.AtPrime p.asIdeal) := hl

variable (K A : Type*) [Field K] [CommRing A] [Algebra K A] [Module.Finite K A]

def schemeWeightedDegree : ℕ := by
  let : IsArtinianRing A := finite_algebra_artinian K A
  let : Fintype (PrimeSpectrum A) := Fintype.ofFinite _
  exact ∑ p : PrimeSpectrum A,
    (Module.length (ActualAffineStalk A p) (ActualAffineStalk A p)).toNat *
      Module.finrank K (IsLocalRing.ResidueField (ActualAffineStalk A p))

theorem scheme_weighted_degree_eq_localizations : schemeWeightedDegree K A = weightedDegree K A := by
  classical
  let : IsArtinianRing A := finite_algebra_artinian K A
  let : Fintype (PrimeSpectrum A) := Fintype.ofFinite _
  change (∑ p : PrimeSpectrum A,
    (Module.length (ActualAffineStalk A p) (ActualAffineStalk A p)).toNat *
      Module.finrank K (IsLocalRing.ResidueField (ActualAffineStalk A p))) = _
  unfold weightedDegree
  apply Finset.sum_congr rfl
  intro p _
  rw [actual_stalk_self_length A p,
    ← (affineStalkResidueAlgEquiv K A p).toLinearEquiv.finrank_eq]

theorem scalar_dimension_eq_scheme_weighted_degree :
    Module.finrank K A = schemeWeightedDegree K A :=
  (scalar_dimension_eq_weighted_degree K A).trans
    (scheme_weighted_degree_eq_localizations K A).symm

#print axioms affineStalkResidueEquiv
#print axioms actual_stalk_self_length
#print axioms scalar_dimension_eq_scheme_weighted_degree
end
end ResidueWeightedLengthAudit
