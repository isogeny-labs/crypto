module
public import Normalization

@[expose] public section
namespace MillerNormValuationAudit
noncomputable section
open scoped WithZero
variable {K : Type*} [Field K]

/-- The actual valuation subring of the constructed function-field valuation. -/
def valuationRing (W : WeierstrassCurve.Affine K) : ValuationSubring W.FunctionField :=
  (functionValuation W).valuationSubring

theorem residue_one_of_small_difference {L : Type*} [Field L] (v : Valuation L ℤᵐ⁰)
    (a : v.valuationSubring) (h : v ((a : L) - 1) < 1) :
    IsLocalRing.residue v.valuationSubring a = 1 := by
  have hm : a - 1 ∈ IsLocalRing.maximalIdeal v.valuationSubring :=
    (Valuation.mem_maximalIdeal_iff (K := L) (v := v)).mpr h
  have hz := (IsLocalRing.residue_eq_zero_iff (a - 1)).mpr hm
  simpa only [map_sub, map_one, sub_eq_zero] using hz

theorem normalized_line_value (W : WeierstrassCurve.Affine K) (x y slope : K) :
    functionValuation W (parameter W ^ 3 * lineFunction W x y slope) = 1 := by
  have h : functionValuation W (parameter W ^ 3 * lineFunction W x y slope - 1) <
      functionValuation W 1 := by
    simpa only [map_one] using line_normalized W x y slope
  simpa only [map_one] using (functionValuation W).map_eq_of_sub_lt h

theorem normalized_vertical_value (W : WeierstrassCurve.Affine K) (x : K) :
    functionValuation W (parameter W ^ 2 * verticalFunction W x) = 1 := by
  have h : functionValuation W (parameter W ^ 2 * verticalFunction W x - 1) <
      functionValuation W 1 := by
    simpa only [map_one] using vertical_normalized W x
  simpa only [map_one] using (functionValuation W).map_eq_of_sub_lt h

def normalizedLine (W : WeierstrassCurve.Affine K) (x y slope : K) : valuationRing W :=
  ⟨parameter W ^ 3 * lineFunction W x y slope, (normalized_line_value W x y slope).le⟩

def normalizedVertical (W : WeierstrassCurve.Affine K) (x : K) : valuationRing W :=
  ⟨parameter W ^ 2 * verticalFunction W x, (normalized_vertical_value W x).le⟩

theorem line_residue_one (W : WeierstrassCurve.Affine K) (x y slope : K) :
    IsLocalRing.residue (valuationRing W) (normalizedLine W x y slope) = 1 :=
  residue_one_of_small_difference (functionValuation W) _ (line_normalized W x y slope)

theorem vertical_residue_one (W : WeierstrassCurve.Affine K) (x : K) :
    IsLocalRing.residue (valuationRing W) (normalizedVertical W x) = 1 :=
  residue_one_of_small_difference (functionValuation W) _ (vertical_normalized W x)

#print axioms line_residue_one
#print axioms vertical_residue_one
end
end MillerNormValuationAudit
