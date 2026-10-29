import WeightedRingGraded
import WeightedChartKernelRec
import WeightedChartRoute

set_option autoImplicit false
noncomputable section
namespace WeightedChartIso
open MvPolynomial Common22WeightedProjHead Common22WeightedProjQuotient
  Common22WeightedProjKernel WeightedRingComponent WeightedRingGraded
variable {K : Type*} [Field K]

theorem z_homogeneous (a3 a2 a1 a0 : K) :
    zbar a3 a2 a1 a0 ∈ degreeSubmodule a3 a2 a1 a0 1 := by
  change component a3 a2 a1 a0 1 (zbar a3 a2 a1 a0) =
    zbar a3 a2 a1 a0
  unfold zbar
  have hz : IsWeightedHomogeneous weights (X (2 : Fin 3) : Poly K) 1 := by
    simpa [weights] using (isWeightedHomogeneous_X (R := K) weights (2 : Fin 3))
  simpa using (component_homogeneous a3 a2 a1 a0 1 1 (X (2 : Fin 3)) hz)

def weightedAwayToAffine (a3 a2 a1 a0 : K) :
    HomogeneousLocalization.Away (degreeSubmodule a3 a2 a1 a0)
      (zbar a3 a2 a1 a0) →+* AffineRing a3 a2 a1 a0 :=
  WeightedChartRoute.awayToOne (degreeSubmodule a3 a2 a1 a0)
    (toAffine a3 a2 a1 a0).toRingHom (by
      simpa [zbar] using toAffine_Z a3 a2 a1 a0)

theorem weightedAwayToAffine_surjective (a3 a2 a1 a0 : K) :
    Function.Surjective (weightedAwayToAffine a3 a2 a1 a0) := by
  exact WeightedChartRoute.awayToOne_surjective
    (degreeSubmodule a3 a2 a1 a0) (toAffine a3 a2 a1 a0).toRingHom
    (z_homogeneous a3 a2 a1 a0)
    (by simpa [zbar] using toAffine_Z a3 a2 a1 a0)
    (toAffine_surjective a3 a2 a1 a0)

theorem weightedAwayToAffine_injective (a3 a2 a1 a0 : K) :
    Function.Injective (weightedAwayToAffine a3 a2 a1 a0) := by
  apply WeightedChartRoute.awayToOne_injective
    (degreeSubmodule a3 a2 a1 a0) (toAffine a3 a2 a1 a0).toRingHom
    (z_homogeneous a3 a2 a1 a0)
    (by simpa [zbar] using toAffine_Z a3 a2 a1 a0)
  intro n a ha hga
  have hmem : a ∈ zMinusOneIdeal a3 a2 a1 a0 := by
    rw [← exact_kernel a3 a2 a1 a0]
    exact RingHom.mem_ker.mpr hga
  rw [zMinusOneIdeal, Ideal.mem_span_singleton] at hmem
  obtain ⟨q, hq⟩ := hmem
  exact WeightedChartKernelRec.homogeneous_kernel_saturation
    (degreeSubmodule a3 a2 a1 a0)
    (z_homogeneous a3 a2 a1 a0) ha hq

def weightedAwayEquivAffine (a3 a2 a1 a0 : K) :
    HomogeneousLocalization.Away (degreeSubmodule a3 a2 a1 a0)
      (zbar a3 a2 a1 a0) ≃+* AffineRing a3 a2 a1 a0 :=
  RingEquiv.ofBijective (weightedAwayToAffine a3 a2 a1 a0)
    ⟨weightedAwayToAffine_injective a3 a2 a1 a0,
      weightedAwayToAffine_surjective a3 a2 a1 a0⟩

#print axioms weightedAwayEquivAffine

#print axioms weightedAwayToAffine_injective
#print axioms weightedAwayToAffine_surjective
end WeightedChartIso
