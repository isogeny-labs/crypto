import DoublingRational

namespace EllipticDoublingDegreeAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open EllipticTranslationAudit EllipticFieldNormAudit EllipticFrobeniusDegreeAudit
open MillerNormValuationAudit FunctionDivisorFinitenessAudit MillerInfinityChartAudit
open scoped EllipticFieldNormAudit
variable {K : Type*} [Field K]

theorem rational_embedding_X (W : WeierstrassCurve.Affine K) :
    ellipticRationalEmbedding W (RatFunc.X : RatFunc K)=xFunction W := by
  change algebraMap (RatFunc K) W.FunctionField
    (algebraMap K[X] (RatFunc K) X)=xFunction W
  rw [rational_base_polynomial]
  simp only [polynomialFunction,RingHom.comp_apply,xFunction,XClass,map_zero,sub_zero]

theorem rational_embedding_polynomial (W : WeierstrassCurve.Affine K) (p : K[X]) :
    ellipticRationalEmbedding W (algebraMap K[X] (RatFunc K) p)=
      aeval (xFunction W) p := by
  rw [show ellipticRationalEmbedding W (algebraMap K[X] (RatFunc K) p)=
    polynomialFunction W p from rational_base_polynomial W p]
  change algebraMap W.CoordinateRing W.FunctionField (mk W (C p))=aeval (xFunction W) p
  rw [polynomial_function_image]
  rfl

theorem actual_duplication_denominator_nonzero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    aeval (xFunction W) (doublingDenominator W)≠0 := by
  simpa only [map_zero] using (transcendental_iff_injective.mp (actual_x_transcendental W)).ne
    (doubling_denominator_ne_zero W hΔ)

def doubledGeneric (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : (genericCurve W).Point :=
  EllipticTranslationAudit.genericPoint W hΔ+EllipticTranslationAudit.genericPoint W hΔ

theorem generic_not_its_negative (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    yFunction W≠(genericCurve W).negY (xFunction W) (yFunction W) := by
  intro h
  have hz := (den_duplication_eq_zero_iff (generic_curve_equation W)).mpr h
  apply actual_duplication_denominator_nonzero W hΔ
  simpa [doublingDenominator,genericCurve,MillerBasechangeAudit.extendedCurve,
    WeierstrassCurve.map_b₂,WeierstrassCurve.map_b₄,WeierstrassCurve.map_b₆,
    function_coefficient_map,map_ofNat] using hz

theorem doubled_generic_nonzero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    doubledGeneric W hΔ≠0 := by
  rw [doubledGeneric,EllipticTranslationAudit.genericPoint,Point.add_self_of_Y_ne (generic_not_its_negative W hΔ)]
  exact Point.some_ne_zero _

def doubledX (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : W.FunctionField :=
  genericX W (doubledGeneric W hΔ)

def doubledY (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : W.FunctionField :=
  genericY W (doubledGeneric W hΔ)

theorem actual_doubled_x (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    doubledX W hΔ=ellipticRationalEmbedding W (doublingRational W) := by
  rw [doubledX,doubledGeneric,EllipticTranslationAudit.genericPoint,
    Point.add_self_of_Y_ne (generic_not_its_negative W hΔ)]
  change (genericCurve W).addX _ _ _ = _
  rw [addX_self_of_Y_ne (generic_curve_equation W) (generic_not_its_negative W hΔ),
    doublingRational,map_div₀,rational_embedding_polynomial,rational_embedding_polynomial]
  simp [doublingNumerator,doublingDenominator,genericCurve,MillerBasechangeAudit.extendedCurve,
    WeierstrassCurve.map_b₂,WeierstrassCurve.map_b₄,WeierstrassCurve.map_b₆,
    WeierstrassCurve.map_b₈,function_coefficient_map,map_ofNat]

theorem doubled_x_transcendental (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Transcendental K (doubledX W hΔ) := by
  rw [actual_doubled_x]
  intro h
  exact doubling_rational_transcendental W hΔ
    ((isAlgebraic_algHom_iff (ellipticRationalEmbedding W)
      (ellipticRationalEmbedding W).injective).mp h)

#print axioms actual_doubled_x
#print axioms doubled_x_transcendental
end
end EllipticDoublingDegreeAudit
