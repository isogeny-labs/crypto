import Mathlib.Algebra.Polynomial.Derivation
import Mathlib.Data.ZMod.Basic
import Mathlib.FieldTheory.RatFunc.Basic
import Mathlib.RingTheory.LaurentSeries
import Mathlib.RingTheory.PowerSeries.Derivative
import Mathlib.RingTheory.PowerSeries.Substitution
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

/-- Differentiating equation (14) of Bostan--Morain--Salvy--Schost and
cancelling the nonzero derivative of a separable normalized coordinate map
gives their equation (15). All curve coefficients are represented by explicit
constants for the supplied derivation. -/
theorem normalizedMapEquation_implies_secondOrder
    (D : Derivation K L L) (x R sourceA sourceB targetA targetB : L)
    (hx : D x = 1)
    (hsourceA : D sourceA = 0) (hsourceB : D sourceB = 0)
    (htargetA : D targetA = 0) (htargetB : D targetB = 0)
    (hseparable : D R ≠ 0)
    (hcurve : (x ^ 3 + sourceA * x + sourceB) * (D R) ^ 2 =
      R ^ 3 + targetA * R + targetB) :
    (3 * x ^ 2 + sourceA) * D R +
        2 * (x ^ 3 + sourceA * x + sourceB) * D (D R) =
      3 * R ^ 2 + targetA := by
  have hd := congrArg D hcurve
  simp only [D.map_add, D.leibniz, D.leibniz_pow, hx, hsourceA,
    hsourceB, htargetA, htargetB, smul_eq_mul, nsmul_eq_mul,
    mul_zero, add_zero, mul_one] at hd
  apply (mul_right_cancel₀ hseparable)
  linear_combination hd

/-- The first-order normalized-map equation after `R(x)=xU(w)`, `w=1/x`,
implies the correspondingly transformed second-order equation. Differentiating
the first equation and combining it with three copies of the original leaves
the desired equation times `U-wU'`; the explicit nonzero hypothesis permits
the final cancellation. -/
theorem transformedFirstOrderEquation_implies_secondOrder
    {A : Type*} [CommRing A] [IsDomain A] [Algebra K A]
    (D : Derivation K A A)
    (w U Up Udd sourceA sourceB targetA targetB : A)
    (hw : D w = 1) (hU : D U = Up) (hUp : D Up = Udd)
    (hsourceA : D sourceA = 0) (hsourceB : D sourceB = 0)
    (htargetA : D targetA = 0) (htargetB : D targetB = 0)
    (hseparable : U - w * Up ≠ 0)
    (hcurve : (1 + sourceA * w ^ 2 + sourceB * w ^ 3) *
        (U - w * Up) ^ 2 =
      U ^ 3 + targetA * w ^ 2 * U + targetB * w ^ 3) :
    (3 + sourceA * w ^ 2) * (U - w * Up) +
        2 * w ^ 2 * (1 + sourceA * w ^ 2 + sourceB * w ^ 3) * Udd =
      3 * U ^ 2 + targetA * w ^ 2 := by
  have hd := congrArg D hcurve
  simp only [D.map_add, D.map_sub, D.leibniz, D.leibniz_pow,
    D.map_one_eq_zero, hw, hU, hUp,
    hsourceA, hsourceB, htargetA, htargetB, smul_eq_mul, nsmul_eq_mul,
    mul_zero, add_zero, mul_one] at hd
  apply (mul_right_cancel₀ hseparable)
  linear_combination 3 * hcurve - w * hd

/-- A cleared-denominator certificate for a quotient `U=N/D` implies the
transformed first-order curve equation. The auxiliary numerator
`D(N-wN')+wD'N` is exactly `D²(U-wU')`; clearing the target equation therefore
costs the visible factor `D⁴`. -/
theorem quotientCertificate_implies_firstOrderEquation
    {A : Type*} [CommRing A] [IsDomain A] [Algebra K A]
    (w U Up N Np D Dp sourceA sourceB targetA targetB : A)
    (hD : D ≠ 0)
    (hquot : D * U = N)
    (hderiv : Dp * U + D * Up = Np)
    (hcert :
      (1 + sourceA * w ^ 2 + sourceB * w ^ 3) *
          (D * (N - w * Np) + w * Dp * N) ^ 2 =
        D * N ^ 3 + targetA * w ^ 2 * N * D ^ 3 +
          targetB * w ^ 3 * D ^ 4) :
    (1 + sourceA * w ^ 2 + sourceB * w ^ 3) *
        (U - w * Up) ^ 2 =
      U ^ 3 + targetA * w ^ 2 * U + targetB * w ^ 3 := by
  have hH : D * (N - w * Np) + w * Dp * N = D ^ 2 * (U - w * Up) := by
    rw [← hquot, ← hderiv]
    ring
  apply (mul_left_cancel₀ (pow_ne_zero 4 hD))
  calc
    D ^ 4 * ((1 + sourceA * w ^ 2 + sourceB * w ^ 3) *
        (U - w * Up) ^ 2) =
        (1 + sourceA * w ^ 2 + sourceB * w ^ 3) *
          (D ^ 2 * (U - w * Up)) ^ 2 := by ring
    _ = (1 + sourceA * w ^ 2 + sourceB * w ^ 3) *
          (D * (N - w * Np) + w * Dp * N) ^ 2 := by rw [hH]
    _ = D * N ^ 3 + targetA * w ^ 2 * N * D ^ 3 +
          targetB * w ^ 3 * D ^ 4 := hcert
    _ = D ^ 4 * (U ^ 3 + targetA * w ^ 2 * U + targetB * w ^ 3) := by
      rw [← hquot]
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

/-- Exact finite Bostan recurrence instantiated by the H116 native producer. -/
def BostanJetRecurrence (sourceA sourceB targetA targetB : K) (h : ℕ → K) : Prop :=
  h 1 = (sourceA - targetA) / 5 ∧
  h 2 = (sourceB - targetB) / 7 ∧
  ∀ k, 3 ≤ k → k ≤ 7 →
    h k =
      3 * (∑ i ∈ Finset.Icc 1 (k - 2), h i * h (k - 1 - i)) /
          (((k - 2) * (2 * k + 3) : ℕ) : K) -
        (((2 * k - 3 : ℕ) : K) * sourceA * h (k - 2)) /
          ((2 * k + 3 : ℕ) : K) -
        (2 * ((k - 3 : ℕ) : K) * sourceB * h (k - 3)) /
          ((2 * k + 3 : ℕ) : K)

/-- Equality of power series below degree `n`. This is a finite congruence:
`PowerSeries.trunc n` keeps exactly the coefficients of degrees `< n`. -/
def PowerSeriesPrefixEq {R : Type*} [Semiring R]
    (n : ℕ) (f g : PowerSeries R) : Prop :=
  PowerSeries.trunc n f = PowerSeries.trunc n g

namespace PowerSeriesPrefixEq

variable {R : Type*} [Field R]
variable {n m : ℕ} {f g h k : PowerSeries R}

theorem refl (f : PowerSeries R) : PowerSeriesPrefixEq n f f := rfl

theorem of_eq (hfg : f = g) : PowerSeriesPrefixEq n f g := by
  subst g
  exact refl f

theorem symm (hfg : PowerSeriesPrefixEq n f g) : PowerSeriesPrefixEq n g f :=
  Eq.symm hfg

theorem trans (hfg : PowerSeriesPrefixEq n f g)
    (hgh : PowerSeriesPrefixEq n g h) : PowerSeriesPrefixEq n f h :=
  Eq.trans hfg hgh

theorem mono (hmn : m ≤ n) (hfg : PowerSeriesPrefixEq n f g) :
    PowerSeriesPrefixEq m f g := by
  unfold PowerSeriesPrefixEq at hfg ⊢
  calc
    PowerSeries.trunc m f =
        PowerSeries.trunc m (PowerSeries.trunc n f : PowerSeries R) :=
      (PowerSeries.trunc_trunc_of_le f hmn).symm
    _ = PowerSeries.trunc m (PowerSeries.trunc n g : PowerSeries R) := by rw [hfg]
    _ = PowerSeries.trunc m g := PowerSeries.trunc_trunc_of_le g hmn

theorem add (hfg : PowerSeriesPrefixEq n f g)
    (hhk : PowerSeriesPrefixEq n h k) :
    PowerSeriesPrefixEq n (f + h) (g + k) := by
  unfold PowerSeriesPrefixEq at hfg hhk ⊢
  simpa only [map_add] using congrArg₂ (· + ·) hfg hhk

theorem neg (hfg : PowerSeriesPrefixEq n f g) :
    PowerSeriesPrefixEq n (-f) (-g) := by
  unfold PowerSeriesPrefixEq at hfg ⊢
  simpa only [map_neg] using congrArg Neg.neg hfg

theorem sub (hfg : PowerSeriesPrefixEq n f g)
    (hhk : PowerSeriesPrefixEq n h k) :
    PowerSeriesPrefixEq n (f - h) (g - k) := by
  simpa only [sub_eq_add_neg] using add hfg (neg hhk)

theorem mul (hfg : PowerSeriesPrefixEq n f g)
    (hhk : PowerSeriesPrefixEq n h k) :
    PowerSeriesPrefixEq n (f * h) (g * k) := by
  unfold PowerSeriesPrefixEq at hfg hhk ⊢
  calc
    PowerSeries.trunc n (f * h) =
        PowerSeries.trunc n
          ((PowerSeries.trunc n f : PowerSeries R) *
            (PowerSeries.trunc n h : PowerSeries R)) :=
      (PowerSeries.trunc_trunc_mul_trunc f h).symm
    _ = PowerSeries.trunc n
          ((PowerSeries.trunc n g : PowerSeries R) *
            (PowerSeries.trunc n k : PowerSeries R)) := by rw [hfg, hhk]
    _ = PowerSeries.trunc n (g * k) :=
      PowerSeries.trunc_trunc_mul_trunc g k

