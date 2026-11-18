import P1Certificate
import ReciprocityImports

namespace EllipticGlobalReciprocityAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit EllipticFinitePlaceProjectionAudit
open P1TameReciprocityAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- Actual rational base place: infinity or the finite x-coordinate. -/
def pointBasePlace (W : WeierstrassCurve.Affine K) : W.Point → Option K
  | .zero=>none
  | .some a _ _=>some a

theorem point_base_place_none (W : WeierstrassCurve.Affine K) (P : W.Point) :
    pointBasePlace W P=none ↔ P=0 := by
  cases P <;> simp [pointBasePlace,Point.zero_def]

theorem point_base_place_eq_iff (W : WeierstrassCurve.Affine K) (P Q : W.Point) :
    pointBasePlace W P=pointBasePlace W Q ↔ P.xRep=Q.xRep := by
  cases P <;> cases Q <;> simp [pointBasePlace,Point.xRep]

theorem finite_fiber_place (W : WeierstrassCurve.Affine K) {a b : K}
    (h : W.Nonsingular a b) (R : W.Point) :
    R∈finiteFiber W (Point.some a b h) ↔ pointBasePlace W R=some a := by
  change R∈(finiteFiber W (Point.some a b h) : Set W.Point) ↔ _
  rw [finite_fiber_coe]
  change R.xRep=(Point.some a b h).xRep ↔ pointBasePlace W R=pointBasePlace W (Point.some a b h)
  exact (point_base_place_eq_iff W R (Point.some a b h)).symm

theorem actual_nonsingular_point_above [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (a : K) : ∃ b : K,W.Nonsingular a b := by
  have h2 : (fiberPolynomial W a).coeff 2=(1 : K) := by
    simp only [fiberPolynomial,coeff_sub,coeff_add,coeff_C_mul,coeff_X_pow,coeff_X,coeff_C]
    norm_num
  have hd : (fiberPolynomial W a).degree≠0 := by
    intro he
    have hz : (fiberPolynomial W a).coeff 2=0 := coeff_eq_zero_of_degree_lt (by rw [he];norm_num)
    rw [h2] at hz
    exact one_ne_zero hz
  obtain ⟨b,hb⟩:=IsAlgClosed.exists_root (fiberPolynomial W a) hd
  exact ⟨b,(W.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp ((equation_fiber_root W a b).mpr hb)⟩

theorem actual_base_place_surjective [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) : Function.Surjective (pointBasePlace W) := by
  intro v
  cases v with
  | none=>exact ⟨0,rfl⟩
  | some a=>
    obtain ⟨b,h⟩:=actual_nonsingular_point_above W hΔ a
    exact ⟨Point.some a b h,rfl⟩

#print axioms point_base_place_none
#print axioms point_base_place_eq_iff
#print axioms finite_fiber_place
#print axioms actual_nonsingular_point_above
#print axioms actual_base_place_surjective
end
end EllipticGlobalReciprocityAudit
