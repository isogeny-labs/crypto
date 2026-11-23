import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

namespace XedniTwoGeneratorBox

instance : Fact (Nat.Prime 43) := ⟨by norm_num⟩

def centered (i : Fin 9) : ZMod 43 := (i.val : ZMod 43) - 4

/-- The explicit 9 by 9 centered coefficient box covers every reduced
scalar when the second rational point reduces to nine times the first. -/
theorem box_covers :
    ∀ t : ZMod 43, ∃ a b : Fin 9, t = centered a + 9 * centered b := by
  decide

#print axioms box_covers

end XedniTwoGeneratorBox