theorem pow (hfg : PowerSeriesPrefixEq n f g) (a : ℕ) :
    PowerSeriesPrefixEq n (f ^ a) (g ^ a) := by
  unfold PowerSeriesPrefixEq at hfg ⊢
  calc
    PowerSeries.trunc n (f ^ a) =
        PowerSeries.trunc n ((PowerSeries.trunc n f : PowerSeries R) ^ a) :=
      (PowerSeries.trunc_trunc_pow f n a).symm
    _ = PowerSeries.trunc n ((PowerSeries.trunc n g : PowerSeries R) ^ a) := by
      rw [hfg]
    _ = PowerSeries.trunc n (g ^ a) := PowerSeries.trunc_trunc_pow g n a

/-- Differentiating a known prefix consumes exactly one coefficient. -/
theorem derivative (hfg : PowerSeriesPrefixEq (n + 1) f g) :
    PowerSeriesPrefixEq n (PowerSeries.derivative f) (PowerSeries.derivative g) := by
  unfold PowerSeriesPrefixEq at hfg ⊢
  rw [PowerSeries.trunc_derivative, PowerSeries.trunc_derivative]
  exact congrArg Polynomial.derivative hfg

theorem coeff_eq (hfg : PowerSeriesPrefixEq n f g) {i : ℕ} (hi : i < n) :
    PowerSeries.coeff i f = PowerSeries.coeff i g := by
  unfold PowerSeriesPrefixEq at hfg
  calc
    PowerSeries.coeff i f =
        PowerSeries.coeff i (PowerSeries.trunc n f) :=
      (PowerSeries.coeff_coe_trunc_of_lt hi).symm
    _ = PowerSeries.coeff i (PowerSeries.trunc n g) := by rw [hfg]
    _ = PowerSeries.coeff i g := PowerSeries.coeff_coe_trunc_of_lt hi

/-- Coefficientwise equality below the cutoff reconstructs prefix equality. -/
theorem of_coeff_eq
    (hcoeff : ∀ i, i < n → PowerSeries.coeff i f = PowerSeries.coeff i g) :
    PowerSeriesPrefixEq n f g := by
  unfold PowerSeriesPrefixEq
  ext i
  simp only [PowerSeries.coeff_trunc]
  split_ifs with hi
  · exact hcoeff i hi
  · rfl

/-- Cancelling a shared leading local parameter consumes exactly one degree of
prefix precision. Unlike `cancel_left`, this applies to the nonunit `X`. -/
theorem cancel_X
    (hfg : PowerSeriesPrefixEq (n + 1)
      (PowerSeries.X * f) (PowerSeries.X * g)) :
    PowerSeriesPrefixEq n f g := by
  apply of_coeff_eq
  intro i hi
  have hc := coeff_eq hfg (i := i + 1) (by omega)
  rw [← pow_one (PowerSeries.X : PowerSeries R)] at hc
  simpa only [PowerSeries.coeff_X_pow_mul] using hc

/-- A power-series unit can be cancelled without losing prefix precision. -/
theorem cancel_left (D : PowerSeries R)
    (hD : PowerSeries.constantCoeff D ≠ 0)
    (hfg : PowerSeriesPrefixEq n (D * f) (D * g)) :
    PowerSeriesPrefixEq n f g := by
  have hi := mul (refl (D⁻¹)) hfg
  simpa only [← mul_assoc, PowerSeries.inv_mul_cancel D hD, one_mul] using hi

/-- Inversion of unit power series preserves every established prefix. -/
theorem inv (hfg : PowerSeriesPrefixEq n f g)
    (hf : PowerSeries.constantCoeff f ≠ 0)
    (hg : PowerSeries.constantCoeff g ≠ 0) :
    PowerSeriesPrefixEq n f⁻¹ g⁻¹ := by
  have hcross := mul (mul (refl (f⁻¹)) hfg) (refl (g⁻¹))
  have hreverse : PowerSeriesPrefixEq n g⁻¹ f⁻¹ := by
    simpa only [← mul_assoc, PowerSeries.inv_mul_cancel f hf, one_mul,
      mul_assoc, PowerSeries.mul_inv_cancel g hg, mul_one] using hcross
  exact symm hreverse

/-- The inverse square used by a double-pole coordinate preserves a unit-series
prefix. -/
theorem inv_sq (hfg : PowerSeriesPrefixEq n f g)
    (hf : PowerSeries.constantCoeff f ≠ 0)
    (hg : PowerSeries.constantCoeff g ≠ 0) :
    PowerSeriesPrefixEq n (f⁻¹ ^ 2) (g⁻¹ ^ 2) :=
  pow (inv hfg hf hg) 2

/-- A monomial contributes nothing below its own degree. This is the exact
Mathlib truncation theorem, packaged in the prefix relation used by H116. -/
theorem X_pow_eq_zero (m : ℕ) :
    PowerSeriesPrefixEq m ((PowerSeries.X : PowerSeries R) ^ m) 0 := by
  unfold PowerSeriesPrefixEq
  simpa using PowerSeries.trunc_X_pow_self_mul m (1 : PowerSeries R)

/-- Adding a correction of order at least `m` does not change the prefix below
degree `m`. -/
theorem add_X_pow_mul (f correction : PowerSeries R) (m : ℕ) :
    PowerSeriesPrefixEq m
      (f + PowerSeries.X ^ m * correction) f := by
  simpa using add (refl f)
    (mul (X_pow_eq_zero m) (refl correction))

/-- Generic double-pole core of the pole-transfer bridge. Agreement of two
local parameters through degree `n` after a shared leading `X` yields agreement
of their normalized inverse-square terms through degree `n-1`. The regular
unit factor in the elliptic x-coordinate remains a separate composition
obligation. -/
theorem leading_X_inv_sq
    (hfg : PowerSeriesPrefixEq (n + 1)
      (PowerSeries.X * f) (PowerSeries.X * g))
    (hf : PowerSeries.constantCoeff f ≠ 0)
    (hg : PowerSeries.constantCoeff g ≠ 0) :
    PowerSeriesPrefixEq n (f⁻¹ ^ 2) (g⁻¹ ^ 2) :=
  inv_sq (cancel_X hfg) hf hg

