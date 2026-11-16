module
public import FiniteFractionField

@[expose] public section
namespace MillerFinitePointAudit
noncomputable section
open scoped WithZero
variable {K : Type*} [Field K]

local instance pointMonoid (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : Monoid (PointLocalRing W x y h) :=
  (inferInstance : CommRing (PointLocalRing W x y h)).toMonoid

local instance pointFractionRing (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : IsFractionRing (PointLocalRing W x y h) W.FunctionField :=
  pointFunctionFractionRing W h

def pointParameter (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Nonsingular x y) :
    PointLocalRing W x y h.1 := by
  classical
  exact if derivativeY W x y ≠ 0 then localU W h.1 else localV W h.1

theorem point_parameter_maximal (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) :
    IsLocalRing.maximalIdeal (PointLocalRing W x y h.1) = Ideal.span {pointParameter W h} := by
  classical
  unfold pointParameter
  split_ifs with ha
  · exact local_maximal_u W h.1 ha
  · exact local_maximal_v W h.1 ((nonsingular_derivatives W h).resolve_left ha)

theorem point_parameter_nonzero (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) : pointParameter W h ≠ 0 := by
  classical
  unfold pointParameter
  split_ifs
  · exact local_u_nonzero W h.1
  · exact local_v_nonzero W h.1

/-- Official normalized height-one adic valuation of the actual finite-point local ring. -/
def finiteAdic (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Nonsingular x y) :
    Valuation W.FunctionField ℤᵐ⁰ := by
  letI := pointLocalDVR W h
  exact (IsDiscreteValuationRing.maximalIdeal (PointLocalRing W x y h.1)).valuation W.FunctionField

theorem finite_adic_parameter (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) :
    finiteAdic W h (pointToField W h.1 (pointParameter W h)) = WithZero.exp (-1 : ℤ) := by
  let := pointLocalDVR W h
  change (IsDiscreteValuationRing.maximalIdeal (PointLocalRing W x y h.1)).valuation
    W.FunctionField (algebraMap (PointLocalRing W x y h.1) W.FunctionField (pointParameter W h)) = _
  rw [IsDedekindDomain.HeightOneSpectrum.valuation_of_algebraMap]
  exact (IsDiscreteValuationRing.maximalIdeal (PointLocalRing W x y h.1)).intValuation_singleton
    (point_parameter_nonzero W h) (point_parameter_maximal W h)

theorem finite_adic_unit (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (u : (PointLocalRing W x y h.1)ˣ) :
    finiteAdic W h (pointToField W h.1 u) = 1 := by
  let := pointLocalDVR W h
  change (IsDiscreteValuationRing.maximalIdeal (PointLocalRing W x y h.1)).valuation
    W.FunctionField (algebraMap (PointLocalRing W x y h.1) W.FunctionField u) = _
  apply (IsDedekindDomain.HeightOneSpectrum.valuation_eq_one_iff_notMem
    (v := IsDiscreteValuationRing.maximalIdeal (PointLocalRing W x y h.1))
    (K := W.FunctionField)).mpr
  change (u : PointLocalRing W x y h.1) ∉ IsLocalRing.maximalIdeal (PointLocalRing W x y h.1)
  simpa only [IsLocalRing.mem_maximalIdeal, mem_nonunits_iff, not_not] using u.isUnit

/-- Exact actual local ideal multiplicity gives the normalized actual function-field valuation. -/
theorem finite_adic_of_principal_power (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (a : PointLocalRing W x y h.1) (n : ℕ)
    (he : Ideal.span {a} = IsLocalRing.maximalIdeal (PointLocalRing W x y h.1) ^ n) :
    finiteAdic W h (pointToField W h.1 a) = WithZero.exp (-(n : ℤ)) := by
  have hs : Ideal.span {pointParameter W h ^ n} = Ideal.span {a} := by
    rw [← Ideal.span_singleton_pow, ← point_parameter_maximal, he]
  obtain ⟨u, hu⟩ := Ideal.span_singleton_eq_span_singleton.mp hs
  rw [← hu, map_mul, map_pow, map_mul, map_pow, finite_adic_parameter, finite_adic_unit,
    mul_one, ← WithZero.exp_nsmul]
  congr 1
  simp

theorem finite_order_of_principal_power (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (a : PointLocalRing W x y h.1) (n : ℕ)
    (he : Ideal.span {a} = IsLocalRing.maximalIdeal (PointLocalRing W x y h.1) ^ n) :
    -WithZero.log (finiteAdic W h (pointToField W h.1 a)) = (n : ℤ) := by
  rw [finite_adic_of_principal_power W h a n he, WithZero.log_exp, neg_neg]

#print axioms finite_adic_of_principal_power
end
end MillerFinitePointAudit
