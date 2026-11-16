import FinitePlaceValuation

namespace EllipticFinitePlaceProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- Actual distinct fullpointfiber. A fixedpoint occurs once. -/
def finiteFiber (W : WeierstrassCurve.Affine K) (P : W.Point) : Finset W.Point := by
  classical
  exact {P,-P}

theorem finite_fiber_coe (W : WeierstrassCurve.Affine K) (P : W.Point) :
    (finiteFiber W P : Set W.Point)=xFiber W P := by
  classical
  change (({P,-P} : Finset W.Point) : Set W.Point)=xFiber W P
  rw [Finset.coe_pair,actual_x_fiber]

theorem finite_fiber_fixed (W : WeierstrassCurve.Affine K) (P : W.Point) (hP : P= -P) :
    finiteFiber W P={P} := by
  classical
  rw [finiteFiber,←hP,Finset.pair_eq_singleton]

theorem finite_fiber_split_card (W : WeierstrassCurve.Affine K) (P : W.Point) (hP : P≠ -P) :
    (finiteFiber W P).card=2 := by
  classical
  rw [finiteFiber,Finset.card_pair hP]

/-- Derived point-index/card relation; no scheme length premise. -/
theorem finite_index_times_fiber_card (W : WeierstrassCurve.Affine K) (P : W.Point) :
    finitePointIndex W P*(finiteFiber W P).card=2 := by
  classical
  by_cases hP : P= -P
  · rw [finitePointIndex,ite_eq_left hP,finite_fiber_fixed W P hP,Finset.card_singleton]
  · rw [finitePointIndex,ite_eq_right hP,finite_fiber_split_card W P hP]

theorem finite_fiber_prod_fixed {M : Type*} [CommMonoid M] (W : WeierstrassCurve.Affine K)
    (P : W.Point) (hP : P= -P) (f : W.Point → M) : (∏ R ∈ finiteFiber W P,f R)=f P := by
  classical
  rw [finite_fiber_fixed W P hP,Finset.prod_singleton]

theorem finite_fiber_prod_split {M : Type*} [CommMonoid M] (W : WeierstrassCurve.Affine K)
    (P : W.Point) (hP : P≠ -P) (f : W.Point → M) :
    (∏ R ∈ finiteFiber W P,f R)=f P*f (-P) := by
  classical
  rw [finiteFiber,Finset.prod_pair hP]

/-- Actual normalized normorder sums over the actual distinct fullpointfiber. -/
theorem actual_finite_norm_order (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (f : W.FunctionFieldˣ) :
    finiteBaseOrder x (normUnit W f : RatFunc K)=∑ R ∈ finiteFiber W (Point.some x y h),pointOrder W f R := by
  classical
  by_cases hP : (Point.some x y h : W.Point)= -Point.some x y h
  · rw [finite_fiber_fixed W (Point.some x y h) hP,Finset.sum_singleton]
    exact fixed_norm_base_order W h hP f
  · rw [finiteFiber,Finset.sum_pair hP]
    have hn:=actual_point_base_norm_order W (Point.some x y h) f
    rw [split_restricted_order_official W h hP] at hn
    exact hn

#print axioms finite_fiber_coe
#print axioms finite_index_times_fiber_card
#print axioms actual_finite_norm_order
end
end EllipticFinitePlaceProjectionAudit
