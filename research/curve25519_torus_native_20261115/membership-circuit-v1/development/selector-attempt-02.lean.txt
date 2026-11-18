import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

set_option autoImplicit false

namespace Curve25519SelectorInstance
def modulus : ℕ := 2^255-19

example {n a : ℕ} (h : a < n) : (a : ZMod n).val = a :=
  ZMod.val_natCast_of_lt h

theorem actual_selector_no_wrap {x d : ℕ} (hx : x < 32768) (hd : d < 32768)
    (h : (x : ZMod modulus) + (d : ZMod modulus) = 20632) : x+d = 20632 := by
  have hlt : x+d < modulus := by
    have bound : 65536 < modulus := by decide
    omega
  have hcap : 20632 < modulus := by decide
  have heq : ((x+d : ℕ) : ZMod modulus) = (20632 : ZMod modulus) := by
    simpa only [Nat.cast_add] using h
  have values := congrArg ZMod.val heq
  rw [ZMod.val_natCast_of_lt hlt, ZMod.val_ofNat_of_lt hcap] at values
  exact values

theorem actual_selector_range {x d : ℕ} (hx : x < 32768) (hd : d < 32768)
    (h : (x : ZMod modulus) + (d : ZMod modulus) = 20632) : x < 20633 := by
  have e := actual_selector_no_wrap hx hd h
  omega

theorem small_modulus_counterexample :
    (31 : ZMod 17) + (3 : ZMod 17) = (17 : ZMod 17) ∧
    (31 : ℕ) < 32 ∧ (3 : ℕ) < 32 ∧ (17 : ℕ) < 31 := by decide

#print axioms actual_selector_no_wrap
#print axioms actual_selector_range
#print axioms small_modulus_counterexample
end Curve25519SelectorInstance
