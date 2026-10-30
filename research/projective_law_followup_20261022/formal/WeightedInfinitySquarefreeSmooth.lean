import WeightedAffineSquarefreeFinal
import WeightedInfinityOpenSmooth
import WeightedInfinityJacobian
import WeightedChartRingSmooth

set_option autoImplicit false
noncomputable section
namespace WeightedInfinitySquarefreeSmooth

open Common22SeparateChartMap
variable {K : Type*} [Field K] [PerfectField K]

theorem affine_open_ring_smooth_of_squarefree (a3 a2 a1 a0 : K)
    (h2 : (2 : K) ≠ 0)
    (hsq : Squarefree (WeightedProjSmoothPoints.quintic a3 a2 a1 a0)) :
    RingHom.Smooth (algebraMap K (AffineOpen a3 a2 a1 a0)) := by
  haveI : Algebra.Smooth K (AffineRing a3 a2 a1 a0) :=
    (RingHom.smooth_algebraMap).mp
      (WeightedAffineSquarefreeFinal.affine_ring_smooth_of_squarefree
        a3 a2 a1 a0 h2 hsq)
  haveI : Algebra.Smooth (AffineRing a3 a2 a1 a0)
      (AffineOpen a3 a2 a1 a0) :=
    Algebra.Smooth.of_isLocalization_Away (xbar a3 a2 a1 a0)
  haveI : Algebra.Smooth K (AffineOpen a3 a2 a1 a0) :=
    Algebra.Smooth.comp K (AffineRing a3 a2 a1 a0) (AffineOpen a3 a2 a1 a0)
  exact (RingHom.smooth_algebraMap).mpr inferInstance

theorem infinity_open_ring_smooth_of_squarefree (a3 a2 a1 a0 : K)
    (h2 : (2 : K) ≠ 0)
    (hsq : Squarefree (WeightedProjSmoothPoints.quintic a3 a2 a1 a0)) :
    RingHom.Smooth (algebraMap K (InfinityOpen a3 a2 a1 a0)) :=
  WeightedInfinityOpenSmooth.infinity_open_ring_smooth_of_affine_open
    a3 a2 a1 a0 (affine_open_ring_smooth_of_squarefree a3 a2 a1 a0 h2 hsq)

/-- The infinity chart is covered by the smooth principal opens at `z` and
at its z-partial derivative. -/
theorem infinity_ring_smooth_of_squarefree (a3 a2 a1 a0 : K)
    (h2 : (2 : K) ≠ 0)
    (hsq : Squarefree (WeightedProjSmoothPoints.quintic a3 a2 a1 a0)) :
    RingHom.Smooth
      (algebraMap K (Common22ExplicitChart.ChartRing a3 a2 a1 a0)) := by
  apply RingHom.Smooth.ofLocalizationSpanTarget
    (algebraMap K (Common22ExplicitChart.ChartRing a3 a2 a1 a0))
    ({Common22ExplicitChart.zbar a3 a2 a1 a0,
      WeightedInfinityJacobian.dZ a3 a2 a1 a0} :
      Set (Common22ExplicitChart.ChartRing a3 a2 a1 a0))
    (WeightedInfinityJacobian.infinity_jacobian_ideal_top a3 a2 a1 a0)
  intro r
  change RingHom.Smooth
    (algebraMap K (Localization.Away (r : Common22ExplicitChart.ChartRing a3 a2 a1 a0)))
  have hr := r.property
  simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hr
  rcases hr with hz | hd
  · rw [hz]
    exact infinity_open_ring_smooth_of_squarefree a3 a2 a1 a0 h2 hsq
  · rw [hd]
    exact WeightedChartRingSmooth.infinity_derivative_open_ring_smooth a3 a2 a1 a0

#print axioms WeightedInfinitySquarefreeSmooth.affine_open_ring_smooth_of_squarefree
#print axioms WeightedInfinitySquarefreeSmooth.infinity_open_ring_smooth_of_squarefree
#print axioms WeightedInfinitySquarefreeSmooth.infinity_ring_smooth_of_squarefree
end WeightedInfinitySquarefreeSmooth
