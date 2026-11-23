module
public import NegationNorm

@[expose] public section
namespace NegationPullbackAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerFinitePointAudit MillerPrincipalIdealAudit FunctionDivisorFinitenessAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing F := (inferInstance : Field F).toCommRing

theorem negation_pullback_certificate :
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K),
      Function.Involutive (coordinateNeg W) ∧ Function.Involutive (fieldNeg W)) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (x y : K)
      (h : W.Nonsingular x y) (f : W.FunctionField),
      finiteAdic W h (fieldNeg W f) = finiteAdic W ((W.nonsingular_neg x y).mpr h) f) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (f : W.FunctionField) (P : W.Point),
      pointOrder W (fieldNeg W f) P = pointOrder W f (-P)) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (f : W.FunctionField) (hf : f ≠ 0),
      functionDivisor W (fieldNeg W f) (field_neg_nonzero W hf) =
        divisorNeg W (functionDivisor W f hf)) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (a : W.CoordinateRing),
      polynomialFunction W (Algebra.norm K[X] a) =
        algebraMap W.CoordinateRing W.FunctionField a *
          fieldNeg W (algebraMap W.CoordinateRing W.FunctionField a)) ∧
    controlFunction ≠ 0 ∧ pointOrder controlCurve controlFunction controlPoint = 3 ∧
    pointOrder controlCurve controlFunction (-controlPoint) = 0 ∧
    pointOrder controlCurve controlFunction controlPoint ≠
      pointOrder controlCurve controlFunction (-controlPoint) ∧
    pointOrder controlCurve (fieldNeg controlCurve controlFunction) controlPoint = 0 := by
  refine ⟨?_, ?_, ?_, ?_, ?_, control_function_nonzero, control_order_three,
    control_negative_order_zero, missing_function_pullback_counter, control_correct_pullback⟩
  · intro K _ W
    exact ⟨coordinate_neg_involutive W, field_neg_square W⟩
  · intro K _ W x y h f
    exact finite_pullback_valuation W h f
  · intro K _ W f P
    exact point_order_pullback W f P
  · intro K _ W f hf
    exact function_divisor_pullback W f hf
  · intro K _ W a
    exact field_norm_product W a

#print axioms negation_pullback_certificate
end
end NegationPullbackAudit
