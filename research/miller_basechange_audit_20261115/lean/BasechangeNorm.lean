module
public import BasechangeFunctions

@[expose] public section
namespace MillerBasechangeAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit
variable {K L : Type*} [Field K] [Field L] [Algebra K L]

theorem coordinate_norm_basechange (W : WeierstrassCurve.Affine K) (a : W.CoordinateRing) :
    Algebra.norm L[X] (CoordinateRing.map W (algebraMap K L) a) =
      (Algebra.norm K[X] a).map (algebraMap K L) := by
  obtain ⟨p, q, rfl⟩ := exists_smul_basis_eq a
  rw [map_add, CoordinateRing.map_smul, CoordinateRing.map_smul, map_one,
    CoordinateRing.map_mk, Polynomial.map_X, norm_smul_basis, norm_smul_basis]
  simp only [Polynomial.map_sub, Polynomial.map_mul, Polynomial.map_pow, Polynomial.map_add,
    Polynomial.map_C, Polynomial.map_X]
  rfl

theorem norm_degree_basechange (W : WeierstrassCurve.Affine K) (a : W.CoordinateRing) :
    normDegree (extendedCurve (L := L) W) (CoordinateRing.map W (algebraMap K L) a) =
      normDegree W a := by
  unfold normDegree
  rw [coordinate_norm_basechange]
  exact Polynomial.degree_map_eq_of_injective (algebraMap K L).injective (Algebra.norm K[X] a)

/-- Actual infinity valuation, including its normalization, survives constant-field extension. -/
theorem function_valuation_basechange (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    functionValuation (extendedCurve (L := L) W) (fieldBaseChange (L := L) W f) =
      functionValuation W f := by
  obtain ⟨a, b, _hb, rfl⟩ := IsFractionRing.div_surjective W.CoordinateRing f
  simp only [map_div₀, field_base_map, function_valuation_map]
  change degreeValue (normDegree (extendedCurve (L := L) W)
      (CoordinateRing.map W (algebraMap K L) a)) /
      degreeValue (normDegree (extendedCurve (L := L) W)
        (CoordinateRing.map W (algebraMap K L) b)) =
    degreeValue (normDegree W a) / degreeValue (normDegree W b)
  rw [norm_degree_basechange, norm_degree_basechange]

theorem infinity_adic_basechange (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    chartAdicValuation (extendedCurve (L := L) W) (fieldBaseChange (L := L) W f) =
      chartAdicValuation W f := by
  simpa only [chart_adic_eq_norm] using function_valuation_basechange (L := L) W f

theorem infinity_order_basechange (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    pointOrder (extendedCurve (L := L) W) (fieldBaseChange (L := L) W f) 0 =
      pointOrder W f 0 := by
  change -WithZero.log (chartAdicValuation (extendedCurve (L := L) W)
    (fieldBaseChange (L := L) W f)) = -WithZero.log (chartAdicValuation W f)
  rw [infinity_adic_basechange]

#print axioms coordinate_norm_basechange
#print axioms infinity_adic_basechange
end
end MillerBasechangeAudit
