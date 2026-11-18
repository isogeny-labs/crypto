import Mathlib.RingTheory.Valuation.Basic
import Mathlib.Tactic.Linarith

namespace Infinity56

/-- A sufficiently accurate jet has the exact order of the true local function.
The native leading terms and Hensel error bound supply these premises. -/
theorem jet_preserves_order {K Γ : Type*} [Field K]
    [LinearOrderedAddCommGroupWithTop Γ] (v : AddValuation K Γ)
    (x jet : K) (herror : v jet < v (x - jet)) : v x = v jet := by
  exact v.map_eq_of_lt_sub herror

/-- Exact chart scaling converts the numerator order into the function order. -/
theorem chart_order {K Γ : Type*} [Field K]
    [LinearOrderedAddCommGroupWithTop Γ] (v : AddValuation K Γ)
    (x num h : K) (n : ℕ) (hchart : x = num / h ^ n) :
    v x = v num - n • v h := by
  rw [hchart, v.map_div, v.map_pow]

/-- Pulling a base pole through a branch multiplies its order by ramification. -/
theorem pullback_pole (sourceOrder baseOrder e pole : ℤ)
    (hbase : baseOrder = -pole) (hcompat : sourceOrder = e * baseOrder) :
    sourceOrder = -(e * pole) := by
  simpa [hbase, mul_neg] using hcompat

/-- For a complete local factor, a full slope denominator forces both the
ramification index and residue degree. The Newton theorem supplies q|e. -/
theorem full_denominator (e f q : ℕ) (hq : 0 < q) (hf : 0 < f)
    (hdegree : e * f = q) (hdiv : q ∣ e) : e = q ∧ f = 1 := by
  have hepos : 0 < e := by nlinarith
  have hle : q ≤ e := Nat.le_of_dvd hepos hdiv
  have hupper : e ≤ q := by nlinarith
  have he : e = q := Nat.le_antisymm hupper hle
  constructor
  · exact he
  · rw [he] at hdegree
    nlinarith
end Infinity56
