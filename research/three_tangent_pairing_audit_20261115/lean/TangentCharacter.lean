import TangentPoint

namespace ThreeTangentPairingAudit
noncomputable section
open WeierstrassCurve.Affine BKThreeNormalizationAudit
  ThreeTorsionRankAudit.General ThreeTorsionCharacterAudit Module
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {K : Type*} [Field K] [DecidableEq K]

theorem model_origin_character (W : WeierstrassCurve.Affine K)
    (h2 : W.a₂=0) (h4 : W.a₄=0) (h6 : W.a₆=0) (hb : W.a₃≠0)
    (hN : W.Nonsingular 0 0) (Q : W.Point) (hQ : (3:ℕ) • Q=0) :
    tangentPair W (Point.some 0 0 hN) Q=modelValue W Q := by
  rcases W with ⟨a,a2,b,a4,a6⟩
  change a2=0 at h2
  change a4=0 at h4
  change a6=0 at h6
  subst a2; subst a4; subst a6
  exact model_tangentPair_origin a b hb Q hQ

theorem normalized_pair_character (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (x y : K) (hN : W.Nonsingular x y) (hP : (3:ℕ) • Point.some x y hN=0)
    (Q : W.Point) (hQ : (3:ℕ) • Q=0) :
    tangentPair W (Point.some x y hN) Q=
      modelValue (normalized W x y) ((shiftEquiv W hΔ x (tangentSlope W x y) y).symm Q) := by
  let N := normalized W x y
  let e : N.Point ≃+ W.Point := shiftEquiv W hΔ x (tangentSlope W x y) y
  have h0 : N.Nonsingular 0 0 := by
    change (shifted W x (tangentSlope W x y) y).Nonsingular 0 0
    have h := unshifted_nonsingular W hΔ x (tangentSlope W x y) y x y hN
    simpa only [sub_self,mul_zero,sub_zero] using h
  have hmap : e (Point.some 0 0 h0)=Point.some x y hN := by
    change Point.some (0+x) (0+tangentSlope W x y*0+y) _=Point.some x y hN
    simp only [zero_add,mul_zero,add_zero]
  have h0k : (3:ℕ) • Point.some 0 0 h0=0 := by
    have hm := map_nsmul e 3 (Point.some 0 0 h0)
    rw [hmap,hP] at hm
    exact e.injective (hm.trans e.map_zero.symm)
  have hqk : (3:ℕ) • e.symm Q=0 := by rw [←map_nsmul,hQ,map_zero]
  have hs := shift_tangentPair W hΔ x (tangentSlope W x y) y (Point.some 0 0 h0) (e.symm Q) h0k hqk
  change tangentPair W (e (Point.some 0 0 h0)) (e (e.symm Q))=_ at hs
  rw [hmap,e.apply_symm_apply] at hs
  rw [hs]
  obtain ⟨h2,h4,h6⟩ := normalized_coefficients W x y hN hP
  exact model_origin_character N h2 h4 h6
    (third_coefficient_ne_zero N h2 h4 h6 (by rwa [normalized_discriminant])) h0 _ hqk

omit [DecidableEq K] in
theorem cube_pow_add (z : K) (hz : z^3=1) (i j : ZMod 3) :
    z^(i+j).val=z^i.val*z^j.val := by
  rw [ZMod.val_add,←pow_add]
  exact (pow_eq_pow_mod (i.val+j.val) hz).symm

variable [IsAlgClosed K] [NeZero (3:K)]

theorem model_character_add (W : WeierstrassCurve.Affine K)
    (h2 : W.a₂=0) (h4 : W.a₄=0) (h6 : W.a₆=0) (hΔ : W.Δ≠0)
    (P Q : ThreeTorsion W) :
    modelValue W (P+Q).val=modelValue W P.val*modelValue W Q.val := by
  have hb := third_coefficient_ne_zero W h2 h4 h6 hΔ
  obtain ⟨b,_,_,hz,hcoord⟩ := model_character_basis W h2 h4 h6 hb hΔ
  rw [hcoord (P+Q),hcoord P,hcoord Q,map_add]
  exact cube_pow_add _ hz _ _

theorem tangentPair_add_right (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q R : ThreeTorsion W) :
    tangentPair W P.val (Q+R).val=tangentPair W P.val Q.val*tangentPair W P.val R.val := by
  rcases P with ⟨P,hP⟩
  cases P with
  | zero => simp [tangentPair]
  | some x y hN =>
    let N := normalized W x y
    let e : N.Point ≃+ W.Point := shiftEquiv W hΔ x (tangentSlope W x y) y
    let q : ThreeTorsion N := ⟨e.symm Q.val,by
      change (3:ℕ) • e.symm Q.val=0
      rw [←map_nsmul,Q.property,map_zero]⟩
    let r : ThreeTorsion N := ⟨e.symm R.val,by
      change (3:ℕ) • e.symm R.val=0
      rw [←map_nsmul,R.property,map_zero]⟩
    rw [normalized_pair_character W hΔ x y hN hP (Q+R).val (Q+R).property,
      normalized_pair_character W hΔ x y hN hP Q.val Q.property,
      normalized_pair_character W hΔ x y hN hP R.val R.property]
    change modelValue N (e.symm (Q.val+R.val))=modelValue N q.val*modelValue N r.val
    rw [map_add]
    obtain ⟨h2,h4,h6⟩ := normalized_coefficients W x y hN hP
    exact model_character_add N h2 h4 h6 (by rwa [normalized_discriminant]) q r

theorem tangentPair_add_left (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q R : ThreeTorsion W) :
    tangentPair W (P+Q).val R.val=tangentPair W P.val R.val*tangentPair W Q.val R.val := by
  rw [tangentPair_swap W (P+Q).val R.val,tangentPair_add_right W hΔ R P Q,
    mul_inv,←tangentPair_swap W P.val R.val,←tangentPair_swap W Q.val R.val]

#print axioms normalized_pair_character
#print axioms tangentPair_add_right
#print axioms tangentPair_add_left
end
end ThreeTangentPairingAudit
