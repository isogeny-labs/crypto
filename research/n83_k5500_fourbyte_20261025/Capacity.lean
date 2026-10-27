import Std

-- Matches the symmetric N83 state-capacity formula in koblitz_orbit_dlp_fast.rs.
def maxStates (k : Nat) : Nat := 83 * (k * (k - 1) / 2) + 42 * k

def witnessWord (stateIndex rootSelector : Nat) : Nat :=
  2 * (stateIndex + 1) + rootSelector

theorem k5500_state_count : maxStates 5500 = 1255377750 := by decide

theorem k5500_id31 : maxStates 5500 < 2^31 := by decide

theorem k5500_load75 : 2 * maxStates 5500 ≤ 3 * 2^30 := by decide

-- Zero is the empty slot. Every valid witness has an exact inverse, despite
-- using no fingerprint bits in its four-byte slot.
theorem witness_word_exact (stateIndex rootSelector : Nat)
    (hstate : stateIndex < maxStates 5500) (hroot : rootSelector < 2) :
    0 < witnessWord stateIndex rootSelector ∧
    witnessWord stateIndex rootSelector < 2^32 ∧
    witnessWord stateIndex rootSelector / 2 - 1 = stateIndex ∧
    witnessWord stateIndex rootSelector % 2 = rootSelector := by
  have hbound := k5500_id31
  have hsize : 2^31 = 2147483648 := by decide
  have hword : 2^32 = 4294967296 := by decide
  unfold witnessWord
  omega

theorem witness_word_injective (i j r s : Nat)
    (hr : r < 2) (hs : s < 2)
    (heq : witnessWord i r = witnessWord j s) : i = j ∧ r = s := by
  unfold witnessWord at heq
  omega

-- Table, 11-byte states, and compact4 Bloom allocation only; the full
-- process RSS requires measurement on the target host.
theorem k5500_principal_bytes :
    4 * 2^32 + 11 * maxStates 5500 + 2^30 = 32062766258 := by decide

#print axioms k5500_state_count
#print axioms k5500_id31
#print axioms k5500_load75
#print axioms witness_word_exact
#print axioms witness_word_injective
#print axioms k5500_principal_bytes
