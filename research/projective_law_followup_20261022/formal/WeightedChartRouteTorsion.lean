import Mathlib.RingTheory.GradedAlgebra.HomogeneousLocalization
set_option autoImplicit false
noncomputable section
namespace WeightedChartRoute
variable {ι A σ : Type*} [CommRing A] [SetLike σ A]
  [AddSubgroupClass σ A] [AddCommMonoid ι] [DecidableEq ι]
variable (𝒜 : ι → σ) [GradedRing 𝒜] {f : A} {d : ι}

theorem away_mk_eq_zero_of_torsion (hf : f ∈ 𝒜 d) (n : ℕ)
    (x : A) (hx : x ∈ 𝒜 (n • d))
    (ht : ∃ k : ℕ, f ^ k * x = 0) :
    HomogeneousLocalization.Away.mk 𝒜 hf n x hx = 0 := by
  apply (HomogeneousLocalization.ext_iff_val _ _).2
  rw [HomogeneousLocalization.Away.val_mk, HomogeneousLocalization.val_zero]
  rw [Localization.mk_eq_mk', IsLocalization.mk'_eq_zero_iff]
  obtain ⟨k, hk⟩ := ht
  exact ⟨⟨f ^ k, ⟨k, rfl⟩⟩, hk⟩
#print axioms WeightedChartRoute.away_mk_eq_zero_of_torsion
end WeightedChartRoute
namespace WeightedChartRoute
variable {ι A σ B : Type*} [CommRing A] [CommRing B] [SetLike σ A]
  [AddSubgroupClass σ A] [AddCommMonoid ι] [DecidableEq ι]
variable (𝒜 : ι → σ) [GradedRing 𝒜] (g : A →+* B) {f : A}

def awayToTarget (hfunit : IsUnit (g f)) :
    HomogeneousLocalization.Away 𝒜 f →+* B :=
  (Localization.awayLift g f hfunit).comp
    (algebraMap (HomogeneousLocalization.Away 𝒜 f) (Localization.Away f))

#check Localization.awayLift_mk
#print axioms WeightedChartRoute.awayToTarget
end WeightedChartRoute
namespace WeightedChartRoute
variable {ι A σ B : Type*} [CommRing A] [CommRing B] [SetLike σ A]
  [AddSubgroupClass σ A] [AddCommMonoid ι] [DecidableEq ι]
variable (𝒜 : ι → σ) [GradedRing 𝒜] (g : A →+* B) {f : A}

def awayToOne (h : g f = 1) : HomogeneousLocalization.Away 𝒜 f →+* B :=
  (Localization.awayLift g f (isUnit_iff_exists_inv.mpr ⟨1, by simpa [h]⟩)).comp
    (algebraMap (HomogeneousLocalization.Away 𝒜 f) (Localization.Away f))

theorem awayToOne_mk {d : ι} (hf : f ∈ 𝒜 d) (h : g f = 1)
    (n : ℕ) (x : A) (hx : x ∈ 𝒜 (n • d)) :
    awayToOne 𝒜 g h (HomogeneousLocalization.Away.mk 𝒜 hf n x hx) = g x := by
  simpa [awayToOne, HomogeneousLocalization.algebraMap_apply,
    HomogeneousLocalization.Away.val_mk] using
    (Localization.awayLift_mk g f x (1 : B) (by simpa [h]) n)
#print axioms WeightedChartRoute.awayToOne_mk
end WeightedChartRoute
