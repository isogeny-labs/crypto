import DoublingGeneric

namespace EllipticDoublingDegreeAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open EllipticTranslationAudit EllipticFieldNormAudit EllipticFrobeniusDegreeAudit
open MillerNormValuationAudit FunctionDivisorFinitenessAudit MillerInfinityChartAudit NegationPullbackAudit
open scoped EllipticFieldNormAudit
variable {K : Type*} [Field K]

theorem doubled_root_equation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    W.polynomial.eval₂ (aeval (doubledX W hΔ)).toRingHom (doubledY W hΔ)=0 := by
  have he := (generic_coordinates_nonsingular W (doubledGeneric W hΔ)
    (doubled_generic_nonzero W hΔ)).1
  rw [(genericCurve W).equation_iff] at he
  simp only [genericCurve,MillerBasechangeAudit.extendedCurve,WeierstrassCurve.map,function_coefficient_map] at he
  change doubledY W hΔ^2+constantHom W W.a₁*doubledX W hΔ*doubledY W hΔ+
    constantHom W W.a₃*doubledY W hΔ = doubledX W hΔ^3+
    constantHom W W.a₂*doubledX W hΔ^2+constantHom W W.a₄*doubledX W hΔ+
    constantHom W W.a₆ at he
  simp only [WeierstrassCurve.Affine.polynomial,eval₂_sub,eval₂_add,eval₂_mul,
    eval₂_pow,eval₂_X,eval₂_C,AlgHom.toRingHom_eq_coe,AlgHom.coe_toRingHom,
    map_add,map_mul,aeval_C,aeval_X,map_pow,function_coefficient_map]
  linear_combination he

