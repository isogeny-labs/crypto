module
public import PointLocalization

@[expose] public section
namespace MillerFinitePointAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerPrincipalIdealAudit
variable {K : Type*} [Field K]

theorem coefficient_eq_mk (W : WeierstrassCurve.Affine K) (c : K) :
    algebraMap K W.CoordinateRing c = mk W (C (C c)) := rfl

def derivativeY (W : WeierstrassCurve.Affine K) (x y : K) : K :=
  2 * y + W.a₁ * x + W.a₃

def negativeDerivativeX (W : WeierstrassCurve.Affine K) (x y : K) : K :=
  3 * x ^ 2 + 2 * W.a₂ * x + W.a₄ - W.a₁ * y

def yBracket (W : WeierstrassCurve.Affine K) (x y : K) : W.CoordinateRing :=
  YClass W (C y) + algebraMap K W.CoordinateRing W.a₁ * XClass W x +
    algebraMap K W.CoordinateRing (derivativeY W x y)

def xBracket (W : WeierstrassCurve.Affine K) (x y : K) : W.CoordinateRing :=
  XClass W x ^ 2 + algebraMap K W.CoordinateRing (3 * x + W.a₂) * XClass W x +
    algebraMap K W.CoordinateRing (negativeDerivativeX W x y)

/-- Exact shifted actual coordinate-ring equation, valid in every characteristic. -/
theorem shifted_curve_relation (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) :
    YClass W (C y) * yBracket W x y = XClass W x * xBracket W x y := by
  have hp : (YClass W (C 0)) ^ 2 +
      (algebraMap K W.CoordinateRing W.a₁ * XClass W 0 +
        algebraMap K W.CoordinateRing W.a₃) * YClass W (C 0) =
      XClass W 0 ^ 3 + algebraMap K W.CoordinateRing W.a₂ * XClass W 0 ^ 2 +
        algebraMap K W.CoordinateRing W.a₄ * XClass W 0 +
        algebraMap K W.CoordinateRing W.a₆ := by
    have he : mk W W.polynomial = 0 := AdjoinRoot.mk_self
    simp only [WeierstrassCurve.Affine.polynomial, map_sub, map_add, map_mul,
      map_pow] at he
    simpa only [coefficient_eq_mk, YClass, XClass, map_zero, sub_zero] using sub_eq_zero.mp he
  have hs := congrArg (algebraMap K W.CoordinateRing) ((W.equation_iff x y).mp h)
  simp only [map_add, map_mul, map_pow] at hs
  have hx : XClass W x = XClass W 0 - algebraMap K W.CoordinateRing x := by
    simp only [XClass, map_sub, map_zero, sub_zero]
    rfl
  have hy : YClass W (C y) = YClass W (C 0) - algebraMap K W.CoordinateRing y := by
    simp only [YClass, map_sub, map_zero, sub_zero]
    rfl
  simp only [yBracket, xBracket, derivativeY, negativeDerivativeX,
    map_add, map_sub, map_mul, map_pow, map_ofNat, hx, hy]
  linear_combination hp - hs

theorem eval_y_bracket (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Equation x y) :
    pointEval W x y h (yBracket W x y) = derivativeY W x y := by
  simp [yBracket, point_eval_XClass, point_eval_YClass, point_eval_constant]

theorem eval_x_bracket (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Equation x y) :
    pointEval W x y h (xBracket W x y) = negativeDerivativeX W x y := by
  simp [xBracket, point_eval_XClass, point_eval_constant]

theorem local_v_in_u (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Equation x y)
    (ha : derivativeY W x y ≠ 0) : localV W h ∈ Ideal.span {localU W h} := by
  let f := algebraMap W.CoordinateRing (PointLocalRing W x y h)
  have hu : IsUnit (f (yBracket W x y)) :=
    (IsLocalization.AtPrime.isUnit_to_map_iff (PointLocalRing W x y h)
      (evaluationIdeal W x y h) (yBracket W x y)).mpr (by
      change pointEval W x y h (yBracket W x y) ≠ 0
      rwa [eval_y_bracket])
  have he := congrArg f (shifted_curve_relation W h)
  simp only [map_mul] at he
  have hm : localV W h * f (yBracket W x y) ∈ Ideal.span {localU W h} := by
    rw [show localV W h * f (yBracket W x y) = localU W h * f (xBracket W x y) from he]
    exact Ideal.mul_mem_right _ _ (Ideal.subset_span (by simp))
  exact (Ideal.mul_unit_mem_iff_mem _ hu).mp hm

