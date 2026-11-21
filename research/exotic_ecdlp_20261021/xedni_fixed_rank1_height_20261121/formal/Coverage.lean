import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

/-! Exact signed-multiple coverage for the fixed order-43 reduction. -/

namespace XedniFixedRankOneCoverage

theorem signed_multiples_cover_nonzero :
    ∀ x : ZMod 43, x ≠ 0 →
      ∃ n : Fin 21,
        x = ((n.val + 1 : ℕ) : ZMod 43) ∨
        x = -((n.val + 1 : ℕ) : ZMod 43) := by decide

theorem prefix14_misses_scalar15 :
    ¬ ∃ n : Fin 14,
      (15 : ZMod 43) = ((n.val + 1 : ℕ) : ZMod 43) ∨
      (15 : ZMod 43) = -((n.val + 1 : ℕ) : ZMod 43) := by decide

def wordEndpoint (m : Fin 3) : ZMod 43 :=
  if m.val = 0 then 1 else if m.val = 1 then 2 else 6

theorem prefix14_words_cover_nonzero :
    ∀ x : ZMod 43, x ≠ 0 →
      ∃ n : Fin 14, ∃ m : Fin 3,
        x = wordEndpoint m * ((n.val + 1 : ℕ) : ZMod 43) ∨
        x = -(wordEndpoint m * ((n.val + 1 : ℕ) : ZMod 43)) := by decide

theorem prefix13_words_miss_scalar15 :
    ¬ ∃ n : Fin 13, ∃ m : Fin 3,
      (15 : ZMod 43) = wordEndpoint m * ((n.val + 1 : ℕ) : ZMod 43) ∨
      (15 : ZMod 43) = -(wordEndpoint m * ((n.val + 1 : ℕ) : ZMod 43)) := by decide

#print axioms signed_multiples_cover_nonzero
#print axioms prefix14_misses_scalar15
#print axioms prefix14_words_cover_nonzero
#print axioms prefix13_words_miss_scalar15

end XedniFixedRankOneCoverage
