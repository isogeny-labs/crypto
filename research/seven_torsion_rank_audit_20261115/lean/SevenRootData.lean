import SevenPolynomial

namespace SevenTorsionRankAudit
noncomputable section
open Polynomial FiveTorsionRankAudit ThreeTorsionRankAudit.General
variable {K : Type*} [Field K]

theorem seven_root_data (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (hx : (psiSeven W).IsRoot x) :
    W.Ψ₂Sq.eval x≠0 ∧ W.Ψ₃.eval x≠0 ∧ W.preΨ₄.eval x≠0 := by
  have hz : (psiSeven W).eval x=0 := hx
  have hd : W.Ψ₂Sq.eval x≠0 := by
    intro he
    have hu := five_eval W x
    rw [he,zero_pow (by decide : (2:ℕ)≠0),zero_mul,zero_sub] at hu
    have hv := seven_eval W x
    rw [hz,he,hu] at hv
    have hp : (W.Ψ₃.eval x)^6=0 := by linear_combination hv
    have ht : W.Ψ₃.eval x=0 := eq_zero_of_pow_eq_zero hp
    have hn := psi_root_denominator_nonzero W hN ht
    have hs : (denominator W x y)^2=0 := by rw [←psi_two_sq_at_point W hN.1,he]
    exact hn (sq_eq_zero_iff.mp hs)
  have ht : W.Ψ₃.eval x≠0 := by
    intro he
    have hh := fourth_eval W x
    rw [he,zero_mul,zero_sub] at hh
    have hv := seven_eval W x
    rw [hz,he,hh] at hv
    have hp : (W.Ψ₂Sq.eval x)^8=0 := by linear_combination -hv
    exact pow_ne_zero 8 hd hp
  refine ⟨hd,ht,?_⟩
  intro he
  have hu := five_eval W x
  rw [he,mul_zero,zero_sub] at hu
  have hv := seven_eval W x
  rw [hz,he,hu] at hv
  have hp : (W.Ψ₃.eval x)^6=0 := by linear_combination hv
  exact pow_ne_zero 6 ht hp

theorem seven_root_denominator (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (hx : (psiSeven W).IsRoot x) : denominator W x y≠0 := by
  have hd := (seven_root_data W hN hx).1
  rw [psi_two_sq_at_point W hN.1] at hd
  exact fun h => hd (by rw [h,zero_pow (by decide : (2:ℕ)≠0)])

#print axioms seven_root_data
end
end SevenTorsionRankAudit
