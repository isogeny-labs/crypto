import DivisionQuartic
import Mathlib.SetTheory.Cardinal.NatCard

namespace ThreeTorsionRankAudit
open Polynomial
noncomputable section
variable {K : Type*} [Field K] [DecidableEq K]

abbrev Root (a b : K) := {x : K // (psi a b).IsRoot x}

abbrev polynomialRootFintype (p : K[X]) (hp : p ≠ 0) : Fintype {x : K // p.IsRoot x} :=
  Fintype.ofFinset p.roots.toFinset (by
    intro x
    rw [Multiset.mem_toFinset]
    exact Polynomial.mem_roots hp)

theorem polynomialRoot_card (p : K[X]) (hp : p ≠ 0) :
    Nat.card {x : K // p.IsRoot x} = p.roots.toFinset.card := by
  let := polynomialRootFintype p hp
  rw [Nat.card_eq_fintype_card]
  exact Fintype.card_ofFinset _ _

omit [DecidableEq K] in
theorem psi_nonzero [NeZero (3 : K)] (a b : K) : psi a b ≠ 0 := by
  exact Polynomial.ne_zero_of_natDegree_gt (show 0 < (psi a b).natDegree by rw [psi_degree]; norm_num)

instance rootFintype [NeZero (3 : K)] (a b : K) : Fintype (Root a b) :=
  polynomialRootFintype (psi a b) (psi_nonzero a b)

theorem root_card_four [IsAlgClosed K] [NeZero (2 : K)] [NeZero (3 : K)]
    (a b : K) (hΔ : (short a b).Δ ≠ 0) : Nat.card (Root a b) = 4 := by
  rw [polynomialRoot_card (psi a b) (psi_nonzero a b),
    Multiset.toFinset_card_of_nodup (Polynomial.nodup_roots (psi_separable a b hΔ))]
  rw [← (IsAlgClosed.splits (psi a b)).natDegree_eq_card_roots,psi_degree]

def squareRootEquiv (c : K) : {y : K // y^2=c} ≃
    {y : K // ((X : K[X])^2-C c).IsRoot y} where
  toFun y := ⟨y.val,by simpa [Polynomial.IsRoot.def] using sub_eq_zero.mpr y.property⟩
  invFun y := ⟨y.val,sub_eq_zero.mp (by simpa [Polynomial.IsRoot.def] using y.property)⟩
  left_inv := by intro y; rfl
  right_inv := by intro y; rfl

instance squareFintype (c : K) : Fintype {y : K // y^2=c} := by
  let := polynomialRootFintype ((X : K[X])^2-C c) (Polynomial.X_pow_sub_C_ne_zero (by decide) c)
  exact Fintype.ofEquiv {y : K // ((X : K[X])^2-C c).IsRoot y} (squareRootEquiv c).symm

theorem square_card_two [IsAlgClosed K] [NeZero (2 : K)] (c : K) (hc : c ≠ 0) :
    Nat.card {y : K // y^2=c} = 2 := by
  rw [Nat.card_congr (squareRootEquiv c)]
  let p : K[X] := X^2-C c
  rw [polynomialRoot_card p (Polynomial.X_pow_sub_C_ne_zero (by decide) c),
    Multiset.toFinset_card_of_nodup
    (Polynomial.nodup_roots (Polynomial.separable_X_pow_sub_C c (NeZero.ne (2 : K)) hc))]
  rw [← (IsAlgClosed.splits p).natDegree_eq_card_roots]
  exact Polynomial.natDegree_X_pow_sub_C

abbrev Fibers (a b : K) := Σ x : Root a b, {y : K // y^2=curveValue a b x.val}

theorem fibers_card_eight [IsAlgClosed K] [NeZero (2 : K)] [NeZero (3 : K)]
    (a b : K) (hΔ : (short a b).Δ ≠ 0) : Nat.card (Fibers a b) = 8 := by
  rw [Nat.card_eq_fintype_card]
  change Fintype.card (Σ x : Root a b, {y : K // y^2=curveValue a b x.val}) = 8
  rw [Fintype.card_sigma]
  have hf (x : Root a b) : Fintype.card {y : K // y^2=curveValue a b x.val} = 2 := by
    rw [← Nat.card_eq_fintype_card]
    exact square_card_two _ (psi_root_curve_value_nonzero a b hΔ x.property)
  simp_rw [hf]
  simp only [Finset.sum_const,Finset.card_univ,smul_eq_mul]
  rw [← Nat.card_eq_fintype_card,root_card_four a b hΔ]

end
end ThreeTorsionRankAudit
