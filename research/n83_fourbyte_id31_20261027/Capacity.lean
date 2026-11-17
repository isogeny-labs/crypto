/-!
The four-byte ID31 slot and the K5500 split-table arithmetic.  The table
capacity lemmas use the same state and load80 request formulas as the Rust
producer; the correspondence to its allocation path is separately tested.
-/
namespace N83FourByteID31

def states (k : Nat) : Nat := 83 * k * (k - 1) / 2 + 42 * k
def request (count : Nat) : Nat := (5 * count + 3) / 4
def slot (state selector : Nat) : Nat := 2 * (state + 1) + selector

theorem slot_roundtrip (state selector : Nat)
    (hstate : state + 1 < 2 ^ 31) (hselector : selector < 2) :
    slot state selector / 2 - 1 = state ∧
    slot state selector % 2 = selector := by
  unfold slot
  omega

theorem slot_nonzero_u32 (state selector : Nat)
    (hstate : state + 1 < 2 ^ 31) (hselector : selector < 2) :
    0 < slot state selector ∧ slot state selector < 2 ^ 32 := by
  unfold slot
  omega

theorem k5500_state_fits : states 5500 + 1 < 2 ^ 31 := by decide
theorem k5500_core_request :
    2 ^ 29 < request (states 4549) ∧
    request (states 4549) ≤ 2 ^ 30 := by decide
theorem k5500_extension_request :
    2 ^ 28 < request (states 5500 - states 4549) ∧
    request (states 5500 - states 4549) ≤ 2 ^ 29 := by decide
theorem k5500_state_count : states 5500 = 1255377750 := by decide
theorem k5500_four_byte_table_bytes :
    4 * (2 ^ 31 + 2 ^ 30) = 12884901888 := by decide
theorem k5500_five_byte_table_bytes :
    5 * (2 ^ 31 + 2 ^ 30) = 16106127360 := by decide
theorem k5500_table_saving :
    5 * (2 ^ 31 + 2 ^ 30) - 4 * (2 ^ 31 + 2 ^ 30) =
      3221225472 := by decide
theorem k5500_three_array_bytes :
    11 * states 5500 +
      4 * (2 ^ 31 + 2 ^ 30) +
      (2 ^ 34 + 2 ^ 33) / 8 = 29915282610 := by decide

#print axioms slot_roundtrip
#print axioms slot_nonzero_u32
#print axioms k5500_state_fits
#print axioms k5500_core_request
#print axioms k5500_extension_request
#print axioms k5500_state_count
#print axioms k5500_four_byte_table_bytes
#print axioms k5500_five_byte_table_bytes
#print axioms k5500_table_saving
#print axioms k5500_three_array_bytes

end N83FourByteID31
