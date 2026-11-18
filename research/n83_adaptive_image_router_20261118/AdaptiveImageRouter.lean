/-!
# Projector-only image routing

The native certificate checks one exhaustive finite fixture.  This file proves
the abstract statements used to interpret it: a pull-push router is constant
on pushforward-kernel cosets, exact relations push forward, and pullback is
injective whenever its push-pull composition is injective.

No finite-field arithmetic, asymptotic cost, or discrete-log conclusion is
formalized here.
-/

set_option autoImplicit false

namespace N83AdaptiveImageRouter

structure AdditiveMonoid (A : Type) where
  zero : A
  add : A → A → A
  add_zero : ∀ a, add a zero = a
  add_assoc : ∀ a b c, add (add a b) c = add a (add b c)

structure AdditiveTransfer (A B : Type)
    (source : AdditiveMonoid A) (target : AdditiveMonoid B) where
  apply : A → B
  map_zero : apply source.zero = target.zero
  map_add : ∀ a b, apply (source.add a b) = target.add (apply a) (apply b)

variable {E J : Type}
variable (elliptic : AdditiveMonoid E) (jacobian : AdditiveMonoid J)

def imageRoute
    (pull : AdditiveTransfer E J elliptic jacobian)
    (push : AdditiveTransfer J E jacobian elliptic)
    (divisor : J) : J :=
  pull.apply (push.apply divisor)

/-- Adding a pushforward-kernel class cannot change the pull-push route. -/
theorem imageRoute_kernel_coset
    (pull : AdditiveTransfer E J elliptic jacobian)
    (push : AdditiveTransfer J E jacobian elliptic)
    (divisor kernel : J)
    (kernel_pushes_zero : push.apply kernel = elliptic.zero) :
    imageRoute elliptic jacobian pull push (jacobian.add divisor kernel) =
      imageRoute elliptic jacobian pull push divisor := by
  unfold imageRoute
  rw [push.map_add, kernel_pushes_zero, elliptic.add_zero]

/-- Applying any normalization after pull-push preserves the coset collapse. -/
theorem normalizedRoute_kernel_coset
    (pull : AdditiveTransfer E J elliptic jacobian)
    (push : AdditiveTransfer J E jacobian elliptic)
    (normalize : J → J)
    (divisor kernel : J)
    (kernel_pushes_zero : push.apply kernel = elliptic.zero) :
    normalize (imageRoute elliptic jacobian pull push
      (jacobian.add divisor kernel)) =
      normalize (imageRoute elliptic jacobian pull push divisor) := by
  rw [imageRoute_kernel_coset elliptic jacobian pull push divisor kernel
    kernel_pushes_zero]

/-- Every exact two-term Jacobian relation pushes to an elliptic relation. -/
theorem exactPair_pushes_zero
    (push : AdditiveTransfer J E jacobian elliptic)
    (left right : J)
    (exact_relation : jacobian.add left right = jacobian.zero) :
    elliptic.add (push.apply left) (push.apply right) = elliptic.zero := by
  rw [← push.map_add, exact_relation, push.map_zero]

/-- A pullback is injective if its push-pull composition is an injective
endomorphism.  For the n=83 order-r subgroup that endomorphism is `[3]`,
which is injective because `gcd(3,r)=1`. -/
theorem pull_injective_if_composition_injective
    (pull : AdditiveTransfer E J elliptic jacobian)
    (push : AdditiveTransfer J E jacobian elliptic)
    (composition : E → E)
    (push_pull : ∀ point, push.apply (pull.apply point) = composition point)
    (composition_injective : Function.Injective composition) :
    Function.Injective pull.apply := by
  intro left right equal_pullbacks
  apply composition_injective
  rw [← push_pull left, ← push_pull right, equal_pullbacks]

end N83AdaptiveImageRouter

#print axioms N83AdaptiveImageRouter.imageRoute_kernel_coset
#print axioms N83AdaptiveImageRouter.normalizedRoute_kernel_coset
#print axioms N83AdaptiveImageRouter.exactPair_pushes_zero
#print axioms N83AdaptiveImageRouter.pull_injective_if_composition_injective
