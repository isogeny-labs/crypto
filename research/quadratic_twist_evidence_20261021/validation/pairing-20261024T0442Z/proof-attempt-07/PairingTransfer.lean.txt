import TwistEigenspace
import Mathlib.GroupTheory.OrderOfElement

/-! A character reduction with explicit homomorphism/nondegeneracy hypotheses.
The concrete Miller function is checked separately in PairingFixture. -/
namespace PairingTransfer

variable {G U : Type*} [AddCommGroup G] [Group U]

theorem character_scalar (e : G →+ Additive U) (P : G) (n : ℕ) :
    (e (n • P)).toMul = (e P).toMul ^ n := by
  rw [map_nsmul]
  rfl

theorem character_order (e : G →+ Additive U) (P : G) (r : ℕ)
    (hp : r.Prime) (hr : r • P = 0) (hne : (e P).toMul ≠ 1) :
    orderOf (e P).toMul = r := by
  let : Fact r.Prime := ⟨hp⟩
  apply orderOf_eq_prime _ hne
  rw [← character_scalar, hr, map_zero]
  rfl

theorem character_log_recovery (e : G →+ Additive U) (P : G) (r : ℕ)
    (hp : r.Prime) (hr : r • P = 0) (hne : (e P).toMul ≠ 1)
    (n m : Fin r) : (e (n.val • P)).toMul = (e P).toMul ^ m.val ↔ n = m := by
  rw [character_scalar]
  constructor
  · intro h
    apply Fin.ext
    apply pow_injOn_Iio_orderOf _ _ h
    · change n.val < orderOf (e P).toMul
      rw [character_order e P r hp hr hne]
      exact n.isLt
    · change m.val < orderOf (e P).toMul
      rw [character_order e P r hp hr hne]
      exact m.isLt
  · rintro rfl
    rfl

theorem norm_product_identity {F : Type*} [CommRing F] (d x y s t : F) :
    (x * s + d * y * t) ^ 2 - d * (x * t + y * s) ^ 2 =
      (x ^ 2 - d * y ^ 2) * (s ^ 2 - d * t ^ 2) := by ring

#print axioms character_scalar
#print axioms character_order
#print axioms character_log_recovery
#print axioms norm_product_identity

end PairingTransfer
