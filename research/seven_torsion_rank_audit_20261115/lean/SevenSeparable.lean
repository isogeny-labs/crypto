import SevenRootData

namespace SevenTorsionRankAudit
noncomputable section
open Polynomial ThreeTorsionRankAudit ThreeTorsionRankAudit.General
variable {K : Type*} [Field K]

theorem seven_degree [NeZero (7:K)] (W : WeierstrassCurve K) : (psiSeven W).natDegree=24 := by
  have h := W.natDegree_preΨ' (n:=7) (NeZero.ne (7:K))
  norm_num at h
  exact h

theorem seven_nonzero [NeZero (7:K)] (W : WeierstrassCurve K) : psiSeven W≠0 :=
  ne_zero_of_natDegree_gt (show 0<(psiSeven W).natDegree by rw [seven_degree]; decide)

theorem seven_separable [IsAlgClosed K] [NeZero (7:K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : (psiSeven W).Separable := by
  rw [Polynomial.separable_def,
    Polynomial.isCoprime_iff_aeval_ne_zero_of_isAlgClosed K K (psiSeven W) (psiSeven W).derivative]
  intro x
  change (psiSeven W).eval x≠0 ∨ (psiSeven W).derivative.eval x≠0
  by_cases hx : (psiSeven W).eval x=0
  · right
    obtain ⟨y,hy⟩ := ordinate_exists W x
    have hN := (W.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp hy
    obtain ⟨hd,_,hh⟩ := seven_root_data W hN hx
    intro hz
    have he := seven_derivative_eval W x
    rw [hz] at he
    have hj : (sevenAux W).eval x=0 :=
      (mul_eq_zero.mp he.symm).resolve_left (mul_ne_zero (NeZero.ne (7:K)) hd)
    have hi := seven_root_derivative_eval W x
    rw [hj,hx,mul_zero,mul_zero,sub_zero] at hi
    have hu : (FiveTorsionRankAudit.psiFive W).eval x=(W.preΨ₄.eval x)^2 :=
      sub_eq_zero.mp ((mul_eq_zero.mp hi.symm).resolve_left (pow_ne_zero 2 hh))
    have hc := seven_collapse_eval W x
    rw [hx,hu] at hc
    have hp : (W.preΨ₄.eval x)^4=0 := by linear_combination hc
    exact pow_ne_zero 4 hh hp
  · exact Or.inl hx

theorem seven_ordinate_separable [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) {x : K} (hx : (psiSeven W).IsRoot x) : (ordinatePolynomial W x).Separable := by
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
    exact seven_root_denominator W hN hx
  · exact Or.inl hy

variable [DecidableEq K]
abbrev SevenRoot (W : WeierstrassCurve K) := {x : K // (psiSeven W).IsRoot x}
instance sevenRootFintype [NeZero (7:K)] (W : WeierstrassCurve K) : Fintype (SevenRoot W) :=
  polynomialRootFintype (psiSeven W) (seven_nonzero W)

theorem seven_root_card [IsAlgClosed K] [NeZero (7:K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : Nat.card (SevenRoot W)=24 := by
  rw [polynomialRoot_card (psiSeven W) (seven_nonzero W),
    Multiset.toFinset_card_of_nodup (Polynomial.nodup_roots (seven_separable W hΔ)),
    ←(IsAlgClosed.splits (psiSeven W)).natDegree_eq_card_roots,seven_degree]

theorem seven_ordinate_card [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) {x : K} (hx : (psiSeven W).IsRoot x) : Nat.card (Ordinate W x)=2 := by
  rw [Nat.card_congr (ordinateRootEquiv W x),
    polynomialRoot_card (ordinatePolynomial W x) (ordinate_nonzero W x),
    Multiset.toFinset_card_of_nodup (Polynomial.nodup_roots (seven_ordinate_separable W hΔ hx)),
    ←(IsAlgClosed.splits (ordinatePolynomial W x)).natDegree_eq_card_roots,ordinate_degree]

#print axioms seven_separable
#print axioms seven_root_card
end
end SevenTorsionRankAudit
