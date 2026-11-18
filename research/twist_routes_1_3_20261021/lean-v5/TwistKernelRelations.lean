import Mathlib.Algebra.BigOperators.Group.Finset.Basic
import Mathlib.Algebra.Group.Hom.Defs
import Mathlib.Tactic.NormNum

/- The structural implication of a degree7 dual identity on an 823-torsion
   group. Concrete curve group laws, maps and the dual identity are externally
   certified, and their formal bridge is not assumed to have been proved. -/
namespace TwistKernelRelations
open scoped BigOperators

variable {A B : Type*} [AddCommGroup A] [AddCommGroup B]

theorem recovery (f : A →+ B) (d : B →+ A)
    (hdual : ∀ a, d (f a) = 7 • a) (ht : ∀ a : A, 823 • a = 0) (a : A) :
    588 • d (f a) = a := by
  rw [hdual]
  calc
    588 • (7 • a) = (4116 : ℕ) • a := by rw [← mul_nsmul]
    _ = (1 + 5 * 823 : ℕ) • a := by norm_num
    _ = a + 5 • (823 • a) := by rw [add_nsmul, one_nsmul, ← mul_nsmul]
    _ = a := by rw [ht]; simp

theorem injective (f : A →+ B) (d : B →+ A)
    (hdual : ∀ a, d (f a) = 7 • a) (ht : ∀ a : A, 823 • a = 0) :
    Function.Injective f := by
  intro a b h
  calc
    a = 588 • d (f a) := (recovery f d hdual ht a).symm
    _ = 588 • d (f b) := by rw [h]
    _ = b := recovery f d hdual ht b

theorem relation_iff (f : A →+ B) (d : B →+ A)
    (hdual : ∀ a, d (f a) = 7 • a) (ht : ∀ a : A, 823 • a = 0)
    {ι : Type*} (s : Finset ι) (c : ι → ℤ) (P : ι → A) (R : A) :
    (∑ i ∈ s, c i • f (P i)) = f R ↔ (∑ i ∈ s, c i • P i) = R := by
  constructor
  · intro h
    apply injective f d hdual ht
    simpa only [map_sum, map_zsmul] using h
  · intro h
    simpa only [map_sum, map_zsmul] using congrArg f h

theorem scalar_transport (f : A →+ B) (d : B →+ A)
    (hdual : ∀ a, d (f a) = 7 • a) (ht : ∀ a : A, 823 • a = 0)
    (G Q : A) (k : ℕ) : k • f G = f Q ↔ k • G = Q := by
  rw [← map_nsmul]
  exact (injective f d hdual ht).eq_iff

end TwistKernelRelations
