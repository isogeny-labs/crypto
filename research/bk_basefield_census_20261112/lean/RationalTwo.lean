import TorsionTransport

namespace BKBasefieldScalingAudit
noncomputable section
open Polynomial ThreeTorsionRankAudit
variable {K : Type*} [Field K] [DecidableEq K]

def cubicPolynomial (a b : K) : K[X] := X^3+C a*X+C b
abbrev twoRoots (a b : K) := (cubicPolynomial a b).rootSet K

theorem cubic_polynomial_ne_zero (a b : K) : cubicPolynomial a b≠0 := by
  intro h
  have hd : (cubicPolynomial a b).natDegree=3 := by unfold cubicPolynomial;compute_degree!
  rw [h,natDegree_zero] at hd
  omega

theorem affine_two_torsion_iff (a b : K) (h2 : (2:K)≠0)
    {x y : K} (h : (shortCurve a b).Nonsingular x y) :
    (2:ℕ) • WeierstrassCurve.Affine.Point.some x y h=0 ↔ y=0 := by
  rw [two_nsmul]
  constructor
  · intro hh
    by_cases hy : y= -y
    · have hz : 2*y=0 := by linear_combination hy
      exact (mul_eq_zero.mp hz).resolve_left h2
    · have hn : y≠(shortCurve a b).negY x y := by
        simpa only [WeierstrassCurve.Affine.negY,shortCurve,zero_mul,sub_zero] using hy
      rw [WeierstrassCurve.Affine.Point.add_self_of_Y_ne hn] at hh
      cases hh
  · intro hy
    subst y
    apply WeierstrassCurve.Affine.Point.add_self_of_Y_eq
    simp [WeierstrassCurve.Affine.negY,shortCurve]

theorem cubic_derivative_nonzero_at_root (a b : K) (hd : 4*a^3+27*b^2≠0)
    (x : K) (hx : cubicValue a b x=0) : 3*x^2+a≠0 := by
  intro hh
  have hq : (quartic a b).eval x=0 := by rw [quartic_identity,hx,hh];ring
  exact quartic_cubic_no_common_root a b hd x hq hx

theorem root_two_nonsingular (a b : K) (hd : 4*a^3+27*b^2≠0) (x : twoRoots a b) :
    (shortCurve a b).Nonsingular x 0 := by
  have hx : cubicValue a b x=0 := by
    simpa [cubicPolynomial,cubicValue] using Polynomial.aeval_eq_zero_of_mem_rootSet x.property
  rw [short_nonsingular_iff]
  exact ⟨by simpa [cubicValue] using hx.symm,
    Or.inl (cubic_derivative_nonzero_at_root a b hd x hx)⟩

def twoCoordinates (a b : K) (h2 : (2:K)≠0) :
    killedPoints a b 2 → Option (twoRoots a b)
  | ⟨.zero,_⟩ => none
  | ⟨.some x y h,ht⟩ => some ⟨x,by
      apply (mem_rootSet_of_ne (cubic_polynomial_ne_zero a b)).mpr
      have hy := (affine_two_torsion_iff a b h2 h).mp ht
      have he := (short_nonsingular_iff a b x y).mp h
      rw [hy] at he
      simpa [cubicPolynomial] using he.1.symm⟩

def fromTwoCoordinates (a b : K) (hd : 4*a^3+27*b^2≠0) (h2 : (2:K)≠0) :
    Option (twoRoots a b) → killedPoints a b 2
  | none => ⟨.zero,by exact nsmul_zero 2⟩
  | some x => ⟨.some x 0 (root_two_nonsingular a b hd x),
      (affine_two_torsion_iff a b h2 (root_two_nonsingular a b hd x)).mpr rfl⟩

def twoCoordinateEquiv (a b : K) (hd : 4*a^3+27*b^2≠0) (h2 : (2:K)≠0) :
    killedPoints a b 2 ≃ Option (twoRoots a b) where
  toFun:=twoCoordinates a b h2
  invFun:=fromTwoCoordinates a b hd h2
  left_inv P:=by
    rcases P with ⟨P,ht⟩
    cases P with
    | zero => rfl
    | some x y h =>
      have hy := (affine_two_torsion_iff a b h2 h).mp ht
      subst y
      rfl
  right_inv P:=by cases P <;> rfl

theorem rational_two_torsion_root_count (a b : K)
    (hd : 4*a^3+27*b^2≠0) (h2 : (2:K)≠0) :
    Nat.card (killedPoints a b 2)=Fintype.card (twoRoots a b)+1 := by
  classical
  rw [Nat.card_congr (twoCoordinateEquiv a b hd h2),Nat.card_eq_fintype_card,
    Fintype.card_option]

theorem geometric_two_torsion_cardinality [IsAlgClosed K] (a b : K)
    (hd : 4*a^3+27*b^2≠0) (h2 : (2:K)≠0) :
    Nat.card (killedPoints a b 2)=4 := by
  have hsep : (cubicPolynomial a b).Separable := by
    rw [separable_def]
    apply (Polynomial.isCoprime_iff_aeval_ne_zero_of_isAlgClosed K K
      (cubicPolynomial a b) (cubicPolynomial a b).derivative).mpr
    intro x
    by_cases hx : cubicValue a b x=0
    · right
      simpa [cubicPolynomial,cubicValue,derivative_add,derivative_mul,derivative_pow]
        using cubic_derivative_nonzero_at_root a b hd x hx
    · left
      simpa [cubicPolynomial,cubicValue] using hx
  have hc : Fintype.card (twoRoots a b)=3 := by
    rw [card_rootSet_eq_natDegree hsep (IsAlgClosed.splits _)]
    unfold cubicPolynomial
    compute_degree!
  rw [rational_two_torsion_root_count a b hd h2,hc]

#print axioms rational_two_torsion_root_count
#print axioms geometric_two_torsion_cardinality
end
end BKBasefieldScalingAudit
