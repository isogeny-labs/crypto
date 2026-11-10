import Mathlib.NumberTheory.LucasPrimality
import Mathlib.Tactic

namespace AnomalousPrattAudit

/-- Turn a checked natural remainder equality into a ZMod power equality. -/
theorem zmod_pow_of_remainder (n a e r : ℕ) (h : a^e%n=r%n) :
    (a : ZMod n)^e=(r : ZMod n) := by
  rw [← Nat.cast_pow]
  exact (ZMod.natCast_eq_natCast_iff _ _ _).mpr h

theorem zmod_pow_ne_of_remainder (n a e : ℕ) (h : a^e%n≠1%n) :
    (a : ZMod n)^e≠1 := by
  intro heq
  apply h
  rw [← Nat.cast_pow] at heq
  exact (ZMod.natCast_eq_natCast_iff _ _ _).mp (by simpa only [Nat.cast_one] using heq)

#print axioms zmod_pow_of_remainder
#print axioms zmod_pow_ne_of_remainder
end AnomalousPrattAudit
