import FieldNormQuadratic

namespace EllipticFieldNormAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open EllipticTranslationAudit MillerNormValuationAudit MillerInfinityChartAudit
open FunctionDivisorFinitenessAudit NegationPullbackAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]

/-- Actual x is transcendental over the actual coefficient field. -/
theorem actual_x_transcendental (W : WeierstrassCurve.Affine K) : Transcendental K (xFunction W) := by
  apply transcendental_iff_injective.mpr
  have he : (aeval (xFunction W) : K[X] →ₐ[K] W.FunctionField).toRingHom=polynomialFunction W := by
    apply RingHom.ext
    intro p
    change p.eval₂ (algebraMap K W.FunctionField) (xFunction W)=polynomialFunction W p
    have hc : algebraMap K W.FunctionField=constantHom W := RingHom.ext (function_coefficient_map W)
    rw [hc]
    exact (polynomial_function_image W p).symm
  change Function.Injective (aeval (xFunction W) : K[X] →ₐ[K] W.FunctionField).toRingHom
  rw [he]
  exact polynomial_function_injective W

/-- The coefficient-field extension is not finite; it is not the quadratic base. -/
theorem coefficient_not_finite (W : WeierstrassCurve.Affine K) : ¬Module.Finite K W.FunctionField := by
  intro h
  let _ : Module.Finite K W.FunctionField := h
  exact actual_x_transcendental W (IsAlgebraic.of_finite K (xFunction W))

/-- Official determinant norm defaults to1 on this wrong infinite-dimensional base. -/
theorem coefficient_norm_default (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    Algebra.norm K f=1 := Algebra.norm_eq_one_of_not_module_finite (coefficient_not_finite W) f

/-- Actual finite-base norm of x has a genuine pole, and is therefore not1. -/
theorem actual_x_norm_not_one (W : WeierstrassCurve.Affine K) :
    algebraMap (RatFunc K) W.FunctionField (Algebra.norm (RatFunc K) (xFunction W))≠1 := by
  rw [actual_field_norm_product,field_neg_x]
  intro he
  have hv : 1<functionValuation W (xFunction W*xFunction W) := by
    rw [map_mul,function_valuation_x]
    norm_num
  rw [he,map_one] at hv
  exact (lt_self_iff_false 1).mp hv

/-- Actual all-model mismatch: the coefficient-base default cannot replace the field norm. -/
theorem wrong_base_norm_counter (W : WeierstrassCurve.Affine K) :
    Algebra.norm K (xFunction W)=1 ∧
      algebraMap (RatFunc K) W.FunctionField (Algebra.norm (RatFunc K) (xFunction W))≠
        constantHom W (Algebra.norm K (xFunction W)) := by
  rw [coefficient_norm_default,map_one]
  exact ⟨rfl,actual_x_norm_not_one W⟩

private instance : Fact (Nat.Prime 5) := ⟨by decide⟩

/-- Concrete credited nonsingular F5 curve, not an abstract supplied norm value. -/
theorem actual_f5_wrong_base_control :
    MillerPrincipalIdealAudit.controlCurve.Δ≠0 ∧
      Algebra.norm (ZMod 5) (xFunction MillerPrincipalIdealAudit.controlCurve)=1 ∧
      algebraMap (RatFunc (ZMod 5)) MillerPrincipalIdealAudit.controlCurve.FunctionField
        (Algebra.norm (RatFunc (ZMod 5)) (xFunction MillerPrincipalIdealAudit.controlCurve))≠1 :=
  ⟨MillerPrincipalIdealAudit.control_delta,coefficient_norm_default _ _,actual_x_norm_not_one _⟩

#print axioms actual_x_transcendental
#print axioms coefficient_not_finite
#print axioms wrong_base_norm_counter
#print axioms actual_f5_wrong_base_control
end
end EllipticFieldNormAudit
