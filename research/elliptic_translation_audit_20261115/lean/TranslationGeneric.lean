module
public import PicardCertificate

@[expose] public section
namespace EllipticTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerBasechangeAudit MillerRecursionAudit
variable {K : Type*} [Field K]

-- Reuse the official localization coefficient algebra; use its actual field ring projection
-- consistently for actual point maps, as in the credited finite-field controls.
noncomputable abbrev functionFieldCommRing (W : WeierstrassCurve.Affine K) :
    CommRing W.FunctionField := (inferInstance : Field W.FunctionField).toCommRing
attribute [instance] functionFieldCommRing
noncomputable instance functionFieldDecidableEq (W : WeierstrassCurve.Affine K) :
    DecidableEq W.FunctionField := Classical.decEq _

theorem function_coefficient_map (W : WeierstrassCurve.Affine K) (c : K) :
    algebraMap K W.FunctionField c=constantHom W c := rfl

abbrev genericCurve (W : WeierstrassCurve.Affine K) :=
  extendedCurve (L := W.FunctionField) W

theorem generic_curve_delta (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    (genericCurve W).Δ ≠ 0 := by
  rw [genericCurve,extendedCurve,WeierstrassCurve.map_Δ]
  simpa only [map_zero] using (algebraMap K W.FunctionField).injective.ne hΔ

theorem generic_curve_equation (W : WeierstrassCurve.Affine K) :
    (genericCurve W).Equation (xFunction W) (yFunction W) := by
  rw [(genericCurve W).equation_iff]
  simp only [genericCurve,extendedCurve,WeierstrassCurve.map,function_coefficient_map]

  linear_combination function_curve_relation W

theorem generic_nonsingular (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    (genericCurve W).Nonsingular (xFunction W) (yFunction W) :=
  (equation_iff_nonsingular_of_Δ_ne_zero (generic_curve_delta W hΔ)).mp (generic_curve_equation W)

/-- The actual generic point, with coordinates in the actual elliptic function field. -/
def genericPoint (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) : (genericCurve W).Point :=
  Point.some (xFunction W) (yFunction W) (generic_nonsingular W hΔ)

theorem x_function_ne_constant (W : WeierstrassCurve.Affine K) (c : K) :
    xFunction W ≠ constantHom W c := by
  intro he
  apply vertical_nonzero W c
  rw [vertical_formula,he,sub_self]

variable [DecidableEq K]

/-- The actual coefficient extension homomorphism on actual points. -/
def constantPoint (W : WeierstrassCurve.Affine K) : W.Point →+ (genericCurve W).Point :=
  pointBaseChange (L := W.FunctionField) W

/-- The actual generic point is outside the image of every actual coefficient point. -/
theorem generic_point_not_constant (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (P : W.Point) : genericPoint W hΔ ≠ constantPoint W P := by
  intro he
  cases P with
  | zero =>
    have hz : genericPoint W hΔ=0 := he.trans (map_zero (constantPoint W))
    exact Point.some_ne_zero (generic_nonsingular W hΔ) hz
  | some x y h =>
    rw [constantPoint,point_base_some] at he
    have hx := (Point.some.inj he).1
    rw [function_coefficient_map] at hx
    exact x_function_ne_constant W x hx

def translatedGeneric (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (Q : W.Point) :
    (genericCurve W).Point := genericPoint W hΔ+constantPoint W Q

theorem translated_generic_not_constant (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q R : W.Point) : translatedGeneric W hΔ Q ≠ constantPoint W R := by
  intro he
  apply generic_point_not_constant W hΔ (R-Q)
  calc
    genericPoint W hΔ = (genericPoint W hΔ+constantPoint W Q)-constantPoint W Q :=
      (add_sub_cancel_right _ _).symm
    _ = constantPoint W R-constantPoint W Q := congrArg (fun T => T-constantPoint W Q) he
    _ = constantPoint W (R-Q) := (map_sub (constantPoint W) R Q).symm

theorem translated_generic_nonzero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) : translatedGeneric W hΔ Q ≠ 0 := by
  intro hz
  apply translated_generic_not_constant W hΔ Q 0
  rw [hz,map_zero]

#print axioms generic_point_not_constant
#print axioms translated_generic_nonzero
end
end EllipticTranslationAudit
