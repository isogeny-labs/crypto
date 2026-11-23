module
public import TranslationGeneration

@[expose] public section
namespace EllipticTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerBasechangeAudit
variable {K : Type*} [Field K]

/-- Explicit domain/codomain adapter for the official actual coefficient-field point map.
Its body is the official Point.map; no point-map law is supplied as a premise. -/
def coefficientPointMap (W : WeierstrassCurve.Affine K)
    (φ : W.FunctionField →ₐ[K] W.FunctionField) :
    (genericCurve W).Point →+ (genericCurve W).Point := Point.map (W' := W) φ

theorem coefficient_point_map_injective (W : WeierstrassCurve.Affine K)
    (φ : W.FunctionField →ₐ[K] W.FunctionField) : Function.Injective (coefficientPointMap W φ) :=
  Point.map_injective (W' := W) φ

theorem generic_coordinates_rebuild (W : WeierstrassCurve.Affine K)
    (P : (genericCurve W).Point) (hP : P ≠ 0) :
    Point.some (genericX W P) (genericY W P) (generic_coordinates_nonsingular W P hP)=P := by
  cases P with
  | zero => exact (hP rfl).elim
  | some x y h => rfl

/-- Official actual point-map naturality of the actual coordinate accessors, including infinity. -/
theorem generic_x_map (W : WeierstrassCurve.Affine K)
    (φ : W.FunctionField →ₐ[K] W.FunctionField) (P : (genericCurve W).Point) :
    genericX W (coefficientPointMap W φ P)=φ (genericX W P) := by
  cases P with
  | zero => exact (map_zero φ).symm
  | some x y h => rfl

theorem generic_y_map (W : WeierstrassCurve.Affine K)
    (φ : W.FunctionField →ₐ[K] W.FunctionField) (P : (genericCurve W).Point) :
    genericY W (coefficientPointMap W φ P)=φ (genericY W P) := by
  cases P with
  | zero => exact (map_zero φ).symm
  | some x y h => rfl

theorem generic_points_eq_of_coordinates (W : WeierstrassCurve.Affine K)
    {P R : (genericCurve W).Point} (hP : P ≠ 0) (hR : R ≠ 0)
    (hx : genericX W P=genericX W R) (hy : genericY W P=genericY W R) : P=R := by
  cases P with
  | zero => exact (hP rfl).elim
  | some x y h =>
    cases R with
    | zero => exact (hR rfl).elim
    | some a b ha =>
      simp only [Point.some.injEq]
      exact ⟨hx,hy⟩

variable [DecidableEq K] [IsAlgClosed K]

def fieldTranslationAlgHom (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (Q : W.Point) :
    W.FunctionField →ₐ[K] W.FunctionField where
  __ := fieldTranslation W hΔ Q
  toFun := fun f => fieldTranslation W hΔ Q f
  commutes' c := by
    rw [function_coefficient_map]
    exact field_translation_constant W hΔ Q c

/-- The actual field lift sends the actual generic point to its actual constant translate. -/
theorem field_point_generic (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (Q : W.Point) :
    coefficientPointMap W (fieldTranslationAlgHom W hΔ Q) (genericPoint W hΔ)=
      translatedGeneric W hΔ Q := by
  apply generic_points_eq_of_coordinates W
  · intro hz
    apply Point.some_ne_zero (generic_nonsingular W hΔ)
    change genericPoint W hΔ=0
    apply coefficient_point_map_injective W (fieldTranslationAlgHom W hΔ Q)
    simpa only [map_zero] using hz
  · exact translated_generic_nonzero W hΔ Q
  · rw [generic_x_map]
    exact field_translation_x W hΔ Q
  · rw [generic_y_map]
    exact field_translation_y W hΔ Q

/-- Actual coefficient points are fixed by the actual coefficient-algebra map. -/
theorem field_point_constant (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q R : W.Point) :
    coefficientPointMap W (fieldTranslationAlgHom W hΔ Q) (constantPoint W R)=constantPoint W R :=
  Point.map_baseChange (W' := W) (fieldTranslationAlgHom W hΔ Q) R

/-- Actual point-map composition on the actual translated generic point. -/
theorem field_point_translated (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q R : W.Point) :
    coefficientPointMap W (fieldTranslationAlgHom W hΔ Q) (translatedGeneric W hΔ R)=
      translatedGeneric W hΔ (Q+R) := by
  rw [translatedGeneric,map_add,field_point_generic,field_point_constant]
  simp only [translatedGeneric,map_add]
  abel

#print axioms field_point_generic
#print axioms field_point_translated
end
end EllipticTranslationAudit
