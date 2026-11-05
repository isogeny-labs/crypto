import SecantQuadratic
import Mathlib.Algebra.MvPolynomial.CommRing

/-! Complete auxiliary-variable top ideal for Boolean ordinate offsets.
    The degree law is proved here rather than inferred from input degree. -/
noncomputable section
open MvPolynomial Finsupp
open scoped BigOperators
namespace PencilTop
variable {K : Type*} [Field K]
abbrev Vars (n : ℕ) := Sum (Fin 3) (Fin n)
abbrev P (K : Type*) [CommSemiring K] (n : ℕ) := MvPolynomial (Vars n) K

def ideal (n : ℕ) (a b : K) : Ideal (P K n) :=
  Ideal.span ({(X (Sum.inl 2))^2, X (Sum.inl 0)*X (Sum.inl 1),
    X (Sum.inl 0)^2-C a*X (Sum.inl 2)*X (Sum.inl 0),
    X (Sum.inl 1)^2-C b*X (Sum.inl 2)*X (Sum.inl 1)} ∪
      Set.range (fun i : Fin n => (X (Sum.inr i))^2))

theorem core_members (n : ℕ) (a b : K) :
    (X (Sum.inl 2) : P K n)^2 ∈ ideal n a b ∧
    X (Sum.inl 0)*X (Sum.inl 1) ∈ ideal n a b ∧
    X (Sum.inl 0)^3 ∈ ideal n a b ∧ X (Sum.inl 1)^3 ∈ ideal n a b ∧
    X (Sum.inl 0)^2*X (Sum.inl 2) ∈ ideal n a b ∧
    X (Sum.inl 1)^2*X (Sum.inl 2) ∈ ideal n a b := by
  let I := ideal n a b
  have hm : (X (Sum.inl 2) : P K n)^2 ∈ I :=
    Ideal.subset_span (by simp [ideal])
  have hxz : (X (Sum.inl 0) : P K n)*X (Sum.inl 1) ∈ I :=
    Ideal.subset_span (by simp [ideal])
  have hx : (X (Sum.inl 0) : P K n)^2-C a*X (Sum.inl 2)*X (Sum.inl 0) ∈ I :=
    Ideal.subset_span (by simp [ideal])
  have hz : (X (Sum.inl 1) : P K n)^2-C b*X (Sum.inl 2)*X (Sum.inl 1) ∈ I :=
    Ideal.subset_span (by simp [ideal])
  have hxm : (X (Sum.inl 0) : P K n)^2*X (Sum.inl 2) ∈ I := by
    convert I.add_mem (I.mul_mem_left (X (Sum.inl 2)) hx)
      (I.mul_mem_left (C a*X (Sum.inl 0)) hm) using 1
    ring
  have hzm : (X (Sum.inl 1) : P K n)^2*X (Sum.inl 2) ∈ I := by
    convert I.add_mem (I.mul_mem_left (X (Sum.inl 2)) hz)
      (I.mul_mem_left (C b*X (Sum.inl 1)) hm) using 1
    ring
  have hx3 : (X (Sum.inl 0) : P K n)^3 ∈ I := by
    convert I.add_mem (I.mul_mem_left (X (Sum.inl 0)) hx)
      (I.mul_mem_left (C a) hxm) using 1
    ring
  have hz3 : (X (Sum.inl 1) : P K n)^3 ∈ I := by
    convert I.add_mem (I.mul_mem_left (X (Sum.inl 1)) hz)
      (I.mul_mem_left (C b) hzm) using 1
    ring
  exact ⟨hm, hxz, hx3, hz3, hxm, hzm⟩

lemma monomial_upward {σ : Type*} (I : Ideal (MvPolynomial σ K))
    {g e : σ →₀ ℕ} (hge : g ≤ e) (h : monomial g (1 : K) ∈ I) (c : K) :
    monomial e c ∈ I := by
  have hm := I.mul_mem_left (monomial (e-g) c) h
  simpa only [monomial_mul, mul_one, tsub_add_cancel_of_le hge] using hm

theorem degree_vars (n : ℕ) (e : Vars n →₀ ℕ) :
    e.degree = e (Sum.inl 0)+e (Sum.inl 1)+e (Sum.inl 2)+∑ i, e (Sum.inr i) := by
  change e.sum (fun _ j => j) = _
  rw [Finsupp.sum_fintype]
  · rw [Fintype.sum_sum_type]
    simp [Fin.sum_univ_succ, add_assoc]
  · intro i
    rfl

