module
public import FiniteDVR

@[expose] public section
namespace MillerFinitePointAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
variable {K : Type*} [Field K]

theorem point_denominator_unit (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) (s : (evaluationIdeal W x y h).primeCompl) :
    IsUnit (algebraMap W.CoordinateRing W.FunctionField s) := by
  apply isUnit_iff_ne_zero.mpr
  intro hz
  have hs : (s : W.CoordinateRing) = 0 := by
    apply IsFractionRing.injective W.CoordinateRing W.FunctionField
    simpa only [map_zero] using hz
  exact s.property (hs ▸ Ideal.zero_mem _)

/-- Explicit inclusion of the actual finite-point localization into the actual function field. -/
def pointToField (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Equation x y) :
    PointLocalRing W x y h →+* W.FunctionField :=
  IsLocalization.lift (point_denominator_unit W h)

theorem point_to_field_map (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) (a : W.CoordinateRing) :
    pointToField W h (algebraMap W.CoordinateRing (PointLocalRing W x y h) a) =
      algebraMap W.CoordinateRing W.FunctionField a :=
  IsLocalization.lift_eq _ a

theorem point_to_field_injective (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : Function.Injective (pointToField W h) := by
  unfold pointToField
  rw [IsLocalization.lift_injective_iff]
  intro a b
  have hi := IsLocalization.injective (S := PointLocalRing W x y h)
    (evaluationIdeal W x y h).primeCompl_le_nonZeroDivisors
  constructor
  · intro he
    exact congrArg (algebraMap W.CoordinateRing W.FunctionField) (hi he)
  · intro he
    exact congrArg (algebraMap W.CoordinateRing (PointLocalRing W x y h))
      (IsFractionRing.injective W.CoordinateRing W.FunctionField he)

instance pointFieldAlgebra (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : Algebra (PointLocalRing W x y h) W.FunctionField :=
  (pointToField W h).toAlgebra

theorem pointFieldFaithful (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : FaithfulSMul (PointLocalRing W x y h) W.FunctionField :=
  (faithfulSMul_iff_algebraMap_injective _ _).mpr (point_to_field_injective W h)

/-- Full fraction field is proved from actual affine fractions, with no field-size assumption. -/
theorem pointFunctionFractionRing (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : IsFractionRing (PointLocalRing W x y h) W.FunctionField := by
  refine @IsFractionRing.of_field (PointLocalRing W x y h) inferInstance
    W.FunctionField inferInstance (pointFieldAlgebra W h) (pointFieldFaithful W h) ?_
  intro f
  obtain ⟨a, b, _hb, hab⟩ := IsFractionRing.div_surjective W.CoordinateRing f
  refine ⟨algebraMap W.CoordinateRing (PointLocalRing W x y h) a,
    algebraMap W.CoordinateRing (PointLocalRing W x y h) b, ?_⟩
  change f = pointToField W h _ / pointToField W h _
  rw [point_to_field_map, point_to_field_map]
  exact hab.symm

#print axioms point_to_field_injective
#print axioms pointFunctionFractionRing
end
end MillerFinitePointAudit
