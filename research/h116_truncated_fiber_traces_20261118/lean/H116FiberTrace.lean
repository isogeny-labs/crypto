import Mathlib.Algebra.Polynomial.Derivation
import Mathlib.Algebra.Polynomial.Roots
import Mathlib.Algebra.Group.Hom.End
import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.AlgebraicGeometry.EllipticCurve.DivisionPolynomial.Degree
import Mathlib.Data.ZMod.Basic
import Mathlib.FieldTheory.RatFunc.Basic
import Mathlib.FieldTheory.RatFunc.IntermediateField
import Mathlib.FieldTheory.Finite.Basic
import Mathlib.FieldTheory.IsAlgClosed.Basic
import Mathlib.FieldTheory.IntermediateField.Adjoin.Basic
import Mathlib.LinearAlgebra.Dimension.Free
import Mathlib.RingTheory.FormalGroup.Basic
import Mathlib.RingTheory.LaurentSeries
import Mathlib.RingTheory.Polynomial.Resultant.Basic
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

/-- The first two reciprocal coefficients used by the native producer depend
only on the target coordinate `u`; every higher shared-jet coefficient is
irrelevant at this order. -/
theorem fiberDenominatorReciprocal_coeff_zero_one
    (u : K) (h : ℕ → K) (g : PowerSeries K)
    (hfg : fiberDenominatorSeries u h * g = 1) :
    PowerSeries.coeff 0 g = 1 ∧ PowerSeries.coeff 1 g = u := by
  have hzero := congrArg (PowerSeries.coeff 0) hfg
  have hg0 : PowerSeries.coeff 0 g = 1 := by
    simpa [PowerSeries.coeff_mul] using hzero
  refine ⟨hg0, ?_⟩
  have hone := fiberDenominatorReciprocalRecurrence u h g hfg 0
  simpa [hg0] using hone

/-- The selected second logarithmic-derivative coefficient has the closed
form `2*h₁-u²`. -/
theorem fiberLogDerivative_coeff_one
    (u : K) (h : ℕ → K) (g : PowerSeries K)
    (hfg : fiberDenominatorSeries u h * g = 1) :
    PowerSeries.coeff 1
        (PowerSeries.derivative (fiberDenominatorSeries u h) * g) =
      2 * h 1 - u ^ 2 := by
  rcases fiberDenominatorReciprocal_coeff_zero_one u h g hfg with ⟨hg0, hg1⟩
  rw [PowerSeries.coeff_one_mul]
  simp only [← PowerSeries.coeff_zero_eq_constantCoeff, hg0, hg1,
    fiberDenominatorSeries_derivative_coeff_zero,
    fiberDenominatorSeries_derivative_coeff_succ]
  ring

/-- In the relative selected channel, the shared `h₁` term cancels exactly.
This proves that the observed `IX1` ordering rule reduces to a scaled
x-coordinate square difference and does not use the higher H116 jet. -/
theorem fiberRelativeLogDerivative_coeff_one
    (u u₀ : K) (h : ℕ → K) (g g₀ : PowerSeries K)
    (hfg : fiberDenominatorSeries u h * g = 1)
    (hfg₀ : fiberDenominatorSeries u₀ h * g₀ = 1) :
    PowerSeries.coeff 1
        (PowerSeries.derivative (fiberDenominatorSeries u₀ h) * g₀ -
          PowerSeries.derivative (fiberDenominatorSeries u h) * g) =
      u ^ 2 - u₀ ^ 2 := by
  rw [map_sub, fiberLogDerivative_coeff_one u₀ h g₀ hfg₀,
    fiberLogDerivative_coeff_one u h g hfg]
  ring

/-- The correction left when two first shared-jet coefficients differ.  This
is a mutation control for the shared-jet premise above. -/
theorem fiberRelativeLogDerivative_coeff_one_unshared
    (u u₀ h₁ h₁₀ : K) :
    (2 * h₁₀ - u₀ ^ 2) - (2 * h₁ - u ^ 2) =
      (u ^ 2 - u₀ ^ 2) + 2 * (h₁₀ - h₁) := by
  ring

/-- Euclidean base-six decomposition transported through natural scalar
multiplication. This is the unconditional group identity behind the public
quotient-digit construction. -/
theorem quotientDigit_nsmul_identity
    {G : Type*} [AddCommMonoid G] (P : G) (k : ℕ) :
    k • P = 6 • ((k / 6) • P) + (k % 6) • P := by
  calc
    k • P = (6 * (k / 6) + k % 6) • P := by congr 1; omega
    _ = (6 * (k / 6)) • P + (k % 6) • P := by rw [add_nsmul]
    _ = 6 • ((k / 6) • P) + (k % 6) • P := by
      rw [mul_nsmul, nsmul_left_comm]

/-- Subtracting the true base-six digit leaves exactly six times the quotient
point. -/
theorem quotientDigit_sub_identity
    {G : Type*} [AddCommGroup G] (P : G) (k : ℕ) :
    (k • P) - (k % 6) • P = 6 • ((k / 6) • P) := by
  rw [quotientDigit_nsmul_identity]
  abel

/-- Each non-base-case quotient step strictly decreases the canonical natural
scalar, so an exact digit oracle would terminate. -/
theorem quotientDigit_strict_decrease (k : ℕ) (hk : 6 ≤ k) : k / 6 < k := by
  omega

/-- Subtraction from an odd natural reverses parity.  This is the arithmetic
half of the x-only Kummer-cover obstruction. -/
theorem odd_sub_mod_two_ne (n k : ℕ) (hn : n % 2 = 1) (hk : k ≤ n) :
    (n - k) % 2 ≠ k % 2 := by
  omega

/-- In a cyclic orbit killed by `n`, the scalar paired with `k` under
subtraction from `n` is the negative point. -/
theorem orbit_sub_eq_neg
    {G : Type*} [AddCommGroup G] (P : G) (n k : ℕ)
    (hk : k ≤ n) (hnP : n • P = 0) :
    (n - k) • P = -(k • P) := by
  apply eq_neg_of_add_eq_zero_left
  rw [← add_nsmul, Nat.sub_add_cancel hk, hnP]

/-- Any decoder fed only an observable invariant under point negation fails on
at least one member of every odd-order scalar pair `k,n-k`.  Since an elliptic
x-coordinate and every quadratic cover in `H116-KUMMER-PARITY-001` are
negation-invariant, none can be a total exact low-parity oracle. -/
theorem negInvariant_binaryDecoder_pair_failure
    {G A : Type*} [AddCommGroup G]
    (P : G) (n k : ℕ) (observable : G → A) (decode : A → ℕ)
    (hn : n % 2 = 1) (hk : k ≤ n) (hnP : n • P = 0)
    (hneg : ∀ Q, observable (-Q) = observable Q) :
    decode (observable (k • P)) ≠ k % 2 ∨
      decode (observable ((n - k) • P)) ≠ (n - k) % 2 := by
  have horbit := orbit_sub_eq_neg P n k hk hnP
  have hobs : observable ((n - k) • P) = observable (k • P) := by
    rw [horbit, hneg]
  by_contra h
  push Not at h
  have hparity : (n - k) % 2 = k % 2 := by
    calc
      (n - k) % 2 = decode (observable ((n - k) • P)) := h.2.symm
      _ = decode (observable (k • P)) := congrArg decode hobs
      _ = k % 2 := h.1
  exact odd_sub_mod_two_ne n k hn hk hparity

/-- Oddness is a necessary boundary: subtraction from the even order four
does not reverse the parity of the paired indices one and three. -/
theorem evenOrder_parityMutation_control : (4 - 1) % 2 = 1 % 2 := by
  norm_num

/-- Multiplying an integer character sum by either quadratic-twist sign does
not change the resulting trace modulo two.  This is the arithmetic core of
the genus-three cover-trace obstruction. -/
theorem signedTwist_traceParity_invariant (trace sum epsilon : ℤ)
    (hepsilon : epsilon = 1 ∨ epsilon = -1) :
    (trace - epsilon * sum) % 2 = (trace - sum) % 2 := by
  rcases hepsilon with rfl | rfl <;> omega

/-- A sum that changes under point negation only by a sign has
negation-invariant trace parity.  The concrete genus-three character sum
supplies `epsilon = chi(-1)`. -/
theorem signedTwist_traceParity_negInvariant
    {G : Type*} [AddCommGroup G]
    (trace : ℤ) (sum : G → ℤ) (epsilon : G → ℤ)
    (hepsilon : ∀ Q, epsilon Q = 1 ∨ epsilon Q = -1)
    (htwist : ∀ Q, sum (-Q) = epsilon Q * sum Q) :
    ∀ Q, (trace - sum (-Q)) % 2 = (trace - sum Q) % 2 := by
  intro Q
  rw [htwist Q]
  exact signedTwist_traceParity_invariant trace (sum Q) (epsilon Q) (hepsilon Q)

/-- Consequently, no deterministic decoder of signed-twist trace parity can
recover scalar parity on both members of an odd-order paired orbit. -/
theorem signedTwist_traceParity_decoder_pair_failure
    {G : Type*} [AddCommGroup G]
    (P : G) (n k : ℕ) (trace : ℤ) (sum : G → ℤ) (epsilon : G → ℤ)
    (decode : ℤ → ℕ)
    (hn : n % 2 = 1) (hk : k ≤ n) (hnP : n • P = 0)
    (hepsilon : ∀ Q, epsilon Q = 1 ∨ epsilon Q = -1)
    (htwist : ∀ Q, sum (-Q) = epsilon Q * sum Q) :
    decode ((trace - sum (k • P)) % 2) ≠ k % 2 ∨
      decode ((trace - sum ((n - k) • P)) % 2) ≠ (n - k) % 2 := by
  let observable : G → ℤ := fun Q => (trace - sum Q) % 2
  apply negInvariant_binaryDecoder_pair_failure P n k observable decode hn hk hnP
  intro Q
  dsimp only [observable]
  exact signedTwist_traceParity_negInvariant trace sum epsilon hepsilon htwist Q

/-- Mutation control: unlike parity, an odd residue can distinguish the two
twist signs.  This is why the next experiment moves to modulus three. -/
theorem oddModulus_signedTwistMutation_control :
    ((0 : ℤ) - (-1) * 1) % 3 ≠ ((0 : ℤ) - 1) % 3 := by
  norm_num

/-- The degree-one and degree-two coefficient convolutions for an elliptic
Weil factor `(1,-t,p)` and a Prym-surface factor `(1,-tau,eta)`. -/
theorem ellipticPrym_lowCoefficient_convolution (p t tau eta : K) :
    (-t) + (-tau) = -(t + tau) ∧
      p + (-t) * (-tau) + eta = p + t * tau + eta := by
  constructor <;> ring

/-- If the first two coefficients of a genus-three Weil polynomial are
supplied by the elliptic--Prym product, the two leading Prym coefficients are
recovered by the formulas used in the native experiment.  Supplying the
geometric isogeny and hence these coefficient equations remains an explicit
endpoint premise. -/
theorem prymCoefficients_of_productLowCoefficients
    (p t s1 e2 tau eta : K)
    (hs1 : -s1 = (-t) + (-tau))
    (he2 : e2 = p + (-t) * (-tau) + eta) :
    tau = s1 - t ∧ eta = e2 - p - t * (s1 - t) := by
  have hs1' : s1 = tau + t := by
    simpa only [neg_neg, neg_add_rev] using congrArg Neg.neg hs1
  have htau : tau = s1 - t := by
    calc
      tau = (tau + t) - t := by abel
      _ = s1 - t := by rw [← hs1']
  have he2' : e2 = p + t * tau + eta := by
    simpa only [neg_mul_neg] using he2
  constructor
  · exact htau
  · rw [htau] at he2'
    calc
      eta = (p + t * (s1 - t) + eta) - p - t * (s1 - t) := by abel
      _ = e2 - p - t * (s1 - t) := by rw [← he2']

/-- The discriminant of the Prym real Weil polynomial in the two equivalent
normal forms used by the experiment. -/
theorem prymRealWeil_discriminant_normalized (p tau eta : K) :
    tau ^ 2 - 4 * (eta - 2 * p) = tau ^ 2 - 4 * eta + 8 * p := by
  ring

/-- The reciprocal degree-four Prym Weil polynomial is the homogenization of
its degree-two real Weil polynomial.  This is the division-free form of the
identity checked by the auxiliary-eigenline experiment. -/
theorem prymWeil_homogenized_realWeil (p tau eta T : K) :
    T ^ 4 - tau * T ^ 3 + eta * T ^ 2 - p * tau * T + p ^ 2 =
      (T ^ 2 + p) ^ 2 - tau * T * (T ^ 2 + p) + (eta - 2 * p) * T ^ 2 := by
  ring

/-- At a nonzero auxiliary eigenvalue, the homogenized identity becomes
`F(T)=T^2 W(T+p/T)`, exactly the quotient identity used as a native control. -/
theorem prymWeil_eq_sq_mul_realWeil (p tau eta T : K) (hT : T ≠ 0) :
    T ^ 4 - tau * T ^ 3 + eta * T ^ 2 - p * tau * T + p ^ 2 =
      T ^ 2 * ((T + p / T) ^ 2 - tau * (T + p / T) + (eta - 2 * p)) := by
  field_simp [hT]
  ring

/-- Closed form for the discriminant of the residual quadratic obtained after
removing the known target intersection from the chiral line cubic. -/
theorem chiralBranchResidual_discriminant_closed (a x y c : K) :
    (x - 1) ^ 2 - 4 * (a + 2 * (x + c * y) + x ^ 2 - x) =
      -3 * x ^ 2 - 6 * x + 1 - 4 * a - 8 * c * y := by
  ring

/-- Simultaneously negating the point and chirality preserves the residual
branch discriminant, matching the exact native chiral-swap control. -/
theorem chiralBranchResidual_negationSwap (a x y c : K) :
    -3 * x ^ 2 - 6 * x + 1 - 4 * a - 8 * (-c) * (-y) =
      -3 * x ^ 2 - 6 * x + 1 - 4 * a - 8 * c * y := by
  ring

/-- If a monic cubic is a linear factor times a monic quadratic, its
discriminant is the square of the quadratic evaluated at the linear root times
the quadratic discriminant.  This is the singularity gate used by the branch
experiment. -/
theorem linearQuadratic_discriminant_factorization (x q0 q1 : K) :
    (q1 - x) ^ 2 * (q0 - x * q1) ^ 2
        - 4 * (q0 - x * q1) ^ 3
        - 4 * (q1 - x) ^ 3 * (-x * q0)
        - 27 * (-x * q0) ^ 2
        + 18 * (q1 - x) * (q0 - x * q1) * (-x * q0) =
      (x ^ 2 + q1 * x + q0) ^ 2 * (q1 ^ 2 - 4 * q0) := by
  ring

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

/-- In a possibly noncommutative endomorphism ring, the Frobenius
characteristic equation and norm relation imply `V + pi = trace` once left
composition by `pi` is cancellable and the scalar trace commutes with `pi`. -/
theorem frobeniusVerschiebung_sum_of_characteristic
    {R : Type*} [Ring R] {pi verschiebung trace cardinality : R}
    (hpiCancel : ∀ a b : R, pi * a = pi * b → a = b)
    (htraceComm : Commute pi trace)
    (hnorm : pi * verschiebung = cardinality)
    (hchar : pi ^ 2 - trace * pi + cardinality = 0) :
    verschiebung + pi = trace := by
  apply hpiCancel
  rw [mul_add, hnorm]
  calc
    cardinality + pi * pi = pi ^ 2 + cardinality := by
      rw [pow_two, add_comm]
    _ = trace * pi := by
      apply eq_of_sub_eq_zero
      calc
        pi ^ 2 + cardinality - trace * pi =
            pi ^ 2 - trace * pi + cardinality := by abel
        _ = 0 := hchar
    _ = pi * trace := htraceComm.eq.symm

/-- The norm and sum relations factor the scalar point count exactly:
`(V-1)(pi-1)=p+1-trace`.  This ring identity is the algebraic input for the
expected degree of `V-1`; converting it into a morphism-degree statement is a
separate elliptic-geometric obligation. -/
theorem verschiebung_sub_one_mul_frobenius_sub_one
    {R : Type*} [Ring R]
    (verschiebung pi trace cardinality : R)
    (hnorm : verschiebung * pi = cardinality)
    (hsum : verschiebung + pi = trace) :
    (verschiebung - 1) * (pi - 1) = cardinality + 1 - trace := by
  calc
    (verschiebung - 1) * (pi - 1) =
        verschiebung * pi - (verschiebung + pi) + 1 := by noncomm_ring
    _ = cardinality - trace + 1 := by rw [hnorm, hsum]
    _ = cardinality + 1 - trace := by abel

/-- A multiplicative degree whose two factors have equal degree turns a scalar
norm factorization into the exact degree of either factor.  This is the
arithmetic core of the elliptic argument: for actual isogenies, `hpaired` is
supplied by duality, `hmul` by multiplicativity of isogeny degree, and
`hscalar` by `deg [N] = N^2`. -/
theorem degree_eq_norm_of_paired_factor
    {M : Type*} [Mul M] (degree : M → ℕ) (left right scalar : M) (norm : ℕ)
    (hfactor : left * right = scalar)
    (hmul : ∀ f g : M, degree (f * g) = degree f * degree g)
    (hscalar : degree scalar = norm * norm)
    (hpaired : degree left = degree right) :
    degree left = norm := by
  have hsquare : degree left * degree left = norm * norm := by
    calc
      degree left * degree left = degree left * degree right := by rw [hpaired]
      _ = degree (left * right) := (hmul left right).symm
      _ = degree scalar := congrArg degree hfactor
      _ = norm * norm := hscalar
  apply le_antisymm
  · by_contra hnot
    have hlt : norm < degree left := Nat.lt_of_not_ge hnot
    nlinarith
  · by_contra hnot
    have hlt : degree left < norm := Nat.lt_of_not_ge hnot
    nlinarith

/-- Exact certificate interface for the missing H116 geometric bridge.  The
already-proved Frobenius--Verschiebung norm identity discharges the factor
premise.  A caller representing genuine elliptic isogenies must still prove
degree multiplicativity, scalar degree, and equality of the dual-pair degrees. -/
theorem verschiebung_sub_one_degree_of_dual_certificate
    {R : Type*} [Ring R]
    (degree : R → ℕ) (verschiebung pi trace cardinality : R) (pointCount : ℕ)
    (hnorm : verschiebung * pi = cardinality)
    (hsum : verschiebung + pi = trace)
    (hmul : ∀ f g : R, degree (f * g) = degree f * degree g)
    (hscalar : degree (cardinality + 1 - trace) = pointCount * pointCount)
    (hdualDegree : degree (verschiebung - 1) = degree (pi - 1)) :
    degree (verschiebung - 1) = pointCount := by
  apply degree_eq_norm_of_paired_factor degree
      (verschiebung - 1) (pi - 1) (cardinality + 1 - trace) pointCount
  · exact verschiebung_sub_one_mul_frobenius_sub_one
      verschiebung pi trace cardinality hnorm hsum
  · exact hmul
  · exact hscalar
  · exact hdualDegree

/-- Mutation control: product degree `N^2` alone does not determine either
factor degree.  At `N=6`, unequal candidate degrees `4` and `9` have the right
product, so the dual/equal-degree premise above is logically necessary. -/
theorem degree_product_without_duality_control :
    (4 : ℕ) * 9 = 6 * 6 ∧ (4 : ℕ) ≠ 6 ∧ (9 : ℕ) ≠ 6 := by
  norm_num

/-- Function-field degrees multiply in a finite field tower. This discharges
the abstract multiplicativity arithmetic once concrete pullbacks from two
dominant curve maps have supplied the three fields and their scalar-tower
structure. Constructing that tower from `V-1`, `pi-1`, and their composition
remains a separate geometric obligation. -/
theorem finiteFieldTower_degree_mul
    {F E M : Type*} [Field F] [Field E] [Field M]
    [Algebra F E] [Algebra E M] [Algebra F M] [IsScalarTower F E M] :
    Module.finrank F M = Module.finrank F E * Module.finrank E M := by
  exact (Module.finrank_mul_finrank F E M).symm

/-- In a finite function-field tower, equal adjacent degrees and total degree
`N²` force each adjacent degree to be `N`. This is the field-tower version of
the paired-factor arithmetic used by H116; it deliberately assumes rather
than invents the pullback tower associated to the elliptic maps. -/
theorem finiteFieldTower_degree_eq_norm_of_paired
    {F E M : Type*} [Field F] [Field E] [Field M]
    [Algebra F E] [Algebra E M] [Algebra F M] [IsScalarTower F E M]
    (norm : ℕ)
    (hpaired : Module.finrank F E = Module.finrank E M)
    (htotal : Module.finrank F M = norm * norm) :
    Module.finrank F E = norm := by
  have hsquare : Module.finrank F E * Module.finrank F E = norm * norm := by
    calc
      Module.finrank F E * Module.finrank F E =
          Module.finrank F E * Module.finrank E M := by rw [hpaired]
      _ = Module.finrank F M :=
        (finiteFieldTower_degree_mul (F := F) (E := E) (M := M)).symm
      _ = norm * norm := htotal
  apply le_antisymm
  · by_contra hnot
    have hlt : norm < Module.finrank F E := Nat.lt_of_not_ge hnot
    nlinarith
  · by_contra hnot
    have hlt : Module.finrank F E < norm := Nat.lt_of_not_ge hnot
    nlinarith

/-- Quadratic function-field sandwich.  Suppose the pulled-back target x-line
is the common base `B`.  The source tower is `B -> Xs -> E`, while the target
tower is `B -> Xt -> E`.  If both curve fields are quadratic over their
x-lines, then the degree of the full curve-field pullback `E/Xt` equals the
coordinate extension degree `Xs/B`.

This theorem consumes already-constructed field embeddings.  In particular,
it does not construct the function fields or prove that a rational coordinate
formula induces the stated extension. -/
theorem quadraticFunctionFieldSandwich_degree
    {B Xs Xt E : Type*}
    [Field B] [Field Xs] [Field Xt] [Field E]
    [Algebra B Xs] [Algebra Xs E] [Algebra B E]
    [Algebra B Xt] [Algebra Xt E]
    [IsScalarTower B Xs E] [IsScalarTower B Xt E]
    (coordinateDegree : ℕ)
    (hsourceQuadratic : Module.finrank Xs E = 2)
    (htargetQuadratic : Module.finrank B Xt = 2)
    (hcoordinate : Module.finrank B Xs = coordinateDegree) :
    Module.finrank Xt E = coordinateDegree := by
  have hbalanced : coordinateDegree * 2 = 2 * Module.finrank Xt E := by
    calc
      coordinateDegree * 2 = Module.finrank B Xs * Module.finrank Xs E := by
        rw [hcoordinate, hsourceQuadratic]
      _ = Module.finrank B E :=
        (finiteFieldTower_degree_mul (F := B) (E := Xs) (M := E)).symm
      _ = Module.finrank B Xt * Module.finrank Xt E :=
        finiteFieldTower_degree_mul (F := B) (E := Xt) (M := E)
      _ = 2 * Module.finrank Xt E := by rw [htargetQuadratic]
  omega

/-- The rational-coordinate extension degree is exactly the maximum degree of
the canonical reduced numerator and denominator.  This is the precise
coordinate-line input consumed by `quadraticFunctionFieldSandwich_degree`.
For a quotient supplied as two separate polynomials, proving that no common
factor is cancelled remains a separate obligation. -/
theorem rationalFunctionCoordinateExtension_degree
    {F : Type*} [Field F] (f : RatFunc F) :
    Module.finrank (IntermediateField.adjoin F {f}) (RatFunc F) =
      max f.num.natDegree f.denom.natDegree := by
  exact RatFunc.finrank_eq_max_natDegree f

/-- A displayed polynomial quotient that is already coprime retains the exact
numerator and denominator degrees after Mathlib's canonical `RatFunc`
normalization.  The normalization divides by the polynomial gcd and then
multiplies both sides by a nonzero constant to make the denominator monic. -/
theorem reducedRationalQuotient_degrees
    {F : Type*} [Field F] (p q : Polynomial F)
    (hq : q ≠ 0) (hcoprime : IsCoprime p q) :
    let f : RatFunc F :=
      algebraMap (Polynomial F) (RatFunc F) p /
        algebraMap (Polynomial F) (RatFunc F) q
    f.num.natDegree = p.natDegree ∧
      f.denom.natDegree = q.natDegree := by
  classical
  let f : RatFunc F :=
    algebraMap (Polynomial F) (RatFunc F) p /
      algebraMap (Polynomial F) (RatFunc F) q
  have hgcdUnit : IsUnit (GCDMonoid.gcd p q) :=
    gcd_isUnit_iff_isRelPrime.mpr hcoprime.isRelPrime
  have hgcd : GCDMonoid.gcd p q = 1 := by
    rw [← normalize_gcd p q, normalize_eq_one]
    exact hgcdUnit
  have hlead : q.leadingCoeff ≠ 0 := Polynomial.leadingCoeff_ne_zero.mpr hq
  change f.num.natDegree = p.natDegree ∧
    f.denom.natDegree = q.natDegree
  constructor
  · dsimp only [f]
    rw [RatFunc.num_div, hgcd]
    simp only [EuclideanDomain.div_one]
    exact Polynomial.natDegree_C_mul (inv_ne_zero hlead)
  · dsimp only [f]
    rw [RatFunc.denom_div p hq, hgcd]
    simp only [EuclideanDomain.div_one]
    exact Polynomial.natDegree_C_mul (inv_ne_zero hlead)

/-- Combining coprime displayed polynomials with the canonical rational-
function degree theorem gives the exact coordinate extension degree.  This is
the no-cancellation bridge needed to instantiate the quadratic sandwich once a
particular numerator and denominator are proved coprime. -/
theorem reducedRationalQuotient_extension_degree
    {F : Type*} [Field F] (p q : Polynomial F)
    (hq : q ≠ 0) (hcoprime : IsCoprime p q) :
    let f : RatFunc F :=
      algebraMap (Polynomial F) (RatFunc F) p /
        algebraMap (Polynomial F) (RatFunc F) q
    Module.finrank (IntermediateField.adjoin F {f}) (RatFunc F) =
      max p.natDegree q.natDegree := by
  let f : RatFunc F :=
    algebraMap (Polynomial F) (RatFunc F) p /
      algebraMap (Polynomial F) (RatFunc F) q
  have hdegree := reducedRationalQuotient_degrees p q hq hcoprime
  change Module.finrank (IntermediateField.adjoin F {f}) (RatFunc F) =
    max p.natDegree q.natDegree
  rw [rationalFunctionCoordinateExtension_degree f, hdegree.1, hdegree.2]

open Polynomial

/-- An explicit Bezout certificate for the scalar-two numerator and the
short-Weierstrass cubic.  Its constant is the short discriminant factor
`4*A^3+27*B^2`, so nonsingularity makes the two polynomials coprime over any
field. -/
theorem shortWeierstrassScalarTwo_explicit_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0) :
    IsCoprime
      (X ^ 4 - C (2 * A) * X ^ 2 - C (8 * B) * X + C (A ^ 2))
      (X ^ 3 + C A * X + C B) := by
  let d : F := 4 * A ^ 3 + 27 * B ^ 2
  have hbezout :
      (C (4 * A) + C 3 * X ^ 2) *
          (X ^ 4 - C (2 * A) * X ^ 2 - C (8 * B) * X + C (A ^ 2)) +
        (C (5 * A) * X + C (27 * B) - C 3 * X ^ 3) *
          (X ^ 3 + C A * X + C B) = C d := by
    dsimp [d]
    simp only [map_add, map_mul, map_pow, map_ofNat]
    ring
  refine Exists.intro (C d⁻¹ * (C (4 * A) + C 3 * X ^ 2)) ?_
  refine Exists.intro
    (C d⁻¹ * (C (5 * A) * X + C (27 * B) - C 3 * X ^ 3)) ?_
  calc
    _ = C d⁻¹ *
        ((C (4 * A) + C 3 * X ^ 2) *
            (X ^ 4 - C (2 * A) * X ^ 2 - C (8 * B) * X + C (A ^ 2)) +
          (C (5 * A) * X + C (27 * B) - C 3 * X ^ 3) *
            (X ^ 3 + C A * X + C B)) := by ring
    _ = C d⁻¹ * C d := by rw [hbezout]
    _ = 1 := by
      rw [← map_mul]
      simp [d, hdisc]

/-- Mathlib's exact scalar-two division-polynomial numerator and denominator
are coprime on a nonsingular short-Weierstrass curve outside characteristic
two. -/
theorem shortWeierstrassScalarTwo_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.Φ 2) (W.ΨSq 2) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have hbase := shortWeierstrassScalarTwo_explicit_isCoprime A B hdisc
  have hfour : IsUnit (C (4 : F) : Polynomial F) := by
    rw [Polynomial.isUnit_C]
    apply isUnit_iff_ne_zero.mpr
    rw [show (4 : F) = (2 : F) * 2 by norm_num]
    exact mul_ne_zero h2 h2
  have hphi : W.Φ 2 =
      X ^ 4 - C (2 * A) * X ^ 2 - C (8 * B) * X + C (A ^ 2) := by
    rw [WeierstrassCurve.Φ_two]
    dsimp [W, WeierstrassCurve.b₄, WeierstrassCurve.b₆,
      WeierstrassCurve.b₈]
    norm_num
    simp only [Polynomial.C_ofNat]
    ring
  have hpsi : W.ΨSq 2 = C (4 : F) * (X ^ 3 + C A * X + C B) := by
    rw [WeierstrassCurve.ΨSq_two]
    dsimp [W, WeierstrassCurve.Ψ₂Sq, WeierstrassCurve.b₂,
      WeierstrassCurve.b₄, WeierstrassCurve.b₆]
    norm_num [Polynomial.C_mul, Polynomial.C_ofNat]
    ring
  change IsCoprime (W.Φ 2) (W.ΨSq 2)
  rw [hphi, hpsi]
  exact (isCoprime_mul_unit_left_right hfour _ _).2 hbase

