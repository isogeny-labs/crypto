import TangentCharacter

namespace ThreeTangentPairingAudit
noncomputable section
open WeierstrassCurve.Affine BKThreeNormalizationAudit
  ThreeTorsionRankAudit.General ThreeTorsionCharacterAudit Module
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {K : Type*} [Field K] [DecidableEq K]

theorem model_character_cube (W : WeierstrassCurve.Affine K)
    (h2 : W.a₂=0) (h4 : W.a₄=0) (h6 : W.a₆=0)
    (Q : W.Point) (hQ : (3:ℕ) • Q=0) : (modelValue W Q)^3=1 := by
  rcases W with ⟨a,a2,b,a4,a6⟩
  change a2=0 at h2
  change a4=0 at h4
  change a6=0 at h6
  subst a2; subst a4; subst a6
  exact pointValue_cube a b Q hQ

theorem tangentPair_cube (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : ThreeTorsion W) : (tangentPair W P.val Q.val)^3=1 := by
  rcases P with ⟨P,hP⟩
  cases P with
  | zero => simp [tangentPair]
  | some x y hN =>
    rw [normalized_pair_character W hΔ x y hN hP Q.val Q.property]
    obtain ⟨h2,h4,h6⟩ := normalized_coefficients W x y hN hP
    apply model_character_cube (normalized W x y) h2 h4 h6
    let e : (normalized W x y).Point ≃+ W.Point := shiftEquiv W hΔ x (tangentSlope W x y) y
    change (3:ℕ) • e.symm Q.val=0
    rw [←map_nsmul,Q.property,map_zero]

theorem tangentPair_ne_zero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : ThreeTorsion W) : tangentPair W P.val Q.val≠0 := by
  intro hz
  have h := tangentPair_cube W hΔ P Q
  rw [hz,zero_pow (by decide : (3:ℕ)≠0)] at h
  exact zero_ne_one h

variable [IsAlgClosed K] [NeZero (3:K)]

theorem tangentPair_left_nondegenerate (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P : ThreeTorsion W) (h : ∀ Q : ThreeTorsion W, tangentPair W P.val Q.val=1) : P=0 := by
  rcases P with ⟨P,hP⟩
  cases P with
  | zero => rfl
  | some x y hN =>
    let N := normalized W x y
    let e : N.Point ≃+ W.Point := shiftEquiv W hΔ x (tangentSlope W x y) y
    obtain ⟨h2,h4,h6⟩ := normalized_coefficients W x y hN hP
    have hd : N.Δ≠0 := by change (normalized W x y).Δ≠0; rwa [normalized_discriminant]
    have hb := third_coefficient_ne_zero N h2 h4 h6 hd
    obtain ⟨b,_,hn,_,_⟩ := model_character_basis N h2 h4 h6 hb hd
    let Q : ThreeTorsion W := ⟨e (b 1).val,by
      change (3:ℕ) • e (b 1).val=0
      rw [←map_nsmul,(b 1).property,map_zero]⟩
    have hh := h Q
    rw [normalized_pair_character W hΔ x y hN hP Q.val Q.property] at hh
    change modelValue N (e.symm (e (b 1).val))=1 at hh
    rw [e.symm_apply_apply] at hh
    exact (hn hh).elim

theorem tangentPair_right_nondegenerate (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : ThreeTorsion W) (h : ∀ P : ThreeTorsion W, tangentPair W P.val Q.val=1) : Q=0 := by
  apply tangentPair_left_nondegenerate W hΔ Q
  intro P
  rw [tangentPair_swap,h P,inv_one]

theorem tangentPair_nsmul_left (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : ThreeTorsion W) (n : ℕ) :
    tangentPair W (n • P).val Q.val=(tangentPair W P.val Q.val)^n := by
  induction n with
  | zero => simp [tangentPair]
  | succ n ih => rw [succ_nsmul,tangentPair_add_left W hΔ,ih,pow_succ]

theorem tangentPair_nsmul_right (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : ThreeTorsion W) (n : ℕ) :
    tangentPair W P.val (n • Q).val=(tangentPair W P.val Q.val)^n := by
  induction n with
  | zero => cases P.val <;> simp [tangentPair]
  | succ n ih => rw [succ_nsmul,tangentPair_add_right W hΔ,ih,pow_succ]

#print axioms tangentPair_cube
#print axioms tangentPair_left_nondegenerate
#print axioms tangentPair_right_nondegenerate
end
end ThreeTangentPairingAudit
