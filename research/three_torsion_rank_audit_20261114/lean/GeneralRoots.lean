import GeneralTriplePoint
import QuarticRoots

namespace ThreeTorsionRankAudit.General
open Polynomial
noncomputable section
variable {K : Type*} [Field K]

def ordinatePolynomial (W : WeierstrassCurve.Affine K) (x : K) : K[X] :=
  W.polynomial.map (Polynomial.evalRingHom x)

theorem ordinate_formula (W : WeierstrassCurve.Affine K) (x : K) :
    ordinatePolynomial W x = X^2+C (W.a₁*x+W.a₃)*X-
      C (x^3+W.a₂*x^2+W.a₄*x+W.a₆) := by
  simp [ordinatePolynomial,WeierstrassCurve.Affine.polynomial]

theorem ordinate_degree (W : WeierstrassCurve.Affine K) (x : K) :
    (ordinatePolynomial W x).natDegree=2 := by
  rw [ordinatePolynomial,W.monic_polynomial.natDegree_map,W.natDegree_polynomial]

theorem ordinate_nonzero (W : WeierstrassCurve.Affine K) (x : K) : ordinatePolynomial W x ≠ 0 :=
  Polynomial.ne_zero_of_natDegree_gt (show 0 < (ordinatePolynomial W x).natDegree by
    rw [ordinate_degree]; norm_num)

