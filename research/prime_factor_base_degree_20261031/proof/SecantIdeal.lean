import Mathlib.RingTheory.MvPolynomial.Ideal
import Mathlib.RingTheory.MvPolynomial.Homogeneous
import Mathlib.RingTheory.Ideal.Quotient.Defs
import Mathlib.Tactic.FinCases
import SecantGraphTop
import TopDegree

/-! Actual polynomial-ideal and homogeneous-quotient bridge.
    No solving-degree or timing claim is encoded in these definitions. -/
noncomputable section
open MvPolynomial Finsupp

namespace SecantIdeal

variable {K : Type*} [Field K]

/-- The degree-n homogeneous part maps to zero in the actual quotient. -/
def Vanishes {σ : Type*} (J : Ideal (MvPolynomial σ K)) (n : ℕ) : Prop :=
  ∀ p : MvPolynomial σ K, p.IsHomogeneous n → p ∈ J

/-- First vanishing homogeneous degree, including every lower-degree witness. -/
def FirstVanishing {σ : Type*} (J : Ideal (MvPolynomial σ K)) (n : ℕ) : Prop :=
  Vanishes J n ∧ ∀ k < n, ¬ Vanishes J k

theorem vanishes_iff_quotient_zero {σ : Type*}
    (J : Ideal (MvPolynomial σ K)) (n : ℕ) :
    Vanishes J n ↔ ∀ p : MvPolynomial σ K, p.IsHomogeneous n →
      Ideal.Quotient.mk J p = 0 := by
  simp only [Vanishes, Ideal.Quotient.eq_zero_iff_mem]

abbrev Exp3 := Fin 3 →₀ ℕ
abbrev Exp2 := Fin 2 →₀ ℕ

def secantExponents (d : ℕ) : Set Exp3 :=
  {single 0 d, single 1 d, single 0 1 + single 1 1, single 2 2}

def denseExponents (b : ℕ) : Set Exp2 :=
  {single 0 b, single 1 b, single 0 2 + single 1 2}

def secant (d : ℕ) : Ideal (MvPolynomial (Fin 3) K) :=
  Ideal.span ((fun e => monomial e (1 : K)) '' secantExponents d)

def dense (b : ℕ) : Ideal (MvPolynomial (Fin 2) K) :=
  Ideal.span ((fun e => monomial e (1 : K)) '' denseExponents b)

theorem degree_three (e : Exp3) : e.degree = e 0 + e 1 + e 2 := by
  change e.sum (fun _ n => n) = _
  rw [Finsupp.sum_fintype]
  · simp [Fin.sum_univ_succ, add_assoc]
  · intro i
    rfl

theorem degree_two (e : Exp2) : e.degree = e 0 + e 1 := by
  change e.sum (fun _ n => n) = _
  rw [Finsupp.sum_fintype]
  · simp [Fin.sum_univ_succ]
  · intro i
    rfl

theorem mixed_le_three (e : Exp3) :
    single 0 1 + single 1 1 ≤ e ↔ 1 ≤ e 0 ∧ 1 ≤ e 1 := by
  constructor
  · intro h
    have h0 := h 0
    have h1 := h 1
    simpa using And.intro h0 h1
  · rintro ⟨h0, h1⟩ i
    fin_cases i <;> simp_all

theorem mixed_le_two (e : Exp2) :
    single 0 2 + single 1 2 ≤ e ↔ 2 ≤ e 0 ∧ 2 ≤ e 1 := by
  constructor
  · intro h
    have h0 := h 0
    have h1 := h 1
    simpa using And.intro h0 h1
  · rintro ⟨h0, h1⟩ i
    fin_cases i <;> simp_all

theorem secant_divisibility (d : ℕ) (e : Exp3) :
    (∃ g ∈ secantExponents d, g ≤ e) ↔
      SecantGraphTop.Covered d (e 0) (e 1) (e 2) := by
  simp [secantExponents, SecantGraphTop.Covered, Finsupp.single_le_iff,
    mixed_le_three]

theorem dense_divisibility (b : ℕ) (e : Exp2) :
    (∃ g ∈ denseExponents b, g ≤ e) ↔ DensePairTop.Covered b (e 0) (e 1) := by
  simp [denseExponents, DensePairTop.Covered, Finsupp.single_le_iff, mixed_le_two]

