import Mathlib.Algebra.Polynomial.Derivation
import Mathlib.Data.ZMod.Basic
import Mathlib.FieldTheory.RatFunc.Basic
import Mathlib.RingTheory.LaurentSeries
import Mathlib.RingTheory.PowerSeries.Derivative
import Mathlib.Tactic

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

/-- The coefficient formula implemented by H116's truncated logarithmic
derivative: multiply the formal derivative by a reciprocal series and take the
`n`th coefficient. -/
theorem powerSeriesDerivativeMulCoeff (f g : PowerSeries K) (n : ℕ) :
    PowerSeries.coeff n (PowerSeries.derivative f * g) =
      ∑ i ∈ Finset.range (n + 1),
        (PowerSeries.coeff (i + 1) f * (i + 1)) *
          PowerSeries.coeff (n - i) g := by
  rw [PowerSeries.coeff_mul]
  rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ
    (fun i j => PowerSeries.coeff i (PowerSeries.derivative f) * PowerSeries.coeff j g) n]
  apply Finset.sum_congr rfl
  intro i hi
  rw [PowerSeries.coeff_derivative]

/-- If `g` is a reciprocal power series for a unit-constant series `f`, its
next coefficient is exactly the finite recurrence used by the native producer. -/
theorem powerSeriesReciprocalRecurrence (f g : PowerSeries K)
    (h0 : PowerSeries.coeff 0 f = 1) (hfg : f * g = 1) (n : ℕ) :
    PowerSeries.coeff (n + 1) g =
      -∑ i ∈ Finset.range (n + 1),
        PowerSeries.coeff (i + 1) f * PowerSeries.coeff (n - i) g := by
  have hcoeff := congrArg (PowerSeries.coeff (n + 1)) hfg
  rw [PowerSeries.coeff_mul] at hcoeff
  rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ
    (fun i j => PowerSeries.coeff i f * PowerSeries.coeff j g) (n + 1),
    Finset.sum_range_succ'] at hcoeff
  simp only [PowerSeries.coeff_one, h0, one_mul, Nat.add_sub_add_right] at hcoeff
  simp at hcoeff
  exact eq_neg_of_add_eq_zero_right hcoeff

/-- Subtracting two logarithmic-derivative coefficient convolutions gives the
relative X-coordinate formula used by the candidate. -/
theorem relativePowerSeriesDerivativeMulCoeff
    (fu gu fv gv : PowerSeries K) (n : ℕ) :
    PowerSeries.coeff n
        (PowerSeries.derivative fu * gu - PowerSeries.derivative fv * gv) =
      (∑ i ∈ Finset.range (n + 1),
          (PowerSeries.coeff (i + 1) fu * (i + 1)) *
            PowerSeries.coeff (n - i) gu) -
        ∑ i ∈ Finset.range (n + 1),
          (PowerSeries.coeff (i + 1) fv * (i + 1)) *
            PowerSeries.coeff (n - i) gv := by
  rw [map_sub, powerSeriesDerivativeMulCoeff, powerSeriesDerivativeMulCoeff]

/-- The concrete formal denominator used by the H116 native code:
`F_U(w)=1-Uw+h₁w²+h₂w³+⋯`. -/
def fiberDenominatorSeries (u : K) (h : ℕ → K) : PowerSeries K :=
  PowerSeries.mk fun
    | 0 => 1
    | 1 => -u
    | n + 2 => h (n + 1)

@[simp] theorem fiberDenominatorSeries_coeff_zero (u : K) (h : ℕ → K) :
    PowerSeries.coeff 0 (fiberDenominatorSeries u h) = 1 := by
  simp [fiberDenominatorSeries]

@[simp] theorem fiberDenominatorSeries_coeff_one (u : K) (h : ℕ → K) :
    PowerSeries.coeff 1 (fiberDenominatorSeries u h) = -u := by
  simp [fiberDenominatorSeries]