/-- The reduced scalar-two x-coordinate quotient on a nonsingular short
Weierstrass curve has rational-function coordinate extension degree exactly
four.  Coprimality above closes the cancellation premise that was previously
external to the generic degree theorem. -/
theorem shortWeierstrassScalarTwo_extension_degree
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    let xMap : RatFunc F :=
      algebraMap (Polynomial F) (RatFunc F) (W.Φ 2) /
        algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 2)
    Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 4 := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  let xMap : RatFunc F :=
    algebraMap (Polynomial F) (RatFunc F) (W.Φ 2) /
      algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 2)
  have h2z : ((2 : ℤ) : F) ≠ 0 := by
    norm_num
    exact h2
  have hcoprime : IsCoprime (W.Φ 2) (W.ΨSq 2) :=
    shortWeierstrassScalarTwo_isCoprime A B hdisc h2
  have hdenom : W.ΨSq 2 ≠ 0 := W.ΨSq_ne_zero h2z
  have hextension := reducedRationalQuotient_extension_degree
    (W.Φ 2) (W.ΨSq 2) hdenom hcoprime
  have hdegrees :
      (W.Φ 2).natDegree = Int.natAbs 2 ^ 2 ∧
        (W.ΨSq 2).natDegree = Int.natAbs 2 ^ 2 - 1 :=
    ⟨W.natDegree_Φ 2, W.natDegree_ΨSq h2z⟩
  change Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 4
  rw [hextension]
  norm_num at hdegrees ⊢
  omega

/-- Mutation control for the sandwich theorem: if the two line-to-curve
degrees differ, equality of the total tower degrees does not identify the two
adjacent map degrees. -/
theorem unequalCoordinateLineDegrees_control :
    (4 : ℕ) * 3 = 2 * 6 ∧ (4 : ℕ) ≠ 6 := by
  norm_num

/-- Exact numerator and denominator degrees for Mathlib's univariate
division-polynomial x-map candidate `Φₙ/Ψₙ²`, provided the scalar is
nonzero in the coefficient field. Connecting this candidate to Mathlib's
pointwise scalar multiplication, and then its coordinate degree to the
scheme/function-field degree of `[n]`, are explicit remaining bridges. -/
theorem scalarMultiplicationXMap_exact_degrees
    {F : Type*} [Field F] (W : WeierstrassCurve F) (n : ℤ)
    (hn : (n : F) ≠ 0) :
    (W.Φ n).natDegree = n.natAbs ^ 2 ∧
      (W.ΨSq n).natDegree = n.natAbs ^ 2 - 1 := by
  exact ⟨W.natDegree_Φ n, W.natDegree_ΨSq hn⟩

/-- The same coordinate certificate has the expected leading coefficients:
`Φₙ` is monic and `Ψₙ²` has leading coefficient `n²`. -/
theorem scalarMultiplicationXMap_leadingCoefficients
    {F : Type*} [Field F] (W : WeierstrassCurve F) (n : ℤ)
    (hn : (n : F) ≠ 0) :
    (W.Φ n).leadingCoeff = 1 ∧
      (W.ΨSq n).leadingCoeff = (n : F) ^ 2 := by
  exact ⟨W.leadingCoeff_Φ n, W.leadingCoeff_ΨSq hn⟩

/-- Generic characteristic boundary: if the scalar vanishes in the
coefficient field and `|n|>1`, the denominator cannot attain the nominal
degree `n²-1`. This turns the excluded-characteristic premise into a
strict degree-collapse theorem, including the case where `Ψₙ²` is zero. -/
theorem scalarMultiplicationXMap_denominator_degree_lt_of_char_dvd
    {F : Type*} [Field F] (W : WeierstrassCurve F) (n : ℤ)
    (hn : (n : F) = 0) (hnAbs : 1 < n.natAbs) :
    (W.ΨSq n).natDegree < n.natAbs ^ 2 - 1 := by
  apply lt_of_le_of_ne (W.natDegree_ΨSq_le n)
  intro heq
  by_cases hzero : W.ΨSq n = 0
  · simp only [hzero, Polynomial.natDegree_zero] at heq
    have hsquare : 1 < n.natAbs ^ 2 := by nlinarith
    have hpositive : 0 < n.natAbs ^ 2 - 1 := Nat.sub_pos_of_lt hsquare
    omega
  · have hlead : (W.ΨSq n).leadingCoeff ≠ 0 :=
      Polynomial.leadingCoeff_ne_zero.mpr hzero
    apply hlead
    rw [← Polynomial.coeff_natDegree, heq, W.coeff_ΨSq n]
    simp [hn]

/-- Characteristic control for the preceding certificate: the nonvanishing
premise is genuinely unavailable for `[p]` over `F_p`. The degree theorem does
not permit silently applying the characteristic-zero formula in this case. -/
theorem scalarMultiplicationXMap_characteristic_control :
    (3 : ZMod 3) = 0 := by
  exact ZMod.natCast_self 3

/-- The univariate scalar-three division polynomial specialized to a short
Weierstrass equation. -/
noncomputable def shortWeierstrassScalarThreePsi
    (F : Type*) [CommRing F] (A B : F) : Polynomial F :=
  C 3 * X ^ 4 + C (6 * A) * X ^ 2 + C (12 * B) * X - C (A ^ 2)

/-- The short-Weierstrass cubic used by the scalar-three numerator. -/
noncomputable def shortWeierstrassScalarThreeCubic
    (F : Type*) [CommRing F] (A B : F) : Polynomial F :=
  X ^ 3 + C A * X + C B

/-- The auxiliary degree-six factor in the scalar-three numerator. -/
noncomputable def shortWeierstrassScalarThreeAux
    (F : Type*) [CommRing F] (A B : F) : Polynomial F :=
  X ^ 6 + C (5 * A) * X ^ 4 + C (20 * B) * X ^ 3 -
    C (5 * A ^ 2) * X ^ 2 - C (4 * A * B) * X -
    C (8 * B ^ 2) - C (A ^ 3)

/-- The scalar-three x-coordinate numerator in factored division-polynomial
form. -/
noncomputable def shortWeierstrassScalarThreePhi
    (F : Type*) [CommRing F] (A B : F) : Polynomial F :=
  X * shortWeierstrassScalarThreePsi F A B ^ 2 -
    C 8 * shortWeierstrassScalarThreeCubic F A B *
      shortWeierstrassScalarThreeAux F A B

/-- A degree-eight explicit Bezout certificate proves coprimality of the
scalar-three numerator and division polynomial on a nonsingular short curve
outside characteristic two. -/
theorem shortWeierstrassScalarThree_explicit_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    IsCoprime (shortWeierstrassScalarThreePhi F A B)
      (shortWeierstrassScalarThreePsi F A B) := by
  let d : F := 4 * A ^ 3 + 27 * B ^ 2
  let U : Polynomial F :=
    C 27 * (C (15 * A) * X + C (27 * B) + C 7 * X ^ 3)
  let V : Polynomial F :=
    -C (1024 * A ^ 4) - C (2499 * A ^ 3) * X ^ 2 -
      C (2487 * A ^ 2 * B) * X - C (1785 * A ^ 2) * X ^ 4 -
      C (7992 * A * B ^ 2) + C (2286 * A * B) * X ^ 3 +
      C (747 * A) * X ^ 6 - C (3924 * B ^ 2) * X ^ 2 +
      C (6057 * B) * X ^ 5 - C 63 * X ^ 8
  have hbezout :
      U * shortWeierstrassScalarThreePhi F A B +
        V * shortWeierstrassScalarThreePsi F A B = C (64 * d ^ 2) := by
    dsimp [U, V, shortWeierstrassScalarThreePhi,
      shortWeierstrassScalarThreePsi, shortWeierstrassScalarThreeCubic,
      shortWeierstrassScalarThreeAux, d]
    simp only [map_add, map_mul, map_pow, map_ofNat]
    ring
  have h64 : (64 : F) ≠ 0 := by
    rw [show (64 : F) = (2 : F) ^ 6 by norm_num]
    exact pow_ne_zero 6 h2
  have hc : 64 * d ^ 2 ≠ 0 := mul_ne_zero h64 (pow_ne_zero 2 hdisc)
  refine Exists.intro (C (64 * d ^ 2)⁻¹ * U) ?_
  refine Exists.intro (C (64 * d ^ 2)⁻¹ * V) ?_
  calc
    _ = C (64 * d ^ 2)⁻¹ *
        (U * shortWeierstrassScalarThreePhi F A B +
          V * shortWeierstrassScalarThreePsi F A B) := by ring
    _ = C (64 * d ^ 2)⁻¹ * C (64 * d ^ 2) := by rw [hbezout]
    _ = C ((64 * d ^ 2)⁻¹ * (64 * d ^ 2)) := Polynomial.C_mul.symm
    _ = 1 := by rw [inv_mul_cancel₀ hc, Polynomial.C_1]

/-- Mathlib's exact scalar-three division-polynomial numerator and squared
denominator are coprime on a nonsingular short-Weierstrass curve outside
characteristic two. -/
theorem shortWeierstrassScalarThree_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.Φ 3) (W.ΨSq 3) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have hq : W.Ψ₃ = shortWeierstrassScalarThreePsi F A B := by
    dsimp [W, WeierstrassCurve.Ψ₃, WeierstrassCurve.b₂,
      WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈,
      shortWeierstrassScalarThreePsi]
    norm_num [Polynomial.C_mul, Polynomial.C_ofNat]
    ring
  have hcubic : W.Ψ₂Sq =
      C (4 : F) * shortWeierstrassScalarThreeCubic F A B := by
    dsimp [W, WeierstrassCurve.Ψ₂Sq, WeierstrassCurve.b₂,
      WeierstrassCurve.b₄, WeierstrassCurve.b₆,
      shortWeierstrassScalarThreeCubic]
    norm_num [Polynomial.C_mul, Polynomial.C_ofNat]
    ring
  have haux : W.preΨ₄ =
      C (2 : F) * shortWeierstrassScalarThreeAux F A B := by
    dsimp [W, WeierstrassCurve.preΨ₄, WeierstrassCurve.b₂,
      WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈,
      shortWeierstrassScalarThreeAux]
    norm_num [Polynomial.C_mul, Polynomial.C_ofNat]
    ring
  have hphi : W.Φ 3 = shortWeierstrassScalarThreePhi F A B := by
    rw [WeierstrassCurve.Φ_three, hq, haux, hcubic]
    dsimp [shortWeierstrassScalarThreePhi]
    norm_num [Polynomial.C_mul, Polynomial.C_ofNat]
    ring
  have hpsi : W.ΨSq 3 = shortWeierstrassScalarThreePsi F A B ^ 2 := by
    rw [WeierstrassCurve.ΨSq_three, hq]
  have hbase := shortWeierstrassScalarThree_explicit_isCoprime A B hdisc h2
  change IsCoprime (W.Φ 3) (W.ΨSq 3)
  rw [hphi, hpsi]
  exact hbase.pow_right

/-- The reduced scalar-three x-coordinate quotient has rational-function
coordinate extension degree exactly nine outside characteristics two and
three. -/
theorem shortWeierstrassScalarThree_extension_degree
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) (h3 : (3 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    let xMap : RatFunc F :=
      algebraMap (Polynomial F) (RatFunc F) (W.Φ 3) /
        algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 3)
    Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 9 := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  let xMap : RatFunc F :=
    algebraMap (Polynomial F) (RatFunc F) (W.Φ 3) /
      algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 3)
  have h3z : ((3 : ℤ) : F) ≠ 0 := by
    norm_num
    exact h3
  have hcoprime : IsCoprime (W.Φ 3) (W.ΨSq 3) :=
    shortWeierstrassScalarThree_isCoprime A B hdisc h2
  have hdenom : W.ΨSq 3 ≠ 0 := W.ΨSq_ne_zero h3z
  have hextension := reducedRationalQuotient_extension_degree
    (W.Φ 3) (W.ΨSq 3) hdenom hcoprime
  have hdegrees := scalarMultiplicationXMap_exact_degrees W 3 h3z
  change Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 9
  rw [hextension]
  norm_num at hdegrees ⊢
  omega

/-- Singular negative control: at `A=B=0`, the displayed scalar-three
numerator and squared denominator both vanish at `X=0`, so they cannot be
coprime. -/
theorem shortWeierstrassScalarThree_singular_control :
    ¬IsCoprime
      (shortWeierstrassScalarThreePhi (ZMod 5) 0 0)
      (shortWeierstrassScalarThreePsi (ZMod 5) 0 0 ^ 2) := by
  let _ : Fact (Nat.Prime 5) := Fact.mk (by norm_num)
  intro h
  have heval := h.map (Polynomial.evalRingHom (0 : ZMod 5))
  norm_num [shortWeierstrassScalarThreePhi,
    shortWeierstrassScalarThreePsi, shortWeierstrassScalarThreeCubic,
    shortWeierstrassScalarThreeAux] at heval

/-- Removing a multiple of the denominator reduces numerator coprimality to
coprimality of the three remaining factors. -/
theorem subMultiple_isCoprime_of_factors
    {R : Type*} [CommRing R] (x m a b c : R)
    (ha : IsCoprime a m) (hb : IsCoprime b m) (hc : IsCoprime c m) :
    IsCoprime (x * m - a * b * c) m := by
  rw [IsCoprime.mul_sub_right_left_iff]
  exact (ha.mul_left hb).mul_left hc

/-- Exact generic no-cancellation reduction for Mathlib's division
polynomials: it is enough to prove that the two adjacent EDS terms and the
parity factor are each coprime to the full squared denominator. -/
theorem divisionPolynomialPhi_isCoprime_of_factors
    {R : Type*} [CommRing R] (W : WeierstrassCurve R) (n : ℤ)
    (hplus : IsCoprime (W.preΨ (n + 1)) (W.ΨSq n))
    (hminus : IsCoprime (W.preΨ (n - 1)) (W.ΨSq n))
    (hparity : IsCoprime (if Even n then 1 else W.Ψ₂Sq) (W.ΨSq n)) :
    IsCoprime (W.Φ n) (W.ΨSq n) := by
  rw [WeierstrassCurve.Φ]
  exact subMultiple_isCoprime_of_factors Polynomial.X (W.ΨSq n)
    (W.preΨ (n + 1)) (W.preΨ (n - 1))
    (if Even n then 1 else W.Ψ₂Sq) hplus hminus hparity

/-- Mutation control for the generic reduction: keeping the second and third
factor certificates cannot compensate for a first factor sharing the
denominator. -/
theorem divisionPolynomialCoprimality_missingFactor_control :
    IsCoprime (1 : ℤ) 2 ∧ IsCoprime (1 : ℤ) 2 ∧
      ¬IsCoprime ((0 : ℤ) * 2 - 2 * 1 * 1) 2 := by
  norm_num [IsCoprime]

