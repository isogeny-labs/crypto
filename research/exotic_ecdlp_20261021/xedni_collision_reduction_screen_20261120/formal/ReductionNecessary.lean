import Mathlib

/-! A relation on a group maps to a relation under every additive homomorphism. -/

namespace XedniCollisionReduction

theorem no_relation_of_reduced_sum {A B : Type*} [AddCommGroup A] [AddCommGroup B]
    (red : A →+ B) (P Q : A) (h : red P + red Q + red Q ≠ 0) :
    P + Q + Q ≠ 0 := by
  intro hsource
  apply h
  have himage := congrArg red hsource
  simpa only [map_add, map_zero] using himage

#print axioms no_relation_of_reduced_sum

end XedniCollisionReduction
