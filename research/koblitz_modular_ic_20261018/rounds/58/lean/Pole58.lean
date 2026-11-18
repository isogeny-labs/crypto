import Mathlib.RingTheory.Valuation.Basic
import Mathlib.Algebra.CharP.Defs
import Mathlib.Tactic.Abel
import Mathlib.Tactic.IntervalCases
import Mathlib.Tactic.NormNum
import Mathlib.Tactic.Linarith
import Mathlib.Tactic.LinearCombination

namespace Pole58

/-- The exact local equation x*unit=s² transfers its order to x.
The actual local parameter and unit premises come from the curve geometry. -/
theorem local_unit_double_zero {K Γ : Type*} [Field K]
    [LinearOrderedAddCommGroupWithTop Γ] (v : AddValuation K Γ)
    (x s u : K) (heq : x * u = s ^ 2) (hu : v u = 0) :
    v x = 2 • v s := by
  have h := congrArg v heq
  simpa only [v.map_mul, v.map_pow, hu, add_zero] using h

/-- A pulled-back double pole at positive ramification cannot meet a
simple-pole bound. Surjectivity and valuation compatibility are premises. -/
theorem no_simple_pole (e sourceOrder : ℤ) (he : 0 < e)
    (hpull : sourceOrder = -(2 * e)) (hbound : -1 ≤ sourceOrder) : False := by
  have he1 : 1 ≤ e := Int.add_one_le_iff.mpr he
  rw [hpull] at hbound
  linarith

/-- Imaginary coefficient extraction in a characteristic-two quadratic
constant extension; x,y are the binary coefficients. -/
theorem imaginary_coefficient {K : Type*} [Field K] [CharP K 2]
    (a x y : K) (ha : a ^ 2 + a + 1 = 0)
    (hx : x ^ 2 = x) (hy : y ^ 2 = y) :
    (x + a * y) ^ 2 + (x + a * y) = y := by
  have htwo : (2 : K) = 0 := CharP.cast_eq_zero K 2
  linear_combination hx + a^2 * hy + y * ha + (a*x*y+x-y) * htwo

/-- The real coefficient is recovered after the imaginary coefficient. -/
theorem real_coefficient {K : Type*} [Field K] [CharP K 2]
    (a x y : K) (ha : a ^ 2 + a + 1 = 0)
    (hx : x ^ 2 = x) (hy : y ^ 2 = y) :
    (x + a * y) + a * ((x + a * y) ^ 2 + (x + a * y)) = x := by
  rw [imaginary_coefficient a x y ha hx hy]
  have htwo : (2 : K) = 0 := CharP.cast_eq_zero K 2
  linear_combination (a*y) * htwo

/-- Target orbit sums cancel when the second Frobenius sends a point
 to its inverse. Equivariance supplies this premise for the finite support. -/
theorem four_cycle_trace_zero {A : Type*} [AddCommGroup A]
    (F : A →+ A) (q : A) (h : F (F q) = -q) :
    q + F q + F (F q) + F (F (F q)) = 0 := by
  rw [h]
  simp only [map_neg]
  abel

/-- A double-pole coordinate under a multiplicity-two bound is unramified. -/
theorem double_bound_unramified (e : ℤ) (he : 0 < e) (hbound : 2 * e ≤ 2) :
    e = 1 := by
  have he1 : 1 ≤ e := Int.add_one_le_iff.mpr he
  linarith

/-- Two disjoint degree-four fibers inside twenty degree-four points
 give a degree coprime to eleven. Fiber geometry supplies the count bound. -/
theorem support_degree_coprime_eleven (k : ℕ) (hk : 0 < k)
    (hbound : k + k ≤ 20) : Nat.Coprime (4 * k) 11 := by
  have hle : k ≤ 10 := by nlinarith
  interval_cases k <;> decide

end Pole58
