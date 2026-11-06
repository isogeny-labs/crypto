import WeightedAffineXPartial

set_option autoImplicit false
noncomputable section
namespace WeightedAffineSquarefreeFinal

variable {K : Type*} [Field K] [PerfectField K]

/-- The affine chart of the squarefree odd-characteristic quintic curve is
smooth over its coefficient field. -/
theorem affine_ring_smooth_of_squarefree (a3 a2 a1 a0 : K)
    (h2 : (2 : K) ≠ 0)
    (hsq : Squarefree (WeightedProjSmoothPoints.quintic a3 a2 a1 a0)) :
    RingHom.Smooth
      (algebraMap K (Common22SeparateChartMap.AffineRing a3 a2 a1 a0)) := by
  apply WeightedAffineRingSmooth.ring_smooth_of_jacobian_span_top
  exact WeightedAffineSquarefreeSmooth.affine_deriv_span_top_of_ids
    a3 a2 a1 a0 h2 hsq
    (WeightedAffineXPartial.affine_x_partial a3 a2 a1 a0)
    (WeightedAffineXPartial.affine_y_partial a3 a2 a1 a0)

#print axioms WeightedAffineSquarefreeFinal.affine_ring_smooth_of_squarefree
end WeightedAffineSquarefreeFinal