/-- Substitution of prefix-equal zero-constant parameters into a fixed regular
power series preserves the prefix. -/
theorem subst (outer : PowerSeries R)
    (hfg : PowerSeriesPrefixEq n f g)
    (hf : PowerSeries.constantCoeff f = 0)
    (hg : PowerSeries.constantCoeff g = 0) :
    PowerSeriesPrefixEq n (outer.subst f) (outer.subst g) := by
  apply of_coeff_eq
  intro i hi
  rw [PowerSeries.coeff_subst'
      (PowerSeries.HasSubst.of_constantCoeff_zero' hf) outer i,
    PowerSeries.coeff_subst'
      (PowerSeries.HasSubst.of_constantCoeff_zero' hg) outer i]
  apply finsum_congr
  intro d
  rw [coeff_eq (pow hfg d) hi]

/-- The regular unit factor of a pole coordinate also survives substitution of
prefix-equal zero-constant parameters. -/
theorem subst_inv (outer : PowerSeries R)
    (hfg : PowerSeriesPrefixEq n f g)
    (hf : PowerSeries.constantCoeff f = 0)
    (hg : PowerSeries.constantCoeff g = 0)
    (houter : PowerSeries.constantCoeff outer ≠ 0) :
    PowerSeriesPrefixEq n (outer.subst f)⁻¹ (outer.subst g)⁻¹ := by
  have hsubst := subst outer hfg hf hg
  have hunit_f : PowerSeries.constantCoeff (outer.subst f) ≠ 0 := by
    change MvPowerSeries.constantCoeff (outer.subst f) ≠ 0
    rw [PowerSeries.constantCoeff_subst_of_constantCoeff_zero hf outer]
    simpa using houter
  have hunit_g : PowerSeries.constantCoeff (outer.subst g) ≠ 0 := by
    change MvPowerSeries.constantCoeff (outer.subst g) ≠ 0
    rw [PowerSeries.constantCoeff_subst_of_constantCoeff_zero hg outer]
    simpa using houter
  exact inv hsubst hunit_f hunit_g

/-- Generic normalized pole-coordinate transfer. After cancelling a common
leading `X`, both the inverse-square pole and the inverse of any fixed regular
unit factor preserve the remaining prefix. -/
theorem leading_X_inv_sq_mul_subst_inv
    (outer : PowerSeries R)
    (hfg : PowerSeriesPrefixEq (n + 1)
      (PowerSeries.X * f) (PowerSeries.X * g))
    (hf : PowerSeries.constantCoeff f ≠ 0)
    (hg : PowerSeries.constantCoeff g ≠ 0)
    (houter : PowerSeries.constantCoeff outer ≠ 0) :
    PowerSeriesPrefixEq n
      (f⁻¹ ^ 2 * (outer.subst (PowerSeries.X * f))⁻¹)
      (g⁻¹ ^ 2 * (outer.subst (PowerSeries.X * g))⁻¹) := by
  apply mul (leading_X_inv_sq hfg hf hg)
  apply subst_inv outer (mono (by omega) hfg) <;> simp_all

end PowerSeriesPrefixEq

/-- The regular unit in the short-Weierstrass local coordinate
`x(z)=z⁻² q(z)⁻¹`. Clearing the curve equation gives this fixed-point identity. -/
def WeierstrassCoordinateUnit
    (curveA curveB : K) (q : PowerSeries K) : Prop :=
  q = 1 + PowerSeries.C curveA * PowerSeries.X ^ 4 * q ^ 2 +
    PowerSeries.C curveB * PowerSeries.X ^ 6 * q ^ 3

/-- Every solution of the coordinate-unit equation has unit constant
coefficient. -/
theorem WeierstrassCoordinateUnit.constantCoeff
    {curveA curveB : K} {q : PowerSeries K}
    (hq : WeierstrassCoordinateUnit curveA curveB q) :
    PowerSeries.constantCoeff q = 1 := by
  unfold WeierstrassCoordinateUnit at hq
  have hc := congrArg PowerSeries.constantCoeff hq
  simpa using hc

/-- The coordinate-unit equation is exactly the regularized affine curve
identity obtained after substituting `x=z⁻²q⁻¹` and clearing the double pole. -/
theorem WeierstrassCoordinateUnit.clearedCurveIdentity
    {curveA curveB : K} {q : PowerSeries K}
    (hq : WeierstrassCoordinateUnit curveA curveB q) :
    q⁻¹ + PowerSeries.C curveA * PowerSeries.X ^ 4 * q +
        PowerSeries.C curveB * PowerSeries.X ^ 6 * q ^ 2 = 1 := by
  have hconst : PowerSeries.constantCoeff q ≠ 0 := by
    rw [hq.constantCoeff]
    exact one_ne_zero
  have hq_ne : q ≠ 0 := by
    intro hzero
    rw [hzero] at hconst
    exact hconst rfl
  apply mul_left_cancel₀ hq_ne
  rw [mul_one]
  calc
    q * (q⁻¹ + PowerSeries.C curveA * PowerSeries.X ^ 4 * q +
        PowerSeries.C curveB * PowerSeries.X ^ 6 * q ^ 2) =
        1 + PowerSeries.C curveA * PowerSeries.X ^ 4 * q ^ 2 +
          PowerSeries.C curveB * PowerSeries.X ^ 6 * q ^ 3 := by
      rw [mul_add, mul_add, PowerSeries.mul_inv_cancel q hconst]
      ring
    _ = q := hq.symm

/-- Scaling `(x,y)` by `(scale²,scale³)` sends the source coordinate unit to
the target coordinate unit by the local-parameter substitution `z ↦ scale*z`. -/
theorem WeierstrassCoordinateUnit.rescale
    {curveA curveB : K} {q : PowerSeries K}
    (hq : WeierstrassCoordinateUnit curveA curveB q) (scale : K) :
    WeierstrassCoordinateUnit (scale ^ 4 * curveA) (scale ^ 6 * curveB)
      (q.subst (PowerSeries.C scale * PowerSeries.X)) := by
  let s : PowerSeries K := PowerSeries.C scale * PowerSeries.X
  have hs0 : PowerSeries.constantCoeff s = 0 := by
    simp [s]
  have hs := PowerSeries.HasSubst.of_constantCoeff_zero' hs0
  unfold WeierstrassCoordinateUnit at hq ⊢
  have h := congrArg (fun u : PowerSeries K => u.subst s) hq
  simp only [PowerSeries.subst_add hs, PowerSeries.subst_mul hs,
    PowerSeries.subst_pow hs, PowerSeries.subst_C, PowerSeries.subst_X hs,
    ] at h
  have h_one : (1 : PowerSeries K).subst s = 1 := by
    rw [← PowerSeries.coe_substAlgHom hs]
    exact map_one (PowerSeries.substAlgHom hs)
  have h' : q.subst s =
      1 + PowerSeries.C curveA * s ^ 4 * (q.subst s) ^ 2 +
        PowerSeries.C curveB * s ^ 6 * (q.subst s) ^ 3 := by
    rw [h_one] at h
    rw [← PowerSeries.C_apply, ← PowerSeries.C_apply] at h
    exact h
  dsimp only [s] at h ⊢
  calc
    q.subst (PowerSeries.C scale * PowerSeries.X) =
        1 + PowerSeries.C curveA *
            (PowerSeries.C scale * PowerSeries.X) ^ 4 *
              (q.subst (PowerSeries.C scale * PowerSeries.X)) ^ 2 +
          PowerSeries.C curveB *
            (PowerSeries.C scale * PowerSeries.X) ^ 6 *
              (q.subst (PowerSeries.C scale * PowerSeries.X)) ^ 3 := by
      simpa [s] using h'
    _ = 1 + PowerSeries.C (scale ^ 4 * curveA) * PowerSeries.X ^ 4 *
            (q.subst (PowerSeries.C scale * PowerSeries.X)) ^ 2 +
          PowerSeries.C (scale ^ 6 * curveB) * PowerSeries.X ^ 6 *
            (q.subst (PowerSeries.C scale * PowerSeries.X)) ^ 3 := by
      simp only [map_mul, map_pow]
      ring

/-- Substituting a scaled variable and then a zero-constant local parameter is
the same as substituting their product once. -/
theorem powerSeries_subst_scale
    (q : PowerSeries K) (scale : K) {z : PowerSeries K}
    (hz : PowerSeries.constantCoeff z = 0) :
    PowerSeries.subst z
        (PowerSeries.subst (PowerSeries.C scale * PowerSeries.X) q) =
      PowerSeries.subst (PowerSeries.C scale * z) q := by
  let s : PowerSeries K := PowerSeries.C scale * PowerSeries.X
  have hs0 : PowerSeries.constantCoeff s = 0 := by simp [s]
  have hs := PowerSeries.HasSubst.of_constantCoeff_zero' hs0
  have hzSubst := PowerSeries.HasSubst.of_constantCoeff_zero' hz
  rw [PowerSeries.subst_comp_subst_apply hs hzSubst]
  congr 1
  dsimp only [s]
  rw [PowerSeries.subst_mul hzSubst, PowerSeries.subst_C,
    PowerSeries.subst_X hzSubst, ← PowerSeries.C_apply]

/-- Exact target-scaling substitution used by the formal-scalar constructor.
If `scale*zNormalized=zScalar`, the rescaled target coordinate unit evaluated
at `zNormalized` is the source coordinate unit evaluated at `zScalar`. -/
theorem WeierstrassCoordinateUnit.rescaled_subst_normalized
    {curveA curveB : K} {q zNormalized zScalar : PowerSeries K}
    (hq : WeierstrassCoordinateUnit curveA curveB q)
    (scale : K) (hz : PowerSeries.constantCoeff zNormalized = 0)
    (hscale : PowerSeries.C scale * zNormalized = zScalar) :
    WeierstrassCoordinateUnit (scale ^ 4 * curveA) (scale ^ 6 * curveB)
        (q.subst (PowerSeries.C scale * PowerSeries.X)) ∧
      PowerSeries.subst zNormalized
          (PowerSeries.subst (PowerSeries.C scale * PowerSeries.X) q) =
        PowerSeries.subst zScalar q := by
  refine ⟨hq.rescale scale, ?_⟩
  rw [powerSeries_subst_scale q scale hz, hscale]

/-- The regularized normalized x-coordinate after writing a zero-constant
local parameter as `X*f`. -/
noncomputable def normalizedWeierstrassXUnit
    (q f : PowerSeries K) : PowerSeries K :=
  f⁻¹ ^ 2 * (q.subst (PowerSeries.X * f))⁻¹

/-- The complete normalized x-ratio unit, including the common source-curve
factor used by the implementation. -/
noncomputable def normalizedWeierstrassXRatio
    (qSource qTarget f : PowerSeries K) : PowerSeries K :=
  qSource * normalizedWeierstrassXUnit qTarget f

/-- The actual short-Weierstrass coordinate unit instantiates the generic
leading-parameter and regular-substitution prefix bridge. -/
theorem WeierstrassCoordinateUnit.normalizedX_prefix
    {curveA curveB : K} {q f g : PowerSeries K} {n : ℕ}
    (hq : WeierstrassCoordinateUnit curveA curveB q)
    (hfg : PowerSeriesPrefixEq (n + 1)
      (PowerSeries.X * f) (PowerSeries.X * g))
    (hf : PowerSeries.constantCoeff f ≠ 0)
    (hg : PowerSeries.constantCoeff g ≠ 0) :
    PowerSeriesPrefixEq n
      (normalizedWeierstrassXUnit q f)
      (normalizedWeierstrassXUnit q g) := by
  apply PowerSeriesPrefixEq.leading_X_inv_sq_mul_subst_inv q hfg hf hg
  rw [hq.constantCoeff]
  exact one_ne_zero

/-- Multiplying by the fixed source-coordinate unit preserves the checked
target-coordinate prefix. -/
theorem WeierstrassCoordinateUnit.normalizedXRatio_prefix
    {sourceA sourceB targetA targetB : K}
    {qSource qTarget f g : PowerSeries K} {n : ℕ}
    (_hqSource : WeierstrassCoordinateUnit sourceA sourceB qSource)
    (hqTarget : WeierstrassCoordinateUnit targetA targetB qTarget)
    (hfg : PowerSeriesPrefixEq (n + 1)
      (PowerSeries.X * f) (PowerSeries.X * g))
    (hf : PowerSeries.constantCoeff f ≠ 0)
    (hg : PowerSeries.constantCoeff g ≠ 0) :
    PowerSeriesPrefixEq n
      (normalizedWeierstrassXRatio qSource qTarget f)
      (normalizedWeierstrassXRatio qSource qTarget g) := by
  apply PowerSeriesPrefixEq.mul (PowerSeriesPrefixEq.refl qSource)
  exact hqTarget.normalizedX_prefix hfg hf hg

/-- A prime strictly above 19 is at least 23. Keeping primality explicit is
necessary: the numerical inequality alone would admit 20, 21 and 22. -/
theorem prime_ge_twentyThree {p : ℕ} [Fact p.Prime] (hp : 19 < p) : 23 ≤ p := by
  have hprime : p.Prime := Fact.out
  by_contra hnot
  have hp22 : p ≤ 22 := by omega
  interval_cases p <;> norm_num at hp hprime

/-- Once the leading local-parameter factor has been removed, a `z^p`
Frobenius correction begins with `z^(p-1)`. For prime `p>19`, that normalized
correction is invisible through every degree strictly below 22. This theorem
formalizes only the precision cutoff; identifying an elliptic Frobenius term
with this monomial is a separate endpoint obligation. -/
theorem frobeniusNormalizedCorrection_invisible
    {p : ℕ} [Fact p.Prime] (hp : 19 < p) :
    PowerSeriesPrefixEq 22
      ((PowerSeries.X : PowerSeries (ZMod p)) ^ (p - 1)) 0 := by
  apply PowerSeriesPrefixEq.mono (m := 22) (n := p - 1)
  · have hp23 := prime_ge_twentyThree hp
    omega
  · exact PowerSeriesPrefixEq.X_pow_eq_zero (p - 1)

/-- In particular, the even `z`-degrees corresponding to `w^0,...,w^10`
cannot contain the normalized Frobenius correction in the frozen prime range. -/
theorem frobeniusNormalizedCorrection_even_coeff_zero
    {p : ℕ} [Fact p.Prime] (hp : 19 < p) (n : ℕ) (hn : n ≤ 10) :
    PowerSeries.coeff (2 * n)
      ((PowerSeries.X : PowerSeries (ZMod p)) ^ (p - 1)) = 0 := by
  rw [PowerSeries.coeff_X_pow]
  have hp23 := prime_ge_twentyThree hp
  split_ifs with heq
  · omega
  · rfl

/-- Equation (14) after substituting `R(x)=xU(w)`, `w=1/x`, and multiplying
by `w³`. The factor `W=U-wU'` is the transformed derivative `R'(x)`. -/
def NormalizedIsogenyFirstOrderEquation
    (sourceA sourceB targetA targetB : K) (h : ℕ → K) : Prop :=
  let U := normalizedMapAtInfinitySeries h
  let W := U - PowerSeries.X * PowerSeries.derivative U
  (1 + PowerSeries.C sourceA * PowerSeries.X ^ 2 +
      PowerSeries.C sourceB * PowerSeries.X ^ 3) * W ^ 2 =
    U ^ 3 + PowerSeries.C targetA * PowerSeries.X ^ 2 * U +
      PowerSeries.C targetB * PowerSeries.X ^ 3

/-- Checkable cleared-denominator certificate for a normalized rational map at
infinity. `N` and `D` are the reversed numerator and denominator series. The
constant coefficient condition both fixes the denominator normalization and
proves that it is nonzero. -/
def NormalizedRationalMapSeriesCertificate
    (sourceA sourceB targetA targetB : K) (h : ℕ → K)
    (N D : PowerSeries K) : Prop :=
  let U := normalizedMapAtInfinitySeries h
  let Np := PowerSeries.derivative N
  let Dp := PowerSeries.derivative D
  let H := D * (N - PowerSeries.X * Np) + PowerSeries.X * Dp * N
  PowerSeries.constantCoeff D = 1 ∧
    D * U = N ∧
    (1 + PowerSeries.C sourceA * PowerSeries.X ^ 2 +
        PowerSeries.C sourceB * PowerSeries.X ^ 3) * H ^ 2 =
      D * N ^ 3 + PowerSeries.C targetA * PowerSeries.X ^ 2 * N * D ^ 3 +
        PowerSeries.C targetB * PowerSeries.X ^ 3 * D ^ 4

/-- Finite polynomial wrapper for the series certificate. Coercion into
PowerSeries preserves multiplication and formal differentiation, so a producer
can emit ordinary reversed numerator and denominator polynomials. -/
def NormalizedRationalMapPolynomialCertificate
    (sourceA sourceB targetA targetB : K) (h : ℕ → K)
    (N D : Polynomial K) : Prop :=
  NormalizedRationalMapSeriesCertificate sourceA sourceB targetA targetB h N D

/-- A valid reversed rational-map certificate supplies transformed equation
(14), with quotient differentiation and every denominator cancellation proved
inside Lean. -/
theorem NormalizedRationalMapSeriesCertificate.to_firstOrder
    (sourceA sourceB targetA targetB : K) (h : ℕ → K)
    (N D : PowerSeries K)
    (hcert : NormalizedRationalMapSeriesCertificate
      sourceA sourceB targetA targetB h N D) :
    NormalizedIsogenyFirstOrderEquation sourceA sourceB targetA targetB h := by
  let U := normalizedMapAtInfinitySeries h
  let Up := PowerSeries.derivative U
  change PowerSeries.constantCoeff D = 1 ∧ D * U = N ∧ _ at hcert
  rcases hcert with ⟨hconst, hquot, hpoly⟩
  have hD : D ≠ 0 := by
    intro hzero
    rw [hzero] at hconst
    exact zero_ne_one hconst
  have hderiv := congrArg PowerSeries.derivative hquot
  simp only [PowerSeries.derivative.leibniz, smul_eq_mul] at hderiv
  have hfirst := quotientCertificate_implies_firstOrderEquation
    (K := K) (A := PowerSeries K)
    PowerSeries.X U Up N (PowerSeries.derivative N) D
      (PowerSeries.derivative D) (PowerSeries.C sourceA)
      (PowerSeries.C sourceB) (PowerSeries.C targetA)
      (PowerSeries.C targetB) hD hquot
      (by simpa [Up, add_comm, mul_comm] using hderiv) hpoly
  simpa [NormalizedIsogenyFirstOrderEquation, U, Up] using hfirst

/-- Finite polynomial certificates are accepted by the same checked bridge. -/
theorem NormalizedRationalMapPolynomialCertificate.to_firstOrder
    (sourceA sourceB targetA targetB : K) (h : ℕ → K)
    (N D : Polynomial K)
    (hcert : NormalizedRationalMapPolynomialCertificate
      sourceA sourceB targetA targetB h N D) :
    NormalizedIsogenyFirstOrderEquation sourceA sourceB targetA targetB h := by
  exact NormalizedRationalMapSeriesCertificate.to_firstOrder
    sourceA sourceB targetA targetB h N D hcert

/-- The first-order normalized curve equation, only below degree ten. -/
def NormalizedIsogenyFirstOrderPrefix
    (sourceA sourceB targetA targetB : K) (h : ℕ → K) : Prop :=
  let U := normalizedMapAtInfinitySeries h
  let W := U - PowerSeries.X * PowerSeries.derivative U
  PowerSeriesPrefixEq 10
    ((1 + PowerSeries.C sourceA * PowerSeries.X ^ 2 +
        PowerSeries.C sourceB * PowerSeries.X ^ 3) * W ^ 2)
    (U ^ 3 + PowerSeries.C targetA * PowerSeries.X ^ 2 * U +
      PowerSeries.C targetB * PowerSeries.X ^ 3)

/-- A fixed-size rational-map certificate. Eleven quotient coefficients are
needed because quotient differentiation consumes one; the cleared curve
identity itself is checked only below degree ten. -/
def NormalizedRationalMapJetCertificate
    (sourceA sourceB targetA targetB : K) (h : ℕ → K)
    (N D : PowerSeries K) : Prop :=
  let U := normalizedMapAtInfinitySeries h
  let Np := PowerSeries.derivative N
  let Dp := PowerSeries.derivative D
  let H := D * (N - PowerSeries.X * Np) + PowerSeries.X * Dp * N
  PowerSeries.constantCoeff D = 1 ∧
    PowerSeriesPrefixEq 11 (D * U) N ∧
    PowerSeriesPrefixEq 10
      ((1 + PowerSeries.C sourceA * PowerSeries.X ^ 2 +
          PowerSeries.C sourceB * PowerSeries.X ^ 3) * H ^ 2)
      (D * N ^ 3 + PowerSeries.C targetA * PowerSeries.X ^ 2 * N * D ^ 3 +
        PowerSeries.C targetB * PowerSeries.X ^ 3 * D ^ 4)

/-- The eleven-coefficient quotient certificate implies the transformed curve
equation through degree nine. Every substitution and cancellation occurs in
the checked truncated power-series congruence. -/
theorem NormalizedRationalMapJetCertificate.to_firstOrderPrefix
    (sourceA sourceB targetA targetB : K) (h : ℕ → K)
    (N D : PowerSeries K)
    (hcert : NormalizedRationalMapJetCertificate
      sourceA sourceB targetA targetB h N D) :
    NormalizedIsogenyFirstOrderPrefix sourceA sourceB targetA targetB h := by
  let U := normalizedMapAtInfinitySeries h
  let Up := PowerSeries.derivative U
  let Np := PowerSeries.derivative N
  let Dp := PowerSeries.derivative D
  let X : PowerSeries K := PowerSeries.X
  let W := U - X * Up
  let F := 1 + PowerSeries.C sourceA * X ^ 2 + PowerSeries.C sourceB * X ^ 3
  let G := U ^ 3 + PowerSeries.C targetA * X ^ 2 * U +
    PowerSeries.C targetB * X ^ 3
  let H := D * (N - X * Np) + X * Dp * N
  change PowerSeries.constantCoeff D = 1 ∧
    PowerSeriesPrefixEq 11 (D * U) N ∧
    PowerSeriesPrefixEq 10 (F * H ^ 2)
      (D * N ^ 3 + PowerSeries.C targetA * X ^ 2 * N * D ^ 3 +
        PowerSeries.C targetB * X ^ 3 * D ^ 4) at hcert
  rcases hcert with ⟨hconst, hquot, hpoly⟩
  have hN : PowerSeriesPrefixEq 10 N (D * U) :=
    PowerSeriesPrefixEq.symm
      (PowerSeriesPrefixEq.mono (by omega) hquot)
  have hderiv : PowerSeriesPrefixEq 10
      (PowerSeries.derivative (D * U)) Np := by
    simpa [Np] using PowerSeriesPrefixEq.derivative (n := 10) hquot
  have hNp : PowerSeriesPrefixEq 10 Np (Dp * U + D * Up) := by
    apply PowerSeriesPrefixEq.trans (PowerSeriesPrefixEq.symm hderiv)
    apply PowerSeriesPrefixEq.of_eq
    simp only [PowerSeries.derivative.leibniz, smul_eq_mul]
    dsimp only [Dp, Up]
    ring
  have hInner : PowerSeriesPrefixEq 10
      (N - X * Np) (D * U - X * (Dp * U + D * Up)) :=
    PowerSeriesPrefixEq.sub hN
      (PowerSeriesPrefixEq.mul (PowerSeriesPrefixEq.refl X) hNp)
  have hHraw : PowerSeriesPrefixEq 10 H
      (D * (D * U - X * (Dp * U + D * Up)) + X * Dp * (D * U)) :=
    PowerSeriesPrefixEq.add
      (PowerSeriesPrefixEq.mul (PowerSeriesPrefixEq.refl D) hInner)
      (PowerSeriesPrefixEq.mul
        (PowerSeriesPrefixEq.mul (PowerSeriesPrefixEq.refl X)
          (PowerSeriesPrefixEq.refl Dp)) hN)
  have hH : PowerSeriesPrefixEq 10 H (D ^ 2 * W) := by
    apply PowerSeriesPrefixEq.trans hHraw
    apply PowerSeriesPrefixEq.of_eq
    dsimp only [W]
    ring
  have hleft : PowerSeriesPrefixEq 10 (F * H ^ 2) (D ^ 4 * (F * W ^ 2)) := by
    apply PowerSeriesPrefixEq.trans
      (PowerSeriesPrefixEq.mul (PowerSeriesPrefixEq.refl F)
        (PowerSeriesPrefixEq.pow hH 2))
    apply PowerSeriesPrefixEq.of_eq
    ring
  have hr1 : PowerSeriesPrefixEq 10 (D * N ^ 3) (D * (D * U) ^ 3) :=
    PowerSeriesPrefixEq.mul (PowerSeriesPrefixEq.refl D)
      (PowerSeriesPrefixEq.pow hN 3)
  have hr2 : PowerSeriesPrefixEq 10
      (PowerSeries.C targetA * X ^ 2 * N * D ^ 3)
      (PowerSeries.C targetA * X ^ 2 * (D * U) * D ^ 3) :=
    PowerSeriesPrefixEq.mul
      (PowerSeriesPrefixEq.mul
        (PowerSeriesPrefixEq.mul (PowerSeriesPrefixEq.refl _)
          (PowerSeriesPrefixEq.refl _)) hN)
      (PowerSeriesPrefixEq.refl _)
  have hr3 : PowerSeriesPrefixEq 10
      (PowerSeries.C targetB * X ^ 3 * D ^ 4)
      (PowerSeries.C targetB * X ^ 3 * D ^ 4) :=
    PowerSeriesPrefixEq.refl _
  have hrightRaw := PowerSeriesPrefixEq.add
    (PowerSeriesPrefixEq.add hr1 hr2) hr3
  have hright : PowerSeriesPrefixEq 10
      (D * N ^ 3 + PowerSeries.C targetA * X ^ 2 * N * D ^ 3 +
        PowerSeries.C targetB * X ^ 3 * D ^ 4)
      (D ^ 4 * G) := by
    apply PowerSeriesPrefixEq.trans hrightRaw
    apply PowerSeriesPrefixEq.of_eq
    dsimp only [G]
    ring
  have hcleared : PowerSeriesPrefixEq 10
      (D ^ 4 * (F * W ^ 2)) (D ^ 4 * G) :=
    PowerSeriesPrefixEq.trans (PowerSeriesPrefixEq.symm hleft)
      (PowerSeriesPrefixEq.trans hpoly hright)
  have hD4 : PowerSeries.constantCoeff (D ^ 4) ≠ 0 := by
    simp [hconst]
  have hfirst := PowerSeriesPrefixEq.cancel_left (D ^ 4) hD4 hcleared
  simpa [NormalizedIsogenyFirstOrderPrefix, U, Up, X, W, F, G] using hfirst

/-- Equation (15) of Bostan--Morain--Salvy--Schost after substituting
`R(x)=x U(1/x)` and multiplying by `w²`. It is written in distributed form so
that coefficient extraction does not hide work in nested Cauchy products. -/
def NormalizedIsogenySecondOrderEquation
    (sourceA sourceB targetA : K) (h : ℕ → K) : Prop :=
  let U := normalizedMapAtInfinitySeries h
  let W := U - PowerSeries.X * PowerSeries.derivative U
  let Udd := PowerSeries.derivative (PowerSeries.derivative U)
  PowerSeries.C 3 * W +
      PowerSeries.C sourceA * (PowerSeries.X ^ 2 * W) +
      PowerSeries.C 2 * (PowerSeries.X ^ 2 * Udd) +
      PowerSeries.C (2 * sourceA) * (PowerSeries.X ^ 4 * Udd) +
      PowerSeries.C (2 * sourceB) * (PowerSeries.X ^ 5 * Udd) =
    PowerSeries.C 3 * (U * U) + PowerSeries.C targetA * PowerSeries.X ^ 2

/-- The transformed first-order curve equation implies the exact distributed
PowerSeries form used for H116. The cancelled factor is automatically nonzero:
its constant coefficient is one for `normalizedMapAtInfinitySeries`. -/
theorem NormalizedIsogenyFirstOrderEquation.to_secondOrder
    (sourceA sourceB targetA targetB : K) (h : ℕ → K)
    (hfirst : NormalizedIsogenyFirstOrderEquation
      sourceA sourceB targetA targetB h) :
    NormalizedIsogenySecondOrderEquation sourceA sourceB targetA h := by
  let U := normalizedMapAtInfinitySeries h
  let Up := PowerSeries.derivative U
  let Udd := PowerSeries.derivative Up
  let W := U - PowerSeries.X * Up
  have hW : W ≠ 0 := by
    intro hz
    have hc := congrArg (PowerSeries.coeff 0) hz
    simp [W, U, Up, normalizedMapAtInfinitySeries, fiberDenominatorSeries] at hc
  have hcurve :
      (1 + PowerSeries.C sourceA * PowerSeries.X ^ 2 +
          PowerSeries.C sourceB * PowerSeries.X ^ 3) * W ^ 2 =
        U ^ 3 + PowerSeries.C targetA * PowerSeries.X ^ 2 * U +
          PowerSeries.C targetB * PowerSeries.X ^ 3 := by
    simpa [NormalizedIsogenyFirstOrderEquation, U, Up, W] using hfirst
  have hcompact := transformedFirstOrderEquation_implies_secondOrder
    PowerSeries.derivative PowerSeries.X U Up Udd
      (PowerSeries.C sourceA) (PowerSeries.C sourceB)
      (PowerSeries.C targetA) (PowerSeries.C targetB)
      (by simp) (by simp [Up]) (by simp [Up, Udd])
      (by simp) (by simp) (by simp) (by simp) hW hcurve
  unfold NormalizedIsogenySecondOrderEquation
  dsimp only
  simp only [map_ofNat, map_mul] at hcompact ⊢
  linear_combination hcompact

/-- The distributed normalized ODE, only below degree nine. -/
def NormalizedIsogenySecondOrderPrefix
    (sourceA sourceB targetA : K) (h : ℕ → K) : Prop :=
  let U := normalizedMapAtInfinitySeries h
  let W := U - PowerSeries.X * PowerSeries.derivative U
  let Udd := PowerSeries.derivative (PowerSeries.derivative U)
  PowerSeriesPrefixEq 9
    (PowerSeries.C 3 * W +
      PowerSeries.C sourceA * (PowerSeries.X ^ 2 * W) +
      PowerSeries.C 2 * (PowerSeries.X ^ 2 * Udd) +
      PowerSeries.C (2 * sourceA) * (PowerSeries.X ^ 4 * Udd) +
      PowerSeries.C (2 * sourceB) * (PowerSeries.X ^ 5 * Udd))
    (PowerSeries.C 3 * (U * U) + PowerSeries.C targetA * PowerSeries.X ^ 2)

/-- Differentiating the ten-coefficient first-order congruence gives the
nine-coefficient ODE congruence. The factor `U-XU'` is cancellable because its
constant coefficient is one. -/
theorem NormalizedIsogenyFirstOrderPrefix.to_secondOrderPrefix
    (sourceA sourceB targetA targetB : K) (h : ℕ → K)
    (hfirst : NormalizedIsogenyFirstOrderPrefix
      sourceA sourceB targetA targetB h) :
    NormalizedIsogenySecondOrderPrefix sourceA sourceB targetA h := by
  let U := normalizedMapAtInfinitySeries h
  let Up := PowerSeries.derivative U
  let Udd := PowerSeries.derivative Up
  let X : PowerSeries K := PowerSeries.X
  let W := U - X * Up
  let F := 1 + PowerSeries.C sourceA * X ^ 2 + PowerSeries.C sourceB * X ^ 3
  let G := U ^ 3 + PowerSeries.C targetA * X ^ 2 * U +
    PowerSeries.C targetB * X ^ 3
  let OL := PowerSeries.C 3 * W +
    PowerSeries.C sourceA * (X ^ 2 * W) +
    PowerSeries.C 2 * (X ^ 2 * Udd) +
    PowerSeries.C (2 * sourceA) * (X ^ 4 * Udd) +
    PowerSeries.C (2 * sourceB) * (X ^ 5 * Udd)
  let OR := PowerSeries.C 3 * (U * U) + PowerSeries.C targetA * X ^ 2
  change PowerSeriesPrefixEq 10 (F * W ^ 2) G at hfirst
  have hcurve9 := PowerSeriesPrefixEq.mono (m := 9) (by omega) hfirst
  have hderiv : PowerSeriesPrefixEq 9
      (PowerSeries.derivative (F * W ^ 2)) (PowerSeries.derivative G) := by
    simpa using PowerSeriesPrefixEq.derivative (n := 9) hfirst
  have hcombined : PowerSeriesPrefixEq 9
      (PowerSeries.C 3 * (F * W ^ 2) -
        X * PowerSeries.derivative (F * W ^ 2))
      (PowerSeries.C 3 * G - X * PowerSeries.derivative G) :=
    PowerSeriesPrefixEq.sub
      (PowerSeriesPrefixEq.mul (PowerSeriesPrefixEq.refl _) hcurve9)
      (PowerSeriesPrefixEq.mul (PowerSeriesPrefixEq.refl X) hderiv)
  have hleft :
      PowerSeries.C 3 * (F * W ^ 2) -
          X * PowerSeries.derivative (F * W ^ 2) = W * OL := by
    dsimp only [F, W, OL, Udd, Up, X]
    simp only [PowerSeries.derivative.leibniz, PowerSeries.derivative_pow,
      PowerSeries.derivative_X, PowerSeries.derivative_C, smul_eq_mul,
      PowerSeries.derivative_one, map_ofNat, map_mul, map_add, map_sub]
    ring
  have hright :
      PowerSeries.C 3 * G - X * PowerSeries.derivative G = W * OR := by
    dsimp only [G, W, OR, Up, X]
    simp only [PowerSeries.derivative.leibniz, PowerSeries.derivative_pow,
      PowerSeries.derivative_X, PowerSeries.derivative_C, smul_eq_mul,
      map_ofNat, map_add]
    ring
  have hfactored : PowerSeriesPrefixEq 9 (W * OL) (W * OR) :=
    PowerSeriesPrefixEq.trans (PowerSeriesPrefixEq.symm
      (PowerSeriesPrefixEq.of_eq hleft))
      (PowerSeriesPrefixEq.trans hcombined (PowerSeriesPrefixEq.of_eq hright))
  have hW0 : PowerSeries.constantCoeff W ≠ 0 := by
    simp [W, U, Up, X, normalizedMapAtInfinitySeries, fiberDenominatorSeries]
  have hode := PowerSeriesPrefixEq.cancel_left W hW0 hfactored
  simpa [NormalizedIsogenySecondOrderPrefix, U, Up, Udd, X, W, OL, OR] using hode

/-- Degrees two and three of the first-order prefix already force the two base
recurrence coefficients. -/
theorem NormalizedIsogenyFirstOrderPrefix.base_coefficients
    (sourceA sourceB targetA targetB : K) (h : ℕ → K)
    (hfirst : NormalizedIsogenyFirstOrderPrefix
      sourceA sourceB targetA targetB h)
    (h5 : (5 : K) ≠ 0) (h7 : (7 : K) ≠ 0) :
    h 1 = (sourceA - targetA) / 5 ∧
      h 2 = (sourceB - targetB) / 7 := by
  unfold NormalizedIsogenyFirstOrderPrefix at hfirst
  dsimp only at hfirst
  constructor
  · have hc := PowerSeriesPrefixEq.coeff_eq hfirst (i := 2) (by omega)
    simp [normalizedMapAtInfinitySeries, fiberDenominatorSeries, pow_succ,
      PowerSeries.coeff_mul, PowerSeries.coeff_derivative,
      PowerSeries.coeff_X] at hc
    simp_rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ_mk] at hc
    norm_num [Finset.sum_range_succ] at hc
    apply (eq_div_iff h5).2
    linear_combination -hc
  · have hc := PowerSeriesPrefixEq.coeff_eq hfirst (i := 3) (by omega)
    simp [normalizedMapAtInfinitySeries, fiberDenominatorSeries, pow_succ,
      PowerSeries.coeff_mul, PowerSeries.coeff_derivative,
      PowerSeries.coeff_X] at hc
    simp_rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ_mk] at hc
    norm_num [Finset.sum_range_succ] at hc
    apply (eq_div_iff h7).2
    linear_combination -hc

/-- The degree-two and degree-three coefficients of transformed equation (14)
force the two initial Bostan jet coefficients. These are therefore consequences
of the first-order geometric interface, not independent data. -/
theorem NormalizedIsogenyFirstOrderEquation.base_coefficients
    (sourceA sourceB targetA targetB : K) (h : ℕ → K)
    (hfirst : NormalizedIsogenyFirstOrderEquation
      sourceA sourceB targetA targetB h)
    (h5 : (5 : K) ≠ 0) (h7 : (7 : K) ≠ 0) :
    h 1 = (sourceA - targetA) / 5 ∧
      h 2 = (sourceB - targetB) / 7 := by
  constructor
  · have hc := congrArg (PowerSeries.coeff 2) hfirst
    simp [normalizedMapAtInfinitySeries, fiberDenominatorSeries, pow_succ,
      PowerSeries.coeff_mul, PowerSeries.coeff_derivative,
      PowerSeries.coeff_X] at hc
    simp_rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ_mk] at hc
    norm_num [Finset.sum_range_succ] at hc
    apply (eq_div_iff h5).2
    linear_combination -hc
  · have hc := congrArg (PowerSeries.coeff 3) hfirst
    simp [normalizedMapAtInfinitySeries, fiberDenominatorSeries, pow_succ,
      PowerSeries.coeff_mul, PowerSeries.coeff_derivative,
      PowerSeries.coeff_X] at hc
    simp_rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ_mk] at hc
    norm_num [Finset.sum_range_succ] at hc
    apply (eq_div_iff h7).2
    linear_combination -hc

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

