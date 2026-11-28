import TangentScalar

namespace ThreeTangentPairingAudit
noncomputable section
open WeierstrassCurve.Affine BKThreeNormalizationAudit
  ThreeTorsionRankAudit.General ThreeTorsionCharacterAudit
variable {K : Type*} [Field K] [DecidableEq K]

/-- Explicit convention; equal-x and infinity pairs receive the alternating value. -/
def tangentPair (W : WeierstrassCurve.Affine K) : W.Point → W.Point → K
  | .some x y _, .some u v _ =>
      if x=u then 1 else -tangentValue W x y u v/tangentValue W u v x y
  | _,_ => 1

theorem tangentPair_self (W : WeierstrassCurve.Affine K) (P : W.Point) :
    tangentPair W P P=1 := by
  cases P <;> simp [tangentPair]

theorem tangentPair_swap (W : WeierstrassCurve.Affine K) (P Q : W.Point) :
    tangentPair W P Q=(tangentPair W Q P)⁻¹ := by
  cases P with
  | zero => cases Q <;> simp [tangentPair]
  | some x y h =>
    cases Q with
    | zero => simp [tangentPair]
    | some u v h' =>
      by_cases hx : x=u
      · simp [tangentPair,hx]
      · simp only [tangentPair,ite_eq_right hx,ite_eq_right (Ne.symm hx)]
        simp [div_eq_mul_inv,mul_comm]

theorem model_tangentPair_origin (a b : K) (hb : b≠0)
    (Q : (curve a b).Point) (hQ : (3:ℕ) • Q=0) :
    tangentPair (curve a b) (origin a b hb) Q=pointValue a b Q := by
  cases Q with
  | zero => rfl
  | some x y hN =>
    by_cases hx : x=0
    · simp [tangentPair,origin,pointValue,modelValue,hx]
    · obtain ⟨hc,he,hd,hy⟩ := outside_data a b x y hN hQ hx
      simp only [tangentPair,origin,ite_eq_right (Ne.symm hx),pointValue,modelValue,ite_eq_right hx]
      exact model_tangent_ratio a b x y hc he hd hy

theorem shift_tangentPair (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (r s t : K) (P Q : (shifted W r s t).Point)
    (hP : (3:ℕ) • P=0) (hQ : (3:ℕ) • Q=0) :
    tangentPair W (pointShift W hΔ r s t P) (pointShift W hΔ r s t Q)=
      tangentPair (shifted W r s t) P Q := by
  cases P with
  | zero => cases Q <;> rfl
  | some x y hN =>
    cases Q with
    | zero => rfl
    | some u v hM =>
      have hd := psi_root_denominator_nonzero (shifted W r s t) hN
        ((three_point_iff (shifted W r s t) hN).mp hP)
      have he := psi_root_denominator_nonzero (shifted W r s t) hM
        ((three_point_iff (shifted W r s t) hM).mp hQ)
      simp only [pointShift,tangentPair,add_left_inj,
        shifted_tangentValue W r s t x y u v hd,
        shifted_tangentValue W r s t u v x y he]

#print axioms tangentPair_swap
#print axioms model_tangentPair_origin
#print axioms shift_tangentPair
end
end ThreeTangentPairingAudit
