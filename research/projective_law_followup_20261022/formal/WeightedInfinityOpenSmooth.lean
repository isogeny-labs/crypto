import Common22SeparateChartMap
import Mathlib.RingTheory.RingHom.Smooth

/-!
# Smoothness transport on the verified genus-2 chart overlap

The open infinity chart D(z) and open affine chart D(x) are isomorphic
as K-algebras. This theorem transports a smoothness certificate across that
isomorphism. It does not establish smoothness of the affine ring itself.
-/

set_option autoImplicit false
noncomputable section
namespace WeightedInfinityOpenSmooth
open Common22SeparateChartMap
variable {K : Type*} [Field K]

/-- Smoothness of the D(x) affine overlap transports to the D(z) infinity
overlap through the checked separate-chart algebra equivalence. -/
theorem infinity_open_ring_smooth_of_affine_open
    (a3 a2 a1 a0 : K)
    (haff : RingHom.Smooth (algebraMap K (AffineOpen a3 a2 a1 a0))) :
    RingHom.Smooth (algebraMap K (InfinityOpen a3 a2 a1 a0)) := by
  haveI : Algebra.Smooth K (AffineOpen a3 a2 a1 a0) :=
    (RingHom.smooth_algebraMap).mp haff
  haveI : Algebra.Smooth K (InfinityOpen a3 a2 a1 a0) :=
    Algebra.Smooth.of_equiv (separateChartOpenEquiv a3 a2 a1 a0)
  exact (RingHom.smooth_algebraMap).mpr inferInstance

#print axioms WeightedInfinityOpenSmooth.infinity_open_ring_smooth_of_affine_open
end WeightedInfinityOpenSmooth