/-- The two denominators in every indexed recurrence step are nonzero under
the frozen prime-characteristic condition. -/
theorem bostanStepDenominatorsNonzero {p k : ℕ} [Fact p.Prime]
    (hp : 19 < p) (hk3 : 3 ≤ k) (hk7 : k ≤ 7) :
    ((((k - 2) * (2 * k + 3) : ℕ) : ZMod p) ≠ 0) ∧
      (((2 * k + 3 : ℕ) : ZMod p) ≠ 0) := by
  rcases bostanDenominatorsNonzero hp with
    ⟨_, _, h9, h11, h13, h15, h17, h22, h39, h60, h85⟩
  interval_cases k <;> norm_num at hk3 hk7 ⊢
  all_goals aesop

private theorem normalizedODE_step3
    (sourceA sourceB targetA : K) (h : ℕ → K)
    (hode : NormalizedIsogenySecondOrderEquation sourceA sourceB targetA h)
    (_hprod : (9 : K) ≠ 0) (hsimple : (9 : K) ≠ 0) :
    h 3 = 3 * (∑ i ∈ Finset.Icc 1 (3 - 2), h i * h (3 - 1 - i)) /
          (((3 - 2) * (2 * 3 + 3) : ℕ) : K) -
        (((2 * 3 - 3 : ℕ) : K) * sourceA * h (3 - 2)) /
          ((2 * 3 + 3 : ℕ) : K) -
        (2 * ((3 - 3 : ℕ) : K) * sourceB * h (3 - 3)) /
          ((2 * 3 + 3 : ℕ) : K) := by
  have hc := congrArg (PowerSeries.coeff 4) hode
  simp [normalizedMapAtInfinitySeries, PowerSeries.coeff_mul,
    PowerSeries.coeff_derivative, PowerSeries.coeff_X] at hc
  simp_rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ_mk] at hc
  norm_num [Finset.sum_range_succ,
    show Finset.Icc 1 1 = {1} by decide] at hc ⊢
  field_simp [hsimple]
  ring_nf at hc ⊢
  linear_combination hc

