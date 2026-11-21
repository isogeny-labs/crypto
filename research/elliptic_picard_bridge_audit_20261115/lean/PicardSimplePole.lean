module
public import PicardRegular

@[expose] public section
namespace EllipticPicardBridgeAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
variable {K : Type*} [Field K]

/-- The actual infinity order of a coordinate-ring function is its negative norm degree. -/
theorem coordinate_infinity_order (W : WeierstrassCurve.Affine K)
    {a : W.CoordinateRing} (ha : a ≠ 0) :
    pointOrder W (algebraMap W.CoordinateRing W.FunctionField a) 0 =
      -((Algebra.norm K[X] a).natDegree : ℤ) := by
  change -WithZero.log (chartAdicValuation W
    (algebraMap W.CoordinateRing W.FunctionField a)) = _
  rw [chart_adic_eq_norm,function_valuation_map]
  change -WithZero.log (degreeValue (Algebra.norm K[X] a).degree) = _
  rw [degree_eq_natDegree (norm_polynomial_nonzero W ha)]
  simp only [degreeValue,WithZero.log_exp,Nat.cast_id]

/-- Derived from actual affine regularity and the official norm-degree exclusion,
without a genus, Riemann-Roch or no-simple-pole premise. Every characteristic. -/
theorem no_simple_infinity_pole [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    {f : W.FunctionField} (hf : f ≠ 0) (hr : FiniteRegular W f) :
    pointOrder W f 0 ≠ -1 := by
  obtain ⟨a,he⟩ := finite_regular_is_coordinate W hΔ hf hr
  have ha : a ≠ 0 := by intro hz; exact hf (he.trans (by rw [hz,map_zero]))
  rw [he,coordinate_infinity_order W ha]
  intro hn
  have hd : (Algebra.norm K[X] a).natDegree = 1 := by omega
  exact natDegree_norm_ne_one a hd

/-- An actual point-minus-infinity divisor is principal only for the identity point. -/
theorem point_minus_origin_principal_only_zero [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (P : W.Point)
    {f : W.FunctionField} (hf : f ≠ 0)
    (hD : functionDivisor W f hf = Finsupp.single P 1 - Finsupp.single 0 1) : P = 0 := by
  classical
  by_contra hP
  have hr : FiniteRegular W f := by
    intro x y h
    have he := congrArg (fun D : W.Point →₀ ℤ => D (Point.some x y h)) hD
    change pointOrder W f (Point.some x y h) = _ at he
    rw [he]
    have h0 : (0 : W.Point) ≠ Point.some x y h := Ne.symm (Point.some_ne_zero h)
    simp only [Finsupp.sub_apply,Finsupp.single_apply,ite_eq_right h0,sub_zero]
    split_ifs <;> norm_num
  have ho := congrArg (fun D : W.Point →₀ ℤ => D 0) hD
  change pointOrder W f 0 = _ at ho
  simp only [Finsupp.sub_apply,Finsupp.single_apply,hP,ite_false,ite_true,zero_sub] at ho
  exact no_simple_infinity_pole W hΔ hf hr ho

#print axioms no_simple_infinity_pole
#print axioms point_minus_origin_principal_only_zero
end
end EllipticPicardBridgeAudit
