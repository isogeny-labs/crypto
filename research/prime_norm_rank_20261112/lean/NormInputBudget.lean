module
public import Mathlib.RingTheory.MvPolynomial.WeightedHomogeneous
public import Mathlib.Algebra.Polynomial.Div
public import Mathlib.Algebra.Polynomial.Coeff
public import Mathlib.Tactic
public section

namespace NormInputBudget
open Polynomial Finset
variable {K σ : Type*} [CommRing K]

/-- Literal support budget, counting the univariate X with weight k. -/
def Budget (w : σ → ℕ) (k : ℕ) (P : (MvPolynomial σ K)[X]) (D : ℕ) : Prop :=
  ∀ j, ∀ e ∈ (P.coeff j).support, Finsupp.weight w e + k*j ≤ D

variable {w : σ → ℕ} {k D E : ℕ} {P Q : (MvPolynomial σ K)[X]}

theorem mono (h : Budget w k P D) (hDE : D ≤ E) : Budget w k P E :=
  fun j e he => (h j e he).trans hDE

theorem zero (w : σ → ℕ) (k D : ℕ) : Budget w k (0 : (MvPolynomial σ K)[X]) D := by
  simp [Budget]

theorem add (hP : Budget w k P D) (hQ : Budget w k Q D) : Budget w k (P+Q) D := by
  classical
  intro j e he
  rw [coeff_add] at he
  rcases mem_union.mp (MvPolynomial.support_add he) with hp | hq
  · exact hP j e hp
  · exact hQ j e hq

theorem neg (hP : Budget w k P D) : Budget w k (-P) D := by
  intro j e he
  rw [coeff_neg] at he
  have hs : e ∈ (P.coeff j).support := by
    simpa only [MvPolynomial.mem_support_iff, MvPolynomial.coeff_neg, neg_ne_zero] using he
  exact hP j e hs

theorem sub (hP : Budget w k P D) (hQ : Budget w k Q D) : Budget w k (P-Q) D := by
  rw [sub_eq_add_neg]
  exact add hP (neg hQ)

theorem sum {I : Type*} (s : Finset I) (f : I → (MvPolynomial σ K)[X])
    (h : ∀ i∈s, Budget w k (f i) D) : Budget w k (∑ i∈s, f i) D := by
  classical
  intro j e he
  rw [finsetSum_coeff] at he
  obtain ⟨i,hi,he⟩ := mem_biUnion.mp (MvPolynomial.support_sum he)
  exact h i hi j e he

theorem mul (hP : Budget w k P D) (hQ : Budget w k Q E) : Budget w k (P*Q) (D+E) := by
  classical
  intro j e he
  rw [coeff_mul] at he
  obtain ⟨ij,hij,he⟩ := mem_biUnion.mp (MvPolynomial.support_sum he)
  obtain ⟨ep,hep,eq,heq,rfl⟩ := Finset.mem_add.mp (MvPolynomial.support_mul _ _ he)
  have hp := hP ij.1 ep hep
  have hq := hQ ij.2 eq heq
  have hij' := Finset.mem_antidiagonal.mp hij
  rw [map_add]
  nlinarith

/-- A monomial budget is stated on actual coefficient support, including zero. -/
theorem monomial (c : MvPolynomial σ K) (j D : ℕ)
    (h : ∀ e∈c.support, Finsupp.weight w e+k*j≤D) :
    Budget w k (Polynomial.monomial j c) D := by
  classical
  intro i e he
  by_cases hi : j=i
  · subst i
    simpa using h e (by simpa using he)
  · simp [coeff_monomial,hi] at he

theorem C (c : MvPolynomial σ K) (D : ℕ)
    (h : ∀ e∈c.support, Finsupp.weight w e≤D) : Budget w k (Polynomial.C c) D := by
  simpa only [monomial_zero_left, mul_zero, add_zero] using
    (monomial (w:=w) (k:=k) c 0 D (by simpa using h))

/-- Public constants have weight zero in the actual multivariate ring. -/
theorem scalar (w : σ → ℕ) (k : ℕ) (c : K) :
    Budget w k (Polynomial.C (MvPolynomial.C c)) 0 := by
  classical
  apply C
  intro e he
  have hz : e=0 := mem_singleton.mp (MvPolynomial.support_monomial_subset he)
  subst e
  simp

theorem indeterminate (w : σ → ℕ) (k : ℕ) (i : σ) (j D : ℕ) (h : w i+k*j≤D) :
    Budget w k (Polynomial.monomial j (MvPolynomial.X i : MvPolynomial σ K)) D := by
  classical
  apply monomial
  intro e he
  have hz : e=Finsupp.single i 1 := mem_singleton.mp (MvPolynomial.support_monomial_subset he)
  subst e
  simpa [Finsupp.weight_single] using h

