import WeightedProjIsIntegral
import Common22SupportHead
import Mathlib.AlgebraicGeometry.FunctionField
import Mathlib.CategoryTheory.ConcreteCategory.EpiMono

/-! The source-shaped common 2+2 expression in the function field of the
actual weighted projective curve. No divisor assertion is made here. -/

set_option autoImplicit false
set_option maxHeartbeats 600000
set_option backward.isDefEq.respectTransparency.types false
set_option backward.isDefEq.respectTransparency false
noncomputable section

namespace WeightedProjSourceFunctionField

open AlgebraicGeometry CategoryTheory WeightedRingGraded
  Common22WeightedProjQuotient WeightedOverlapSpecialized

variable {K : Type} [Field K]

private abbrev curve (a3 a2 a1 a0 : K) :=
  Proj (degreeSubmodule a3 a2 a1 a0)

private abbrev affineOpen (a3 a2 a1 a0 : K) :
    (curve a3 a2 a1 a0).Opens :=
  Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
    (Common22WeightedProjKernel.zbar a3 a2 a1 a0)

theorem affineOpen_nonempty (a3 a2 a1 a0 : K) :
    Nonempty (affineOpen a3 a2 a1 a0) := by
  obtain ⟨p, _, hpz⟩ :=
    WeightedProjOverlapNonempty.actual_x_z_overlap_nonempty a3 a2 a1 a0
  exact ⟨⟨p, hpz⟩⟩

/-- The actual affine chart ring injects into the function field via chart
sections and the generic germ. -/
def affineToFunctionField (a3 a2 a1 a0 : K)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable) :
    letI : IsIntegral (curve a3 a2 a1 a0) :=
      WeightedProjIsIntegral.whole_curve_integral a3 a2 a1 a0 hsep
    AffineRing a3 a2 a1 a0 →+* (curve a3 a2 a1 a0).functionField := by
  letI : IsIntegral (curve a3 a2 a1 a0) :=
    WeightedProjIsIntegral.whole_curve_integral a3 a2 a1 a0 hsep
  letI : Nonempty (affineOpen a3 a2 a1 a0) :=
    affineOpen_nonempty a3 a2 a1 a0
  exact ((Scheme.ΓSpecIso ↧(AffineRing a3 a2 a1 a0)).inv ≫
    (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).hom.appTop ≫
    (affineOpen a3 a2 a1 a0).topIso.hom ≫
    (curve a3 a2 a1 a0).germToFunctionField
      (affineOpen a3 a2 a1 a0)).hom

theorem affineToFunctionField_injective (a3 a2 a1 a0 : K)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable) :
    letI : IsIntegral (curve a3 a2 a1 a0) :=
      WeightedProjIsIntegral.whole_curve_integral a3 a2 a1 a0 hsep
    Function.Injective (affineToFunctionField a3 a2 a1 a0 hsep) := by
  letI : IsIntegral (curve a3 a2 a1 a0) :=
    WeightedProjIsIntegral.whole_curve_integral a3 a2 a1 a0 hsep
  letI : Nonempty (affineOpen a3 a2 a1 a0) :=
    affineOpen_nonempty a3 a2 a1 a0
  have hγ : Function.Injective
      (Scheme.ΓSpecIso ↧(AffineRing a3 a2 a1 a0)).inv :=
    (ConcreteCategory.bijective_of_isIso _).1
  have hc : Function.Injective
      (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).hom.appTop :=
    (ConcreteCategory.bijective_of_isIso _).1
  have ht : Function.Injective (affineOpen a3 a2 a1 a0).topIso.hom :=
    (ConcreteCategory.bijective_of_isIso _).1
  exact ((curve a3 a2 a1 a0).germToFunctionField_injective
      (affineOpen a3 a2 a1 a0)).comp (ht.comp (hc.comp hγ))

/-- The cubic numerator, with the coefficients of the source common 2+2
branch and affine coordinate `x`. -/
def sourceNumerator (a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 : K) :
    AffineRing a3 a2 a1 a0 :=
  let (B, A, C, _, _) :=
    Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
  let x := xbar a3 a2 a1 a0
  let ι := algebraMap K (AffineRing a3 a2 a1 a0)
  ι B * (ι s1 * x + ι t1) -
    (ι C * x + ι A) * (x ^ 2 + ι q1 * x + ι r1)

/-- The source-style rational expression `N/B-y` on the actual curve.
The coefficient `B` is a parameter; denominator nonvanishing is a separate
hypothesis for using this as the source branch. -/
def sourceFunction (a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 : K)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable) :
    letI : IsIntegral (curve a3 a2 a1 a0) :=
      WeightedProjIsIntegral.whole_curve_integral a3 a2 a1 a0 hsep
    (curve a3 a2 a1 a0).functionField := by
  letI : IsIntegral (curve a3 a2 a1 a0) :=
    WeightedProjIsIntegral.whole_curve_integral a3 a2 a1 a0 hsep
  let B := (Common22SupportHead.coefficientData
    q1 r1 s1 t1 q2 r2 s2 t2).1
  let φ := affineToFunctionField a3 a2 a1 a0 hsep
  exact φ (sourceNumerator a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2) /
    φ (algebraMap K (AffineRing a3 a2 a1 a0) B) -
    φ (ybar a3 a2 a1 a0)

/-- The source branch denominator remains nonzero in the actual function
field when its field coefficient is nonzero. -/
theorem sourceDenominator_ne_zero (a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 : K)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable)
    (hB : (Common22SupportHead.coefficientData
      q1 r1 s1 t1 q2 r2 s2 t2).1 ≠ 0) :
    letI : IsIntegral (curve a3 a2 a1 a0) :=
      WeightedProjIsIntegral.whole_curve_integral a3 a2 a1 a0 hsep
    affineToFunctionField a3 a2 a1 a0 hsep
      (algebraMap K (AffineRing a3 a2 a1 a0)
        (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1) ≠ 0 := by
  letI : IsIntegral (curve a3 a2 a1 a0) :=
    WeightedProjIsIntegral.whole_curve_integral a3 a2 a1 a0 hsep
  letI : IsDomain (AffineRing a3 a2 a1 a0) := by
    change IsDomain (Common22SeparateChartMap.AffineRing a3 a2 a1 a0)
    exact Common22SeparableAffineDomain.affine_ring_domain_of_separable
      a3 a2 a1 a0 hsep
  simpa using (affineToFunctionField_injective a3 a2 a1 a0 hsep).ne
    ((map_ne_zero (algebraMap K (AffineRing a3 a2 a1 a0))).mpr hB)

#print axioms affineOpen_nonempty
#print axioms affineToFunctionField
#print axioms affineToFunctionField_injective
#print axioms sourceNumerator
#print axioms sourceFunction
#print axioms sourceDenominator_ne_zero

end WeightedProjSourceFunctionField
