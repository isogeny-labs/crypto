module
public import LocalNegation

@[expose] public section
namespace NegationPullbackAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerFinitePointAudit
open scoped WithZero
variable {K : Type*} [Field K]

local instance pointMonoid (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : Monoid (PointLocalRing W x y h) :=
  (inferInstance : CommRing (PointLocalRing W x y h)).toMonoid

local instance pointFractionRing (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : IsFractionRing (PointLocalRing W x y h) W.FunctionField :=
  pointFunctionFractionRing W h

theorem point_field_map (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) (a : PointLocalRing W x y h) :
    algebraMap (PointLocalRing W x y h) W.FunctionField a = pointToField W h a := rfl

theorem finite_neg_parameter (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) :
    finiteAdic W ((W.nonsingular_neg x y).mpr h)
      (fieldNeg W (pointToField W h.1 (pointParameter W h))) = WithZero.exp (-1 : ℤ) := by
  rw [← local_neg_field]
  have ht := local_neg_uniformizer W h.1 (pointParameter W h) (point_parameter_maximal W h)
  have hv := finite_adic_of_principal_power W ((W.nonsingular_neg x y).mpr h)
    (localNeg W h.1 (pointParameter W h)) 1 (by simpa only [pow_one] using ht.symm)
  simpa only [Nat.cast_one, neg_mul, one_mul] using hv

theorem finite_neg_unit (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (u : (PointLocalRing W x y h.1)ˣ) :
    finiteAdic W ((W.nonsingular_neg x y).mpr h)
      (fieldNeg W (pointToField W h.1 u)) = 1 := by
  have hv := finite_adic_unit W ((W.nonsingular_neg x y).mpr h)
    (Units.map (localNeg W h.1).toMonoidHom u)
  change finiteAdic W ((W.nonsingular_neg x y).mpr h)
    (pointToField W _ (localNeg W h.1 u)) = 1 at hv
  rw [local_neg_field] at hv
  exact hv

/-- Every actual function is transported with its normalized local valuation, with no
valuation-invariance assumption. -/
theorem finite_neg_valuation (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (f : W.FunctionField) :
    finiteAdic W ((W.nonsingular_neg x y).mpr h) (fieldNeg W f) = finiteAdic W h f := by
  let := pointLocalDVR W h
  by_cases hf : f = 0
  · rw [hf, map_zero, map_zero, map_zero]
  have hπ : Irreducible (pointParameter W h) :=
    (IsDiscreteValuationRing.irreducible_iff_uniformizer _).mpr (point_parameter_maximal W h)
  obtain ⟨n, u, hu⟩ := IsDiscreteValuationRing.exists_units_eq_smul_zpow_of_irreducible hπ hf
  have he : f = pointToField W h.1 u * pointToField W h.1 (pointParameter W h) ^ n := by
    simpa only [Units.smul_def, Algebra.smul_def, point_field_map] using hu
  rw [he]
  simp only [map_mul, map_zpow₀, finite_adic_unit, finite_neg_parameter, finite_adic_parameter]
  rw [finite_neg_unit W h u]

/-- The source orientation: v_P(iota*f)=v_{-P}(f). -/
theorem finite_pullback_valuation (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (f : W.FunctionField) :
    finiteAdic W h (fieldNeg W f) = finiteAdic W ((W.nonsingular_neg x y).mpr h) f := by
  have he := finite_neg_valuation W h (fieldNeg W f)
  rw [field_neg_square] at he
  exact he.symm

#print axioms finite_neg_valuation
#print axioms finite_pullback_valuation
end
end NegationPullbackAudit
