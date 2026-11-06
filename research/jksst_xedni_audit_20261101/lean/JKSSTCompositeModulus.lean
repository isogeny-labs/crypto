import Mathlib

/-!
# Composite subgroup orders do not give the field-style `1/q` count

The exact `1/q` relation probability for a fixed nonzero coefficient vector
uses that the coefficient defines a nonzero linear functional over a field.
It is false over a general cyclic group of composite order.

Here the scalar ring is `ZMod 4` and the nonzero coefficient is `2`.  Exactly
two samples, `0` and `2`, solve `2*a = 0`; hence the uniform probability is
`2/4 = 1/2`, not `1/4`.
-/

namespace JKSSTCompositeModulus

/-- Samples satisfying the relation with coefficient `2` modulo `4`. -/
def relationEventModFour : Finset (ZMod 4) :=
  Finset.univ.filter fun a ↦ (2 : ZMod 4) * a = 0

/-- The coefficient really is nonzero modulo four. -/
theorem two_ne_zero_mod_four : (2 : ZMod 4) ≠ 0 := by
  decide

/-- There are exactly two solutions to `2*a = 0` modulo four. -/
theorem card_relationEventModFour : relationEventModFour.card = 2 := by
  decide

/-- The uniform sample space has four elements. -/
theorem card_sampleSpaceModFour : (Finset.univ : Finset (ZMod 4)).card = 4 := by
  decide

/-- Cross-multiplication makes the failed `1/4` assertion denominator-free:
if the event had probability `1/4`, then `4 * #event = #sampleSpace`; Lean
computes `8 ≠ 4` instead. -/
theorem relation_probability_not_one_quarter :
    4 * relationEventModFour.card ≠
      (Finset.univ : Finset (ZMod 4)).card := by
  rw [card_relationEventModFour, card_sampleSpaceModFour]
  norm_num

/-- The exact cross-multiplied identity for probability `1/2`. -/
theorem relation_probability_is_one_half :
    2 * relationEventModFour.card =
      (Finset.univ : Finset (ZMod 4)).card := by
  rw [card_relationEventModFour, card_sampleSpaceModFour]

end JKSSTCompositeModulus
