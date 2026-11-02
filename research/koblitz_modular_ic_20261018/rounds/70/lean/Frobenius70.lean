import Mathlib.Tactic.LinearCombination

namespace Frobenius70

/-- Two independently witnessed obstruction rows kill the whole two-class kernel. -/
theorem two_row_kernel_zero {K : Type*} [Field K]
    (a b c d x y : K) (hdet : a * d - b * c ≠ 0)
    (h₁ : a * x + b * y = 0) (h₂ : c * x + d * y = 0) :
    x = 0 ∧ y = 0 := by
  have hxmul : (a * d - b * c) * x = 0 := by
    linear_combination d * h₁ - b * h₂
  have hymul : (a * d - b * c) * y = 0 := by
    linear_combination a * h₂ - c * h₁
  exact ⟨(mul_eq_zero.mp hxmul).resolve_left hdet,
         (mul_eq_zero.mp hymul).resolve_left hdet⟩

/-- R70's saved binary rows (1,0) and (1,1) kill the kernel over any field. -/
theorem saved_row_minor_zero {K : Type*} [Field K] (x y : K)
    (h₁ : (1 : K) * x + 0 * y = 0)
    (h₂ : (1 : K) * x + 1 * y = 0) : x = 0 ∧ y = 0 := by
  have hdet : (1 : K) * 1 - 0 * 1 ≠ 0 := by simp
  exact two_row_kernel_zero 1 0 1 1 x y hdet h₁ h₂

/-- Iterate characteristic-two squaring as used by descending Frobenius. -/
def squareIter {K : Type*} [Monoid K] : ℕ → K → K
  | 0, x => x
  | n + 1, x => (squareIter n x) ^ 2

/-- A square-stable lattice contains every iterated square of each element. -/
theorem squareIter_mem {K : Type*} [Monoid K] (W : Set K)
    (hsq : ∀ x ∈ W, x ^ 2 ∈ W) {x : K} (hx : x ∈ W) :
    ∀ n, squareIter n x ∈ W := by
  intro n
  induction n with
  | zero => simpa [squareIter] using hx
  | succ n ih => simpa [squareIter] using hsq (squareIter n x) ih

/-- Conditional fixed-point step: the integral-closure criterion is explicit. -/
theorem stable_sandwich_eq_closure {K : Type*} [Monoid K]
    (W C V₀ : Set K) (hCW : C ⊆ W) (hWV : W ⊆ V₀)
    (hsq : ∀ x ∈ W, x ^ 2 ∈ W)
    (hcriterion : ∀ x, (∀ n, squareIter n x ∈ V₀) → x ∈ C) :
    W = C := by
  ext x
  constructor
  · intro hx
    apply hcriterion x
    intro n
    exact hWV (squareIter_mem W hsq hx n)
  · intro hx
    exact hCW hx

end Frobenius70