private theorem normalizedODE_step4
    (sourceA sourceB targetA : K) (h : ℕ → K)
    (hode : NormalizedIsogenySecondOrderEquation sourceA sourceB targetA h)
    (hprod : (22 : K) ≠ 0) (hsimple : (11 : K) ≠ 0) :
    h 4 = 3 * (∑ i ∈ Finset.Icc 1 (4 - 2), h i * h (4 - 1 - i)) /
          (((4 - 2) * (2 * 4 + 3) : ℕ) : K) -
        (((2 * 4 - 3 : ℕ) : K) * sourceA * h (4 - 2)) /
          ((2 * 4 + 3 : ℕ) : K) -
        (2 * ((4 - 3 : ℕ) : K) * sourceB * h (4 - 3)) /
          ((2 * 4 + 3 : ℕ) : K) := by
  have hc := congrArg (PowerSeries.coeff 5) hode
  simp [normalizedMapAtInfinitySeries, PowerSeries.coeff_mul,
    PowerSeries.coeff_derivative, PowerSeries.coeff_X] at hc
  simp_rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ_mk] at hc
  norm_num [Finset.sum_range_succ,
    show Finset.Icc 1 2 = {1, 2} by decide] at hc ⊢
  field_simp [hprod, hsimple]
  ring_nf at hc ⊢
  linear_combination 11 * hc

