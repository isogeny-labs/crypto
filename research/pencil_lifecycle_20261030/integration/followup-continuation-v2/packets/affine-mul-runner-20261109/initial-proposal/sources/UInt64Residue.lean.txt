import CanonicalResidue
set_option autoImplicit false
namespace PencilWordRefinement

theorem canonical_product_bound (p a b : ℕ) (hp : p < 2^31)
    (ha : a < p) (hb : b < p) : a*b < 2^64 := by
  have ha' : a < 2^31 := lt_trans ha hp
  have hb' : b < 2^31 := lt_trans hb hp
  have h := Nat.mul_le_mul (Nat.le_of_lt ha') (Nat.le_of_lt hb')
  omega

theorem uint64_mul_mod (p a b : UInt64) (hp : p.toNat < 2^31)
    (ha : a.toNat < p.toNat) (hb : b.toNat < p.toNat) :
    ((a*b)%p).toNat = (a.toNat*b.toNat)%p.toNat := by
  rw [UInt64.toNat_mod, UInt64.toNat_mul,
    Nat.mod_eq_of_lt (canonical_product_bound p.toNat a.toNat b.toNat hp ha hb)]

theorem uint64_mul_decode (p a b : UInt64) (hp : p.toNat < 2^31)
    (ha : a.toNat < p.toNat) (hb : b.toNat < p.toNat) :
    (((a*b)%p).toNat : ZMod p.toNat) =
    (a.toNat : ZMod p.toNat)*(b.toNat : ZMod p.toNat) := by
  rw [uint64_mul_mod p a b hp ha hb, ZMod.natCast_mod, Nat.cast_mul]

theorem uint64_square_decode (p a : UInt64) (hp : p.toNat < 2^31)
    (ha : a.toNat < p.toNat) :
    (((a*a)%p).toNat : ZMod p.toNat) = (a.toNat : ZMod p.toNat)^2 := by
  rw [uint64_mul_decode p a a hp ha ha, pow_two]

theorem uint64_mul_canonical (p a b : UInt64) (hp : 0 < p.toNat) :
    ((a*b)%p).toNat < p.toNat := by
  rw [UInt64.toNat_mod]
  exact Nat.mod_lt _ hp

-- Kernel-evaluated controls; no native_decide.
example : (((4 : UInt64)*(4 : UInt64))%(5 : UInt64)).toNat = 1 := by decide
example : (((9223372036854775808 : UInt64)*(2 : UInt64))%(5 : UInt64)).toNat = 0 := by decide
example : (9223372036854775808 * 2 : ℕ)%5 = 1 := by decide
example : (0 : ZMod 5) = ((5 : ℕ) : ZMod 5) := by decide
end PencilWordRefinement
