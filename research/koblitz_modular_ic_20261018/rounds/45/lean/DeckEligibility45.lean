import Mathlib.Algebra.Group.Basic
import Mathlib.Tactic.Abel
namespace DeckEligibility45
variable {A : Type*} [AddCommGroup A]
def mixed (p : ℕ → A) : A :=
  4 • p 0+p 1-p 2+p 3-p 4-4 • p 5-p 6+p 7-p 8+p 9

theorem sign_fixed_zero (p : ℕ → A)
    (h : ∀ i, i<5 → p (i+5)=p i) : mixed p=0 := by
  have h0 := h 0 (by decide)
  have h1 := h 1 (by decide)
  have h2 := h 2 (by decide)
  have h3 := h 3 (by decide)
  have h4 := h 4 (by decide)
  simp only [mixed, h0, h1, h2, h3, h4]
  abel

theorem scalar_fixed_zero (p : ℕ → A)
    (h : ∀ i, i<8 → p (i+2)=p i) : mixed p=0 := by
  have h0 := h 0 (by decide)
  have h1 := h 1 (by decide)
  have h2 := h 2 (by decide)
  have h3 := h 3 (by decide)
  have h4 := h 4 (by decide)
  have h5 := h 5 (by decide)
  have h6 := h 6 (by decide)
  have h7 := h 7 (by decide)
  simp only [mixed, h7, h6, h5, h4, h3, h2, h1, h0]
  abel

theorem nonzero_requires_both (p : ℕ → A) (hn : mixed p≠0) :
    (¬ ∀ i, i<5 → p (i+5)=p i) ∧
    (¬ ∀ i, i<8 → p (i+2)=p i) := by
  exact ⟨fun h => hn (sign_fixed_zero p h), fun h => hn (scalar_fixed_zero p h)⟩
end DeckEligibility45