private theorem normalizedODE_step5
    (sourceA sourceB targetA : K) (h : ℕ → K)
    (hode : NormalizedIsogenySecondOrderEquation sourceA sourceB targetA h)
    (hprod : (39 : K) ≠ 0) (hsimple : (13 : K) ≠ 0) :
    h 5 = 3 * (∑ i ∈ Finset.Icc 1 (5 - 2), h i * h (5 - 1 - i)) /
          (((5 - 2) * (2 * 5 + 3) : ℕ) : K) -
        (((2 * 5 - 3 : ℕ) : K) * sourceA * h (5 - 2)) /
          ((2 * 5 + 3 : ℕ) : K) -
        (2 * ((5 - 3 : ℕ) : K) * sourceB * h (5 - 3)) /
          ((2 * 5 + 3 : ℕ) : K) := by
  have hc := congrArg (PowerSeries.coeff 6) hode
  simp [normalizedMapAtInfinitySeries, PowerSeries.coeff_mul,
    PowerSeries.coeff_derivative, PowerSeries.coeff_X] at hc
  simp_rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ_mk] at hc
  norm_num [Finset.sum_range_succ,
    show Finset.Icc 1 3 = {1, 2, 3} by decide] at hc ⊢
  field_simp [hprod, hsimple]
  ring_nf at hc ⊢
  linear_combination 13 * hc

private theorem normalizedODE_step6
    (sourceA sourceB targetA : K) (h : ℕ → K)
    (hode : NormalizedIsogenySecondOrderEquation sourceA sourceB targetA h)
    (hprod : (60 : K) ≠ 0) (hsimple : (15 : K) ≠ 0) :
    h 6 = 3 * (∑ i ∈ Finset.Icc 1 (6 - 2), h i * h (6 - 1 - i)) /
          (((6 - 2) * (2 * 6 + 3) : ℕ) : K) -
        (((2 * 6 - 3 : ℕ) : K) * sourceA * h (6 - 2)) /
          ((2 * 6 + 3 : ℕ) : K) -
        (2 * ((6 - 3 : ℕ) : K) * sourceB * h (6 - 3)) /
          ((2 * 6 + 3 : ℕ) : K) := by
  have hc := congrArg (PowerSeries.coeff 7) hode
  simp [normalizedMapAtInfinitySeries, PowerSeries.coeff_mul,
    PowerSeries.coeff_derivative, PowerSeries.coeff_X] at hc
  simp_rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ_mk] at hc
  norm_num [Finset.sum_range_succ,
    show Finset.Icc 1 4 = {1, 2, 3, 4} by decide] at hc ⊢
  field_simp [hprod, hsimple]
  ring_nf at hc ⊢
  linear_combination 15 * hc

private theorem normalizedODE_step7
    (sourceA sourceB targetA : K) (h : ℕ → K)
    (hode : NormalizedIsogenySecondOrderEquation sourceA sourceB targetA h)
    (hprod : (85 : K) ≠ 0) (hsimple : (17 : K) ≠ 0) :
    h 7 = 3 * (∑ i ∈ Finset.Icc 1 (7 - 2), h i * h (7 - 1 - i)) /
          (((7 - 2) * (2 * 7 + 3) : ℕ) : K) -
        (((2 * 7 - 3 : ℕ) : K) * sourceA * h (7 - 2)) /
          ((2 * 7 + 3 : ℕ) : K) -
        (2 * ((7 - 3 : ℕ) : K) * sourceB * h (7 - 3)) /
          ((2 * 7 + 3 : ℕ) : K) := by
  have hc := congrArg (PowerSeries.coeff 8) hode
  simp [normalizedMapAtInfinitySeries, PowerSeries.coeff_mul,
    PowerSeries.coeff_derivative, PowerSeries.coeff_X] at hc
  simp_rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ_mk] at hc
  norm_num [Finset.sum_range_succ,
    show Finset.Icc 1 5 = {1, 2, 3, 4, 5} by decide] at hc ⊢
  field_simp [hprod, hsimple]
  ring_nf at hc ⊢
  linear_combination 17 * hc

