import Mathlib.RingTheory.Polynomial.Eisenstein.Basic
import Mathlib.RingTheory.Ideal.Maximal
import Mathlib.Tactic.IntervalCases
import Mathlib.Algebra.MvPolynomial.Division

/-! A reusable Eisenstein step for the explicit infinity-chart equation. -/

set_option autoImplicit false

noncomputable section

namespace Common22Eisenstein

open Polynomial

variable {R : Type*} [CommRing R] [IsDomain R]

/-- The constant term of `W² - t*u` has exactly one factor of `t` if
`u` is not divisible by `t`. -/
theorem constant_not_mem_prime_square (t u : R) (ht : Prime t) (hu : ¬t ∣ u) :
    -(t * u) ∉ (Ideal.span {t} : Ideal R) ^ 2 := by
  intro h
  have hd : t * t ∣ t * u := by
    have h' : t * u ∈ (Ideal.span {t} : Ideal R) ^ 2 := (Ideal.neg_mem_iff _).mp h
    rw [Ideal.span_singleton_pow] at h'
    simpa only [Ideal.mem_span_singleton, pow_two] using h'
  exact hu ((mul_dvd_mul_iff_left ht.ne_zero).mp hd)

/-- Eisenstein at `(t)` in the outer variable, uniformly in the coefficient
domain and in every characteristic. -/
theorem outer_quadratic_irreducible (t u : R) (ht : Prime t) (hu : ¬t ∣ u) :
    Irreducible (X ^ 2 - C (t * u) : R[X]) := by
  let P : Ideal R := Ideal.span {t}
  have hmonic : (X ^ 2 - C (t * u) : R[X]).Monic :=
    monic_X_pow_sub_C _ (by decide)
  have hE : (X ^ 2 - C (t * u) : R[X]).IsEisensteinAt P := by
    apply hmonic.isEisensteinAt_of_mem_of_notMem
    · exact (Ideal.isPrime_span_singleton_of_prime ht).ne_top
    · intro n hn
      rw [natDegree_X_pow_sub_C] at hn
      interval_cases n <;> simp [P, coeff_X_pow, Ideal.mem_span_singleton]
    · simpa [P] using constant_not_mem_prime_square t u ht hu
  have hdegree : (X ^ 2 - C (t * u) : R[X]).natDegree = 2 :=
    natDegree_X_pow_sub_C
  exact hE.irreducible (Ideal.isPrime_span_singleton_of_prime ht)
    hmonic.isPrimitive (by rw [hdegree]; decide)

end Common22Eisenstein

namespace Common22Eisenstein

open MvPolynomial

variable {K : Type*} [Field K]

def innerU (a3 a2 a1 a0 : K) : MvPolynomial (Fin 1) K :=
  1 + MvPolynomial.C a3 * MvPolynomial.X 0 ^ 2 +
    MvPolynomial.C a2 * MvPolynomial.X 0 ^ 3 +
    MvPolynomial.C a1 * MvPolynomial.X 0 ^ 4 +
    MvPolynomial.C a0 * MvPolynomial.X 0 ^ 5

theorem innerU_not_dvd (a3 a2 a1 a0 : K) :
    ¬(MvPolynomial.X 0 : MvPolynomial (Fin 1) K) ∣ innerU a3 a2 a1 a0 := by
  intro h
  rcases h with ⟨q, hq⟩
  have hh := congrArg (fun p : MvPolynomial (Fin 1) K => p.constantCoeff) hq
  simp [innerU] at hh

/-- The exact outer quadratic arising after separating the two variables is
irreducible over the one-variable polynomial coefficient ring. -/
theorem nested_chart_irreducible (a3 a2 a1 a0 : K) :
    Irreducible (Polynomial.X ^ 2 -
      Polynomial.C ((MvPolynomial.X 0 : MvPolynomial (Fin 1) K) *
        innerU a3 a2 a1 a0)) := by
  exact outer_quadratic_irreducible (MvPolynomial.X 0) (innerU a3 a2 a1 a0)
    (show Prime (MvPolynomial.X 0 : MvPolynomial (Fin 1) K) from MvPolynomial.X_prime)
    (innerU_not_dvd a3 a2 a1 a0)

end Common22Eisenstein

#print axioms Common22Eisenstein.constant_not_mem_prime_square
#print axioms Common22Eisenstein.outer_quadratic_irreducible
#print axioms Common22Eisenstein.innerU_not_dvd
#print axioms Common22Eisenstein.nested_chart_irreducible