theorem local_u_in_v (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Equation x y)
    (hb : negativeDerivativeX W x y ≠ 0) : localU W h ∈ Ideal.span {localV W h} := by
  let f := algebraMap W.CoordinateRing (PointLocalRing W x y h)
  have hu : IsUnit (f (xBracket W x y)) :=
    (IsLocalization.AtPrime.isUnit_to_map_iff (PointLocalRing W x y h)
      (evaluationIdeal W x y h) (xBracket W x y)).mpr (by
      change pointEval W x y h (xBracket W x y) ≠ 0
      rwa [eval_x_bracket])
  have he := congrArg f (shifted_curve_relation W h)
  simp only [map_mul] at he
  have hm : localU W h * f (xBracket W x y) ∈ Ideal.span {localV W h} := by
    rw [show localU W h * f (xBracket W x y) = localV W h * f (yBracket W x y) from he.symm]
    exact Ideal.mul_mem_right _ _ (Ideal.subset_span (by simp))
  exact (Ideal.mul_unit_mem_iff_mem _ hu).mp hm

theorem local_maximal_u (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Equation x y)
    (ha : derivativeY W x y ≠ 0) :
    IsLocalRing.maximalIdeal (PointLocalRing W x y h) = Ideal.span {localU W h} := by
  rw [local_maximal_pair, Ideal.span_pair_eq_span_left_iff_dvd]
  exact Ideal.mem_span_singleton.mp (local_v_in_u W h ha)

theorem local_maximal_v (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Equation x y)
    (hb : negativeDerivativeX W x y ≠ 0) :
    IsLocalRing.maximalIdeal (PointLocalRing W x y h) = Ideal.span {localV W h} := by
  rw [local_maximal_pair, Set.pair_comm, Ideal.span_pair_eq_span_left_iff_dvd]
  exact Ideal.mem_span_singleton.mp (local_u_in_v W h hb)

theorem nonsingular_derivatives (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) : derivativeY W x y ≠ 0 ∨ negativeDerivativeX W x y ≠ 0 := by
  rcases ((W.nonsingular_iff' x y).mp h).2 with hx | hy
  · right
    exact neg_ne_zero.mp (by simpa only [negativeDerivativeX, neg_sub] using hx)
  · exact Or.inl hy

theorem local_maximal_nonzero (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Equation x y) :
    IsLocalRing.maximalIdeal (PointLocalRing W x y h) ≠ ⊥ := by
  intro hz
  have hu : localU W h ∈ IsLocalRing.maximalIdeal (PointLocalRing W x y h) := by
    rw [local_maximal_pair]
    exact Ideal.subset_span (by simp)
  rw [hz] at hu
  exact local_u_nonzero W h hu

/-- Local regularity/DVR is derived from the actual nonsingular equation, not a DVR premise. -/
theorem pointLocalDVR (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Nonsingular x y) :
    IsDiscreteValuationRing (PointLocalRing W x y h.1) where
  toIsPrincipalIdealRing := by
    have hp : (IsLocalRing.maximalIdeal (PointLocalRing W x y h.1)).IsPrincipal := by
      rcases nonsingular_derivatives W h with ha | hb
      · rw [local_maximal_u W h.1 ha]; infer_instance
      · rw [local_maximal_v W h.1 hb]; infer_instance
    exact ((tfae_of_isNoetherianRing_of_isLocalRing_of_isDomain
      (PointLocalRing W x y h.1)).out 5 1).mp hp
  toIsLocalRing := inferInstance
  not_a_field' := local_maximal_nonzero W h.1

#print axioms shifted_curve_relation
#print axioms pointLocalDVR
end
end MillerFinitePointAudit
