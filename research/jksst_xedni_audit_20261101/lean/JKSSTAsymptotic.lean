import Mathlib

/-!
# Why an asymptotic error term cannot be deleted

JKSST quote a lattice-point asymptotic with an `O`-term and then use only
the main term as a finite upper bound.  This file isolates the logical issue.
It does **not** model their particular lattice count: it gives a counterexample
to the general inference

`f = main + O(error)  ⇒  f ≤ main`.

The counterexample is `f(n) = n + 1`, whose residual from the main term `n`
is `O(1)`, but is positive for every finite `n`.
-/

open Filter Asymptotics

namespace JKSSTAsymptotic

def count (n : ℕ) : ℝ := n + 1

def mainTerm (n : ℕ) : ℝ := n

def constantError (_n : ℕ) : ℝ := 1

/-- The residual in the counterexample really is `O(1)` at infinity. -/
theorem residual_isBigO :
    (fun n ↦ count n - mainTerm n) =O[atTop] constantError := by
  have h : constantError =O[atTop] constantError :=
    isBigO_refl constantError atTop
  convert h using 1
  funext n
  simp [count, mainTerm, constantError]

/-- Nevertheless, the alleged finite upper bound fails at every index. -/
theorem mainTerm_lt_count (n : ℕ) : mainTerm n < count n := by
  simp [count, mainTerm]

/-- A single formal witness refuting the rule that one may delete an
unspecified `O`-term to obtain an upper bound by the main term. -/
theorem bigO_remainder_does_not_imply_main_upper_bound :
    ∃ f main error : ℕ → ℝ,
      (fun n ↦ f n - main n) =O[atTop] error ∧
      ∀ n, main n < f n := by
  exact ⟨count, mainTerm, constantError, residual_isBigO, mainTerm_lt_count⟩

end JKSSTAsymptotic
