/-!
Two-bit side-fingerprint packing and the exact K5500 split capacity.
The Rust bridge is checked by the executable root-index controls.
-/
namespace N83FourByteFP2

def packed (a b c d : Nat) : Nat := a + 4 * b + 16 * c + 64 * d

theorem packed_fits_byte (a b c d : Nat)
    (ha : a < 4) (hb : b < 4) (hc : c < 4) (hd : d < 4) :
    packed a b c d < 256 := by
  unfold packed
  omega

theorem packed_slot0 (a b c d : Nat) (ha : a < 4) :
    packed a b c d % 4 = a := by
  unfold packed
  omega

theorem packed_slot1 (a b c d : Nat) (ha : a < 4) (hb : b < 4) :
    (packed a b c d / 4) % 4 = b := by
  unfold packed
  omega

theorem packed_slot2 (a b c d : Nat)
    (ha : a < 4) (hb : b < 4) (hc : c < 4) :
    (packed a b c d / 16) % 4 = c := by
  unfold packed
  omega

theorem packed_slot3 (a b c d : Nat)
    (ha : a < 4) (hb : b < 4) (hc : c < 4) (hd : d < 4) :
    (packed a b c d / 64) % 4 = d := by
  unfold packed
  omega

theorem k5500_sidecar_bytes :
    (2 ^ 31 + 2 ^ 30) / 4 = 805306368 := by decide

theorem k5500_fp2_three_array_bytes :
    11 * (83 * 5500 * (5500 - 1) / 2 + 42 * 5500) +
    4 * (2 ^ 31 + 2 ^ 30) +
    (2 ^ 31 + 2 ^ 30) / 4 +
    (2 ^ 34 + 2 ^ 33) / 8 = 30720588978 := by decide

theorem k5500_saving_vs_five_byte :
    33136508082 - 30720588978 = 2415919104 := by decide

#print axioms packed_fits_byte
#print axioms packed_slot0
#print axioms packed_slot1
#print axioms packed_slot2
#print axioms packed_slot3
#print axioms k5500_sidecar_bytes
#print axioms k5500_fp2_three_array_bytes
#print axioms k5500_saving_vs_five_byte

end N83FourByteFP2
