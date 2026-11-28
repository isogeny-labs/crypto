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

/-- One descending step retains precisely elements whose square stays inside. -/
def squarePreimage {K : Type*} [Monoid K] (V : Set K) : Set K :=
  {x | x ∈ V ∧ x ^ 2 ∈ V}

/-- The finite chain used by the complete native quotient calculations. -/
def descend {K : Type*} [Monoid K] : ℕ → Set K → Set K
  | 0, V => V
  | n + 1, V => squarePreimage (descend n V)

/-- A square-closed integral closure stays in every descending lattice. -/
theorem closure_subset_descend {K : Type*} [Monoid K]
    (C V : Set K) (hCV : C ⊆ V) (hsq : ∀ x ∈ C, x ^ 2 ∈ C) :
    ∀ n, C ⊆ descend n V := by
  intro n
  induction n with
  | zero => simpa [descend] using hCV
  | succ n ih =>
      intro x hx
      exact ⟨ih hx, ih (hsq x hx)⟩

/-- If the sixth exact step is the order, trace-dual containment proves normality. -/
theorem closure_eq_order_of_six_steps {K : Type*} [Monoid K]
    (O C V₀ : Set K) (hOC : O ⊆ C) (hCV : C ⊆ V₀)
    (hsq : ∀ x ∈ C, x ^ 2 ∈ C) (hV6 : descend 6 V₀ = O) :
    C = O := by
  apply Set.Subset.antisymm
  · intro x hx
    have h : x ∈ descend 6 V₀ := (closure_subset_descend C V₀ hCV hsq 6) hx
    simpa [hV6] using h
  · exact hOC

end Frobenius70
