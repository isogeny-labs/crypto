import WeightedChartKernelRec
import WeightedChartRoute
import Mathlib.RingTheory.Ideal.Quotient.Operations
set_option autoImplicit false
noncomputable section
namespace WeightedChartDehomog
variable {A σ : Type*} [CommRing A] [SetLike σ A] [AddSubgroupClass σ A]
variable (𝒜 : ℕ → σ) [GradedRing 𝒜] (f : A)

def oneIdeal : Ideal A := Ideal.span {f-1}

abbrev oneQuotient := A ⧸ oneIdeal f

def quotientMap : A →+* oneQuotient f := Ideal.Quotient.mk (oneIdeal f)

theorem quotientMap_f : quotientMap f f = 1 := by
  have h : (Ideal.Quotient.mk (oneIdeal f)) (f-1) = 0 :=
    Ideal.Quotient.eq_zero_iff_mem.mpr (Ideal.subset_span (by simp))
  simpa [quotientMap, oneIdeal, map_sub] using sub_eq_zero.mp h

def awayEquivQuotient (hf : f ∈ 𝒜 1) :
    HomogeneousLocalization.Away 𝒜 f ≃+* oneQuotient f :=
  RingEquiv.ofBijective (WeightedChartRoute.awayToOne 𝒜 (quotientMap f) (quotientMap_f f))
    ⟨WeightedChartRoute.awayToOne_injective 𝒜 (quotientMap f) hf (quotientMap_f f) (by
      intro n a ha hga
      have hmem : a ∈ oneIdeal f :=
        Ideal.Quotient.eq_zero_iff_mem.mp hga
      rw [oneIdeal, Ideal.mem_span_singleton] at hmem
      obtain ⟨q, hq⟩ := hmem
      exact WeightedChartKernelRec.homogeneous_kernel_saturation 𝒜 hf ha hq),
     WeightedChartRoute.awayToOne_surjective 𝒜 (quotientMap f) hf (quotientMap_f f)
       (Ideal.Quotient.mk_surjective)⟩

#print axioms awayEquivQuotient
end WeightedChartDehomog