theorem all_high_monomials (n : ℕ) (a b : K) (e : Vars n →₀ ℕ)
    (he : n+3 ≤ e.degree) (c : K) : monomial e c ∈ ideal n a b := by
  classical
  obtain ⟨hm, hxz, hx3, hz3, hxm, hzm⟩ := core_members n a b
  by_cases hbits : ∃ i : Fin n, 2 ≤ e (Sum.inr i)
  · obtain ⟨i, hi⟩ := hbits
    apply monomial_upward _ (Finsupp.single_le_iff.mpr hi) _ c
    have h : (X (Sum.inr i) : P K n)^2 ∈ ideal n a b :=
      Ideal.subset_span (by simp [ideal])
    simpa [← X_pow_eq_monomial] using h
  have hb : (∑ i : Fin n, e (Sum.inr i)) ≤ n := by
    calc
      _ ≤ ∑ _i : Fin n, 1 := by
        apply Finset.sum_le_sum
        intro i hi
        have hn : ¬ 2 ≤ e (Sum.inr i) := fun hh => hbits ⟨i, hh⟩
        omega
      _ = n := by simp
  rw [degree_vars] at he
  have hc : 3 ≤ e (Sum.inl 0)+e (Sum.inl 1)+e (Sum.inl 2) := by omega
  by_cases h : 2 ≤ e (Sum.inl 2)
  · apply monomial_upward _ (Finsupp.single_le_iff.mpr h) _ c
    simpa [← X_pow_eq_monomial] using hm
  by_cases hxz' : 1 ≤ e (Sum.inl 0) ∧ 1 ≤ e (Sum.inl 1)
  · apply monomial_upward _ (show single (Sum.inl 0) 1+single (Sum.inl 1) 1 ≤ e from ?_) _ c
    · intro i
      rcases i with i | i
      · fin_cases i <;> simp_all
      · simp
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using hxz
  by_cases hx3' : 3 ≤ e (Sum.inl 0)
  · apply monomial_upward _ (Finsupp.single_le_iff.mpr hx3') _ c
    simpa [← X_pow_eq_monomial] using hx3
  by_cases hz3' : 3 ≤ e (Sum.inl 1)
  · apply monomial_upward _ (Finsupp.single_le_iff.mpr hz3') _ c
    simpa [← X_pow_eq_monomial] using hz3
  have hlast : (2 ≤ e (Sum.inl 0) ∧ 1 ≤ e (Sum.inl 2)) ∨
      (2 ≤ e (Sum.inl 1) ∧ 1 ≤ e (Sum.inl 2)) := by omega
  rcases hlast with hlast | hlast
  · apply monomial_upward _ (show single (Sum.inl 0) 2+single (Sum.inl 2) 1 ≤ e from ?_) _ c
    · intro i
      rcases i with i | i
      · fin_cases i <;> simp_all
      · simp
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using hxm
  · apply monomial_upward _ (show single (Sum.inl 1) 2+single (Sum.inl 2) 1 ≤ e from ?_) _ c
    · intro i
      rcases i with i | i
      · fin_cases i <;> simp_all
      · simp
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using hzm

theorem vanishes (n : ℕ) (a b : K) : SecantIdeal.Vanishes (ideal n a b) (n+3) := by
  classical
  intro p hp
  rw [p.as_sum]
  apply Ideal.sum_mem
  intro e he
  exact all_high_monomials n a b e (by rw [SecantIdeal.homogeneous_degree hp he]) _

abbrev WitnessVars (n : ℕ) := Sum (Fin 2) (Fin n)
abbrev R (K : Type*) [Field K] (n : ℕ) := MvPolynomial (WitnessVars n) K

def allBits (n : ℕ) : WitnessVars n →₀ ℕ := ∑ i, single i 1
@[simp] theorem allBits_apply (n : ℕ) (i : WitnessVars n) : allBits n i = 1 := by
  classical
  rcases i with i | i
  · fin_cases i <;> simp [allBits]
  · simp [allBits, Finsupp.single_apply]
theorem allBits_support (n : ℕ) : (allBits n).support = Finset.univ := by
  classical
  ext i
  simp [Finsupp.mem_support_iff]

def squares (n : ℕ) : Ideal (R K n) :=
  Ideal.span ((fun e => monomial e (1 : K)) ''
    Set.range (fun i : WitnessVars n => single i 2))
theorem square_mem (n : ℕ) (i : WitnessVars n) :
    (X i : R K n)^2 ∈ squares n := by
  apply Ideal.subset_span
  exact ⟨single i 2, ⟨i, rfl⟩, by simp [← X_pow_eq_monomial]⟩

def wx (n : ℕ) (a : K) : R K n :=
  C (a/2)*X (Sum.inl 0)+X (Sum.inl 1)
def raw (n : ℕ) (a : K) : P K n →+* R K n :=
  eval₂Hom C (Sum.elim
    (fun i : Fin 3 => if i = 0 then wx n a else if i = 1 then 0 else X (Sum.inl 0))
    (fun i : Fin n => X (Sum.inr i)))
@[simp] theorem raw_C (n : ℕ) (a c : K) : raw n a (C c) = C c := eval₂Hom_C _ _ _
@[simp] theorem raw_core (n : ℕ) (a : K) (i : Fin 3) :
    raw n a (X (Sum.inl i)) =
      if i = 0 then wx n a else if i = 1 then 0 else X (Sum.inl 0) :=
  eval₂Hom_X' _ _ _
@[simp] theorem raw_bit (n : ℕ) (a : K) (i : Fin n) :
    raw n a (X (Sum.inr i)) = X (Sum.inr i) := eval₂Hom_X' _ _ _

theorem wx_relation (n : ℕ) (a : K) (h2 : (2 : K) ≠ 0) :
    wx n a^2-C a*X (Sum.inl 0)*wx n a =
      C ((a/2)^2-a*(a/2))*(X (Sum.inl 0) : R K n)^2+X (Sum.inl 1)^2 := by
  have hhalf : a/2+a/2 = a := by field_simp [h2]; ring
  have hC : C (a/2)+C (a/2) = (C a : R K n) := by rw [← _root_.map_add, hhalf]
  simp only [wx, _root_.map_sub, _root_.map_mul, _root_.map_pow]
  linear_combination X (Sum.inl 0)*X (Sum.inl 1)*hC

theorem ideal_le_comap (n : ℕ) (a b : K) (h2 : (2 : K) ≠ 0) :
    ideal n a b ≤ Ideal.comap (raw n a) (squares n) := by
  classical
  apply Ideal.span_le.mpr
  intro p hp
  change raw n a p ∈ squares n
  rcases hp with hp | hp
  · simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hp
    rcases hp with rfl | rfl | rfl | rfl
    · simpa [_root_.map_pow] using square_mem (K := K) n (Sum.inl 0)
    · simp [_root_.map_mul]
    · simp only [_root_.map_sub, _root_.map_pow, _root_.map_mul, raw_core, raw_C]
      simp only [if_true, show (2 : Fin 3) ≠ 0 by decide,
        show (2 : Fin 3) ≠ 1 by decide, if_false]
      rw [wx_relation n a h2]
      exact (squares n).add_mem
        ((squares n).mul_mem_left _ (square_mem n (Sum.inl 0))) (square_mem n (Sum.inl 1))
    · simp [_root_.map_sub, _root_.map_pow, _root_.map_mul]
  · obtain ⟨i, rfl⟩ := hp
    simpa [_root_.map_pow] using square_mem (K := K) n (Sum.inr i)

theorem squarefree_not_mem (n : ℕ) :
    (X (Sum.inl 0)*X (Sum.inl 1)*∏ i : Fin n, X (Sum.inr i) : R K n) ∉ squares n := by
  classical
  have heq : (X (Sum.inl 0)*X (Sum.inl 1)*∏ i : Fin n, X (Sum.inr i) : R K n) =
      monomial (allBits n) 1 := by
    have hh := prod_X_pow_eq_monomial (R := K) (s := allBits n)
    simpa [allBits_support, Fintype.prod_sum_type, Fin.prod_univ_succ, mul_assoc] using hh
  rw [heq]
  intro h
  have hs := (mem_ideal_span_monomial_image.mp h) (allBits n)
    (by simp [MvPolynomial.support_monomial])
  obtain ⟨g, ⟨i, rfl⟩, hi⟩ := hs
  have hh := hi i
  simp at hh

def highWitness (n : ℕ) : P K n :=
  X (Sum.inl 2)*X (Sum.inl 0)*∏ i : Fin n, X (Sum.inr i)

theorem vanishes_succ {σ : Type*} (I : Ideal (MvPolynomial σ K)) (d : ℕ)
    (hv : SecantIdeal.Vanishes I d) : SecantIdeal.Vanishes I (d+1) := by
  classical
  intro p hp
  rw [p.as_sum]
  apply Ideal.sum_mem
  intro e he
  have hdeg := SecantIdeal.homogeneous_degree hp he
  have he0 : e ≠ 0 := by
    intro hz
    simp [hz] at hdeg
  obtain ⟨i, hi⟩ : ∃ i, e i ≠ 0 := by
    by_contra! hh
    apply he0
    ext i
    simp [hh]
  have hle : single i 1 ≤ e := Finsupp.single_le_iff.mpr (by omega)
  have hsub : (e-single i 1).degree = d := by
    have hh := congrArg Finsupp.degree (tsub_add_cancel_of_le hle)
    rw [Finsupp.degree_add, Finsupp.degree_single] at hh
    omega
  have hm := hv (monomial (e-single i 1) (p.coeff e))
    (isHomogeneous_monomial _ hsub)
  have hh := I.mul_mem_left (monomial (single i 1) (1 : K)) hm
  simpa only [monomial_mul, one_mul, add_comm (single i 1),
    tsub_add_cancel_of_le hle] using hh

theorem vanishes_mono {σ : Type*} (I : Ideal (MvPolynomial σ K))
    {d e : ℕ} (hv : SecantIdeal.Vanishes I d) (hde : d ≤ e) :
    SecantIdeal.Vanishes I e := by
  induction e, hde using Nat.le_induction with
  | base => exact hv
  | succ e hde ih => exact vanishes_succ I e ih

theorem highWitness_homogeneous (n : ℕ) :
    (highWitness n : P K n).IsHomogeneous (n+2) := by
  classical
  have hp := IsHomogeneous.prod (Finset.univ : Finset (Fin n))
    (fun i => (X (Sum.inr i) : P K n)) (fun _ => 1)
    (fun i _ => isHomogeneous_X K (Sum.inr i))
  simpa [highWitness, Nat.add_comm] using
    ((isHomogeneous_X K (Sum.inl 2)).mul (isHomogeneous_X K (Sum.inl 0))).mul hp

theorem highWitness_not_mem (n : ℕ) (a b : K) (h2 : (2 : K) ≠ 0) :
    (highWitness n : P K n) ∉ ideal n a b := by
  intro h
  have hh := ideal_le_comap n a b h2 h
  change raw n a (highWitness n) ∈ squares n at hh
  have ht : (C (a/2)*(X (Sum.inl 0) : R K n)^2*
      ∏ i : Fin n, X (Sum.inr i)) ∈ squares n :=
    (squares n).mul_mem_right _ ((squares n).mul_mem_left _ (square_mem n (Sum.inl 0)))
  have hsub := (squares (K := K) n).sub_mem hh ht
  have heq : raw n a (highWitness n)-C (a/2)*(X (Sum.inl 0) : R K n)^2*
      (∏ i : Fin n, X (Sum.inr i)) =
        X (Sum.inl 0)*X (Sum.inl 1)*∏ i : Fin n, X (Sum.inr i) := by
    simp [highWitness, _root_.map_mul, _root_.map_prod, wx]
    ring
  rw [heq] at hsub
  exact squarefree_not_mem n hsub

theorem last_nonzero (n : ℕ) (a b : K) (h2 : (2 : K) ≠ 0) :
    ¬ SecantIdeal.Vanishes (ideal n a b) (n+2) := by
  intro hv
  exact highWitness_not_mem n a b h2 (hv _ (highWitness_homogeneous n))

theorem first_vanishing (n : ℕ) (a b : K) (h2 : (2 : K) ≠ 0) :
    SecantIdeal.FirstVanishing (ideal n a b) (n+3) := by
  refine ⟨vanishes n a b, ?_⟩
  intro d hd hv
  exact last_nonzero n a b h2 (vanishes_mono (ideal n a b) hv (by omega))

end PencilTop

#print axioms PencilTop.vanishes
#print axioms PencilTop.last_nonzero
#print axioms PencilTop.first_vanishing
