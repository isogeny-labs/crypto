module
public import LocalComparison
public import Mathlib.Algebra.Field.Subfield.Basic

@[expose] public section
namespace MillerInfinityChartAudit
noncomputable section
open MillerNormValuationAudit Polynomial
open WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
variable {K : Type*} [Field K]

def fractionToField (W : WeierstrassCurve.Affine K) :
    FractionRing (ChartRing W) →+* W.FunctionField :=
  IsFractionRing.lift (g := chartToField W) (chart_to_field_injective W)

theorem fraction_map_chart (W : WeierstrassCurve.Affine K) (b : ChartRing W) :
    fractionToField W (algebraMap (ChartRing W) (FractionRing (ChartRing W)) b) = chartToField W b :=
  IsFractionRing.lift_algebraMap _ _

theorem fraction_contains_chart (W : WeierstrassCurve.Affine K) (b : ChartRing W) :
    chartToField W b ∈ (fractionToField W).fieldRange :=
  ⟨algebraMap (ChartRing W) (FractionRing (ChartRing W)) b, fraction_map_chart W b⟩

theorem polynomial_function_image (W : WeierstrassCurve.Affine K) (p : K[X]) :
    algebraMap W.CoordinateRing W.FunctionField (mk W (C p)) =
      p.eval₂ (constantHom W) (xFunction W) := by
  have he : (algebraMap W.CoordinateRing W.FunctionField).comp ((mk W).comp C) =
      Polynomial.eval₂RingHom (constantHom W) (xFunction W) := by
    apply Polynomial.ringHom_ext
    · intro c
      change algebraMap W.CoordinateRing W.FunctionField (mk W (C (C c))) =
        (C c).eval₂ (constantHom W) (xFunction W)
      rw [eval₂_C]
      rfl
    · change algebraMap W.CoordinateRing W.FunctionField (mk W (C X)) =
        X.eval₂ (constantHom W) (xFunction W)
      rw [eval₂_X]
      simp only [xFunction, XClass, map_zero, sub_zero]
  exact RingHom.congr_fun he p

/-- The chart fraction field contains the full actual elliptic function field. -/
theorem fraction_field_range_full (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    f ∈ (fractionToField W).fieldRange := by
  let S := (fractionToField W).fieldRange
  have hc (c : K) : constantHom W c ∈ S := by
    simpa only [chart_to_field_constant] using
      fraction_contains_chart W (algebraMap K (ChartRing W) c)
  have ht : parameter W ∈ S := by
    simpa only [chart_to_field_t] using fraction_contains_chart W (tCoord W)
  have hz : (yFunction W)⁻¹ ∈ S := by
    simpa only [chart_to_field_z] using fraction_contains_chart W (zCoord W)
  have hy : yFunction W ∈ S := by
    simpa only [inv_inv] using S.inv_mem hz
  have hx : xFunction W ∈ S := by
    have he : parameter W / (yFunction W)⁻¹ = xFunction W := by
      simp [parameter, div_inv_eq_mul, y_function_nonzero W]
    rw [← he]
    exact S.div_mem ht hz
  have hp (p : K[X]) : p.eval₂ (constantHom W) (xFunction W) ∈ S := by
    induction p using Polynomial.induction_on' with
    | add p q ihp ihq =>
      rw [eval₂_add]
      exact S.add_mem ihp ihq
    | monomial n c =>
      rw [eval₂_monomial]
      exact S.mul_mem (hc c) (S.pow_mem hx n)
  have hcoordinate (a : W.CoordinateRing) : algebraMap W.CoordinateRing W.FunctionField a ∈ S := by
    obtain ⟨p, q, rfl⟩ := exists_smul_basis_eq a
    have h := S.add_mem (hp p) (S.mul_mem (hp q) hy)
    simpa only [smul, map_add, map_mul, map_one, mul_one, polynomial_function_image,
      yFunction, YClass, map_zero, sub_zero] using h
  obtain ⟨a, b, _hb, hab⟩ := IsFractionRing.div_surjective W.CoordinateRing f
  rw [← hab]
  exact S.div_mem (hcoordinate a) (hcoordinate b)

theorem fraction_to_field_surjective (W : WeierstrassCurve.Affine K) :
    Function.Surjective (fractionToField W) :=
  fun f => fraction_field_range_full W f

/-- Fraction-field equality is derived from actual chart coordinate functions and actual fractions. -/
theorem chartFunctionFractionRing (W : WeierstrassCurve.Affine K) :
    IsFractionRing (ChartRing W) W.FunctionField := by
  refine @IsFractionRing.of_field (ChartRing W) inferInstance W.FunctionField inferInstance
    (chartFieldAlgebra W) (chartFieldFaithful W) ?_
  intro f
  obtain ⟨a, ha⟩ := fraction_to_field_surjective W f
  obtain ⟨u, v, _hv, huv⟩ := IsFractionRing.div_surjective (ChartRing W) a
  refine ⟨u, v, ?_⟩
  change f = chartToField W u / chartToField W v
  rw [← ha, ← huv, map_div₀, fraction_map_chart, fraction_map_chart]

theorem localFunctionFractionRing (W : WeierstrassCurve.Affine K) :
    IsFractionRing (ChartLocalRing W) W.FunctionField := by
  refine @IsFractionRing.of_field (ChartLocalRing W) inferInstance W.FunctionField inferInstance
    (localFieldAlgebra W) (localFieldFaithful W) ?_
  intro f
  obtain ⟨a, b, _hb, hab⟩ := @IsFractionRing.div_surjective (ChartRing W) inferInstance
    W.FunctionField inferInstance (chartFieldAlgebra W) (chartFunctionFractionRing W) f
  refine ⟨algebraMap (ChartRing W) (ChartLocalRing W) a,
    algebraMap (ChartRing W) (ChartLocalRing W) b, ?_⟩
  change f = localToField W _ / localToField W _
  rw [local_to_field_map, local_to_field_map]
  exact hab.symm

#print axioms fraction_to_field_surjective
#print axioms polynomial_function_image
end
end MillerInfinityChartAudit
