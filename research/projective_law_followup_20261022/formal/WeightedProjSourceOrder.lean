import WeightedProjLocallyNoetherian
import WeightedProjSourceFunctionField
import Mathlib.AlgebraicGeometry.OrderOfVanishing

/-! The source-shaped common 2+2 function now has an order function on the
actual weighted-Proj curve. The pointwise value is only geometrically
meaningful after proving that its point has coheight one and the function
is nonzero; neither assertion is inferred here. -/

set_option autoImplicit false
set_option maxHeartbeats 600000
noncomputable section

namespace WeightedProjSourceOrder

open AlgebraicGeometry WeightedRingGraded Common22WeightedProjQuotient Order

variable {K : Type} [Field K]

private abbrev curve (a3 a2 a1 a0 : K) :=
  Proj (degreeSubmodule a3 a2 a1 a0)

/-- The order of the source common 2+2 rational function at any point of the
actual weighted-Proj scheme. `Scheme.ord` returns zero outside coheight one
and on the zero function, so a value alone is not a divisor certificate. -/
def sourceOrder (a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 : K)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable)
    (z : curve a3 a2 a1 a0) : ℤ := by
  letI : IsIntegral (curve a3 a2 a1 a0) :=
    WeightedProjIsIntegral.whole_curve_integral a3 a2 a1 a0 hsep
  letI : IsLocallyNoetherian (curve a3 a2 a1 a0) :=
    WeightedProjLocallyNoetherian.whole_curve_locally_noetherian a3 a2 a1 a0
  exact (curve a3 a2 a1 a0).ord
    (WeightedProjSourceFunctionField.sourceFunction
      a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 hsep) z

/-- The API's explicit junk-value boundary, specialized to the source function. -/
theorem sourceOrder_zero_of_coheight_ne_one
    (a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 : K)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable)
    (z : curve a3 a2 a1 a0) (hz : coheight z ≠ 1) :
    sourceOrder a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 hsep z = 0 := by
  letI : IsIntegral (curve a3 a2 a1 a0) :=
    WeightedProjIsIntegral.whole_curve_integral a3 a2 a1 a0 hsep
  letI : IsLocallyNoetherian (curve a3 a2 a1 a0) :=
    WeightedProjLocallyNoetherian.whole_curve_locally_noetherian a3 a2 a1 a0
  exact Scheme.ord_eq_zero_of_coheight_neq_one hz _

/-- At a point already known to have coheight one, the specialized order is
the genuine fraction-field valuation from Mathlib's stalk construction. -/
theorem sourceOrder_eq_ordHom_of_coheight_one
    (a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 : K)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable)
    (z : curve a3 a2 a1 a0) (hz : coheight z = 1) :
    letI : IsIntegral (curve a3 a2 a1 a0) :=
      WeightedProjIsIntegral.whole_curve_integral a3 a2 a1 a0 hsep
    letI : IsLocallyNoetherian (curve a3 a2 a1 a0) :=
      WeightedProjLocallyNoetherian.whole_curve_locally_noetherian a3 a2 a1 a0
    sourceOrder a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 hsep z =
      Multiplicative.toAdd (((curve a3 a2 a1 a0).ordHom z hz
        (WeightedProjSourceFunctionField.sourceFunction
          a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 hsep)).unzeroD 1) := by
  letI : IsIntegral (curve a3 a2 a1 a0) :=
    WeightedProjIsIntegral.whole_curve_integral a3 a2 a1 a0 hsep
  letI : IsLocallyNoetherian (curve a3 a2 a1 a0) :=
    WeightedProjLocallyNoetherian.whole_curve_locally_noetherian a3 a2 a1 a0
  exact Scheme.ord_eq_ordHom_of_coheight_eq_one hz _

#print axioms sourceOrder
#print axioms sourceOrder_zero_of_coheight_ne_one
#print axioms sourceOrder_eq_ordHom_of_coheight_one

end WeightedProjSourceOrder