/-- For a normalized EDS whose recurrence parameter is a square `s^2`, every
odd term is coprime to `s` as soon as the third term is. -/
theorem preNormEDS'_odd_isCoprime
    {R : Type*} [CommRing R] (s c d : R)
    (hc : IsCoprime c s) :
    ∀ n : ℕ, Odd n → IsCoprime (preNormEDS' (s ^ 2) c d n) s := by
  intro n
  induction n using Nat.strong_induction_on with
  | h n ih =>
      intro hn
      obtain ⟨k, rfl⟩ := hn.exists_bit1
      rcases k with _ | k
      · simpa using (isCoprime_one_left : IsCoprime (1 : R) s)
      rcases k with _ | m
      · simpa using hc
      rw [show 2 * (m + 2) + 1 = 2 * (m + 2) + 1 by rfl,
        preNormEDS'_odd]
      by_cases hm : Even m
      · simp only [ite_eq_left hm, mul_one]
        have hm1 : Odd (m + 1) := hm.add_one
        have hm3 : Odd (m + 3) := by
          simpa [show m + 3 = (m + 1) + 2 by omega] using
            hm1.add_even (even_two : Even (2 : ℕ))
        have h1 := ih (m + 1) (by omega) hm1
        have h3 := ih (m + 3) (by omega) hm3
        have hprod : IsCoprime
            (preNormEDS' (s ^ 2) c d (m + 1) *
              preNormEDS' (s ^ 2) c d (m + 3) ^ 3) s :=
          h1.mul_left (h3.pow_left (m := 3))
        rw [show preNormEDS' (s ^ 2) c d (m + 4) *
              preNormEDS' (s ^ 2) c d (m + 2) ^ 3 * s ^ 2 -
              preNormEDS' (s ^ 2) c d (m + 1) *
                preNormEDS' (s ^ 2) c d (m + 3) ^ 3 =
            (preNormEDS' (s ^ 2) c d (m + 4) *
              preNormEDS' (s ^ 2) c d (m + 2) ^ 3 * s) * s -
              (preNormEDS' (s ^ 2) c d (m + 1) *
                preNormEDS' (s ^ 2) c d (m + 3) ^ 3) by ring]
        rwa [IsCoprime.mul_sub_right_left_iff]
      · have hmOdd : Odd m := Nat.not_even_iff_odd.mp hm
        simp only [ite_eq_right hm, mul_one]
        have hm2 : Odd (m + 2) := by
          exact hmOdd.add_even (even_two : Even (2 : ℕ))
        have hm4 : Odd (m + 4) := by
          exact hmOdd.add_even (by decide : Even (4 : ℕ))
        have h2 := ih (m + 2) (by omega) hm2
        have h4 := ih (m + 4) (by omega) hm4
        have hprod : IsCoprime
            (preNormEDS' (s ^ 2) c d (m + 4) *
              preNormEDS' (s ^ 2) c d (m + 2) ^ 3) s :=
          h4.mul_left (h2.pow_left (m := 3))
        rw [show preNormEDS' (s ^ 2) c d (m + 4) *
              preNormEDS' (s ^ 2) c d (m + 2) ^ 3 -
              preNormEDS' (s ^ 2) c d (m + 1) *
                preNormEDS' (s ^ 2) c d (m + 3) ^ 3 * s ^ 2 =
            (preNormEDS' (s ^ 2) c d (m + 4) *
              preNormEDS' (s ^ 2) c d (m + 2) ^ 3) -
              (preNormEDS' (s ^ 2) c d (m + 1) *
                preNormEDS' (s ^ 2) c d (m + 3) ^ 3 * s) * s by ring]
        rwa [IsCoprime.sub_mul_right_left_iff]

/-- Explicit short-curve Bezout certificate for the base odd EDS term
`Psi3` and the two-torsion factor `Psi2Sq`. -/
theorem shortWeierstrassPsiThree_PsiTwoSq_explicit_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    IsCoprime
      (3 * X ^ 4 + C (6 * A) * X ^ 2 + C (12 * B) * X - C (A ^ 2))
      (C 4 * (X ^ 3 + C A * X + C B)) := by
  let d : F := 4 * A ^ 3 + 27 * B ^ 2
  let s : Polynomial F := C 4 * (X ^ 3 + C A * X + C B)
  let q : Polynomial F :=
    3 * X ^ 4 + C (6 * A) * X ^ 2 + C (12 * B) * X - C (A ^ 2)
  let U : Polynomial F := C 3 * (C (7 * A) * X + C (9 * B) + C 3 * X ^ 3)
  let V : Polynomial F := -C 4 * (C (4 * A) + C 3 * X ^ 2)
  have hbezout : U * s + V * q = C (4 * d) := by
    dsimp [U, V, s, q, d]
    simp only [map_add, map_mul, map_pow, map_ofNat]
    ring
  have h4d : (4 : F) * d ≠ 0 := by
    apply mul_ne_zero
    · rw [show (4 : F) = 2 * 2 by norm_num]
      exact mul_ne_zero h2 h2
    · exact hdisc
  change IsCoprime q s
  refine ⟨C ((4 * d)⁻¹) * V, C ((4 * d)⁻¹) * U, ?_⟩
  calc
    _ = C ((4 * d)⁻¹) * (U * s + V * q) := by ring
    _ = C ((4 * d)⁻¹) * C (4 * d) := by rw [hbezout]
    _ = 1 := by
      rw [← Polynomial.C_mul, inv_mul_cancel₀ h4d, Polynomial.C_1]

/-- Mathlib's exact short-curve `Psi3` and `Psi2Sq` are coprime on a
nonsingular curve outside characteristic two. -/
theorem shortWeierstrassPsiThree_PsiTwoSq_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime W.Ψ₃ W.Ψ₂Sq := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have hbase := shortWeierstrassPsiThree_PsiTwoSq_explicit_isCoprime
    A B hdisc h2
  have hq : W.Ψ₃ =
      3 * X ^ 4 + C (6 * A) * X ^ 2 + C (12 * B) * X - C (A ^ 2) := by
    dsimp [W, WeierstrassCurve.Ψ₃, WeierstrassCurve.b₂,
      WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]
    norm_num [Polynomial.C_mul, Polynomial.C_ofNat]
    ring
  have hs : W.Ψ₂Sq = C 4 * (X ^ 3 + C A * X + C B) := by
    dsimp [W, WeierstrassCurve.Ψ₂Sq, WeierstrassCurve.b₂,
      WeierstrassCurve.b₄, WeierstrassCurve.b₆]
    norm_num [Polynomial.C_mul, Polynomial.C_ofNat]
    ring
  dsimp only
  rw [hq, hs]
  exact hbase

/-- Every odd-index Mathlib auxiliary division polynomial is coprime to the
two-torsion factor on a nonsingular short curve outside characteristic two. -/
theorem shortWeierstrass_prePsi_odd_isCoprime_PsiTwoSq
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) (n : ℕ) (hn : Odd n) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.preΨ' n) W.Ψ₂Sq := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have hbase : IsCoprime W.Ψ₃ W.Ψ₂Sq :=
    shortWeierstrassPsiThree_PsiTwoSq_isCoprime A B hdisc h2
  exact preNormEDS'_odd_isCoprime W.Ψ₂Sq W.Ψ₃ W.preΨ₄ hbase n hn

/-- The parity factor in Mathlib's `Phi` definition is always coprime to the
full squared division-polynomial denominator for natural scalar indices. -/
theorem shortWeierstrass_parityFactor_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) (n : ℕ) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (if Even (n : ℤ) then 1 else W.Ψ₂Sq) (W.ΨSq n) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  dsimp only
  by_cases hn : Even n
  · have hnz : Even (n : ℤ) := by exact_mod_cast hn
    rw [ite_eq_left hnz]
    exact isCoprime_one_left
  · have hnOdd : Odd n := Nat.not_even_iff_odd.mp hn
    have hnz : ¬Even (n : ℤ) := by
      intro h
      apply hn
      exact_mod_cast h
    have hpre := shortWeierstrass_prePsi_odd_isCoprime_PsiTwoSq
      A B hdisc h2 n hnOdd
    rw [WeierstrassCurve.ΨSq_ofNat]
    simp only [ite_eq_right hn, mul_one, ite_eq_right hnz]
    exact hpre.symm.pow_right

/-- On nonsingular short curves, generic numerator/denominator coprimality is
reduced to the two adjacent EDS factors; the parity premise is discharged. -/
theorem shortWeierstrassPhi_isCoprime_of_adjacent
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) (n : ℕ) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.preΨ ((n : ℤ) + 1)) (W.ΨSq n) →
    IsCoprime (W.preΨ ((n : ℤ) - 1)) (W.ΨSq n) →
    IsCoprime (W.Φ n) (W.ΨSq n) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  dsimp only
  intro hplus hminus
  apply divisionPolynomialPhi_isCoprime_of_factors W n hplus hminus
  exact shortWeierstrass_parityFactor_isCoprime A B hdisc h2 n

/-- Singular negative control: for `A=B=0`, both short-curve base
polynomials vanish at zero and are not coprime. -/
theorem shortWeierstrassPsiThree_PsiTwoSq_singular_control :
    let W : WeierstrassCurve (ZMod 5) := WeierstrassCurve.mk 0 0 0 0 0
    ¬IsCoprime W.Ψ₃ W.Ψ₂Sq := by
  let _ : Fact (Nat.Prime 5) := Fact.mk (by norm_num)
  let W : WeierstrassCurve (ZMod 5) := WeierstrassCurve.mk 0 0 0 0 0
  dsimp only
  intro h
  have heval := h.map (Polynomial.evalRingHom (0 : ZMod 5))
  norm_num [WeierstrassCurve.Ψ₃, WeierstrassCurve.Ψ₂Sq,
    WeierstrassCurve.b₂, WeierstrassCurve.b₄, WeierstrassCurve.b₆,
    WeierstrassCurve.b₈] at heval

/-- The scalar-three no-cancellation certificate forces the first nontrivial
adjacent auxiliary division-polynomial pair to be coprime. -/
theorem shortWeierstrass_prePsiThreeFour_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime W.Ψ₃ W.preΨ₄ := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have hscalar3 : IsCoprime (W.Φ 3) (W.ΨSq 3) :=
    shortWeierstrassScalarThree_isCoprime A B hdisc h2
  have hsquare : W.ΨSq 3 = W.Ψ₃ * W.Ψ₃ := by
    rw [WeierstrassCurve.ΨSq_three, pow_two]
  rw [hsquare] at hscalar3
  have hphi3 : IsCoprime (W.Φ 3) W.Ψ₃ :=
    hscalar3.of_mul_right_left
  rw [WeierstrassCurve.Φ_three] at hphi3
  have hprod : IsCoprime (W.preΨ₄ * W.Ψ₂Sq) W.Ψ₃ := by
    rw [show X * W.Ψ₃ ^ 2 - W.preΨ₄ * W.Ψ₂Sq =
      (X * W.Ψ₃) * W.Ψ₃ - W.preΨ₄ * W.Ψ₂Sq by ring] at hphi3
    exact IsCoprime.mul_sub_right_left_iff.mp hphi3
  exact hprod.of_mul_left_left.symm

/-- The odd EDS recurrence bootstraps the next adjacent pair from
`Psi3` coprime to `prePsi4`. -/
theorem shortWeierstrass_prePsiFourFive_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime W.preΨ₄ (W.preΨ' 5) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h34 : IsCoprime W.Ψ₃ W.preΨ₄ :=
    shortWeierstrass_prePsiThreeFour_isCoprime A B hdisc h2
  have h5 : W.preΨ' 5 = W.preΨ₄ * W.Ψ₂Sq ^ 2 - W.Ψ₃ ^ 3 := by
    rw [show 5 = 2 * (0 + 2) + 1 by norm_num, WeierstrassCurve.preΨ'_odd]
    simp
  dsimp only
  rw [h5]
  apply IsCoprime.symm
  rw [show W.preΨ₄ * W.Ψ₂Sq ^ 2 - W.Ψ₃ ^ 3 =
    (W.Ψ₂Sq ^ 2) * W.preΨ₄ - W.Ψ₃ ^ 3 by ring,
    IsCoprime.mul_sub_right_left_iff]
  exact h34.pow_left

/-- Scalar four has no numerator/denominator cancellation on every
nonsingular short curve outside characteristic two. -/
theorem shortWeierstrassScalarFour_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.Φ 4) (W.ΨSq 4) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h34 : IsCoprime W.Ψ₃ W.preΨ₄ :=
    shortWeierstrass_prePsiThreeFour_isCoprime A B hdisc h2
  have h45 : IsCoprime W.preΨ₄ (W.preΨ' 5) :=
    shortWeierstrass_prePsiFourFive_isCoprime A B hdisc h2
  have h3two : IsCoprime W.Ψ₃ W.Ψ₂Sq :=
    shortWeierstrassPsiThree_PsiTwoSq_isCoprime A B hdisc h2
  have h5two : IsCoprime (W.preΨ' 5) W.Ψ₂Sq :=
    shortWeierstrass_prePsi_odd_isCoprime_PsiTwoSq A B hdisc h2 5 (by decide)
  have hplus : IsCoprime (W.preΨ (5 : ℕ)) (W.ΨSq (4 : ℕ)) := by
    rw [WeierstrassCurve.preΨ_ofNat]
    change IsCoprime (W.preΨ' 5) (W.ΨSq (4 : ℤ))
    rw [WeierstrassCurve.ΨSq_four]
    exact h45.symm.pow_right.mul_right h5two
  have hminus : IsCoprime (W.preΨ (3 : ℕ)) (W.ΨSq (4 : ℕ)) := by
    rw [WeierstrassCurve.preΨ_ofNat, WeierstrassCurve.preΨ'_three]
    change IsCoprime W.Ψ₃ (W.ΨSq (4 : ℤ))
    rw [WeierstrassCurve.ΨSq_four]
    exact h34.pow_right.mul_right h3two
  convert
    (shortWeierstrassPhi_isCoprime_of_adjacent A B hdisc h2 4 hplus hminus) using 1

/-- The reduced scalar-four x-coordinate quotient has coordinate extension
degree exactly sixteen outside characteristic two. -/
theorem shortWeierstrassScalarFour_extension_degree
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    let xMap : RatFunc F :=
      algebraMap (Polynomial F) (RatFunc F) (W.Φ 4) /
        algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 4)
    Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 16 := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  let xMap : RatFunc F :=
    algebraMap (Polynomial F) (RatFunc F) (W.Φ 4) /
      algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 4)
  have h4z : ((4 : ℤ) : F) ≠ 0 := by
    norm_num
    rw [show (4 : F) = 2 * 2 by norm_num]
    exact mul_ne_zero h2 h2
  have hcoprime : IsCoprime (W.Φ 4) (W.ΨSq 4) :=
    shortWeierstrassScalarFour_isCoprime A B hdisc h2
  have hdenom : W.ΨSq 4 ≠ 0 := W.ΨSq_ne_zero h4z
  have hextension := reducedRationalQuotient_extension_degree
    (W.Φ 4) (W.ΨSq 4) hdenom hcoprime
  have hdegrees := scalarMultiplicationXMap_exact_degrees W 4 h4z
  change Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 16
  rw [hextension]
  norm_num at hdegrees ⊢
  omega

/-- Singular scalar-four control: at `A=B=0`, numerator and denominator both
vanish at zero and hence are not coprime. -/
theorem shortWeierstrassScalarFour_singular_control :
    let W : WeierstrassCurve (ZMod 5) := WeierstrassCurve.mk 0 0 0 0 0
    ¬IsCoprime (W.Φ 4) (W.ΨSq 4) := by
  let _ : Fact (Nat.Prime 5) := Fact.mk (by norm_num)
  let W : WeierstrassCurve (ZMod 5) := WeierstrassCurve.mk 0 0 0 0 0
  dsimp only
  intro h
  have heval := h.map (Polynomial.evalRingHom (0 : ZMod 5))
  norm_num [WeierstrassCurve.Φ_four, WeierstrassCurve.ΨSq_four,
    WeierstrassCurve.Ψ₃, WeierstrassCurve.preΨ₄,
    WeierstrassCurve.Ψ₂Sq, WeierstrassCurve.b₂, WeierstrassCurve.b₄,
    WeierstrassCurve.b₆, WeierstrassCurve.b₈] at heval

/-- The scalar-five recurrence reduces modulo `Psi3` to the already-proved
scalar-four adjacent and parity certificates. -/
theorem shortWeierstrass_prePsiThreeFive_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime W.Ψ₃ (W.preΨ' 5) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h34 : IsCoprime W.Ψ₃ W.preΨ₄ :=
    shortWeierstrass_prePsiThreeFour_isCoprime A B hdisc h2
  have h3two : IsCoprime W.Ψ₃ W.Ψ₂Sq :=
    shortWeierstrassPsiThree_PsiTwoSq_isCoprime A B hdisc h2
  have h5 : W.preΨ' 5 = W.preΨ₄ * W.Ψ₂Sq ^ 2 - W.Ψ₃ ^ 3 := by
    rw [show 5 = 2 * (0 + 2) + 1 by norm_num, WeierstrassCurve.preΨ'_odd]
    simp
  dsimp only
  rw [h5]
  apply IsCoprime.symm
  rw [show W.preΨ₄ * W.Ψ₂Sq ^ 2 - W.Ψ₃ ^ 3 =
    (W.preΨ₄ * W.Ψ₂Sq ^ 2) - (W.Ψ₃ ^ 2) * W.Ψ₃ by ring,
    IsCoprime.sub_mul_right_left_iff]
  exact h34.symm.mul_left (h3two.symm.pow_left)

/-- The even EDS recurrence then bootstraps the adjacent scalar-five/six
auxiliary pair. -/
theorem shortWeierstrass_prePsiFiveSix_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.preΨ' 5) (W.preΨ' 6) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h45 : IsCoprime W.preΨ₄ (W.preΨ' 5) :=
    shortWeierstrass_prePsiFourFive_isCoprime A B hdisc h2
  have h35 : IsCoprime W.Ψ₃ (W.preΨ' 5) :=
    shortWeierstrass_prePsiThreeFive_isCoprime A B hdisc h2
  have h6 : W.preΨ' 6 = W.Ψ₃ * (W.preΨ' 5 - W.preΨ₄ ^ 2) := by
    rw [show 6 = 2 * (0 + 3) by norm_num, WeierstrassCurve.preΨ'_even]
    simp
    ring
  dsimp only
  rw [h6]
  apply IsCoprime.symm
  rw [IsCoprime.mul_left_iff]
  constructor
  · exact h35
  · rw [show W.preΨ' 5 - W.preΨ₄ ^ 2 =
      1 * W.preΨ' 5 - W.preΨ₄ ^ 2 by ring,
      IsCoprime.mul_sub_right_left_iff]
    exact h45.pow_left

/-- Scalar five has no numerator/denominator cancellation on every
nonsingular short curve outside characteristic two. -/
theorem shortWeierstrassScalarFive_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.Φ 5) (W.ΨSq 5) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h45 : IsCoprime W.preΨ₄ (W.preΨ' 5) :=
    shortWeierstrass_prePsiFourFive_isCoprime A B hdisc h2
  have h56 : IsCoprime (W.preΨ' 5) (W.preΨ' 6) :=
    shortWeierstrass_prePsiFiveSix_isCoprime A B hdisc h2
  have hden5 : W.ΨSq (5 : ℤ) = W.preΨ' 5 ^ 2 := by
    rw [show (5 : ℤ) = ((5 : ℕ) : ℤ) by norm_num,
      WeierstrassCurve.ΨSq_ofNat]
    norm_num
  have hplus : IsCoprime (W.preΨ (6 : ℕ)) (W.ΨSq (5 : ℕ)) := by
    rw [WeierstrassCurve.preΨ_ofNat]
    change IsCoprime (W.preΨ' 6) (W.ΨSq (5 : ℤ))
    rw [hden5]
    exact h56.symm.pow_right
  have hminus : IsCoprime (W.preΨ (4 : ℕ)) (W.ΨSq (5 : ℕ)) := by
    rw [WeierstrassCurve.preΨ_ofNat, WeierstrassCurve.preΨ'_four]
    change IsCoprime W.preΨ₄ (W.ΨSq (5 : ℤ))
    rw [hden5]
    exact h45.pow_right
  convert
    (shortWeierstrassPhi_isCoprime_of_adjacent A B hdisc h2 5 hplus hminus) using 1

/-- The reduced scalar-five x-coordinate quotient has coordinate extension
degree exactly twenty-five outside characteristics two and five. -/
theorem shortWeierstrassScalarFive_extension_degree
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) (h5 : (5 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    let xMap : RatFunc F :=
      algebraMap (Polynomial F) (RatFunc F) (W.Φ 5) /
        algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 5)
    Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 25 := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  let xMap : RatFunc F :=
    algebraMap (Polynomial F) (RatFunc F) (W.Φ 5) /
      algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 5)
  have h5z : ((5 : ℤ) : F) ≠ 0 := by
    norm_num
    exact h5
  have hcoprime : IsCoprime (W.Φ 5) (W.ΨSq 5) :=
    shortWeierstrassScalarFive_isCoprime A B hdisc h2
  have hdenom : W.ΨSq 5 ≠ 0 := W.ΨSq_ne_zero h5z
  have hextension := reducedRationalQuotient_extension_degree
    (W.Φ 5) (W.ΨSq 5) hdenom hcoprime
  have hdegrees := scalarMultiplicationXMap_exact_degrees W 5 h5z
  change Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 25
  rw [hextension]
  norm_num at hdegrees ⊢
  omega

/-- Singular scalar-five control: at `A=B=0`, numerator and denominator both
vanish at zero and hence are not coprime. -/
theorem shortWeierstrassScalarFive_singular_control :
    let W : WeierstrassCurve (ZMod 7) := WeierstrassCurve.mk 0 0 0 0 0
    ¬IsCoprime (W.Φ 5) (W.ΨSq 5) := by
  let _ : Fact (Nat.Prime 7) := Fact.mk (by norm_num)
  let W : WeierstrassCurve (ZMod 7) := WeierstrassCurve.mk 0 0 0 0 0
  dsimp only
  intro h
  have heval := h.map (Polynomial.evalRingHom (0 : ZMod 7))
  norm_num [WeierstrassCurve.Φ, WeierstrassCurve.ΨSq,
    WeierstrassCurve.preΨ, preNormEDS, preNormEDS',
    WeierstrassCurve.Ψ₃, WeierstrassCurve.preΨ₄,
    WeierstrassCurve.Ψ₂Sq, WeierstrassCurve.b₂, WeierstrassCurve.b₄,
    WeierstrassCurve.b₆, WeierstrassCurve.b₈] at heval

/-- Generic two-factor EDS bootstrap.  The recurrence-shaped pair remains
coprime whenever the three smaller base pairs are coprime. -/
theorem edsTwoFactorBootstrap_isCoprime
    {R : Type*} [CommRing R] (a b c s : R)
    (hc : c = b * s ^ 2 - a ^ 3)
    (hab : IsCoprime a b) (has : IsCoprime a s)
    (hbc : IsCoprime b c) :
    IsCoprime (a * (c - b ^ 2)) (c * a ^ 3 - b ^ 3 * s ^ 2) := by
  let t := c - b ^ 2
  let d := a * t
  let e := c * a ^ 3 - b ^ 3 * s ^ 2
  have heAlt : e = (a ^ 3 - b ^ 2) * t - b ^ 4 := by
    dsimp [e, t]
    rw [hc]
    ring
  have hea : IsCoprime e a := by
    dsimp [e]
    rw [show c * a ^ 3 - b ^ 3 * s ^ 2 =
        (c * a ^ 2) * a - b ^ 3 * s ^ 2 by ring,
      IsCoprime.mul_sub_right_left_iff]
    exact hab.symm.pow_left.mul_left (has.symm.pow_left)
  have hbt : IsCoprime b t := by
    apply IsCoprime.symm
    dsimp [t]
    rw [show c - b ^ 2 = c - b * b by ring,
      IsCoprime.sub_mul_left_left_iff]
    exact hbc.symm
  have het : IsCoprime e t := by
    rw [heAlt, IsCoprime.mul_sub_right_left_iff]
    exact hbt.pow_left
  change IsCoprime d e
  dsimp [d]
  exact hea.symm.mul_left het.symm

/-- Mutation control for the generic bootstrap: retaining `a` coprime to `b`
and `b` coprime to `c` does not compensate for omitting `a` coprime to `s`. -/
theorem edsTwoFactorBootstrap_missingAS_control :
    IsCoprime (2 : ℤ) 1 ∧ IsCoprime (1 : ℤ) (-4) ∧
      ¬IsCoprime (2 * ((-4) - 1 ^ 2))
        ((-4) * 2 ^ 3 - 1 ^ 3 * 2 ^ 2) := by
  norm_num [IsCoprime]

/-- The companion successor stencil: the same three base certificates also
prove coprimality of the next generated pair. -/
theorem edsTwoFactorSuccessor_isCoprime
    {R : Type*} [CommRing R] (a b c s : R)
    (hc : c = b * s ^ 2 - a ^ 3)
    (hab : IsCoprime a b) (has : IsCoprime a s)
    (hbc : IsCoprime b c) :
    IsCoprime (c * a ^ 3 - b ^ 3 * s ^ 2)
      (b * (a ^ 2 * (a * (c - b ^ 2)) - c ^ 2)) := by
  let t := c - b ^ 2
  let d := a * t
  let e := c * a ^ 3 - b ^ 3 * s ^ 2
  let u := a ^ 2 * d - c ^ 2
  have hcs : IsCoprime c s := by
    rw [hc, show b * s ^ 2 - a ^ 3 = (b * s) * s - a ^ 3 by ring,
      IsCoprime.mul_sub_right_left_iff]
    exact has.pow_left
  have heAlt : e = (a ^ 3 - b ^ 2) * t - b ^ 4 := by
    dsimp [e, t]
    rw [hc]
    ring
  have hbe : IsCoprime b e := by
    apply IsCoprime.symm
    dsimp [e]
    rw [show c * a ^ 3 - b ^ 3 * s ^ 2 =
        (c * a ^ 3) - (b ^ 2 * s ^ 2) * b by ring,
      IsCoprime.sub_mul_right_left_iff]
    exact hbc.symm.mul_left hab.pow_left
  have hce : IsCoprime c e := by
    apply IsCoprime.symm
    dsimp [e]
    rw [show c * a ^ 3 - b ^ 3 * s ^ 2 =
        (a ^ 3) * c - b ^ 3 * s ^ 2 by ring,
      IsCoprime.mul_sub_right_left_iff]
    exact hbc.pow_left.mul_left (hcs.symm.pow_left)
  have hbt : IsCoprime b t := by
    apply IsCoprime.symm
    dsimp [t]
    rw [show c - b ^ 2 = c - b * b by ring,
      IsCoprime.sub_mul_left_left_iff]
    exact hbc.symm
  have het : IsCoprime e t := by
    rw [heAlt, IsCoprime.mul_sub_right_left_iff]
    exact hbt.pow_left
  have huAlt : u = e - c * t := by
    dsimp [u, d, e, t]
    rw [hc]
    ring
  have heu : IsCoprime e u := by
    apply IsCoprime.symm
    rw [huAlt, show e - c * t = 1 * e - c * t by ring,
      IsCoprime.mul_sub_right_left_iff]
    exact hce.mul_left het.symm
  change IsCoprime e (b * u)
  exact hbe.symm.mul_right heu

/-- The second companion successor stencil.  The same invariant proves
coprimality of one further generated pair. -/
theorem edsTwoFactorSecondSuccessor_isCoprime
    {R : Type*} [CommRing R] (a b c s : R)
    (hc : c = b * s ^ 2 - a ^ 3)
    (hab : IsCoprime a b) (has : IsCoprime a s)
    (hbc : IsCoprime b c) :
    IsCoprime (b * (a ^ 2 * (a * (c - b ^ 2)) - c ^ 2))
      ((a * (c - b ^ 2)) * b ^ 3 * s ^ 2 - a * c ^ 3) := by
  let t := c - b ^ 2
  let d := a * t
  let e := c * a ^ 3 - b ^ 3 * s ^ 2
  let u := a ^ 2 * d - c ^ 2
  let f := b * u
  let g := d * b ^ 3 * s ^ 2 - a * c ^ 3
  have hac : IsCoprime a c := by
    apply IsCoprime.symm
    rw [hc, show b * s ^ 2 - a ^ 3 = b * s ^ 2 - a ^ 2 * a by ring,
      IsCoprime.sub_mul_right_left_iff]
    exact hab.symm.mul_left (has.symm.pow_left)
  have htc : IsCoprime t c := by
    dsimp [t]
    rw [show c - b ^ 2 = -b ^ 2 + c * 1 by ring,
      IsCoprime.add_mul_left_left_iff, IsCoprime.neg_left_iff]
    exact hbc.pow_left
  have huc : IsCoprime u c := by
    dsimp [u]
    rw [show a ^ 2 * d - c ^ 2 = a ^ 2 * d - c * c by ring,
      IsCoprime.sub_mul_left_left_iff]
    exact hac.pow_left.mul_left (hac.mul_left htc)
  have hsucc : IsCoprime e f := by
    dsimp [e, f, u, d, t]
    exact edsTwoFactorSuccessor_isCoprime a b c s hc hab has hbc
  have hue : IsCoprime u e := hsucc.of_mul_right_right.symm
  have hbg : IsCoprime b g := by
    apply IsCoprime.symm
    dsimp [g]
    rw [show d * b ^ 3 * s ^ 2 - a * c ^ 3 =
        (d * b ^ 2 * s ^ 2) * b - a * c ^ 3 by ring,
      IsCoprime.mul_sub_right_left_iff]
    exact hab.mul_left (hbc.symm.pow_left)
  have hugScaled : IsCoprime u (a ^ 2 * g) := by
    rw [show a ^ 2 * g = b ^ 3 * s ^ 2 * u - c ^ 2 * e by
      dsimp [g, u, e]
      ring,
      IsCoprime.mul_sub_right_right_iff]
    exact huc.pow_right.mul_right hue
  have hug : IsCoprime u g := hugScaled.of_mul_right_right
  change IsCoprime f g
  dsimp [f]
  exact hbg.mul_left hug

/-- The third companion successor stencil.  Factoring the preceding odd
output exposes one further subtract-multiple reduction. -/
theorem edsTwoFactorThirdSuccessor_isCoprime
    {R : Type*} [CommRing R] (a b c s : R)
    (hc : c = b * s ^ 2 - a ^ 3)
    (hab : IsCoprime a b) (has : IsCoprime a s)
    (hbc : IsCoprime b c) :
    IsCoprime ((a * (c - b ^ 2)) * b ^ 3 * s ^ 2 - a * c ^ 3)
      (c * (b ^ 2 * (c * a ^ 3 - b ^ 3 * s ^ 2) -
        a * (a * (c - b ^ 2)) ^ 2)) := by
  let t := c - b ^ 2
  let d := a * t
  let e := c * a ^ 3 - b ^ 3 * s ^ 2
  let u := a ^ 2 * d - c ^ 2
  let f := b * u
  let w := b ^ 2 * u - c * t ^ 2
  let g := d * b ^ 3 * s ^ 2 - a * c ^ 3
  let v := b ^ 2 * e - a * d ^ 2
  let h := c * v
  have hcs : IsCoprime c s := by
    rw [hc, show b * s ^ 2 - a ^ 3 = (b * s) * s - a ^ 3 by ring,
      IsCoprime.mul_sub_right_left_iff]
    exact has.pow_left
  have hac : IsCoprime a c := by
    apply IsCoprime.symm
    rw [hc, show b * s ^ 2 - a ^ 3 = b * s ^ 2 - a ^ 2 * a by ring,
      IsCoprime.sub_mul_right_left_iff]
    exact hab.symm.mul_left (has.symm.pow_left)
  have htc : IsCoprime t c := by
    dsimp [t]
    rw [show c - b ^ 2 = -b ^ 2 + c * 1 by ring,
      IsCoprime.add_mul_left_left_iff, IsCoprime.neg_left_iff]
    exact hbc.pow_left
  have hbt : IsCoprime b t := by
    apply IsCoprime.symm
    dsimp [t]
    rw [show c - b ^ 2 = c - b * b by ring,
      IsCoprime.sub_mul_left_left_iff]
    exact hbc.symm
  have htu : IsCoprime t u := by
    dsimp [u, d]
    rw [show a ^ 2 * (a * t) - c ^ 2 = -c ^ 2 + t * a ^ 3 by ring,
      IsCoprime.add_mul_left_right_iff, IsCoprime.neg_right_iff]
    exact htc.pow_right
  have hgFactor : g = a * w := by
    dsimp [g, w, u, d, t]
    rw [hc]
    ring
  have hvAlt : v = w - t * u := by
    dsimp [v, w, u, e, d, t]
    rw [hc]
    ring
  have hfg : IsCoprime f g := by
    dsimp [f, u, g, d, t]
    exact edsTwoFactorSecondSuccessor_isCoprime a b c s hc hab has hbc
  have hug : IsCoprime u g := hfg.of_mul_left_right
  have huw : IsCoprime u w := by
    rw [hgFactor] at hug
    exact hug.of_mul_right_right
  have hwt : IsCoprime w t := by
    dsimp [w]
    rw [show b ^ 2 * u - c * t ^ 2 = b ^ 2 * u - (c * t) * t by ring,
      IsCoprime.sub_mul_right_left_iff]
    exact hbt.pow_left.mul_left htu.symm
  have hwv : IsCoprime w v := by
    rw [hvAlt, show w - t * u = -(t * u) + w * 1 by ring,
      IsCoprime.add_mul_left_right_iff, IsCoprime.neg_right_iff]
    exact hwt.mul_right huw.symm
  have hde : IsCoprime d e := by
    dsimp [d, e, t]
    exact edsTwoFactorBootstrap_isCoprime a b c s hc hab has hbc
  have hae : IsCoprime a e := hde.of_mul_left_left
  have hav : IsCoprime a v := by
    dsimp [v]
    rw [IsCoprime.sub_mul_left_right_iff]
    exact hab.pow_right.mul_right hae
  have hgv : IsCoprime g v := by
    rw [hgFactor]
    exact hav.mul_left hwv
  have hgc : IsCoprime g c := by
    dsimp [g]
    rw [show d * b ^ 3 * s ^ 2 - a * c ^ 3 =
        d * b ^ 3 * s ^ 2 - (a * c ^ 2) * c by ring,
      IsCoprime.sub_mul_right_left_iff]
    exact ((hac.mul_left htc).mul_left hbc.pow_left).mul_left
      (hcs.symm.pow_left)
  change IsCoprime g h
  dsimp [h]
  exact hgc.mul_right hgv

/-- The fourth companion successor stencil.  The next odd output is a
multiple of the preceding inner factor minus a product of inherited factors. -/
theorem edsTwoFactorFourthSuccessor_isCoprime
    {R : Type*} [CommRing R] (a b c s : R)
    (hc : c = b * s ^ 2 - a ^ 3)
    (hab : IsCoprime a b) (has : IsCoprime a s)
    (hbc : IsCoprime b c) :
    IsCoprime
      (c * (b ^ 2 * (c * a ^ 3 - b ^ 3 * s ^ 2) -
        a * (a * (c - b ^ 2)) ^ 2))
      ((c * a ^ 3 - b ^ 3 * s ^ 2) * c ^ 3 -
        b * (a * (c - b ^ 2)) ^ 3 * s ^ 2) := by
  let t := c - b ^ 2
  let d := a * t
  let e := c * a ^ 3 - b ^ 3 * s ^ 2
  let u := a ^ 2 * d - c ^ 2
  let w := b ^ 2 * u - c * t ^ 2
  let g := d * b ^ 3 * s ^ 2 - a * c ^ 3
  let v := b ^ 2 * e - a * d ^ 2
  let h := c * v
  let i := e * c ^ 3 - b * d ^ 3 * s ^ 2
  have hcs : IsCoprime c s := by
    rw [hc, show b * s ^ 2 - a ^ 3 = (b * s) * s - a ^ 3 by ring,
      IsCoprime.mul_sub_right_left_iff]
    exact has.pow_left
  have hac : IsCoprime a c := by
    apply IsCoprime.symm
    rw [hc, show b * s ^ 2 - a ^ 3 = b * s ^ 2 - a ^ 2 * a by ring,
      IsCoprime.sub_mul_right_left_iff]
    exact hab.symm.mul_left (has.symm.pow_left)
  have htc : IsCoprime t c := by
    dsimp [t]
    rw [show c - b ^ 2 = -b ^ 2 + c * 1 by ring,
      IsCoprime.add_mul_left_left_iff, IsCoprime.neg_left_iff]
    exact hbc.pow_left
  have hgFactor : g = a * w := by
    dsimp [g, w, u, d, t]
    rw [hc]
    ring
  have hvAlt : v = w - t * u := by
    dsimp [v, w, u, e, d, t]
    rw [hc]
    ring
  have hiAlt : i = c ^ 2 * v - t * u * e := by
    dsimp [i, v, u, e, d, t]
    rw [hc]
    ring
  have hgh : IsCoprime g h := by
    dsimp [g, h, v, e, d, t]
    exact edsTwoFactorThirdSuccessor_isCoprime a b c s hc hab has hbc
  have hvg : IsCoprime v g := hgh.of_mul_right_right.symm
  have hvw : IsCoprime v w := by
    rw [hgFactor] at hvg
    exact hvg.of_mul_right_right
  have hvtu : IsCoprime v (t * u) := by
    rw [show t * u = w + (-1) * v by rw [hvAlt]; ring,
      IsCoprime.add_mul_right_right_iff]
    exact hvw
  have hvt : IsCoprime v t := hvtu.of_mul_right_left
  have hvu : IsCoprime v u := hvtu.of_mul_right_right
  have hde : IsCoprime d e := by
    dsimp [d, e, t]
    exact edsTwoFactorBootstrap_isCoprime a b c s hc hab has hbc
  have hae : IsCoprime a e := hde.of_mul_left_left
  have hve : IsCoprime v e := by
    dsimp [v]
    rw [IsCoprime.mul_sub_right_left_iff]
    exact hae.mul_left hde.pow_left
  have hvi : IsCoprime v i := by
    rw [hiAlt, IsCoprime.mul_sub_right_right_iff]
    exact (hvt.mul_right hvu).mul_right hve
  have hcd : IsCoprime c d := by
    dsimp [d]
    exact hac.symm.mul_right htc.symm
  have hci : IsCoprime c i := by
    dsimp [i]
    rw [show e * c ^ 3 - b * d ^ 3 * s ^ 2 =
        (e * c ^ 2) * c - b * d ^ 3 * s ^ 2 by ring,
      IsCoprime.mul_sub_right_right_iff]
    exact (hbc.symm.mul_right hcd.pow_right).mul_right hcs.pow_right
  change IsCoprime h i
  dsimp [h]
  exact hci.mul_left hvi

/-- The first two-factor recurrence bootstrap: `prePsi6=Psi3*t`, while an
alternate exact expression for `prePsi7` is a multiple of `t` minus
`prePsi4^4`. -/
theorem shortWeierstrass_prePsiSixSeven_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.preΨ' 6) (W.preΨ' 7) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h34 : IsCoprime W.Ψ₃ W.preΨ₄ :=
    shortWeierstrass_prePsiThreeFour_isCoprime A B hdisc h2
  have h45 : IsCoprime W.preΨ₄ (W.preΨ' 5) :=
    shortWeierstrass_prePsiFourFive_isCoprime A B hdisc h2
  have h3two : IsCoprime W.Ψ₃ W.Ψ₂Sq :=
    shortWeierstrassPsiThree_PsiTwoSq_isCoprime A B hdisc h2
  have h5 : W.preΨ' 5 = W.preΨ₄ * W.Ψ₂Sq ^ 2 - W.Ψ₃ ^ 3 := by
    rw [show 5 = 2 * (0 + 2) + 1 by norm_num, WeierstrassCurve.preΨ'_odd]
    simp
  have h6 : W.preΨ' 6 = W.Ψ₃ * (W.preΨ' 5 - W.preΨ₄ ^ 2) := by
    rw [show 6 = 2 * (0 + 3) by norm_num, WeierstrassCurve.preΨ'_even]
    simp
    ring
  have h7 : W.preΨ' 7 =
      W.preΨ' 5 * W.Ψ₃ ^ 3 - W.preΨ₄ ^ 3 * W.Ψ₂Sq ^ 2 := by
    rw [show 7 = 2 * (1 + 2) + 1 by norm_num, WeierstrassCurve.preΨ'_odd]
    norm_num
  dsimp only
  rw [h6, h7]
  exact edsTwoFactorBootstrap_isCoprime W.Ψ₃ W.preΨ₄
    (W.preΨ' 5) W.Ψ₂Sq h5 h34 h3two h45

/-- Scalar six has no numerator/denominator cancellation on every nonsingular
short curve outside characteristic two. -/
theorem shortWeierstrassScalarSix_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.Φ 6) (W.ΨSq 6) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h56 : IsCoprime (W.preΨ' 5) (W.preΨ' 6) :=
    shortWeierstrass_prePsiFiveSix_isCoprime A B hdisc h2
  have h67 : IsCoprime (W.preΨ' 6) (W.preΨ' 7) :=
    shortWeierstrass_prePsiSixSeven_isCoprime A B hdisc h2
  have h5two : IsCoprime (W.preΨ' 5) W.Ψ₂Sq :=
    shortWeierstrass_prePsi_odd_isCoprime_PsiTwoSq A B hdisc h2 5 (by decide)
  have h7two : IsCoprime (W.preΨ' 7) W.Ψ₂Sq :=
    shortWeierstrass_prePsi_odd_isCoprime_PsiTwoSq A B hdisc h2 7 (by decide)
  have hden6 : W.ΨSq (6 : ℤ) = W.preΨ' 6 ^ 2 * W.Ψ₂Sq := by
    rw [show (6 : ℤ) = ((6 : ℕ) : ℤ) by norm_num,
      WeierstrassCurve.ΨSq_ofNat]
    norm_num
  have hplus : IsCoprime (W.preΨ (7 : ℕ)) (W.ΨSq (6 : ℕ)) := by
    rw [WeierstrassCurve.preΨ_ofNat]
    change IsCoprime (W.preΨ' 7) (W.ΨSq (6 : ℤ))
    rw [hden6]
    exact h67.symm.pow_right.mul_right h7two
  have hminus : IsCoprime (W.preΨ (5 : ℕ)) (W.ΨSq (6 : ℕ)) := by
    rw [WeierstrassCurve.preΨ_ofNat]
    change IsCoprime (W.preΨ' 5) (W.ΨSq (6 : ℤ))
    rw [hden6]
    exact h56.pow_right.mul_right h5two
  convert
    (shortWeierstrassPhi_isCoprime_of_adjacent A B hdisc h2 6 hplus hminus) using 1

/-- The reduced scalar-six x-coordinate quotient has coordinate extension
degree exactly thirty-six outside characteristics two and three. -/
theorem shortWeierstrassScalarSix_extension_degree
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) (h3 : (3 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    let xMap : RatFunc F :=
      algebraMap (Polynomial F) (RatFunc F) (W.Φ 6) /
        algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 6)
    Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 36 := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  let xMap : RatFunc F :=
    algebraMap (Polynomial F) (RatFunc F) (W.Φ 6) /
      algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 6)
  have h6z : ((6 : ℤ) : F) ≠ 0 := by
    norm_num
    rw [show (6 : F) = 2 * 3 by norm_num]
    exact mul_ne_zero h2 h3
  have hcoprime : IsCoprime (W.Φ 6) (W.ΨSq 6) :=
    shortWeierstrassScalarSix_isCoprime A B hdisc h2
  have hdenom : W.ΨSq 6 ≠ 0 := W.ΨSq_ne_zero h6z
  have hextension := reducedRationalQuotient_extension_degree
    (W.Φ 6) (W.ΨSq 6) hdenom hcoprime
  have hdegrees := scalarMultiplicationXMap_exact_degrees W 6 h6z
  change Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 36
  rw [hextension]
  norm_num at hdegrees ⊢
  omega

/-- Singular scalar-six control: at `A=B=0`, numerator and denominator both
vanish at zero and hence are not coprime. -/
theorem shortWeierstrassScalarSix_singular_control :
    let W : WeierstrassCurve (ZMod 7) := WeierstrassCurve.mk 0 0 0 0 0
    ¬IsCoprime (W.Φ 6) (W.ΨSq 6) := by
  let _ : Fact (Nat.Prime 7) := Fact.mk (by norm_num)
  let W : WeierstrassCurve (ZMod 7) := WeierstrassCurve.mk 0 0 0 0 0
  dsimp only
  intro h
  have heval := h.map (Polynomial.evalRingHom (0 : ZMod 7))
  norm_num [WeierstrassCurve.Φ, WeierstrassCurve.ΨSq,
    WeierstrassCurve.preΨ, preNormEDS, preNormEDS',
    WeierstrassCurve.Ψ₃, WeierstrassCurve.preΨ₄,
    WeierstrassCurve.Ψ₂Sq, WeierstrassCurve.b₂, WeierstrassCurve.b₄,
    WeierstrassCurve.b₆, WeierstrassCurve.b₈] at heval

/-- Instantiate the generic successor stencil to obtain the next adjacent
auxiliary division-polynomial pair. -/
theorem shortWeierstrass_prePsiSevenEight_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.preΨ' 7) (W.preΨ' 8) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h34 : IsCoprime W.Ψ₃ W.preΨ₄ :=
    shortWeierstrass_prePsiThreeFour_isCoprime A B hdisc h2
  have h45 : IsCoprime W.preΨ₄ (W.preΨ' 5) :=
    shortWeierstrass_prePsiFourFive_isCoprime A B hdisc h2
  have h3two : IsCoprime W.Ψ₃ W.Ψ₂Sq :=
    shortWeierstrassPsiThree_PsiTwoSq_isCoprime A B hdisc h2
  have h5 : W.preΨ' 5 = W.preΨ₄ * W.Ψ₂Sq ^ 2 - W.Ψ₃ ^ 3 := by
    rw [show 5 = 2 * (0 + 2) + 1 by norm_num, WeierstrassCurve.preΨ'_odd]
    simp
  have h6 : W.preΨ' 6 = W.Ψ₃ * (W.preΨ' 5 - W.preΨ₄ ^ 2) := by
    rw [show 6 = 2 * (0 + 3) by norm_num, WeierstrassCurve.preΨ'_even]
    simp
    ring
  have h7 : W.preΨ' 7 =
      W.preΨ' 5 * W.Ψ₃ ^ 3 - W.preΨ₄ ^ 3 * W.Ψ₂Sq ^ 2 := by
    rw [show 7 = 2 * (1 + 2) + 1 by norm_num, WeierstrassCurve.preΨ'_odd]
    norm_num
  have h8 : W.preΨ' 8 = W.preΨ₄ *
      (W.Ψ₃ ^ 2 * W.preΨ' 6 - W.preΨ' 5 ^ 2) := by
    rw [show 8 = 2 * (1 + 3) by norm_num, WeierstrassCurve.preΨ'_even]
    simp
    ring
  change IsCoprime (W.preΨ' 7) (W.preΨ' 8)
  rw [h7, h8, h6]
  exact edsTwoFactorSuccessor_isCoprime W.Ψ₃ W.preΨ₄
    (W.preΨ' 5) W.Ψ₂Sq h5 h34 h3two h45

/-- Scalar seven has no numerator/denominator cancellation on every
nonsingular short curve outside characteristic two. -/
theorem shortWeierstrassScalarSeven_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.Φ 7) (W.ΨSq 7) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h67 : IsCoprime (W.preΨ' 6) (W.preΨ' 7) :=
    shortWeierstrass_prePsiSixSeven_isCoprime A B hdisc h2
  have h78 : IsCoprime (W.preΨ' 7) (W.preΨ' 8) :=
    shortWeierstrass_prePsiSevenEight_isCoprime A B hdisc h2
  have hden7 : W.ΨSq (7 : ℤ) = W.preΨ' 7 ^ 2 := by
    rw [show (7 : ℤ) = ((7 : ℕ) : ℤ) by norm_num,
      WeierstrassCurve.ΨSq_ofNat]
    norm_num
  have hplus : IsCoprime (W.preΨ (8 : ℕ)) (W.ΨSq (7 : ℕ)) := by
    rw [WeierstrassCurve.preΨ_ofNat]
    change IsCoprime (W.preΨ' 8) (W.ΨSq (7 : ℤ))
    rw [hden7]
    exact h78.symm.pow_right
  have hminus : IsCoprime (W.preΨ (6 : ℕ)) (W.ΨSq (7 : ℕ)) := by
    rw [WeierstrassCurve.preΨ_ofNat]
    change IsCoprime (W.preΨ' 6) (W.ΨSq (7 : ℤ))
    rw [hden7]
    exact h67.pow_right
  convert
    (shortWeierstrassPhi_isCoprime_of_adjacent A B hdisc h2 7 hplus hminus) using 1

/-- The reduced scalar-seven x-coordinate quotient has coordinate extension
degree exactly forty-nine outside characteristics two and seven. -/
theorem shortWeierstrassScalarSeven_extension_degree
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) (h7 : (7 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    let xMap : RatFunc F :=
      algebraMap (Polynomial F) (RatFunc F) (W.Φ 7) /
        algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 7)
    Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 49 := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  let xMap : RatFunc F :=
    algebraMap (Polynomial F) (RatFunc F) (W.Φ 7) /
      algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 7)
  have h7z : ((7 : ℤ) : F) ≠ 0 := by
    norm_num
    exact h7
  have hcoprime : IsCoprime (W.Φ 7) (W.ΨSq 7) :=
    shortWeierstrassScalarSeven_isCoprime A B hdisc h2
  have hdenom : W.ΨSq 7 ≠ 0 := W.ΨSq_ne_zero h7z
  have hextension := reducedRationalQuotient_extension_degree
    (W.Φ 7) (W.ΨSq 7) hdenom hcoprime
  have hdegrees := scalarMultiplicationXMap_exact_degrees W 7 h7z
  change Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 49
  rw [hextension]
  norm_num at hdegrees ⊢
  omega

/-- Singular scalar-seven control: at `A=B=0`, numerator and denominator both
vanish at zero and hence are not coprime. -/
theorem shortWeierstrassScalarSeven_singular_control :
    let W : WeierstrassCurve (ZMod 11) := WeierstrassCurve.mk 0 0 0 0 0
    ¬IsCoprime (W.Φ 7) (W.ΨSq 7) := by
  let _ : Fact (Nat.Prime 11) := Fact.mk (by norm_num)
  let W : WeierstrassCurve (ZMod 11) := WeierstrassCurve.mk 0 0 0 0 0
  dsimp only
  intro h
  have heval := h.map (Polynomial.evalRingHom (0 : ZMod 11))
  norm_num [WeierstrassCurve.Φ, WeierstrassCurve.ΨSq,
    WeierstrassCurve.preΨ, preNormEDS, preNormEDS',
    WeierstrassCurve.Ψ₃, WeierstrassCurve.preΨ₄,
    WeierstrassCurve.Ψ₂Sq, WeierstrassCurve.b₂, WeierstrassCurve.b₄,
    WeierstrassCurve.b₆, WeierstrassCurve.b₈] at heval

/-- Instantiate the second successor stencil at the next exact EDS pair. -/
theorem shortWeierstrass_prePsiEightNine_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.preΨ' 8) (W.preΨ' 9) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h34 : IsCoprime W.Ψ₃ W.preΨ₄ :=
    shortWeierstrass_prePsiThreeFour_isCoprime A B hdisc h2
  have h45 : IsCoprime W.preΨ₄ (W.preΨ' 5) :=
    shortWeierstrass_prePsiFourFive_isCoprime A B hdisc h2
  have h3two : IsCoprime W.Ψ₃ W.Ψ₂Sq :=
    shortWeierstrassPsiThree_PsiTwoSq_isCoprime A B hdisc h2
  have h5 : W.preΨ' 5 = W.preΨ₄ * W.Ψ₂Sq ^ 2 - W.Ψ₃ ^ 3 := by
    rw [show 5 = 2 * (0 + 2) + 1 by norm_num, WeierstrassCurve.preΨ'_odd]
    simp
  have h6 : W.preΨ' 6 = W.Ψ₃ * (W.preΨ' 5 - W.preΨ₄ ^ 2) := by
    rw [show 6 = 2 * (0 + 3) by norm_num, WeierstrassCurve.preΨ'_even]
    simp
    ring
  have h8 : W.preΨ' 8 = W.preΨ₄ *
      (W.Ψ₃ ^ 2 * W.preΨ' 6 - W.preΨ' 5 ^ 2) := by
    rw [show 8 = 2 * (1 + 3) by norm_num, WeierstrassCurve.preΨ'_even]
    simp
    ring
  have h9 : W.preΨ' 9 = W.preΨ' 6 * W.preΨ₄ ^ 3 * W.Ψ₂Sq ^ 2 -
      W.Ψ₃ * W.preΨ' 5 ^ 3 := by
    rw [show 9 = 2 * (2 + 2) + 1 by norm_num, WeierstrassCurve.preΨ'_odd]
    norm_num
  change IsCoprime (W.preΨ' 8) (W.preΨ' 9)
  rw [h8, h9, h6]
  exact edsTwoFactorSecondSuccessor_isCoprime W.Ψ₃ W.preΨ₄
    (W.preΨ' 5) W.Ψ₂Sq h5 h34 h3two h45

/-- Scalar eight has no numerator/denominator cancellation on every
nonsingular short curve outside characteristic two. -/
theorem shortWeierstrassScalarEight_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.Φ 8) (W.ΨSq 8) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h78 : IsCoprime (W.preΨ' 7) (W.preΨ' 8) :=
    shortWeierstrass_prePsiSevenEight_isCoprime A B hdisc h2
  have h89 : IsCoprime (W.preΨ' 8) (W.preΨ' 9) :=
    shortWeierstrass_prePsiEightNine_isCoprime A B hdisc h2
  have h7two : IsCoprime (W.preΨ' 7) W.Ψ₂Sq :=
    shortWeierstrass_prePsi_odd_isCoprime_PsiTwoSq A B hdisc h2 7 (by decide)
  have h9two : IsCoprime (W.preΨ' 9) W.Ψ₂Sq :=
    shortWeierstrass_prePsi_odd_isCoprime_PsiTwoSq A B hdisc h2 9 (by decide)
  have hden8 : W.ΨSq (8 : ℤ) = W.preΨ' 8 ^ 2 * W.Ψ₂Sq := by
    rw [show (8 : ℤ) = ((8 : ℕ) : ℤ) by norm_num,
      WeierstrassCurve.ΨSq_ofNat]
    norm_num
  have hplus : IsCoprime (W.preΨ (9 : ℕ)) (W.ΨSq (8 : ℕ)) := by
    rw [WeierstrassCurve.preΨ_ofNat]
    change IsCoprime (W.preΨ' 9) (W.ΨSq (8 : ℤ))
    rw [hden8]
    exact h89.symm.pow_right.mul_right h9two
  have hminus : IsCoprime (W.preΨ (7 : ℕ)) (W.ΨSq (8 : ℕ)) := by
    rw [WeierstrassCurve.preΨ_ofNat]
    change IsCoprime (W.preΨ' 7) (W.ΨSq (8 : ℤ))
    rw [hden8]
    exact h78.pow_right.mul_right h7two
  convert
    (shortWeierstrassPhi_isCoprime_of_adjacent A B hdisc h2 8 hplus hminus) using 1

/-- The reduced scalar-eight x-coordinate quotient has coordinate extension
degree exactly sixty-four outside characteristic two. -/
theorem shortWeierstrassScalarEight_extension_degree
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    let xMap : RatFunc F :=
      algebraMap (Polynomial F) (RatFunc F) (W.Φ 8) /
        algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 8)
    Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 64 := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  let xMap : RatFunc F :=
    algebraMap (Polynomial F) (RatFunc F) (W.Φ 8) /
      algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 8)
  have h8z : ((8 : ℤ) : F) ≠ 0 := by
    norm_num
    rw [show (8 : F) = 2 * 2 * 2 by norm_num]
    exact mul_ne_zero (mul_ne_zero h2 h2) h2
  have hcoprime : IsCoprime (W.Φ 8) (W.ΨSq 8) :=
    shortWeierstrassScalarEight_isCoprime A B hdisc h2
  have hdenom : W.ΨSq 8 ≠ 0 := W.ΨSq_ne_zero h8z
  have hextension := reducedRationalQuotient_extension_degree
    (W.Φ 8) (W.ΨSq 8) hdenom hcoprime
  have hdegrees := scalarMultiplicationXMap_exact_degrees W 8 h8z
  change Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 64
  rw [hextension]
  norm_num at hdegrees ⊢
  omega

/-- Singular scalar-eight control: at `A=B=0`, numerator and denominator both
vanish at zero and hence are not coprime. -/
theorem shortWeierstrassScalarEight_singular_control :
    let W : WeierstrassCurve (ZMod 11) := WeierstrassCurve.mk 0 0 0 0 0
    ¬IsCoprime (W.Φ 8) (W.ΨSq 8) := by
  let _ : Fact (Nat.Prime 11) := Fact.mk (by norm_num)
  let W : WeierstrassCurve (ZMod 11) := WeierstrassCurve.mk 0 0 0 0 0
  dsimp only
  intro h
  have heval := h.map (Polynomial.evalRingHom (0 : ZMod 11))
  norm_num [WeierstrassCurve.Φ, WeierstrassCurve.ΨSq,
    WeierstrassCurve.preΨ, preNormEDS, preNormEDS',
    WeierstrassCurve.Ψ₃, WeierstrassCurve.preΨ₄,
    WeierstrassCurve.Ψ₂Sq, WeierstrassCurve.b₂, WeierstrassCurve.b₄,
    WeierstrassCurve.b₆, WeierstrassCurve.b₈] at heval

/-- Instantiate the third successor stencil at `prePsi9,prePsi10`. -/
theorem shortWeierstrass_prePsiNineTen_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.preΨ' 9) (W.preΨ' 10) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h34 : IsCoprime W.Ψ₃ W.preΨ₄ :=
    shortWeierstrass_prePsiThreeFour_isCoprime A B hdisc h2
  have h45 : IsCoprime W.preΨ₄ (W.preΨ' 5) :=
    shortWeierstrass_prePsiFourFive_isCoprime A B hdisc h2
  have h3two : IsCoprime W.Ψ₃ W.Ψ₂Sq :=
    shortWeierstrassPsiThree_PsiTwoSq_isCoprime A B hdisc h2
  have h5 : W.preΨ' 5 = W.preΨ₄ * W.Ψ₂Sq ^ 2 - W.Ψ₃ ^ 3 := by
    rw [show 5 = 2 * (0 + 2) + 1 by norm_num, WeierstrassCurve.preΨ'_odd]
    simp
  have h6 : W.preΨ' 6 = W.Ψ₃ * (W.preΨ' 5 - W.preΨ₄ ^ 2) := by
    rw [show 6 = 2 * (0 + 3) by norm_num, WeierstrassCurve.preΨ'_even]
    simp
    ring
  have h7 : W.preΨ' 7 =
      W.preΨ' 5 * W.Ψ₃ ^ 3 - W.preΨ₄ ^ 3 * W.Ψ₂Sq ^ 2 := by
    rw [show 7 = 2 * (1 + 2) + 1 by norm_num, WeierstrassCurve.preΨ'_odd]
    norm_num
  have h9 : W.preΨ' 9 = W.preΨ' 6 * W.preΨ₄ ^ 3 * W.Ψ₂Sq ^ 2 -
      W.Ψ₃ * W.preΨ' 5 ^ 3 := by
    rw [show 9 = 2 * (2 + 2) + 1 by norm_num, WeierstrassCurve.preΨ'_odd]
    norm_num
  have h10 : W.preΨ' 10 = W.preΨ' 5 *
      (W.preΨ₄ ^ 2 * W.preΨ' 7 - W.Ψ₃ * W.preΨ' 6 ^ 2) := by
    rw [show 10 = 2 * (2 + 3) by norm_num, WeierstrassCurve.preΨ'_even]
    norm_num
    ring
  change IsCoprime (W.preΨ' 9) (W.preΨ' 10)
  rw [h9, h10, h7, h6]
  exact edsTwoFactorThirdSuccessor_isCoprime W.Ψ₃ W.preΨ₄
    (W.preΨ' 5) W.Ψ₂Sq h5 h34 h3two h45

/-- Scalar nine has no numerator/denominator cancellation on every
nonsingular short curve outside characteristic two. -/
theorem shortWeierstrassScalarNine_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.Φ 9) (W.ΨSq 9) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h89 : IsCoprime (W.preΨ' 8) (W.preΨ' 9) :=
    shortWeierstrass_prePsiEightNine_isCoprime A B hdisc h2
  have h910 : IsCoprime (W.preΨ' 9) (W.preΨ' 10) :=
    shortWeierstrass_prePsiNineTen_isCoprime A B hdisc h2
  have hden9 : W.ΨSq (9 : ℤ) = W.preΨ' 9 ^ 2 := by
    rw [show (9 : ℤ) = ((9 : ℕ) : ℤ) by norm_num,
      WeierstrassCurve.ΨSq_ofNat]
    norm_num
  have hplus : IsCoprime (W.preΨ (10 : ℕ)) (W.ΨSq (9 : ℕ)) := by
    rw [WeierstrassCurve.preΨ_ofNat]
    change IsCoprime (W.preΨ' 10) (W.ΨSq (9 : ℤ))
    rw [hden9]
    exact h910.symm.pow_right
  have hminus : IsCoprime (W.preΨ (8 : ℕ)) (W.ΨSq (9 : ℕ)) := by
    rw [WeierstrassCurve.preΨ_ofNat]
    change IsCoprime (W.preΨ' 8) (W.ΨSq (9 : ℤ))
    rw [hden9]
    exact h89.pow_right
  convert
    (shortWeierstrassPhi_isCoprime_of_adjacent A B hdisc h2 9 hplus hminus) using 1

/-- The reduced scalar-nine x-coordinate quotient has coordinate extension
degree exactly eighty-one outside characteristics two and three. -/
theorem shortWeierstrassScalarNine_extension_degree
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) (h3 : (3 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    let xMap : RatFunc F :=
      algebraMap (Polynomial F) (RatFunc F) (W.Φ 9) /
        algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 9)
    Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 81 := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  let xMap : RatFunc F :=
    algebraMap (Polynomial F) (RatFunc F) (W.Φ 9) /
      algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 9)
  have h9z : ((9 : ℤ) : F) ≠ 0 := by
    norm_num
    rw [show (9 : F) = 3 * 3 by norm_num]
    exact mul_ne_zero h3 h3
  have hcoprime : IsCoprime (W.Φ 9) (W.ΨSq 9) :=
    shortWeierstrassScalarNine_isCoprime A B hdisc h2
  have hdenom : W.ΨSq 9 ≠ 0 := W.ΨSq_ne_zero h9z
  have hextension := reducedRationalQuotient_extension_degree
    (W.Φ 9) (W.ΨSq 9) hdenom hcoprime
  have hdegrees := scalarMultiplicationXMap_exact_degrees W 9 h9z
  change Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 81
  rw [hextension]
  norm_num at hdegrees ⊢
  omega

/-- Singular scalar-nine control: at `A=B=0`, numerator and denominator both
vanish at zero and hence are not coprime. -/
theorem shortWeierstrassScalarNine_singular_control :
    let W : WeierstrassCurve (ZMod 11) := WeierstrassCurve.mk 0 0 0 0 0
    ¬IsCoprime (W.Φ 9) (W.ΨSq 9) := by
  let _ : Fact (Nat.Prime 11) := Fact.mk (by norm_num)
  let W : WeierstrassCurve (ZMod 11) := WeierstrassCurve.mk 0 0 0 0 0
  dsimp only
  intro h
  have heval := h.map (Polynomial.evalRingHom (0 : ZMod 11))
  norm_num [WeierstrassCurve.Φ, WeierstrassCurve.ΨSq,
    WeierstrassCurve.preΨ, preNormEDS, preNormEDS',
    WeierstrassCurve.Ψ₃, WeierstrassCurve.preΨ₄,
    WeierstrassCurve.Ψ₂Sq, WeierstrassCurve.b₂, WeierstrassCurve.b₄,
    WeierstrassCurve.b₆, WeierstrassCurve.b₈] at heval

/-- Instantiate the fourth successor stencil at `prePsi10,prePsi11`. -/
theorem shortWeierstrass_prePsiTenEleven_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.preΨ' 10) (W.preΨ' 11) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h34 : IsCoprime W.Ψ₃ W.preΨ₄ :=
    shortWeierstrass_prePsiThreeFour_isCoprime A B hdisc h2
  have h45 : IsCoprime W.preΨ₄ (W.preΨ' 5) :=
    shortWeierstrass_prePsiFourFive_isCoprime A B hdisc h2
  have h3two : IsCoprime W.Ψ₃ W.Ψ₂Sq :=
    shortWeierstrassPsiThree_PsiTwoSq_isCoprime A B hdisc h2
  have h5 : W.preΨ' 5 = W.preΨ₄ * W.Ψ₂Sq ^ 2 - W.Ψ₃ ^ 3 := by
    rw [show 5 = 2 * (0 + 2) + 1 by norm_num, WeierstrassCurve.preΨ'_odd]
    simp
  have h6 : W.preΨ' 6 = W.Ψ₃ * (W.preΨ' 5 - W.preΨ₄ ^ 2) := by
    rw [show 6 = 2 * (0 + 3) by norm_num, WeierstrassCurve.preΨ'_even]
    simp
    ring
  have h7 : W.preΨ' 7 =
      W.preΨ' 5 * W.Ψ₃ ^ 3 - W.preΨ₄ ^ 3 * W.Ψ₂Sq ^ 2 := by
    rw [show 7 = 2 * (1 + 2) + 1 by norm_num, WeierstrassCurve.preΨ'_odd]
    norm_num
  have h10 : W.preΨ' 10 = W.preΨ' 5 *
      (W.preΨ₄ ^ 2 * W.preΨ' 7 - W.Ψ₃ * W.preΨ' 6 ^ 2) := by
    rw [show 10 = 2 * (2 + 3) by norm_num, WeierstrassCurve.preΨ'_even]
    norm_num
    ring
  have h11 : W.preΨ' 11 = W.preΨ' 7 * W.preΨ' 5 ^ 3 -
      W.preΨ₄ * W.preΨ' 6 ^ 3 * W.Ψ₂Sq ^ 2 := by
    rw [show 11 = 2 * (3 + 2) + 1 by norm_num, WeierstrassCurve.preΨ'_odd]
    norm_num
  change IsCoprime (W.preΨ' 10) (W.preΨ' 11)
  rw [h10, h11, h7, h6]
  exact edsTwoFactorFourthSuccessor_isCoprime W.Ψ₃ W.preΨ₄
    (W.preΨ' 5) W.Ψ₂Sq h5 h34 h3two h45

/-- Scalar ten has no numerator/denominator cancellation on every nonsingular
short curve outside characteristic two. -/
theorem shortWeierstrassScalarTen_isCoprime
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.Φ 10) (W.ΨSq 10) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h910 : IsCoprime (W.preΨ' 9) (W.preΨ' 10) :=
    shortWeierstrass_prePsiNineTen_isCoprime A B hdisc h2
  have h1011 : IsCoprime (W.preΨ' 10) (W.preΨ' 11) :=
    shortWeierstrass_prePsiTenEleven_isCoprime A B hdisc h2
  have h9two : IsCoprime (W.preΨ' 9) W.Ψ₂Sq :=
    shortWeierstrass_prePsi_odd_isCoprime_PsiTwoSq A B hdisc h2 9 (by decide)
  have h11two : IsCoprime (W.preΨ' 11) W.Ψ₂Sq :=
    shortWeierstrass_prePsi_odd_isCoprime_PsiTwoSq A B hdisc h2 11 (by decide)
  have hden10 : W.ΨSq (10 : ℤ) = W.preΨ' 10 ^ 2 * W.Ψ₂Sq := by
    rw [show (10 : ℤ) = ((10 : ℕ) : ℤ) by norm_num,
      WeierstrassCurve.ΨSq_ofNat]
    norm_num
  have hplus : IsCoprime (W.preΨ (11 : ℕ)) (W.ΨSq (10 : ℕ)) := by
    rw [WeierstrassCurve.preΨ_ofNat]
    change IsCoprime (W.preΨ' 11) (W.ΨSq (10 : ℤ))
    rw [hden10]
    exact h1011.symm.pow_right.mul_right h11two
  have hminus : IsCoprime (W.preΨ (9 : ℕ)) (W.ΨSq (10 : ℕ)) := by
    rw [WeierstrassCurve.preΨ_ofNat]
    change IsCoprime (W.preΨ' 9) (W.ΨSq (10 : ℤ))
    rw [hden10]
    exact h910.pow_right.mul_right h9two
  convert
    (shortWeierstrassPhi_isCoprime_of_adjacent A B hdisc h2 10 hplus hminus) using 1

/-- The reduced scalar-ten x-coordinate quotient has coordinate extension
degree exactly one hundred outside characteristics two and five. -/
theorem shortWeierstrassScalarTen_extension_degree
    {F : Type*} [Field F] (A B : F)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0) (h5 : (5 : F) ≠ 0) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    let xMap : RatFunc F :=
      algebraMap (Polynomial F) (RatFunc F) (W.Φ 10) /
        algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 10)
    Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 100 := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  let xMap : RatFunc F :=
    algebraMap (Polynomial F) (RatFunc F) (W.Φ 10) /
      algebraMap (Polynomial F) (RatFunc F) (W.ΨSq 10)
  have h10z : ((10 : ℤ) : F) ≠ 0 := by
    norm_num
    rw [show (10 : F) = 2 * 5 by norm_num]
    exact mul_ne_zero h2 h5
  have hcoprime : IsCoprime (W.Φ 10) (W.ΨSq 10) :=
    shortWeierstrassScalarTen_isCoprime A B hdisc h2
  have hdenom : W.ΨSq 10 ≠ 0 := W.ΨSq_ne_zero h10z
  have hextension := reducedRationalQuotient_extension_degree
    (W.Φ 10) (W.ΨSq 10) hdenom hcoprime
  have hdegrees := scalarMultiplicationXMap_exact_degrees W 10 h10z
  change Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = 100
  rw [hextension]
  norm_num at hdegrees ⊢
  omega

/-- Singular scalar-ten control: at `A=B=0`, numerator and denominator both
vanish at zero and hence are not coprime. -/
theorem shortWeierstrassScalarTen_singular_control :
    let W : WeierstrassCurve (ZMod 11) := WeierstrassCurve.mk 0 0 0 0 0
    ¬IsCoprime (W.Φ 10) (W.ΨSq 10) := by
  let _ : Fact (Nat.Prime 11) := Fact.mk (by norm_num)
  let W : WeierstrassCurve (ZMod 11) := WeierstrassCurve.mk 0 0 0 0 0
  dsimp only
  intro h
  have heval := h.map (Polynomial.evalRingHom (0 : ZMod 11))
  norm_num [WeierstrassCurve.Φ, WeierstrassCurve.ΨSq,
    WeierstrassCurve.preΨ, preNormEDS, preNormEDS',
    WeierstrassCurve.Ψ₃, WeierstrassCurve.preΨ₄,
    WeierstrassCurve.Ψ₂Sq, WeierstrassCurve.b₂, WeierstrassCurve.b₄,
    WeierstrassCurve.b₆, WeierstrassCurve.b₈] at heval

/-- Schmidt's universal multiplication-polynomial resultant formula, once
aligned with Mathlib's exact `Φ,ΨSq` conventions, implies scalar
numerator/denominator coprimality for every natural index. -/
theorem shortWeierstrassScalar_isCoprime_of_resultantFormula
    {F : Type*} [Field F] (A B : F) (n : ℕ)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0)
    (hresultant :
      let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
      Polynomial.resultant (W.Φ (n : ℤ)) (W.ΨSq (n : ℤ)) =
        ((-16 : F) * (4 * A ^ 3 + 27 * B ^ 2)) ^
          (n ^ 2 * (n ^ 2 - 1) / 6)) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    IsCoprime (W.Φ (n : ℤ)) (W.ΨSq (n : ℤ)) := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  have h16 : (-16 : F) ≠ 0 := by
    rw [show (-16 : F) = -(2 ^ 4) by norm_num]
    exact neg_ne_zero.mpr (pow_ne_zero 4 h2)
  have hbase : (-16 : F) * (4 * A ^ 3 + 27 * B ^ 2) ≠ 0 :=
    mul_ne_zero h16 hdisc
  have hresultantNe :
      Polynomial.resultant (W.Φ (n : ℤ)) (W.ΨSq (n : ℤ)) ≠ 0 := by
    rw [hresultant]
    exact pow_ne_zero _ hbase
  have hmonic : (W.Φ (n : ℤ)).Monic := W.leadingCoeff_Φ (n : ℤ)
  rw [← Polynomial.isUnit_resultant_iff_isCoprime hmonic]
  exact isUnit_iff_ne_zero.mpr hresultantNe

/-- Under the same source-linked resultant certificate, the reduced scalar
x-coordinate quotient has exact coordinate extension degree `n²`. -/
theorem shortWeierstrassScalar_extension_degree_of_resultantFormula
    {F : Type*} [Field F] (A B : F) (n : ℕ)
    (hdisc : 4 * A ^ 3 + 27 * B ^ 2 ≠ 0)
    (h2 : (2 : F) ≠ 0)
    (hn : (((n : ℕ) : ℤ) : F) ≠ 0)
    (hresultant :
      let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
      Polynomial.resultant (W.Φ (n : ℤ)) (W.ΨSq (n : ℤ)) =
        ((-16 : F) * (4 * A ^ 3 + 27 * B ^ 2)) ^
          (n ^ 2 * (n ^ 2 - 1) / 6)) :
    let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
    let xMap : RatFunc F :=
      algebraMap (Polynomial F) (RatFunc F) (W.Φ (n : ℤ)) /
        algebraMap (Polynomial F) (RatFunc F) (W.ΨSq (n : ℤ))
    Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = n ^ 2 := by
  let W : WeierstrassCurve F := WeierstrassCurve.mk 0 0 0 A B
  let xMap : RatFunc F :=
    algebraMap (Polynomial F) (RatFunc F) (W.Φ (n : ℤ)) /
      algebraMap (Polynomial F) (RatFunc F) (W.ΨSq (n : ℤ))
  have hcoprime : IsCoprime (W.Φ (n : ℤ)) (W.ΨSq (n : ℤ)) :=
    shortWeierstrassScalar_isCoprime_of_resultantFormula
      A B n hdisc h2 hresultant
  have hdenom : W.ΨSq (n : ℤ) ≠ 0 := W.ΨSq_ne_zero hn
  have hextension := reducedRationalQuotient_extension_degree
    (W.Φ (n : ℤ)) (W.ΨSq (n : ℤ)) hdenom hcoprime
  have hdegrees := scalarMultiplicationXMap_exact_degrees W (n : ℤ) hn
  change Module.finrank (IntermediateField.adjoin F {xMap}) (RatFunc F) = n ^ 2
  rw [hextension]
  norm_num at hdegrees ⊢
  omega

/-- Over an algebraically closed field, absence of a common root is an exact
certificate that two polynomials are coprime. -/
theorem polynomial_isCoprime_of_no_common_root
    {F : Type*} [Field F] [IsAlgClosed F] {p q : Polynomial F}
    (hroot : ∀ x : F, p.IsRoot x → q.IsRoot x → False) :
    IsCoprime p q := by
  classical
  apply EuclideanDomain.gcd_isUnit_iff.mp
  rw [Polynomial.isUnit_iff_degree_eq_zero]
  by_contra hdegree
  let x := Classical.choose
    (IsAlgClosed.exists_root (EuclideanDomain.gcd p q) hdegree)
  have hx := Classical.choose_spec
    (IsAlgClosed.exists_root (EuclideanDomain.gcd p q) hdegree)
  exact hroot x
    (hx.dvd (EuclideanDomain.gcd_dvd_left p q))
    (hx.dvd (EuclideanDomain.gcd_dvd_right p q))

/-- A point killed by adjacent integer scalars is zero in any additive
commutative group. -/
theorem adjacentIntegerTorsion_eq_zero
    {G : Type*} [AddCommGroup G] (n : ℤ) (P : G)
    (hn : n • P = 0) (hnext : (n + 1) • P = 0) : P = 0 := by
  calc
    P = (n + 1) • P - n • P := by
      rw [add_zsmul]
      simp
    _ = 0 := by rw [hnext, hn, sub_zero]

/-- Root semantics bridge: if every common polynomial root produces a nonzero
group element killed by adjacent integers, then the polynomials are coprime. -/
theorem polynomial_isCoprime_of_adjacent_torsion_root_semantics
    {F : Type*} [Field F] [IsAlgClosed F] {p q : Polynomial F}
    {G : Type*} [AddCommGroup G] (n : ℤ)
    (hsemantic : ∀ x : F, p.IsRoot x → q.IsRoot x →
      ∃ P : G, P ≠ 0 ∧ n • P = 0 ∧ (n + 1) • P = 0) :
    IsCoprime p q := by
  apply polynomial_isCoprime_of_no_common_root
  intro x hpx hqx
  let witness := hsemantic x hpx hqx
  let P := Classical.choose witness
  have hspec := Classical.choose_spec witness
  have hP := hspec.1
  have hn := hspec.2.1
  have hnext := hspec.2.2
  exact hP (adjacentIntegerTorsion_eq_zero n P hn hnext)

/-- Base-field root criterion: it is enough to exclude common roots after
mapping both polynomials into any algebraically closed field extension. -/
theorem polynomial_isCoprime_of_no_common_root_after_map
    {F E : Type*} [Field F] [Field E] [IsAlgClosed E]
    (f : F →+* E) {p q : Polynomial F}
    (hroot : ∀ x : E, (p.map f).IsRoot x → (q.map f).IsRoot x → False) :
    IsCoprime p q := by
  rw [← Polynomial.isCoprime_map f]
  exact polynomial_isCoprime_of_no_common_root hroot

/-- Base-field semantic bridge: adjacent-torsion evidence attached to common
roots after algebraic-closure base change proves coprimality before the base
change. -/
theorem polynomial_isCoprime_of_adjacent_torsion_root_semantics_after_map
    {F E : Type*} [Field F] [Field E] [IsAlgClosed E]
    (f : F →+* E) {p q : Polynomial F}
    {G : Type*} [AddCommGroup G] (n : ℤ)
    (hsemantic : ∀ x : E, (p.map f).IsRoot x → (q.map f).IsRoot x →
      ∃ P : G, P ≠ 0 ∧ n • P = 0 ∧ (n + 1) • P = 0) :
    IsCoprime p q := by
  rw [← Polynomial.isCoprime_map f]
  exact polynomial_isCoprime_of_adjacent_torsion_root_semantics n hsemantic

/-- Every x-coordinate over an algebraically closed field lies below a point
of an arbitrary affine Weierstrass equation. -/
theorem weierstrass_exists_equation_point_above
    {E : Type*} [Field E] [IsAlgClosed E]
    (W : WeierstrassCurve.Affine E) (x : E) :
    ∃ y : E, W.Equation x y := by
  let c := W.a₁ * x + W.a₃
  let r := x ^ 3 + W.a₂ * x ^ 2 + W.a₄ * x + W.a₆
  let g : E[X] := X ^ 2 + C c * X - C r
  have hgdeg : g.degree ≠ 0 := by
    have hc : g.coeff 2 ≠ 0 := by
      dsimp [g]
      simp
    intro hdegree
    have hle := Polynomial.le_degree_of_ne_zero hc
    rw [hdegree] at hle
    norm_num at hle
  obtain ⟨y, hy⟩ := IsAlgClosed.exists_root g hgdeg
  refine ⟨y, (W.equation_iff x y).mpr ?_⟩
  rw [IsRoot.def] at hy
  dsimp [g, c, r] at hy ⊢
  simp only [eval_sub, eval_add, eval_mul, eval_pow, eval_X, eval_C] at hy
  linear_combination hy

/-- On an elliptic curve, the lifted affine equation point is a nonzero
element of Mathlib's affine point group. -/
theorem affineWeierstrass_exists_nonzero_point_above
    {E : Type*} [Field E] [DecidableEq E] [IsAlgClosed E]
    (W : WeierstrassCurve.Affine E) [W.IsElliptic] (x : E) :
    ∃ y : E, ∃ h : W.Equation x y,
      WeierstrassCurve.Affine.Point.mk h ≠ 0 := by
  obtain ⟨y, hxy⟩ := weierstrass_exists_equation_point_above W x
  refine ⟨y, hxy, ?_⟩
  exact WeierstrassCurve.Affine.Point.some_ne_zero _

/-- Root-to-point assembly after base change: once the actual pointwise
division-polynomial scalar equations are supplied, adjacent-root coprimality
holds over the original coefficient field. -/
theorem polynomial_isCoprime_of_adjacent_torsion_curve_semantics_after_map
    {F E : Type*} [Field F] [Field E] [DecidableEq E] [IsAlgClosed E]
    (f : F →+* E) {p q : Polynomial F}
    (W : WeierstrassCurve.Affine E) [W.IsElliptic] (n : ℤ)
    (hscalar : ∀ x y : E, ∀ h : W.Equation x y,
      (p.map f).IsRoot x → (q.map f).IsRoot x →
      n • WeierstrassCurve.Affine.Point.mk h = 0 ∧
        (n + 1) • WeierstrassCurve.Affine.Point.mk h = 0) :
    IsCoprime p q := by
  apply polynomial_isCoprime_of_adjacent_torsion_root_semantics_after_map
    (G := W.Point) f n
  intro x hpx hqx
  obtain ⟨y, hxy, hnonzero⟩ :=
    affineWeierstrass_exists_nonzero_point_above W x
  exact ⟨WeierstrassCurve.Affine.Point.mk hxy, hnonzero,
    (hscalar x y hxy hpx hqx).1, (hscalar x y hxy hpx hqx).2⟩

/-- Algebraic closure is essential for unconditional lifting: over `F_5`,
the nonsingular short equation `y^2=x^3+x+1` has no point above `x=1`. -/
theorem finite_field_point_lift_control :
    ¬∃ y : ZMod 5,
      (WeierstrassCurve.mk 0 0 0 1 1 :
        WeierstrassCurve (ZMod 5)).toAffine.Equation 1 y := by
  rintro ⟨y, hy⟩
  rw [WeierstrassCurve.Affine.equation_iff] at hy
  dsimp [WeierstrassCurve.toAffine] at hy
  norm_num at hy
  fin_cases y <;> contradiction

/-- Boundary control: the zero point always satisfies both adjacent torsion
equations, so the nonzero clause in the root-semantics bridge is essential. -/
theorem adjacentIntegerTorsion_zero_control
    {G : Type*} [AddGroup G] (n : ℤ) :
    n • (0 : G) = 0 ∧ (n + 1) • (0 : G) = 0 := by
  simp

/-- Specialize the Frobenius--Verschiebung ring calculation to additive
endomorphisms of a point group. Integer scalars are central automatically, and
pointwise injectivity of Frobenius supplies cancellation of left composition.
Thus the only remaining elliptic certificates are the concrete norm and
characteristic-polynomial identities (plus construction and injectivity of the
concrete Frobenius map). -/
theorem endomorphism_frobeniusVerschiebung_sum
    {G : Type*} [AddCommGroup G]
    (pi verschiebung : AddMonoid.End G) (trace cardinality : ℤ)
    (hpi : Function.Injective pi)
    (hnorm : pi * verschiebung = (cardinality : AddMonoid.End G))
    (hchar : pi ^ 2 - (trace : AddMonoid.End G) * pi +
        (cardinality : AddMonoid.End G) = 0) :
    verschiebung + pi = (trace : AddMonoid.End G) := by
  apply frobeniusVerschiebung_sum_of_characteristic
      (pi := pi) (verschiebung := verschiebung)
      (trace := (trace : AddMonoid.End G))
      (cardinality := (cardinality : AddMonoid.End G))
  · intro a b hab
    ext point
    apply hpi
    exact DFunLike.congr_fun hab point
  · rw [Commute]
    ext point
    change pi (trace • point) = trace • pi point
    exact pi.map_zsmul trace point
  · exact hnorm
  · exact hchar

/-- Frozen mutation control showing why pointwise injectivity cannot simply be
dropped from `endomorphism_frobeniusVerschiebung_sum`. On the additive group
`ZMod 2`, zero maps satisfy the displayed norm and characteristic equations
for cardinality two and trace one, but their sum is not the trace scalar map. -/
theorem endomorphism_injectivity_control :
    let pi : AddMonoid.End (ZMod 2) := 0
    let verschiebung : AddMonoid.End (ZMod 2) := 0
    pi * verschiebung = (2 : AddMonoid.End (ZMod 2)) ∧
      pi ^ 2 - (1 : AddMonoid.End (ZMod 2)) * pi +
          (2 : AddMonoid.End (ZMod 2)) = 0 ∧
      verschiebung + pi ≠ (1 : AddMonoid.End (ZMod 2)) := by
  dsimp
  constructor
  · ext point
    change 0 = (2 : ℤ) • point
    exact (CharTwo.two_zsmul point).symm
  constructor
  · rw [pow_two]
    simp only [zero_mul, one_mul, sub_zero, zero_add]
    ext point
    change (2 : ℤ) • point = 0
    exact CharTwo.two_zsmul point
  · intro h
    have h1 := DFunLike.congr_fun h (1 : ZMod 2)
    norm_num at h1

/-- Coordinatewise finite-field Frobenius as an actual additive endomorphism
of Mathlib's nonsingular affine points. The curve is defined over the finite
field `k`, then base-changed to the field `E`. -/
noncomputable def affineFrobeniusPointEnd
    {k E : Type*} [Field k] [Fintype k] [Field E] [DecidableEq E]
    [Algebra k E] (W : WeierstrassCurve.Affine k) :
    AddMonoid.End
      (WeierstrassCurve.Affine.Point (W.baseChange E)) :=
  WeierstrassCurve.Affine.Point.map (W' := W)
    (FiniteField.frobeniusAlgHom k E)

/-- The coordinatewise finite-field Frobenius on affine elliptic points is
injective. This is inherited from Mathlib's injectivity theorem for the point
map induced by a field algebra homomorphism. -/
theorem affineFrobeniusPointEnd_injective
    {k E : Type*} [Field k] [Fintype k] [Field E] [DecidableEq E]
    [Algebra k E] (W : WeierstrassCurve.Affine k) :
    Function.Injective (affineFrobeniusPointEnd (E := E) W) := by
  exact WeierstrassCurve.Affine.Point.map_injective (W' := W)
    (FiniteField.frobeniusAlgHom k E)

/-- On a finite-field base change, the point Frobenius raises both affine
coordinates to the cardinality of the base field. -/
theorem affineFrobeniusPointEnd_some
    {k E : Type*} [Field k] [Fintype k] [Field E] [DecidableEq E]
    [Algebra k E] (W : WeierstrassCurve.Affine k) {x y : E}
    (h : WeierstrassCurve.Affine.Nonsingular (W.baseChange E) x y) :
    affineFrobeniusPointEnd (E := E) W
        (WeierstrassCurve.Affine.Point.some x y h) =
      WeierstrassCurve.Affine.Point.some
        (x ^ Fintype.card k) (y ^ Fintype.card k)
        ((W.baseChange_nonsingular
          (FiniteField.frobeniusAlgHom k E).injective ..).mpr h) := by
  rfl

/-- The Frobenius--Verschiebung sum identity on Mathlib's actual affine point
group. Frobenius construction and injectivity are now internal; only the
Verschiebung, norm, and characteristic equations remain explicit. -/
theorem affinePoint_frobeniusVerschiebung_sum
    {k E : Type*} [Field k] [Fintype k] [Field E] [DecidableEq E]
    [Algebra k E] (W : WeierstrassCurve.Affine k)
    (verschiebung : AddMonoid.End
      (WeierstrassCurve.Affine.Point (W.baseChange E))) (trace : ℤ)
    (hnorm : affineFrobeniusPointEnd (E := E) W * verschiebung =
      ((Fintype.card k : ℤ) : AddMonoid.End
        (WeierstrassCurve.Affine.Point (W.baseChange E))))
    (hchar : affineFrobeniusPointEnd (E := E) W ^ 2 -
        (trace : AddMonoid.End
          (WeierstrassCurve.Affine.Point (W.baseChange E))) *
            affineFrobeniusPointEnd (E := E) W +
        ((Fintype.card k : ℤ) : AddMonoid.End
          (WeierstrassCurve.Affine.Point (W.baseChange E))) = 0) :
    verschiebung + affineFrobeniusPointEnd (E := E) W =
      (trace : AddMonoid.End
        (WeierstrassCurve.Affine.Point (W.baseChange E))) := by
  exact endomorphism_frobeniusVerschiebung_sum
    (affineFrobeniusPointEnd (E := E) W) verschiebung trace (Fintype.card k)
    (affineFrobeniusPointEnd_injective (E := E) W) hnorm hchar

/-- Over a prime field, the actual affine-point Frobenius sends coordinates
exactly to their `p`-th powers. -/
theorem zmodAffineFrobeniusPointEnd_some
    {p : ℕ} [Fact p.Prime]
    {E : Type*} [Field E] [DecidableEq E] [Algebra (ZMod p) E]
    (W : WeierstrassCurve.Affine (ZMod p)) {x y : E}
    (h : WeierstrassCurve.Affine.Nonsingular (W.baseChange E) x y) :
    affineFrobeniusPointEnd (E := E) W
        (WeierstrassCurve.Affine.Point.some x y h) =
      WeierstrassCurve.Affine.Point.some
        (x ^ p) (y ^ p)
        (by
          simpa only [FiniteField.coe_frobeniusAlgHom, ZMod.card] using
            ((W.baseChange_nonsingular
              (FiniteField.frobeniusAlgHom (ZMod p) E).injective ..).mpr h)) := by
  simpa only [ZMod.card] using affineFrobeniusPointEnd_some W h

/-- Prime-field specialization: on the actual affine point group,
`V + pi = [trace]` follows from the norm and characteristic equations alone;
the coordinatewise `p`-power Frobenius and its injectivity are constructed by
the preceding declarations. -/
theorem zmodAffinePoint_frobeniusVerschiebung_sum
    {p : ℕ} [Fact p.Prime]
    {E : Type*} [Field E] [DecidableEq E] [Algebra (ZMod p) E]
    (W : WeierstrassCurve.Affine (ZMod p))
    (verschiebung : AddMonoid.End
      (WeierstrassCurve.Affine.Point (W.baseChange E))) (trace : ℤ)
    (hnorm : affineFrobeniusPointEnd (E := E) W * verschiebung =
      ((p : ℤ) : AddMonoid.End
        (WeierstrassCurve.Affine.Point (W.baseChange E))))
    (hchar : affineFrobeniusPointEnd (E := E) W ^ 2 -
        (trace : AddMonoid.End
          (WeierstrassCurve.Affine.Point (W.baseChange E))) *
            affineFrobeniusPointEnd (E := E) W +
        ((p : ℤ) : AddMonoid.End
          (WeierstrassCurve.Affine.Point (W.baseChange E))) = 0) :
    verschiebung + affineFrobeniusPointEnd (E := E) W =
      (trace : AddMonoid.End
        (WeierstrassCurve.Affine.Point (W.baseChange E))) := by
  have hnorm' : affineFrobeniusPointEnd (E := E) W * verschiebung =
      ((Fintype.card (ZMod p) : ℤ) : AddMonoid.End
        (WeierstrassCurve.Affine.Point (W.baseChange E))) := by
    simpa only [ZMod.card] using hnorm
  have hchar' : affineFrobeniusPointEnd (E := E) W ^ 2 -
        (trace : AddMonoid.End
          (WeierstrassCurve.Affine.Point (W.baseChange E))) *
            affineFrobeniusPointEnd (E := E) W +
        ((Fintype.card (ZMod p) : ℤ) : AddMonoid.End
          (WeierstrassCurve.Affine.Point (W.baseChange E))) = 0 := by
    simpa only [ZMod.card] using hchar
  exact affinePoint_frobeniusVerschiebung_sum W verschiebung trace hnorm' hchar'

/-- Inverse Frobenius on the affine point group over an algebraic extension of
the prime field. Mathlib supplies the field-level Frobenius algebra
automorphism; this definition maps its inverse functorially to points. -/
noncomputable def zmodAffineFrobeniusPointInvEnd
    {p : ℕ} [Fact p.Prime]
    {E : Type*} [Field E] [DecidableEq E] [Algebra (ZMod p) E]
    [Algebra.IsAlgebraic (ZMod p) E]
    (W : WeierstrassCurve.Affine (ZMod p)) :
    AddMonoid.End (WeierstrassCurve.Affine.Point (W.baseChange E)) :=
  WeierstrassCurve.Affine.Point.map (W' := W)
    (FiniteField.frobeniusAlgEquivOfAlgebraic (ZMod p) E).symm.toAlgHom

/-- Frobenius composed with the point map induced by inverse Frobenius is the
identity endomorphism. -/
theorem zmodAffineFrobeniusPointEnd_mul_inv
    {p : ℕ} [Fact p.Prime]
    {E : Type*} [Field E] [DecidableEq E] [Algebra (ZMod p) E]
    [Algebra.IsAlgebraic (ZMod p) E]
    (W : WeierstrassCurve.Affine (ZMod p)) :
    affineFrobeniusPointEnd (E := E) W *
      zmodAffineFrobeniusPointInvEnd W = 1 := by
  ext point
  change WeierstrassCurve.Affine.Point.map
      (FiniteField.frobeniusAlgHom (ZMod p) E)
      (WeierstrassCurve.Affine.Point.map
        (FiniteField.frobeniusAlgEquivOfAlgebraic
          (ZMod p) E).symm.toAlgHom point) = point
  rw [WeierstrassCurve.Affine.Point.map_map]
  have hcomp : (FiniteField.frobeniusAlgHom (ZMod p) E).comp
      (FiniteField.frobeniusAlgEquivOfAlgebraic
        (ZMod p) E).symm.toAlgHom =
      AlgHom.id (ZMod p) E := by
    ext x
    exact (FiniteField.frobeniusAlgEquivOfAlgebraic
      (ZMod p) E).apply_symm_apply x
  rw [hcomp]
  cases point <;> rfl

/-- Pointwise Verschiebung over an algebraic prime-field extension, defined as
`[p]` followed by inverse Frobenius. This is an additive point endomorphism;
identifying it with a particular rational or scheme morphism remains a
separate semantic obligation. -/
noncomputable def zmodAffineVerschiebungPointEnd
    {p : ℕ} [Fact p.Prime]
    {E : Type*} [Field E] [DecidableEq E] [Algebra (ZMod p) E]
    [Algebra.IsAlgebraic (ZMod p) E]
    (W : WeierstrassCurve.Affine (ZMod p)) :
    AddMonoid.End (WeierstrassCurve.Affine.Point (W.baseChange E)) :=
  (p : AddMonoid.End
      (WeierstrassCurve.Affine.Point (W.baseChange E))) *
    zmodAffineFrobeniusPointInvEnd W

/-- The constructed pointwise Verschiebung satisfies the exact norm relation
`pi * V = [p]`. -/
theorem zmodAffineFrobenius_mul_verschiebung
    {p : ℕ} [Fact p.Prime]
    {E : Type*} [Field E] [DecidableEq E] [Algebra (ZMod p) E]
    [Algebra.IsAlgebraic (ZMod p) E]
    (W : WeierstrassCurve.Affine (ZMod p)) :
    affineFrobeniusPointEnd (E := E) W *
      zmodAffineVerschiebungPointEnd W =
        (p : AddMonoid.End
          (WeierstrassCurve.Affine.Point (W.baseChange E))) := by
  rw [zmodAffineVerschiebungPointEnd, ← mul_assoc]
  have hcomm : Commute (affineFrobeniusPointEnd (E := E) W)
      (p : AddMonoid.End
        (WeierstrassCurve.Affine.Point (W.baseChange E))) := by
    rw [Commute]
    ext point
    change affineFrobeniusPointEnd (E := E) W (p • point) =
      p • affineFrobeniusPointEnd (E := E) W point
    exact (affineFrobeniusPointEnd (E := E) W).map_nsmul p point
  rw [hcomm.eq, mul_assoc, zmodAffineFrobeniusPointEnd_mul_inv, mul_one]

/-- For the constructed pointwise Verschiebung, the Frobenius characteristic
equation is the only remaining algebraic premise needed to derive
`V + pi = [trace]` on affine points. -/
theorem zmodAffineVerschiebung_sum_of_characteristic
    {p : ℕ} [Fact p.Prime]
    {E : Type*} [Field E] [DecidableEq E] [Algebra (ZMod p) E]
    [Algebra.IsAlgebraic (ZMod p) E]
    (W : WeierstrassCurve.Affine (ZMod p)) (trace : ℤ)
    (hchar : affineFrobeniusPointEnd (E := E) W ^ 2 -
        (trace : AddMonoid.End
          (WeierstrassCurve.Affine.Point (W.baseChange E))) *
            affineFrobeniusPointEnd (E := E) W +
        ((p : ℤ) : AddMonoid.End
          (WeierstrassCurve.Affine.Point (W.baseChange E))) = 0) :
    zmodAffineVerschiebungPointEnd W +
        affineFrobeniusPointEnd (E := E) W =
      (trace : AddMonoid.End
        (WeierstrassCurve.Affine.Point (W.baseChange E))) := by
  exact zmodAffinePoint_frobeniusVerschiebung_sum W
    (zmodAffineVerschiebungPointEnd W) trace
    (zmodAffineFrobenius_mul_verschiebung W) hchar

/-- Rearrange an additive-endomorphism sum identity into the exact orientation
used by the H116 map: `V - 1 = [trace - 1] - pi`. -/
theorem endomorphism_sub_one_eq_trace_sub_one_sub
    {G : Type*} [AddCommGroup G]
    (verschiebung pi : AddMonoid.End G) (trace : ℤ)
    (hsum : verschiebung + pi = (trace : AddMonoid.End G)) :
    verschiebung - 1 =
      ((trace - 1 : ℤ) : AddMonoid.End G) - pi := by
  have hV : verschiebung = (trace : AddMonoid.End G) - pi :=
    eq_sub_of_add_eq hsum
  rw [hV, Int.cast_sub, Int.cast_one]
  abel

/-- On a point fixed by Verschiebung, the sum identity `V+pi=[trace]`
specializes Frobenius to the scalar eigenvalue `trace-1`.  This is the kernel
eigenspace mechanism exposed by the retained rational-reconstruction v1
failure. -/
theorem endomorphism_frobenius_eigen_on_verschiebung_fixed
    {G : Type*} [AddCommGroup G]
    (verschiebung pi : AddMonoid.End G) (trace : ℤ)
    (hsum : verschiebung + pi = (trace : AddMonoid.End G))
    (point : G) (hfixed : verschiebung point = point) :
    pi point = (trace - 1) • point := by
  have hpoint := DFunLike.congr_fun hsum point
  change verschiebung point + pi point = trace • point at hpoint
  rw [hfixed] at hpoint
  have hpi : pi point = trace • point - point :=
    eq_sub_of_add_eq (by simpa [add_comm] using hpoint)
  rw [hpi]
  simpa only [one_zsmul, sub_eq_add_neg] using
    (sub_zsmul point trace 1).symm

/-- Iterating an additive-endomorphism eigenrelation raises its integer
eigenvalue to the same natural power. -/
theorem endomorphism_iterate_eigen
    {G : Type*} [AddCommGroup G]
    (pi : AddMonoid.End G) (eigenvalue : ℤ) (point : G)
    (heigen : pi point = eigenvalue • point) :
    ∀ n : ℕ, (pi ^ n) point = (eigenvalue ^ n) • point := by
  intro n
  induction n with
  | zero => simp
  | succ n ih =>
      rw [pow_succ]
      change (pi ^ n) (pi point) = _
      rw [heigen, map_zsmul, ih, pow_succ]
      simp [mul_comm, mul_smul]

/-- Consequently, on `ker(V-1)`, membership in a degree-`n` Frobenius fixed
locus is exactly the scalar condition `[(trace-1)^n]P=P`. -/
theorem endomorphism_frobenius_fixedLocus_iff_scalar_on_verschiebung_fixed
    {G : Type*} [AddCommGroup G]
    (verschiebung pi : AddMonoid.End G) (trace : ℤ)
    (hsum : verschiebung + pi = (trace : AddMonoid.End G))
    (point : G) (hfixed : verschiebung point = point) (n : ℕ) :
    (pi ^ n) point = point ↔ ((trace - 1) ^ n) • point = point := by
  rw [endomorphism_iterate_eigen pi (trace - 1) point
    (endomorphism_frobenius_eigen_on_verschiebung_fixed
      verschiebung pi trace hsum point hfixed)]

/-- An additive endomorphism that vanishes on a set of additive generators
vanishes everywhere. This is the proof-carrying interface used to compress a
finite point-group characteristic check. -/
theorem endomorphism_eq_zero_of_eq_zero_on_generators
    {G : Type*} [AddCommGroup G]
    (f : AddMonoid.End G) (generators : Set G)
    (hgenerate : AddSubgroup.closure generators = ⊤)
    (hzero : ∀ point ∈ generators, f point = 0) :
    f = 0 := by
  apply AddMonoidHom.eq_of_eqOn_dense hgenerate
  intro point hpoint
  change f point = 0
  exact hzero point hpoint

/-- Checking the Frobenius characteristic expression on a generating set is
sufficient to establish the global additive-endomorphism equation. -/
theorem endomorphism_characteristic_of_generators
    {G : Type*} [AddCommGroup G]
    (pi : AddMonoid.End G) (trace cardinality : ℤ)
    (generators : Set G)
    (hgenerate : AddSubgroup.closure generators = ⊤)
    (hchar : ∀ point ∈ generators,
      (pi ^ 2 - (trace : AddMonoid.End G) * pi +
        (cardinality : AddMonoid.End G)) point = 0) :
    pi ^ 2 - (trace : AddMonoid.End G) * pi +
        (cardinality : AddMonoid.End G) = 0 := by
  exact endomorphism_eq_zero_of_eq_zero_on_generators
    _ generators hgenerate hchar

/-- The prime-field Frobenius trace attached to an affine Weierstrass curve,
defined by the exact point-count formula `p+1-#E(F_p)`. -/
noncomputable def zmodAffineTrace
    {p : ℕ} [Fact p.Prime]
    (W : WeierstrassCurve.Affine (ZMod p)) : ℤ :=
  (p : ℤ) + 1 - (Nat.card W.Point : ℤ)

/-- Under the explicit Frobenius characteristic equation, the constructed
pointwise Verschiebung has exactly the `V-1=[trace-1]-pi` orientation used by
the candidate rational map. This is still an equality of additive point
endomorphisms, not an identification of rational or scheme morphisms. -/
theorem zmodAffineVerschiebung_sub_one_eq
    {p : ℕ} [Fact p.Prime]
    {E : Type*} [Field E] [DecidableEq E] [Algebra (ZMod p) E]
    [Algebra.IsAlgebraic (ZMod p) E]
    (W : WeierstrassCurve.Affine (ZMod p)) (trace : ℤ)
    (hchar : affineFrobeniusPointEnd (E := E) W ^ 2 -
        (trace : AddMonoid.End
          (WeierstrassCurve.Affine.Point (W.baseChange E))) *
            affineFrobeniusPointEnd (E := E) W +
        ((p : ℤ) : AddMonoid.End
          (WeierstrassCurve.Affine.Point (W.baseChange E))) = 0) :
    zmodAffineVerschiebungPointEnd W - 1 =
      ((trace - 1 : ℤ) : AddMonoid.End
        (WeierstrassCurve.Affine.Point (W.baseChange E))) -
          affineFrobeniusPointEnd (E := E) W := by
  apply endomorphism_sub_one_eq_trace_sub_one_sub
  exact zmodAffineVerschiebung_sum_of_characteristic W trace hchar

/-- A proof-carrying finite-group endpoint: characteristic checks on a supplied
generating set, with trace fixed to `p+1-#E(F_p)`, imply the exact pointwise
`V-1` orientation. This does not promote a finite extension certificate to an
algebraic-closure or rational-map theorem. -/
theorem zmodAffineVerschiebung_sub_one_eq_of_generators
    {p : ℕ} [Fact p.Prime]
    {E : Type*} [Field E] [DecidableEq E] [Algebra (ZMod p) E]
    [Algebra.IsAlgebraic (ZMod p) E]
    (W : WeierstrassCurve.Affine (ZMod p))
    (generators : Set (WeierstrassCurve.Affine.Point (W.baseChange E)))
    (hgenerate : AddSubgroup.closure generators = ⊤)
    (hchar : ∀ point ∈ generators,
      (affineFrobeniusPointEnd (E := E) W ^ 2 -
        (zmodAffineTrace W : AddMonoid.End
          (WeierstrassCurve.Affine.Point (W.baseChange E))) *
            affineFrobeniusPointEnd (E := E) W +
        ((p : ℤ) : AddMonoid.End
          (WeierstrassCurve.Affine.Point (W.baseChange E)))) point = 0) :
    zmodAffineVerschiebungPointEnd W - 1 =
      ((zmodAffineTrace W - 1 : ℤ) : AddMonoid.End
        (WeierstrassCurve.Affine.Point (W.baseChange E))) -
          affineFrobeniusPointEnd (E := E) W := by
  apply zmodAffineVerschiebung_sub_one_eq
  exact endomorphism_characteristic_of_generators
    _ (zmodAffineTrace W) p generators hgenerate hchar

/-- Two elements of an algebraic prime-field extension lie in one finite
intermediate field, so a common positive Frobenius power fixes both. -/
theorem zmodAlgebraicPair_frobeniusPeriodic
    {p : ℕ} [Fact p.Prime]
    {E : Type*} [Field E] [Algebra (ZMod p) E]
    [Algebra.IsAlgebraic (ZMod p) E] (x y : E) :
    ∃ n : ℕ, 0 < n ∧ x ^ (p ^ n) = x ∧ y ^ (p ^ n) = y := by
  let L : IntermediateField (ZMod p) E :=
    IntermediateField.adjoin (ZMod p) {x, y}
  let _ : FiniteDimensional (ZMod p) L :=
    IntermediateField.finiteDimensional_adjoin_pair
      (Algebra.IsIntegral.isIntegral (R := ZMod p) x)
      (Algebra.IsIntegral.isIntegral (R := ZMod p) y)
  let _ : Finite L := Module.finite_of_finite (ZMod p)
  let _ : Fintype L := Fintype.ofFinite L
  let xL : L := ⟨x, IntermediateField.mem_adjoin_pair_left (ZMod p) x y⟩
  let yL : L := ⟨y, IntermediateField.mem_adjoin_pair_right (ZMod p) x y⟩
  refine ⟨Module.finrank (ZMod p) L, Module.finrank_pos, ?_, ?_⟩
  all_goals
    have hcard : Fintype.card L = p ^ Module.finrank (ZMod p) L := by
      calc
        Fintype.card L = Fintype.card (ZMod p) ^ Module.finrank (ZMod p) L :=
          Module.card_eq_pow_finrank (K := ZMod p) (V := L)
        _ = p ^ Module.finrank (ZMod p) L := by rw [ZMod.card]
  · exact Subtype.ext_iff.mp (by
      simpa only [hcard] using (FiniteField.pow_card xL))
  · exact Subtype.ext_iff.mp (by
      simpa only [hcard] using (FiniteField.pow_card yL))

/-- The `n`th power of point Frobenius raises both affine coordinates to
`p^n`. The dependent nonsingularity witness is constructed internally. -/
theorem zmodAffineFrobeniusPointEnd_pow_some
    {p : ℕ} [Fact p.Prime]
    {E : Type*} [Field E] [DecidableEq E] [Algebra (ZMod p) E]
    (W : WeierstrassCurve.Affine (ZMod p)) {x y : E}
    (h : WeierstrassCurve.Affine.Nonsingular (W.baseChange E) x y)
    (n : ℕ) :
    ∃ hn : WeierstrassCurve.Affine.Nonsingular (W.baseChange E)
        (x ^ (p ^ n)) (y ^ (p ^ n)),
      ((affineFrobeniusPointEnd (E := E) W) ^ n)
          (WeierstrassCurve.Affine.Point.some x y h) =
        WeierstrassCurve.Affine.Point.some
          (x ^ (p ^ n)) (y ^ (p ^ n)) hn := by
  induction n with
  | zero =>
      refine ⟨?_, ?_⟩
      · simpa using h
      · simp
  | succ n ih =>
      obtain ⟨hn, ih⟩ := ih
      have hn' : WeierstrassCurve.Affine.Nonsingular (W.baseChange E)
          (x ^ (p ^ (n + 1))) (y ^ (p ^ (n + 1))) := by
        simpa only [FiniteField.coe_frobeniusAlgHom, ZMod.card, pow_succ,
          pow_mul] using
          ((W.baseChange_nonsingular
            (FiniteField.frobeniusAlgHom (ZMod p) E).injective ..).mpr hn)
      refine ⟨hn', ?_⟩
      rw [pow_succ']
      change affineFrobeniusPointEnd (E := E) W
        (((affineFrobeniusPointEnd (E := E) W) ^ n)
          (WeierstrassCurve.Affine.Point.some x y h)) = _
      rw [ih]
      simpa only [hn', pow_succ, pow_mul] using
        (zmodAffineFrobeniusPointEnd_some W hn)

/-- Every affine point over an algebraic prime-field extension is fixed by a
positive power of point Frobenius. Thus the finite Frobenius-fixed loci exhaust
the algebraic point group. -/
theorem zmodAffineFrobeniusPointEnd_periodic
    {p : ℕ} [Fact p.Prime]
    {E : Type*} [Field E] [DecidableEq E] [Algebra (ZMod p) E]
    [Algebra.IsAlgebraic (ZMod p) E]
    (W : WeierstrassCurve.Affine (ZMod p))
    (point : WeierstrassCurve.Affine.Point (W.baseChange E)) :
    ∃ n : ℕ, 0 < n ∧
      ((affineFrobeniusPointEnd (E := E) W) ^ n) point = point := by
  cases point with
  | zero =>
      refine ⟨1, by omega, ?_⟩
      change affineFrobeniusPointEnd (E := E) W 0 = 0
      rfl
  | some x y h =>
      obtain ⟨n, hn, hx, hy⟩ :=
        zmodAlgebraicPair_frobeniusPeriodic (p := p) x y
      obtain ⟨hpow, heq⟩ :=
        zmodAffineFrobeniusPointEnd_pow_some W h n
      refine ⟨n, hn, ?_⟩
      calc
        _ = WeierstrassCurve.Affine.Point.some
            (x ^ (p ^ n)) (y ^ (p ^ n)) hpow := heq
        _ = WeierstrassCurve.Affine.Point.some x y h := by
          simpa only [WeierstrassCurve.Affine.Point.some.injEq] using
            And.intro hx hy

/-- If every point is periodic under `pi`, then vanishing on every positive
fixed locus of `pi^n` proves that an additive endomorphism is globally zero. -/
theorem endomorphism_eq_zero_of_periodic_fixedLoci
    {G : Type*} [AddCommGroup G]
    (pi f : AddMonoid.End G)
    (hperiodic : ∀ point, ∃ n : ℕ, 0 < n ∧ (pi ^ n) point = point)
    (hzero : ∀ n : ℕ, 0 < n → ∀ point, (pi ^ n) point = point → f point = 0) :
    f = 0 := by
  ext point
  obtain ⟨n, hn, hfix⟩ := hperiodic point
  exact hzero n hn point hfix

/-- The Frobenius characteristic equation on every positive fixed locus
promotes to the global endomorphism equation when those loci exhaust points. -/
theorem endomorphism_characteristic_of_periodic_fixedLoci
    {G : Type*} [AddCommGroup G]
    (pi : AddMonoid.End G) (trace cardinality : ℤ)
    (hperiodic : ∀ point, ∃ n : ℕ, 0 < n ∧ (pi ^ n) point = point)
    (hchar : ∀ n : ℕ, 0 < n → ∀ point, (pi ^ n) point = point →
      (pi ^ 2 - (trace : AddMonoid.End G) * pi +
        (cardinality : AddMonoid.End G)) point = 0) :
    pi ^ 2 - (trace : AddMonoid.End G) * pi +
      (cardinality : AddMonoid.End G) = 0 := by
  exact endomorphism_eq_zero_of_periodic_fixedLoci
    pi _ hperiodic hchar

/-- Checking only one fixed locus cannot replace exhaustion by every positive
Frobenius period: negation on `ZMod 3` fixes only zero at period one, where the
identity mutation vanishes, but period two exposes the mutation at one. -/
theorem endomorphism_single_fixedLocus_control :
    let pi : AddMonoid.End (ZMod 3) := -1
    let f : AddMonoid.End (ZMod 3) := 1
    (∀ point, pi point = point → f point = 0) ∧
      f ≠ 0 ∧
      ∃ point, (pi ^ 2) point = point ∧ f point ≠ 0 := by
  dsimp
  constructor
  · intro point hfix
    change -point = point at hfix
    have : (2 : ZMod 3) * point = 0 := by linear_combination -hfix
    have htwo : (2 : ZMod 3) ≠ 0 := by decide
    exact (mul_eq_zero.mp this).resolve_left htwo
  constructor
  · intro h
    have h1 := DFunLike.congr_fun h (1 : ZMod 3)
    norm_num at h1
  · refine ⟨1, ?_, by norm_num⟩
    change -(-(1 : ZMod 3)) = 1
    simp

/-- Algebraic point exhaustion reduces the generic characteristic premise to
its restrictions on every positive Frobenius-fixed locus, and then yields the
exact pointwise `V-1` orientation. It does not prove the restricted
characteristic identities themselves. -/
theorem zmodAffineVerschiebung_sub_one_eq_of_fixedLoci
    {p : ℕ} [Fact p.Prime]
    {E : Type*} [Field E] [DecidableEq E] [Algebra (ZMod p) E]
    [Algebra.IsAlgebraic (ZMod p) E]
    (W : WeierstrassCurve.Affine (ZMod p))
    (hchar : ∀ n : ℕ, 0 < n →
      ∀ point : WeierstrassCurve.Affine.Point (W.baseChange E),
        ((affineFrobeniusPointEnd (E := E) W) ^ n) point = point →
        (affineFrobeniusPointEnd (E := E) W ^ 2 -
          (zmodAffineTrace W : AddMonoid.End
            (WeierstrassCurve.Affine.Point (W.baseChange E))) *
              affineFrobeniusPointEnd (E := E) W +
          (p : AddMonoid.End
            (WeierstrassCurve.Affine.Point (W.baseChange E)))) point = 0) :
    zmodAffineVerschiebungPointEnd W - 1 =
      ((zmodAffineTrace W - 1 : ℤ) : AddMonoid.End
        (WeierstrassCurve.Affine.Point (W.baseChange E))) -
          affineFrobeniusPointEnd (E := E) W := by
  apply zmodAffineVerschiebung_sub_one_eq
  exact endomorphism_characteristic_of_periodic_fixedLoci
    _ (zmodAffineTrace W) p
    (zmodAffineFrobeniusPointEnd_periodic W) hchar

/-- Every one-dimensional formal group law differs from its first input by a
multiple of its second variable. This is the algebraic filtration statement
behind the H116 Frobenius cutoff; it uses Mathlib's actual `FormalGroup`, not a
locally copied group-law interface. -/
theorem FormalGroup.secondVariable_dvd_sub_X
    {R : Type*} [CommRing R] (F : FormalGroup R) :
    (MvPowerSeries.X 1 : MvPowerSeries (Fin 2) R) ∣
      F.toPowerSeries - MvPowerSeries.X 0 := by
  rw [MvPowerSeries.X_dvd_iff]
  intro m hm
  have h := congrArg (PowerSeries.coeff (m 0)) F.Xzero_eq_X
  change MvPowerSeries.coeff (Finsupp.single () (m 0))
      (MvPowerSeries.subst ![MvPowerSeries.X (), 0] F.toPowerSeries) =
    MvPowerSeries.coeff (Finsupp.single () (m 0)) (MvPowerSeries.X ()) at h
  rw [MvPowerSeries.coeff_subst, finsum_eq_single _ m] at h
  · have heq : MvPowerSeries.coeff m F.toPowerSeries =
        MvPowerSeries.coeff m (MvPowerSeries.X 0) := by
      simp [hm] at h
      change MvPowerSeries.coeff m F.toPowerSeries *
          PowerSeries.coeff (m 0) (PowerSeries.X ^ m 0) =
        PowerSeries.coeff (m 0) PowerSeries.X at h
      rw [PowerSeries.coeff_X_pow] at h
      simp at h
      rw [h]
      change MvPowerSeries.coeff (Finsupp.single () (m 0))
          (MvPowerSeries.X ()) =
        MvPowerSeries.coeff m (MvPowerSeries.X 0)
      rw [MvPowerSeries.coeff_X, MvPowerSeries.coeff_X]
      by_cases hm0 : m 0 = 1
      · have hms : m = Finsupp.single 0 1 := by
          ext i
          fin_cases i
          · simpa using hm0
          · simpa using hm
        simp [hms]
      · have hms : m ≠ Finsupp.single 0 1 := by
          intro he
          apply hm0
          rw [he]
          simp
        simp [hm0, hms]
    simp [heq]
  · intro d hd
    by_cases hd1 : d 1 = 0
    · have hd0 : d 0 ≠ m 0 := by
        intro hdm
        apply hd
        ext i
        fin_cases i
        · exact hdm
        · exact hd1.trans hm.symm
      simp [hd1]
      change MvPowerSeries.coeff d F.toPowerSeries *
        PowerSeries.coeff (m 0) (PowerSeries.X ^ d 0) = 0
      rw [PowerSeries.coeff_X_pow]
      simp [Ne.symm hd0]
    · simp [hd1]
  · exact MvPowerSeries.HasSubst.X_zero

/-- Substituting two zero-constant parameters into a formal group makes the
second input an explicit multiplicative correction to the first. -/
theorem FormalGroup.add_correction_eq
    {R : Type*} [CommRing R] (F : FormalGroup R)
    {f correction : PowerSeries R}
    (hf : PowerSeries.constantCoeff f = 0)
    (hc : PowerSeries.constantCoeff correction = 0) :
    ∃ q : PowerSeries R,
      F.toPowerSeries.subst ![f, correction] = f + correction * q := by
  obtain ⟨q, hq⟩ := FormalGroup.secondVariable_dvd_sub_X F
  let a : Fin 2 → PowerSeries R := ![f, correction]
  have ha : MvPowerSeries.HasSubst a :=
    MvPowerSeries.hasSubst_of_constantCoeff_nilpotent fun i => by
      fin_cases i
      · change IsNilpotent (MvPowerSeries.constantCoeff f)
        rw [show MvPowerSeries.constantCoeff f = 0 by
          simpa [PowerSeries.constantCoeff] using hf]
        exact IsNilpotent.zero
      · change IsNilpotent (MvPowerSeries.constantCoeff correction)
        rw [show MvPowerSeries.constantCoeff correction = 0 by
          simpa [PowerSeries.constantCoeff] using hc]
        exact IsNilpotent.zero
  refine ⟨MvPowerSeries.subst a q, ?_⟩
  have hsub := congrArg (MvPowerSeries.subst a) hq
  rw [MvPowerSeries.subst_sub ha, MvPowerSeries.subst_X ha,
    MvPowerSeries.subst_mul ha, MvPowerSeries.subst_X ha] at hsub
  simpa [a, add_comm] using (sub_eq_iff_eq_add.mp hsub)

/-- A formal-group correction divisible by `X^m` cannot change coefficients
strictly below degree `m`. -/
theorem FormalGroup.filtered_correction_prefix
    {R : Type*} [Field R] (F : FormalGroup R)
    {f correction tail : PowerSeries R} {m : ℕ}
    (hf : PowerSeries.constantCoeff f = 0)
    (hc : PowerSeries.constantCoeff correction = 0)
    (horder : correction = PowerSeries.X ^ m * tail) :
    PowerSeriesPrefixEq m
      (F.toPowerSeries.subst ![f, correction]) f := by
  obtain ⟨q, hq⟩ := FormalGroup.add_correction_eq F hf hc
  rw [hq, horder]
  simpa only [mul_assoc] using
    PowerSeriesPrefixEq.add_X_pow_mul f (tail * q) m

/-- Composing the filtered formal-group correction with the H116 pole bridge
loses exactly the shared leading `X`: an order `n+1` parameter correction gives
an order `n` normalized double-pole/regular-unit prefix. -/
theorem FormalGroup.filtered_pole_prefix
    {R : Type*} [Field R] (F : FormalGroup R)
    {outer scalarParameter correction correctionTail
      actualUnit scalarUnit : PowerSeries R} {n : ℕ}
    (hscalarZero : PowerSeries.constantCoeff scalarParameter = 0)
    (hcorrectionZero : PowerSeries.constantCoeff correction = 0)
    (hcorrectionOrder : correction =
      PowerSeries.X ^ (n + 1) * correctionTail)
    (hactual : F.toPowerSeries.subst ![scalarParameter, correction] =
      PowerSeries.X * actualUnit)
    (hscalar : scalarParameter = PowerSeries.X * scalarUnit)
    (hactualUnit : PowerSeries.constantCoeff actualUnit ≠ 0)
    (hscalarUnit : PowerSeries.constantCoeff scalarUnit ≠ 0)
    (houter : PowerSeries.constantCoeff outer ≠ 0) :
    PowerSeriesPrefixEq n
      (actualUnit⁻¹ ^ 2 *
        (outer.subst (PowerSeries.X * actualUnit))⁻¹)
      (scalarUnit⁻¹ ^ 2 *
        (outer.subst (PowerSeries.X * scalarUnit))⁻¹) := by
  have hparameter := FormalGroup.filtered_correction_prefix F
    hscalarZero hcorrectionZero hcorrectionOrder
  rw [hactual, hscalar] at hparameter
  exact PowerSeriesPrefixEq.leading_X_inv_sq_mul_subst_inv
    outer hparameter hactualUnit hscalarUnit houter

/-- Associativity cancels a certified omitted parameter against a certified
right inverse. This is the orientation needed to turn
`F(actual, omitted) = scalar` into `actual = F(scalar, inverseCorrection)`
without assuming an inverse-series API or copying a group law. -/
theorem FormalGroup.right_cancel_with_inverse
    {R : Type*} [CommRing R] (F : FormalGroup R)
    {actual omitted scalar inverseCorrection : PowerSeries R}
    (hactualZero : PowerSeries.constantCoeff actual = 0)
    (homittedZero : PowerSeries.constantCoeff omitted = 0)
    (hinverseZero : PowerSeries.constantCoeff inverseCorrection = 0)
    (hsum : F.toPowerSeries.subst ![actual, omitted] = scalar)
    (hinverse : F.toPowerSeries.subst ![omitted, inverseCorrection] = 0) :
    F.toPowerSeries.subst ![scalar, inverseCorrection] = actual := by
  have hactualSubst : PowerSeries.HasSubst actual :=
    PowerSeries.HasSubst.of_constantCoeff_zero' hactualZero
  have homittedSubst : PowerSeries.HasSubst omitted :=
    PowerSeries.HasSubst.of_constantCoeff_zero' homittedZero
  have hinverseSubst : PowerSeries.HasSubst inverseCorrection :=
    PowerSeries.HasSubst.of_constantCoeff_zero' hinverseZero
  calc
    F.toPowerSeries.subst ![scalar, inverseCorrection] =
        F.toPowerSeries.subst ![
          F.toPowerSeries.subst ![actual, omitted], inverseCorrection] := by
      rw [hsum]
    _ = F.toPowerSeries.subst ![actual,
          F.toPowerSeries.subst ![omitted, inverseCorrection]] :=
      F.assoc' hactualSubst homittedSubst hinverseSubst
    _ = F.toPowerSeries.subst ![actual, 0] := by rw [hinverse]
    _ = actual := F.add_zero hactualSubst

/-- The endomorphism-oriented H116 endpoint. A certified relation
`F(actual, omitted) = scalar`, a certified right inverse of the omitted
parameter, and an order-`n+1` factorization of that inverse imply the same
order-`n` normalized pole prefix as a directly supplied correction. -/
theorem FormalGroup.filtered_right_cancellation_pole_prefix
    {R : Type*} [Field R] (F : FormalGroup R)
    {outer actualParameter omittedParameter scalarParameter inverseCorrection
      inverseTail actualUnit scalarUnit : PowerSeries R} {n : ℕ}
    (hactualZero : PowerSeries.constantCoeff actualParameter = 0)
    (homittedZero : PowerSeries.constantCoeff omittedParameter = 0)
    (hscalarZero : PowerSeries.constantCoeff scalarParameter = 0)
    (hinverseZero : PowerSeries.constantCoeff inverseCorrection = 0)
    (hsum : F.toPowerSeries.subst ![actualParameter, omittedParameter] =
      scalarParameter)
    (hinverse : F.toPowerSeries.subst ![omittedParameter, inverseCorrection] = 0)
    (hinverseOrder : inverseCorrection =
      PowerSeries.X ^ (n + 1) * inverseTail)
    (hactual : actualParameter = PowerSeries.X * actualUnit)
    (hscalar : scalarParameter = PowerSeries.X * scalarUnit)
    (hactualUnit : PowerSeries.constantCoeff actualUnit ≠ 0)
    (hscalarUnit : PowerSeries.constantCoeff scalarUnit ≠ 0)
    (houter : PowerSeries.constantCoeff outer ≠ 0) :
    PowerSeriesPrefixEq n
      (actualUnit⁻¹ ^ 2 *
        (outer.subst (PowerSeries.X * actualUnit))⁻¹)
      (scalarUnit⁻¹ ^ 2 *
        (outer.subst (PowerSeries.X * scalarUnit))⁻¹) := by
  have hcancel :
      F.toPowerSeries.subst ![scalarParameter, inverseCorrection] =
        actualParameter :=
    FormalGroup.right_cancel_with_inverse F hactualZero homittedZero
      hinverseZero hsum hinverse
  apply FormalGroup.filtered_pole_prefix F hscalarZero hinverseZero
    hinverseOrder
  · rw [hcancel, hactual]
  · exact hscalar
  · exact hactualUnit
  · exact hscalarUnit
  · exact houter

/-- In a commutative formal group, any certified zero-constant right inverse
has at least the order of the omitted parameter. The factorization is derived
from the group law; the inverse order is not an additional hypothesis. -/
theorem FormalGroup.right_inverse_order
    {R : Type*} [CommRing R] (F : FormalGroup R) [F.IsComm]
    {omitted omittedTail inverseCorrection : PowerSeries R} {m : ℕ}
    (homittedZero : PowerSeries.constantCoeff omitted = 0)
    (hinverseZero : PowerSeries.constantCoeff inverseCorrection = 0)
    (hinverse : F.toPowerSeries.subst ![omitted, inverseCorrection] = 0)
    (homittedOrder : omitted = PowerSeries.X ^ m * omittedTail) :
    ∃ inverseTail : PowerSeries R,
      inverseCorrection = PowerSeries.X ^ m * inverseTail := by
  have homittedSubst : PowerSeries.HasSubst omitted :=
    PowerSeries.HasSubst.of_constantCoeff_zero' homittedZero
  have hinverseSubst : PowerSeries.HasSubst inverseCorrection :=
    PowerSeries.HasSubst.of_constantCoeff_zero' hinverseZero
  have hinverseLeft :
      F.toPowerSeries.subst ![inverseCorrection, omitted] = 0 := by
    rw [F.comm' hinverseSubst homittedSubst]
    exact hinverse
  obtain ⟨q, hq⟩ :=
    FormalGroup.add_correction_eq F hinverseZero homittedZero
  have hzero : inverseCorrection + omitted * q = 0 := by
    rw [← hq, hinverseLeft]
  refine ⟨-(omittedTail * q), ?_⟩
  calc
    inverseCorrection = -(omitted * q) :=
      eq_neg_of_add_eq_zero_left hzero
    _ = -(PowerSeries.X ^ m * omittedTail * q) := by rw [homittedOrder]
    _ = PowerSeries.X ^ m * (-(omittedTail * q)) := by ring

/-- The commutative endpoint needs the order of the omitted Frobenius
parameter, not a separate order assumption for its certified inverse. -/
theorem FormalGroup.filtered_right_cancellation_pole_prefix_of_omitted_order
    {R : Type*} [Field R] (F : FormalGroup R) [F.IsComm]
    {outer actualParameter omittedParameter omittedTail scalarParameter
      inverseCorrection actualUnit scalarUnit : PowerSeries R} {n : ℕ}
    (hactualZero : PowerSeries.constantCoeff actualParameter = 0)
    (homittedZero : PowerSeries.constantCoeff omittedParameter = 0)
    (hscalarZero : PowerSeries.constantCoeff scalarParameter = 0)
    (hinverseZero : PowerSeries.constantCoeff inverseCorrection = 0)
    (hsum : F.toPowerSeries.subst ![actualParameter, omittedParameter] =
      scalarParameter)
    (hinverse : F.toPowerSeries.subst ![omittedParameter, inverseCorrection] = 0)
    (homittedOrder : omittedParameter =
      PowerSeries.X ^ (n + 1) * omittedTail)
    (hactual : actualParameter = PowerSeries.X * actualUnit)
    (hscalar : scalarParameter = PowerSeries.X * scalarUnit)
    (hactualUnit : PowerSeries.constantCoeff actualUnit ≠ 0)
    (hscalarUnit : PowerSeries.constantCoeff scalarUnit ≠ 0)
    (houter : PowerSeries.constantCoeff outer ≠ 0) :
    PowerSeriesPrefixEq n
      (actualUnit⁻¹ ^ 2 *
        (outer.subst (PowerSeries.X * actualUnit))⁻¹)
      (scalarUnit⁻¹ ^ 2 *
        (outer.subst (PowerSeries.X * scalarUnit))⁻¹) := by
  obtain ⟨inverseTail, hinverseOrder⟩ :=
    FormalGroup.right_inverse_order F homittedZero hinverseZero hinverse
      homittedOrder
  exact FormalGroup.filtered_right_cancellation_pole_prefix F
    hactualZero homittedZero hscalarZero hinverseZero hsum hinverse
    hinverseOrder hactual hscalar hactualUnit hscalarUnit houter

/-- Short-Weierstrass negation can be exposed as the exact certificate
`F(omitted, -omitted) = 0`. With that certificate, the endpoint uses the
explicit correction `-omitted` and needs neither a separately constructed
inverse series nor a separate inverse-order premise. -/
theorem FormalGroup.filtered_right_negation_pole_prefix
    {R : Type*} [Field R] (F : FormalGroup R)
    {outer actualParameter omittedParameter omittedTail scalarParameter
      actualUnit scalarUnit : PowerSeries R} {n : ℕ}
    (hactualZero : PowerSeries.constantCoeff actualParameter = 0)
    (homittedZero : PowerSeries.constantCoeff omittedParameter = 0)
    (hscalarZero : PowerSeries.constantCoeff scalarParameter = 0)
    (hsum : F.toPowerSeries.subst ![actualParameter, omittedParameter] =
      scalarParameter)
    (hneg : F.toPowerSeries.subst ![omittedParameter, -omittedParameter] = 0)
    (homittedOrder : omittedParameter =
      PowerSeries.X ^ (n + 1) * omittedTail)
    (hactual : actualParameter = PowerSeries.X * actualUnit)
    (hscalar : scalarParameter = PowerSeries.X * scalarUnit)
    (hactualUnit : PowerSeries.constantCoeff actualUnit ≠ 0)
    (hscalarUnit : PowerSeries.constantCoeff scalarUnit ≠ 0)
    (houter : PowerSeries.constantCoeff outer ≠ 0) :
    PowerSeriesPrefixEq n
      (actualUnit⁻¹ ^ 2 *
        (outer.subst (PowerSeries.X * actualUnit))⁻¹)
      (scalarUnit⁻¹ ^ 2 *
        (outer.subst (PowerSeries.X * scalarUnit))⁻¹) := by
  have hnegZero : PowerSeries.constantCoeff (-omittedParameter) = 0 := by
    simp [homittedZero]
  have hnegOrder : -omittedParameter =
      PowerSeries.X ^ (n + 1) * (-omittedTail) := by
    rw [homittedOrder]
    ring
  exact FormalGroup.filtered_right_cancellation_pole_prefix F
    hactualZero homittedZero hscalarZero hnegZero hsum hneg hnegOrder
    hactual hscalar hactualUnit hscalarUnit houter

/-- The standard short-Weierstrass local parameter in affine coordinates. -/
def shortWeierstrassLocalParameter (x y : K) : K :=
  -x / y

/-- Short-Weierstrass point negation `(x,y) ↦ (x,-y)` negates the local
parameter exactly. -/
theorem shortWeierstrassLocalParameter_neg (x y : K) :
    shortWeierstrassLocalParameter x (-y) =
      -shortWeierstrassLocalParameter x y := by
  unfold shortWeierstrassLocalParameter
  rw [div_neg]

omit [Algebra K L] in
/-- The short-Weierstrass local parameter commutes with every field ring
homomorphism applied coordinatewise. -/
theorem shortWeierstrassLocalParameter_map (f : K →+* L) (x y : K) :
    shortWeierstrassLocalParameter (f x) (f y) =
      f (shortWeierstrassLocalParameter x y) := by
  simp [shortWeierstrassLocalParameter]

/-- If a coordinate endomorphism is the `p`-power map, its action on the
short-Weierstrass local parameter is exactly `z ↦ z^p`. -/
theorem shortWeierstrassLocalParameter_frobenius
    (phi : K →+* K) (p : ℕ) (hphi : ∀ a : K, phi a = a ^ p) (x y : K) :
    shortWeierstrassLocalParameter (phi x) (phi y) =
      shortWeierstrassLocalParameter x y ^ p := by
  rw [shortWeierstrassLocalParameter_map, hphi]

/-- A total version of the short-Weierstrass affine local parameter on
Mathlib's point type: infinity maps to zero and `(x,y)` maps to `-x/y`. -/
def affinePointLocalParameter
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K) :
    W.Point → K
  | .zero => 0
  | .some x y _ => shortWeierstrassLocalParameter x y

@[simp] theorem affinePointLocalParameter_some
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K)
    {x y : K} (h : W.Nonsingular x y) :
    affinePointLocalParameter W
      (WeierstrassCurve.Affine.Point.some x y h) =
        shortWeierstrassLocalParameter x y := by
  rfl

/-- Evaluate the pointwise `V-1=[trace-1]-pi` identity through the selected
affine local parameter. This closes the orientation and point-observation
step while deliberately leaving rational-map and formal-series transport as a
separate semantic obligation. -/
theorem zmodAffineVerschiebung_sub_one_localParameter
    {p : ℕ} [Fact p.Prime]
    {E : Type*} [Field E] [DecidableEq E] [Algebra (ZMod p) E]
    [Algebra.IsAlgebraic (ZMod p) E]
    (W : WeierstrassCurve.Affine (ZMod p)) (trace : ℤ)
    (hchar : affineFrobeniusPointEnd (E := E) W ^ 2 -
        (trace : AddMonoid.End
          (WeierstrassCurve.Affine.Point (W.baseChange E))) *
            affineFrobeniusPointEnd (E := E) W +
        ((p : ℤ) : AddMonoid.End
          (WeierstrassCurve.Affine.Point (W.baseChange E))) = 0)
    (point : WeierstrassCurve.Affine.Point (W.baseChange E)) :
    affinePointLocalParameter (W.baseChange E)
        (zmodAffineVerschiebungPointEnd W point - point) =
      affinePointLocalParameter (W.baseChange E)
        ((trace - 1) • point - affineFrobeniusPointEnd (E := E) W point) := by
  have horient := DFunLike.congr_fun
    (zmodAffineVerschiebung_sub_one_eq W trace hchar) point
  exact congrArg (affinePointLocalParameter (W.baseChange E)) horient

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

/-- A finite set of regular evaluations separates two supplied rational maps
once the cardinality of the set is strictly larger than both cross-product
degrees.  This is a certificate interface: callers must still supply the
numerators, denominators, denominator nonvanishing, and degree bound. -/
theorem rationalMap_eq_of_eval_finset
    {F : Type*} [Field F]
    (num₁ den₁ num₂ den₂ : Polynomial F) (samples : Finset F)
    (hden : ∀ x ∈ samples, den₁.eval x ≠ 0 ∧ den₂.eval x ≠ 0)
    (heval : ∀ x ∈ samples,
      num₁.eval x / den₁.eval x = num₂.eval x / den₂.eval x)
    (hdegree : max (num₁ * den₂).natDegree (num₂ * den₁).natDegree < samples.card) :
    num₁ * den₂ = num₂ * den₁ := by
  apply Polynomial.eq_of_natDegree_lt_card_of_eval_eq' _ _ samples
  · intro x hx
    rw [Polynomial.eval_mul, Polynomial.eval_mul]
    exact (div_eq_div_iff (hden x hx).1 (hden x hx).2).mp (heval x hx)
  · exact hdegree

/-- The strict degree bound in `rationalMap_eq_of_eval_finset` cannot in
general be relaxed to a non-strict one: `X^3-X` is a nonzero polynomial of
degree `#(ZMod 3)` that vanishes on the whole field. -/
theorem rationalMap_separation_strict_control :
    let f : Polynomial (ZMod 3) := Polynomial.X ^ 3 - Polynomial.X
    (∀ x : ZMod 3, f.eval x = 0) ∧
      f ≠ 0 ∧ f.natDegree = Fintype.card (ZMod 3) := by
  dsimp
  constructor
  · intro x
    simp [ZMod.pow_card]
  constructor
  · intro h
    have := congrArg (fun f : Polynomial (ZMod 3) => f.coeff 1) h
    norm_num at this
  · norm_num [Polynomial.natDegree_sub_eq_left_of_natDegree_lt]

/-- Translating an `n`-torsion point by one supplied division point remains
in the same multiplication fiber. -/
theorem divisionFiber_of_torsionTranslate
    {G : Type*} [AddCommGroup G] (n : ℕ) {Q R T : G}
    (hR : n • R = Q) (hT : n • T = 0) :
    n • (R + T) = Q := by
  rw [nsmul_add, hR, hT, add_zero]

/-- The difference of two points in one multiplication fiber is torsion. -/
theorem divisionFiber_difference_is_torsion
    {G : Type*} [AddCommGroup G] (n : ℕ) {Q R S : G}
    (hR : n • R = Q) (hS : n • S = Q) :
    n • (S - R) = 0 := by
  rw [nsmul_sub, hS, hR, sub_self]

/-- Translation by a Frobenius-fixed point leaves the additive connecting
cocycle `sigma(S)-S` unchanged. -/
theorem frobeniusCocycle_fixedTranslate
    {G : Type*} [AddCommGroup G] (sigma : G →+ G) {R : G}
    (hR : sigma R = R) (T : G) :
    sigma (R + T) - (R + T) = sigma T - T := by
  rw [map_add, hR]
  abel

/-- The complete cocycle multiset on a translated torsion multiset is
independent of the fixed translate, including all multiplicities. -/
theorem frobeniusCocycle_multiset_translate
    {G : Type*} [AddCommGroup G] (sigma : G →+ G)
    (torsion : Multiset G) {R : G} (hR : sigma R = R) :
    torsion.map (fun T => sigma (R + T) - (R + T)) =
      torsion.map (fun T => sigma T - T) := by
  have hfun : (fun T : G => sigma (R + T) - (R + T)) =
      (fun T : G => sigma T - T) := by
    funext T
    exact frobeniusCocycle_fixedTranslate sigma hR T
  rw [hfun]

/-- Selecting the fixed rational division point itself makes its Frobenius
cocycle identically zero. -/
theorem fixedPoint_frobeniusCocycle_zero
    {G : Type*} [AddCommGroup G] (sigma : G →+ G) {R : G}
    (hR : sigma R = R) : sigma R - R = 0 := by
  exact sub_eq_zero.mpr hR

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
#print axioms fiberDenominatorReciprocal_coeff_zero_one
#print axioms fiberLogDerivative_coeff_one
#print axioms fiberRelativeLogDerivative_coeff_one
#print axioms fiberRelativeLogDerivative_coeff_one_unshared
#print axioms quotientDigit_nsmul_identity
#print axioms quotientDigit_sub_identity
#print axioms quotientDigit_strict_decrease
#print axioms odd_sub_mod_two_ne
#print axioms orbit_sub_eq_neg
#print axioms negInvariant_binaryDecoder_pair_failure
#print axioms evenOrder_parityMutation_control
#print axioms signedTwist_traceParity_invariant
#print axioms signedTwist_traceParity_negInvariant
#print axioms signedTwist_traceParity_decoder_pair_failure
#print axioms oddModulus_signedTwistMutation_control
#print axioms ellipticPrym_lowCoefficient_convolution
#print axioms prymCoefficients_of_productLowCoefficients
#print axioms prymRealWeil_discriminant_normalized
#print axioms prymWeil_homogenized_realWeil
#print axioms prymWeil_eq_sq_mul_realWeil
#print axioms chiralBranchResidual_discriminant_closed
#print axioms chiralBranchResidual_negationSwap
#print axioms linearQuadratic_discriminant_factorization
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
#print axioms frobeniusVerschiebung_sum_of_characteristic
#print axioms verschiebung_sub_one_mul_frobenius_sub_one
#print axioms degree_eq_norm_of_paired_factor
#print axioms verschiebung_sub_one_degree_of_dual_certificate
#print axioms degree_product_without_duality_control
#print axioms finiteFieldTower_degree_mul
#print axioms finiteFieldTower_degree_eq_norm_of_paired
#print axioms quadraticFunctionFieldSandwich_degree
#print axioms rationalFunctionCoordinateExtension_degree
#print axioms reducedRationalQuotient_degrees
#print axioms reducedRationalQuotient_extension_degree
#print axioms shortWeierstrassScalarTwo_explicit_isCoprime
#print axioms shortWeierstrassScalarTwo_isCoprime
#print axioms shortWeierstrassScalarTwo_extension_degree
#print axioms unequalCoordinateLineDegrees_control
#print axioms scalarMultiplicationXMap_exact_degrees
#print axioms scalarMultiplicationXMap_leadingCoefficients
#print axioms scalarMultiplicationXMap_denominator_degree_lt_of_char_dvd
#print axioms scalarMultiplicationXMap_characteristic_control
#print axioms shortWeierstrassScalarThree_explicit_isCoprime
#print axioms shortWeierstrassScalarThree_isCoprime
#print axioms shortWeierstrassScalarThree_extension_degree
#print axioms shortWeierstrassScalarThree_singular_control
#print axioms subMultiple_isCoprime_of_factors
#print axioms divisionPolynomialPhi_isCoprime_of_factors
#print axioms divisionPolynomialCoprimality_missingFactor_control
#print axioms preNormEDS'_odd_isCoprime
#print axioms shortWeierstrassPsiThree_PsiTwoSq_explicit_isCoprime
#print axioms shortWeierstrassPsiThree_PsiTwoSq_isCoprime
#print axioms shortWeierstrass_prePsi_odd_isCoprime_PsiTwoSq
#print axioms shortWeierstrass_parityFactor_isCoprime
#print axioms shortWeierstrassPhi_isCoprime_of_adjacent
#print axioms shortWeierstrassPsiThree_PsiTwoSq_singular_control
#print axioms shortWeierstrass_prePsiThreeFour_isCoprime
#print axioms shortWeierstrass_prePsiFourFive_isCoprime
#print axioms shortWeierstrassScalarFour_isCoprime
#print axioms shortWeierstrassScalarFour_extension_degree
#print axioms shortWeierstrassScalarFour_singular_control
#print axioms shortWeierstrass_prePsiThreeFive_isCoprime
#print axioms shortWeierstrass_prePsiFiveSix_isCoprime
#print axioms shortWeierstrassScalarFive_isCoprime
#print axioms shortWeierstrassScalarFive_extension_degree
#print axioms shortWeierstrassScalarFive_singular_control
#print axioms edsTwoFactorBootstrap_isCoprime
#print axioms edsTwoFactorBootstrap_missingAS_control
#print axioms edsTwoFactorSuccessor_isCoprime
#print axioms edsTwoFactorSecondSuccessor_isCoprime
#print axioms edsTwoFactorThirdSuccessor_isCoprime
#print axioms edsTwoFactorFourthSuccessor_isCoprime
#print axioms shortWeierstrass_prePsiSixSeven_isCoprime
#print axioms shortWeierstrassScalarSix_isCoprime
#print axioms shortWeierstrassScalarSix_extension_degree
#print axioms shortWeierstrassScalarSix_singular_control
#print axioms shortWeierstrass_prePsiSevenEight_isCoprime
#print axioms shortWeierstrassScalarSeven_isCoprime
#print axioms shortWeierstrassScalarSeven_extension_degree
#print axioms shortWeierstrassScalarSeven_singular_control
#print axioms shortWeierstrass_prePsiEightNine_isCoprime
#print axioms shortWeierstrassScalarEight_isCoprime
#print axioms shortWeierstrassScalarEight_extension_degree
#print axioms shortWeierstrassScalarEight_singular_control
#print axioms shortWeierstrass_prePsiNineTen_isCoprime
#print axioms shortWeierstrassScalarNine_isCoprime
#print axioms shortWeierstrassScalarNine_extension_degree
#print axioms shortWeierstrassScalarNine_singular_control
#print axioms shortWeierstrass_prePsiTenEleven_isCoprime
#print axioms shortWeierstrassScalarTen_isCoprime
#print axioms shortWeierstrassScalarTen_extension_degree
#print axioms shortWeierstrassScalarTen_singular_control
#print axioms shortWeierstrassScalar_isCoprime_of_resultantFormula
#print axioms shortWeierstrassScalar_extension_degree_of_resultantFormula
#print axioms polynomial_isCoprime_of_no_common_root
#print axioms adjacentIntegerTorsion_eq_zero
#print axioms polynomial_isCoprime_of_adjacent_torsion_root_semantics
#print axioms polynomial_isCoprime_of_no_common_root_after_map
#print axioms polynomial_isCoprime_of_adjacent_torsion_root_semantics_after_map
#print axioms weierstrass_exists_equation_point_above
#print axioms affineWeierstrass_exists_nonzero_point_above
#print axioms polynomial_isCoprime_of_adjacent_torsion_curve_semantics_after_map
#print axioms finite_field_point_lift_control
#print axioms adjacentIntegerTorsion_zero_control
#print axioms endomorphism_frobeniusVerschiebung_sum
#print axioms endomorphism_injectivity_control
#print axioms affineFrobeniusPointEnd_injective
#print axioms affineFrobeniusPointEnd_some
#print axioms affinePoint_frobeniusVerschiebung_sum
#print axioms zmodAffineFrobeniusPointEnd_some
#print axioms zmodAffinePoint_frobeniusVerschiebung_sum
#print axioms zmodAffineFrobeniusPointEnd_mul_inv
#print axioms zmodAffineFrobenius_mul_verschiebung
#print axioms zmodAffineVerschiebung_sum_of_characteristic
#print axioms endomorphism_sub_one_eq_trace_sub_one_sub
#print axioms endomorphism_frobenius_eigen_on_verschiebung_fixed
#print axioms endomorphism_iterate_eigen
#print axioms endomorphism_frobenius_fixedLocus_iff_scalar_on_verschiebung_fixed
#print axioms zmodAffineVerschiebung_sub_one_eq
#print axioms zmodAffineVerschiebung_sub_one_localParameter
#print axioms endomorphism_eq_zero_of_eq_zero_on_generators
#print axioms endomorphism_characteristic_of_generators
#print axioms zmodAffineVerschiebung_sub_one_eq_of_generators
#print axioms zmodAlgebraicPair_frobeniusPeriodic
#print axioms zmodAffineFrobeniusPointEnd_pow_some
#print axioms zmodAffineFrobeniusPointEnd_periodic
#print axioms endomorphism_eq_zero_of_periodic_fixedLoci
#print axioms endomorphism_characteristic_of_periodic_fixedLoci
#print axioms endomorphism_single_fixedLocus_control
#print axioms zmodAffineVerschiebung_sub_one_eq_of_fixedLoci
#print axioms FormalGroup.secondVariable_dvd_sub_X
#print axioms FormalGroup.add_correction_eq
#print axioms FormalGroup.filtered_correction_prefix
#print axioms FormalGroup.filtered_pole_prefix
#print axioms FormalGroup.right_cancel_with_inverse
#print axioms FormalGroup.filtered_right_cancellation_pole_prefix
#print axioms FormalGroup.right_inverse_order
#print axioms FormalGroup.filtered_right_cancellation_pole_prefix_of_omitted_order
#print axioms FormalGroup.filtered_right_negation_pole_prefix
#print axioms shortWeierstrassLocalParameter_neg
#print axioms shortWeierstrassLocalParameter_map
#print axioms shortWeierstrassLocalParameter_frobenius
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
#print axioms rationalMap_eq_of_eval_finset
#print axioms rationalMap_separation_strict_control
#print axioms divisionFiber_of_torsionTranslate
#print axioms divisionFiber_difference_is_torsion
#print axioms frobeniusCocycle_fixedTranslate
#print axioms frobeniusCocycle_multiset_translate
#print axioms fixedPoint_frobeniusCocycle_zero

end H116FiberTrace
