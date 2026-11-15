module
public import FiniteZeros

@[expose] public section
namespace FunctionDivisorFinitenessAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerPrincipalIdealAudit MillerFinitePointAudit MillerRecursionAudit
variable {K : Type*} [Field K]

local instance pointMonoid (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : Monoid (PointLocalRing W x y h) :=
  (inferInstance : CommRing (PointLocalRing W x y h)).toMonoid

local instance functionMonoid (W : WeierstrassCurve.Affine K) : Monoid W.FunctionField :=
  (inferInstance : Field W.FunctionField).toMonoid

/-- An actual regular function nonvanishing at the actual point is a unit in its actual local ring. -/
theorem coordinate_nonvanishing_value (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (a : W.CoordinateRing) (ha : pointEval W x y h.1 a ≠ 0) :
    finiteAdic W h (algebraMap W.CoordinateRing W.FunctionField a) = 1 := by
  have hu : IsUnit (algebraMap W.CoordinateRing (PointLocalRing W x y h.1) a) :=
    (IsLocalization.AtPrime.isUnit_to_map_iff (PointLocalRing W x y h.1)
      (evaluationIdeal W x y h.1) a).mpr ha
  obtain ⟨u, hu⟩ := hu
  have hv := finite_adic_unit W h u
  rw [hu, point_to_field_map] at hv
  exact hv

/-- Every actual nonzero rational function has only finitely many nonzero actual local orders. -/
theorem function_order_support_finite (W : WeierstrassCurve.Affine K) (f : W.FunctionField)
    (hf : f ≠ 0) : (Function.support (pointOrder W f)).Finite := by
  obtain ⟨a, b, hb, hab⟩ := IsFractionRing.div_surjective W.CoordinateRing f
  have ha0 : a ≠ 0 := by
    intro ha
    rw [ha, map_zero, zero_div] at hab
    exact hf hab.symm
  have hb0 : b ≠ 0 := nonZeroDivisors.ne_zero hb
  have hs : ({(0 : W.Point)} ∪ affineZeroSet W a ∪ affineZeroSet W b).Finite :=
    ((Set.finite_singleton 0).union (affine_zero_set_finite W ha0)).union
      (affine_zero_set_finite W hb0)
  apply hs.subset
  intro R hR
  by_contra hn
  have h0 : R ≠ 0 := fun h => hn (Or.inl (Or.inl h))
  have hna : R ∉ affineZeroSet W a := fun h => hn (Or.inl (Or.inr h))
  have hnb : R ∉ affineZeroSet W b := fun h => hn (Or.inr h)
  cases R with
  | zero => exact h0 rfl
  | some x y h =>
    have va := coordinate_nonvanishing_value W h a hna
    have vb := coordinate_nonvanishing_value W h b hnb
    apply hR
    rw [← hab]
    change -WithZero.log (finiteAdic W h
      (algebraMap W.CoordinateRing W.FunctionField a / algebraMap W.CoordinateRing W.FunctionField b)) = 0
    rw [map_div₀, va, vb]
    simp

/-- General principal point divisor from actual local valuations, with finite support derived. -/
def functionDivisor (W : WeierstrassCurve.Affine K) (f : W.FunctionField) (hf : f ≠ 0) :
    W.Point →₀ ℤ := Finsupp.ofSupportFinite (pointOrder W f) (function_order_support_finite W f hf)

theorem function_divisor_apply (W : WeierstrassCurve.Affine K) (f : W.FunctionField) (hf : f ≠ 0)
    (R : W.Point) : functionDivisor W f hf R = pointOrder W f R := rfl

theorem function_divisor_one (W : WeierstrassCurve.Affine K) :
    functionDivisor W 1 one_ne_zero = 0 := by
  ext R
  exact point_order_one W R

theorem function_divisor_mul (W : WeierstrassCurve.Affine K) {f g : W.FunctionField}
    (hf : f ≠ 0) (hg : g ≠ 0) :
    functionDivisor W (f * g) (mul_ne_zero hf hg) = functionDivisor W f hf + functionDivisor W g hg := by
  ext R
  exact point_order_mul W hf hg R

theorem function_divisor_div (W : WeierstrassCurve.Affine K) {f g : W.FunctionField}
    (hf : f ≠ 0) (hg : g ≠ 0) :
    functionDivisor W (f / g) (div_ne_zero hf hg) = functionDivisor W f hf - functionDivisor W g hg := by
  ext R
  exact point_order_div W hf hg R

/-- Actual rational function units map additively to the actual point-divisor group. -/
def principalDivisorHom (W : WeierstrassCurve.Affine K) :
    Additive W.FunctionFieldˣ →+ (W.Point →₀ ℤ) where
  toFun u := functionDivisor W (u.toMul : W.FunctionField) u.toMul.ne_zero
  map_zero' := by
    ext R
    exact point_order_one W R
  map_add' u v := by
    ext R
    exact point_order_mul W u.toMul.ne_zero v.toMul.ne_zero R

def principalPointDivisors (W : WeierstrassCurve.Affine K) : AddSubgroup (W.Point →₀ ℤ) :=
  (principalDivisorHom W).range

theorem function_divisor_mem_principal (W : WeierstrassCurve.Affine K)
    (f : W.FunctionField) (hf : f ≠ 0) : functionDivisor W f hf ∈ principalPointDivisors W :=
  ⟨Additive.ofMul (Units.mk0 f hf), rfl⟩

variable [DecidableEq K]

/-- The general divisor agrees with the earlier actual all-n Miller divisor, not a changed model. -/
theorem function_divisor_miller (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) :
    functionDivisor W (millerFunction W P n) (miller_function_nonzero W P n) = millerDivisor W P n := by
  ext R
  exact (miller_has_point_divisor W P n).2 R

#print axioms function_order_support_finite
#print axioms function_divisor_mul
end
end FunctionDivisorFinitenessAudit
