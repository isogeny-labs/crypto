module
public import FiniteValuation

@[expose] public section
namespace MillerFinitePointAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerPrincipalIdealAudit
variable {K : Type*} [Field K]

def coordinateMultiplicity (xr yr x y : K) : ℕ := by
  classical
  exact if xr = x ∧ yr = y then 1 else 0

/-- Localizing a point ideal gives the actual maximal ideal at that point, and the unit ideal
at every other actual affine point. The comparison follows from actual coordinate evaluation. -/
theorem localized_point_ideal (W : WeierstrassCurve.Affine K) {xr yr : K}
    (hR : W.Equation xr yr) (x y : K) :
    Ideal.map (algebraMap W.CoordinateRing (PointLocalRing W xr yr hR))
      (XYIdeal W x (C y)) =
      IsLocalRing.maximalIdeal (PointLocalRing W xr yr hR) ^ coordinateMultiplicity xr yr x y := by
  classical
  unfold coordinateMultiplicity
  split_ifs with hxy
  · rcases hxy with ⟨rfl, rfl⟩
    rw [pow_one, ← evaluation_ideal_coordinates W hR]
    exact Localization.AtPrime.map_eq_maximalIdeal
  · rw [pow_zero, Ideal.one_eq_top]
    let f := algebraMap W.CoordinateRing (PointLocalRing W xr yr hR)
    by_cases hx : xr = x
    · have hy : yr ≠ y := fun hy => hxy ⟨hx, hy⟩
      have hu : IsUnit (f (YClass W (C y))) :=
        (IsLocalization.AtPrime.isUnit_to_map_iff (PointLocalRing W xr yr hR)
          (evaluationIdeal W xr yr hR) (YClass W (C y))).mpr (by
            change pointEval W xr yr hR (YClass W (C y)) ≠ 0
            rw [point_eval_YClass, eval_C]
            exact sub_ne_zero.mpr hy)
      exact Ideal.eq_top_of_isUnit_mem _
        (Ideal.mem_map_of_mem f (Ideal.subset_span (by simp))) hu
    · have hu : IsUnit (f (XClass W x)) :=
        (IsLocalization.AtPrime.isUnit_to_map_iff (PointLocalRing W xr yr hR)
          (evaluationIdeal W xr yr hR) (XClass W x)).mpr (by
            change pointEval W xr yr hR (XClass W x) ≠ 0
            rw [point_eval_XClass]
            exact sub_ne_zero.mpr hx)
      exact Ideal.eq_top_of_isUnit_mem _
        (Ideal.mem_map_of_mem f (Ideal.subset_span (by simp))) hu

variable [DecidableEq K]

def chordMultiplicity (W : WeierstrassCurve.Affine K) (xr yr x₁ y₁ x₂ y₂ : K) : ℕ :=
  coordinateMultiplicity xr yr x₁ y₁ + coordinateMultiplicity xr yr x₂ y₂ +
    coordinateMultiplicity xr yr (W.addX x₁ x₂ (W.slope x₁ x₂ y₁ y₂))
      (W.negY (W.addX x₁ x₂ (W.slope x₁ x₂ y₁ y₂))
        (W.addY x₁ x₂ y₁ (W.slope x₁ x₂ y₁ y₂)))

theorem localized_chord_principal (W : WeierstrassCurve.Affine K) {xr yr x₁ y₁ x₂ y₂ : K}
    (hR : W.Equation xr yr) (h₁ : W.Nonsingular x₁ y₁) (h₂ : W.Nonsingular x₂ y₂)
    (hxy : ¬ (x₁ = x₂ ∧ y₁ = W.negY x₂ y₂)) :
    Ideal.span {(algebraMap W.CoordinateRing (PointLocalRing W xr yr hR)
      (lineClass W x₁ y₁ (W.slope x₁ x₂ y₁ y₂)))} =
      IsLocalRing.maximalIdeal (PointLocalRing W xr yr hR) ^
        chordMultiplicity W xr yr x₁ y₁ x₂ y₂ := by
  let f := algebraMap W.CoordinateRing (PointLocalRing W xr yr hR)
  have he := congrArg (Ideal.map f) (chord_principal_ideal W h₁ h₂ hxy)
  simp only [f, pointIdeal, Ideal.map_mul, localized_point_ideal, lineIdeal,
    Ideal.map_span, Set.image_singleton] at he
  rw [← he, chordMultiplicity, pow_add, pow_add]

/-- Every finite zero order of the actual chord/tangent function, with repeated points counted. -/
theorem finite_chord_order (W : WeierstrassCurve.Affine K) {xr yr x₁ y₁ x₂ y₂ : K}
    (hR : W.Nonsingular xr yr) (h₁ : W.Nonsingular x₁ y₁) (h₂ : W.Nonsingular x₂ y₂)
    (hxy : ¬ (x₁ = x₂ ∧ y₁ = W.negY x₂ y₂)) :
    -WithZero.log (finiteAdic W hR
      (MillerNormValuationAudit.lineFunction W x₁ y₁ (W.slope x₁ x₂ y₁ y₂))) =
      (chordMultiplicity W xr yr x₁ y₁ x₂ y₂ : ℤ) := by
  change -WithZero.log (finiteAdic W hR
    (algebraMap W.CoordinateRing W.FunctionField
      (lineClass W x₁ y₁ (W.slope x₁ x₂ y₁ y₂)))) = _
  rw [← point_to_field_map W hR.1]
  exact finite_order_of_principal_power W hR _ _ (localized_chord_principal W hR.1 h₁ h₂ hxy)

omit [DecidableEq K] in
theorem localized_vertical_principal (W : WeierstrassCurve.Affine K) {xr yr x y : K}
    (hR : W.Equation xr yr) (h : W.Nonsingular x y) :
    Ideal.span {algebraMap W.CoordinateRing (PointLocalRing W xr yr hR) (XClass W x)} =
      IsLocalRing.maximalIdeal (PointLocalRing W xr yr hR) ^
        (coordinateMultiplicity xr yr x (W.negY x y) + coordinateMultiplicity xr yr x y) := by
  let f := algebraMap W.CoordinateRing (PointLocalRing W xr yr hR)
  have he := congrArg (Ideal.map f) (XYIdeal_neg_mul h)
  simp only [f, Ideal.map_mul, localized_point_ideal, XIdeal,
    Ideal.map_span, Set.image_singleton] at he
  rw [← he, pow_add]

omit [DecidableEq K] in
/-- The actual vertical function has order two at a two-torsion point and order one at
each member of a distinct negative pair, as a single formula. -/
theorem finite_vertical_order (W : WeierstrassCurve.Affine K) {xr yr x y : K}
    (hR : W.Nonsingular xr yr) (h : W.Nonsingular x y) :
    -WithZero.log (finiteAdic W hR (MillerNormValuationAudit.verticalFunction W x)) =
      ((coordinateMultiplicity xr yr x (W.negY x y) + coordinateMultiplicity xr yr x y : ℕ) : ℤ) := by
  change -WithZero.log (finiteAdic W hR
    (algebraMap W.CoordinateRing W.FunctionField (XClass W x))) = _
  rw [← point_to_field_map W hR.1]
  exact finite_order_of_principal_power W hR _ _ (localized_vertical_principal W hR.1 h)

#print axioms finite_chord_order
#print axioms finite_vertical_order
end
end MillerFinitePointAudit
