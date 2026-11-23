module
public import EvaluationFormulas

@[expose] public section
namespace FinitePointEvaluationAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerFinitePointAudit MillerPrincipalIdealAudit MillerNormValuationAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
local instance : Fact (Nat.Prime 3) := ⟨by decide⟩
local instance : CommRing F3 := (inferInstance : Field F3).toCommRing

/-- Transparent positional abbreviation of the full actual singleton-product counterexample. -/
abbrev SingletonAddFailure {K : Type*} [Field K] (W : WeierstrassCurve.Affine K)
    {x y : K} (h : W.Nonsingular x y) : Prop :=
  totalizedPointDivisorValue W (.some x y h) (localPointEval W h.1 (localU W h.1))
      (Finsupp.single (.some x y h) 1 + Finsupp.single (.some x y h) (-1)) ≠
    totalizedPointDivisorValue W (.some x y h) (localPointEval W h.1 (localU W h.1))
      (Finsupp.single (.some x y h) 1) *
    totalizedPointDivisorValue W (.some x y h) (localPointEval W h.1 (localU W h.1))
      (Finsupp.single (.some x y h) (-1))

theorem finite_point_evaluation_certificate :
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (x y : K) (h : W.Equation x y),
      RingHom.ker (localPointEval W h) = IsLocalRing.maximalIdeal (PointLocalRing W x y h) ∧
      (∀ a : PointLocalRing W x y h,
        pointResidueEquiv W h (IsLocalRing.residue (PointLocalRing W x y h) a) = localPointEval W h a) ∧
      (∀ c : K, localPointEval W h (algebraMap W.CoordinateRing (PointLocalRing W x y h)
        (algebraMap K W.CoordinateRing c)) = c) ∧
      ∀ a : PointLocalRing W x y h, IsUnit a ↔ localPointEval W h a ≠ 0) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (x y : K) (h : W.Nonsingular x y),
      Function.Bijective (localToFiniteRing W h) ∧ ∀ a : PointLocalRing W x y h.1,
        (finiteLocalEquiv W h a : W.FunctionField) = pointToField W h.1 a) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (P : W.Point),
      Function.Surjective (regularEvaluation W P) ∧
      (∀ c : K, regularEvaluation W P (regularConstant W P c) = c) ∧
      (∀ a : RegularRing W P, regularEvaluation W P a ≠ 0 ↔ localFunctionValuation W P a = 1) ∧
      ∀ (f : W.FunctionField) (_ : f ≠ 0), pointOrder W f P = 0 ↔ localFunctionValuation W P f = 1) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (x y : K) (h : W.Nonsingular x y)
      (a b : W.CoordinateRing) (_ : pointEval W x y h.1 b ≠ 0),
      ∃ r : RegularRing W (.some x y h),
        (r : W.FunctionField) = algebraMap W.CoordinateRing W.FunctionField a /
          algebraMap W.CoordinateRing W.FunctionField b ∧
        regularEvaluation W (.some x y h) r = pointEval W x y h.1 a / pointEval W x y h.1 b) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ)
      (D : W.Point →₀ ℤ), Disjoint D.support (functionDivisor W f f.ne_zero).support ↔
        f ∈ supportUnits W D.support) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
      (f g : W.FunctionFieldˣ)
      (hf : Disjoint D.support (functionDivisor W f f.ne_zero).support)
      (hg : Disjoint D.support (functionDivisor W g g.ne_zero).support),
      evaluateDivisor W D (f * g) (disjoint_principal_mul W D f g hf hg) =
        evaluateDivisor W D f hf * evaluateDivisor W D g hg) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (T : Finset W.Point)
      (D E : W.Point →₀ ℤ), divisorEvaluationOn W T (D + E) =
        divisorEvaluationOn W T D * divisorEvaluationOn W T E) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (T : Finset W.Point)
      (D : W.Point →₀ ℤ) (_ : D.support ⊆ T) (c : Kˣ) (f : supportUnits W T),
      divisorEvaluationOn W T D (supportConstantUnits W T c * f) =
        c ^ divisorDegree W D * divisorEvaluationOn W T D f ∧
      (divisorDegree W D = 0 → divisorEvaluationOn W T D (supportConstantUnits W T c * f) =
        divisorEvaluationOn W T D f)) ∧
    (∀ (K : Type) [Field K] [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (_ : W.Δ ≠ 0)
      (D : W.Point →₀ ℤ) (_ : divisorDegree W D = 0) (f g : W.FunctionFieldˣ)
      (hf : Disjoint D.support (functionDivisor W f f.ne_zero).support)
      (hg : Disjoint D.support (functionDivisor W g g.ne_zero).support)
      (_ : functionDivisor W f f.ne_zero = functionDivisor W g g.ne_zero),
      evaluateDivisor W D f hf = evaluateDivisor W D g hg) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (x : K)
      (e : W.FunctionField →+* K) (_ : ∀ c : K, e (constantHom W c) = c)
      (_ : e (xFunction W) = x), False) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (x y : K) (h : W.Nonsingular x y),
      (¬ ∃ a : PointLocalRing W x y h.1, pointToField W h.1 a = (verticalFunction W x)⁻¹) ∧
      1 < finiteAdic W h (verticalFunction W x)⁻¹ ∧
      finiteRegularEval W h (finiteLocalEquiv W h (localU W h.1)) = 0 ∧ SingletonAddFailure W h) ∧
    rationalCurve.Δ ≠ 0 ∧ rationalCurve.Nonsingular (0 : F3) 1 ∧
    pointOrder rationalCurve (verticalFunction rationalCurve 0) controlPoint = 1 ∧
    pointOrder rationalCurve (verticalFunction rationalCurve 0) 0 = -2 ∧
    controlDivisor.support ≠ controlPrincipalDivisor.support ∧
    ¬ Disjoint controlDivisor.support controlPrincipalDivisor.support := by
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, rational_delta, control_nonsingular,
    control_point_order, control_infinity_order, control_support_unequal, control_support_overlap⟩
  · intro K _ W x y h
    exact ⟨local_point_kernel W h, point_residue_eval W h,
      local_point_constant W h, local_point_unit_iff W h⟩
  · intro K _ W x y h
    exact ⟨⟨local_to_finite_injective W h, local_to_finite_surjective W h⟩, finite_local_equiv_coe W h⟩
  · intro K _ W P
    refine ⟨regular_evaluation_surjective W P, regular_eval_constant W P,
      regular_evaluation_value_iff W P, ?_⟩
    intro f hf
    exact point_order_zero_iff_value_one W f hf P
  · intro K _ W x y h a b hb
    exact finite_fraction_evaluation W h a b hb
  · intro K _ W f D
    exact disjoint_principal_support_iff W f D
  · intro K _ W D f g hf hg
    exact evaluate_divisor_mul W D f g hf hg
  · intro K _ W T D E
    exact divisor_evaluation_on_add W T D E
  · intro K _ W T D hD c f
    exact ⟨divisor_evaluation_scalar W T D hD c f,
      fun hdeg => degree_zero_scalar_invariance W T D hD hdeg c f⟩
  · intro K _ _ W hΔ D hdeg f g hf hg hfg
    exact geometric_degree_zero_choice W hΔ D hdeg f g hf hg hfg
  · intro K _ W x e hc hx
    exact no_total_point_evaluation W x e hc hx
  · intro K _ W x y h
    exact ⟨inverse_vertical_not_local W h.1, inverse_vertical_has_pole W h,
      vertical_regular_value_zero W h, totalized_add_failure W h⟩

#print axioms finite_point_evaluation_certificate
end
end FinitePointEvaluationAudit
