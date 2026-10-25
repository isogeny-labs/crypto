import Init.Grind.Module.Basic

/-!
# The algebraic step in a certified GHS subgroup transfer

`source` is the actual subgroup, not the entire elliptic curve.  `pull` may
be trace-conorm, a quotient isogeny, or their composition.  The geometric
maps and the equality `push ∘ pull = [degree]` are explicit hypotheses;
the checker must supply them for the particular candidate.  The Bézout
certificate is also an input, obtained from the certified subgroup order.
-/

set_option autoImplicit false

namespace GhsSubgroupTransfer

open Lean.Grind

variable {E J : Type} [IntModule E] [IntModule J]

/-- A map that sends source scalar multiples to destination scalar multiples. -/
structure AdditiveTransfer (E J : Type) [IntModule E] [IntModule J] where
  apply : E → J
  map_zero : apply 0 = 0
  map_add : ∀ (x y : E), apply (x + y) = apply x + apply y
  map_zsmul : ∀ (n : Int) (x : E), apply (n • x) = n • apply x

/-- A certified degree coprime to the source subgroup exponent makes
the explicit transfer injective.  The proof works for a composite r as long
as the given Bézout equality holds. -/
theorem injective_of_push_pull
    (pull : AdditiveTransfer E J) (push : AdditiveTransfer J E)
    (degree order u v : Int)
    (annihilates : ∀ x : E, order • x = 0)
    (bezout : u * degree + v * order = 1)
    (composition : ∀ x : E, push.apply (pull.apply x) = degree • x) :
    Function.Injective pull.apply := by
  intro x y hxy
  have hscaled : degree • x = degree • y := by
    calc
      degree • x = push.apply (pull.apply x) := (composition x).symm
      _ = push.apply (pull.apply y) := congrArg push.apply hxy
      _ = degree • y := composition y
  have hdifference : degree • (x - y) = 0 := by
    rw [IntModule.zsmul_sub, hscaled, AddCommGroup.sub_self]
  have hannihilation : order • (x - y) = 0 := annihilates (x - y)
  have hzero : x - y = 0 := by
    calc
      x - y = (1 : Int) • (x - y) := (IntModule.one_zsmul _).symm
      _ = (u * degree + v * order) • (x - y) := by rw [bezout]
      _ = u • (degree • (x - y)) + v • (order • (x - y)) := by
        rw [IntModule.add_zsmul, IntModule.mul_zsmul, IntModule.mul_zsmul]
      _ = 0 := by simp only [hdifference, hannihilation, IntModule.zsmul_zero, AddCommMonoid.zero_add]
  exact AddCommGroup.sub_eq_zero_iff.mp hzero

/-- The scalar relation is preserved by the same explicit transfer. -/
theorem preserves_discrete_log (pull : AdditiveTransfer E J)
    (generator target : E) (scalar : Int)
    (relation : target = scalar • generator) :
    pull.apply target = scalar • pull.apply generator := by
  rw [relation, pull.map_zsmul]

/-- If the destination has no nonzero points killed by the source subgroup
order, every additive transfer annihilates that subgroup.  The finite-field
Weil bound and Lagrange's theorem establish `destination_torsion_trivial`
for the excluded concrete profiles; this theorem isolates the map step. -/
theorem image_zero_of_no_destination_torsion
    (pull : AdditiveTransfer E J) (order : Int)
    (annihilates : ∀ x : E, order • x = 0)
    (destination_torsion_trivial : ∀ y : J, order • y = 0 → y = 0)
    (x : E) : pull.apply x = 0 := by
  apply destination_torsion_trivial
  rw [← pull.map_zsmul, annihilates x, pull.map_zero]

/-- A generator has a nonzero image once injectivity is certified. -/
theorem image_nonzero
    (pull : AdditiveTransfer E J) (push : AdditiveTransfer J E)
    (degree order u v : Int)
    (annihilates : ∀ x : E, order • x = 0)
    (bezout : u * degree + v * order = 1)
    (composition : ∀ x : E, push.apply (pull.apply x) = degree • x)
    (generator : E) (generator_ne_zero : generator ≠ 0) :
    pull.apply generator ≠ 0 := by
  intro h
  have hsame : pull.apply generator = pull.apply 0 := by
    rw [h, pull.map_zero]
  exact generator_ne_zero
    (injective_of_push_pull pull push degree order u v annihilates bezout composition hsame)

end GhsSubgroupTransfer
