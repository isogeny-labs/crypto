import ThreeNormalForm

namespace ThreeTangentPairingAudit
noncomputable section
open BKThreeNormalizationAudit ThreeTorsionRankAudit.General ThreeTorsionCharacterAudit
variable {K : Type*} [Field K]

/-- Tangent line with coefficient one on the evaluated y-coordinate. -/
def tangentValue (W : WeierstrassCurve.Affine K) (x y u v : K) : K :=
  v-y-tangentSlope W x y*(u-x)

theorem shifted_tangentSlope (W : WeierstrassCurve.Affine K) (r s t x y : K)
    (hd : denominator (shifted W r s t) x y≠0) :
    tangentSlope W (x+r) (y+s*x+t)=tangentSlope (shifted W r s t) x y+s := by
  unfold tangentSlope
  have hn := numerator_identity W r s t x y
  have he := denominator_identity W r s t x y
  change numerator W (x+r) (y+s*x+t)=
    numerator (shifted W r s t) x y+s*denominator (shifted W r s t) x y at hn
  change denominator W (x+r) (y+s*x+t)=denominator (shifted W r s t) x y at he
  rw [hn,he]
  field_simp

theorem shifted_tangentValue (W : WeierstrassCurve.Affine K) (r s t x y u v : K)
    (hd : denominator (shifted W r s t) x y≠0) :
    tangentValue W (x+r) (y+s*x+t) (u+r) (v+s*u+t)=
      tangentValue (shifted W r s t) x y u v := by
  unfold tangentValue
  rw [shifted_tangentSlope W r s t x y hd]
  ring

theorem model_tangent_origin (a b x y : K) :
    tangentValue (curve a b) 0 0 x y=y := by
  simp [tangentValue,tangentSlope,numerator,denominator,curve]

theorem model_tangent_to_origin (a b x y : K)
    (he : equation a b x y=0) (hd : d a b x y≠0) :
    tangentValue (curve a b) x y 0 0=y*t a b x y/d a b x y := by
  simp only [tangentValue,tangentSlope,numerator,denominator,curve,zero_sub,
    mul_zero,zero_mul,add_zero]
  change -y-(3*x^2-a*y)/d a b x y*(-x)=_
  field_simp
  unfold equation at he
  unfold d t
  linear_combination -3*he

theorem model_tangent_to_origin_nonzero (a b x y : K)
    (hc : cubic a b x=0) (he : equation a b x y=0)
    (hd : d a b x y≠0) (hy : y≠0) :
    tangentValue (curve a b) x y 0 0≠0 := by
  rw [model_tangent_to_origin a b x y he hd]
  exact div_ne_zero (mul_ne_zero hy (character_denominator_nonzero a b x y hc he hd)) hd

theorem model_tangent_ratio (a b x y : K)
    (hc : cubic a b x=0) (he : equation a b x y=0)
    (hd : d a b x y≠0) (hy : y≠0) :
    -tangentValue (curve a b) 0 0 x y/tangentValue (curve a b) x y 0 0=
      characterValue a b x y := by
  rw [model_tangent_origin,model_tangent_to_origin a b x y he hd]
  have ht := character_denominator_nonzero a b x y hc he hd
  unfold characterValue
  field_simp

#print axioms shifted_tangentValue
#print axioms model_tangent_ratio
end
end ThreeTangentPairingAudit
