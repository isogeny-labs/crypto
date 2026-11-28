import FieldNormMinpoly

namespace EllipticFieldNormAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerInfinityChartAudit FunctionDivisorFinitenessAudit NegationPullbackAudit
open scoped EllipticFieldNormAudit
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

theorem quadratic_derivative (W : WeierstrassCurve.Affine K) :
    (functionQuadratic W).derivative=C (2 : RatFunc K)*X+
      C (algebraMap K[X] (RatFunc K) (linearH W)) := by
  simp [functionQuadratic,derivative_pow]

/-- Exact inseparable boundary, obtained from the actual derivative and actual polynomial injection. -/
theorem quadratic_derivative_zero (W : WeierstrassCurve.Affine K) :
    (functionQuadratic W).derivative=0 ↔ (2 : K)=0 ∧ W.a₁=0 ∧ W.a₃=0 := by
  rw [quadratic_derivative]
  constructor
  · intro h
    have h2 : (2 : RatFunc K)=0 := by
      have he := congrArg (fun p : (RatFunc K)[X] => p.coeff 1) h
      simpa using he
    have hh : algebraMap K[X] (RatFunc K) (linearH W)=0 := by
      have he := congrArg (fun p : (RatFunc K)[X] => p.coeff 0) h
      simpa using he
    have hlin : linearH W=0 := (IsFractionRing.injective K[X] (RatFunc K)) (by simpa only [map_zero] using hh)
    refine ⟨?_,?_,?_⟩
    · apply (algebraMap K (RatFunc K)).injective
      simpa only [map_ofNat,map_zero] using h2
    · have he := congrArg (fun p : K[X] => p.coeff 1) hlin
      simpa [linearH] using he
    · have he := congrArg (fun p : K[X] => p.coeff 0) hlin
      simpa [linearH] using he
  · rintro ⟨h2,h1,h3⟩
    have h2' : (2 : RatFunc K)=0 := by simpa only [map_ofNat,map_zero] using congrArg (algebraMap K (RatFunc K)) h2
    simp [linearH,h1,h3,h2']

theorem actual_y_separable_iff (W : WeierstrassCurve.Affine K) :
    IsSeparable (RatFunc K) (yFunction W) ↔ ¬((2 : K)=0 ∧ W.a₁=0 ∧ W.a₃=0) := by
  change (minpoly (RatFunc K) (yFunction W)).Separable ↔ _
  rw [actual_y_minpoly,separable_iff_derivative_ne_zero (function_quadratic_irreducible W)]
  exact not_congr (quadratic_derivative_zero W)

/-- The actual basis1,y transfers separability of the generator to every actual field element. -/
theorem field_separable_iff_generator (W : WeierstrassCurve.Affine K) :
    Algebra.IsSeparable (RatFunc K) W.FunctionField ↔ IsSeparable (RatFunc K) (yFunction W) := by
  constructor
  · intro h
    let _ : Algebra.IsSeparable (RatFunc K) W.FunctionField := h
    exact Algebra.IsSeparable.isSeparable (RatFunc K) (yFunction W)
  · intro hy
    constructor
    intro f
    obtain ⟨a,b,hf⟩ := rational_linear_representation W f
    rw [←hf]
    exact Field.isSeparable_add (isSeparable_algebraMap a)
      (Field.isSeparable_mul (isSeparable_algebraMap b) hy)

theorem actual_field_separable_iff (W : WeierstrassCurve.Affine K) :
    Algebra.IsSeparable (RatFunc K) W.FunctionField ↔ ¬((2 : K)=0 ∧ W.a₁=0 ∧ W.a₃=0) := by
  rw [field_separable_iff_generator,actual_y_separable_iff]

theorem inseparable_boundary_delta (W : WeierstrassCurve.Affine K)
    (h2 : (2 : K)=0) (h1 : W.a₁=0) (h3 : W.a₃=0) : W.Δ=0 := by
  have h4 : (4 : K)=0 := by linear_combination 2*h2
  simp [WeierstrassCurve.Δ,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
    WeierstrassCurve.b₆,h1,h3,h2,h4]

/-- Every nonsingular general model gives a separable quadratic extension, including char2. -/
theorem nonsingular_field_separable (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Algebra.IsSeparable (RatFunc K) W.FunctionField := by
  apply (actual_field_separable_iff W).mpr
  rintro ⟨h2,h1,h3⟩
  exact hΔ (inseparable_boundary_delta W h2 h1 h3)

#print axioms actual_y_separable_iff
#print axioms actual_field_separable_iff
#print axioms nonsingular_field_separable
end
end EllipticFieldNormAudit