private theorem normalizedODEPrefix_step6
    (sourceA sourceB targetA : K) (h : ℕ → K)
    (hode : NormalizedIsogenySecondOrderPrefix sourceA sourceB targetA h)
    (hprod : (60 : K) ≠ 0) (hsimple : (15 : K) ≠ 0) :
    h 6 = 3 * (∑ i ∈ Finset.Icc 1 (6 - 2), h i * h (6 - 1 - i)) /
          (((6 - 2) * (2 * 6 + 3) : ℕ) : K) -
        (((2 * 6 - 3 : ℕ) : K) * sourceA * h (6 - 2)) /
          ((2 * 6 + 3 : ℕ) : K) -
        (2 * ((6 - 3 : ℕ) : K) * sourceB * h (6 - 3)) /
          ((2 * 6 + 3 : ℕ) : K) := by
  unfold NormalizedIsogenySecondOrderPrefix at hode
  dsimp only at hode
  have hc := PowerSeriesPrefixEq.coeff_eq hode (i := 7) (by omega)
  simp [normalizedMapAtInfinitySeries, PowerSeries.coeff_mul,
    PowerSeries.coeff_derivative, PowerSeries.coeff_X] at hc
  simp_rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ_mk] at hc
  norm_num [Finset.sum_range_succ,
    show Finset.Icc 1 4 = {1, 2, 3, 4} by decide] at hc ⊢
  field_simp [hprod, hsimple]
  ring_nf at hc ⊢
  linear_combination 15 * hc

private theorem normalizedODEPrefix_step7
    (sourceA sourceB targetA : K) (h : ℕ → K)
    (hode : NormalizedIsogenySecondOrderPrefix sourceA sourceB targetA h)
    (hprod : (85 : K) ≠ 0) (hsimple : (17 : K) ≠ 0) :
    h 7 = 3 * (∑ i ∈ Finset.Icc 1 (7 - 2), h i * h (7 - 1 - i)) /
          (((7 - 2) * (2 * 7 + 3) : ℕ) : K) -
        (((2 * 7 - 3 : ℕ) : K) * sourceA * h (7 - 2)) /
          ((2 * 7 + 3 : ℕ) : K) -
        (2 * ((7 - 3 : ℕ) : K) * sourceB * h (7 - 3)) /
          ((2 * 7 + 3 : ℕ) : K) := by
  unfold NormalizedIsogenySecondOrderPrefix at hode
  dsimp only at hode
  have hc := PowerSeriesPrefixEq.coeff_eq hode (i := 8) (by omega)
  simp [normalizedMapAtInfinitySeries, PowerSeries.coeff_mul,
    PowerSeries.coeff_derivative, PowerSeries.coeff_X] at hc
  simp_rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ_mk] at hc
  norm_num [Finset.sum_range_succ,
    show Finset.Icc 1 5 = {1, 2, 3, 4, 5} by decide] at hc ⊢
  field_simp [hprod, hsimple]
  ring_nf at hc ⊢
  linear_combination 17 * hc

/-- A normalized at-infinity jet satisfying the transformed isogeny equation
and the two initial coefficient identities satisfies the exact finite recurrence
used by H116. -/
theorem BostanJetRecurrence.of_normalizedSecondOrderEquation
    (sourceA sourceB targetA targetB : K) (h : ℕ → K)
    (hbase1 : h 1 = (sourceA - targetA) / 5)
    (hbase2 : h 2 = (sourceB - targetB) / 7)
    (hode : NormalizedIsogenySecondOrderEquation sourceA sourceB targetA h)
    (hden : ∀ k, 3 ≤ k → k ≤ 7 →
      ((((k - 2) * (2 * k + 3) : ℕ) : K) ≠ 0) ∧
        (((2 * k + 3 : ℕ) : K) ≠ 0)) :
    BostanJetRecurrence sourceA sourceB targetA targetB h := by
  refine ⟨hbase1, hbase2, ?_⟩
  intro k hk3 hk7
  have hd := hden k hk3 hk7
  interval_cases k
  · exact normalizedODE_step3 sourceA sourceB targetA h hode hd.1 hd.2
  · exact normalizedODE_step4 sourceA sourceB targetA h hode hd.1 hd.2
  · exact normalizedODE_step5 sourceA sourceB targetA h hode hd.1 hd.2
  · exact normalizedODE_step6 sourceA sourceB targetA h hode hd.1 hd.2
  · exact normalizedODE_step7 sourceA sourceB targetA h hode hd.1 hd.2

/-- The nine-coefficient ODE prefix is sufficient for all five recurrence
steps: the last step reads only coefficient eight. -/
theorem BostanJetRecurrence.of_normalizedSecondOrderPrefix
    (sourceA sourceB targetA targetB : K) (h : ℕ → K)
    (hbase1 : h 1 = (sourceA - targetA) / 5)
    (hbase2 : h 2 = (sourceB - targetB) / 7)
    (hode : NormalizedIsogenySecondOrderPrefix sourceA sourceB targetA h)
    (hden : ∀ k, 3 ≤ k → k ≤ 7 →
      ((((k - 2) * (2 * k + 3) : ℕ) : K) ≠ 0) ∧
        (((2 * k + 3 : ℕ) : K) ≠ 0)) :
    BostanJetRecurrence sourceA sourceB targetA targetB h := by
  refine ⟨hbase1, hbase2, ?_⟩
  intro k hk3 hk7
  rcases hden k hk3 hk7 with ⟨hprod, hsimple⟩
  interval_cases k <;> norm_num at hprod hsimple
  · have hc := PowerSeriesPrefixEq.coeff_eq hode (i := 4) (by omega)
    simp [normalizedMapAtInfinitySeries,
      PowerSeries.coeff_mul, PowerSeries.coeff_derivative,
      PowerSeries.coeff_X] at hc
    simp_rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ_mk] at hc
    norm_num [Finset.sum_range_succ,
      show Finset.Icc 1 1 = {1} by decide] at hc ⊢
    field_simp [hsimple]
    ring_nf at hc ⊢
    linear_combination hc
  · have hc := PowerSeriesPrefixEq.coeff_eq hode (i := 5) (by omega)
    simp [normalizedMapAtInfinitySeries,
      PowerSeries.coeff_mul, PowerSeries.coeff_derivative,
      PowerSeries.coeff_X] at hc
    simp_rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ_mk] at hc
    norm_num [Finset.sum_range_succ,
      show Finset.Icc 1 2 = {1, 2} by decide] at hc ⊢
    field_simp [hprod, hsimple]
    ring_nf at hc ⊢
    linear_combination 11 * hc
  · have hc := PowerSeriesPrefixEq.coeff_eq hode (i := 6) (by omega)
    simp [normalizedMapAtInfinitySeries,
      PowerSeries.coeff_mul, PowerSeries.coeff_derivative,
      PowerSeries.coeff_X] at hc
    simp_rw [Finset.Nat.sum_antidiagonal_eq_sum_range_succ_mk] at hc
    norm_num [Finset.sum_range_succ,
      show Finset.Icc 1 3 = {1, 2, 3} by decide] at hc ⊢
    field_simp [hprod, hsimple]
    ring_nf at hc ⊢
    linear_combination 13 * hc
  · exact normalizedODEPrefix_step6 sourceA sourceB targetA h hode hprod hsimple
  · exact normalizedODEPrefix_step7 sourceA sourceB targetA h hode hprod hsimple

/-- A ten-coefficient first-order prefix supplies the base coefficients and the
nine-coefficient ODE prefix, hence the complete finite recurrence. -/
theorem BostanJetRecurrence.of_normalizedFirstOrderPrefix
    (sourceA sourceB targetA targetB : K) (h : ℕ → K)
    (hfirst : NormalizedIsogenyFirstOrderPrefix
      sourceA sourceB targetA targetB h)
    (h5 : (5 : K) ≠ 0) (h7 : (7 : K) ≠ 0)
    (hden : ∀ k, 3 ≤ k → k ≤ 7 →
      ((((k - 2) * (2 * k + 3) : ℕ) : K) ≠ 0) ∧
        (((2 * k + 3 : ℕ) : K) ≠ 0)) :
    BostanJetRecurrence sourceA sourceB targetA targetB h := by
  rcases NormalizedIsogenyFirstOrderPrefix.base_coefficients
    sourceA sourceB targetA targetB h hfirst h5 h7 with ⟨hbase1, hbase2⟩
  exact BostanJetRecurrence.of_normalizedSecondOrderPrefix
    sourceA sourceB targetA targetB h hbase1 hbase2
      (NormalizedIsogenyFirstOrderPrefix.to_secondOrderPrefix
        sourceA sourceB targetA targetB h hfirst) hden

/-- Transformed equation (14) alone implies the finite recurrence whenever the
displayed small denominators are nonzero. The initial coefficients and the
second-order equation are both derived internally. -/
theorem BostanJetRecurrence.of_normalizedFirstOrderEquation
    (sourceA sourceB targetA targetB : K) (h : ℕ → K)
    (hfirst : NormalizedIsogenyFirstOrderEquation
      sourceA sourceB targetA targetB h)
    (h5 : (5 : K) ≠ 0) (h7 : (7 : K) ≠ 0)
    (hden : ∀ k, 3 ≤ k → k ≤ 7 →
      ((((k - 2) * (2 * k + 3) : ℕ) : K) ≠ 0) ∧
        (((2 * k + 3 : ℕ) : K) ≠ 0)) :
    BostanJetRecurrence sourceA sourceB targetA targetB h := by
  rcases NormalizedIsogenyFirstOrderEquation.base_coefficients
    sourceA sourceB targetA targetB h hfirst h5 h7 with ⟨hbase1, hbase2⟩
  exact BostanJetRecurrence.of_normalizedSecondOrderEquation
    sourceA sourceB targetA targetB h hbase1 hbase2
      (NormalizedIsogenyFirstOrderEquation.to_secondOrder
        sourceA sourceB targetA targetB h hfirst) hden

