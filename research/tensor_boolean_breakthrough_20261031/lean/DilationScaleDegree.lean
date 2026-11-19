import DilationScaleSpecialization

/-!
# Structural degree bound for simultaneous Semaev scaling

This module assigns a total-exponent bound to the nested polynomial-ring
representation of the canonical Semaev polynomial.  It proves that replacing
every input by `tᵢ X` produces a univariate polynomial of at most that degree.
The bound is field-independent and tuple-independent.  A closed arity-only
estimate for this structural quantity remains a separate theorem.
-/

namespace DilationSemaevBridge

noncomputable section

open Polynomial RecursiveSemaevDegreeAudit

universe u v

/-- Maximum of `coefficient bound + outer exponent` over the actual support. -/
def polynomialScaleDegreeBound {R : Type u} [Semiring R]
    (coefficientBound : R → ℕ) (p : Polynomial R) : ℕ :=
  p.support.sup fun i => coefficientBound (p.coeff i) + i

/-- Evaluating one polynomial layer at a linear polynomial respects the
support-wise scale-degree bound. -/
theorem natDegree_eval₂_le_polynomialScaleDegreeBound
    {R : Type u} [Semiring R] {S : Type v} [Semiring S]
    (coefficientBound : R → ℕ) (p : Polynomial R) (phi : R →+* Polynomial S)
    (x : Polynomial S)
    (coefficientDegree : ∀ i ∈ p.support,
      (phi (p.coeff i)).natDegree ≤ coefficientBound (p.coeff i))
    (xDegree : x.natDegree ≤ 1) :
    (p.eval₂ phi x).natDegree ≤ polynomialScaleDegreeBound coefficientBound p := by
  rw [Polynomial.eval₂_eq_sum, Polynomial.sum_def]
  apply Polynomial.natDegree_sum_le_of_forall_le
  intro i hi
  calc
    (phi (p.coeff i) * x ^ i).natDegree ≤
        (phi (p.coeff i)).natDegree + (x ^ i).natDegree :=
      Polynomial.natDegree_mul_le
    _ ≤ coefficientBound (p.coeff i) + i := by
      gcongr
      · exact coefficientDegree i hi
      · calc
          (x ^ i).natDegree ≤ i * x.natDegree := Polynomial.natDegree_pow_le
          _ ≤ i * 1 := Nat.mul_le_mul_left i xDegree
          _ = i := Nat.mul_one i
    _ ≤ polynomialScaleDegreeBound coefficientBound p := by
      unfold polynomialScaleDegreeBound
      exact Finset.le_sup (s := p.support)
        (f := fun i => coefficientBound (p.coeff i) + i) hi

theorem linearPolynomial_natDegree_le_one {K : Type u} [Field K] (t : K) :
    (Polynomial.C t * Polynomial.X).natDegree ≤ 1 := by
  exact (Polynomial.natDegree_mul_le.trans (by simp))

/-- Substitution `X ↦ tX` does not increase ordinary univariate degree. -/
theorem natDegree_eval₂_C_linear_le {K : Type u} [Field K]
    (p : Polynomial K) (t : K) :
    (p.eval₂ Polynomial.C (Polynomial.C t * Polynomial.X)).natDegree ≤
      p.natDegree := by
  rw [Polynomial.eval₂_eq_sum, Polynomial.sum_def]
  apply Polynomial.natDegree_sum_le_of_forall_le
  intro i hi
  calc
    (Polynomial.C (p.coeff i) * (Polynomial.C t * Polynomial.X) ^ i).natDegree ≤
        (Polynomial.C (p.coeff i)).natDegree +
          ((Polynomial.C t * Polynomial.X) ^ i).natDegree :=
      Polynomial.natDegree_mul_le
    _ ≤ 0 + i := by
      gcongr
      · simp
      · calc
          ((Polynomial.C t * Polynomial.X) ^ i).natDegree ≤
              i * (Polynomial.C t * Polynomial.X).natDegree :=
            Polynomial.natDegree_pow_le
          _ ≤ i * 1 := Nat.mul_le_mul_left i (linearPolynomial_natDegree_le_one t)
          _ = i := Nat.mul_one i
    _ = i := Nat.zero_add i
    _ ≤ p.natDegree := Polynomial.le_natDegree_of_mem_supp i hi

/-- Structural total-exponent bound for the nested coefficient ring holding
the first `n+2` input variables. -/
def inputScaleDegreeBound {K : Type u} [Field K] :
    (n : ℕ) → InputRing K n → ℕ
  | 0, p => polynomialScaleDegreeBound Polynomial.natDegree p
  | n + 1, p => polynomialScaleDegreeBound (inputScaleDegreeBound n) p

set_option backward.isDefEq.respectTransparency false in
theorem inputEvalTo_scale_natDegree_le {K : Type u} [Field K] :
    ∀ (n : ℕ) (t : Fin (n + 2) → K) (p : InputRing K n),
      (inputEvalTo Polynomial.C n
        (fun i => Polynomial.C (t i) * Polynomial.X) p).natDegree ≤
          inputScaleDegreeBound n p := by
  intro n
  induction n with
  | zero =>
      intro t p
      change (p.eval₂
        (Polynomial.eval₂RingHom Polynomial.C
          (Polynomial.C (t 0) * Polynomial.X))
        (Polynomial.C (t 1) * Polynomial.X)).natDegree ≤ _
      apply natDegree_eval₂_le_polynomialScaleDegreeBound
      · intro i hi
        exact natDegree_eval₂_C_linear_le (p.coeff i) (t 0)
      · exact linearPolynomial_natDegree_le_one (t 1)
  | succ n ih =>
      intro t p
      change (p.eval₂
        (inputEvalTo Polynomial.C n
          (fun i => Polynomial.C (Fin.init t i) * Polynomial.X))
        (Polynomial.C (t (Fin.last (n + 2))) * Polynomial.X)).natDegree ≤ _
      apply natDegree_eval₂_le_polynomialScaleDegreeBound
      · intro i hi
        exact ih (Fin.init t) (p.coeff i)
      · exact linearPolynomial_natDegree_le_one (t (Fin.last (n + 2)))

/-- Structural bound for all `n+3` inputs of the canonical polynomial. -/
def canonicalScaleDegreeBound {K : Type u} [Field K]
    (W : WeierstrassCurve.Affine K) (n : ℕ) : ℕ :=
  polynomialScaleDegreeBound (inputScaleDegreeBound n)
    (recursivePolynomial W n)

theorem simultaneousScalePolynomial_natDegree_le
    {K : Type u} [Field K] (W : WeierstrassCurve.Affine K)
    (n : ℕ) (t : Fin (n + 3) → K) :
    (simultaneousScalePolynomial W n t).natDegree ≤
      canonicalScaleDegreeBound W n := by
  unfold simultaneousScalePolynomial evalCanonicalTo canonicalScaleDegreeBound
  apply natDegree_eval₂_le_polynomialScaleDegreeBound
  · intro i hi
    exact inputEvalTo_scale_natDegree_le n (Fin.init t)
      ((recursivePolynomial W n).coeff i)
  · exact linearPolynomial_natDegree_le_one (t (Fin.last (n + 2)))

end

end DilationSemaevBridge
