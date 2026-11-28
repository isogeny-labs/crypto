module
public import PicardMaximal

@[expose] public section
namespace EllipticPicardBridgeAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerFinitePointAudit FinitePointEvaluationAudit GeometricPrincipalDivisorAudit
variable {K : Type*} [Field K]

/-- Actual affine denominators of an actual full-function-field element. -/
def fractionDenominators (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    Ideal W.CoordinateRing where
  carrier := {a | ∃ b : W.CoordinateRing,
    algebraMap W.CoordinateRing W.FunctionField a * f = algebraMap W.CoordinateRing W.FunctionField b}
  zero_mem' := ⟨0,by simp⟩
  add_mem' := by
    rintro a c ⟨b,hb⟩ ⟨d,hd⟩
    exact ⟨b+d,by rw [map_add,add_mul,hb,hd,map_add]⟩
  smul_mem' := by
    rintro c a ⟨b,hb⟩
    exact ⟨c*b,by rw [smul_eq_mul,map_mul,mul_assoc,hb,map_mul]⟩

def FiniteRegular (W : WeierstrassCurve.Affine K) (f : W.FunctionField) : Prop :=
  ∀ x y : K, ∀ h : W.Nonsingular x y, 0 ≤ pointOrder W f (Point.some x y h)

/-- Actual finite regularity supplies a denominator outside the actual evaluation ideal. -/
theorem regular_denominator (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) {f : W.FunctionField} (hf : f ≠ 0)
    (hr : 0 ≤ pointOrder W f (Point.some x y h)) :
    ∃ a : W.CoordinateRing, a ∈ fractionDenominators W f ∧ a ∉ evaluationIdeal W x y h.1 := by
  have hv : finiteAdic W h f ≤ 1 := (point_order_nonnegative_iff W hf _).mp hr
  obtain ⟨a,ha⟩ := local_to_finite_surjective W h ⟨f,hv⟩
  have haf : pointToField W h.1 a = f := congrArg Subtype.val ha
  obtain ⟨⟨b,s⟩,he⟩ := IsLocalization.surj (evaluationIdeal W x y h.1).primeCompl a
  have ht := congrArg (pointToField W h.1) he
  rw [map_mul,point_to_field_map,point_to_field_map,haf] at ht
  exact ⟨s,⟨b,by rwa [mul_comm]⟩,s.property⟩

/-- No supplied normality/global-regularity premise: the actual denominator ideal and
weak Nullstellensatz give affine coordinate-ring membership. -/
theorem finite_regular_is_coordinate [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    {f : W.FunctionField} (hf : f ≠ 0) (hr : FiniteRegular W f) :
    ∃ a : W.CoordinateRing, f = algebraMap W.CoordinateRing W.FunctionField a := by
  by_contra hn
  have hd : fractionDenominators W f ≠ ⊤ := by
    intro he
    have hm : (1 : W.CoordinateRing) ∈ fractionDenominators W f := by rw [he]; trivial
    obtain ⟨b,hb⟩ := hm
    exact hn ⟨b,by simpa only [map_one,one_mul] using hb⟩
  obtain ⟨I,hI,hle⟩ := (fractionDenominators W f).exists_le_maximal hd
  let : I.IsMaximal := hI
  obtain ⟨x,y,h,he⟩ := maximal_ideal_is_actual_point W hΔ I
  obtain ⟨a,ha,hnm⟩ := regular_denominator W h hf (hr x y h)
  exact hnm (he ▸ hle ha)

/-- The actual coordinate-ring inclusion also has no finite poles. -/
theorem coordinate_finite_regular (W : WeierstrassCurve.Affine K)
    {a : W.CoordinateRing} (ha : algebraMap W.CoordinateRing W.FunctionField a ≠ 0) :
    FiniteRegular W (algebraMap W.CoordinateRing W.FunctionField a) := by
  intro x y h
  apply (point_order_nonnegative_iff W ha _).mpr
  have hv := finite_local_integral W h
    (algebraMap W.CoordinateRing (PointLocalRing W x y h.1) a)
  rwa [point_to_field_map] at hv

theorem finite_regular_iff_coordinate [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) {f : W.FunctionField} (hf : f ≠ 0) :
    FiniteRegular W f ↔ ∃ a : W.CoordinateRing,
      f = algebraMap W.CoordinateRing W.FunctionField a := by
  constructor
  · exact finite_regular_is_coordinate W hΔ hf
  · rintro ⟨a,rfl⟩
    exact coordinate_finite_regular W hf

#print axioms regular_denominator
#print axioms finite_regular_iff_coordinate
end
end EllipticPicardBridgeAudit