theorem pow (hP : Budget w k P D) (n : ℕ) : Budget w k (P^n) (D*n) := by
  induction n with
  | zero => simpa using scalar w k (1:K)
  | succ n ih => simpa [pow_succ,Nat.mul_succ] using mul ih hP

theorem publicPolynomial (w : σ → ℕ) (k : ℕ) (p : K[X]) :
    Budget w k (p.map MvPolynomial.C) (k*p.natDegree) := by
  classical
  intro j e he
  rw [coeff_map] at he
  have hz : e=0 := mem_singleton.mp (MvPolynomial.support_monomial_subset he)
  subst e
  have hj : p.coeff j≠0 := by
    intro hj
    simp [hj] at he
  simpa using Nat.mul_le_mul_left k (le_natDegree_of_ne_zero hj)

/-- Evaluation at a public scalar does not increase coefficient weights. -/
theorem evalScalar (hP : Budget w k P D) (r : K) :
    Budget w k (Polynomial.C (P.eval (MvPolynomial.C r))) D := by
  classical
  have heq : Polynomial.C (P.eval (MvPolynomial.C r)) =
      ∑ j∈P.support, Polynomial.C (P.coeff j) *
        Polynomial.C (MvPolynomial.C (r^j)) := by
    simp only [Polynomial.eval_eq_sum,Polynomial.sum_def,map_sum,map_mul,map_pow]
  rw [heq]
  apply sum
  intro j hj
  have hc : Budget w k (Polynomial.C (P.coeff j)) D := by
    apply C
    intro e he
    have h := hP j e he
    omega
  simpa using mul hc (scalar w k (r^j))

/-- The budget applies literally to every supported leading coefficient. -/
theorem leading (hP : Budget w k P D) :
    ∀ e∈P.leadingCoeff.support, Finsupp.weight w e+k*P.natDegree≤D := by
  simpa only [Polynomial.leadingCoeff] using hP P.natDegree

theorem leading_degree (hP : Budget w k P D) (hp : P≠0) : k*P.natDegree≤D := by
  have hl : P.leadingCoeff≠0 := Polynomial.leadingCoeff_ne_zero.mpr hp
  obtain ⟨e,he⟩ := MvPolynomial.ne_zero_iff.mp hl
  have hb := leading hP e (MvPolynomial.mem_support_iff.mpr he)
  omega

/-- Full computed monic division preserves the weighted coefficient budget.
Both quotient and remainder are the literal Mathlib operations. -/
theorem division (P : (MvPolynomial σ K)[X]) {Q : (MvPolynomial σ K)[X]}
    (hQ : Q.Monic) (hQB : Budget w k Q (k*Q.natDegree))
    (D : ℕ) (hP : Budget w k P D) :
    Budget w k (P %ₘ Q) D ∧ Budget w k (P /ₘ Q) (D-k*Q.natDegree) := by
  classical
  by_cases h : Q.degree≤P.degree ∧ P≠0
  · let z := Polynomial.C P.leadingCoeff * X^(P.natDegree-Q.natDegree)
    have hQP : Q.natDegree≤P.natDegree :=
      natDegree_le_natDegree h.1
    have hD := leading_degree hP h.2
    have hQD : k*Q.natDegree≤D := (Nat.mul_le_mul_left k hQP).trans hD
    have hz : Budget w k z (D-k*Q.natDegree) := by
      rw [show z=Polynomial.monomial (P.natDegree-Q.natDegree) P.leadingCoeff by
        simp [z,C_mul_X_pow_eq_monomial]]
      apply monomial
      intro e he
      have hl := leading hP e he
      have hmul := Nat.mul_le_mul_left k hQP
      rw [Nat.mul_sub_left_distrib]
      omega
    have hs : Budget w k (P-Q*z) D := by
      apply sub hP
      apply mono (mul hQB hz)
      omega
    have hwf := div_wf_lemma h hQ
    have ih := division (P-Q*z) hQ hQB D hs
    have hr : P %ₘ Q=(P-Q*z) %ₘ Q := by
      unfold modByMonic
      rw [dite_eq_left hQ, dite_eq_left hQ, divModByMonicAux]
      simp only [dite_eq_left h]
      rfl
    have hd : P /ₘ Q=z+(P-Q*z) /ₘ Q := by
      unfold divByMonic
      rw [dite_eq_left hQ, dite_eq_left hQ, divModByMonicAux]
      simp only [dite_eq_left h]
      rfl
    rw [hr,hd]
    exact ⟨ih.1,add hz ih.2⟩
  · have hr : P %ₘ Q=P := by
      unfold modByMonic
      rw [dite_eq_left hQ,divModByMonicAux]
      simp only [dite_eq_right h]
    have hd : P /ₘ Q=0 := by
      unfold divByMonic
      rw [dite_eq_left hQ,divModByMonicAux]
      simp only [dite_eq_right h]
    rw [hr,hd]
    exact ⟨hP,zero _ _ _⟩
termination_by P

end NormInputBudget
