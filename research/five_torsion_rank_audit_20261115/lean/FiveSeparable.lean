import FiveRootData

namespace FiveTorsionRankAudit
noncomputable section
open Polynomial ThreeTorsionRankAudit ThreeTorsionRankAudit.General
variable {K : Type*} [Field K]

theorem five_degree [NeZero (5:K)] (W : WeierstrassCurve K) : (psiFive W).natDegree=12 := by
  have h := W.natDegree_preΨ' (n:=5) (NeZero.ne (5:K))
  norm_num at h
  exact h

theorem five_nonzero [NeZero (5:K)] (W : WeierstrassCurve K) : psiFive W≠0 :=
  ne_zero_of_natDegree_gt (show 0<(psiFive W).natDegree by rw [five_degree]; decide)

theorem five_separable [IsAlgClosed K] [NeZero (5:K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : (psiFive W).Separable := by
  rw [Polynomial.separable_def,
    Polynomial.isCoprime_iff_aeval_ne_zero_of_isAlgClosed K K (psiFive W) (psiFive W).derivative]
  intro x
  change (psiFive W).eval x≠0 ∨ (psiFive W).derivative.eval x≠0
  by_cases hx : (psiFive W).eval x=0
  · right
    obtain ⟨y,hy⟩ := ordinate_exists W x
    have hN := (W.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp hy
    obtain ⟨hd,ht⟩ := five_root_data W hN hx
    intro hz
    have he := five_derivative_eval W x
    rw [hz] at he
    have hv : (central W).eval x*W.preΨ₄.eval x-(W.Ψ₃.eval x)^2=0 :=
      (mul_eq_zero.mp he.symm).resolve_left (mul_ne_zero (NeZero.ne (5:K)) hd)
    have hi := five_root_identity_eval W x
    rw [hv,hx,mul_zero,mul_zero,sub_zero] at hi
    have hh : W.preΨ₄.eval x=0 := (mul_eq_zero.mp hi).resolve_left (pow_ne_zero 2 ht)
    have hf := five_eval W x
    rw [hx,hh,mul_zero] at hf
    have hb : (W.Ψ₃.eval x)^3=0 := by linear_combination hf
    exact pow_ne_zero 3 ht hb
  · exact Or.inl hx

theorem five_ordinate_separable [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) {x : K} (hx : (psiFive W).IsRoot x) : (ordinatePolynomial W x).Separable := by
  rw [Polynomial.separable_def,
    Polynomial.isCoprime_iff_aeval_ne_zero_of_isAlgClosed K K
      (ordinatePolynomial W x) (ordinatePolynomial W x).derivative]
  intro y
  change (ordinatePolynomial W x).eval y≠0 ∨ (ordinatePolynomial W x).derivative.eval y≠0
  by_cases hy : (ordinatePolynomial W x).eval y=0
  · right
    have he := (ordinate_root_iff W x y).mp hy
    have hN := (W.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp he
    rw [ordinate_derivative_eval]
    exact five_root_denominator W hN hx
  · exact Or.inl hy

variable [DecidableEq K]
abbrev FiveRoot (W : WeierstrassCurve K) := {x : K // (psiFive W).IsRoot x}
instance fiveRootFintype [NeZero (5:K)] (W : WeierstrassCurve K) : Fintype (FiveRoot W) :=
  polynomialRootFintype (psiFive W) (five_nonzero W)

theorem five_root_card [IsAlgClosed K] [NeZero (5:K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : Nat.card (FiveRoot W)=12 := by
  rw [polynomialRoot_card (psiFive W) (five_nonzero W),
    Multiset.toFinset_card_of_nodup (Polynomial.nodup_roots (five_separable W hΔ)),
    ←(IsAlgClosed.splits (psiFive W)).natDegree_eq_card_roots,five_degree]

theorem five_ordinate_card [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) {x : K} (hx : (psiFive W).IsRoot x) : Nat.card (Ordinate W x)=2 := by
  rw [Nat.card_congr (ordinateRootEquiv W x),
    polynomialRoot_card (ordinatePolynomial W x) (ordinate_nonzero W x),
    Multiset.toFinset_card_of_nodup (Polynomial.nodup_roots (five_ordinate_separable W hΔ hx)),
    ←(IsAlgClosed.splits (ordinatePolynomial W x)).natDegree_eq_card_roots,ordinate_degree]

#print axioms five_separable
#print axioms five_root_card
end
end FiveTorsionRankAudit
