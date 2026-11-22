import Mathlib.Algebra.Group.Subgroup.Ker
import Mathlib.Algebra.Group.Basic
import Mathlib.Tactic.Abel

namespace TwistAffineCompression

variable {K V H C T : Type*}
  [AddCommGroup K] [AddCommGroup V] [AddCommGroup H]
  [AddCommGroup C] [AddCommGroup T]

/- Parameterizing the homogeneous known constraints preserves the full set of
   target values. The concrete finite RREF basis/range bridge is external. -/
theorem target_values_parameterization
    (p : K →+ V) (h : V →+ H) (f : V →+ C) (target : V →+ T)
    (hp : p.range = h.ker) (x0 : V) (b : H) (c : C) (k : T)
    (hx0 : h x0 = b) :
    (∃ x, h x = b ∧ f x = c ∧ target x = k) ↔
    (∃ v, f (x0 + p v) = c ∧ target (x0 + p v) = k) := by
  constructor
  · rintro ⟨x, hx, hf, ht⟩
    have hz : x - x0 ∈ h.ker := by
      change h (x - x0) = 0
      rw [map_sub, hx, hx0, sub_self]
    rw [← hp] at hz
    rcases hz with ⟨v, hv⟩
    have he : x0 + p v = x := by rw [hv]; abel
    exact ⟨v, by simpa only [he] using hf, by simpa only [he] using ht⟩
  · rintro ⟨v, hf, ht⟩
    have hz : p v ∈ h.ker := by
      rw [← hp]
      exact ⟨v, rfl⟩
    change h (p v) = 0 at hz
    refine ⟨x0 + p v, ?_, hf, ht⟩
    rw [map_add, hx0, hz, add_zero]

/- Explicit one-edge substitution, valid also for negative multipliers and
   arbitrary families of rows. It preserves every possible target k. -/
theorem substitute_one_edge {R I : Type*} [CommRing R]
    (a b d e : I → R) (c x k : R) :
    (∃ y : R, y = c*x ∧ ∀ i, a i*x + b i*y + d i*k = e i) ↔
    (∀ i, (a i + b i*c)*x + d i*k = e i) := by
  constructor
  · rintro ⟨y, hy, hr⟩ i
    simpa only [hy, add_mul, mul_assoc] using hr i
  · intro hr
    refine ⟨c*x, rfl, ?_⟩
    intro i
    simpa only [add_mul, mul_assoc] using hr i

end TwistAffineCompression
