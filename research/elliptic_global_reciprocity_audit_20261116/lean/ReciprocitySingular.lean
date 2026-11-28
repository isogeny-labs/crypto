import ReciprocityDivisors

namespace EllipticGlobalReciprocityAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open MillerPrincipalIdealAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit EllipticFinitePlaceProjectionAudit
open P1TameReciprocityAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- Actual singular cusp model; it is not asserted to be an elliptic curve. -/
def singularCusp (K : Type*) [Field K] : WeierstrassCurve.Affine K := ⟨0,0,0,0,0⟩

theorem singular_cusp_delta : (singularCusp K).Δ=0 := by
  simp [singularCusp,WeierstrassCurve.Δ,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
    WeierstrassCurve.b₆,WeierstrassCurve.b₈]

theorem singular_cusp_finite_x_nonzero {x y : K} (h : (singularCusp K).Nonsingular x y) : x≠0 := by
  intro hx
  subst x
  have he:=(equation_iff 0 y).mp h.1
  have hy : y^2=0 := by simpa [singularCusp] using he
  have hy0 : y=0 := by
    rw [pow_two,mul_eq_zero,or_self] at hy
    exact hy
  subst y
  have hn:=(nonsingular_iff' (W:=singularCusp K) 0 0).mp h
  simp [singularCusp] at hn

def singularCuspX : (singularCusp K).FunctionFieldˣ :=
  Units.mk0 (verticalFunction (singularCusp K) 0) (vertical_nonzero (singularCusp K) 0)

/-- Missing the singular point removes the actual finite zero even over an algebraic closure. -/
theorem singular_cusp_x_divisor :
    functionDivisor (singularCusp K) (singularCuspX (K:=K))
      (singularCuspX (K:=K)).ne_zero=Finsupp.single 0 (-2) := by
  classical
  ext P
  cases P with
  | zero=>
    rw [function_divisor_apply]
    change -WithZero.log (chartAdicValuation (singularCusp K) (verticalFunction (singularCusp K) 0))=_
    rw [chart_adic_vertical,WithZero.log_exp]
    simp only [←Point.zero_def,Finsupp.single_eq_same]
  | some x y h=>
    have hn : x-0≠0:=by simpa only [sub_zero] using singular_cusp_finite_x_nonzero h
    have hv:=coordinate_nonvanishing_value (singularCusp K) h
      (WeierstrassCurve.Affine.CoordinateRing.XClass (singularCusp K) 0)
      (by rw [point_eval_XClass];exact hn)
    rw [function_divisor_apply]
    change -WithZero.log (finiteAdic (singularCusp K) h (verticalFunction (singularCusp K) 0))=_
    change finiteAdic (singularCusp K) h (verticalFunction (singularCusp K) 0)=1 at hv
    rw [hv,WithZero.log_one,neg_zero]
    simp

theorem singular_cusp_global_constant (c : Kˣ) :
    globalTameProduct (singularCusp K) (coefficientUnit (singularCusp K) c) (singularCuspX (K:=K))=c^(-2 : ℤ) := by
  rw [global_constant_degree,singular_cusp_x_divisor,degree_single]

#print axioms singular_cusp_delta
#print axioms singular_cusp_finite_x_nonzero
#print axioms singular_cusp_x_divisor
#print axioms singular_cusp_global_constant
end
end EllipticGlobalReciprocityAudit