/-- Prime-field specialization with every recurrence denominator discharged by
`p > 19`. -/
theorem BostanJetRecurrence.of_normalizedSecondOrderEquation_zmod
    {p : ℕ} [Fact p.Prime] (hp : 19 < p)
    (sourceA sourceB targetA targetB : ZMod p) (h : ℕ → ZMod p)
    (hbase1 : h 1 = (sourceA - targetA) / 5)
    (hbase2 : h 2 = (sourceB - targetB) / 7)
    (hode : NormalizedIsogenySecondOrderEquation sourceA sourceB targetA h) :
    BostanJetRecurrence sourceA sourceB targetA targetB h := by
  exact BostanJetRecurrence.of_normalizedSecondOrderEquation
    sourceA sourceB targetA targetB h hbase1 hbase2 hode
      (fun k hk3 hk7 ↦ bostanStepDenominatorsNonzero hp hk3 hk7)

/-- In prime characteristic `p>19`, transformed equation (14) by itself forces
the exact recurrence used by the H116 producer. -/
theorem BostanJetRecurrence.of_normalizedFirstOrderEquation_zmod
    {p : ℕ} [Fact p.Prime] (hp : 19 < p)
    (sourceA sourceB targetA targetB : ZMod p) (h : ℕ → ZMod p)
    (hfirst : NormalizedIsogenyFirstOrderEquation
      sourceA sourceB targetA targetB h) :
    BostanJetRecurrence sourceA sourceB targetA targetB h := by
  rcases bostanDenominatorsNonzero hp with ⟨h5, h7, _⟩
  exact BostanJetRecurrence.of_normalizedFirstOrderEquation
    sourceA sourceB targetA targetB h hfirst h5 h7
      (fun k hk3 hk7 ↦ bostanStepDenominatorsNonzero hp hk3 hk7)

/-- A finite reversed numerator/denominator certificate implies the complete
H116 recurrence over every prime field in the frozen characteristic range. -/
theorem BostanJetRecurrence.of_normalizedRationalMapPolynomialCertificate_zmod
    {p : ℕ} [Fact p.Prime] (hp : 19 < p)
    (sourceA sourceB targetA targetB : ZMod p) (h : ℕ → ZMod p)
    (N D : Polynomial (ZMod p))
    (hcert : NormalizedRationalMapPolynomialCertificate
      sourceA sourceB targetA targetB h N D) :
    BostanJetRecurrence sourceA sourceB targetA targetB h := by
  exact BostanJetRecurrence.of_normalizedFirstOrderEquation_zmod hp
    sourceA sourceB targetA targetB h
      (NormalizedRationalMapPolynomialCertificate.to_firstOrder
        sourceA sourceB targetA targetB h N D hcert)

/-- The fixed eleven-coefficient rational-map certificate implies the complete
H116 recurrence over every prime field in the frozen characteristic range. -/
theorem BostanJetRecurrence.of_normalizedRationalMapJetCertificate_zmod
    {p : ℕ} [Fact p.Prime] (hp : 19 < p)
    (sourceA sourceB targetA targetB : ZMod p) (h : ℕ → ZMod p)
    (N D : PowerSeries (ZMod p))
    (hcert : NormalizedRationalMapJetCertificate
      sourceA sourceB targetA targetB h N D) :
    BostanJetRecurrence sourceA sourceB targetA targetB h := by
  rcases bostanDenominatorsNonzero hp with ⟨h5, h7, _⟩
  exact BostanJetRecurrence.of_normalizedFirstOrderPrefix
    sourceA sourceB targetA targetB h
      (NormalizedRationalMapJetCertificate.to_firstOrderPrefix
        sourceA sourceB targetA targetB h N D hcert)
      h5 h7 (fun k hk3 hk7 ↦ bostanStepDenominatorsNonzero hp hk3 hk7)

/-- The seven coefficients satisfying the H116 recurrence are unique. -/
theorem BostanJetRecurrence.unique
    {sourceA sourceB targetA targetB : K} {h g : ℕ → K}
    (hh : BostanJetRecurrence sourceA sourceB targetA targetB h)
    (hg : BostanJetRecurrence sourceA sourceB targetA targetB g) :
    ∀ k, 1 ≤ k → k ≤ 7 → h k = g k := by
  rcases hh with ⟨hh1, hh2, hhstep⟩
  rcases hg with ⟨hg1, hg2, hgstep⟩
  intro k
  induction k using Nat.strong_induction_on with
  | h k ih =>
      intro hk1 hk7
      by_cases hk_one : k = 1
      · simpa [hk_one] using hh1.trans hg1.symm
      by_cases hk_two : k = 2
      · simpa [hk_two] using hh2.trans hg2.symm
      have hk3 : 3 ≤ k := by omega
      rw [hhstep k hk3 hk7, hgstep k hk3 hk7]
      have hsum :
          (∑ i ∈ Finset.Icc 1 (k - 2), h i * h (k - 1 - i)) =
            ∑ i ∈ Finset.Icc 1 (k - 2), g i * g (k - 1 - i) := by
        apply Finset.sum_congr rfl
        intro i hi
        have hi_bounds := Finset.mem_Icc.mp hi
        have hi_pos : 1 ≤ i := hi_bounds.1
        have hi_lt : i < k := by omega
        have hki_pos : 1 ≤ k - 1 - i := by omega
        have hki_lt : k - 1 - i < k := by omega
        rw [ih i hi_lt hi_pos (by omega),
          ih (k - 1 - i) hki_lt hki_pos (by omega)]
      rw [hsum, ih (k - 2) (by omega) (by omega) (by omega)]
      by_cases hk_three : k = 3
      · subst k
        norm_num
      · rw [ih (k - 3) (by omega) (by omega) (by omega)]

/-- Any two solutions of the finite recurrence induce identical `F_U`
coefficients through the producer's truncation degree eight. -/
theorem fiberDenominatorSeries_coeff_eq_of_bostanRecurrence
    {sourceA sourceB targetA targetB : K} {h g : ℕ → K}
    (hh : BostanJetRecurrence sourceA sourceB targetA targetB h)
    (hg : BostanJetRecurrence sourceA sourceB targetA targetB g)
    (u : K) (n : ℕ) (hn : n ≤ 8) :
    PowerSeries.coeff n (fiberDenominatorSeries u h) =
      PowerSeries.coeff n (fiberDenominatorSeries u g) := by
  rcases n with _ | _ | n
  · simp
  · simp
  · rw [fiberDenominatorSeries_coeff_succ_succ,
      fiberDenominatorSeries_coeff_succ_succ]
    exact BostanJetRecurrence.unique hh hg (n + 1) (by omega) (by omega)

#print axioms logDeriv_mul
#print axioms logDeriv_finset
#print axioms logDeriv_div
#print axioms denominator_cancellation
#print axioms normalizedMapEquation_implies_secondOrder
#print axioms transformedFirstOrderEquation_implies_secondOrder
#print axioms quotientCertificate_implies_firstOrderEquation
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
#print axioms bostanStepDenominatorsNonzero
#print axioms NormalizedIsogenyFirstOrderEquation.to_secondOrder
#print axioms NormalizedIsogenyFirstOrderEquation.base_coefficients
#print axioms NormalizedRationalMapSeriesCertificate.to_firstOrder
#print axioms NormalizedRationalMapPolynomialCertificate.to_firstOrder
#print axioms PowerSeriesPrefixEq.derivative
#print axioms PowerSeriesPrefixEq.of_coeff_eq
#print axioms PowerSeriesPrefixEq.cancel_X
#print axioms PowerSeriesPrefixEq.cancel_left
#print axioms PowerSeriesPrefixEq.inv
#print axioms PowerSeriesPrefixEq.inv_sq
#print axioms PowerSeriesPrefixEq.X_pow_eq_zero
#print axioms PowerSeriesPrefixEq.add_X_pow_mul
#print axioms PowerSeriesPrefixEq.leading_X_inv_sq
#print axioms PowerSeriesPrefixEq.subst
#print axioms PowerSeriesPrefixEq.subst_inv
#print axioms PowerSeriesPrefixEq.leading_X_inv_sq_mul_subst_inv
#print axioms WeierstrassCoordinateUnit.constantCoeff
#print axioms WeierstrassCoordinateUnit.clearedCurveIdentity
#print axioms WeierstrassCoordinateUnit.rescale
#print axioms powerSeries_subst_scale
#print axioms WeierstrassCoordinateUnit.rescaled_subst_normalized
#print axioms WeierstrassCoordinateUnit.normalizedX_prefix
#print axioms WeierstrassCoordinateUnit.normalizedXRatio_prefix
#print axioms prime_ge_twentyThree
#print axioms frobeniusNormalizedCorrection_invisible
#print axioms frobeniusNormalizedCorrection_even_coeff_zero
#print axioms NormalizedRationalMapJetCertificate.to_firstOrderPrefix
#print axioms NormalizedIsogenyFirstOrderPrefix.to_secondOrderPrefix
#print axioms NormalizedIsogenyFirstOrderPrefix.base_coefficients
#print axioms BostanJetRecurrence.of_normalizedSecondOrderEquation
#print axioms BostanJetRecurrence.of_normalizedSecondOrderPrefix
#print axioms BostanJetRecurrence.of_normalizedSecondOrderEquation_zmod
#print axioms BostanJetRecurrence.of_normalizedFirstOrderEquation
#print axioms BostanJetRecurrence.of_normalizedFirstOrderPrefix
#print axioms BostanJetRecurrence.of_normalizedFirstOrderEquation_zmod
#print axioms BostanJetRecurrence.of_normalizedRationalMapPolynomialCertificate_zmod
#print axioms BostanJetRecurrence.of_normalizedRationalMapJetCertificate_zmod
#print axioms BostanJetRecurrence.unique
#print axioms fiberDenominatorSeries_coeff_eq_of_bostanRecurrence

end H116FiberTrace