def doublingCoordinate (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    W.CoordinateRing →+* W.FunctionField :=
  AdjoinRoot.lift (aeval (doubledX W hΔ)).toRingHom (doubledY W hΔ) (doubled_root_equation W hΔ)

theorem doubling_coordinate_polynomial (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (p : K[X]) :
    doublingCoordinate W hΔ (mk W (C p))=aeval (doubledX W hΔ) p :=
  AdjoinRoot.lift_of (doubled_root_equation W hΔ)

theorem doubling_coordinate_root (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    doublingCoordinate W hΔ (mk W X)=doubledY W hΔ :=
  AdjoinRoot.lift_root (doubled_root_equation W hΔ)

theorem doubling_coordinate_injective (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Function.Injective (doublingCoordinate W hΔ) := by
  apply (injective_iff_map_eq_zero _).mpr
  intro a ha
  by_contra hn
  have he := congrArg (doublingCoordinate W hΔ) (coordinate_norm_product W a)
  rw [doubling_coordinate_polynomial,map_mul,ha,zero_mul] at he
  have hi : Function.Injective (aeval (doubledX W hΔ) : K[X] →ₐ[K] W.FunctionField) :=
    transcendental_iff_injective.mp (doubled_x_transcendental W hΔ)
  have hz : Algebra.norm K[X] a=0 := hi (he.trans (map_zero _).symm)
  exact norm_polynomial_nonzero W hn hz

def doublingFieldMap (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    W.FunctionField →+* W.FunctionField := IsFractionRing.lift (doubling_coordinate_injective W hΔ)

theorem doubling_field_coordinate (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (a : W.CoordinateRing) :
    doublingFieldMap W hΔ (algebraMap W.CoordinateRing W.FunctionField a)=doublingCoordinate W hΔ a :=
  IsFractionRing.lift_algebraMap _ a

theorem doubling_field_constant (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (c : K) :
    doublingFieldMap W hΔ (constantHom W c)=constantHom W c := by
  change doublingFieldMap W hΔ
    (algebraMap W.CoordinateRing W.FunctionField (mk W (C (C c))))=constantHom W c
  rw [doubling_field_coordinate,doubling_coordinate_polynomial,aeval_C,function_coefficient_map]

def doublingAlgHom (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    W.FunctionField →ₐ[K] W.FunctionField where
  __ := doublingFieldMap W hΔ
  toFun := fun f => doublingFieldMap W hΔ f
  commutes' c := doubling_field_constant W hΔ c

theorem doubling_field_x (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    doublingAlgHom W hΔ (xFunction W)=doubledX W hΔ := by
  change doublingFieldMap W hΔ (xFunction W)=doubledX W hΔ
  have he := doubling_field_coordinate W hΔ (mk W (C X))
  rw [doubling_coordinate_polynomial,aeval_X] at he
  simpa only [xFunction,XClass,map_zero,sub_zero] using he

theorem doubling_field_y (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    doublingAlgHom W hΔ (yFunction W)=doubledY W hΔ := by
  change doublingFieldMap W hΔ (yFunction W)=doubledY W hΔ
  have he := doubling_field_coordinate W hΔ (mk W X)
  rw [doubling_coordinate_root] at he
  simpa only [yFunction,YClass,map_zero,sub_zero] using he

theorem doubling_generic_naturality (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    coefficientPointMap W (doublingAlgHom W hΔ) (EllipticTranslationAudit.genericPoint W hΔ)=doubledGeneric W hΔ := by
  apply generic_points_eq_of_coordinates W
  · intro hz
    apply Point.some_ne_zero (generic_nonsingular W hΔ)
    change EllipticTranslationAudit.genericPoint W hΔ=0
    apply coefficient_point_map_injective W (doublingAlgHom W hΔ)
    simpa only [map_zero] using hz
  · exact doubled_generic_nonzero W hΔ
  · rw [generic_x_map]
    exact doubling_field_x W hΔ
  · rw [generic_y_map]
    exact doubling_field_y W hΔ

def doublingImage (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    IntermediateField K W.FunctionField := (doublingAlgHom W hΔ).fieldRange

/-- A generic actual field-map degree lemma, instantiated below with the constructed map. -/
theorem fieldmap_rational_degree (W : WeierstrassCurve.Affine K)
    (σ : W.FunctionField →ₐ[K] W.FunctionField) (f : RatFunc K)
    (hx : σ (xFunction W)=ellipticRationalEmbedding W f) :
    Module.finrank σ.fieldRange W.FunctionField=
      Module.finrank (IntermediateField.adjoin K {f}) (RatFunc K) := by
  have hH : ellipticRationalImage W=IntermediateField.adjoin K {xFunction W} := by
    rw [ellipticRationalImage,AlgHom.fieldRange_eq_map,←RatFunc.adjoin_X,
      IntermediateField.adjoin_map,Set.image_singleton,rational_embedding_X]
  have hm : (ellipticRationalImage W).map σ=
      (IntermediateField.adjoin K {f}).map (ellipticRationalEmbedding W) := by
    rw [hH,IntermediateField.adjoin_map,Set.image_singleton,hx,
      IntermediateField.adjoin_map,Set.image_singleton]
  have hstable : (ellipticRationalImage W).map σ≤ellipticRationalImage W := by
    rw [hm,ellipticRationalImage,AlgHom.fieldRange_eq_map]
    exact IntermediateField.map_mono _ le_top
  rw [field_image_degree_transfer σ (ellipticRationalImage W) hstable 2 (by decide)
    (actual_rational_image_degree_two W),hm,ellipticRationalImage,
    AlgHom.fieldRange_eq_map (ellipticRationalEmbedding W),
    IntermediateField.relfinrank_map_map,IntermediateField.relfinrank_top_right]

theorem actual_doubling_degree (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Module.finrank (doublingImage W hΔ) W.FunctionField=4 := by
  exact (fieldmap_rational_degree W (doublingAlgHom W hΔ) (doublingRational W)
    ((doubling_field_x W hΔ).trans (actual_doubled_x W hΔ))).trans (doubling_rational_degree W hΔ)

theorem actual_doubling_finite (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Module.Finite (doublingImage W hΔ) W.FunctionField := by
  apply Module.finite_of_finrank_pos
  rw [actual_doubling_degree]
  decide

#print axioms doubling_coordinate_injective
#print axioms actual_doubling_degree
end
end EllipticDoublingDegreeAudit
