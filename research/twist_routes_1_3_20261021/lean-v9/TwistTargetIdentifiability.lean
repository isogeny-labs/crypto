import Mathlib.Algebra.Group.Hom.Defs
import Mathlib.Algebra.Group.Basic

namespace TwistTargetIdentifiability

variable {A B C : Type*} [AddCommGroup A] [AddCommGroup B] [AddCommGroup C]

/- Abstract affine solution fiber and target projection. The finite matrix,
   spanning kernel basis, curve and concrete extraction bridge is external. -/
theorem target_fiber_iff_kernel (f : A →+ B) (target : A →+ C) (x0 : A) :
    (∀ x, f x = f x0 → target x = target x0) ↔
    (∀ v, f v = 0 → target v = 0) := by
  constructor
  · intro h v hv
    have he : target (x0+v) = target x0 := h (x0+v) (by rw [map_add, hv, add_zero])
    have he' : target x0 + target v = target x0 + 0 := by
      simpa only [map_add, add_zero] using he
    exact add_left_cancel he'
  · intro h x hx
    have hv : f (x-x0) = 0 := by rw [map_sub, hx, sub_self]
    have ht : target (x-x0) = 0 := h (x-x0) hv
    exact sub_eq_zero.mp (by simpa only [map_sub] using ht)

theorem kernel_witness_changes_target (f : A →+ B) (target : A →+ C) (x0 v : A)
    (hv : f v = 0) (ht : target v ≠ 0) :
    ∃ x, f x = f x0 ∧ target x ≠ target x0 := by
  refine ⟨x0+v, ?_, ?_⟩
  · rw [map_add, hv, add_zero]
  · intro he
    apply ht
    have he' : target x0 + target v = target x0 + 0 := by
      simpa only [map_add, add_zero] using he
    exact add_left_cancel he'

end TwistTargetIdentifiability