@[simp] theorem fiberDenominatorSeries_coeff_succ_succ
    (u : K) (h : ℕ → K) (n : ℕ) :
    PowerSeries.coeff (n + 2) (fiberDenominatorSeries u h) = h (n + 1) := by
  simp [fiberDenominatorSeries]

@[simp] theorem fiberDenominatorSeries_derivative_coeff_zero
    (u : K) (h : ℕ → K) :
    PowerSeries.coeff 0 (PowerSeries.derivative (fiberDenominatorSeries u h)) = -u := by
  rw [PowerSeries.coeff_derivative]
  simp

@[simp] theorem fiberDenominatorSeries_derivative_coeff_succ
    (u : K) (h : ℕ → K) (n : ℕ) :
    PowerSeries.coeff (n + 1) (PowerSeries.derivative (fiberDenominatorSeries u h)) =
      h (n + 1) * (n + 2) := by
  rw [PowerSeries.coeff_derivative]
  rw [fiberDenominatorSeries_coeff_succ_succ]
  have hcast : ((n + 1 : ℕ) : K) + 1 = ((n + 2 : ℕ) : K) := by
    push_cast
    ring
  have hcast' : ((n + 2 : ℕ) : K) = (n : K) + 2 := by
    push_cast
    ring
  rw [hcast, hcast']

/-- The reciprocal recurrence specialized to the concrete H116 denominator
series, with its unit constant coefficient discharged definitionally. -/
theorem fiberDenominatorReciprocalRecurrence
    (u : K) (h : ℕ → K) (g : PowerSeries K)
    (hfg : fiberDenominatorSeries u h * g = 1) (n : ℕ) :
    PowerSeries.coeff (n + 1) g =
      -∑ i ∈ Finset.range (n + 1),
        PowerSeries.coeff (i + 1) (fiberDenominatorSeries u h) *
          PowerSeries.coeff (n - i) g := by
  exact powerSeriesReciprocalRecurrence (fiberDenominatorSeries u h) g
    (fiberDenominatorSeries_coeff_zero u h) hfg n

/-- The shared normalized at-infinity series before subtracting the target
coordinate. Under the remaining geometric bridge it represents `S(x)/x` at
`w=1/x`. -/
def normalizedMapAtInfinitySeries (h : ℕ → K) : PowerSeries K :=
  fiberDenominatorSeries 0 h

/-- Formal-series form of `(S(x)-U)/x`: the target contributes only the
linear term `-U w`; every higher coefficient is shared. -/
theorem fiberDenominatorSeries_eq_normalizedMap_sub_linear
    (u : K) (h : ℕ → K) :
    fiberDenominatorSeries u h =
      normalizedMapAtInfinitySeries h - PowerSeries.C u * PowerSeries.X := by
  ext n
  rcases n with _ | _ | n
  · simp [fiberDenominatorSeries, normalizedMapAtInfinitySeries]
  · simp [fiberDenominatorSeries, normalizedMapAtInfinitySeries]
  · simp [fiberDenominatorSeries, normalizedMapAtInfinitySeries]

/-- Two target denominators with the same normalized map differ in exactly one
coefficient. This is the concrete shared-jet fact used for relative X traces. -/
theorem fiberDenominatorSeries_sub
    (u v : K) (h : ℕ → K) :
    fiberDenominatorSeries u h - fiberDenominatorSeries v h =
      PowerSeries.C (v - u) * PowerSeries.X := by
  rw [fiberDenominatorSeries_eq_normalizedMap_sub_linear,
    fiberDenominatorSeries_eq_normalizedMap_sub_linear]
  ext n
  rcases n with _ | _ | n
  · simp
  · simp
  · simp

/-- For prime characteristic `p > 19`, every denominator occurring while the
H116 producer constructs `h₁,…,h₇` from Bostan et al. recurrence (16) is
nonzero in `ZMod p`. The denominators above `19` are handled by their displayed
factorizations, so the theorem does not incorrectly infer that they are `< p`. -/
theorem bostanDenominatorsNonzero {p : ℕ} [Fact p.Prime] (hp : 19 < p) :
    (5 : ZMod p) ≠ 0 ∧ (7 : ZMod p) ≠ 0 ∧ (9 : ZMod p) ≠ 0 ∧
    (11 : ZMod p) ≠ 0 ∧ (13 : ZMod p) ≠ 0 ∧ (15 : ZMod p) ≠ 0 ∧
    (17 : ZMod p) ≠ 0 ∧ (22 : ZMod p) ≠ 0 ∧ (39 : ZMod p) ≠ 0 ∧
    (60 : ZMod p) ≠ 0 ∧ (85 : ZMod p) ≠ 0 := by
  have cast_ne (m : ℕ) (hm : 0 < m) (hmp : m < p) : (m : ZMod p) ≠ 0 := by
    intro hzero
    have hdvd : p ∣ m := (ZMod.natCast_eq_zero_iff m p).mp hzero
    exact (Nat.not_dvd_of_pos_of_lt hm hmp) hdvd
  have h2 : (2 : ZMod p) ≠ 0 := cast_ne 2 (by omega) (by omega)
  have h3 : (3 : ZMod p) ≠ 0 := cast_ne 3 (by omega) (by omega)
  have h4 : (4 : ZMod p) ≠ 0 := cast_ne 4 (by omega) (by omega)
  have h5 : (5 : ZMod p) ≠ 0 := cast_ne 5 (by omega) (by omega)
  have h7 : (7 : ZMod p) ≠ 0 := cast_ne 7 (by omega) (by omega)
  have h9 : (9 : ZMod p) ≠ 0 := cast_ne 9 (by omega) (by omega)
  have h11 : (11 : ZMod p) ≠ 0 := cast_ne 11 (by omega) (by omega)
  have h13 : (13 : ZMod p) ≠ 0 := cast_ne 13 (by omega) (by omega)
  have h15 : (15 : ZMod p) ≠ 0 := cast_ne 15 (by omega) (by omega)
  have h17 : (17 : ZMod p) ≠ 0 := cast_ne 17 (by omega) (by omega)
  have h22 : (22 : ZMod p) ≠ 0 := by
    rw [show (22 : ZMod p) = (2 : ZMod p) * 11 by norm_num]
    exact mul_ne_zero h2 h11
  have h39 : (39 : ZMod p) ≠ 0 := by
    rw [show (39 : ZMod p) = (3 : ZMod p) * 13 by norm_num]
    exact mul_ne_zero h3 h13
  have h60 : (60 : ZMod p) ≠ 0 := by
    rw [show (60 : ZMod p) = (4 : ZMod p) * 15 by norm_num]
    exact mul_ne_zero h4 h15
  have h85 : (85 : ZMod p) ≠ 0 := by
    rw [show (85 : ZMod p) = (5 : ZMod p) * 17 by norm_num]
    exact mul_ne_zero h5 h17
  exact ⟨h5, h7, h9, h11, h13, h15, h17, h22, h39, h60, h85⟩

#print axioms logDeriv_mul
#print axioms logDeriv_finset
#print axioms logDeriv_div
#print axioms denominator_cancellation
#print axioms rootProductLogDeriv
#print axioms rootPowerSumOfCoeff
#print axioms relativeCoeffCancellation
#print axioms residueCancelDenominator
#print axioms polynomialProductDerivative
#print axioms powerSeriesDerivativeMulCoeff
#print axioms powerSeriesReciprocalRecurrence
#print axioms relativePowerSeriesDerivativeMulCoeff
#print axioms fiberDenominatorSeries_coeff_zero
#print axioms fiberDenominatorSeries_coeff_one
#print axioms fiberDenominatorSeries_coeff_succ_succ
#print axioms fiberDenominatorSeries_derivative_coeff_zero
#print axioms fiberDenominatorSeries_derivative_coeff_succ
#print axioms fiberDenominatorReciprocalRecurrence
#print axioms fiberDenominatorSeries_eq_normalizedMap_sub_linear
#print axioms fiberDenominatorSeries_sub
#print axioms bostanDenominatorsNonzero

end H116FiberTrace
