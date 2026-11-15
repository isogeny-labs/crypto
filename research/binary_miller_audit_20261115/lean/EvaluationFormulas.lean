module
public import EvaluationControls

@[expose] public section
namespace FinitePointEvaluationAudit
noncomputable section
open MillerFinitePointAudit MillerNormValuationAudit FunctionDivisorFinitenessAudit
open GeometricPrincipalDivisorAudit MillerPrincipalIdealAudit
variable {K : Type*} [Field K]

local instance formulaPointMonoid (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : Monoid (PointLocalRing W x y h) :=
  (inferInstance : CommRing (PointLocalRing W x y h)).toMonoid

/-- Actual coordinate-fraction evaluation, with the denominator condition proved sufficient. -/
theorem finite_fraction_evaluation (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (a b : W.CoordinateRing) (hb : pointEval W x y h.1 b ≠ 0) :
    ∃ r : RegularRing W (.some x y h),
      (r : W.FunctionField) = algebraMap W.CoordinateRing W.FunctionField a /
        algebraMap W.CoordinateRing W.FunctionField b ∧
      regularEvaluation W (.some x y h) r = pointEval W x y h.1 a / pointEval W x y h.1 b := by
  let a' : PointLocalRing W x y h.1 := algebraMap W.CoordinateRing (PointLocalRing W x y h.1) a
  let b' : PointLocalRing W x y h.1 := algebraMap W.CoordinateRing (PointLocalRing W x y h.1) b
  have hb' : localPointEval W h.1 b' ≠ 0 := by
    change localPointEval W h.1 (algebraMap W.CoordinateRing (PointLocalRing W x y h.1) b) ≠ 0
    rw [local_point_map]
    exact hb
  have hu : IsUnit b' := (local_point_unit_iff W h.1 b').mpr hb'
  let u : (PointLocalRing W x y h.1)ˣ := hu.unit
  have he : (u : PointLocalRing W x y h.1) = b' := IsUnit.unit_spec hu
  refine ⟨finiteLocalEquiv W h (a' * (↑(u⁻¹) : PointLocalRing W x y h.1)), ?_, ?_⟩
  · change pointToField W h.1 (a' * (↑(u⁻¹) : PointLocalRing W x y h.1)) = _
    simp only [map_mul, map_units_inv, he, a', b', point_to_field_map, div_eq_mul_inv]
  · change finiteRegularEval W h (finiteLocalEquiv W h
      (a' * (↑(u⁻¹) : PointLocalRing W x y h.1))) = _
    rw [finite_regular_eval_local]
    simp only [map_mul, map_units_inv, he, a', b', local_point_map, div_eq_mul_inv]

theorem point_unit_fraction_value (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (a b : W.CoordinateRing) (hb : pointEval W x y h.1 b ≠ 0)
    (f : (RegularRing W (.some x y h)).unitGroup)
    (hf : ((f : W.FunctionFieldˣ) : W.FunctionField) =
      algebraMap W.CoordinateRing W.FunctionField a / algebraMap W.CoordinateRing W.FunctionField b) :
    (pointUnitEvaluation W (.some x y h) f : K) = pointEval W x y h.1 a / pointEval W x y h.1 b := by
  obtain ⟨r, hr, hv⟩ := finite_fraction_evaluation W h a b hb
  have he : ((RegularRing W (.some x y h)).unitGroupMulEquiv f : RegularRing W (.some x y h)) = r := by
    apply Subtype.ext
    change ((f : W.FunctionFieldˣ) : W.FunctionField) = (r : W.FunctionField)
    exact hf.trans hr.symm
  change regularEvaluation W (.some x y h)
    ((RegularRing W (.some x y h)).unitGroupMulEquiv f : RegularRing W (.some x y h)) = _
  rw [he]
  exact hv

/-- Qualified forward direction of source Exercise26.1.1 on actual elliptic geometric divisors. -/
theorem geometric_degree_zero_choice [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ ≠ 0) (D : W.Point →₀ ℤ) (hdeg : divisorDegree W D = 0)
    (f g : W.FunctionFieldˣ)
    (hf : Disjoint D.support (functionDivisor W f f.ne_zero).support)
    (hg : Disjoint D.support (functionDivisor W g g.ne_zero).support)
    (hfg : functionDivisor W f f.ne_zero = functionDivisor W g g.ne_zero) :
    evaluateDivisor W D f hf = evaluateDivisor W D g hg := by
  obtain ⟨c, hc, he⟩ := (same_divisor_iff_scalar W hΔ f.ne_zero g.ne_zero).mp hfg
  let cu : Kˣ := Units.mk0 c hc
  let fu : supportUnits W D.support := ⟨f, (disjoint_principal_support_iff W f D).mp hf⟩
  let gu : supportUnits W D.support := ⟨g, (disjoint_principal_support_iff W g D).mp hg⟩
  have hs : fu = supportConstantUnits W D.support cu * gu := by
    apply Subtype.ext
    apply Units.ext
    exact he
  change divisorEvaluationOn W D.support D fu = divisorEvaluationOn W D.support D gu
  rw [hs]
  exact degree_zero_scalar_invariance W D.support D (fun _ hp => hp) hdeg cu gu

#print axioms finite_fraction_evaluation
#print axioms point_unit_fraction_value
#print axioms geometric_degree_zero_choice
end
end FinitePointEvaluationAudit
