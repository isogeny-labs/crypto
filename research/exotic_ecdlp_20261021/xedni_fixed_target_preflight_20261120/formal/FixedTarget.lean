import Mathlib.Algebra.Group.Hom.Defs
import Mathlib.Tactic

namespace XedniFixedTarget

variable {A B : Type*} [AddGroup A] [AddGroup B]

/-- A rational relation can reduce only to its prescribed target for fixed `G`.
The good-reduction map is supplied as an additive homomorphism. -/
theorem projected_relation (reduce : A →+ B) (P Q : A)
    (h : P + 2 • Q = 0) : reduce P = -(2 • reduce Q) := by
  have hm : reduce P + 2 • reduce Q = 0 := by
    simpa using congrArg reduce h
  exact (add_eq_zero_iff_eq_neg).mp hm

/-- There is at most one reduced target satisfying the fixed coefficient relation. -/
theorem unique_target (G T₁ T₂ : B)
    (h₁ : T₁ + 2 • G = 0) (h₂ : T₂ + 2 • G = 0) : T₁ = T₂ := by
  exact add_right_cancel (h₁.trans h₂.symm)

#print axioms projected_relation
#print axioms unique_target

end XedniFixedTarget
