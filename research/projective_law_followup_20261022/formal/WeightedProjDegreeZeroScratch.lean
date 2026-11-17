import Common22WeightedProjKernel

set_option autoImplicit false
noncomputable section
namespace WeightedProjDegreeZeroScratch
open MvPolynomial Common22WeightedProjHead Common22WeightedProjQuotient
variable {K : Type*} [Field K]

-- Every point of the ordinary quotient admits a finite weighted-homogeneous lift.
theorem weightedRing_finite_homogeneous_lift (a3 a2 a1 a0 : K)
    (q : WeightedRing a3 a2 a1 a0) :
    ∃ (s : Finset ℕ) (p : ℕ → Poly K),
      (∀ i ∈ s, IsWeightedHomogeneous weights (p i) i) ∧
      q = ∑ i ∈ s, Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (p i) := by
  obtain ⟨f, rfl⟩ := Ideal.Quotient.mk_surjective q
  classical
  have hs : (Function.support fun i : ℕ => weightedHomogeneousComponent weights i f).Finite :=
    weightedHomogeneousComponent_finsupp f
  let s : Finset ℕ := hs.toFinset
  refine ⟨s, fun i => weightedHomogeneousComponent weights i f, ?_, ?_⟩
  · intro i hi
    exact weightedHomogeneousComponent_isWeightedHomogeneous (w := weights) i f
  · have hf : (∑ i ∈ s, weightedHomogeneousComponent weights i f) = f := by
      simpa only [s, finsum_eq_sum _ hs] using
        (MvPolynomial.sum_weightedHomogeneousComponent weights f)
    calc
      Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) f =
          Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0)
            (∑ i ∈ s, weightedHomogeneousComponent weights i f) := by rw [hf]
      _ = ∑ i ∈ s, Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0)
            (weightedHomogeneousComponent weights i f) := by rw [map_sum]


-- Quotient equality can be checked degree by degree; this is the descent condition.
theorem weightedRing_eq_iff_components (a3 a2 a1 a0 : K) (f g : Poly K) :
    Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) f =
      Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) g ↔
      ∀ n : ℕ,
        Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0)
          (weightedHomogeneousComponent weights n f) =
        Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0)
          (weightedHomogeneousComponent weights n g) := by
  rw [Ideal.Quotient.eq]
  simp_rw [Ideal.Quotient.eq]
  rw [MvPolynomial.mem_iff_weightedHomogeneousComponent_mem K weights
    (relationIdeal_homogeneous a3 a2 a1 a0) (f - g)]
  simp only [map_sub]

#print axioms weightedRing_eq_iff_components

#print axioms weightedRing_finite_homogeneous_lift
end WeightedProjDegreeZeroScratch
