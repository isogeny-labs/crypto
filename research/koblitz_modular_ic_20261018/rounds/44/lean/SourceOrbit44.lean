import Mathlib.Logic.Function.Iterate
import Mathlib.Algebra.Group.Basic
import Mathlib.Tactic.Abel

namespace SourceOrbit44

theorem sign_fixed_of_fifth_orbit {X : Type*} (F S : X → X)
    (p : X) (d : ℕ) (hf : (F^[5]) p=p)
    (hs : S p=(F^[5*d]) p) : S p=p := by
  rw [hs, Function.iterate_mul]
  exact Function.iterate_fixed hf d

variable {A : Type*} [AddCommGroup A]

def mixed (p : ℕ → A) : A :=
  4 • p 0+p 1-p 2+p 3-p 4-4 • p 5-p 6+p 7-p 8+p 9

theorem mixed_zero_of_sign_pairs (p : ℕ → A)
    (h : ∀ i, i<5 → p (i+5)=p i) : mixed p=0 := by
  have h0 : p 5=p 0 := h 0 (by decide)
  have h1 : p 6=p 1 := h 1 (by decide)
  have h2 : p 7=p 2 := h 2 (by decide)
  have h3 : p 8=p 3 := h 3 (by decide)
  have h4 : p 9=p 4 := h 4 (by decide)
  simp only [mixed, h0, h1, h2, h3, h4]
  abel

theorem mixed_zero_of_fifth_frobenius (p : ℕ → A) (F : A → A)
    (d : ℕ) (hf : ∀ i, i<5 → (F^[5]) (p i)=p i)
    (hs : ∀ i, i<5 → p (i+5)=(F^[5*d]) (p i)) : mixed p=0 := by
  apply mixed_zero_of_sign_pairs
  intro i hi
  rw [hs i hi, Function.iterate_mul]
  exact Function.iterate_fixed (hf i hi) d

end SourceOrbit44