theorem ordinate_root_iff (W : WeierstrassCurve.Affine K) (x y : K) :
    (ordinatePolynomial W x).IsRoot y ↔ W.Equation x y := by
  simp only [ordinate_formula,Polynomial.IsRoot.def,eval_sub,eval_add,eval_pow,eval_mul,eval_X,eval_C,
    WeierstrassCurve.Affine.equation_iff']
  ring_nf

theorem ordinate_derivative_eval (W : WeierstrassCurve.Affine K) (x y : K) :
    (ordinatePolynomial W x).derivative.eval y=denominator W x y := by
  rw [ordinate_formula]
  simp [Polynomial.derivative_pow,denominator]
  ring

theorem ordinate_exists [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (x : K) :
    ∃ y : K, W.Equation x y := by
  obtain ⟨y,hy⟩ := IsAlgClosed.exists_root (ordinatePolynomial W x) (by
    intro hd
    have hn := Polynomial.natDegree_eq_of_degree_eq_some hd
    rw [ordinate_degree] at hn
    norm_num at hn)
  exact ⟨y,(ordinate_root_iff W x y).mp hy⟩

theorem psi_derivative (W : WeierstrassCurve.Affine K) : W.Ψ₃.derivative=3*W.Ψ₂Sq := by
  simp [WeierstrassCurve.Ψ₃,WeierstrassCurve.Ψ₂Sq,Polynomial.derivative_pow]
  norm_num only [map_ofNat]
  ring

theorem psi_two_sq_at_point (W : WeierstrassCurve.Affine K) {x y : K} (he : W.Equation x y) :
    W.Ψ₂Sq.eval x=(denominator W x y)^2 := by
  simpa [WeierstrassCurve.Ψ₂Sq,denominator] using W.den_duplication_eq he

theorem psi_separable [IsAlgClosed K] [NeZero (3 : K)] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ ≠ 0) : W.Ψ₃.Separable := by
  rw [Polynomial.separable_def,
    Polynomial.isCoprime_iff_aeval_ne_zero_of_isAlgClosed K K W.Ψ₃ W.Ψ₃.derivative]
  intro x
  change W.Ψ₃.eval x ≠ 0 ∨ W.Ψ₃.derivative.eval x ≠ 0
  by_cases hx : W.Ψ₃.eval x=0
  · right
    obtain ⟨y,hy⟩ := ordinate_exists W x
    have hN := (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp hy
    have hd := psi_root_denominator_nonzero W hN hx
    have hs : W.Ψ₂Sq.eval x ≠ 0 := by rw [psi_two_sq_at_point W hy]; exact pow_ne_zero _ hd
    rw [psi_derivative]
    simpa using mul_ne_zero (NeZero.ne (3 : K)) hs
  · exact Or.inl hx

theorem ordinate_separable [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ ≠ 0) {x : K} (hx : W.Ψ₃.IsRoot x) : (ordinatePolynomial W x).Separable := by
  rw [Polynomial.separable_def,
    Polynomial.isCoprime_iff_aeval_ne_zero_of_isAlgClosed K K
      (ordinatePolynomial W x) (ordinatePolynomial W x).derivative]
  intro y
  change (ordinatePolynomial W x).eval y ≠ 0 ∨ (ordinatePolynomial W x).derivative.eval y ≠ 0
  by_cases hy : (ordinatePolynomial W x).eval y=0
  · right
    have he := (ordinate_root_iff W x y).mp hy
    have hN := (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp he
    rw [ordinate_derivative_eval]
    exact psi_root_denominator_nonzero W hN hx
  · exact Or.inl hy

variable [DecidableEq K]
abbrev Root (W : WeierstrassCurve.Affine K) := {x : K // W.Ψ₃.IsRoot x}
abbrev Ordinate (W : WeierstrassCurve.Affine K) (x : K) := {y : K // W.Equation x y}

instance rootFintype [NeZero (3 : K)] (W : WeierstrassCurve.Affine K) : Fintype (Root W) :=
  polynomialRootFintype W.Ψ₃ (W.Ψ₃_ne_zero (NeZero.ne _))

def ordinateRootEquiv (W : WeierstrassCurve.Affine K) (x : K) :
    Ordinate W x ≃ {y : K // (ordinatePolynomial W x).IsRoot y} where
  toFun y := ⟨y.val,(ordinate_root_iff W x y.val).mpr y.property⟩
  invFun y := ⟨y.val,(ordinate_root_iff W x y.val).mp y.property⟩
  left_inv y := by rfl
  right_inv y := by rfl

instance ordinateFintype (W : WeierstrassCurve.Affine K) (x : K) : Fintype (Ordinate W x) := by
  let := polynomialRootFintype (ordinatePolynomial W x) (ordinate_nonzero W x)
  exact Fintype.ofEquiv {y : K // (ordinatePolynomial W x).IsRoot y} (ordinateRootEquiv W x).symm

theorem root_card_four [IsAlgClosed K] [NeZero (3 : K)] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ ≠ 0) : Nat.card (Root W)=4 := by
  rw [polynomialRoot_card W.Ψ₃ (W.Ψ₃_ne_zero (NeZero.ne _)),
    Multiset.toFinset_card_of_nodup (Polynomial.nodup_roots (psi_separable W hΔ)),
    ← (IsAlgClosed.splits W.Ψ₃).natDegree_eq_card_roots,W.natDegree_Ψ₃ (NeZero.ne _)]

theorem ordinate_card_two [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ ≠ 0) {x : K} (hx : W.Ψ₃.IsRoot x) : Nat.card (Ordinate W x)=2 := by
  rw [Nat.card_congr (ordinateRootEquiv W x),
    polynomialRoot_card (ordinatePolynomial W x) (ordinate_nonzero W x),
    Multiset.toFinset_card_of_nodup (Polynomial.nodup_roots (ordinate_separable W hΔ hx)),
    ← (IsAlgClosed.splits (ordinatePolynomial W x)).natDegree_eq_card_roots,ordinate_degree]

abbrev Fibers (W : WeierstrassCurve.Affine K) := Σ x : Root W, Ordinate W x.val

theorem fibers_card_eight [IsAlgClosed K] [NeZero (3 : K)] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ ≠ 0) : Nat.card (Fibers W)=8 := by
  rw [Nat.card_eq_fintype_card,Fintype.card_sigma]
  have hc (x : Root W) : Fintype.card (Ordinate W x.val)=2 := by
    rw [← Nat.card_eq_fintype_card]
    exact ordinate_card_two W hΔ x.property
  simp_rw [hc]
  simp only [Finset.sum_const,Finset.card_univ,smul_eq_mul]
  rw [← Nat.card_eq_fintype_card,root_card_four W hΔ]

end
end ThreeTorsionRankAudit.General
