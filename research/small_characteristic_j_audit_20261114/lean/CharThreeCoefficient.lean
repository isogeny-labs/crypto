import DirectSource

namespace SmallCharacteristicJAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General
variable {K : Type*} [Field K] [DecidableEq K] [CharP K 3]

omit [DecidableEq K] in
theorem psi_char_three (W : WeierstrassCurve.Affine K) (x : K) :
    W.Ψ₃.eval x=W.b₂*x^3+W.b₈ := by
  simp only [WeierstrassCurve.Ψ₃,Polynomial.eval_add,Polynomial.eval_mul,
    Polynomial.eval_pow,Polynomial.eval_ofNat,Polynomial.eval_X,Polynomial.eval_C]
  have h3 : (3:K)=0 := CharP.cast_eq_zero K 3
  rw [h3]
  ring

omit [DecidableEq K] in
theorem b8_nonzero_when_b2_zero (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (h2 : W.b₂=0) : W.b₈≠0 := by
  intro h8
  have hb := W.b_relation_of_char_three
  rw [h2,h8,zero_mul,zero_sub] at hb
  have h4sq : W.b₄^2=0 := by linear_combination hb
  have h4 : W.b₄=0 := by
    by_contra hn
    exact pow_ne_zero 2 hn h4sq
  apply hΔ
  rw [W.Δ_of_char_three,h2,h4]
  ring

theorem only_zero_when_b2_zero (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (h2 : W.b₂=0) (P : W.Point) (ht : (3:ℕ) • P=0) : P=0 := by
  cases P with
  | zero => rfl
  | some x y hN =>
    have hp := (three_point_iff W hN).mp ht
    change W.Ψ₃.eval x=0 at hp
    rw [psi_char_three,h2,zero_mul,zero_add] at hp
    exact False.elim (b8_nonzero_when_b2_zero W hΔ h2 hp)

theorem card_one_when_b2_zero (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (h2 : W.b₂=0) : Nat.card (ThreeTorsion W)=1 := by
  apply Nat.card_eq_one_iff_exists.mpr
  refine ⟨0,fun P => Subtype.ext (only_zero_when_b2_zero W hΔ h2 P.val P.property)⟩

theorem nonzero_three_point [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (h2 : W.b₂≠0) : ∃ P : W.Point,P≠0 ∧ (3:ℕ) • P=0 := by
  obtain ⟨x,hx⟩ := IsAlgClosed.exists_pow_nat_eq (-W.b₈/W.b₂) (by decide : 0<(3:ℕ))
  obtain ⟨y,hy⟩ := ordinate_exists W x
  have hN := (W.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp hy
  refine ⟨Point.some x y hN,Point.some_ne_zero hN,?_⟩
  apply (three_point_iff W hN).mpr
  change W.Ψ₃.eval x=0
  rw [psi_char_three,hx]
  field_simp [h2]
  ring

theorem card_three_when_b2_nonzero [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (h2 : W.b₂≠0) : Nat.card (ThreeTorsion W)=3 := by
  rcases ThreeTorsionCharThreeAudit.char_three_card_one_or_three W hΔ with hc | hc
  · obtain ⟨P,hP,ht⟩ := nonzero_three_point W hΔ h2
    obtain ⟨hs,_⟩ := Nat.card_eq_one_iff_unique.mp hc
    have hz := hs.elim (⟨P,ht⟩ : ThreeTorsion W) 0
    exact False.elim (hP (congrArg Subtype.val hz))
  · exact hc

theorem char_three_coefficient_classification [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card (ThreeTorsion W)=if W.b₂=0 then 1 else 3 := by
  split_ifs with h2
  · exact card_one_when_b2_zero W hΔ h2
  · exact card_three_when_b2_nonzero W hΔ h2

#print axioms nonzero_three_point
#print axioms char_three_coefficient_classification
end
end SmallCharacteristicJAudit
