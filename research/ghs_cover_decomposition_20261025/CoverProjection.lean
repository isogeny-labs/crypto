import SubgroupTransfer

/-!
# The complementary cover projector cannot carry the pulled source subgroup

For a same-field cover, `pull` is the divisor-class pullback and `push` is
the norm.  Their geometric composition is stated as a hypothesis here:
`push (pull x) = degree • x`.  The concrete cover formulas are not proved by
this module.  The endomorphism `degree*id - pull*push` is a scalar multiple
of the complementary or Prym projector whenever the usual isogeny
decomposition exists.  This file proves only its exact action on the pulled
subgroup, which is the transfer-relevant point.
-/

set_option autoImplicit false

namespace GhsCoverProjection

open GhsSubgroupTransfer
open Lean.Grind

variable {E J : Type} [IntModule E] [IntModule J]

def complementaryOperator
    (pull : AdditiveTransfer E J) (push : AdditiveTransfer J E)
    (degree : Int) (y : J) : J :=
  degree • y - pull.apply (push.apply y)

/-- A complementary projector built from norm and pullback annihilates the
source image.  A factorization of the Jacobian alone therefore does not give
a new transfer to this complementary factor. -/
theorem complementary_kills_pull
    (pull : AdditiveTransfer E J) (push : AdditiveTransfer J E)
    (degree : Int)
    (composition : ∀ x : E, push.apply (pull.apply x) = degree • x)
    (x : E) : complementaryOperator pull push degree (pull.apply x) = 0 := by
  unfold complementaryOperator
  rw [composition x, pull.map_zsmul]
  exact AddCommGroup.sub_self _

end GhsCoverProjection
