import Mathlib.Data.ZMod.Basic
import SameSupportClassifier

set_option autoImplicit false
namespace PencilWordRefinement

/-- Nat expression of Field::neg for n=1; not a Rust execution semantics. -/
def residueNeg (p a : ℕ) : ℕ := (p - a % p) % p
/-- Nat expression of Field::sub for n=1; machine addition needs the bound below. -/
def residueSub (p a b : ℕ) : ℕ := (a + residueNeg p b) % p

theorem canonical_equality (p a b : ℕ) (ha : a < p) (hb : b < p) :
    ((a : ZMod p) = (b : ZMod p)) ↔ a = b := by
  rw [ZMod.natCast_eq_natCast_iff', Nat.mod_eq_of_lt ha, Nat.mod_eq_of_lt hb]

theorem neg_decode (p b : ℕ) (hb : b < p) :
    ((residueNeg p b : ℕ) : ZMod p) = -(b : ZMod p) := by
  unfold residueNeg
  rw [ZMod.natCast_mod, Nat.mod_eq_of_lt hb, Nat.cast_sub (Nat.le_of_lt hb),
    ZMod.natCast_self, zero_sub]

theorem sub_decode (p a b : ℕ) (hb : b < p) :
    ((residueSub p a b : ℕ) : ZMod p) = (a : ZMod p) - (b : ZMod p) := by
  unfold residueSub
  rw [ZMod.natCast_mod, Nat.cast_add, neg_decode p b hb, sub_eq_add_neg]

theorem neg_equality_reflection (p a b : ℕ) (hp : 0 < p)
    (ha : a < p) (hb : b < p) :
    a = residueNeg p b ↔ (a : ZMod p) = -(b : ZMod p) := by
  have hn : residueNeg p b < p := Nat.mod_lt _ hp
  rw [← canonical_equality p a (residueNeg p b) ha hn, neg_decode p b hb]

theorem sub_zero_reflection (p a b : ℕ) (hp : 0 < p)
    (ha : a < p) (hb : b < p) :
    residueSub p a b = 0 ↔ a = b := by
  have hn : residueSub p a b < p := Nat.mod_lt _ hp
  rw [← canonical_equality p (residueSub p a b) 0 hn hp,
    sub_decode p a b hb, Nat.cast_zero, sub_eq_zero, canonical_equality p a b ha hb]

/-- Explicit study-domain wrapper. Primality is retained for the downstream
field theorem; reflection itself needs only a positive modulus and canonical
representatives. The first conclusion bounds the one machine addition only. -/
theorem bounded_prime_neg_sub (p a b : ℕ) (hprime : Nat.Prime p)
    (hfive : 5 ≤ p) (hbound : p < 2^31) (ha : a < p) (hb : b < p) :
    a + residueNeg p b < 2^64 ∧
    (a = residueNeg p b ↔ (a : ZMod p) = -(b : ZMod p)) ∧
    (residueSub p a b = 0 ↔ a = b) := by
  have hp : 0 < p := hprime.pos
  have hn : residueNeg p b < p := Nat.mod_lt _ hp
  refine ⟨?_, neg_equality_reflection p a b hp ha hb,
    sub_zero_reflection p a b hp ha hb⟩
  omega
end PencilWordRefinement