theorem secant_mem (d : ℕ) (p : MvPolynomial (Fin 3) K) :
    p ∈ secant d ↔ ∀ e ∈ p.support,
      SecantGraphTop.Covered d (e 0) (e 1) (e 2) := by
  simp only [secant, mem_ideal_span_monomial_image, secant_divisibility]

theorem dense_mem (b : ℕ) (p : MvPolynomial (Fin 2) K) :
    p ∈ dense b ↔ ∀ e ∈ p.support, DensePairTop.Covered b (e 0) (e 1) := by
  simp only [dense, mem_ideal_span_monomial_image, dense_divisibility]

theorem homogeneous_degree {σ : Type*} {p : MvPolynomial σ K} {n : ℕ}
    (hp : p.IsHomogeneous n) {e : σ →₀ ℕ} (he : e ∈ p.support) : e.degree = n := by
  simpa only [Finsupp.degree_eq_weight_one] using hp (MvPolynomial.mem_support_iff.mp he)

theorem secant_vanishes (d : ℕ) : Vanishes (secant (K := K) d) (d + 1) := by
  intro p hp
  rw [secant_mem]
  intro e he
  have h := homogeneous_degree hp he
  rw [degree_three] at h
  exact SecantGraphTop.all_above d (e 0) (e 1) (e 2) (by omega)

theorem dense_vanishes (b : ℕ) : Vanishes (dense (K := K) b) (b + 1) := by
  intro p hp
  rw [dense_mem]
  intro e he
  have h := homogeneous_degree hp he
  rw [degree_two] at h
  exact DensePairTop.all_above b (e 0) (e 1) (by omega)

theorem secant_survives_below (d n : ℕ) (hd : 2 ≤ d) (hn : n < d + 1) :
    ¬ Vanishes (secant (K := K) d) n := by
  classical
  obtain ⟨i, j, k, hdeg, hnot⟩ := (SecantGraphTop.first_full_degree d hd).2 n hn
  let e : Exp3 := single 0 i + single 1 j + single 2 k
  have he : e.degree = n := by simpa [e, add_assoc] using hdeg
  have hh := isHomogeneous_monomial (1 : K) he
  intro hv
  have hm := (secant_mem d (monomial e (1 : K))).mp (hv _ hh)
  have hm' := hm e (by simp [MvPolynomial.support_monomial])
  apply hnot
  simpa [e] using hm'

theorem dense_survives_below (b n : ℕ) (hb : 2 ≤ b) (hn : n < b + 1) :
    ¬ Vanishes (dense (K := K) b) n := by
  classical
  obtain ⟨i, j, hdeg, hnot⟩ := DensePairTop.survives_below b n hb (by omega)
  let e : Exp2 := single 0 i + single 1 j
  have he : e.degree = n := by simpa [e] using hdeg
  have hh := isHomogeneous_monomial (1 : K) he
  intro hv
  have hm := (dense_mem b (monomial e (1 : K))).mp (hv _ hh)
  have hm' := hm e (by simp [MvPolynomial.support_monomial])
  apply hnot
  simpa [e] using hm'

theorem secant_first_vanishing (d : ℕ) (hd : 2 ≤ d) :
    FirstVanishing (secant (K := K) d) (d + 1) :=
  ⟨secant_vanishes d, fun n hn => secant_survives_below d n hd hn⟩

theorem dense_first_vanishing (b : ℕ) (hb : 2 ≤ b) :
    FirstVanishing (dense (K := K) b) (b + 1) :=
  ⟨dense_vanishes b, fun n hn => dense_survives_below b n hb hn⟩

/-- Exact quotient-degree reduction for the specified genuine polynomial ideals. -/
theorem strict_polynomial_ideal_reduction (d : ℕ) (hd : 2 ≤ d) :
    FirstVanishing (secant (K := K) d) (d + 1) ∧
    FirstVanishing (dense (K := K) (2 * d)) (2 * d + 1) ∧
    d + 1 < 2 * d + 1 := by
  exact ⟨secant_first_vanishing d hd, dense_first_vanishing (2*d) (by omega),
    SecantGraphTop.strict_dense_reduction d hd⟩

end SecantIdeal

#print axioms SecantIdeal.vanishes_iff_quotient_zero
#print axioms SecantIdeal.secant_first_vanishing
#print axioms SecantIdeal.dense_first_vanishing
#print axioms SecantIdeal.strict_polynomial_ideal_reduction
