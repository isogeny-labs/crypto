import JVOrigin

namespace JVLocalMultiplicityAudit
noncomputable section
open JVHilbertAudit

instance origin_prime : originIdeal.IsPrime := origin_maximal.isPrime

theorem prime_eq_origin (p : Ideal A) (hp : p.IsPrime) : p = originIdeal := by
  have hm : originIdeal ≤ p := by
    intro a ha
    obtain ⟨n, hn⟩ := origin_ideal_nilpotent a ha
    apply hp.mem_of_pow_mem n
    rw [hn]
    exact p.zero_mem
  exact (origin_maximal.eq_of_le hp.ne_top hm).symm

def originPoint : PrimeSpectrum A := ⟨originIdeal, origin_prime⟩

theorem prime_unique (p : PrimeSpectrum A) : p = originPoint := by
  apply PrimeSpectrum.ext
  exact prime_eq_origin p.asIdeal p.isPrime

def primeEquivUnit : PrimeSpectrum A ≃ PUnit.{1} where
  toFun _ := PUnit.unit
  invFun _ := originPoint
  left_inv p := (prime_unique p).symm
  right_inv p := by cases p; rfl

theorem spectrum_card : Nat.card (PrimeSpectrum A) = 1 := by
  calc
    Nat.card (PrimeSpectrum A) = Nat.card PUnit.{1} := Nat.card_congr primeEquivUnit
    _ = 1 := by simp

theorem origin_isolated : IsOpen ({originPoint} : Set (PrimeSpectrum A)) := by
  have he : ({originPoint} : Set (PrimeSpectrum A)) = Set.univ := by
    apply Set.eq_univ_of_forall
    intro p
    exact Set.mem_singleton_iff.mpr (prime_unique p)
  rw [he]
  exact isOpen_univ

abbrev OriginLocalization := Localization.AtPrime originIdeal

theorem origin_complement_units : originIdeal.primeCompl ≤ IsUnit.submonoid A := by
  intro a ha
  apply unit_of_origin_nonzero a
  intro hz
  apply ha
  exact hz

/-- Actual canonical prime localization, with all denominator-unit facts derived. -/
def originLocalEquiv : A ≃ₐ[A] OriginLocalization :=
  IsLocalization.atUnits A originIdeal.primeCompl origin_complement_units

theorem localization_map (a : A) : originLocalEquiv a = algebraMap A OriginLocalization a :=
  originLocalEquiv.commutes a

theorem localization_map_bijective : Function.Bijective (algebraMap A OriginLocalization) := by
  have he : (originLocalEquiv : A → OriginLocalization) = algebraMap A OriginLocalization := by
    funext a
    exact localization_map a
  rw [← he]
  exact originLocalEquiv.bijective

theorem actual_localization_length : Module.length OriginLocalization OriginLocalization = 8 := by
  have hs : Module.length A OriginLocalization = Module.length OriginLocalization OriginLocalization :=
    Module.length_eq_of_surjective (R := OriginLocalization) (S := A) (M := OriginLocalization)
      localization_map_bijective.2
  calc
    Module.length OriginLocalization OriginLocalization = Module.length A OriginLocalization := hs.symm
    _ = Module.length A A := originLocalEquiv.toLinearEquiv.length_eq.symm
    _ = 8 := actual_ring_length

theorem localization_local : IsLocalRing OriginLocalization := inferInstance

def localizedOrigin : OriginLocalization →+* F :=
  originEvaluation.toRingHom.comp originLocalEquiv.symm.toRingHom

theorem localized_origin_surjective : Function.Surjective localizedOrigin :=
  origin_surjective.comp originLocalEquiv.symm.surjective

local instance : IsLocalRing OriginLocalization := localization_local

abbrev ActualLocalizedResidue := IsLocalRing.ResidueField OriginLocalization

theorem localized_kernel_maximal : RingHom.ker localizedOrigin = IsLocalRing.maximalIdeal OriginLocalization :=
  IsLocalRing.ker_eq_maximalIdeal (R := OriginLocalization) (K := F)
    localizedOrigin localized_origin_surjective

def localizedResidueEquiv : ActualLocalizedResidue ≃+* F :=
  (Ideal.quotEquivOfEq localized_kernel_maximal.symm).trans
    (localizedOrigin.quotientKerEquivOfSurjective localized_origin_surjective)

#print axioms prime_eq_origin
#print axioms actual_localization_length
#print axioms localizedResidueEquiv
end
end JVLocalMultiplicityAudit
