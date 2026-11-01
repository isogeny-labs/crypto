import CoverProjection

/-!
# Image-endomorphism and Prym-factor identities for the binary cubic cover

The geometric input is explicit: `push (pull x) = degree • x`.  The first two
theorems derive the image endomorphism equation and the complementary-kernel
property in any integer module.  The final theorems check the polynomial
factorization used by the n=83 order audit.
-/

set_option autoImplicit false

namespace N83JacobianProjector

open GhsSubgroupTransfer

variable {E J : Type} [IntModule E] [IntModule J]

def imageOperator
    (pull : AdditiveTransfer E J) (push : AdditiveTransfer J E) (y : J) : J :=
  pull.apply (push.apply y)

/-- If `push ∘ pull=[degree]`, then `e=pull ∘ push` satisfies
`e²=[degree]e`. -/
theorem imageOperator_quadratic
    (pull : AdditiveTransfer E J) (push : AdditiveTransfer J E)
    (degree : Int)
    (composition : ∀ x : E, push.apply (pull.apply x) = degree • x)
    (y : J) :
    imageOperator pull push (imageOperator pull push y) =
      degree • imageOperator pull push y := by
  unfold imageOperator
  rw [composition, pull.map_zsmul]

/-- The unnormalised complementary component `[degree]y-e(y)` is killed by
the pushforward. -/
theorem complementary_push_zero
    (pull : AdditiveTransfer E J) (push : AdditiveTransfer J E)
    (degree : Int)
    (composition : ∀ x : E, push.apply (pull.apply x) = degree • x)
    (y : J) :
    push.apply (GhsCoverProjection.complementaryOperator pull push degree y) = 0 := by
  unfold GhsCoverProjection.complementaryOperator
  rw [push.map_sub, push.map_zsmul, composition]
  exact AddCommGroup.sub_self _

/-- Unconditional integer-polynomial expansion behind the Prym factorization. -/
theorem prym_factor_expansion (x a q : Int) :
    (x ^ 2 - a * x + q) * (x ^ 2 + a * x + q) =
      x ^ 4 + (2 * q - a ^ 2) * x ^ 2 + q ^ 2 := by
  ring

/-- At n=83, `a=2^42` and `q=2^83` satisfy `a²=2q`, so the middle
coefficient vanishes. -/
theorem prym_factorization (x a q : Int) (trace_relation : a ^ 2 = 2 * q) :
    (x ^ 2 - a * x + q) * (x ^ 2 + a * x + q) = x ^ 4 + q ^ 2 := by
  rw [prym_factor_expansion, trace_relation]
  ring

end N83JacobianProjector
