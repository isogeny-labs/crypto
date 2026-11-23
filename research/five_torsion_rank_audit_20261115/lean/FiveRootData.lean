import FivePolynomial

namespace FiveTorsionRankAudit
noncomputable section
open Polynomial ThreeTorsionRankAudit.General
variable {K : Type*} [Field K]

/-- Nonsingularity alone excludes both doubling and three-torsion factors. -/
theorem five_root_data (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (hx : (psiFive W).IsRoot x) :
    W.Ψ₂Sq.eval x≠0 ∧ W.Ψ₃.eval x≠0 := by
  have hz : (psiFive W).eval x=0 := hx
  have hf := five_eval W x
  have hd : W.Ψ₂Sq.eval x≠0 := by
    intro he
    have ht : W.Ψ₃.eval x=0 := by
      apply eq_zero_of_pow_eq_zero (n:=3)
      rw [hz,he] at hf
      linear_combination hf
    have hn := psi_root_denominator_nonzero W hN ht
    have hs : (denominator W x y)^2=0 := by
      rw [←psi_two_sq_at_point W hN.1,he]
    exact hn (sq_eq_zero_iff.mp hs)
  refine ⟨hd,?_⟩
  intro ht
  have hh := fourth_eval W x
  rw [ht,zero_mul,zero_sub] at hh
  rw [hz,ht,hh] at hf
  have hp : (W.Ψ₂Sq.eval x)^4=0 := by linear_combination hf
  exact pow_ne_zero 4 hd hp

theorem five_root_denominator (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (hx : (psiFive W).IsRoot x) : denominator W x y≠0 := by
  have hd := (five_root_data W hN hx).1
  rw [psi_two_sq_at_point W hN.1] at hd
  exact fun h => hd (by rw [h,zero_pow (by decide : (2:ℕ)≠0)])

#print axioms five_root_data
end
end FiveTorsionRankAudit
