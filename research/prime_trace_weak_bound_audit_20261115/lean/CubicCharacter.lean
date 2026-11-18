module
public import Mathlib.FieldTheory.Finite.Basic
public import Mathlib.Algebra.Polynomial.Derivative
public import Mathlib.Tactic

@[expose] public section
namespace PrimeTraceWeakBoundAudit
noncomputable section
open Polynomial

variable {K : Type*} [Field K] [Fintype K]

theorem field_vanishing_divides (P : K[X]) (hP : ∀ x : K, P.eval x = 0) :
    (X ^ Fintype.card K - X : K[X]) ∣ P := by
  classical
  by_cases h0 : P = 0
  · simp only [h0, dvd_zero]
  let L : K[X] := X ^ Fintype.card K - X
  have hLm : L.Monic := by
    apply (monic_X_pow _).sub_of_left
    rw [degree_X, degree_X_pow]
    exact_mod_cast Fintype.one_lt_card
  have hLr : Multiset.card L.roots = L.natDegree := by
    rw [FiniteField.roots_X_pow_card_sub_X,
      FiniteField.X_pow_card_sub_X_natDegree_eq K Fintype.one_lt_card]
    rfl
  have hf := prod_multiset_X_sub_C_of_monic_of_roots_card_eq hLm hLr
  change L ∣ P
  rw [← hf]
  apply (Multiset.prod_X_sub_C_dvd_iff_le_roots h0 _).mpr
  rw [FiniteField.roots_X_pow_card_sub_X]
  apply (Multiset.le_iff_subset (Finset.univ : Finset K).nodup).mpr
  intro x _
  exact (mem_roots h0).mpr (hP x)

theorem cubic_euler_nonconstant (hq : 3 < Fintype.card K) (hF : ringChar K ≠ 2)
    (f : K[X]) (hf : f.natDegree = 3) (ε : K) :
    ¬ ∀ x : K, (f.eval x) ^ ((Fintype.card K - 1) / 2) = ε := by
  intro hc
  let q := Fintype.card K
  let n := (q - 1) / 2
  have hqodd := FiniteField.odd_card_of_char_ne_two hF
  have hqn : q = 2 * n + 1 := by
    dsimp [q, n]
    omega
  have hn : 0 < n := by omega
  have hf0 : f ≠ 0 := by
    intro hz
    rw [hz, natDegree_zero] at hf
    omega
  let L : K[X] := X ^ q - X
  let P : K[X] := f ^ n - C ε
  have hL0 : L ≠ 0 := FiniteField.X_pow_card_sub_X_ne_zero K (by omega)
  have hLd : L.natDegree = q := FiniteField.X_pow_card_sub_X_natDegree_eq K (by omega)
  have hPd : P.natDegree = 3 * n := by
    dsimp [P]
    rw [natDegree_sub_C, natDegree_pow, hf, Nat.mul_comm]
  have hP0 : P ≠ 0 := by
    intro hz
    rw [hz, natDegree_zero] at hPd
    omega
  have hLP : L ∣ P := field_vanishing_divides P (by
    intro x
    simpa only [P, eval_sub, eval_pow, eval_C] using sub_eq_zero.mpr (hc x))
  obtain ⟨g, hg⟩ := hLP
  have hg0 : g ≠ 0 := by
    intro hz
    apply hP0
    rw [hg, hz, mul_zero]
  have hgd : q + g.natDegree = 3 * n := by
    have he := congrArg Polynomial.natDegree hg
    rw [natDegree_mul hL0 hg0, hLd] at he
    exact he.symm.trans hPd
  have hdL : derivative L = -1 := by
    dsimp [L, q]
    rw [derivative_sub, derivative_X_pow, derivative_X, Nat.cast_card_eq_zero K,
      C_0, zero_mul, zero_sub]
  have he := congrArg (fun a : K[X] => f * derivative a) hg
  change f * derivative (f ^ n - C ε) = f * derivative (L * g) at he
  rw [derivative_sub, derivative_C, sub_zero, derivative_mul, hdL, derivative_pow] at he
  have hpow : f * f ^ (n - 1) = f ^ n := by
    rw [← pow_succ']
    congr 1
    omega
  have he2 : C (n : K) * f ^ n * derivative f = f * (-1 * g + L * derivative g) := by
    calc
      _ = f * (C (n : K) * f ^ (n - 1) * derivative f) := by rw [← hpow]; ring
      _ = _ := he
  have hfpow : f ^ n = L * g + C ε := sub_eq_iff_eq_add.mp hg
  rw [hfpow] at he2
  have hi : L * (f * derivative g - C (n : K) * derivative f * g) =
      f * g + C (n : K) * C ε * derivative f := by
    linear_combination -he2
  have hfder : (derivative f).natDegree ≤ 2 := by
    have hb := natDegree_derivative_le f
    rw [hf] at hb
    exact hb
  have hsdeg : (C (n : K) * C ε * derivative f).natDegree ≤ 2 := by
    rw [← C_mul]
    exact (natDegree_C_mul_le _ _).trans hfder
  have hfg : (f * g).natDegree ≤ f.natDegree + g.natDegree := natDegree_mul_le
  have hfglt : (f * g).natDegree < q := by
    rw [hf] at hfg
    omega
  have hrdeg : (f * g + C (n : K) * C ε * derivative f).natDegree < q :=
    lt_of_le_of_lt (natDegree_add_le _ _) (max_lt
      hfglt
      (lt_of_le_of_lt hsdeg (by omega)))
  have hr0 : f * g + C (n : K) * C ε * derivative f = 0 := by
    by_contra hz
    have hb := natDegree_le_of_dvd ⟨_, hi.symm⟩ hz
    rw [hLd] at hb
    omega
  have heq : f * g = -(C (n : K) * C ε * derivative f) := add_eq_zero_iff_eq_neg.mp hr0
  have hb : (f * g).natDegree ≤ 2 := by
    rw [heq, natDegree_neg]
    exact hsdeg
  rw [natDegree_mul hf0 hg0, hf] at hb
  omega

#print axioms field_vanishing_divides
#print axioms cubic_euler_nonconstant
end
end PrimeTraceWeakBoundAudit
