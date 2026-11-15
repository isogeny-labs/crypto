module
public import NegationField

@[expose] public section
namespace NegationPullbackAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerPrincipalIdealAudit MillerFinitePointAudit
variable {K : Type*} [Field K]

theorem negation_denominators (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) :
    (evaluationIdeal W x y h).primeCompl.map (coordinateNegEquiv W).toMonoidHom =
      (evaluationIdeal W x (W.negY x y) ((W.equation_neg x y).mpr h)).primeCompl := by
  ext a
  constructor
  · rintro ⟨b, hb, rfl⟩
    change pointEval W x (W.negY x y) _ (coordinateNeg W b) ≠ 0
    have he := point_eval_negation W h (coordinateNeg W b)
    rw [coordinate_neg_involutive W] at he
    exact he ▸ hb
  · intro ha
    refine ⟨coordinateNeg W a, ?_, coordinate_neg_involutive W a⟩
    change pointEval W x y h (coordinateNeg W a) ≠ 0
    rw [point_eval_negation W h a]
    exact ha

/-- Actual local-ring isomorphism induced by the actual coordinate involution. -/
def localNeg (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Equation x y) :
    PointLocalRing W x y h ≃+*
      PointLocalRing W x (W.negY x y) ((W.equation_neg x y).mpr h) :=
  IsLocalization.ringEquivOfRingEquiv _ _ (coordinateNegEquiv W) (negation_denominators W h)

theorem local_neg_map (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) (a : W.CoordinateRing) :
    localNeg W h (algebraMap W.CoordinateRing (PointLocalRing W x y h) a) =
      algebraMap W.CoordinateRing
        (PointLocalRing W x (W.negY x y) ((W.equation_neg x y).mpr h)) (coordinateNeg W a) :=
  IsLocalization.ringEquivOfRingEquiv_eq (negation_denominators W h) a

theorem local_neg_field (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) (a : PointLocalRing W x y h) :
    pointToField W ((W.equation_neg x y).mpr h) (localNeg W h a) =
      fieldNeg W (pointToField W h a) := by
  have he : (pointToField W ((W.equation_neg x y).mpr h)).comp (localNeg W h).toRingHom =
      (fieldNeg W).toRingHom.comp (pointToField W h) := by
    apply IsLocalization.ringHom_ext (evaluationIdeal W x y h).primeCompl
    apply RingHom.ext
    intro b
    change pointToField W _ (localNeg W h (algebraMap W.CoordinateRing _ b)) =
      fieldNeg W (pointToField W h (algebraMap W.CoordinateRing _ b))
    rw [local_neg_map, point_to_field_map, point_to_field_map, field_neg_map]
  exact RingHom.congr_fun he a

theorem local_neg_maximal (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) :
    Ideal.map (localNeg W h).toRingHom (IsLocalRing.maximalIdeal (PointLocalRing W x y h)) =
      IsLocalRing.maximalIdeal
        (PointLocalRing W x (W.negY x y) ((W.equation_neg x y).mpr h)) :=
  IsLocalRing.map_ringEquiv_maximalIdeal (localNeg W h)

theorem local_neg_uniformizer (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) (t : PointLocalRing W x y h)
    (ht : IsLocalRing.maximalIdeal (PointLocalRing W x y h) = Ideal.span {t}) :
    IsLocalRing.maximalIdeal
        (PointLocalRing W x (W.negY x y) ((W.equation_neg x y).mpr h)) =
      Ideal.span {localNeg W h t} := by
  have he := congrArg (Ideal.map (localNeg W h).toRingHom) ht
  rw [local_neg_maximal, Ideal.map_span, Set.image_singleton] at he
  exact he

#print axioms localNeg
#print axioms local_neg_field
end
end NegationPullbackAudit
