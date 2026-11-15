module
public import NormDegree

@[expose] public section
namespace MillerNormValuationAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open scoped WithZero nonZeroDivisors
variable {K : Type*} [Field K]

theorem nonzero_support_complement (W : WeierstrassCurve.Affine K) :
    W.CoordinateRing⁰ ≤ (coordinateValuation W).supp.primeCompl := by
  intro f hf
  change coordinateValuation W f ≠ 0
  exact (coordinate_valuation_zero_iff W f).not.mpr (nonZeroDivisors.ne_zero hf)

/-- Extend the actual norm-degree valuation to the actual fraction field. -/
def functionValuation (W : WeierstrassCurve.Affine K) : Valuation W.FunctionField ℤᵐ⁰ :=
  (coordinateValuation W).extendToLocalization (nonzero_support_complement W) W.FunctionField

theorem function_valuation_map (W : WeierstrassCurve.Affine K) (f : W.CoordinateRing) :
    functionValuation W (algebraMap W.CoordinateRing W.FunctionField f) = coordinateValuation W f :=
by
  unfold functionValuation
  exact Valuation.extendToLocalization_apply_map_apply _ _ _ _

def constantHom (W : WeierstrassCurve.Affine K) : K →+* W.FunctionField :=
  (algebraMap W.CoordinateRing W.FunctionField).comp
    ((mk W).comp (Polynomial.C.comp Polynomial.C))

def xFunction (W : WeierstrassCurve.Affine K) : W.FunctionField :=
  algebraMap W.CoordinateRing W.FunctionField (XClass W 0)

def yFunction (W : WeierstrassCurve.Affine K) : W.FunctionField :=
  algebraMap W.CoordinateRing W.FunctionField (YClass W 0)

theorem norm_degree_x (W : WeierstrassCurve.Affine K) :
    normDegree W (XClass W 0) = 2 := by
  have h := norm_degree_basis W (X : K[X]) 0
  have hn : max (2 • (X : K[X]).degree) (2 • (0 : K[X]).degree + 3) = 2 := by
    norm_num [two_nsmul]
  rw [hn] at h
  simpa only [XClass, map_zero, sub_zero, zero_smul, add_zero, smul, mul_one] using h

theorem norm_degree_y (W : WeierstrassCurve.Affine K) :
    normDegree W (YClass W 0) = 3 := by
  have h := norm_degree_basis W (0 : K[X]) 1
  have hn : max (2 • (0 : K[X]).degree) (2 • (1 : K[X]).degree + 3) = 3 := by
    norm_num [two_nsmul]
  rw [hn] at h
  simpa only [YClass, map_zero, sub_zero, zero_smul, zero_add, one_smul] using h

theorem function_valuation_x (W : WeierstrassCurve.Affine K) :
    functionValuation W (xFunction W) = WithZero.exp (2 : ℤ) := by
  rw [xFunction, function_valuation_map]
  change degreeValue (normDegree W (XClass W 0)) = _
  rw [norm_degree_x]
  rfl

theorem function_valuation_y (W : WeierstrassCurve.Affine K) :
    functionValuation W (yFunction W) = WithZero.exp (3 : ℤ) := by
  rw [yFunction, function_valuation_map]
  change degreeValue (normDegree W (YClass W 0)) = _
  rw [norm_degree_y]
  rfl

theorem function_valuation_constant (W : WeierstrassCurve.Affine K) {c : K} (hc : c ≠ 0) :
    functionValuation W (constantHom W c) = 1 := by
  change functionValuation W (algebraMap W.CoordinateRing W.FunctionField (mk W (C (C c)))) = 1
  rw [function_valuation_map]
  have h := norm_degree_basis W (C c) 0
  have hn : max (2 • (C c).degree) (2 • (0 : K[X]).degree + 3) = 0 := by
    simp [degree_C hc]
  rw [hn] at h
  simp only [zero_smul, add_zero, smul, mul_one] at h
  change degreeValue (normDegree W (mk W (C (C c)))) = 1
  rw [h]
  rfl

theorem function_valuation_constant_le (W : WeierstrassCurve.Affine K) (c : K) :
    functionValuation W (constantHom W c) ≤ 1 := by
  by_cases hc : c = 0
  · simp [hc]
  · rw [function_valuation_constant W hc]

#print axioms function_valuation_x
#print axioms function_valuation_y
#print axioms function_valuation_constant
end
end MillerNormValuationAudit
