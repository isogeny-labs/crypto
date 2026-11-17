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
namespace WeightedChartRoute
open DirectSum
variable {A σ B : Type*} [CommRing A] [CommRing B] [SetLike σ A]
  [AddSubgroupClass σ A]
variable (𝒜 : ℕ → σ) [GradedRing 𝒜] (g : A →+* B) {f : A}

theorem awayToOne_surjective (hf : f ∈ 𝒜 1) (h : g f = 1)
    (hg : Function.Surjective g) :
    Function.Surjective (awayToOne 𝒜 g h) := by
  intro b
  obtain ⟨a, rfl⟩ := hg b
  classical
  let s : Finset ℕ := (decompose 𝒜 a).support
  let t : ℕ → HomogeneousLocalization.Away 𝒜 f := fun i =>
    HomogeneousLocalization.Away.mk 𝒜 hf i (decompose 𝒜 a i : A)
      (by simpa using (decompose 𝒜 a i).property)
  refine ⟨∑ i ∈ s, t i, ?_⟩
  rw [map_sum]
  simp only [t, awayToOne_mk]
  rw [← map_sum]
  congr 1
  exact DirectSum.sum_support_decompose 𝒜 a
#print axioms WeightedChartRoute.awayToOne_surjective
end WeightedChartRoute
namespace WeightedChartRoute
variable {A σ B : Type*} [CommRing A] [CommRing B] [SetLike σ A]
  [AddSubgroupClass σ A]
variable (𝒜 : ℕ → σ) [GradedRing 𝒜] (g : A →+* B) {f : A}

theorem awayToOne_injective (hf : f ∈ 𝒜 1) (h : g f = 1)
    (hker : ∀ (n : ℕ) (a : A), a ∈ 𝒜 (n • (1 : ℕ)) →
      g a = 0 → ∃ k : ℕ, f ^ k * a = 0) :
    Function.Injective (awayToOne 𝒜 g h) := by
  have hzero (x : HomogeneousLocalization.Away 𝒜 f)
      (hx : awayToOne 𝒜 g h x = 0) : x = 0 := by
    obtain ⟨n, a, ha, rfl⟩ :=
      HomogeneousLocalization.Away.mk_surjective 𝒜 hf x
    have hga : g a = 0 := by simpa only [awayToOne_mk] using hx
    exact away_mk_eq_zero_of_torsion 𝒜 hf n a ha (hker n a ha hga)
  intro x y hxy
  apply sub_eq_zero.mp
  apply hzero
  rw [map_sub, hxy, sub_self]
#print axioms WeightedChartRoute.awayToOne_injective
end WeightedChartRoute
