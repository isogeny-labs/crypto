import Mathlib.Algebra.Group.Hom.Defs
import Mathlib.Tactic

namespace XedniVariableCoefficient

variable {A B : Type*} [AddGroup A] [AddGroup B]

/-- Any rational relation with coefficient `k` prescribes the reduced target. -/
theorem projected_relation (reduce : A →+ B) (P Q : A) (k : ℕ)
    (h : P + k • Q = 0) : reduce P = -(k • reduce Q) := by
  have hm : reduce P + k • reduce Q = 0 := by
    simpa using congrArg reduce h
  exact (add_eq_zero_iff_eq_neg).mp hm

/-- For a fixed coefficient and generator, at most one reduced target fits. -/
theorem unique_target (G T₁ T₂ : B) (k : ℕ)
    (h₁ : T₁ + k • G = 0) (h₂ : T₂ + k • G = 0) : T₁ = T₂ := by
  exact add_right_cancel (h₁.trans h₂.symm)

/-- If two coefficients fit one target, their generator multiples coincide. -/
theorem coefficient_collision (G T : B) (k₁ k₂ : ℕ)
    (h₁ : T + k₁ • G = 0) (h₂ : T + k₂ • G = 0) : k₁ • G = k₂ • G := by
  exact add_left_cancel (h₁.trans h₂.symm)

#print axioms projected_relation
#print axioms unique_target
#print axioms coefficient_collision

end XedniVariableCoefficient
