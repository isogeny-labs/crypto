import Mathlib.Algebra.Polynomial.Derivation
import Mathlib.FieldTheory.RatFunc.Basic
import Mathlib.RingTheory.LaurentSeries

/-!
# H116 generic fiber-trace identity layer

These theorems prove the product logarithmic-derivative, denominator
cancellation, abstract coefficient/power-sum, and simple-residue steps used by
H116. They deliberately do not assert the missing elliptic formal-group jet,
an exact scalar-digit predicate, a decoder, or a subexponential ECDLP bound.
-/

namespace H116FiberTrace

variable {K L : Type*} [Field K] [Field L] [Algebra K L]

/-- The exact two-factor logarithmic-derivative law in any differential field. -/
theorem logDeriv_mul (D : Derivation K L L) {a b : L} (ha : a ≠ 0) (hb : b ≠ 0) :
    D (a * b) / (a * b) = D a / a + D b / b := by
  rw [D.leibniz]
  simp only [smul_eq_mul]
  field_simp
  ring

/-- The logarithmic derivative of a nonzero finite product is the sum of the
logarithmic derivatives of its factors. -/
theorem logDeriv_finset (D : Derivation K L L) {ι : Type*} [DecidableEq ι]
    (s : Finset ι) (f : ι → L) (hf : ∀ i ∈ s, f i ≠ 0) :
    D (∏ i ∈ s, f i) / (∏ i ∈ s, f i) = ∑ i ∈ s, D (f i) / f i := by
  induction s using Finset.induction with
  | empty => simp
  | @insert a s ha ih =>
      have hfa : f a ≠ 0 := hf a (Finset.mem_insert_self a s)
      have hfs : ∀ i ∈ s, f i ≠ 0 := fun i hi => hf i (Finset.mem_insert_of_mem hi)
      rw [Finset.prod_insert ha, Finset.sum_insert ha]
      rw [logDeriv_mul D hfa (Finset.prod_ne_zero_iff.mpr hfs), ih hfs]

/-- Division subtracts logarithmic derivatives. -/
theorem logDeriv_div (D : Derivation K L L) {a b : L} (ha : a ≠ 0) (hb : b ≠ 0) :
    D (a / b) / (a / b) = D a / a - D b / b := by
  have hab : a / b = a * b⁻¹ := div_eq_mul_inv a b
  rw [hab]
  rw [logDeriv_mul D ha (inv_ne_zero hb), D.leibniz_inv]
  simp only [smul_eq_mul]
  field_simp
  ring

/-- In a relative logarithmic derivative, a shared denominator cancels exactly. -/
theorem denominator_cancellation (D : Derivation K L L) {nu nv d : L}
    (hnu : nu ≠ 0) (hnv : nv ≠ 0) (hd : d ≠ 0) :
    D (nu / d) / (nu / d) - D (nv / d) / (nv / d) =
      D nu / nu - D nv / nv := by
  rw [logDeriv_div D hnu hd, logDeriv_div D hnv hd]
  ring

/-- A factored fiber polynomial has logarithmic derivative equal to the sum of
its reciprocal linear factors. Root multiplicities are represented by indices. -/
theorem rootProductLogDeriv (D : Derivation K L L) {ι : Type*} [DecidableEq ι]
    (s : Finset ι) (x : L) (roots : ι → L)
    (hx : D x = 1) (hroots : ∀ i ∈ s, D (roots i) = 0)
    (hne : ∀ i ∈ s, x - roots i ≠ 0) :
    D (∏ i ∈ s, (x - roots i)) / (∏ i ∈ s, (x - roots i)) =
      ∑ i ∈ s, 1 / (x - roots i) := by
  rw [logDeriv_finset D s (fun i => x - roots i) hne]
  apply Finset.sum_congr rfl
  intro i hi
  rw [D.map_sub, hx, hroots i hi]
  simp

/-- Any additive coefficient functional that maps reciprocal factors to powers
maps the product logarithmic derivative to the corresponding power sum. This
isolates the still-open concrete Laurent-at-infinity instance. -/
theorem rootPowerSumOfCoeff (D : Derivation K L L) {ι : Type*} [DecidableEq ι]
    (s : Finset ι) (x : L) (roots : ι → L) (j : ℕ) (coeff : L →+ L)
    (hx : D x = 1) (hroots : ∀ i ∈ s, D (roots i) = 0)
    (hne : ∀ i ∈ s, x - roots i ≠ 0)
    (hcoeff : ∀ i ∈ s, coeff (1 / (x - roots i)) = roots i ^ j) :
    coeff (D (∏ i ∈ s, (x - roots i)) / (∏ i ∈ s, (x - roots i))) =
      ∑ i ∈ s, roots i ^ j := by
  rw [rootProductLogDeriv D s x roots hx hroots hne, map_sum]
  apply Finset.sum_congr rfl
  intro i hi
  exact hcoeff i hi

/-- Relative denominator cancellation survives every additive coefficient
functional, in particular any valid Laurent coefficient extraction map. -/
theorem relativeCoeffCancellation (D : Derivation K L L) (coeff : L →+ L)
    {nu nv d : L} (hnu : nu ≠ 0) (hnv : nv ≠ 0) (hd : d ≠ 0) :
    coeff (D (nu / d) / (nu / d) - D (nv / d) / (nv / d)) =
      coeff (D nu / nu - D nv / nv) := by
  exact congrArg coeff (denominator_cancellation D hnu hnv hd)

/-- The shared denominator in `F'_u(α)=D(α)R'(α)` cancels from the simple-root
residue. This is the algebraic core of the H116 `Y`-moment residue step. -/
theorem residueCancelDenominator {den rprime fprime : L}
    (hden : den ≠ 0) (hrprime : rprime ≠ 0)
    (hderiv : fprime = den * rprime) : den / fprime = 1 / rprime := by
  rw [hderiv]
  field_simp

/-- Mathlib's finite-product polynomial derivative theorem, exposed with the
exact H116 project dependency and no local re-derivation. -/
theorem polynomialProductDerivative {ι : Type*} [DecidableEq ι]
    (s : Finset ι) (f : ι → Polynomial K) :
    Polynomial.derivative (∏ b ∈ s, f b) =
      ∑ a ∈ s, (∏ b ∈ s.erase a, f b) * Polynomial.derivative (f a) :=
  Polynomial.derivative_prod_finset

#print axioms logDeriv_mul
#print axioms logDeriv_finset
#print axioms logDeriv_div
#print axioms denominator_cancellation
#print axioms rootProductLogDeriv
#print axioms rootPowerSumOfCoeff
#print axioms relativeCoeffCancellation
#print axioms residueCancelDenominator
#print axioms polynomialProductDerivative

end H116FiberTrace
