module
public import ChartLocalization

@[expose] public section
namespace MillerInfinityChartAudit
noncomputable section
open MillerNormValuationAudit
variable {K : Type*} [Field K]

/-- Every denominator allowed at the actual chart origin maps to a unit in the constructed DVR. -/
theorem allowed_denominator_unit (W : WeierstrassCurve.Affine K)
    (s : (originIdeal W).primeCompl) :
    IsUnit (algebraMap (ChartRing W) (valuationRing W) s) := by
  apply (IsLocalRing.residue_ne_zero_iff_isUnit _).mp
  intro h
  have h0 : coefficientResidue W (originEval W s) = coefficientResidue W 0 := by
    rw [residue_originEval, map_zero]
    exact h
  exact s.property ((coefficientResidue W).injective h0)

/-- Explicit universal localization map; its existence is proved, not assumed. -/
def localToValRing (W : WeierstrassCurve.Affine K) : ChartLocalRing W →+* valuationRing W :=
  IsLocalization.lift (allowed_denominator_unit W)

theorem local_to_val_map (W : WeierstrassCurve.Affine K) (b : ChartRing W) :
    localToValRing W (algebraMap (ChartRing W) (ChartLocalRing W) b) = b :=
  IsLocalization.lift_eq _ b

theorem local_to_val_injective (W : WeierstrassCurve.Affine K) :
    Function.Injective (localToValRing W) := by
  unfold localToValRing
  rw [IsLocalization.lift_injective_iff]
  intro a b
  have hl := IsLocalization.injective (S := ChartLocalRing W)
    (originIdeal W).primeCompl_le_nonZeroDivisors
  constructor
  · intro h
    exact congrArg (algebraMap (ChartRing W) (valuationRing W)) (hl h)
  · intro h
    have hab : a = b := Subtype.val_injective h
    exact congrArg (algebraMap (ChartRing W) (ChartLocalRing W)) hab

def chartToField (W : WeierstrassCurve.Affine K) : ChartRing W →+* W.FunctionField :=
  (valuationRing W).subtype.comp (chartAlgebra W).val.toRingHom

def localToField (W : WeierstrassCurve.Affine K) : ChartLocalRing W →+* W.FunctionField :=
  (valuationRing W).subtype.comp (localToValRing W)

theorem chart_to_field_injective (W : WeierstrassCurve.Affine K) :
    Function.Injective (chartToField W) :=
  Subtype.val_injective.comp Subtype.val_injective

theorem local_to_field_injective (W : WeierstrassCurve.Affine K) :
    Function.Injective (localToField W) :=
  Subtype.val_injective.comp (local_to_val_injective W)

theorem local_to_field_map (W : WeierstrassCurve.Affine K) (b : ChartRing W) :
    localToField W (algebraMap (ChartRing W) (ChartLocalRing W) b) = chartToField W b := by
  change (localToValRing W (algebraMap (ChartRing W) (ChartLocalRing W) b) : W.FunctionField) = _
  rw [local_to_val_map]
  rfl

theorem chart_to_field_t (W : WeierstrassCurve.Affine K) :
    chartToField W (tCoord W) = parameter W := rfl

theorem chart_to_field_z (W : WeierstrassCurve.Affine K) :
    chartToField W (zCoord W) = (yFunction W)⁻¹ := rfl

theorem chart_to_field_constant (W : WeierstrassCurve.Affine K) (c : K) :
    chartToField W (algebraMap K (ChartRing W) c) = constantHom W c := rfl

theorem local_to_field_t (W : WeierstrassCurve.Affine K) :
    localToField W (localT W) = parameter W := by
  rw [localT, local_to_field_map, chart_to_field_t]

instance chartFieldAlgebra (W : WeierstrassCurve.Affine K) : Algebra (ChartRing W) W.FunctionField :=
  (chartToField W).toAlgebra

instance localFieldAlgebra (W : WeierstrassCurve.Affine K) : Algebra (ChartLocalRing W) W.FunctionField :=
  (localToField W).toAlgebra

theorem chartFieldFaithful (W : WeierstrassCurve.Affine K) :
    FaithfulSMul (ChartRing W) W.FunctionField :=
  (faithfulSMul_iff_algebraMap_injective _ _).mpr (chart_to_field_injective W)

theorem localFieldFaithful (W : WeierstrassCurve.Affine K) :
    FaithfulSMul (ChartLocalRing W) W.FunctionField :=
  (faithfulSMul_iff_algebraMap_injective _ _).mpr (local_to_field_injective W)

#print axioms local_to_val_injective
#print axioms local_to_field_t
end
end MillerInfinityChartAudit
